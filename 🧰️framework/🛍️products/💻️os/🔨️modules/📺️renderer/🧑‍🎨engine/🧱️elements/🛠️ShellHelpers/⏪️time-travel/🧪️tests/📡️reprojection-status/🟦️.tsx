/** 📡️ A history change replaying before adoption is announced outside the History panel, from the shell's status, in both locales. */
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { type ActionDescriptor, type HistoryPatch } from "@semio-tech/framework";
import {
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  programHistoryProjectionsWithLoadV1,
  shellHistoryProjectionWithLoadV1,
  shellHistoryProjectionAfterPatchV1
} from "../../../🔩️plumbing/🟦️.ts";
import { syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { HistoryReprojectionStatus, historyReprojectionControlV1 } from "../../🟦️.tsx";
import { historyReprojectionStatus } from "@semio-tech/framework";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
import { LOCALES, ariaFindings, controller, framework, readJson, shellHelpers } from "../🧰️history-test-support/🟦️.tsx";

//#region 📡️ReprojectionStatus
describe("📡️ a history change replaying before adoption is announced outside the History panel, in the kernel's own words", () => {
  afterEach(() => cleanup());
  type ReprojectionCase = { readonly name: string; readonly reprojection: NonNullable<HistoryPatch["reprojection"]>; readonly title: Readonly<Record<"en" | "de", string>>; readonly text: Readonly<Record<"en" | "de", string>>; readonly paused: boolean; readonly fault: string | null };
  const reprojectionCases = (readJson(join(framework, "🔨️modules", "🎠️kernel", "🧪️tests/🧫️fixtures", "🧫️history-reprojection", "🔣️.json")) as { readonly cases: readonly ReprojectionCase[] }).cases;
  const mountStatus = (reprojection: NonNullable<HistoryPatch["reprojection"]>, locale: "en" | "de", sessionOpen = false) => {
    const dispatched: ActionDescriptor[] = [];
    const view = render(createElement(HistoryReprojectionStatus, { reprojection, locale, sessionOpen, controllerId: controller, onAction: (action: ActionDescriptor) => dispatched.push(action) }));
    return { ...view, dispatched, root: view.container.querySelector<HTMLElement>("[data-semio-history-reprojection]")! };
  };

  it("shows every kernel case in English and German — title, line, progress bar only while it replays, the code only as data", () => {
    expect(reprojectionCases.length).toBeGreaterThanOrEqual(11);
    for (const testCase of reprojectionCases) {
      for (const locale of LOCALES) {
        syncShellLabelLocale(locale);
        const { root, unmount } = mountStatus(testCase.reprojection, locale);
        const label = `${testCase.name} (${locale})`;
        const announcement = root.querySelector('[role="status"][aria-live="polite"]');
        expect([root.getAttribute("data-semio-history-reprojection"), root.getAttribute("data-notice-code")], label).toEqual([testCase.reprojection.kind ?? "remote", testCase.fault]);
        expect(root.querySelector("[data-semio-history-reprojection-text]")?.textContent, label).toBe(testCase.text[locale]);
        expect(announcement?.textContent, label).toBe(`${testCase.title[locale]}: ${testCase.text[locale]}`);
        const refused = testCase.fault !== null && testCase.reprojection.total === 0;
        expect(root.querySelector("progress") !== null, label).toBe(!refused && !testCase.paused && testCase.reprojection.total > 0);
        if (testCase.fault !== null) expect(root.textContent, label).not.toContain(testCase.fault);
        expect(ariaFindings([root]), label).toEqual([]);
        unmount();
      }
    }
    syncShellLabelLocale("en");
  });

  it("announces a replay when it starts, a pause or a refusal when it happens, and never every progress step", () => {
    const view = mountStatus({ done: 1, total: 240, kind: "step" }, "en");
    const announced = () => view.container.querySelector('[role="status"]')?.textContent;
    expect(announced()).toBe("History step: Replaying history: 1 of 240 steps");
    view.rerender(createElement(HistoryReprojectionStatus, { reprojection: { done: 120, total: 240, kind: "step" }, locale: "en", sessionOpen: false, controllerId: controller, onAction: () => {} }));
    expect([announced(), view.container.querySelector("[data-semio-history-reprojection-text]")?.textContent, view.container.querySelector("progress")?.getAttribute("aria-valuetext")]).toEqual(["History step: Replaying history: 1 of 240 steps", "Replaying history: 120 of 240 steps", "Replaying history: 120 of 240 steps"]);
    view.rerender(createElement(HistoryReprojectionStatus, { reprojection: { done: 0, total: 0, kind: "step", fault: "history.step-blocked" }, locale: "en", sessionOpen: false, controllerId: controller, onAction: () => {} }));
    expect(announced()).toBe("History step: History step refused: Later mutations would end with errors — fix or withdraw them first.");
  });

  it("offers Cancel replay while it replays and Replay again while a remote change is paused — never while a session owns them", () => {
    expect([historyReprojectionControlV1({ done: 1, total: 4 }, false), historyReprojectionControlV1({ done: 1, total: 4, kind: "remote", paused: true }, false), historyReprojectionControlV1({ done: 1, total: 4, kind: "load" }, false), historyReprojectionControlV1({ done: 0, total: 0, fault: "history.replaying" }, false), historyReprojectionControlV1({ done: 1, total: 4 }, true)]).toEqual(["cancelReplay", "rerun", "cancelReplay", null, null]);
    for (const [locale, cancel, rerun] of [["en", "Cancel replay", "Replay again"], ["de", "Neuanwendung abbrechen", "Erneut anwenden"]] as const) {
      syncShellLabelLocale(locale);
      const running = mountStatus({ done: 2, total: 7, kind: "load" }, locale);
      (running.getByRole("button", { name: cancel }) as HTMLButtonElement).click();
      expect(running.dispatched).toEqual([{ controllerId: controller, action: "historyEditCancelReplay", args: {} }]);
      running.unmount();
      const paused = mountStatus({ done: 5, total: 12, kind: "remote", paused: true }, locale);
      (paused.getByRole("button", { name: rerun }) as HTMLButtonElement).click();
      expect(paused.dispatched).toEqual([{ controllerId: controller, action: "historyEditRerun", args: {} }]);
      paused.unmount();
      expect(mountStatus({ done: 2, total: 7 }, locale, true).root.querySelector("button")).toBeNull();
      cleanup();
    }
    syncShellLabelLocale("en");
  });

  it("shows a stepped document load as the shell's own status while no session is open, and leaves none once the load settles, fails or is cancelled", () => {
    const key = "toy#1";
    const loading = programHistoryProjectionsWithLoadV1({}, key, { completed: 0, total: 1 });
    expect(loading[key]?.reprojection, "the host's first poll of the load").toEqual({ kind: "load", done: 0, total: 1 });
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      const status = mountStatus(loading[key]!.reprojection!, locale);
      const expected = historyReprojectionStatus({ kind: "load", done: 0, total: 1 }, "native", locale);
      expect(
        [status.root.getAttribute("data-semio-history-reprojection"), status.root.getAttribute("data-semio-history-reprojection-phase"), status.root.querySelector("[data-semio-history-reprojection-text]")?.textContent, status.root.querySelector('[role="status"]')?.textContent, status.root.querySelector("[data-semio-history-reprojection-control]")?.getAttribute("data-semio-history-reprojection-control")],
        `no session, no check-in: the load alone raises the status (${locale})`,
      ).toEqual(["load", "progress", expected.text, `${expected.title}: ${expected.text}`, "cancelReplay"]);
      status.unmount();
    }
    syncShellLabelLocale("en");
    const known = shellHistoryProjectionAfterPatchV1(EMPTY_SHELL_HISTORY_PROJECTION_V1, { cursor: 7, canUndo: true, editCount: 3 }, true);
    const stepped = shellHistoryProjectionWithLoadV1(known, { completed: 2, total: 5 });
    expect([stepped.reprojection, stepped.cursor, stepped.canUndo, stepped.editCount, stepped.entries === known.entries], "a load step moves the reprojection alone").toEqual([{ kind: "load", done: 2, total: 5 }, 7, true, 3, true]);
    expect(shellHistoryProjectionWithLoadV1(stepped, { completed: 2, total: 5 }) === stepped, "a poll reporting the same progress wakes no reader").toBe(true);
    const ended = shellHistoryProjectionWithLoadV1(stepped, null);
    expect([ended.reprojection, ended.cursor, shellHistoryProjectionWithLoadV1(ended, null) === ended], "a settled, failed or cancelled load leaves no status").toEqual([null, 7, true]);
    const remote = shellHistoryProjectionAfterPatchV1(EMPTY_SHELL_HISTORY_PROJECTION_V1, { cursor: 4, reprojection: { done: 1, total: 9, kind: "remote" } }, true);
    expect(shellHistoryProjectionWithLoadV1(remote, null) === remote, "ending a load never clears a replay of another kind").toBe(true);
    const projections = { [key]: stepped, "other#2": remote };
    const cleared = programHistoryProjectionsWithLoadV1(projections, key, null);
    expect(
      [cleared[key]?.reprojection, cleared["other#2"] === remote, programHistoryProjectionsWithLoadV1(cleared, key, null) === cleared, programHistoryProjectionsWithLoadV1(cleared, "", { completed: 1, total: 2 }) === cleared, programHistoryProjectionsWithLoadV1(cleared, "unknown#9", null) === cleared],
      "only the loading program's projection moves; no program or nothing to end is the same map",
    ).toEqual([null, true, true, true, true]);
    const host = readModuleTreeSource(join(shellHelpers, "..", "🏛️ShellHost", "⚡️host-effects", "📥️document-load-effects", "🟦️.ts"));
    const start = host.indexOf("export async function applyLoadDocumentEffect(");
    expect(start, "applyLoadDocumentEffect must still be where this law reads it").toBeGreaterThanOrEqual(0);
    const load = host.slice(start, host.indexOf("\nexport ", start + 1));
    expect(
      [load.split("programHistoryProjectionsWithLoadV1(").length - 1, /progress: \(status\) => \{[^}]*showLoad\(status\);/u.test(load), /\} finally \{\s*showLoad\(null\);/u.test(load)],
      "the shell folds every polled step of a load into the loading program's projection and ends it however the load ends",
    ).toEqual([1, true, true]);
  });

  it("folds the waiting reprojection from every patch — a patch without one clears it", () => {
    const replaying = shellHistoryProjectionAfterPatchV1(EMPTY_SHELL_HISTORY_PROJECTION_V1, { cursor: 4, reprojection: { done: 1, total: 9, kind: "remote" } }, true);
    expect([EMPTY_SHELL_HISTORY_PROJECTION_V1.reprojection, replaying.reprojection, shellHistoryProjectionAfterPatchV1(replaying, { cursor: 5 }, false).reprojection]).toEqual([null, { done: 1, total: 9, kind: "remote" }, null]);
  });
});
//#endregion 📡️ReprojectionStatus
