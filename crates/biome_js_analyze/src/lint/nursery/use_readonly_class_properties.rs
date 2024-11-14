use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsExpression, AnyJsFormalParameter, AnyJsPropertyModifier, AnyJsStatement, AnyTsPropertyParameterModifier, JsAssignmentExpression, JsClassDeclaration, JsConstructorClassMember, JsFunctionBody, JsLanguage, JsMethodClassMember, JsPropertyClassMember, JsStatementList, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TsPropertyParameter
};
use biome_rowan::{
    declare_node_union, syntax::SyntaxTrivia, AstNode, AstNodeExt, AstNodeList, AstSeparatedList,
    BatchMutationExt, SyntaxElement, TextRange, TriviaPiece, WalkEvent,
};
use rustc_hash::FxHashSet;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Require private members to be marked as `readonly` if they're never modified outside of the constructor
    ///
    /// Private member variables (whether using the private modifier or private # fields) are never permitted
    /// to be modified outside of their declaring class. If that class never modifies their value,
    /// they may safely be marked as readonly.
    ///
    /// This rule reports on private members and marks them as `readonly`
    /// if they're never modified outside of the constructor.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```typescript
    /// class Container {
    ///     // These member variables could be marked as readonly
    ///     private neverModifiedMember = true;
    ///     private onlyModifiedInConstructor: number;
    ///     #neverModifiedPrivateField = 3;
    ///
    ///     public constructor(
    ///         onlyModifiedInConstructor: number,
    ///         // Private parameter properties can also be marked as readonly
    ///         private neverModifiedParameter: string,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```typescript
    /// class Container {
    ///     private readonly neverModifiedMember = true;
    ///     private readonly onlyModifiedInConstructor: number;
    ///
    ///     // This is modified later on by the class
    ///     #modifiedLaterPrivateField = 'unchanged';
    ///
    ///     public constructor(
    ///         onlyModifiedInConstructor: number
    ///         private readonly neverModifiedParameter: string,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    ///
    ///     public mutatePrivateField() {
    ///         this.#modifiedLaterPrivateField = 'mutated';
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// This rule accepts the following options:
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "checkAllProperties": false
    ///     }
    /// }
    /// ```
    ///
    /// ### checkAllProperties
    ///
    /// Check on all properties (`public` and `protected` properties). Default: `false`.
    ///
    pub UseReadonlyClassProperties {
        version: "1.0.0",
        name: "useReadonlyClassProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-readonly")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyClassPropertiesLike = JsPropertyClassMember | TsPropertyParameter
}

impl Rule for UseReadonlyClassProperties {
    type Query = Ast<JsClassDeclaration>;
    type State = AnyClassPropertiesLike;
    type Signals = Box<[Self::State]>;
    type Options = ReadonlyClassPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class_declaration = ctx.query();
        let options = ctx.options();
        let eligible_properties =
            get_eligible_properties(class_declaration, !options.check_all_properties);
        if eligible_properties.is_empty() {
            Vec::new()
        } else {
            find_properties_need_add_readonly(class_declaration.syntax(), eligible_properties)
        }
        .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut name = String::new();
        if let Some(token) = state.text_trimmed() {
            name = token;
        }
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {
                "Member '"{name}"' is never reassigned; mark it as `readonly`."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation
            .replace_element_discard_trivia(state.syntax().clone().into(), state.replace_syntax());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add `readonly` decorator." }.to_owned(),
            mutation,
        ))
    }
}

/// Rule's options.
#[derive(
    Default, Debug, Clone, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReadonlyClassPropertiesOptions {
    /// If `true`, then check all class properties member
    ///
    /// If `false`, then only check private members
    #[serde(default, skip_serializing_if = "is_default")]
    pub check_all_properties: bool,
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

// fixed
fn get_constructor_eligible_params(
    class_declaration: &JsClassDeclaration,
    only_private: bool,
) -> Vec<AnyClassPropertiesLike> {
    let constructor_member =
        class_declaration
            .members()
            .into_iter()
            .find_map(|member| match member {
                AnyJsClassMember::JsConstructorClassMember(constructor) => Some(constructor),
                _ => None,
            });

    if let Some(constructor_member) = constructor_member {
        if let Ok(constructor_params) = constructor_member.parameters() {
            return constructor_params
                .parameters()
                .iter()
                .filter_map(|parameter| match parameter.ok()? {
                    AnyJsConstructorParameter::TsPropertyParameter(property_parameter) => {
                        let mut eligible = false;
                        let modifiers = property_parameter.modifiers();
                        modifiers.iter().for_each(|modifier| match modifier {
                            AnyTsPropertyParameterModifier::TsAccessibilityModifier(
                                accessibility_modifier,
                            ) => {
                                eligible = !only_private || accessibility_modifier.is_private();
                            }
                            AnyTsPropertyParameterModifier::TsReadonlyModifier(_) => {
                                eligible = false; // self has `readonly` modifier ignore it
                            }
                            _ => {}
                        });

                        eligible.then(|| property_parameter.into())
                    }
                    _ => None,
                })
                .collect();
        }
    }

    Vec::new()
}

// fixed
fn get_eligible_property(
    property_class_member: &JsPropertyClassMember,
    only_private: bool,
) -> bool {
    let modifiers = property_class_member.modifiers();

    if modifiers.is_empty() {
        if !only_private {
            return true;
        }
        let class_member_name = property_class_member.name();
        if let Ok(AnyJsClassMemberName::JsPrivateClassMemberName(_)) = class_member_name {
            return true;
        }
        return false;
    }

    let mut eligible = false;
    for modifier in modifiers.iter() {
        match modifier {
            AnyJsPropertyModifier::JsAccessorModifier(_)
            | AnyJsPropertyModifier::TsReadonlyModifier(_) => {
                eligible = false;
                break;
            }
            AnyJsPropertyModifier::TsAccessibilityModifier(accessibility_modifier) => {
                eligible = !only_private || accessibility_modifier.is_private();
            }
            _ => {}
        }
    }

    eligible
}

// fixed
fn get_eligible_properties(
    class_declaration: &JsClassDeclaration,
    only_private: bool,
) -> Vec<AnyClassPropertiesLike> {
    class_declaration
        .members()
        .iter()
        .filter_map(|class_member| match class_member {
            AnyJsClassMember::JsPropertyClassMember(property_class_member) => {
                let eligible = get_eligible_property(&property_class_member, only_private);
                eligible.then(|| property_class_member.into())
            }
            _ => None,
        })
        .chain(get_constructor_eligible_params(
            class_declaration,
            only_private,
        ))
        .collect()
}

// fixed
fn get_property_name(assignment: AnyJsAssignment) -> Option<String> {
    match assignment {
        AnyJsAssignment::JsStaticMemberAssignment(static_member_assignment) => {
            let member = static_member_assignment.member();
            if let Ok(member) = member {
                let value_token = member.value_token();
                if let Ok(value_token) = value_token {
                    let mut name = String::from(value_token.text_trimmed());
                    if member.as_js_private_name().is_some() {
                        name = format!("#{}", name);
                    }
                    return Some(name);
                }
            }
            None
        }
        _ => None,
    }
}

// maybe effect var
// 0. AccessExpression ?
// + / - = ?
// 1. BinaryExpression
// this.#app >>
// this.#app >>>
// 2. DeleteExpression
// delete this.#app
// 3. DestructuringAssignment
// - ObjectLiteralExpression
// - ArrayLiteralExpression
// - SpreadAssignment
// [...this.app] = newValue // => JsArrayAssignmentPattern
// [this.app] = newValue
// 4. PostfixUnaryExpression / PrefixUnaryExpression
// this.#app++ / ++this.#app
// consider return class ?
// in constructor has complex op  => (() => {})()
// object defined

fn find_params() {
    todo!();
}

// if function return on one and as this => pass
// else ignore it
// ignore as!!!!!!!!!

fn find_property_modify_in_function(statements: JsStatementList, knowns_refers: Vec<&str>) {
    let refers = knowns_refers;
    for statement in statements {
        match statement {
            AnyJsStatement::JsBlockStatement(block_statement) => {
                let next_statements = block_statement.statements();
                find_property_modify_in_function(next_statements, refers.clone());
            }
            AnyJsStatement::JsBogusStatement(_) => {} // ignore exp.function a }
            AnyJsStatement::JsBreakStatement(_) => {} // ignore
            AnyJsStatement::JsClassDeclaration(_class_declaration) => {
                // need test, maybe will check it by rules
            }
            AnyJsStatement::JsContinueStatement(_) => {} // ignore
            AnyJsStatement::JsDebuggerStatement(_) => {} // ignore
            AnyJsStatement::JsDoWhileStatement(do_while_statements) => {
                let js_statement = do_while_statements.body();
                if let Ok(js_statement) = js_statement {
                    let block_statement = js_statement.as_js_block_statement();
                    if let Some(block_statement) = block_statement {
                        let next_statements = block_statement.statements();
                        find_property_modify_in_function(next_statements, refers.clone());
                    }
                }
            }
            AnyJsStatement::JsEmptyStatement(_) => {} // ignore
            AnyJsStatement::JsExpressionStatement(expression_statement) =>  {
                let expression = expression_statement.expression();
                if let Ok(expression) = expression {
                    match expression {
                        AnyJsExpression::AnyJsLiteralExpression(any_js_literal_expression) => todo!(),
                        AnyJsExpression::JsArrayExpression(js_array_expression) => todo!(),
                        AnyJsExpression::JsArrowFunctionExpression(js_arrow_function_expression) => todo!(),
                        AnyJsExpression::JsAssignmentExpression(js_assignment_expression) => todo!(),
                        AnyJsExpression::JsAwaitExpression(js_await_expression) => todo!(),
                        AnyJsExpression::JsBinaryExpression(js_binary_expression) => todo!(),
                        AnyJsExpression::JsBogusExpression(js_bogus_expression) => todo!(),
                        AnyJsExpression::JsCallExpression(js_call_expression) => todo!(),
                        AnyJsExpression::JsClassExpression(js_class_expression) => todo!(),
                        AnyJsExpression::JsComputedMemberExpression(js_computed_member_expression) => todo!(),
                        AnyJsExpression::JsConditionalExpression(js_conditional_expression) => todo!(),
                        AnyJsExpression::JsFunctionExpression(js_function_expression) => todo!(),
                        AnyJsExpression::JsIdentifierExpression(js_identifier_expression) => todo!(),
                        AnyJsExpression::JsImportCallExpression(js_import_call_expression) => todo!(),
                        AnyJsExpression::JsImportMetaExpression(js_import_meta_expression) => todo!(),
                        AnyJsExpression::JsInExpression(js_in_expression) => todo!(),
                        AnyJsExpression::JsInstanceofExpression(js_instanceof_expression) => todo!(),
                        AnyJsExpression::JsLogicalExpression(js_logical_expression) => todo!(),
                        AnyJsExpression::JsMetavariable(js_metavariable) => todo!(),
                        AnyJsExpression::JsNewExpression(js_new_expression) => todo!(),
                        AnyJsExpression::JsNewTargetExpression(js_new_target_expression) => todo!(),
                        AnyJsExpression::JsObjectExpression(js_object_expression) => todo!(),
                        AnyJsExpression::JsParenthesizedExpression(js_parenthesized_expression) => todo!(),
                        AnyJsExpression::JsPostUpdateExpression(js_post_update_expression) => todo!(),
                        AnyJsExpression::JsPreUpdateExpression(js_pre_update_expression) => todo!(),
                        AnyJsExpression::JsSequenceExpression(js_sequence_expression) => todo!(),
                        AnyJsExpression::JsStaticMemberExpression(js_static_member_expression) => todo!(),
                        AnyJsExpression::JsSuperExpression(js_super_expression) => todo!(),
                        AnyJsExpression::JsTemplateExpression(js_template_expression) => todo!(),
                        AnyJsExpression::JsThisExpression(js_this_expression) => todo!(),
                        AnyJsExpression::JsUnaryExpression(js_unary_expression) => todo!(),
                        AnyJsExpression::JsYieldExpression(js_yield_expression) => todo!(),
                        AnyJsExpression::JsxTagExpression(jsx_tag_expression) => todo!(),
                        AnyJsExpression::TsAsExpression(ts_as_expression) => todo!(),
                        AnyJsExpression::TsInstantiationExpression(ts_instantiation_expression) => todo!(),
                        AnyJsExpression::TsNonNullAssertionExpression(ts_non_null_assertion_expression) => todo!(),
                        AnyJsExpression::TsSatisfiesExpression(ts_satisfies_expression) => todo!(),
                        AnyJsExpression::TsTypeAssertionExpression(ts_type_assertion_expression) => todo!(),
                    }
                }
            },
            AnyJsStatement::JsForInStatement(for_in_statement) => {
                let js_statement = for_in_statement.body();
                // TODO may AnyJsStatement can package
                // in this case
                // for item in x
                //      this.app = item
                // not in js_block_statement
                if let Ok(js_statement) = js_statement {
                    let block_statement = js_statement.as_js_block_statement();
                    if let Some(block_statement) = block_statement {
                        let next_statements: JsStatementList = block_statement.statements();
                        find_property_modify_in_function(next_statements, refers.clone());
                    }
                }
            },
            AnyJsStatement::JsForOfStatement(js_for_of_statement) => todo!(),
            AnyJsStatement::JsForStatement(js_for_statement) => todo!(),
            AnyJsStatement::JsFunctionDeclaration(js_function_declaration) => {
                // may update in js func
            },
            AnyJsStatement::JsIfStatement(js_if_statement) => todo!(),
            AnyJsStatement::JsLabeledStatement(js_labeled_statement) => todo!(),
            AnyJsStatement::JsMetavariable(js_metavariable) => todo!(),
            AnyJsStatement::JsReturnStatement(js_return_statement) => todo!(),
            AnyJsStatement::JsSwitchStatement(js_switch_statement) => todo!(),
            AnyJsStatement::JsThrowStatement(js_throw_statement) => todo!(),
            AnyJsStatement::JsTryFinallyStatement(js_try_finally_statement) => todo!(),
            AnyJsStatement::JsTryStatement(js_try_statement) => todo!(),
            AnyJsStatement::JsVariableStatement(js_variable_statement) => todo!(),
            AnyJsStatement::JsWhileStatement(js_while_statement) => todo!(),
            AnyJsStatement::JsWithStatement(js_with_statement) => todo!(),
            AnyJsStatement::TsDeclareFunctionDeclaration(ts_declare_function_declaration) => {
                todo!()
            }
            AnyJsStatement::TsDeclareStatement(ts_declare_statement) => todo!(),
            AnyJsStatement::TsEnumDeclaration(ts_enum_declaration) => todo!(),
            AnyJsStatement::TsExternalModuleDeclaration(ts_external_module_declaration) => todo!(),
            AnyJsStatement::TsGlobalDeclaration(ts_global_declaration) => todo!(),
            AnyJsStatement::TsImportEqualsDeclaration(ts_import_equals_declaration) => todo!(),
            AnyJsStatement::TsInterfaceDeclaration(ts_interface_declaration) => todo!(),
            AnyJsStatement::TsModuleDeclaration(ts_module_declaration) => todo!(),
            AnyJsStatement::TsTypeAliasDeclaration(ts_type_alias_declaration) => todo!(),
            _ => {}
        }
    }
}

fn find_method_member_change(method_class_member: &JsMethodClassMember) {
    let instances: Vec<&str> = vec!["this"];
    // TODO check params
    // let parameters = method_class_member.parameters();
    let fn_body = method_class_member.body();
    if let Ok(fn_body) = fn_body {
        let statements = fn_body.statements();
        for _statement in statements {}
    }
}

fn find_properties_need_add_readonly(
    syntax: &JsSyntaxNode,
    mut properties: Vec<AnyClassPropertiesLike>,
) -> Vec<AnyClassPropertiesLike> {
    let mut constructor_member = false;
    let mut changed_properties: FxHashSet<String> = FxHashSet::default();
    let events = syntax.preorder();

    for event in events {
        match event {
            WalkEvent::Enter(syntax_node) => match syntax_node.kind() {
                // check all param if has JsInitializerClause
                // and JsInitializerClause is JsParenthesizedExpression
                // TODO only check `JS_METHOD_CLASS_MEMBER` and `JS_CONSTRUCTOR_CLASS_MEMBER`
                // need context to check like `{} as this . `
                JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
                    constructor_member = false;
                    let method_member = JsMethodClassMember::unwrap_cast(syntax_node.clone());
                    find_method_member_change(&method_member);
                }
                JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => {
                    // TODO static member if chang in constructor
                    constructor_member = true;
                    let _constructor_member =
                        JsConstructorClassMember::unwrap_cast(syntax_node.clone());
                }
                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    if constructor_member {
                        continue; // skip constructor assignment
                    }
                    let assignment_expression =
                        JsAssignmentExpression::unwrap_cast(syntax_node.clone());
                    let assignment_pattern = assignment_expression.left();
                    if let Ok(AnyJsAssignmentPattern::AnyJsAssignment(assignment)) =
                        assignment_pattern
                    {
                        let name = get_property_name(assignment);
                        if let Some(name) = name {
                            changed_properties.insert(name);
                        }
                    }
                }
                _ => {}
            },
            WalkEvent::Leave(_) => {}
        }
    }

    if !changed_properties.is_empty() {
        properties.retain(|property| {
            let name = property.text_trimmed();
            if let Some(name) = name {
                return !changed_properties.contains(&name);
            }
            true
        });
    }

    properties.into_iter().collect()
}

fn get_replace_class_member_name(
    class_member_name: AnyJsClassMemberName,
) -> Option<(AnyJsClassMemberName, SyntaxTrivia<JsLanguage>)> {
    match class_member_name {
        AnyJsClassMemberName::JsComputedMemberName(_) => None,
        AnyJsClassMemberName::JsMetavariable(_) => None,
        AnyJsClassMemberName::JsLiteralMemberName(literal_member_name) => {
            let value_token = literal_member_name.value();
            if let Ok(value_token) = value_token {
                let leading = value_token.leading_trivia();
                let replace_value_token = value_token.with_leading_trivia([]);
                let replace_class_member_name = literal_member_name
                    .replace_token_discard_trivia(value_token, replace_value_token);
                if let Some(replace_class_member_name) = replace_class_member_name {
                    return Some((
                        AnyJsClassMemberName::JsLiteralMemberName(replace_class_member_name),
                        leading,
                    ));
                }
            }
            None
        }
        AnyJsClassMemberName::JsPrivateClassMemberName(private_class_member_name) => {
            let hash_token = private_class_member_name.hash_token();
            if let Ok(hash_token) = hash_token {
                let leading = hash_token.leading_trivia();
                let replace_hash_token = hash_token.with_leading_trivia([]);
                let replace_class_member_name = private_class_member_name
                    .replace_token_discard_trivia(hash_token, replace_hash_token);
                if let Some(replace_class_member_name) = replace_class_member_name {
                    return Some((
                        AnyJsClassMemberName::JsPrivateClassMemberName(replace_class_member_name),
                        leading,
                    ));
                }
            }
            None
        }
    }
}

impl AnyClassPropertiesLike {
    pub fn text_trimmed(&self) -> Option<String> {
        match self {
            AnyClassPropertiesLike::JsPropertyClassMember(property_class_member) => {
                let class_member_name = property_class_member.name();
                if let Ok(class_member_name) = class_member_name {
                    let member_name = class_member_name.name();
                    if let Some(member_name) = member_name {
                        let mut name = String::from(member_name.text());
                        if class_member_name
                            .as_js_private_class_member_name()
                            .is_some()
                        {
                            name = format!("#{}", name);
                        }
                        return Some(name);
                    }
                }
                None
            }
            AnyClassPropertiesLike::TsPropertyParameter(property_parameter) => {
                match property_parameter.formal_parameter().ok()? {
                    AnyJsFormalParameter::JsFormalParameter(formal_parameter) => Some(
                        formal_parameter
                            .binding()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()?
                            .name_token()
                            .ok()?
                            .text_trimmed()
                            .to_string(),
                    ),
                    _ => None,
                }
            }
        }
    }

    pub fn range(&self) -> Option<TextRange> {
        match self {
            AnyClassPropertiesLike::JsPropertyClassMember(property_class_member) => {
                Some(property_class_member.name().ok()?.range())
            }
            AnyClassPropertiesLike::TsPropertyParameter(property_parameter) => {
                match property_parameter.formal_parameter().ok()? {
                    AnyJsFormalParameter::JsBogusParameter(_)
                    | AnyJsFormalParameter::JsMetavariable(_) => None,
                    AnyJsFormalParameter::JsFormalParameter(param) => Some(
                        param
                            .binding()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()?
                            .range(),
                    ),
                }
            }
        }
    }

    pub fn replace_syntax(&self) -> SyntaxElement<JsLanguage> {
        match self {
            AnyClassPropertiesLike::JsPropertyClassMember(member) => {
                let class_member_name = member.name();
                if let Ok(class_member_name) = class_member_name {
                    let mut class_member_name = class_member_name;
                    let modifiers = member.modifiers();
                    let mut replace_modifiers: Vec<AnyJsPropertyModifier> = Vec::new();

                    let mut readonly_token = JsSyntaxToken::new_detached(
                        JsSyntaxKind::TS_READONLY_MODIFIER,
                        "readonly ",
                        [],
                        [TriviaPiece::whitespace(1)],
                    );
                    if modifiers.is_empty() {
                        let replace_items =
                            get_replace_class_member_name(class_member_name.clone());
                        if let Some((replace_class_member_name, replace_leading)) = replace_items {
                            class_member_name = replace_class_member_name;
                            let replace_readonly_token =
                                readonly_token.with_leading_trivia_pieces(replace_leading.pieces());
                            readonly_token = replace_readonly_token;
                        }
                    }

                    modifiers
                        .iter()
                        .for_each(|modifier| replace_modifiers.push(modifier));

                    replace_modifiers.push(AnyJsPropertyModifier::TsReadonlyModifier(
                        make::ts_readonly_modifier(readonly_token),
                    ));

                    let modifiers = make::js_property_modifier_list(replace_modifiers);
                    let mut builder = make::js_property_class_member(modifiers, class_member_name);

                    let property_annotation = member.property_annotation();
                    if let Some(property_annotation) = property_annotation {
                        builder = builder.with_property_annotation(property_annotation);
                    }
                    let semicolon_token = member.semicolon_token();
                    if let Some(semicolon_token) = semicolon_token {
                        builder = builder.with_semicolon_token(semicolon_token);
                    }
                    let value = member.value();
                    if let Some(value) = value {
                        builder = builder.with_value(value);
                    }

                    let replace_member = builder.build();
                    return SyntaxElement::Node(replace_member.into_syntax());
                }
            }
            AnyClassPropertiesLike::TsPropertyParameter(parameter) => {
                let formal_parameter = parameter.formal_parameter();
                if let Ok(formal_parameter) = formal_parameter {
                    let decorators = parameter.decorators();
                    let modifiers = parameter.modifiers();

                    let mut replace_modifiers: Vec<AnyTsPropertyParameterModifier> = Vec::new();
                    modifiers
                        .iter()
                        .for_each(|modifier| replace_modifiers.push(modifier));
                    replace_modifiers.push(AnyTsPropertyParameterModifier::TsReadonlyModifier(
                        make::ts_readonly_modifier(JsSyntaxToken::new_detached(
                            JsSyntaxKind::TS_READONLY_MODIFIER,
                            "readonly ",
                            [],
                            [TriviaPiece::whitespace(1)],
                        )),
                    ));

                    let modifiers = make::ts_property_parameter_modifier_list(replace_modifiers);
                    let replace_parameter =
                        make::ts_property_parameter(decorators, modifiers, formal_parameter);
                    return SyntaxElement::Node(replace_parameter.into_syntax());
                }
            }
        }
        self.syntax().clone().into()
    }
}
