/** 🪪 Window action panel staging and the palette redirect and keybinding rule. */
import utilityActionPolicy from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧰️utility-action-policy/🔣️.json";
import utilityAssignmentCases from "../../🧱️elements/🐚️Shell/🧰️utility-assignment/🧪️tests/🧫️fixtures/🔣️.json";
import { manifestUtilitiesInitialFixture as initialWindowUtilityCases } from "../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🟦️.ts";
import { resolveUtilityActivationV1 as resolveAssignmentPress } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { createElement, useState, type ReactElement } from "react";
import { parseResolvedPluginViewState, windowViewContext } from "../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";
import Ajv2020 from "ajv/dist/2020";
import chordKeyTokensFixture from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/⌨️chord-key-tokens.json";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { actionSemanticsForKind, type ActionDefinition, type AppDefinition, type AppWindowKindDefinition, createMemoryStoragePort } from "@semio-tech/framework";
import { buildActiveUtilityByWindowId, initialShellState, actionStageKey, actionRequiresStagedForm, resolveKeybindingIntent, WindowActionPane, type ResolvedActionArgDef, type ResolvedActionDefinition, shellReducer } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { windowActionPaneNode } from "../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/🟦️.tsx";
import {
  chordUsesCanonicalKeyTokens,
  keyboardEventMatchesChord
} from "../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";
import {
  clipboardWriteFragmentFromEffect,
  pasteActionWithRetainedFragment,
  pasteArgsFragment
} from "../../🧱️elements/🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
import "../🧰️engine-contract-helpers/🟦️.ts";

// 🧰️ Window Actions & Utilities Contract (WS-2): staged argument forms (P1/P2), palette redirect (P3),
// keybinding rule (P4), and registry-derived utility activation (P5).

describe("window action panel — staging and single dispatch (P1/P2)", () => {
  afterEach(() => cleanup());

  const numberArg = (id: string, required: boolean, def?: number): ResolvedActionArgDef => ({ id, label: id[0]!.toUpperCase() + id.slice(1), schema: { kind: "number", integer: false }, required, ...(def === undefined ? {} : { default: def }) });

  const twoArgAction: ResolvedActionDefinition = { id: "extrude", label: "Extrude", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [numberArg("depth", true), numberArg("segments", true)] };
  const zeroArgAction: ResolvedActionDefinition = { id: "flatten", label: "Flatten", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] };
  const defaultedAction: ResolvedActionDefinition = { id: "bevel", label: "Bevel", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [numberArg("radius", true, 2)] };

  function Harness({ actions, onExecute, disabled }: { actions: readonly ResolvedActionDefinition[]; onExecute: (descriptor: unknown) => void; disabled?: boolean }): ReactElement {
    const [expanded, setExpanded] = useState<string | null>(null);
    const [staged, setStaged] = useState<Record<string, Record<string, unknown>>>({});
    return createElement(WindowActionPane, {
      windowId: "w1",
      controllerId: "c",
      actions,
      expandedActionId: expanded,
      stagedArgsByKey: staged,
      disabled: Boolean(disabled),
      onExpandedChange: setExpanded,
      onStageArg: (actionId, argId, value) => setStaged((prev) => ({ ...prev, [actionStageKey("w1", actionId)]: { ...(prev[actionStageKey("w1", actionId)] ?? {}), [argId]: value } })),
      onResetArgs: (actionId) =>
        setStaged((prev) => {
          const next = { ...prev };
          delete next[actionStageKey("w1", actionId)];
          return next;
        }),
      onExecute,
    });
  }

  const buttonByText = (container: HTMLElement, text: string): HTMLButtonElement => {
    const match = [...container.querySelectorAll("button")].find((button) => button.textContent?.includes(text));
    if (!match) throw new Error(`button "${text}" not found`);
    return match as HTMLButtonElement;
  };

  // 🌳️ Action rows render as Tree items (`role="treeitem"`), not `<button>`s — only the Execute/Reset
  // form actions render as real buttons. Row clicks (fire a zero-arg action, toggle an arg-carrying one)
  // go through this helper instead of `buttonByText`.
  const rowByText = (container: HTMLElement, text: string): HTMLElement => {
    const match = [...container.querySelectorAll('[role="treeitem"]')].find((row) => row.querySelector('[data-slot="tree-label"]')?.textContent?.trim() === text);
    if (!match) throw new Error(`tree row "${text}" not found`);
    return match as HTMLElement;
  };

  it("stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Extrude…"));
    const inputs = container.querySelectorAll('input[type="number"]');
    expect(inputs).toHaveLength(2);
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(onExecute).not.toHaveBeenCalled();
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "extrude", args: { depth: 3, segments: 2 } });
    // staged values survive Execute (tweak-and-repeat): the inputs still hold their values
    expect((container.querySelectorAll('input[type="number"]')[0] as HTMLInputElement).value).toBe("3");
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(2);
  });

  it("gates Execute on required args, but a default-satisfied required arg counts without staging", () => {
    const onExecute = vi.fn();
    const unavailable = (button: HTMLButtonElement) => [button.getAttribute("aria-disabled") === "true", button.disabled];
    const required = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(required.container, "Extrude…"));
    expect(unavailable(buttonByText(required.container, "Execute")), "a gated Execute stays focusable, aria-disabled").toEqual([true, false]);
    fireEvent.click(buttonByText(required.container, "Execute"));
    expect(onExecute).not.toHaveBeenCalled();
    const inputs = required.container.querySelectorAll('input[type="number"]');
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    expect(unavailable(buttonByText(required.container, "Execute"))).toEqual([true, false]);
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(unavailable(buttonByText(required.container, "Execute"))).toEqual([false, false]);
    cleanup();

    const defaulted = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(defaulted.container, "Bevel…"));
    expect(unavailable(buttonByText(defaulted.container, "Execute"))).toEqual([false, false]);
    fireEvent.click(buttonByText(defaulted.container, "Execute"));
    expect(onExecute).toHaveBeenLastCalledWith({ controllerId: "c", action: "bevel", args: { radius: 2 } });
  });

  it("Reset restores defaults while keeping the form expanded", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(container, "Bevel…"));
    const input = () => container.querySelector('input[type="number"]') as HTMLInputElement;
    expect(input().value).toBe("2");
    fireEvent.change(input(), { target: { value: "9" } });
    expect(input().value).toBe("9");
    fireEvent.click(buttonByText(container, "Reset"));
    // still expanded (Execute/Reset buttons present) and back to the default effective value
    expect(input().value).toBe("2");
    expect([...container.querySelectorAll("button")].some((b) => b.textContent?.includes("Execute"))).toBe(true);
  });

  it("a zero-arg action row fires immediately with no args object", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "flatten" });
  });

  it.each(["en", "de"] as const)("guest utility assignments preserve armed tools and window scope (%s)", (locale) => {
    const invalid = structuredClone(utilityAssignmentCases);
    Object.assign(invalid.cases[0]!.steps[0]!, { kind: "unknown" });
    for (const row of utilityAssignmentCases.cases) {
      let state = initialShellState({ locale: "en", plugins: [], locks: { locale, terminology: "native" }, storage: createMemoryStoragePort() });
      for (const step of row.steps) {
        const utilityId = step.kind === "press" ? resolveAssignmentPress(state.actionPane.activeUtilityByWindowId[step.windowId], step.utilityId) : step.utilityId || null;
        state = shellReducer(state, { type: "SET_ACTIVE_UTILITY", windowId: step.windowId, utilityId });
        expect(Object.fromEntries(utilityAssignmentCases.windowIds.map((window) => [window, state.actionPane.activeUtilityByWindowId[window] ?? null])), row.id).toEqual(step.expected);
      }
    }
  });

  it.each(["en", "de"] as const)("initial window utility decisions follow declared arms and preserve resolved registers (%s)", async (locale) => {
    const { resolveInitialWindowUtility } = await import("../../../../../../../🔨️modules/🛂️manifest/🪛️utilities/🌅️initial/🟦️.ts");
    const { createVersionedRegisterV1 } = await import("../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts");
    const ajv = new Ajv2020({ strict: true, $data: true });
    const fixture = {...initialWindowUtilityCases,cases:initialWindowUtilityCases.cases.map(row=>{
      const activeUtilityByWindowId:Record<string,string|null>={};
      for(const[id,value]of Object.entries(row.activeUtilityByWindowId)){if(typeof value!=="string"&&value!==null)throw Error("Invalid authored utility register");activeUtilityByWindowId[id]=value;}
      return {...row,activeUtilityByWindowId};
    })};
    for (const row of fixture.cases) {
      const authored = { utilities: row.utilityIds, ...(row.initialUtilityId === null ? {} : { initialUtilityId: row.initialUtilityId }) };
      const actual = resolveInitialWindowUtility(row);
      expect(actual, row.id).toEqual(row.expected);
      const register = createVersionedRegisterV1<string | null>(row.activeUtilityByWindowId[row.windowId] ?? null);
      const before = register.read();
      if (actual.write) expect(register.write(actual.utilityId, null).kind).toBe("applied");
      else expect(register.read()).toBe(before);
      expect(register.read().value).toBe(row.expected.utilityId);
      if (actual.utilityId !== null) {
        const rendered = register.read();
        expect(register.write(null, rendered.generation).kind).toBe("applied");
        const cleared = { ...row, activeUtilityByWindowId: { ...row.activeUtilityByWindowId, [row.windowId]: null } };
        expect(resolveInitialWindowUtility(cleared)).toEqual({ write: false, utilityId: null });
        expect(register.write(actual.utilityId, rendered.generation).kind).toBe("refused");
        expect(register.read().value).toBeNull();
      }
      const key = row.windowId.replaceAll("~", "~0").replaceAll("/", "~1");
      const oracle = ajv.compile({
        type: "object",
        required: ["activeUtilityByWindowId", "initialUtilityId", "activeToolId", "actual"],
        properties: {
          activeUtilityByWindowId: { type: "object" },
          initialUtilityId: { type: ["string", "null"] },
          activeToolId: { type: ["string", "null"] },
          actual: { type: "object", required: ["write", "utilityId"], properties: { write: { type: "boolean" }, utilityId: { type: ["string", "null"] } } },
        },
        allOf: [{
          if: { properties: { activeUtilityByWindowId: { type: "object", properties: { [row.windowId]: {} }, required: [row.windowId] } } },
          then: { properties: { actual: { type: "object", properties: { write: { const: false }, utilityId: { const: { $data: `/activeUtilityByWindowId/${key}` } } } } } },
          else: {
            properties: { actual: { type: "object", properties: { write: { const: true } } } },
            if: { properties: { activeToolId: { type: "null" } } },
            then: { properties: { actual: { type: "object", properties: { utilityId: { const: { $data: "/initialUtilityId" } } } } } },
            else: { properties: { actual: { type: "object", properties: { utilityId: { type: "null" } } } } },
          },
        }],
      });
      expect(oracle({ ...row, actual }), JSON.stringify(oracle.errors)).toBe(true);
    }
    expect(() => resolveInitialWindowUtility({ ...fixture.cases[0]!, initialUtilityId: "unknown" })).toThrow();
  });

  it("initial utility nullable admission preserves the resolved host context contract", async () => {
    const { testResolvedHostContext } = await import("../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts");
    testResolvedHostContext();
  });

  it.each(["en", "de"] as const)("initial utility registers preserve an explicit clear in the store and guest projection (%s)", (locale) => {
    const row = initialWindowUtilityCases.cases.find((row) => row.id === "fresh-window-without-initial")!;
    const state = initialShellState({ locale: "en", plugins: [], locks: { locale, terminology: "native" }, storage: createMemoryStoragePort() });
    const action = { type: "SET_ACTIVE_UTILITY" as const, windowId: row.windowId, utilityId: null };
    const cleared = shellReducer(state, action);
    expect(Object.hasOwn(cleared.actionPane.activeUtilityByWindowId, row.windowId)).toBe(true);
    expect(cleared.actionPane.activeUtilityByWindowId[row.windowId]).toBeNull();
    expect(shellReducer(cleared, action)).toBe(cleared);
    const projected = buildActiveUtilityByWindowId(cleared.actionPane.activeUtilityByWindowId);
    expect(projected).toEqual({ [row.windowId]: null });
    const view = parseResolvedPluginViewState({ locale, terminology: "native", windowInstances: [{ id: row.windowId, windowKindId: "drawing-composite" }], activeUtilityByWindowId: projected });
    expect(view.activeUtilityByWindowId).toEqual(projected);
    const addressed = windowViewContext({ ...view, activeUtilityId: "pen" }, row.windowId)!;
    expect(addressed.activeUtilityId).toBeUndefined();
    expect(addressed.activeUtilityByWindowId).toEqual(projected);
  });

  it("renders every row disabled when an active utility gates actions", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute, disabled: true }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).not.toHaveBeenCalled();
  });

  it.each(["en", "de"])("neutral utility declarations keep mounted palette commands executable (%s)", (locale) => {
    const actions = utilityActionPolicy.actions.map((id) => ({ id, label: { native: { en: id, de: id }, reuse: { en: id, de: id } }, iconId: "box", semantics: actionSemanticsForKind(id === "exportDocument" ? "view" : "mutation"), kind: id === "exportDocument" ? "view" : "mutation", inPalette: true, args: [] })) as ActionDefinition[];
    const windowKind = { id: utilityActionPolicy.windowKind, actions } as unknown as AppWindowKindDefinition;
    const execute = vi.fn();
    const { container, rerender } = render(createElement("div"));
    for (const policy of [...utilityActionPolicy.utilities, { id: "foreign-modal-tool", allowsActionsWhileActive: false }]) {
      const app = { controllerId: "policy-controller", windowKinds: [windowKind], utilities: [policy] } as unknown as AppDefinition;
      const before = execute.mock.calls.length;
      const node = windowActionPaneNode(app, windowKind, windowKind.id, { expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: { [windowKind.id]: policy.id } }, execute, vi.fn(), undefined, "native", locale);
      rerender(createElement("div", null, node));
      for (const id of utilityActionPolicy.actions) fireEvent.click(rowByText(container, id));
      expect(execute.mock.calls.slice(before).map(([action]) => action.action)).toEqual(policy.allowsActionsWhileActive ? utilityActionPolicy.actions : []);
    }
    cleanup();
  });

  it("groups actions into category sections like the command panel", () => {
    const createAction: ResolvedActionDefinition = { id: "box", label: "Box", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, category: "create", args: [] };
    const transformAction: ResolvedActionDefinition = { id: "move", label: "Move", iconId: "move", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, category: "transform", args: [] };
    const historyAction: ResolvedActionDefinition = { id: "undo", label: "Undo", iconId: "undo", semantics: actionSemanticsForKind("history"), kind: "history", inPalette: true, args: [] };
    const uncategorizedAction: ResolvedActionDefinition = { id: "flatten2", label: "Flatten2", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] };
    const { container } = render(createElement(Harness, { actions: [createAction, transformAction, historyAction, uncategorizedAction], onExecute: vi.fn() }));
    const textOf = (text: string) => [...container.querySelectorAll("*")].some((el) => el.textContent?.trim() === text && el.children.length === 0);
    expect(textOf("Create")).toBe(true);
    expect(textOf("Transform")).toBe(true);
    expect(textOf("History")).toBe(true);
    expect(textOf("Actions")).toBe(true);
    expect(rowByText(container, "Box")).toBeTruthy();
    expect(rowByText(container, "Move")).toBeTruthy();
    expect(rowByText(container, "Undo")).toBeTruthy();
    expect(rowByText(container, "Flatten2")).toBeTruthy();
  });

  // 🧹️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B53: the rail is a user-facing surface, so it carries the
  // SAME curation the command palette and the shell fallback menu already apply (`if (!action.inPalette)
  // continue`). Without it the puzzle3d rail rendered 96 rows — `worldPointerDown`, `suggestionsTick`,
  // `registerBrushMesh`, `interactionSelect`, `noteShellCommand`, … — and pushed `Export` to row 79 at
  // y=1990 inside an 807 px band, 1 125 px below the fold (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §3).
  const paneRowIds = (actions: readonly ActionDefinition[]): string[] => {
    const windowKind = { id: "main", actions } as unknown as AppWindowKindDefinition;
    const app = { controllerId: "c", windowKinds: [windowKind], utilities: [] } as unknown as AppDefinition;
    const node = windowActionPaneNode(app, windowKind, "main", { expandedByWindowId: {}, stagedArgsByKey: {}, activeUtilityByWindowId: {} }, vi.fn(), vi.fn());
    if (node === undefined) return [];
    return [...render(createElement("div", null, node)).container.querySelectorAll('[id^="action."]')].map((row) => row.id);
  };

  // 📜️ A LONG rail must still reach its staged-argument form. `Tree` clips itself, so a rail taller than
  // the pane used to hide its own last rows: generation3d's 32-row rail rendered a 960 px tree inside a
  // 901 px pane body at 1600×1000 and put the expanded `addWidget` form's `kind` combobox at y=1038 —
  // outside the viewport, `click` timed out, and the verb dispatched with no staged argument
  // (ticket 26/09/18 B3c §4.3, re-measured by FL1). jsdom lays nothing out, so what this law can hold is
  // the structure that makes the band reachable: the rail is the scroll container, it may shrink inside
  // its pane, and the staged form — control AND execute — renders inside it rather than beside it.
  it("a 32-row rail scrolls its own band and keeps the staged-argument form inside it", () => {
    const rows: ResolvedActionDefinition[] = Array.from({ length: 32 }, (_, index) => ({
      id: `row${index}`,
      label: `Row ${index}`,
      iconId: "box",
      semantics: actionSemanticsForKind("mutation"),
      kind: "mutation",
      inPalette: true,
      args:
        index === 31
          ? [
              {
                id: "kind",
                label: "Kind",
                schema: {
                  kind: "string",
                  options: [
                    { value: "inputSlider", label: "inputSlider" },
                    { value: "note", label: "note" },
                  ],
                },
                required: true,
              },
            ]
          : [],
    }));
    const { container } = render(createElement(Harness, { actions: rows, onExecute: vi.fn() }));
    const pane = container.querySelector('[data-slot="window-action-pane"]');
    expect(pane).toBeTruthy();
    const paneClass = pane!.className;
    expect(paneClass).toContain("overflow-y-auto");
    expect(paneClass).toContain("min-h-0");
    expect(paneClass).toContain("flex-1");
    expect(container.querySelectorAll('[id^="action.row"]').length).toBeGreaterThanOrEqual(32);
    fireEvent.click(rowByText(container, "Row 31…"));
    const control = pane!.querySelector('[id="action.row31.arg.kind"]');
    const execute = [...pane!.querySelectorAll("button")].find((button) => button.id.includes("row31") && button.id.endsWith("execute"));
    expect(control).toBeTruthy();
    expect(execute).toBeTruthy();
  });

  it("renders only palette-eligible rows, and no rail at all when every declared action is dispatch plumbing", () => {
    const action = (id: string, inPalette: boolean, category?: string): ActionDefinition => ({
      id,
      label: id,
      iconId: "box",
      semantics: actionSemanticsForKind("shell"),
      kind: "shell",
      inPalette,
      ...(category === undefined ? {} : { category }),
      args: [],
    });
    expect(paneRowIds([action("worldPointerDown", false), action("exportSnapshot", true, "file"), action("interactionSelect", false)])).toEqual(["action.category.file", "action.exportSnapshot"]);
    cleanup();
    expect(paneRowIds([action("worldPointerDown", false), action("noteShellCommand", false)])).toEqual([]);
  });
});

describe("palette redirect and keybinding rule (P3/P4)", () => {
  const argAction: ActionDefinition = {
    id: "extrude",
    label: "Extrude",
    iconId: "box",
    semantics: actionSemanticsForKind("mutation"),
    kind: "mutation",
    inPalette: true,
    args: [{ id: "depth", label: "Depth", schema: { kind: "number", integer: false }, required: true }],
  };
  const zeroAction: ActionDefinition = { id: "flatten", label: "Flatten", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] };

  it("only actions with a user-facing arg redirect to a staged form (P3 decision)", () => {
    const hiddenAction: ActionDefinition = { ...zeroAction, id: "toolRunStart", args: [{ id: "toolId", label: "Tool", schema: { kind: "string", options: [] }, presentation: { kind: "hidden" }, required: true }] };
    expect(actionRequiresStagedForm(argAction)).toBe(true);
    expect(actionRequiresStagedForm(zeroAction)).toBe(false);
    expect(actionRequiresStagedForm(hiddenAction)).toBe(false);
    expect(actionRequiresStagedForm({ args: [...hiddenAction.args!, ...argAction.args!] })).toBe(true);
  });

  it("matches chord key tokens against event.key verbatim per ⌨️chord-key-tokens fixture", () => {
    const forbidden = chordKeyTokensFixture.forbiddenKeyNames as string[];
    for (const name of forbidden) {
      expect(chordUsesCanonicalKeyTokens(`mod+${name}`)).toBe(false);
      expect(keyboardEventMatchesChord({ key: ".", ctrlKey: true, metaKey: false, shiftKey: false, altKey: false }, `mod+${name}`)).toBe(false);
    }
    for (const row of chordKeyTokensFixture.canonical as { chord: string; eventKey: string }[]) {
      expect(chordUsesCanonicalKeyTokens(row.chord)).toBe(true);
      expect(keyboardEventMatchesChord({ key: row.eventKey, ctrlKey: true, metaKey: false, shiftKey: false, altKey: false }, row.chord)).toBe(true);
    }
  });

  it("keybinding intent: arg-less fires, arg-action opens unless already expanded and valid then executes (P4)", () => {
    expect(resolveKeybindingIntent(zeroAction, null, {})).toEqual({ kind: "fire" });
    expect(resolveKeybindingIntent(undefined, null, {})).toEqual({ kind: "fire" });
    // not expanded → open
    expect(resolveKeybindingIntent(argAction, null, {})).toEqual({ kind: "open", actionId: "extrude" });
    expect(resolveKeybindingIntent(argAction, "other", { depth: 3 })).toEqual({ kind: "open", actionId: "extrude" });
    // expanded but required arg missing → stays open, never silent-fires
    expect(resolveKeybindingIntent(argAction, "extrude", {})).toEqual({ kind: "open", actionId: "extrude" });
    // expanded and valid → execute with merged effective args
    expect(resolveKeybindingIntent(argAction, "extrude", { depth: 4 })).toEqual({ kind: "execute", actionId: "extrude", args: { depth: 4 } });
  });

  it("retains a clipboardWrite fragment and injects it onto the next paste dispatch", () => {
    const fragment = { objects: [{ id: "slab-1" }] };
    expect(clipboardWriteFragmentFromEffect({ notify: { message: "x" } })).toBeUndefined();
    expect(clipboardWriteFragmentFromEffect({ clipboardWrite: { fragment } })).toBe(fragment);
    expect(pasteActionWithRetainedFragment({ action: "copy" }, fragment)).toEqual({ action: "copy" });
    expect(pasteActionWithRetainedFragment({ action: "paste" }, undefined)).toEqual({ action: "paste" });
    expect(pasteActionWithRetainedFragment({ action: "paste", args: { fragment: { kept: true } } }, fragment)).toEqual({ action: "paste", args: { fragment: { kept: true } } });
    expect(pasteActionWithRetainedFragment({ action: "paste" }, fragment)).toEqual({ action: "paste", args: { fragment } });
    expect(pasteArgsFragment({ args: { fragment } })).toBe(fragment);
    expect(pasteArgsFragment({ args: {} })).toBeUndefined();
  });
});
