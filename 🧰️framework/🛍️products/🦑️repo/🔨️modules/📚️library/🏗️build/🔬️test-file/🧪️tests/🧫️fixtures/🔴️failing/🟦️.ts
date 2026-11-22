import * as bun from "bun:test";

bun.test("the failing fixture fails on purpose", () => {
  bun.expect(1 + 1).toBe(3);
});
