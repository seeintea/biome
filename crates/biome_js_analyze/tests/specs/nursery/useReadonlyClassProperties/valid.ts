function ignore0() {}

const ignore1 = function () {};

const ignore2 = () => {};

const container0 = { member: true };
container0.member;

const container1 = { member: 1 };
+container1.member;

const container2 = { member: 1 };
++container2.member;

const container3 = { member: 1 };
container3.member++;

class TestEmpty {}

class TestReadonlyStatic {
	private static readonly correctlyReadonlyStatic = 7;
	static readonly #correctlyReadonlyStatic = 7;
}

class TestModifiableStatic {
	private static correctlyModifiableStatic = 7;
	static #correctlyModifiableStatic = 7;

	public constructor() {
		TestModifiableStatic.correctlyModifiableStatic += 1;
		TestModifiableStatic.#correctlyModifiableStatic += 1;
	}
}

class TestModifiableByParameterProperty {
	private static readonly correctlyModifiableByParameterProperty = 7;
	static readonly #correctlyModifiableByParameterProperty = 7;

	public constructor(
		public correctlyModifiablePublicParameter0: number = (() => {
			return (TestModifiableStatic.correctlyModifiableByParameterProperty += 1);
		})(),
		public correctlyModifiablePublicParameter1: number = (() => {
			return (TestModifiableStatic.#correctlyModifiableByParameterProperty += 1);
		})()
	) {}
}

class TestReadonlyInline {
	private readonly correctlyReadonlyInline = 7;
	readonly #correctlyReadonlyInline = 7;
}

class TestReadonlyDelayed {
	private readonly correctlyReadonlyDelayed = 7;
	readonly #correctlyReadonlyDelayed = 7;

	public constructor() {
		this.correctlyReadonlyDelayed += 1;
		this.#correctlyReadonlyDelayed += 1;
	}
}

class TestModifiableInline {
	private correctlyModifiableInline = 7;
	#correctlyModifiableInline = 7;

	public mutate() {
		this.correctlyModifiableInline += 1;
		this.#correctlyModifiableInline += 1;

		return class {
			private correctlyModifiableInline = 7;
			#correctlyModifiableInline = 7;

			mutate() {
				this.correctlyModifiableInline += 1;
				this.#correctlyModifiableInline += 1;
			}
		};
	}
}

class TestModifiableDelayed {
	private correctlyModifiableDelayed = 7;
	#correctlyModifiableDelayed = 7;

	public mutate() {
		this.correctlyModifiableDelayed += 1;
		this.#correctlyModifiableDelayed += 1;
	}
}

class TestModifiableDeleted {
	private correctlyModifiableDeleted = 7;

	public mutate() {
		delete this.correctlyModifiableDeleted;
	}
}

class TestModifiableWithinConstructor {
	private correctlyModifiableWithinConstructor = 7;
	#correctlyModifiableWithinConstructor = 7;

	public constructor() {
		(() => {
			this.correctlyModifiableWithinConstructor += 1;
			this.#correctlyModifiableWithinConstructor += 1;
		})();
	}
}

class TestModifiableWithinConstructorArrowFunction {
	private correctlyModifiableWithinConstructorArrowFunction = 7;
	#correctlyModifiableWithinConstructorArrowFunction = 7;

	public constructor() {
		(() => {
			this.correctlyModifiableWithinConstructorArrowFunction += 1;
			this.#correctlyModifiableWithinConstructorArrowFunction += 1;
		})();
	}
}

class TestModifiableWithinConstructorInFunctionExpression {
	private correctlyModifiableWithinConstructorInFunctionExpression = 7;
	#correctlyModifiableWithinConstructorInFunctionExpression = 7;

	public constructor() {
		const self = this;

		(() => {
			self.correctlyModifiableWithinConstructorInFunctionExpression += 1;
			self.#correctlyModifiableWithinConstructorInFunctionExpression += 1;
		})();
	}
}

class TestModifiableWithinConstructorInGetAccessor {
	private correctlyModifiableWithinConstructorInGetAccessor = 7;
	#correctlyModifiableWithinConstructorInGetAccessor = 7;

	public constructor() {
		const self = this;

		const confusingObject = {
			get accessor0() {
				return (self.correctlyModifiableWithinConstructorInGetAccessor += 1);
			},
			get accessor1() {
				return (self.#correctlyModifiableWithinConstructorInGetAccessor = 1);
			},
		};
	}
}

class TestModifiableWithinConstructorInMethodDeclaration {
	private correctlyModifiableWithinConstructorInMethodDeclaration = 7;
	#correctlyModifiableWithinConstructorInMethodDeclaration = 7;

	public constructor() {
		const self = this;

		const confusingObject = {
			methodDeclaration0() {
				self.correctlyModifiableWithinConstructorInMethodDeclaration = 7;
			},
			methodDeclaration1() {
				self.#correctlyModifiableWithinConstructorInMethodDeclaration = 7;
			},
		};
	}
}

class TestModifiableWithinConstructorInSetAccessor {
	private correctlyModifiableWithinConstructorInSetAccessor = 7;
	#correctlyModifiableWithinConstructorInSetAccessor = 7;

	public constructor() {
		const self = this;

		const confusingObject = {
			set accessor0(value: number) {
				self.correctlyModifiableWithinConstructorInSetAccessor += value;
			},
			set accessor1(value: number) {
				self.#correctlyModifiableWithinConstructorInSetAccessor += value;
			},
		};
	}
}

class TestModifiablePostDecremented {
	private correctlyModifiablePostDecremented = 7;
	#correctlyModifiablePostDecremented = 7;

	public mutate() {
		this.correctlyModifiablePostDecremented -= 1;
		this.#correctlyModifiablePostDecremented -= 1;
	}
}

class TestModifiablePreDecremented {
	private correctlyModifiablePreDecremented = 7;
	#correctlyModifiablePreDecremented = 7;

	public mutate() {
		--this.correctlyModifiablePreDecremented;
		++this.#correctlyModifiablePreDecremented;
	}
}

class TestPublicModifiable {
	public publicModifiable = 7;
}

class TestProtectedModifiable {
	protected protectedModifiable = 7;
}

class TestReadonlyParameter {
	public constructor(private readonly correctlyReadonlyParameter = 7) {}
}

class TestCorrectlyModifiableParameter {
	public constructor(private correctlyModifiableParameter = 7) {}

	public mutate() {
		this.correctlyModifiableParameter += 1;
	}
}

class TestComputedParameter {
	private ["computed-ignored-by-rule"] = 1;

	public mutate() {
		this["computed"] = 1;
	}
}

class TestCorrectlyModifiableWithinObject {
	private value: number = 0;
	#value: number = 0;

	mutate0(newValue: { value: number }) {
		({ value: this.value } = newValue);
		return this.value;
	}

	mutate1(newValue: { value: number }) {
		({ value: this.#value } = newValue);
		return this.#value;
	}
}

function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		private name: string;
		#name: string;

		public mutate(value: string) {
			this.name = value;
			this.#name = value;
		}
	};
}

class TestCorrectlyModifiableWithinObjectSpread {
	private value: Record<string, number> = {};
	#value: Record<string, number> = {};

	mutate0(newValue: Record<string, number>) {
		({ ...this.value } = newValue);
		return this.value;
	}

	mutate1(newValue: Record<string, number>) {
		({ ...this.#value } = newValue);
		return this.#value;
	}
}

class TestCorrectlyModifiableWithinArray {
	private value: number = 0;
	#value: number = 0;

	mutate0(newValue: number[]) {
		[this.value] = newValue;
		return this.value;
	}

	mutate1(newValue: number[]) {
		[this.#value] = newValue;
		return this.#value;
	}
}

class TestCorrectlyModifiableWithinArraySpread {
	private value: number[] = [];
	#value: number[] = [];

	mutate0(newValue: number[]) {
		[...this.value] = newValue;
		return this.value;
	}

	mutate1(newValue: number[]) {
		[...this.#value] = newValue;
		return this.#value;
	}
}

class TestCorrectlyModifiableWithinObjectReassignment {
	private testObj = {
		prop: "",
	};
	#testObj = {
		prop: "",
	};

	public mutate0(): void {
		this.testObj = "";
	}

	public mutate1(): void {
		this.#testObj = "";
	}
}

class TestCorrectlyModifiableWithinClassInstance {
	private instance = new ClassInstance();
	#instance = new ClassInstance();

	public mutate(): void {
		this.instance = new ClassInstance();
		this.#instance = new ClassInstance();
	}
}

class TestCorrectlyIntersection {
	private prop: number = 3;
	#prop: number = 3;

	mutate() {
		const that = {} as this & { _foo: "bar" };
		that.prop = 1;
		that.#prop = 2;
	}
}

class TestCorrectlyInUnion {
	private prop: number = 3;
	#prop: number = 3;

	mutate() {
		const that = {} as this | (this & { _foo: "bar" });
		that.prop = 1;
		that.#prop = 2;
	}
}

class TestCorrectlyInStaticIntersection {
	private static prop: number;
	static #prop: number;

	mutate() {
		const that = {} as typeof TestCorrectlyInStaticIntersection & {
			_foo: "bar";
		};
		that.prop = 1;
		that.#prop = 2;
	}
}

class TestCorrectlyInTestStaticUnion {
	private static prop: number = 1;
	static #prop: number = 1;

	mutate() {
		const that = {} as
			| typeof TestCorrectlyInTestStaticUnion
			| (typeof TestCorrectlyInTestStaticUnion & { _foo: "bar" });
		that.prop = 1;
		that.#prop = 2;
	}
}

class TestCorrectlyBothIntersection {
	private prop1: number = 1;
	private static prop2: number;
	#prop1: number = 1;
	static #prop2: number;

	mutate() {
		const that = {} as typeof TestCorrectlyBothIntersection & this;
		that.prop1 = 1;
		that.prop2 = 1;
		that.#prop1 = 1;
		that.#prop2 = 1;
	}
}

class TestCorrectlyBothIntersection1 {
	private prop1: number = 1;
	private static prop2: number;
	#prop1: number = 1;
	static #prop2: number;

	mutate() {
		const that = {} as this & typeof TestCorrectlyBothIntersection1;
		that.prop1 = 1;
		that.prop2 = 1;
		that.#prop1 = 1;
		that.#prop2 = 1;
	}
}

class TestStaticPrivateAccessor {
	private static accessor staticAcc = 1;
	static accessor #staticAcc = 1;
}

class TestPrivateAccessor {
	private accessor acc = 3;
	accessor #acc = 1;
}
