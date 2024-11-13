class TestIncorrectlyModifiableStatic {
	private static incorrectlyModifiableStatic = 7;
	static #incorrectlyModifiableStatic = 7;
}

class TestIncorrectlyModifiableStaticArrow {
	private static incorrectlyModifiableStaticArrow = () => 7;
	static #incorrectlyModifiableStaticArrow = () => 7;
}

class TestIncorrectlyModifiableInline {
	private incorrectlyModifiableInline = 7;
	#incorrectlyModifiableInline = 7;

	public createConfusingChildClass() {
		return class {
			private incorrectlyModifiableInline = 7;
			#incorrectlyModifiableInline = 7;
		};
	}
}

class TestIncorrectlyModifiableDelayed {
	private incorrectlyModifiableDelayed = 7;
	#incorrectlyModifiableDelayed = 7;

	public constructor() {
		this.incorrectlyModifiableDelayed = 7;
		this.#incorrectlyModifiableDelayed = 7;
	}
}

class TestChildClassExpressionModifiable {
	private childClassExpressionModifiable = 7;
	#childClassExpressionModifiable = 7;

	public createConfusingChildClass() {
		return class {
			private childClassExpressionModifiable = 7;
			#childClassExpressionModifiable = 7;

			mutate() {
				this.childClassExpressionModifiable += 1;
				this.#childClassExpressionModifiable += 7;
			}
		};
	}
}

class TestIncorrectlyModifiablePostMinus {
	private incorrectlyModifiablePostMinus = 7;
	#incorrectlyModifiablePostMinus = 7;

	public mutate() {
		this.incorrectlyModifiablePostMinus - 1;
		this.#incorrectlyModifiablePostMinus - 1;
	}
}

class TestIncorrectlyModifiablePreMinus {
	private incorrectlyModifiablePreMinus = 7;
	#incorrectlyModifiablePreMinus = 7;

	public mutate() {
		-this.incorrectlyModifiablePreMinus;
		-this.#incorrectlyModifiablePreMinus;
	}
}

class TestOverlappingClassVariable {
	private overlappingClassVariable = 7;

	public workWithSimilarClass(other: SimilarClass) {
		other.overlappingClassVariable = 7;
	}
}

class TestIncorrectlyModifiableParameter {
	public constructor(private incorrectlyModifiableParameter = 7) {}
}

class TestCorrectlyNonInlineLambdas {
	private incorrectlyInlineLambda = () => 7;
}

function ClassWithName<TBase extends new (...args: any[]) => {}>(Base: TBase) {
	return class extends Base {
		private name: string;
		#name: string;
	};
}

class TestObjectMutate {
	private testObj = {
		prop: "",
	};
	#testObj = {
		prop: "",
	};

	public mutate(): void {
		this.testObj.prop = "";
		this.#testObj.prop = "";
	}
}

class TestClassInsMutate {
	private testObj = new TestObject();
	#testObj = new TestObject();

	public mutate(): void {
		this.testObj.prop = 10;
		this.#testObj.prop = 10;
	}
}

class TestOptionalChain {
	private testObj = {};
	#testObj = {};
	public getChain(): void {
		this.testObj?.prop;
		this.#testObj?.prop;
		this.testObj!.prop;
		this.#testObj!.prop;
	}
}

class TestObjectMethodCall {
	private testObj = {};
	#testObj = {};
	public doSomething(): void {
		this.testObj.prop.doesSomething();
		this.#testObj.prop.doesSomething();
	}
}

class TestInIntersection {
	private prop: number = 3;

	mutate() {
		const that = {} as this & { _foo: "bar" };
		that._foo = 1;
	}
}

class TestInIntersection1 {
	private prop: number = 3;

	test() {
		const that = {} as this | (this & { _foo: "bar" });
		that.prop;
	}
}

class TestInIntersectionInConstructor {
	private prop: number;

	constructor() {
		const that = {} as this & { _foo: "bar" };
		that.prop = 1;
	}
}
