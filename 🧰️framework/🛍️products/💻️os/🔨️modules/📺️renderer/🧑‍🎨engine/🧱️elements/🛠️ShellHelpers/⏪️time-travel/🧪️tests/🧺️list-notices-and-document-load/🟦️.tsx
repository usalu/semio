/** 🧺️ List inputs add and remove items through the outputs, history-lane refusals reach the person, a cancelled import frees the import the guest started and a whole-document load shows its progress. */
import { join } from "node:path";
import { afterAll, afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent } from "@semio-tech/ui-react/test";
import { type ActionDescriptor, type BuiltNode } from "@semio-tech/framework";
import { EMPTY_SHELL_HISTORY_PROJECTION_V1, shellHistoryProjectionAfterPatchV1 } from "../../../🔩️plumbing/🟦️.ts";
import {
  historyRefusalCodeV1,
  historyRefusalOfOutputV1,
  HISTORY_SILENT_REFUSALS_V1,
  historyFaultNoticeV1,
  historyLaneNoticeV1,
  historyOutputNoticeV1,
  historySilentRefusalOfFaultV1,
  historySilentRefusalOfOutputV1
} from "../../../🛣️route-and-chrome/🟦️.tsx";
import { panelActionRoutesThroughHostV1 } from "../../../🌳️tree-windows/🟦️.tsx";
import { syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { TREE_ROW_TONE_CLASSES } from "../../../../🗣️Interpreter/🌳️tree-walk/🟦️.ts";
import { HISTORY_NOTICE_LABELS } from "@semio-tech/framework";
import { documentLoadCancelledV1 } from "../../../🧾️transfer-checkin-tutorial/🟦️.ts";
import { IMPORT_ABORT_ACTION_ID, importOpenedFilesV1 } from "../../../🎞️media-import/🟦️.ts";
import { GENERATION, LOCALES, ariaFindings, bind, button, computeAccessibleName, controller, corpus, framework, mountHistoryBody, node, readJson, section, treeItem } from "../🧰️history-test-support/🟦️.tsx";

//#region 🧺️ListInputs
/** 🧺️ A list input as the runtime builds it (gap N2): its row counts the items and holds "Add item" (`historyEditInput{path:
 * <list>/-, edit: insert}`, disabled at `maxItems`), each item row holds "Remove item" (`{path: <list>/<i>, edit: remove}`,
 * disabled at `minItems`) — one control per row; a row that reads otherwise than its button never stands in for it, so the
 * button keeps its own name. */
const listBody = (labels: { readonly add: string; readonly remove: string; readonly count: string }, addable: boolean, removable: boolean): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.editor.inputs", "Inputs", [
      treeItem("framework.history.editor.input.points.row", "Points", [button("framework.history.editor.input.points.add", labels.add, bind(controller, "historyEditInput", { generation: GENERATION, path: "/points/-", edit: "insert" }), { disabled: !addable })], { description: labels.count }),
      ...[0, 1].map((index) => treeItem(`framework.history.editor.input.points.${index}.row`, `Points ${index + 1}`, [button(`framework.history.editor.input.points.${index}.remove`, labels.remove, bind(controller, "historyEditInput", { generation: GENERATION, path: `/points/${index}`, edit: "remove" }), { disabled: !removable })])),
    ]),
  ]);

describe("🧺️ list inputs add and remove items through the one input verb", () => {
  afterEach(() => cleanup());

  it("adds and removes by name in English and German, and a disabled Add or Remove — and its row — fires nothing", () => {
    for (const labels of [{ add: "Add item", remove: "Remove item", count: "2 items" }, { add: "Element hinzufügen", remove: "Element entfernen", count: "2 Elemente" }]) {
      const dispatched: ActionDescriptor[] = [];
      const open = mountHistoryBody(listBody(labels, true, true), (action) => dispatched.push(action));
      const named = (name: string) => [...open.container.querySelectorAll<HTMLButtonElement>("button")].filter((button) => computeAccessibleName(button) === name);
      fireEvent.click(named(labels.add)[0]!);
      fireEvent.click(named(labels.remove)[1]!);
      expect(dispatched.map(({ action, args }) => [action, args]), labels.add).toEqual([["historyEditInput", { generation: GENERATION, path: "/points/-", edit: "insert" }], ["historyEditInput", { generation: GENERATION, path: "/points/1", edit: "remove" }]]);
      for (const row of open.container.querySelectorAll<HTMLElement>('[role="treeitem"]')) fireEvent.click(row);
      expect([dispatched.length, ariaFindings([open.container])], `${labels.add}: a row that does not say "add" or "remove" never adds or removes`).toEqual([2, []]);
      open.unmount();
      const full = mountHistoryBody(listBody(labels, false, false), (action) => dispatched.push(action));
      const buttons = [...full.container.querySelectorAll<HTMLButtonElement>("button")].filter((button) => [labels.add, labels.remove].includes(computeAccessibleName(button)));
      expect([buttons.length, buttons.every((button) => button.disabled)]).toEqual([3, true]);
      for (const button of buttons) fireEvent.click(button);
      for (const row of full.container.querySelectorAll<HTMLElement>('[role="treeitem"]')) fireEvent.click(row);
      expect(dispatched.length, `${labels.add}: nothing fires at the item bounds`).toBe(2);
      full.unmount();
    }
  });
});
//#endregion 🧺️ListInputs

//#region 📢️HistoryLaneNotices
describe("📢️ history-lane refusals reach the person in the kernel's own words, never as a raw code", () => {
  afterAll(() => syncShellLabelLocale("en"));

  it("tells every kernel notice in English and German — the fault's own code, a cause's, or a verb's silent rejection — `{n}` the structured edit count", () => {
    const fixture = readJson(join(framework, "🔨️modules", "🎠️kernel", "🧪️tests/🧫️fixtures", "🧫️history-notices", "🔣️.json")) as { readonly notices: readonly { readonly code: string; readonly en: string; readonly de: string }[] };
    expect(fixture.notices.map((row) => row.code)).toEqual(HISTORY_NOTICE_LABELS.map((row) => row.code));
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const row of fixture.notices) {
        const expected = { text: row[locale].replace("{n}", "64"), kind: "warning", code: row.code };
        expect([historyFaultNoticeV1({ code: row.code, severity: "error" }, 64), historyFaultNoticeV1({ code: "module.vcs", severity: "error", causes: [{ code: row.code }] }, 64), historyOutputNoticeV1({ rejected: row.code }, 64)], `${row.code} (${locale})`).toEqual([expected, expected, expected]);
      }
    }
    syncShellLabelLocale("en");
    expect(historyLaneNoticeV1({ code: "history.full" }, 12)?.text, "the count is the projection's, never digits read from a fault message").toBe("This document's history is full (12 edits).");
    expect(fixture.notices.find((row) => row.code === "document.loading")).toEqual({ code: "document.loading", en: "The document is still loading — wait for it or cancel it first.", de: "Das Dokument wird noch geladen — abwarten oder zuerst abbrechen." });
    expect([historyFaultNoticeV1({ code: "module.vcs" }, 0), historyOutputNoticeV1({ rejected: "app.unknown" }, 0), historyLaneNoticeV1({ code: "timeTravel.frozen" }, 0), historyFaultNoticeV1({ code: "timeTravel.frozen", severity: "warning" }, 0)?.code]).toEqual([null, null, null, "timeTravel.frozen"]);
  });

  it("never tells the human a refusal the corpus marks silent — a late event of an older session — and still names it for the trace", () => {
    const refusals = corpus.refusals as readonly { readonly code: string; readonly silent?: boolean }[];
    expect([refusals.filter((row) => row.silent === true).map((row) => row.code), [...HISTORY_SILENT_REFUSALS_V1]], "the corpus and the shell name the same silent refusals").toEqual([["timeTravel.stale"], ["timeTravel.stale"]]);
    for (const row of refusals) {
      const code = historyRefusalCodeV1(row.code);
      const silent = row.silent === true;
      expect(
        [historyOutputNoticeV1({ rejected: row.code }, 0) === null, historyFaultNoticeV1({ code: row.code, severity: "warning" }, 0) === null, historyFaultNoticeV1({ code: "module.vcs", causes: [{ code: row.code }] }, 0) === null, historySilentRefusalOfOutputV1({ rejected: row.code }), historySilentRefusalOfFaultV1({ code: row.code }), historySilentRefusalOfFaultV1({ code: "module.vcs", causes: [{ code: row.code }] }), historyRefusalOfOutputV1({ rejected: row.code })],
        `${row.code}: a verb's silent result, the fault's own code or a cause's`,
      ).toEqual([silent, silent, silent, silent ? code : null, silent ? code : null, silent ? code : null, code]);
    }
    expect([historySilentRefusalOfOutputV1({ rejected: "app.unknown" }), historySilentRefusalOfOutputV1(null), historySilentRefusalOfFaultV1({ code: "module.vcs" })]).toEqual([null, null, null]);
  });

  it("folds the history's edit count from every patch — the `{n}` a later history-full notice names", () => {
    const counted = shellHistoryProjectionAfterPatchV1(EMPTY_SHELL_HISTORY_PROJECTION_V1, { cursor: 64, editCount: 64 }, true);
    expect([EMPTY_SHELL_HISTORY_PROJECTION_V1.editCount, counted.editCount, shellHistoryProjectionAfterPatchV1(counted, { cursor: 65, editCount: 3 }, false).editCount, shellHistoryProjectionAfterPatchV1(counted, { cursor: 66 }, false).editCount]).toEqual([0, 64, 3, 0]);
  });
});
//#endregion 📢️HistoryLaneNotices

//#region ⛔️ImportAbort
describe("⛔️ a cancelled import frees the import the guest still holds", () => {
  const opened = ["a.png", "b.png", "c.png"].map((name) => ({ name, contents: `data:,${name}` }));
  const app = (declares: boolean) => ({ actions: declares ? [{ id: IMPORT_ABORT_ACTION_ID }] : [{ id: "importFramePayload" }] }) as unknown as Parameters<typeof importOpenedFilesV1>[0];
  const run = async (declares: boolean, cancelAfter: number | "before" | null) => {
    const controller = new AbortController();
    if (cancelAfter === "before") controller.abort();
    const dispatched: string[] = [];
    const outcome = await importOpenedFilesV1(app(declares), opened, "importFramePayload", true, async (action, args) => {
      dispatched.push(action === IMPORT_ABORT_ACTION_ID ? `${action} ${JSON.stringify(args)}` : `${action} ${(args as { readonly name: string }).name}`);
      if (dispatched.length === cancelAfter) controller.abort();
    }, controller.signal);
    return [outcome, dispatched];
  };

  it("sends importAbort once a cancel stops an import the guest already holds, and nothing when no file reached it or the app declares no abort", async () => {
    expect(await run(true, 1)).toEqual(["cancelled", ["importFramePayload a.png", "importAbort {}"]]);
    expect(await run(true, 2)).toEqual(["cancelled", ["importFramePayload a.png", "importFramePayload b.png", "importAbort {}"]]);
    expect(await run(false, 1)).toEqual(["cancelled", ["importFramePayload a.png"]]);
    expect(await run(true, "before")).toEqual(["cancelled", []]);
    expect(await run(true, null)).toEqual(["done", ["importFramePayload a.png", "importFramePayload b.png", "importFramePayload c.png"]]);
    await expect(importOpenedFilesV1(app(true), opened, "importFramePayload", true, async () => { throw new Error("guest refused"); }, new AbortController().signal)).rejects.toThrow("guest refused");
  });
});
//#endregion ⛔️ImportAbort

//#region ⏹️DocumentLoad
/** ⏹️ The runtime's `framework.history.reprojection` section while a whole document loads (`HistoryPatch.reprojection {kind:
 * load}`): the status row in words, and Cancel replay (`historyEditCancelReplay`, no session open) as a button row. */
const loadingBody = (texts: { readonly section: string; readonly status: string; readonly cancel: string }): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.reprojection", texts.section, [
      treeItem("framework.history.reprojection.status", texts.status, [], { icon: "cloud-download", tone: "info" }),
      treeItem("framework.history.reprojection.cancelReplay.row", texts.cancel, [button("framework.history.reprojection.cancelReplay", texts.cancel, bind(controller, "historyEditCancelReplay"))]),
    ]),
  ]);

describe("⏹️ a whole-document load shows its progress in the history body and is cancelled from there", () => {
  afterEach(() => cleanup());

  it("reads the load's progress in words and dispatches Cancel replay to the program, in English and German", () => {
    for (const texts of [{ section: "Document load", status: "Loading document: 3 of 240", cancel: "Cancel replay" }, { section: "Dokument laden", status: "Dokument wird geladen: 3 von 240", cancel: "Neuanwendung abbrechen" }]) {
      const dispatched: ActionDescriptor[] = [];
      const view = mountHistoryBody(loadingBody(texts), (action) => dispatched.push(action));
      const toned = [...view.container.querySelectorAll<HTMLElement>(`.${TREE_ROW_TONE_CLASSES.info!}`)].some((row) => (row.textContent ?? "").includes(texts.status));
      expect([toned, ariaFindings([view.container])], texts.status).toEqual([true, []]);
      (view.getByRole("button", { name: texts.cancel }) as HTMLButtonElement).click();
      expect(dispatched.map(({ controllerId, action, args }) => ({ controllerId, action, args: args ?? null })), texts.cancel).toEqual([{ controllerId: controller, action: "historyEditCancelReplay", args: null }]);
      expect(panelActionRoutesThroughHostV1({ controllerId: controller, action: "historyEditCancelReplay" })).toBe(false);
      view.unmount();
    }
  });

  it("tells a cancelled load — the caller's or the program's own — from a failed one", () => {
    const aborted = new AbortController();
    aborted.abort(new Error("person cancelled"));
    expect([documentLoadCancelledV1(new DOMException("cancelled", "AbortError")), documentLoadCancelledV1(new Error("person cancelled"), aborted.signal), documentLoadCancelledV1(new Error("malformed archive")), documentLoadCancelledV1(new DOMException("timeout", "TimeoutError"), new AbortController().signal)]).toEqual([true, true, false, false]);
  });
});
//#endregion ⏹️DocumentLoad
