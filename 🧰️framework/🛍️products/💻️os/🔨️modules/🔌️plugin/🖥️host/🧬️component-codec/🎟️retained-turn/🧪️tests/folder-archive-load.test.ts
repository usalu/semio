import { expect, test } from "bun:test";
import law from "./folder-archive-load.json";

test("the folder archive load fixture is closed over instance and request bytes", () => {
  for (const row of law.cases) {
    expect(Object.keys(row).sort()).toEqual(["instance", "request"]);
    expect(Number.isInteger(row.instance) && row.instance > 0 && row.instance <= 0xffffffff).toBe(true);
    expect(row.request.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255)).toBe(true);
  }
});
