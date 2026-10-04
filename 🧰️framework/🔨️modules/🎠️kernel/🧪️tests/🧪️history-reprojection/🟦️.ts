/** 📡️ TypeScript twin of `🧪️tests/🧪️history-reprojection/🦀️.rs`, driven by the SAME fixture
 * (`🧫️fixtures/🧫️history-reprojection/🔣️.json`): Ajv validates it against its schema of record, hostile rows are refused, the
 * twin's {@link HISTORY_REPROJECTION_LABELS} carry exactly its rows and {@link historyReprojectionStatus} reads every case to
 * its status in both locales. */
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🧫️history-reprojection/🔣️.json";
import schema from "../../🧬️schema/🔣️history-reprojection/🔣️.json";
import { HISTORY_REPROJECTION_LABELS, historyReprojectionStatus, type HistoryReprojection } from "../../🟦️.ts";

const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);

describe("history reprojection status", () => {
  it("the fixture satisfies its schema and refuses hostile rows", () => {
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const first = fixture.cases[0]!;
    expect(validate({ ...fixture, labels: [{ key: "remote.title", en: "Remote history change" }] })).toBe(false);
    expect(validate({ ...fixture, labels: [{ key: "remote title", en: "a", de: "b" }] })).toBe(false);
    expect(validate({ ...fixture, cases: [{ ...first, reprojection: { done: 1, total: 2, kind: "sideways" } }] })).toBe(false);
    expect(validate({ ...fixture, cases: [{ ...first, reprojection: { done: -1, total: 2 } }] })).toBe(false);
    expect(validate({ ...fixture, cases: [{ ...first, text: { en: first.text.en } }] })).toBe(false);
  });

  it("the twin carries exactly the fixture rows", () => {
    expect(HISTORY_REPROJECTION_LABELS.map((row) => ({ ...row }))).toEqual(fixture.labels);
  });

  it("every case reads to its status in both locales and never shows its raw code", () => {
    expect(fixture.cases.length).toBeGreaterThanOrEqual(11);
    for (const testCase of fixture.cases) {
      const reprojection = testCase.reprojection as HistoryReprojection;
      for (const locale of ["en", "de"] as const) {
        const status = historyReprojectionStatus(reprojection, "native", locale);
        expect(status, `${testCase.name} (${locale})`).toEqual({ title: testCase.title[locale], text: testCase.text[locale], done: reprojection.done, total: reprojection.total, paused: testCase.paused, fault: testCase.fault });
        if (status.fault !== null) expect(status.text).not.toContain(status.fault);
        expect(historyReprojectionStatus(reprojection, "reuse", locale)).toEqual(status);
      }
    }
  });
});
