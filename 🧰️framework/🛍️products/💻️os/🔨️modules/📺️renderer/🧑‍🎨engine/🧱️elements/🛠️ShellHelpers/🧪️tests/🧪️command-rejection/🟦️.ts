// #region 🧲️Header
/** ⚔️ What a human is told when one of their command batches is refused (🎫️ 26/09/23 C10, audit G-P2-3; 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3): the neutral corpus drives `commandRejectionNoticeV1` over the one typed
 * rejection contract every producer answers — the hub's graded `MutationMessage`s and every local refusal — and pins the
 * exact notice text in English and German, its severity and its notice code; no producer's English `reason` reaches the
 * notice. Every expected rejection is first admitted by the store's own schema through Ajv (third party), and every
 * rejection of the store's decode corpus yields a notice without throwing. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterAll, describe, expect, it } from "vitest";
import type { CommandRejectionV1 } from "@semio-tech/framework-os";
import { HISTORY_REFUSAL_LABEL_KEYS, LOCAL_COMMAND_REJECTION_NOTICES_V1, commandRejectionNoticeV1, historyRefusalCodeV1, historyRefusalNoticeV1, historyRefusalOfFaultV1, shellLabel, syncShellLabelLocale } from "../../🟦️.tsx";
import corpus from "../../🧫️fixtures/🧫️command-rejection/🔣️.json";
import decodeCorpus from "../../../../../../🏪️store/🧫️fixtures/🧫️command-rejection/🔣️.json";
import rejectionSchema from "../../../../../../🏪️store/🔄️sync/🧬️schema/🔣️command-rejection/🔣️.json";
import historyPatchSchema from "../../../../../../../../../🔨️modules/🎠️kernel/🧬️schema/🔣️history-patch/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Laws
type Row = { readonly name: string; readonly rejection: CommandRejectionV1; readonly kind: string; readonly code: string; readonly notice: Readonly<Record<"en" | "de", string>> };
const fixture = corpus as unknown as { readonly rows: readonly Row[] };
const decoded = decodeCorpus as unknown as { readonly hub: readonly { readonly expect: CommandRejectionV1 }[]; readonly local: readonly { readonly expect: CommandRejectionV1 }[] };

describe("command rejection notice", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("names the refused history step, the conflict the hub graded or the local refusal, in the human's language, and never its English reason", async () => {
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: false, allErrors: true });
    ajv.addSchema(historyPatchSchema);
    ajv.addSchema(rejectionSchema, "rejection");
    const valid = ajv.getSchema("rejection#/$defs/Rejection")!;
    for (const row of fixture.rows) expect(valid(row.rejection), `${row.name} ${JSON.stringify(valid.errors)}`).toBe(true);
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      for (const row of fixture.rows) {
        const notice = commandRejectionNoticeV1(row.rejection);
        expect({ text: notice.text, kind: notice.kind, code: notice.code }, `${row.name} (${locale})`).toEqual({ text: row.notice[locale], kind: row.kind, code: row.code });
        expect(notice.text.includes(row.rejection.reason), `${row.name} (${locale})`).toBe(false);
      }
    }
  });

  it("tells every local refusal code, and every rejection the store decodes, without throwing", () => {
    const localCodes = (rejectionSchema as { $defs: { Code: { enum: string[] } } }).$defs.Code.enum.filter((code) => code.startsWith("local."));
    expect(Object.keys(LOCAL_COMMAND_REJECTION_NOTICES_V1)).toEqual(localCodes);
    expect(new Set(fixture.rows.map((row) => row.rejection.code))).toEqual(new Set((rejectionSchema as { $defs: { Code: { enum: string[] } } }).$defs.Code.enum));
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      for (const rejection of [...decoded.hub, ...decoded.local].map((row) => row.expect)) {
        const notice = commandRejectionNoticeV1(rejection);
        expect(notice.text.length, rejection.code).toBeGreaterThan(0);
        expect(notice.text.includes("ui."), rejection.code).toBe(false);
      }
    }
  });

  it("localizes every history-edit refusal code in both languages, and a dispatch fault names one by its code or its first cause", () => {
    const codes = Object.keys(HISTORY_REFUSAL_LABEL_KEYS);
    expect(codes).toEqual(["history.malformed-transition", "history.unknown-target", "history.transition-refused", "timeTravel.frozen", "timeTravel.illegal", "timeTravel.stale", "timeTravel.blocked", "timeTravel.empty", "timeTravel.cancelled", "timeTravel.name-invalid"]);
    const texts = { en: [] as string[], de: [] as string[] };
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      for (const code of codes) {
        const text = String(shellLabel(HISTORY_REFUSAL_LABEL_KEYS[historyRefusalCodeV1(code)!]));
        expect(text, `${code} (${locale})`).not.toBe(HISTORY_REFUSAL_LABEL_KEYS[historyRefusalCodeV1(code)!]);
        texts[locale].push(text);
      }
    }
    expect(texts.en.every((text, index) => text !== texts.de[index])).toBe(true);
    syncShellLabelLocale("en");
    expect(historyRefusalNoticeV1("timeTravel.frozen", "warning")).toEqual({ text: "Editing is paused while history is being edited", kind: "warning", code: "timeTravel.frozen" });
    expect(historyRefusalNoticeV1("history.transition-refused", "info").kind).toBe("error");
    expect(historyRefusalNoticeV1("timeTravel.name-invalid").kind).toBe("warning");
    expect(historyRefusalCodeV1("mutation.clamped")).toBeNull();
    expect(historyRefusalCodeV1(7)).toBeNull();
    expect(historyRefusalOfFaultV1({ code: "timeTravel.stale" })).toBe("timeTravel.stale");
    expect(historyRefusalOfFaultV1({ code: "mutation.rejected", causes: [{ code: "mutation.clamped" }, { code: "history.unknown-target" }] })).toBe("history.unknown-target");
    expect(historyRefusalOfFaultV1({ code: "mutation.rejected", causes: [{ code: "mutation.clamped" }] })).toBeNull();
  });
});
//#endregion 🧪️Laws
