// #region 🧲️Header
/** 📣️ What the React shell tells a person whose dispatch an app refused (design §20.12): the neutral corpus
 * `🛂️manifest/🧫️fixtures/🧫️fault-notices` drives `appFaultNoticeV1` over the app's published `faultNotices` in the shell's own
 * language and terminology — framework codes first, `{name}` from `Fault.params` only, the fault's own severity — and a code no
 * table declares yields no notice (the generic refusal stays; a raw code never reaches the person). Ajv (third party) admits every
 * corpus fault against the diagnostic `Fault` schema. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterAll, describe, expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020.js";
import type { FaultNoticeDefinition } from "@semio-tech/framework";
import { appFaultNoticeV1, syncShellLabelLocale } from "../../🟦️.tsx";
import corpus from "../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🧫️fault-notices/🔣️.json";
import faultSchema from "../../../../../../../../../🔨️modules/⚠️diagnostic/🧬️schema/🎛️controlled/🔣️.json";
import valueSchema from "../../../../../../../../../🔨️modules/🌱️value/⚠️refusal/🧬️schema/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Laws
type Resolution = { readonly id: string; readonly fault: { readonly code: string; readonly message?: string; readonly params?: Readonly<Record<string, string>>; readonly causes?: readonly { readonly code: string }[] }; readonly terminology: string; readonly locale: "en" | "de"; readonly expected: { readonly code: string; readonly text: string } | null };
const tables = corpus.tables as unknown as readonly { readonly id: string; readonly notices: readonly FaultNoticeDefinition[] }[];
const app = { faultNotices: tables.find((table) => table.id === corpus.appNotices)!.notices };

describe("📣️ app fault notices reach the person in the shell's language, never as a raw code", () => {
  afterAll(() => syncShellLabelLocale("en"));

  it("tells every corpus refusal with the app's own words, the framework's first, and the fault's own severity", () => {
    for (const severity of ["warning", "error"] as const) {
      for (const row of corpus.resolutions as unknown as readonly Resolution[]) {
        const fault = { origin: "app", severity, message: row.fault.message ?? "", retryable: false, code: row.fault.code, ...(row.fault.params === undefined ? {} : { params: row.fault.params }), ...(row.fault.causes === undefined ? {} : { causes: row.fault.causes.map((cause) => ({ message: "", code: cause.code })) }) };
        syncShellLabelLocale(row.locale);
        expect(appFaultNoticeV1(fault, app, row.terminology), `${row.id} (${severity})`).toEqual(row.expected === null ? null : { ...row.expected, kind: severity });
      }
    }
  });

  it("admits every corpus fault through the diagnostic Fault schema (Ajv, third party)", () => {
    const ajv = new Ajv2020({ strict: true }).addSchema(valueSchema).addSchema(faultSchema);
    const validate = ajv.getSchema(`${faultSchema.$id}#/$defs/Fault`)!;
    for (const row of corpus.resolutions as unknown as readonly Resolution[]) {
      const fault = { origin: "app", severity: "error", message: row.fault.message ?? "", code: row.fault.code, ...(row.fault.params === undefined ? {} : { params: row.fault.params }) };
      expect(validate(fault), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);
    }
  });

  it("declares nothing for an app without notices and an unknown terminology reads the native row", () => {
    syncShellLabelLocale("de");
    expect(appFaultNoticeV1({ code: "generation3d.gumball.mesh-missing" }, undefined, "native")).toBeNull();
    expect(appFaultNoticeV1({ code: "generation3d.gumball.mesh-missing" }, { faultNotices: [] }, "native")).toBeNull();
    expect(appFaultNoticeV1({ code: "generation3d.gumball.no-shape-source" }, app, "unknown")?.text).toBe("Ein Widget auswählen, das eine Form erzeugt.");
  });
});
//#endregion 🧪️Laws
