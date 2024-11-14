class SingleItemTest {
	private single: number;

	constructor(
		private next: number = (() => {
			this.single += 1;
			return this.single;
		})()
	) {}

	mutate() {
		const that = {} as this & { _foo: 2 };
		that.single = 2;
		this.single += 1;
		this.single++;
		-this.single;
		this.single;
		this.single >>> 1;
		this.single & 1;
	}
}
