class TestIncorrectlyModifiableStatic {
	private static incorrectlyModifiableStatic = 7;
}

class TestIncorrectlyModifiableStatic {
	static #incorrectlyModifiableStatic = 7;
}

class TestIncorrectlyModifiableStaticArrow {
	private static incorrectlyModifiableStaticArrow = () => 7;
}

class TestIncorrectlyModifiableStaticArrow {
	static #incorrectlyModifiableStaticArrow = () => 7;
}

class TestIncorrectlyModifiableInline {
	private incorrectlyModifiableInline = 7;

	public createConfusingChildClass() {
		return class {
			private incorrectlyModifiableInline = 7;
		};
	}
}

class TestIncorrectlyModifiableInline {
	#incorrectlyModifiableInline = 7;

	public createConfusingChildClass() {
		return class {
			#incorrectlyModifiableInline = 7;
		};
	}
}

class TestIncorrectlyModifiableDelayed {
	private incorrectlyModifiableDelayed = 7;

	public constructor() {
		this.incorrectlyModifiableDelayed = 7;
	}
}

class TestIncorrectlyModifiableDelayed {
	#incorrectlyModifiableDelayed = 7;

	public constructor() {
		this.#incorrectlyModifiableDelayed = 7;
	}
}

class TestChildClassExpressionModifiable {
	private childClassExpressionModifiable = 7;

	public createConfusingChildClass() {
		return class {
			private childClassExpressionModifiable = 7;

			mutate() {
				this.childClassExpressionModifiable += 1;
			}
		};
	}
}

class TestChildClassExpressionModifiable {
	#childClassExpressionModifiable = 7;

	public createConfusingChildClass() {
		return class {
			#childClassExpressionModifiable = 7;

			mutate() {
				this.#childClassExpressionModifiable += 1;
			}
		};
	}
}

class TestIncorrectlyModifiablePostMinus {
	private incorrectlyModifiablePostMinus = 7;

	public mutate() {
		this.incorrectlyModifiablePostMinus - 1;
	}
}

class TestIncorrectlyModifiablePostMinus {
	#incorrectlyModifiablePostMinus = 7;

	public mutate() {
		this.#incorrectlyModifiablePostMinus - 1;
	}
}

class TestIncorrectlyModifiablePostPlus {
	private incorrectlyModifiablePostPlus = 7;

	public mutate() {
		this.incorrectlyModifiablePostPlus + 1;
	}
}

class TestIncorrectlyModifiablePostPlus {
	#incorrectlyModifiablePostPlus = 7;

	public mutate() {
		this.#incorrectlyModifiablePostPlus + 1;
	}
}

class TestIncorrectlyModifiablePreMinus {
	private incorrectlyModifiablePreMinus = 7;

	public mutate() {
		-this.incorrectlyModifiablePreMinus;
	}
}

class TestIncorrectlyModifiablePreMinus {
	#incorrectlyModifiablePreMinus = 7;

	public mutate() {
		-this.#incorrectlyModifiablePreMinus;
	}
}

class TestIncorrectlyModifiablePrePlus {
	private incorrectlyModifiablePrePlus = 7;

	public mutate() {
		+this.incorrectlyModifiablePrePlus;
	}
}

class TestIncorrectlyModifiablePrePlus {
	#incorrectlyModifiablePrePlus = 7;

	public mutate() {
		+this.#incorrectlyModifiablePrePlus;
	}
}

/* 1 */
class TestOverlappingClassVariable {
	private overlappingClassVariable = 7;

	public workWithSimilarClass(other: SimilarClass) {
		other.overlappingClassVariable = 7;
	}
}

class SimilarClass {
	public overlappingClassVariable = 7;
}
/* 1 */

class TestIncorrectlyModifiableParameter {
	public constructor(private incorrectlyModifiableParameter = 7) {}
}

class TestIncorrectlyModifiableParameter {
	public constructor(
		public ignore: boolean,
		private incorrectlyModifiableParameter = 7
	) {}
}

class TestCorrectlyNonInlineLambdas {
	private incorrectlyInlineLambda = () => 7;
}

function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		private _name: string;
	};
}

function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		#name: string;
	};
}

class Test {
	private testObj = {
		prop: "",
	};

	public test(): void {
		this.testObj.prop = "";
	}
}

class Test {
	#testObj = {
		prop: "",
	};

	public test(): void {
		this.#testObj.prop = "";
	}
}

/* 2 */
class TestObject {
	public prop: number;
}

class Test {
	private testObj = new TestObject();

	public test(): void {
		this.testObj.prop = 10;
	}
}
/* 2 */

class Test {
	private testObj = {
		prop: "",
	};
	public test(): void {
		this.testObj.prop;
	}
}

class Test {
	#testObj = {
		prop: "",
	};
	public test(): void {
		this.#testObj.prop;
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj?.prop;
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj?.prop;
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj!.prop;
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj!.prop;
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj.prop.prop = "";
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj.prop.prop = "";
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj.prop.doesSomething();
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj.prop.doesSomething();
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj?.prop.prop;
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj?.prop.prop;
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj?.prop?.prop;
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj?.prop?.prop;
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj.prop?.prop;
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj.prop?.prop;
	}
}

class Test {
	private testObj = {};
	public test(): void {
		this.testObj!.prop?.prop;
	}
}

class Test {
	#testObj = {};
	public test(): void {
		this.#testObj!.prop?.prop;
	}
}

class Test {
	private prop: number = 3;

	test() {
		const that = {} as this & { _foo: "bar" };
		that._foo = 1;
	}
}

class Test {
	private prop: number = 3;

	test() {
		const that = {} as this | (this & { _foo: "bar" });
		that.prop;
	}
}

class Test {
	private prop: number;

	constructor() {
		const that = {} as this & { _foo: "bar" };
		that.prop = 1;
	}
}
