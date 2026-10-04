/** 📢️ App fault notices over `🧫️fixtures/🧫️fault-notices/🔣️.json` (design §20.12) — TypeScript twin of the Rust laws beside this
 * file (`manifest::validate_fault_notices`, `kernel::fault_notice`); oracles: Ajv (strict, semio vocabulary) gives the schema-only
 * verdict of every table and checks the corpus itself (`$defs.FaultNoticeCorpus`), i18next (`{name}` interpolation) fills every resolved notice. */
import { describe, expect, test } from "bun:test";
import i18next from "i18next";
import { semioSchemaAjvV1 } from "../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import toolRunSchema from "../../../⏯️tool-run/🧬️schema/🔣️.json";
import { FRAMEWORK_FAULT_NOTICE_LABELS, faultNotice, HISTORY_NOTICE_LABELS } from "../../../🎠️kernel/🟦️.ts";
import fixture from "../../🧫️fixtures/🧫️fault-notices/🔣️.json";
import { SHELL_LOCALES, SHELL_TERMINOLOGIES, faultNoticePlaceholders, fillFaultNotice, isFaultNoticeCode, validateFaultNotices, type FaultNoticeDefinition, type FaultNoticeError, type ShellLocale, type ShellTerminology } from "../../🟦️.ts";

const MANIFEST_SCHEMA = "https://json.schemas.assets.semio-tech.com/framework/manifest/schema.json";
const ajv = semioSchemaAjvV1({ allErrors: true });
ajv.addSchema(toolRunSchema);
const validateTable = ajv.getSchema(`${MANIFEST_SCHEMA}#/$defs/FaultNoticeTable`)!;
const tables = fixture.tables as unknown as readonly { readonly id: string; readonly schemaValid: boolean; readonly notices: readonly FaultNoticeDefinition[]; readonly errors: readonly FaultNoticeError[] }[];
const appNotices = tables.find((table) => table.id === fixture.appNotices)!.notices;

/** 🌍️ One i18next instance per terminology namespace whose resources are the framework rows and the app notices. */
const oracle = i18next.createInstance();
await oracle.init({
  initAsync: false,
  showSupportNotice: false,
  lng: "en",
  supportedLngs: [...SHELL_LOCALES],
  ns: [...SHELL_TERMINOLOGIES],
  keySeparator: false,
  nsSeparator: false,
  interpolation: { prefix: "{", suffix: "}", escapeValue: false },
  resources: Object.fromEntries(SHELL_LOCALES.map((locale) => [locale, Object.fromEntries(SHELL_TERMINOLOGIES.map((terminology) => [terminology, Object.fromEntries([...HISTORY_NOTICE_LABELS.map((row) => [row.code, row[locale]]), ...FRAMEWORK_FAULT_NOTICE_LABELS.map((row) => [row.code, row[locale]]), ...appNotices.map((notice) => [notice.code, notice.label[terminology][locale]])])]))])),
});

const translate = oracle.t.bind(oracle) as unknown as (key: string, options: Readonly<Record<string, unknown>>) => string;

describe("📢️ app fault notices", () => {
  test("the corpus validates against its schema", () => {
    const validate = ajv.getSchema(`${MANIFEST_SCHEMA}#/$defs/FaultNoticeCorpus`)!;
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  });

  test("every table answers exactly its corpus refusals", () => {
    for (const table of tables) expect(validateFaultNotices(table.notices), table.id).toEqual([...table.errors]);
  });

  test("Ajv's schema-only verdict matches every table's schemaValid", () => {
    for (const table of tables) expect(validateTable(table.notices), table.id).toBe(table.schemaValid);
  });

  test("every fault resolves to its corpus notice, framework first, and i18next fills the same text", () => {
    for (const row of fixture.resolutions) {
      const fault = row.fault as { readonly code: string; readonly causes?: readonly { readonly code: string }[]; readonly params?: Readonly<Record<string, string>> };
      const terminology = row.terminology as ShellTerminology;
      const locale = row.locale as ShellLocale;
      const actual = faultNotice(fault, appNotices, terminology, locale);
      expect(actual, row.id).toEqual(row.expected);
      if (actual !== null) expect(translate(actual.code, { lng: locale, ns: terminology, replace: fault.params ?? {} }), `${row.id} (i18next)`).toBe(actual.text);
    }
  });

  test("a notice fills only named params and is never half-filled", () => {
    expect(fillFaultNotice("Die Widget-Art {kind} fehlt.", { kind: "brep.mesh.translate" })).toBe("Die Widget-Art brep.mesh.translate fehlt.");
    expect(fillFaultNotice("Needs {kind}.", undefined)).toBeNull();
    expect(fillFaultNotice("Needs {count}.", { kind: "x" })).toBeNull();
    expect(faultNoticePlaceholders("{a} and {b} and {a}")).toEqual(["a", "b"]);
    expect(faultNoticePlaceholders("{a")).toBeNull();
    expect([isFaultNoticeCode("generation3d.gumball.mesh-missing"), isFaultNoticeCode("history.full"), isFaultNoticeCode("app.command.Rejected")]).toEqual([true, false, false]);
  });
});
