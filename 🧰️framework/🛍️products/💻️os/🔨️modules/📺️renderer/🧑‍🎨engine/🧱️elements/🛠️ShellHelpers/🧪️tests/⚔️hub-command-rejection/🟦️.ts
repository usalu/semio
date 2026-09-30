// #region 🧲️Header
/** ⚔️ What a human is told when the hub refuses one of their command batches (🎫️ 26/09/23 C10, audit G-P2-3; 26/09/30
 * NON-DESTRUCTIVE-HISTORY-EDITING): the neutral corpus drives `hubCommandRejectionNoticeV1` over the hub's own canonical
 * MutationMessage payloads and pins the exact notice text in English and German, its severity and its notice code; the
 * hub's English diagnostic `reason` never reaches the notice. The payload bytes are encoded by Node's own
 * `TextEncoder`/`JSON.stringify` (the hub's `encode_messages` shape), independent of the shell's decoder. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterAll, describe, expect, it } from "vitest";
import { HISTORY_REFUSAL_LABEL_KEYS, historyRefusalCodeV1, historyRefusalNoticeV1, historyRefusalOfFaultV1, hubCommandRejectionNoticeV1, shellLabel, syncShellLabelLocale } from "../../🟦️.tsx";
import corpus from "../../🧫️fixtures/⚔️hub-command-rejection/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Laws
type Row = { readonly name: string; readonly reason: string; readonly messages: readonly unknown[] | null; readonly kind: string; readonly code: string; readonly notice: Readonly<Record<"en" | "de", string>> };
const fixture = corpus as unknown as { readonly rows: readonly Row[]; readonly hostile: readonly { readonly name: string; readonly bytes: string }[] };
const bytes = (text: string): number[] => Array.from(new TextEncoder().encode(text));

describe("hub command rejection notice", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("names the refused history step or the conflict the hub graded, in the human's language, and never its English reason", () => {
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      for (const row of fixture.rows) {
        const notice = hubCommandRejectionNoticeV1(row.messages === null ? [] : bytes(JSON.stringify(row.messages)));
        expect({ text: notice.text, kind: notice.kind, code: notice.code }, `${row.name} (${locale})`).toEqual({ text: row.notice[locale], kind: row.kind, code: row.code });
        expect(notice.text.includes(row.reason), `${row.name} (${locale})`).toBe(false);
      }
    }
  });

  it("refuses a payload that is not the hub's MutationMessage array", () => {
    for (const row of fixture.hostile) expect(() => hubCommandRejectionNoticeV1(bytes(row.bytes)), row.name).toThrow();
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
