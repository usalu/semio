/** 🏛️ TypeScript twin of `🧪️tests/🧪️framework-notices/🦀️.rs`, driven by the SAME fixture (`🧫️fixtures/🧫️framework-notices/🔣️.json`):
 * Ajv validates it against its schema of record, hostile rows are refused, the twin's {@link FRAMEWORK_FAULT_NOTICE_LABELS} carry
 * exactly its rows, and a guest fault under one of its codes resolves to its notice in both locales. */
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🧫️framework-notices/🔣️.json";
import schema from "../../🧬️schema/🔣️framework-notices/🔣️.json";
import { FRAMEWORK_FAULT_NOTICE_LABELS, faultNotice, frameworkFaultNotice, historyNotice } from "../../🟦️.ts";

const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);

describe("framework notices", () => {
  it("the notice catalog projection satisfies its domain schema and refuses hostile rows", () => {
    expect(validate({ notices: fixture.notices }), JSON.stringify(validate.errors)).toBe(true);
    const first = fixture.notices[0]!;
    expect(validate({ notices: [{ code: first.code, en: first.en }] })).toBe(false);
    expect(validate({ notices: [{ ...first, code: "drawing.gesture.owner" }] })).toBe(false);
  });

  it("the twin carries exactly the fixture rows, none of them a history-lane notice", () => {
    expect(FRAMEWORK_FAULT_NOTICE_LABELS.map((row) => ({ ...row }))).toEqual(fixture.notices);
    for (const row of fixture.notices) {
      expect(frameworkFaultNotice(row.code)).toEqual({ en: row.en, de: row.de });
      expect(historyNotice(row.code)).toBeUndefined();
    }
    expect(frameworkFaultNotice("history.full")).toBeUndefined();
  });

  it("a guest fault under a framework code resolves to its notice in both locales", () => {
    const row = fixture.notices.find((notice) => notice.code === "mutation.too-large")!;
    expect(faultNotice({ code: row.code }, [], "native", "en")).toEqual({ code: row.code, text: row.en });
    expect(faultNotice({ code: "plugin.sdk", causes: [{ code: row.code }] }, [], "reuse", "de")).toEqual({ code: row.code, text: row.de });
  });

  it("a channel mismatch names both versions in both locales and never shows an unfilled placeholder", () => {
    const row = fixture.notices.find((notice) => notice.code === "plugin.channel-mismatch")!;
    const params = { guest: "20", host: "21" };
    for (const locale of ["en", "de"] as const) expect(faultNotice({ code: row.code, params }, [], "native", locale)).toEqual({ code: row.code, text: row[locale].replace("{guest}", "20").replace("{host}", "21") });
    expect(faultNotice({ code: row.code }, [], "native", "en")).toBeNull();
  });
});
