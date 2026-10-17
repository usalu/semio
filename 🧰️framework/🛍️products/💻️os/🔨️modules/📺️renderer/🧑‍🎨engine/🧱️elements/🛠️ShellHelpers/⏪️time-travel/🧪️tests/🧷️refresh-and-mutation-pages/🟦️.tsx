/** 🧷️ A body refresh never remounts an input that is open, and every mutation of a history row is reachable, paged and named. */
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it } from "vitest";
import { createElement, Fragment, useLayoutEffect } from "react";
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { composeControlKeybindings, TREE_WINDOW_PATH_SEPARATOR, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { type ActionDescriptor, type BuiltNode, type HistoryTimeTravel } from "@semio-tech/framework";
import { builtNodeToSnapshot, UiDocumentStore } from "../../../../📃️UiDocumentStore/🟦️.tsx";
import { panelTabDefinitionToNode } from "../../../🛣️route-and-chrome/🟦️.tsx";
import { syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { HISTORY_ROW_KEY_PREFIX, TIME_TRAVEL_ROW_ACTION_REFUSALS, TIME_TRAVEL_ROW_ACTION_VERBS, TimeTravelBand, timeTravelRowActionRefusalsV1 } from "../../🟦️.tsx";
import { RowActionRefusalsContext } from "../../../../🗣️Interpreter/👥️presence-overlay/🟦️.ts";
import { LOCALES, ariaFindings, axes, bind, cases, computeAccessibleDescription, computeAccessibleName, controller, corpus, corpusLabel, draftArgs, historyTab, mountHistoryBody, node, overlay, rowActionCorpus, section, treeItem } from "../🧰️history-test-support/🟦️.tsx";

//#region 🧷️RefreshKeepsOpenInputs
/** 🧯️ The draft editor's input rows as the runtime builds them, behind `leading` band rows — a body refresh that adds or
 * drops a row ahead of the inputs (a fault line, a progress line, Next problem) renumbers every node after it. */
const inputsBody = (leading: number): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.timeTravel", "History editing", [treeItem("framework.history.timeTravel.status", "Editing a mutation", [], { icon: "clock" }), ...Array.from({ length: leading }, (_, index) => treeItem(`framework.history.timeTravel.line.${index}`, `Line ${index}`, []))]),
    section("framework.history.editor.inputs", "Inputs", [
      treeItem("framework.history.editor.input.factor.row", "Factor", [node("framework.history.editor.input.factor", { type: "slider", value: 1.5, min: 0.1, max: 10, step: 0.01, unit: null, snaps: [0.25, 0.5, 1, 2, 4], appearance: "track", scale: "log", precision: 2, displayUnit: null, displayFactor: null, limits: { min: { value: 0, exclusive: true, refusal: "Must be greater than 0" }, max: null } }, [], [bind(controller, "historyEditInput", draftArgs("/factor"), "change")], { label: "Factor" })], { description: "Spreads positions from the pivot." }),
      treeItem("framework.history.editor.input.dx.row", "dx", [node("framework.history.editor.input.dx", { type: "numberStepper", value: 80, step: 1, uniform: true, min: -1000, max: 1000, precision: null }, [], [bind(controller, "historyEditInput", draftArgs("/dx"), "change")], { label: "dx" })]),
      treeItem("framework.history.editor.input.name.row", "Name", [node("framework.history.editor.input.name", { type: "input", kind: "text", value: "Old", placeholder: null, commit: "blur", min: null, max: null, step: null, accept: null, precision: null, snaps: [] }, [], [bind(controller, "historyEditInput", draftArgs("/name"), "commit")], { label: "Name" })]),
      treeItem("framework.history.editor.input.note.row", "Note", [node("framework.history.editor.input.note", { type: "input", kind: "longText", value: "One", placeholder: null, commit: "blur", min: null, max: null, step: null, accept: null, precision: null, snaps: [] }, [], [bind(controller, "historyEditInput", draftArgs("/note"), "commit")], { label: "Note" })]),
    ]),
  ]);

describe("🧷️ a body refresh never remounts an input that is being edited (live fault F1)", () => {
  afterEach(() => cleanup());

  /** 🖼️ Mounts the History leaf over a store the test refreshes, as the shell's body-store cache does (`loadSnapshot`). */
  const mountInputs = (dispatched: ActionDescriptor[]) => {
    const store = new UiDocumentStore("panel:framework.panel.history");
    store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", inputsBody(0)));
    const leaf = panelTabDefinitionToNode(historyTab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, (action) => dispatched.push(action), 1, overlay);
    if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
    const source = leaf.trees[0]!.tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const view = render(createElement(Fragment, null, config.emptyState));
    return { ...view, refresh: (leading: number) => act(() => store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", inputsBody(leading)))) };
  };
  const readoutEditor = (container: HTMLElement) => container.querySelector<HTMLInputElement>('[data-slot="slider-content"] input[type="number"]');
  const control = (container: HTMLElement, key: string) => container.querySelector<HTMLElement>(`[id$="/framework.history.editor.input.${key}"]`);

  it("keeps an open slider readout editor — its typed text, its focus — and then refuses the typed value naming the bound", async () => {
    const user = userEvent.setup();
    const dispatched: ActionDescriptor[] = [];
    const view = mountInputs(dispatched);
    const heading = [...view.container.querySelectorAll<HTMLElement>('[data-slot="tree-section-row"]')].find((row) => row.textContent?.includes("Inputs"))!;
    act(() => heading.focus());
    await act(async () => {
      await user.dblClick(view.container.querySelector<HTMLElement>('[data-slot="slider-value"]')!);
    });
    const field = readoutEditor(view.container)!;
    expect(document.activeElement, "the readout editor takes focus from the tree row").toBe(field);
    fireEvent.change(field, { target: { value: "-5" } });
    for (const leading of [2, 0, 1]) {
      view.refresh(leading);
      expect([readoutEditor(view.container) === field, field.value, document.activeElement === field], `a refresh with ${leading} leading rows`).toEqual([true, "-5", true]);
    }
    await act(async () => {
      await user.keyboard("{Enter}");
    });
    expect([readoutEditor(view.container) === field, field.getAttribute("aria-invalid"), view.container.querySelector('[role="alert"]')?.textContent, dispatched], "the typed value is refused in place, naming the bound").toEqual([true, "true", "Must be greater than 0", []]);
  });

  it("keeps a stepper's field, a text draft and a multi-line draft — each element, its typed text and the focus", async () => {
    const view = mountInputs([]);
    const stepper = view.container.querySelector<HTMLInputElement>('[data-stepper-input="true"]')!;
    const name = control(view.container, "name") as HTMLInputElement;
    const note = control(view.container, "note") as HTMLTextAreaElement;
    expect([stepper !== null, name?.tagName, note?.tagName]).toEqual([true, "INPUT", "TEXTAREA"]);
    fireEvent.change(name, { target: { value: "New name" } });
    fireEvent.change(note, { target: { value: "One\nTwo" } });
    for (const [focused, leading] of [[stepper, 3], [name, 0], [note, 2]] as const) {
      act(() => focused.focus());
      view.refresh(leading);
      expect([view.container.querySelector('[data-stepper-input="true"]') === stepper, control(view.container, "name") === name, control(view.container, "note") === note, name.value, note.value, document.activeElement === focused], `a refresh with ${leading} leading rows`).toEqual([true, true, true, "New name", "One\nTwo", true]);
    }
  });
});
//#endregion 🧷️RefreshKeepsOpenInputs

//#region 🪟️MutationPages
/** 📚️ A history row as the runtime now builds it (gap N1): a windowed item over every mutation of its transaction — the
 * projected rows materialised, the rest paged on demand — and each mutation's Edit row action, disabled with its reason
 * in its name and no row activation while the session would refuse Begin (gap N15). */
const pagedHistoryBody = (refusal: string | null, editLabel: string): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    node("framework.history.commands", { type: "treeSection", label: "Commands", defaultOpen: true, headerToolbar: null, window: { rowExtent: "standard", total: 1, offset: 0 } }, [
      node(`${HISTORY_ROW_KEY_PREFIX}4`, { type: "treeItem", label: "Drag 40 items by (8, 0)", description: null, icon: "move", defaultOpen: true, draggable: null, dragData: null, dimmed: null, selected: null, window: { rowExtent: "standard", total: 40, offset: 0 }, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null },
        Array.from({ length: 8 }, (_, index) =>
          node(`framework.history.mutation.m-${index}`, { type: "treeItem", label: `Drag selection ${index + 1}`, description: null, icon: "circle", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [{ icon: "edit", label: editLabel, reason: refusal, verb: "historyEditBegin", placement: "row", disabled: refusal !== null }], target: { scope: controller, version: 1, args: { mutationId: `m-${index}` }, activation: refusal === null ? "historyEditBegin" : null } }),
        ),
      ),
    ]),
  ]);

/** 🎬️ Mutation rows as the runtime builds them since design §22.1: every row holds `[Edit, Withdraw]`, a row with an accepted
 * draft `[Edit, Restore]`, a withdrawn row `[Edit]` alone — each action its own verb on the row's ONE target (`{mutationId}`),
 * a refused one disabled with its reason. */
const actionRowsBody = (rows: readonly { readonly id: string; readonly label: string; readonly actions: readonly { readonly icon: string; readonly label: string; readonly verb: string; readonly reason?: string }[] }[]): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    node("framework.history.commands", { type: "treeSection", label: "Commands", defaultOpen: true, headerToolbar: null, window: null }, [
      node(`${HISTORY_ROW_KEY_PREFIX}4`, { type: "treeItem", label: "Arrange", description: null, icon: "move", defaultOpen: true, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null },
        rows.map((row) =>
          node(`framework.history.mutation.${row.id}`, { type: "treeItem", label: row.label, description: null, icon: "circle", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: row.actions.map((action) => ({ icon: action.icon, label: action.label, reason: action.reason ?? null, verb: action.verb, placement: "row", disabled: action.reason !== undefined })), target: { scope: controller, version: 1, args: { mutationId: row.id }, activation: null } }),
        ),
      ),
    ]),
  ]);

describe("🪟️ every mutation of a history row is reachable, and a refused Edit says why", () => {
  afterEach(() => cleanup());

  it("refuses every row action in the commit that shows a replaying or finalizing stage — from the session patch alone — and nothing flips when the guest's rows say the same", () => {
    expect(corpus.rowActions, "the React tables are the corpus's").toEqual({ verbs: [...TIME_TRAVEL_ROW_ACTION_VERBS], stages: TIME_TRAVEL_ROW_ACTION_REFUSALS });
    expect([timeTravelRowActionRefusalsV1(null), ...(["editing", "reviewing", "choosing"] as const).map((stage) => timeTravelRowActionRefusalsV1({ stage }))], "no session, or a stage whose rows speak for themselves: the shell refuses none").toEqual([null, null, null, null]);
    const rows = (reason?: string) =>
      actionRowsBody([
        { id: "m-0", label: "Drag selection", actions: [{ icon: "edit", label: "Edit", verb: "historyEditBegin", reason }, { icon: "eye-off", label: "Withdraw", verb: "historyEditWithdraw", reason }] },
        { id: "m-1", label: "Rotate", actions: [{ icon: "edit", label: "Edit", verb: "historyEditBegin", reason }, { icon: "rotate-ccw", label: "Restore", verb: "historyEditRestore", reason }] },
      ]);
    const sessionOf = (stage: HistoryTimeTravel["stage"]): HistoryTimeTravel => cases.find((entry) => entry.session.stage === stage)!.session;
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      const dispatched: ActionDescriptor[] = [];
      const store = new UiDocumentStore("panel:framework.panel.history");
      store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", rows()));
      const leaf = panelTabDefinitionToNode(historyTab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, (action) => dispatched.push(action), 1, overlay);
      if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
      const source = leaf.trees[0]!.tree;
      const body = ("resolveTree" in source ? source.resolveTree() : source).emptyState;
      const commits: (readonly [string | null, readonly (string | null)[]])[] = [];
      const Witness = () => {
        useLayoutEffect(() => {
          commits.push([document.querySelector("[data-semio-time-travel]")?.getAttribute("data-semio-time-travel") ?? null, [...document.querySelectorAll('[data-slot="action"]')].map((button) => button.getAttribute("aria-disabled"))]);
        });
        return null;
      };
      const bindings = composeControlKeybindings(new Map(), {});
      const Probe = ({ session }: { readonly session: HistoryTimeTravel }) =>
        createElement(RowActionRefusalsContext.Provider, { value: timeTravelRowActionRefusalsV1(session) }, createElement(UiKeybindingsProvider, { bindings, children: createElement(TimeTravelBand, { session, terminology: axes.terminology, locale, controllerId: corpus.controllerId, onAction: () => undefined }) }), body, createElement(Witness));
      const view = render(createElement(Probe, { session: sessionOf("reviewing") }));
      const actions = () => [...view.container.querySelectorAll<HTMLButtonElement>('[data-slot="action"]')];
      const read = () => actions().map((button) => [button.disabled, button.getAttribute("aria-disabled"), button.getAttribute("aria-describedby") === null ? null : computeAccessibleDescription(button)]);
      const mounted = actions();
      expect([mounted.length, read()], `reviewing: the rows speak for themselves (${locale})`).toEqual([4, mounted.map(() => [false, null, null])]);
      for (const stage of ["replaying", "finalizing"] as const) {
        const reason = corpusLabel(corpus.rowActions.stages[stage], locale);
        act(() => mounted[0]!.focus());
        commits.length = 0;
        view.rerender(createElement(Probe, { session: sessionOf(stage) }));
        const shown = commits.filter(([staged]) => staged === stage);
        expect([shown.length >= 1, shown.every(([, disabled]) => disabled.length === 4 && disabled.every((value) => value === "true"))], `${stage}: the commit that shows the stage already refuses every row action (${locale})`).toEqual([true, true]);
        expect([actions().every((button, index) => button === mounted[index]), read(), document.activeElement === mounted[0]], `${stage}: the same buttons, focusable, each naming why; the focus stays (${locale})`).toEqual([true, mounted.map(() => [false, "true", reason]), true]);
        for (const button of actions()) fireEvent.click(button);
        expect(dispatched, `${stage}: a refused row action dispatches nothing (${locale})`).toEqual([]);
        act(() => store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", rows(reason))));
        expect([actions().every((button, index) => button === mounted[index]), read()], `${stage}: the guest's refreshed rows say the same and nothing flips (${locale})`).toEqual([true, mounted.map(() => [false, "true", reason])]);
        act(() => store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", rows())));
        view.rerender(createElement(Probe, { session: sessionOf("reviewing") }));
        expect(read(), `after ${stage} the rows speak for themselves again (${locale})`).toEqual(mounted.map(() => [false, null, null]));
      }
      fireEvent.click(actions()[1]!);
      expect(dispatched.map((action) => action.action), `a runnable row action runs (${locale})`).toEqual(["historyEditWithdraw"]);
      view.unmount();
    }
    syncShellLabelLocale("en");
  });

  it("offers Edit beside Withdraw or Restore on a mutation row, each its own verb on the row's one target, a refused one naming why", () => {
    const edit = { icon: "edit", label: "Edit", verb: "historyEditBegin" };
    const withdraw = { icon: "eye-off", label: "Withdraw", verb: "historyEditWithdraw" };
    const restore = { icon: "rotate-ccw", label: "Restore", verb: "historyEditRestore" };
    const notEditable = "The inputs of this mutation cannot be edited";
    const dispatched: ActionDescriptor[] = [];
    const view = mountHistoryBody(actionRowsBody([
      { id: "m-0", label: "Drag selection", actions: [edit, withdraw] },
      { id: "m-1", label: "Rotate", actions: [edit, restore] },
      { id: "m-2", label: "Import", actions: [{ ...edit, reason: notEditable }, withdraw] },
      { id: "m-3", label: "Scale", actions: [edit] },
    ]), (action) => dispatched.push(action));
    const actions = (id: string) => [...view.container.querySelector<HTMLElement>(`[id$="/framework.history.mutation.${id}"]`)!.querySelectorAll<HTMLButtonElement>('[data-slot="action"]')];
    expect(["m-0", "m-1", "m-2", "m-3"].map((id) => actions(id).map((button) => computeAccessibleName(button)))).toEqual([["Edit", "Withdraw"], ["Edit", "Restore"], ["Edit", "Withdraw"], ["Edit"]]);
    const refused = actions("m-2")[0]!;
    expect([refused.getAttribute("aria-disabled"), refused.disabled, refused.tabIndex >= 0, computeAccessibleDescription(refused)], "a mutation whose inputs cannot be edited keeps its Edit, refused and saying why").toEqual(["true", false, true, notEditable]);
    for (const id of ["m-0", "m-1", "m-2", "m-3"]) for (const button of actions(id)) fireEvent.click(button);
    expect(dispatched.map((action) => [action.action, action.args])).toEqual([
      ["historyEditBegin", { mutationId: "m-0" }],
      ["historyEditWithdraw", { mutationId: "m-0" }],
      ["historyEditBegin", { mutationId: "m-1" }],
      ["historyEditRestore", { mutationId: "m-1" }],
      ["historyEditWithdraw", { mutationId: "m-2" }],
      ["historyEditBegin", { mutationId: "m-3" }],
    ]);
    expect(ariaFindings([view.container])).toEqual([]);
    view.unmount();
  });

  it("keeps a refused Edit focusable and naming its reason for every refusal a session stage names, in English and German — also while its own dispatch is still pending", async () => {
    const named = (corpus.refusals as readonly { readonly code: string; readonly text: Readonly<Record<"en" | "de", string>> }[]).filter((row) => row.code.startsWith("timeTravel."));
    expect(named.length).toBeGreaterThanOrEqual(18);
    for (const locale of LOCALES) {
      for (const row of named) {
        const view = mountHistoryBody(pagedHistoryBody(row.text[locale], "Edit"), () => undefined);
        const edits = [...view.container.querySelectorAll<HTMLButtonElement>('[data-slot="action"]')];
        expect([edits.length, edits.every((button) => !button.disabled && button.tabIndex >= 0 && button.getAttribute("aria-disabled") === "true" && computeAccessibleDescription(button) === row.text[locale])], `${row.code} (${locale}): never the disabled attribute, always the reason`).toEqual([8, true]);
        view.unmount();
      }
    }
    const dispatched: ActionDescriptor[] = [];
    const answers: (() => void)[] = [];
    const answer = () => act(async () => {
      answers.shift()!();
      await Promise.resolve();
      await Promise.resolve();
    });
    const store = new UiDocumentStore("panel:framework.panel.history");
    store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", pagedHistoryBody(null, "Edit")));
    const leaf = panelTabDefinitionToNode(historyTab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, (action) => (dispatched.push(action), new Promise<void>((resolve) => answers.push(resolve))) as unknown as void, 1, overlay);
    if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
    const source = leaf.trees[0]!.tree;
    const view = render(createElement(Fragment, null, ("resolveTree" in source ? source.resolveTree() : source).emptyState));
    const pressed = view.container.querySelector<HTMLButtonElement>('[data-slot="action"]')!;
    act(() => pressed.focus());
    await act(async () => {
      fireEvent.click(pressed);
    });
    expect([dispatched.map((action) => action.action), pressed.disabled, pressed.getAttribute("aria-busy"), pressed.getAttribute("aria-disabled"), document.activeElement === pressed], "the Edit whose Begin is still in flight is busy, not gone: it keeps the focus the press gave it").toEqual([["historyEditBegin"], false, "true", "true", true]);
    fireEvent.click(pressed);
    expect(dispatched.length, "a press while its own dispatch is in flight dispatches nothing").toBe(1);
    await answer();
    const answered = view.container.querySelector<HTMLButtonElement>('[data-slot="action"]')!;
    expect([answered === pressed, answered.disabled, answered.getAttribute("aria-busy"), answered.getAttribute("aria-disabled"), answered.getAttribute("aria-describedby"), document.activeElement === answered], "the verb's answer — a silent `{rejected}` result included — ends the pending state: the button is what the guest publishes again, enabled and without a reason, and keeps the focus").toEqual([true, false, null, null, null, true]);
    await act(async () => {
      fireEvent.click(answered);
    });
    expect([dispatched.length, answered.getAttribute("aria-busy")], "and it runs again").toEqual([2, "true"]);
    const reason = named.find((row) => row.code === "timeTravel.illegal")!.text.en;
    act(() => store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", pagedHistoryBody(reason, "Edit"))));
    const refused = view.container.querySelector<HTMLButtonElement>('[data-slot="action"]')!;
    expect([dispatched.length, refused === pressed, refused.disabled, computeAccessibleDescription(refused), document.activeElement === refused], "the session it opened refuses a second Begin: the same button, still focused, now says why").toEqual([2, true, false, reason, true]);
    await answer();
    expect([refused.getAttribute("aria-busy"), refused.getAttribute("aria-disabled"), computeAccessibleDescription(refused), document.activeElement === refused], "the answer to that Begin leaves the guest's refusal standing").toEqual([null, "true", reason, true]);
    view.unmount();
  });

  it("windows a history row over all its mutations under the path the runtime files its page request under", () => {
    const view = mountHistoryBody(pagedHistoryBody(null, "Edit"), () => undefined);
    const group = view.container.querySelector<HTMLElement>(`[data-tree-window-path="${`framework.history.commands${TREE_WINDOW_PATH_SEPARATOR}${HISTORY_ROW_KEY_PREFIX}4`}"]`);
    expect([group?.getAttribute("data-tree-window-total"), group?.getAttribute("data-tree-window-length"), group?.querySelector('[data-tree-window-spacer="trailing"]')?.getAttribute("data-tree-window-rows")]).toEqual(["40", "8", "32"]);
    const rows = [...group!.querySelectorAll<HTMLElement>("[data-tree-window-row]")].filter((row) => row.parentElement?.closest("[data-tree-window-path]") === group);
    expect(rows.map((row) => [row.getAttribute("role"), row.getAttribute("aria-posinset"), row.getAttribute("aria-setsize")]), "every materialised mutation row tells assistive technology its place among all 40").toEqual(Array.from({ length: 8 }, (_, index) => ["treeitem", String(index + 1), "40"]));
    view.unmount();
  });

  it("dispatches Edit while Begin is legal, and names the refusal and dispatches nothing while it is not, in English and German", () => {
    const dispatched: ActionDescriptor[] = [];
    const legal = mountHistoryBody(pagedHistoryBody(null, "Edit"), (action) => dispatched.push(action));
    fireEvent.click([...legal.container.querySelectorAll<HTMLButtonElement>("button")].find((button) => computeAccessibleName(button) === "Edit")!);
    expect(dispatched.map((action) => [action.action, action.args])).toEqual([["historyEditBegin", { mutationId: "m-0" }]]);
    legal.unmount();
    const illegal = (corpus.refusals as readonly { readonly code: string; readonly text: Readonly<Record<"en" | "de", string>> }[]).find((row) => row.code === "timeTravel.illegal")!;
    for (const [locale, edit] of [["en", "Edit"], ["de", "Bearbeiten"]] as const) {
      const reason = `${edit}: ${illegal.text[locale]}`;
      const expected = rowActionCorpus.find((row) => row.id === `disabled-explained-${locale}`)!;
      expect([edit, expected.focusable, expected.actionable]).toEqual([expected.label, true, false]);
      const refused = mountHistoryBody(pagedHistoryBody(illegal.text[locale], edit), (action) => dispatched.push(action));
      const edits = [...refused.container.querySelectorAll<HTMLButtonElement>("button")].filter((button) => `${computeAccessibleName(button)}: ${computeAccessibleDescription(button)}` === reason);
      for (const button of edits) expect([computeAccessibleName(button), computeAccessibleDescription(button)]).toEqual([expected.label, illegal.text[locale]]);
      expect([edits.length, edits.every((button) => button.getAttribute("aria-disabled") === "true" && !button.disabled && button.tabIndex >= 0)], `${locale}: a refused Edit stays focusable and is exposed as unavailable (aria-disabled), its name saying why`).toEqual([8, true]);
      for (const button of edits) fireEvent.click(button);
      for (const row of refused.container.querySelectorAll<HTMLElement>('[role="treeitem"]')) fireEvent.click(row);
      expect([dispatched.length, ariaFindings([refused.container])], `${locale}: a refused Edit and its row fire nothing`).toEqual([1, []]);
      refused.unmount();
    }
  });
});
//#endregion 🪟️MutationPages
