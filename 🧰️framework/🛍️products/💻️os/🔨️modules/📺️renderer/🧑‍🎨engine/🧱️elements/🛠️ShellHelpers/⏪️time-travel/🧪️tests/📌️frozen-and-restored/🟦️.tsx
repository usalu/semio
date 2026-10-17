/** 📌️ No checkpoint reaches a frozen history (e2e R2-6), and a restored document's history replaces the displayed one. */
import { join } from "node:path";
import { afterAll, afterEach, describe, expect, it } from "vitest";
import { createElement, useState } from "react";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { type HistoryEntry, type HistoryTimeTravel } from "@semio-tech/framework";
import { checkpointGateV1, checkpointOnCloseKeyV1, useCheckpointOnCloseV1 } from "../../../🧾️transfer-checkin-tutorial/🟦️.ts";
import {
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  shellHistoryCursorDomV1,
  shellHistoryProjectionAfterPatchV1
} from "../../../🔩️plumbing/🟦️.ts";
import { syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { LOCALES, cases, editingSession, readJson, shellHelpers } from "../🧰️history-test-support/🟦️.tsx";

//#region 📌️FrozenCheckIn
describe("📌️ no checkpoint reaches a frozen history (e2e R2-6)", () => {
  afterEach(() => cleanup());

  it("dispatches a checkpoint only outside a session: an automatic one waits, an explicit one is told why", () => {
    for (const entry of cases) expect([checkpointGateV1(entry.session, "auto"), checkpointGateV1(entry.session, "check-in")], entry.name).toEqual(["wait", "frozen"]);
    expect([checkpointGateV1(null, "auto"), checkpointGateV1(null, "release")]).toEqual(["dispatch", "dispatch"]);
  });

  it("checkpoints the document being left once, and never on a new session object of the same program", () => {
    const sent: string[] = [];
    type ProbeProps = { readonly session: { readonly pluginId: string; readonly instanceId: number; readonly viewState: string }; readonly documentId: string | null; readonly editor: boolean; readonly timeTravel: HistoryTimeTravel | null };
    function Probe({ session, documentId, editor, timeTravel }: ProbeProps) {
      const [opened] = useState(() => session.viewState);
      useCheckpointOnCloseV1(checkpointOnCloseKeyV1(editor, session, documentId), () => {
        if (checkpointGateV1(timeTravel, "auto") === "dispatch") sent.push(`${session.pluginId}#${session.instanceId}@${documentId}:${opened}`);
      });
      return null;
    }
    const program = { pluginId: "puzzle", instanceId: 3 };
    const view = render(createElement(Probe, { session: { ...program, viewState: "before-finalize" }, documentId: "doc-1", editor: true, timeTravel: null }));
    view.rerender(createElement(Probe, { session: { ...program, viewState: "after-new-alternative-submit" }, documentId: "doc-1", editor: true, timeTravel: null }));
    expect(sent, "the New-alternative submit mints a new session object of the same program").toEqual([]);
    view.rerender(createElement(Probe, { session: { ...program, viewState: "after-new-alternative-submit" }, documentId: "doc-2", editor: true, timeTravel: editingSession }));
    expect(sent, "leaving doc-1 checkpoints it with the closure that opened it").toEqual(["puzzle#3@doc-1:before-finalize"]);
    view.rerender(createElement(Probe, { session: { ...program, viewState: "x" }, documentId: "doc-3", editor: true, timeTravel: null }));
    expect(sent, "leaving doc-2 while its history was being edited dispatches nothing").toEqual(["puzzle#3@doc-1:before-finalize"]);
    view.unmount();
    expect(sent).toEqual(["puzzle#3@doc-1:before-finalize", "puzzle#3@doc-3:before-finalize"]);
    expect([checkpointOnCloseKeyV1(false, program, "doc-1"), checkpointOnCloseKeyV1(true, null, "doc-1"), checkpointOnCloseKeyV1(true, program, ""), checkpointOnCloseKeyV1(true, program, "doc-1")]).toEqual([null, null, null, "puzzle#3@doc-1"]);
  });
});
//#endregion 📌️FrozenCheckIn

//#region 🗃️RestoredHistory
describe("🗃️ a restored document's history replaces the displaced rows and reads every label in each language (R2-2, G5)", () => {
  afterAll(() => syncShellLabelLocale("en"));

  it("lists every row of the read-back snapshot, none of the displaced document's, and draws each label from its LocalizedLabel, never from op text", () => {
    const reload = readJson(join(shellHelpers, "..", "..", "..", "..", "🔌️plugin", "🧪️tests/🧫️fixtures", "🧫️history-label-reload", "🔣️.json")) as { readonly cases: readonly { readonly id: string; readonly expected: { readonly en: string; readonly de: string } }[] };
    const localized = (text: { readonly en: string; readonly de: string }) => ({ native: text, reuse: text });
    const row = (seq: number, editId: string, label: { readonly en: string; readonly de: string }) => ({ seq, editId, actionId: "apply", label: localized(label), kind: "mutation", timestamp: "t", opLines: [`set ${editId}`], opCount: 1, applied: true, revertible: true, count: 1, mutations: [] }) as unknown as HistoryEntry;
    const displaced = shellHistoryProjectionAfterPatchV1(EMPTY_SHELL_HISTORY_PROJECTION_V1, { cursor: 1, upserts: [row(1, "fresh-instance", { en: "Set Active Example", de: "Aktives Beispiel festlegen" })], canUndo: true }, true);
    const restored = shellHistoryProjectionAfterPatchV1(displaced, { cursor: reload.cases.length, upserts: reload.cases.map((entry, index) => row(index + 1, entry.id, entry.expected)), canUndo: true, currentCheckpointId: "cp-restored" }, true);
    expect(Object.keys(restored.entries).sort()).toEqual(reload.cases.map((entry) => `edit:${entry.id}`).sort());
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      const dom = shellHistoryCursorDomV1(restored, { terminology: "native", locale }) as { readonly labels: readonly string[]; readonly undoLabel: string | null; readonly currentCheckpointId: string | null };
      expect(dom.labels, locale).toEqual(reload.cases.map((entry) => entry.expected[locale]));
      expect([dom.undoLabel, dom.currentCheckpointId], locale).toEqual([reload.cases.at(-1)!.expected[locale], "cp-restored"]);
      expect(dom.labels.some((label) => label.startsWith("set ")), `${locale}: no op text`).toBe(false);
    }
  });
});
//#endregion 🗃️RestoredHistory
