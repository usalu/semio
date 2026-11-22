import * as bun from "bun:test";

bun.test("the passing fixture passes", () => {
  bun.expect(1 + 1).toBe(2);
});
