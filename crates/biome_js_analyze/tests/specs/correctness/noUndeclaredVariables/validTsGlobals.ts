/* should not generate diagnostics */
type B<T> = PromiseLike<T>;
type U<T extends string> = Uppercase<T>;
