/** 🛰️ ShellHost's reveal and focus wiring drives the History panel from the corpus's transitions. */
import { afterEach, describe, expect, it } from "vitest";
import { createElement, useCallback, useReducer } from "react";
import { createMachine, transition, type AnyMachineSnapshot } from "xstate";
import { cleanup, render } from "@semio-tech/ui-react/test";
import { createMemoryStoragePort, type HistoryTimeTravel } from "@semio-tech/framework";
import type { PanelDock, PanelTabNode } from "@semio-tech/ui-react";
import { type ShellState } from "../../../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { type ShellAction } from "../../../../🐚️Shell/🎛️shell-actions/🟦️.ts";
import { initialShellState, shellReducer } from "../../../../🐚️Shell/🕹️shell-store/🟦️.ts";
import { revealHistoryPanelV1, timeTravelTransitionV1, useTimeTravelRevealV1, type TimeTravelFocusTargetV1 } from "../../🟦️.tsx";
import { Transition, corpus, nextFrames } from "../🧰️history-test-support/🟦️.tsx";

//#region 🛰️ShellReveal
/** 🗄️ A dock the way ShellHost composes it: Settings and the History leaf at the bottom right; the merged mobile panel
 * keeps History inside its "More" branch. */
const leaf = (id: string): PanelTabNode => ({ kind: "leaf", id, icon: null, name: id, order: 0, trees: [] }) as unknown as PanelTabNode;
const emptyAnchors = (): Record<string, readonly PanelTabNode[]> => ({ "top-left": [], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] });
const dockWith = (history: boolean): PanelDock => ({ anchors: { ...emptyAnchors(), "bottom-right": [leaf("framework.settings"), ...(history ? [leaf("framework.panel.history")] : [])] } }) as unknown as PanelDock;
const mobileTabs: readonly PanelTabNode[] = [leaf("framework.category.tool"), { kind: "branch", id: "framework.mobile.more", icon: null, name: "More", order: 1, children: [leaf("framework.panel.history")] } as unknown as PanelTabNode];
type RevealProps = { readonly session: HistoryTimeTravel | null; readonly mobile: boolean; readonly history: boolean; readonly root: Element | null; readonly seen: { state: ShellState; actions: string[] } };

/** 🛰️ The shell's real layout reducer driven by the reveal hook exactly as ShellHost mounts it. */
function RevealProbe({ session, mobile, history, root, seen }: RevealProps) {
  const [state, dispatchState] = useReducer(shellReducer, undefined, () => initialShellState({ locale: "en", plugins: [], storage: createMemoryStoragePort() }));
  const dispatch = useCallback((action: ShellAction) => {
    seen.actions.push(action.type);
    dispatchState(action);
  }, [seen]);
  useTimeTravelRevealV1(session, { mobile, dock: dockWith(history), mobilePanelTabs: mobileTabs, dispatch, root, frames: 3 });
  seen.state = state;
  return null;
}

describe("🛰️ ShellHost's reveal and focus wiring drives the shell's real layout reducer", () => {
  afterEach(() => cleanup());
  const at = (stage: HistoryTimeTravel["stage"], extra: Partial<HistoryTimeTravel> = {}): HistoryTimeTravel => ({ sessionId: "1", generation: 1, stage, blocking: false, acceptedCount: 0, ...extra });
  const editorDom = (root: HTMLElement) => {
    root.innerHTML = '<div id="panel:framework.panel.history/framework.history.editor.input.dx.row"><input id="panel:framework.panel.history/framework.history.editor.input.dx.row/framework.history.editor.input.dx"></div>';
    return root.querySelector<HTMLInputElement>("input")!;
  };
  const mount = (props: Omit<RevealProps, "seen">) => {
    const seen = { state: undefined as unknown as ShellState, actions: [] as string[] };
    const view = render(createElement(RevealProbe, { ...props, seen }));
    return { seen, rerender: (next: Partial<Omit<RevealProps, "seen">>) => view.rerender(createElement(RevealProbe, { ...props, ...next, seen })), view };
  };

  it("opens and selects the History tab on its desktop anchor at the edge into a session, once per session", () => {
    const probe = mount({ session: null, mobile: false, history: true, root: null });
    expect(probe.seen.actions).toEqual([]);
    probe.rerender({ session: at("editing", { target: "m-2" }) });
    expect([probe.seen.actions, probe.seen.state.layout.panels["bottom-right"]]).toEqual([["SET_PANEL_PATH", "SET_PANEL_VISIBLE"], { ...probe.seen.state.layout.panels["bottom-right"], visible: true, path: ["framework.panel.history"] }]);
    probe.rerender({ session: at("replaying", { target: "m-2", generation: 2, done: 3, total: 8 }) });
    probe.rerender({ session: at("replaying", { target: "m-2", generation: 2, done: 5, total: 8 }) });
    expect(probe.seen.actions, "a stage change and progress of the same session reveal nothing again").toHaveLength(2);
    probe.rerender({ session: at("editing", { sessionId: "2", target: "m-4" }) });
    expect(probe.seen.actions).toHaveLength(4);
  });

  it("opens the merged mobile panel on the History tab, and dispatches nothing where the shell has no History tab", () => {
    const phone = mount({ session: null, mobile: true, history: true, root: null });
    phone.rerender({ session: at("reviewing", { review: "ready", acceptedCount: 1 }) });
    expect([phone.seen.actions, phone.seen.state.layout.mobilePanelPath, phone.seen.state.layout.mobilePanelVisible]).toEqual([["SET_MOBILE_PANEL_PATH", "SET_MOBILE_PANEL_VISIBLE"], ["framework.mobile.more", "framework.panel.history"], true]);
    phone.view.unmount();
    const bare = mount({ session: null, mobile: false, history: false, root: null });
    bare.rerender({ session: at("editing", { target: "m-2" }) });
    expect(bare.seen.actions).toEqual([]);
    expect([revealHistoryPanelV1({ mobile: false, dock: dockWith(false), mobilePanelTabs: mobileTabs, dispatch: () => undefined }), revealHistoryPanelV1({ mobile: true, dock: dockWith(true), mobilePanelTabs: [], dispatch: () => undefined })]).toEqual([false, false]);
  });

  it("keeps a focus waiting for the editor through progress patches, and cancels it when the session closes", async () => {
    const root = document.createElement("div");
    document.body.append(root);
    const probe = mount({ session: null, mobile: false, history: true, root });
    probe.rerender({ session: at("editing", { target: "m-2" }) });
    probe.rerender({ session: at("editing", { target: "m-2", acceptedCount: 0, blocking: false, generation: 1 }) });
    const dx = editorDom(root);
    await nextFrames(2);
    expect(document.activeElement, "the editor that mounted late takes the focus Begin asked for").toBe(dx);
    dx.blur();
    root.innerHTML = "";
    probe.rerender({ session: at("editing", { target: "m-5", generation: 2 }) });
    probe.rerender({ session: null });
    editorDom(root);
    await nextFrames(4);
    expect(document.activeElement, "a closed session moves no focus").toBe(document.body);
    root.remove();
  });

  it("leaves someone's typing alone for the editor and the band, while the modal finalize prompt always takes focus", async () => {
    const root = document.createElement("div");
    const chat = document.createElement("input");
    document.body.append(root, chat);
    const dx = editorDom(root);
    chat.focus();
    const probe = mount({ session: null, mobile: false, history: true, root });
    probe.rerender({ session: at("editing", { target: "m-2" }) });
    await nextFrames(2);
    expect([document.activeElement === chat, document.activeElement === dx]).toEqual([true, false]);
    const prompt = document.createElement("div");
    prompt.setAttribute("role", "dialog");
    prompt.innerHTML = '<input id="prompt-name">';
    document.body.append(prompt);
    probe.rerender({ session: at("choosing", { generation: 3 }) });
    await nextFrames(2);
    expect(document.activeElement?.id).toBe("prompt-name");
    prompt.remove();
    chat.remove();
    root.remove();
  });

  it("scrolls the first blocking row into view once it is mounted, moving no focus, and drops the scroll when the session closes", async () => {
    const root = document.createElement("div");
    document.body.append(root);
    const scrolled: string[] = [];
    const scrollIntoView = Element.prototype.scrollIntoView;
    Element.prototype.scrollIntoView = function (this: Element) {
      scrolled.push(this.id);
    };
    try {
      const probe = mount({ session: at("replaying", { target: "m-2", done: 8, total: 8, acceptedCount: 1 }), mobile: false, history: true, root });
      const before = probe.seen.actions.length;
      probe.rerender({ session: at("reviewing", { generation: 2, review: "blocked", blocking: true, acceptedCount: 1, nextProblem: { mutationId: "m-5" } }) });
      expect(probe.seen.actions.slice(before), "a replay that completed blocked reveals the History panel again").toEqual(["SET_PANEL_PATH", "SET_PANEL_VISIBLE"]);
      await nextFrames(1);
      expect(scrolled, "the row is not mounted yet").toEqual([]);
      root.innerHTML = '<div id="panel:framework.panel.history/framework.history.entry.4"><div id="panel:framework.panel.history/framework.history.entry.4/framework.history.mutation.m-5" tabindex="0"></div></div>';
      await nextFrames(2);
      expect([scrolled, document.activeElement === document.body]).toEqual([["panel:framework.panel.history/framework.history.entry.4/framework.history.mutation.m-5"], true]);
      root.innerHTML = "";
      probe.rerender({ session: at("editing", { generation: 3, target: "m-5" }) });
      probe.rerender({ session: at("reviewing", { generation: 4, review: "blocked", blocking: true, acceptedCount: 1, nextProblem: { mutationId: "m-9", store: "parts/p-1" } }) });
      probe.rerender({ session: null });
      root.innerHTML = '<div id="panel:framework.panel.history/framework.history.mutation.m-9"></div>';
      await nextFrames(4);
      expect(scrolled, "a closed session scrolls nothing").toHaveLength(1);
    } finally {
      Element.prototype.scrollIntoView = scrollIntoView;
      root.remove();
    }
  });

  it("agrees with an xstate model of the reveal-and-focus statechart on every corpus transition", () => {
    const stages = ["editing", "replaying", "reviewing", "choosing", "finalizing"] as const;
    const focusOf = { editing: "editor", replaying: "band", reviewing: "band", choosing: "dialog", finalizing: null } as const;
    type Move = { readonly type: "session"; readonly stage: (typeof stages)[number] | null; readonly fresh: boolean; readonly retarget: boolean; readonly problem: boolean };
    const toward = (from: string) => [
      { guard: ({ event }: { event: Move }) => event.stage === null, target: "#chrome.inactive", actions: [{ type: "answer", params: { reveal: false, focus: null } }] },
      { guard: ({ event }: { event: Move }) => event.stage === "reviewing" && event.problem && (event.fresh || from !== "reviewing"), target: "#chrome.reviewing", actions: [{ type: "answer", params: { reveal: true, focus: "band" } }] },
      ...stages.flatMap((stage) => [
        { guard: ({ event }: { event: Move }) => event.stage === stage && event.fresh, target: `#chrome.${stage}`, actions: [{ type: "answer", params: { reveal: true, focus: focusOf[stage] } }] },
        { guard: ({ event }: { event: Move }) => event.stage === stage && (from !== stage || (stage === "editing" && event.retarget)), target: `#chrome.${stage}`, actions: [{ type: "answer", params: { reveal: false, focus: focusOf[stage] } }] },
        { guard: ({ event }: { event: Move }) => event.stage === stage, target: `#chrome.${stage}`, actions: [{ type: "answer", params: { reveal: false, focus: null } }] },
      ]),
    ];
    const machine = createMachine({ id: "chrome", initial: "inactive", states: Object.fromEntries(["inactive", ...stages].map((state) => [state, { on: { session: toward(state) } }])) } as never);
    for (const row of corpus.transitions as readonly Transition[]) {
      const snapshot = machine.resolveState({ value: row.from?.stage ?? "inactive" } as never) as AnyMachineSnapshot;
      const [, actions] = transition(machine, snapshot, { type: "session", stage: row.to?.stage ?? null, fresh: row.to !== null && (row.from === null || row.from.sessionId !== row.to.sessionId), retarget: row.from?.target !== row.to?.target, problem: row.to?.nextProblem !== undefined } as never);
      const answer = (actions as unknown as readonly { readonly type: string; readonly params: { readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null } }[]).find((action) => action.type === "answer")!.params;
      expect(answer, `${row.name}: the statechart`).toEqual({ reveal: row.reveal, focus: row.focus });
      expect(timeTravelTransitionV1(row.from, row.to), `${row.name}: the shell`).toEqual({ ...answer, scrollTo: row.scrollTo });
    }
  });
});
//#endregion 🛰️ShellReveal
