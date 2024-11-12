class TestModifiableStatic {
	private static correctlyModifiableStatic = 7;
	a: number

	public constructor() {
		TestModifiableStatic.correctlyModifiableStatic += 1;
	}

	getStatic() {
		this.a = 1;
	}
}
