import { describe, expect, it } from "vitest";
import fixture from "./🧫️fixtures/🔣️.json";
import { naturalFileNameV1, naturalFileSaveNameV1 } from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";

const at = new Date(Date.UTC(2026, 10, 20, 9, 8, 7));

describe("saved natural file names keep the original file name", () => {
  for (const row of fixture.cases) {
    it(row.id, () => {
      expect(naturalFileSaveNameV1(row.original ?? undefined, row.appId, row.extension, at)).toBe(row.expected);
    });
  }

  it("falls back to the timestamped kind name only without an original", () => {
    expect(naturalFileNameV1("s.stdio.xlsx@ecma-376/*#editor", ".xlsx", at)).toBe("xlsx-20261120T090807Z.xlsx");
  });
});
