export {};

declare module "bun:test" {
  /**
   * 🧪️ Test-side matcher typing aligned with the Jest contract: the expected value of an equality matcher is compared at run time, so it is not pinned to the literal type of the actual value (a literal `"stalled"` against a widened `string`, a plain object against a branded row).
   * @see https://bun.sh/docs/test/writing#matchers
   * @see ../../../../../../../../node_modules/bun-types/test.d.ts
   */
  interface Matchers<T> {
    toBe<X = unknown>(expected: X): void;
    toEqual<X = unknown>(expected: X): void;
    toStrictEqual<X = unknown>(expected: X): void;
  }
}
