// #region 🧲️Header
/** @emoji ⚔️ What a human is told when the hub refuses one of their command batches (🎫️ 26/09/23 C10, audit G-P2-3): the
 * neutral corpus drives `hubCommandRejectionReasonKeyV1` over the hub's own canonical MutationMessage payloads and pins the
 * exact notice text in English and German; the hub's English diagnostic `reason` never reaches the notice. The payload
 * bytes are encoded by Node's own `TextEncoder`/`JSON.stringify` (the hub's `encode_messages` shape), independent of the
 * shell's decoder. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterAll, describe, expect, it } from "vitest";
import { hubCommandRejectionReasonKeyV1, shellLabel, syncShellLabelLocale } from "../../🟦️.tsx";
import corpus from "../../🧫️fixtures/⚔️hub-command-rejection/🔣️.json";
// #endregion 🔌️Adapters

//#region 🧪️Laws
type Row = { readonly name: string; readonly reason: string; readonly messages: readonly unknown[] | null; readonly reasonKey: "ui.conflict.hubConcurrentEdit" | "ui.conflict.hubConcurrentInvariant" | null; readonly notice: Readonly<Record<"en" | "de", string>> };
const fixture = corpus as unknown as { readonly rows: readonly Row[]; readonly hostile: readonly { readonly name: string; readonly bytes: string }[] };
const bytes = (text: string): number[] => Array.from(new TextEncoder().encode(text));

describe("hub command rejection notice", () => {
  afterAll(() => {
    syncShellLabelLocale("en");
  });

  it("names the conflict the hub graded, in the human's language, and never its English reason", () => {
    for (const locale of ["en", "de"] as const) {
      syncShellLabelLocale(locale);
      for (const row of fixture.rows) {
        const key = hubCommandRejectionReasonKeyV1(row.messages === null ? [] : bytes(JSON.stringify(row.messages)));
        expect(key, row.name).toBe(row.reasonKey);
        const notice = key === null ? String(shellLabel("ui.conflict.hubRejected")) : `${shellLabel("ui.conflict.hubRejected")}: ${shellLabel(key)}`;
        expect(notice, `${row.name} (${locale})`).toBe(row.notice[locale]);
        expect(notice.includes(row.reason), `${row.name} (${locale})`).toBe(false);
      }
    }
  });

  it("refuses a payload that is not the hub's MutationMessage array", () => {
    for (const row of fixture.hostile) expect(() => hubCommandRejectionReasonKeyV1(bytes(row.bytes)), row.name).toThrow();
  });
});
//#endregion 🧪️Laws
