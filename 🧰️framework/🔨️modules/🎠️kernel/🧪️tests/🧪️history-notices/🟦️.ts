/** 📢️ TypeScript twin of `🧪️tests/🧪️history-notices/🦀️.rs`, driven by the SAME fixture (`🧫️fixtures/🧫️history-notices/🔣️.json`):
 * Ajv validates it against its schema of record, hostile rows are refused, and the twin's {@link HISTORY_NOTICE_LABELS}
 * carry exactly its rows. */
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🧫️history-notices/🔣️.json";
import schema from "../../🧬️schema/🔣️history-notices/🔣️.json";
import { HISTORY_NOTICE_LABELS, historyNotice } from "../../🟦️.ts";

const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);

describe("history notices", () => {
  it("the notice catalog projection satisfies its domain schema and refuses hostile rows", () => {
    expect(validate({ notices: fixture.notices }), JSON.stringify(validate.errors)).toBe(true);
    const first = fixture.notices[0]!;
    expect(validate({ notices: [{ code: first.code, en: first.en }] })).toBe(false);
    expect(validate({ notices: [{ ...first, code: "tool transaction open" }] })).toBe(false);
  });

  it("the twin carries exactly the fixture rows and answers nothing for other codes", () => {
    expect(HISTORY_NOTICE_LABELS.map((row) => ({ ...row }))).toEqual(fixture.notices);
    for (const row of fixture.notices) expect(historyNotice(row.code)).toEqual({ en: row.en, de: row.de });
    expect(historyNotice("timeTravel.busy")).toBeUndefined();
  });
});
