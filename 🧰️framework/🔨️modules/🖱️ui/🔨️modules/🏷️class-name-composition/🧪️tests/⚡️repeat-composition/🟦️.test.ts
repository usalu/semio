/** ⚡️ A class list composed before costs no pattern matching again: every render composes the same few thousand lists, so
 * classifying each token anew (a scan of every utility family) made one pointer press a long task. */
import { describe, expect, spyOn, test } from "bun:test";
import { cn } from "../../🟦️.ts";

describe("repeat composition", () => {
  test("composes an already composed list without testing a single pattern, with the same result", () => {
    const list = ["flex p-2 p-4 md:hover:text-sm text-[color:red] truncate", { "mt-1 mt-3": true }, ["gap-x-2 gap-4"]] as const;
    const first = cn(...list);
    const test = spyOn(RegExp.prototype, "test");
    const second = cn(...list);
    const tested = test.mock.calls.length;
    test.mockRestore();
    expect(second).toBe(first);
    expect(tested).toBe(0);
  });

  test("keeps a falsy input apart from the string that spells it", () => {
    expect(cn("a", false)).toBe("a");
    expect(cn("a", "false")).toBe("a false");
    expect(cn("a", false)).toBe("a");
  });

  test("stays correct past the bound of its caches, which empty rather than grow without end", () => {
    const expected = (index: number): string => `p-${index} text-sm`;
    for (let index = 0; index < 45_000; index++) expect(cn(`p-${index % 7} p-${index}`, "text-sm")).toBe(expected(index));
    const again = ["flex p-2 p-4", "gap-x-2 gap-4"] as const;
    expect(cn(...again)).toBe(cn(...again));
    expect(cn(...again)).toBe("flex p-4 gap-4");
    const test = spyOn(RegExp.prototype, "test");
    cn(...again);
    const tested = test.mock.calls.length;
    test.mockRestore();
    expect(tested).toBe(0);
  });
});
