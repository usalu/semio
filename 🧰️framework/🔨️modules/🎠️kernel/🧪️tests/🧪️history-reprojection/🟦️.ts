/** 📡️ TypeScript twin of `🧪️tests/🧪️history-reprojection/🦀️.rs`, driven by the SAME fixture
 * (`🧫️fixtures/🧫️history-reprojection/🔣️.json`): Ajv validates it against its schema of record, hostile rows are refused, the
 * twin's {@link HISTORY_REPROJECTION_LABELS} carry exactly its rows and {@link historyReprojectionStatus} reads every case to
 * its status in both locales. */
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { createInstance } from "i18next";
import fixture from "../../🧫️fixtures/🧫️history-reprojection/🔣️.json";
import schema from "../../🧬️schema/🔣️history-patch/🔣️.json";
import { HISTORY_REPROJECTION_LABELS, historyReprojectionStatus, type HistoryReprojection } from "../../🟦️.ts";

describe("history reprojection status", () => {
  it("replay steps include final settlement and match the independent translation oracle", async () => {
    const row = fixture.cases.find(row => row.name === "operation complete while edit settlement remains")!;
    for (const locale of ["en", "de"] as const) {
      const oracle = createInstance();
      await oracle.init({ lng: locale, fallbackLng: false, keySeparator: false, resources: { [locale]: { translation: Object.fromEntries(fixture.labels.map(label => [label.key, label[locale].replace(/\{(\w+)\}/gu, "{{$1}}")])) } } });
      const expected = `${oracle.t("step.progress", row.reprojection)} · ${oracle.t("work.processed", row.reprojection)}`;
      expect(expected).toBe(row.text[locale]);
      expect(historyReprojectionStatus(row.reprojection as HistoryReprojection, "native", locale).text).toBe(expected);
      expect(row.reprojection.done).toBeLessThan(row.reprojection.total);
      console.log(`[DEBUG] replay settlement caption locale=${locale} done=${row.reprojection.done} total=${row.reprojection.total} text=${expected}`);
    }
  });

  it("the fixture satisfies its schema and refuses hostile rows", () => {
    const first = fixture.cases[0]!;
    const ajv = new Ajv({ allErrors: true, strict: false });
    ajv.addSchema(schema);
    const validate = ajv.compile({ $ref: `${schema.$id}#/definitions/HistoryReprojection` });
    for (const row of fixture.cases) expect(validate(row.reprojection), JSON.stringify(validate.errors)).toBe(true);
    for (const processed of [-1, 4294967296, "128"]) expect(validate({ ...first.reprojection, processed })).toBe(false);
    expect(validate({ ...first.reprojection, rawCursor: {} })).toBe(false);
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
