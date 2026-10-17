/** 📝️ The Rust-shaped history draft editor: a session change reveals the History panel and focuses the right control, and the editor is named, operable by keyboard alone and ARIA-valid. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { UIDialog } from "@semio-tech/ui-react";
import { type ActionDescriptor, type BuiltNode } from "@semio-tech/framework";
import { syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { scheduleTimeTravelFocusV1, timeTravelBandControlsV1, timeTravelFocusElementV1, timeTravelFocusIsHeldV1, timeTravelTransitionV1 } from "../../🟦️.tsx";
import { TREE_ROW_TONE_CLASSES } from "../../../../🗣️Interpreter/🌳️tree-walk/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
import { EditorKeys, GENERATION, LOCALES, Transition, ariaFindings, band, bind, button, byIdSuffix, cases, computeAccessibleName, controlButton, controller, corpus, corpusLabel, draftArgs, draftBody, editingSession, framework, historyTab, mountBand, mountHistoryBody, nextFrames, node, readJson, section, shellHelpers, treeItem } from "../🧰️history-test-support/🟦️.tsx";

//#region ⏪️DraftEditor
describe("🧭️ a session change reveals the History panel and moves focus where the session continues", () => {
  afterEach(() => {
    cleanup();
    syncShellLabelLocale("en");
  });

  it("reveals and focuses exactly as the shared corpus's transitions say", () => {
    const transitions = corpus.transitions as readonly Transition[];
    expect(new Set(transitions.map((row) => row.focus))).toEqual(new Set(["editor", "band", "dialog", null]));
    for (const row of transitions) expect(timeTravelTransitionV1(row.from, row.to), row.name).toEqual({ reveal: row.reveal, focus: row.focus, scrollTo: row.scrollTo });
    expect(transitions.filter((row) => row.scrollTo !== null).map((row) => [row.reveal, row.to?.stage, row.scrollTo === `framework.history.mutation.${row.to?.nextProblem?.mutationId}`]), "a scroll target is the row of the mutation the session names, on a revealing edge into a review").toEqual(transitions.filter((row) => row.scrollTo !== null).map(() => [true, "reviewing", true]));
  });

  it("offers Next problem first in a blocked review and opens the mutation the session names — never one the shell computed", () => {
    const blocked = cases.filter((entry) => entry.session.nextProblem !== undefined);
    expect(blocked.map((entry) => [entry.session.stage, entry.controls[0]?.control, entry.controls[0]?.args]), "the corpus names a next problem with and without a member store").toEqual([["reviewing", "nextProblem", { mutationId: "m-5" }], ["reviewing", "nextProblem", { mutationId: "m-7", store: "parts/p-1" }]]);
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const entry of blocked) {
        const view = mountBand(entry.session, locale);
        const next = controlButton(view.container, "nextProblem")!;
        expect([next.textContent, next.id, next.getAttribute("aria-disabled"), band(view.container).querySelector("button")], `${entry.name} (${locale})`).toEqual([corpusLabel("ui.timeTravel.nextProblem", locale), "shell.time-travel.next-problem", null, next]);
        next.click();
        expect(view.dispatched, `${entry.name} (${locale})`).toEqual([{ controllerId: corpus.controllerId, action: "historyEditBegin", args: entry.session.nextProblem }]);
        view.unmount();
      }
    }
    const silent = cases.filter((entry) => entry.session.stage === "reviewing" && entry.session.nextProblem === undefined);
    expect([silent.length > 0, silent.every((entry) => timeTravelBandControlsV1(entry.session).every((row) => row.control !== "nextProblem"))], "a review whose session names no next problem — blocked or not — offers none").toEqual([true, true]);
  });

  it("finds the draft's first input, else its Accept, the band and the prompt in the shell's own DOM", () => {
    const withInputs = mountHistoryBody(draftBody(true), () => undefined);
    expect(timeTravelFocusElementV1(withInputs.container, "editor")?.id.endsWith("/framework.history.editor.input.dx")).toBe(true);
    withInputs.unmount();
    const withoutInputs = mountHistoryBody(draftBody(false), () => undefined);
    const accept = timeTravelFocusElementV1(withoutInputs.container, "editor");
    expect([accept?.tagName, accept?.closest("[id]")?.id.endsWith("/framework.history.editor.accept.row"), accept === null ? "" : computeAccessibleName(accept)]).toEqual(["BUTTON", true, "Accept"]);
    withoutInputs.unmount();
    const view = mountBand(editingSession, "en");
    expect(timeTravelFocusElementV1(view.container, "band")).toBe(band(view.container));
    expect([band(view.container).tabIndex, timeTravelFocusElementV1(view.container, "dialog")]).toEqual([-1, null]);
    const prompt = document.createElement("div");
    prompt.setAttribute("role", "dialog");
    prompt.innerHTML = '<p>Finish editing history</p><input id="prompt-name"><button type="button">Back</button>';
    view.container.append(prompt);
    expect(timeTravelFocusElementV1(view.container, "dialog")?.id).toBe("prompt-name");
  });

  it("resolves focus on the shared corpus's editor keys — the keys the Rust producer is held to — inside the History panel", () => {
    const keys = corpus.editorKeys as EditorKeys;
    const flattened = (pointer: string) => `framework.history.editor.input${pointer.replaceAll("/", ".")}`;
    expect([historyTab.kind.id, keys.inputs.map((row) => [row.control, row.row])]).toEqual([keys.panel, keys.inputs.map((row) => [flattened(row.pointer), `${flattened(row.pointer)}.row`])]);
    const field = (key: string, pointer: string) => node(key, { type: "input", kind: "number", value: "1", placeholder: null, commit: "blur", min: null, max: null, step: null, accept: null, precision: null, snaps: [] }, [], [bind(controller, "historyEditInput", draftArgs(pointer), "commit")], { label: pointer });
    const editor = (inputs: boolean): BuiltNode =>
      node("framework.history", { type: "tree", interactionDomain: null }, [
        section("framework.history.editor", "Drag selection", [treeItem(keys.accept.row, "Accept", [button(keys.accept.control, "Accept", bind(controller, "historyEditAccept", { generation: GENERATION }))])]),
        ...(inputs ? [section("framework.history.editor.inputs", "Inputs", keys.inputs.map((row) => treeItem(row.row, row.pointer, [field(row.control, row.pointer)])))] : []),
      ]);
    const withInputs = mountHistoryBody(editor(true), () => undefined);
    const first = timeTravelFocusElementV1(withInputs.container, "editor")!;
    expect([first.id.startsWith(`panel:${keys.panel}/`), first.id.endsWith(`/${keys.inputs[0]!.control}`), timeTravelFocusIsHeldV1(first)]).toEqual([true, true, false]);
    expect(keys.inputs.map((row) => byIdSuffix(withInputs.container, row.control)?.closest(`[id$="/${row.row}"]`) != null)).toEqual(keys.inputs.map(() => true));
    withInputs.unmount();
    const withoutInputs = mountHistoryBody(editor(false), () => undefined);
    const accept = timeTravelFocusElementV1(withoutInputs.container, "editor");
    expect([accept?.tagName, accept?.closest("[id]")?.id.endsWith(`/${keys.accept.row}`)]).toEqual(["BUTTON", true]);
    withoutInputs.unmount();
  });

  it("moves focus once its target mounts, keeps the prompt's own focus and never takes it from someone typing elsewhere", async () => {
    const root = document.createElement("div");
    document.body.append(root);
    const cancel = scheduleTimeTravelFocusV1(root, "band");
    await nextFrames(2);
    root.innerHTML = '<div data-semio-time-travel="editing" role="status" tabindex="-1"></div>';
    await nextFrames(2);
    expect(document.activeElement).toBe(root.firstElementChild);
    cancel();
    const chat = document.createElement("input");
    document.body.append(chat);
    chat.focus();
    scheduleTimeTravelFocusV1(root, "band");
    await nextFrames(2);
    expect([document.activeElement === chat, timeTravelFocusIsHeldV1(chat)]).toEqual([true, true]);
    const historyField = document.createElement("input");
    historyField.id = "panel:framework.panel.history/framework.history.editor.input.dx";
    const promptField = document.createElement("input");
    const prompt = document.createElement("div");
    prompt.setAttribute("role", "dialog");
    prompt.append(promptField);
    root.append(historyField, prompt);
    expect([timeTravelFocusIsHeldV1(historyField), timeTravelFocusIsHeldV1(promptField), timeTravelFocusIsHeldV1(document.createElement("button")), timeTravelFocusIsHeldV1(null)]).toEqual([false, false, false, false]);
    promptField.focus();
    scheduleTimeTravelFocusV1(root, "dialog");
    await nextFrames(2);
    expect(document.activeElement).toBe(promptField);
    chat.remove();
    root.remove();
  });
});

describe("♿️ the draft editor is named, operable by keyboard alone and says each outcome in words, icon and tone", () => {
  afterEach(() => cleanup());

  it("names every input control and dispatches each one's draft verb with the session generation", async () => {
    const dispatched: ActionDescriptor[] = [];
    const view = mountHistoryBody(draftBody(true), (action) => {
      dispatched.push(action);
      return Promise.resolve();
    });
    const dx = byIdSuffix(view.container, "framework.history.editor.input.dx") as HTMLInputElement;
    const slider = view.container.querySelector<HTMLElement>('[role="slider"]')!;
    const mode = byIdSuffix(view.container, "framework.history.editor.input.mode")!;
    const chip = byIdSuffix(view.container, "framework.history.editor.input.targets.chip.0")!;
    const useSelection = byIdSuffix(view.container, "framework.history.editor.input.targets.useSelection")!;
    expect([computeAccessibleName(dx), [dx.min, dx.max, dx.step], computeAccessibleName(slider), slider.getAttribute("aria-valuetext"), view.container.querySelectorAll('[data-slot="slider-tick"]').length, computeAccessibleName(mode), computeAccessibleName(chip), computeAccessibleName(useSelection)]).toEqual(["dx", ["-1000", "1000", "1"], "Angle", "0 °", 3, "Mode", "Remove node-1", "Use selection"]);
    const user = userEvent.setup();
    await act(async () => {
      dx.focus();
      await user.keyboard("{ArrowUp}");
      dx.blur();
    });
    await act(async () => {
      await user.click(chip);
      await user.click(useSelection);
    });
    const pivotX = byIdSuffix(view.container, "framework.history.editor.input.pivot.0") as HTMLInputElement;
    expect([computeAccessibleName(pivotX), pivotX.min, pivotX.max, pivotX.step]).toEqual(["x", "-500", "500", "0.5"]);
    await act(async () => {
      await user.clear(pivotX);
      await user.type(pivotX, "12.5{Enter}");
    });
    const drafts = dispatched.map(({ action, args }) => ({ action, path: (args as { path?: string }).path, value: (args as { value?: unknown }).value, generation: (args as { generation?: number }).generation }));
    expect(drafts.filter((draft) => draft.path === "/pivot/0")).toEqual([{ action: "historyEditInput", path: "/pivot/0", value: 12.5, generation: GENERATION }]);
    expect(drafts.filter((draft) => draft.path === "/dx").at(-1)).toEqual({ action: "historyEditInput", path: "/dx", value: 81, generation: GENERATION });
    expect(drafts.filter((draft) => draft.path === "/targets")).toEqual([
      { action: "historyEditInput", path: "/targets", value: ["node-2"], generation: GENERATION },
      { action: "historyEditUseSelection", path: "/targets", value: undefined, generation: GENERATION },
    ]);
  });

  it("lists the main line and each alternative, says which is current in words, and switches by a named button", async () => {
    const dispatched: ActionDescriptor[] = [];
    const view = mountHistoryBody(draftBody(true), (action) => {
      dispatched.push(action);
      return Promise.resolve();
    });
    const text = view.container.textContent ?? "";
    expect(["Main line", "Current", "Variant", "Edited history"].every((line) => text.includes(line))).toBe(true);
    const variant = byIdSuffix(view.container, "framework.history.alternative.alt-1")!;
    const switches = [...variant.querySelectorAll<HTMLButtonElement>("button")].filter((element) => computeAccessibleName(element) === "Switch");
    expect([switches.length, byIdSuffix(view.container, "framework.history.alternative.trunk")!.querySelectorAll("button").length > 0 && [...byIdSuffix(view.container, "framework.history.alternative.trunk")!.querySelectorAll("button")].some((element) => computeAccessibleName(element) === "Switch")]).toEqual([1, false]);
    await act(async () => {
      await userEvent.setup().click(switches[0]!);
    });
    expect(dispatched.map(({ controllerId, action, args }) => ({ controllerId, action, args }))).toEqual([{ controllerId: controller, action: "switchAlternative", args: { alternativeId: "alt-1" } }]);
  });

  it("reaches every editor control by Tab, in row order", async () => {
    const view = mountHistoryBody(draftBody(true), () => undefined);
    const user = userEvent.setup();
    const wanted = ["framework.history.editor.accept", "framework.history.editor.input.dx", "framework.history.editor.input.mode", "framework.history.editor.input.pivot.0", "framework.history.editor.input.pivot.1", "framework.history.editor.input.targets.chip.0", "framework.history.editor.input.targets.chip.1", "framework.history.editor.input.targets.useSelection"];
    const reached: string[] = [];
    for (let step = 0; step < 40 && reached.length < wanted.length + 1; step += 1) {
      await act(async () => {
        await user.tab();
      });
      const active = document.activeElement as HTMLElement | null;
      const owner = active?.id ? active.id : (active?.closest("[id]")?.id ?? "");
      const key = active?.getAttribute("role") === "slider" ? "slider" : (wanted.find((candidate) => owner.endsWith(`/${candidate}`) || owner.endsWith(`/${candidate}.row`)) ?? null);
      if (key !== null && !reached.includes(key)) reached.push(key);
    }
    expect(reached).toEqual([wanted[0], wanted[1], "slider", ...wanted.slice(2)]);
    view.unmount();
  }, 30_000);

  it("says each outcome in words and icon and tones its row by severity, never by colour alone", () => {
    const view = mountHistoryBody(draftBody(true), () => undefined);
    const rowOf = (text: string) => [...view.container.querySelectorAll<HTMLElement>("*")].find((element) => element.childElementCount === 0 && element.textContent === text) ?? null;
    const toneOf = (text: string) => Object.values(TREE_ROW_TONE_CLASSES).find((name) => rowOf(text)?.closest(`.${name}`)) ?? null;
    expect(["Warning: Partially applied", "Error: Target missing", "Edited", "Draft: Drag selection · Error: Target missing", "Editing a mutation: Drag selection · Error"].map(toneOf)).toEqual(["text-warning", "text-destructive", null, "text-destructive", "text-destructive"]);
    for (const text of ["Warning: Partially applied", "Error: Target missing"]) expect(rowOf(text)?.closest("[id]")?.querySelector("svg"), text).not.toBeNull();
  });

  it("fits a phone: the band wraps within 90 % of the viewport, paints its own opaque surface, and every control is at least a 24 × 24 touch target from the size tokens", () => {
    const styles = join(framework, "🔨️modules", "🖱️ui", "🎨️styling");
    const [uiCss, paletteCss] = [readFileSync(join(styles, "🖌️ui", "🎨️.css"), "utf8"), readFileSync(join(styles, "🎨️palette", "🎨️.css"), "utf8")];
    const multiplier = Number(/--size-large:\s*calc\(([0-9.]+) \* var\(--ui-spacing\)\)/u.exec(uiCss)?.[1]);
    const spacingPx = (density: "compact" | "touch"): number => {
      const found = new RegExp(`--spacing-${density}:\\s*([0-9.]+)(px|rem)`, "u").exec(paletteCss)!;
      return Number(found[1]) * (found[2] === "rem" ? 16 : 1);
    };
    expect([/@utility min-h-large \{\s*min-height: var\(--size-large\);/u.test(uiCss), /@utility min-w-large \{\s*min-width: var\(--size-large\);/u.test(uiCss), /@utility ui-surface \{\s*background-color: var\(--surface-bg\);/u.test(uiCss), /--color-menu\b/u.test(uiCss)], "the utilities the band wears exist, and `bg-menu` names no colour").toEqual([true, true, true, false]);
    expect([multiplier * spacingPx("compact") >= 24, multiplier * spacingPx("touch") >= 24], `the large size token is ${multiplier} × --ui-spacing: ${multiplier * spacingPx("compact")} px compact, ${multiplier * spacingPx("touch")} px touch (WCAG 2.5.8 asks 24 × 24)`).toEqual([true, true]);
    for (const entry of cases) {
      const view = mountBand(entry.session, "en");
      const status = band(view.container);
      expect([status.classList.contains("max-w-[90vw]"), status.classList.contains("flex-wrap"), status.getAttribute("data-level"), status.classList.contains("ui-surface"), status.classList.contains("bg-menu")], entry.name).toEqual([true, true, "panel", true, false]);
      for (const control of status.querySelectorAll("button")) expect([control.classList.contains("min-h-large"), control.classList.contains("min-w-large")], `${entry.name}: ${control.textContent}`).toEqual([true, true]);
      view.unmount();
    }
  });

  it("mounts the shell's bottom bands in the layout's subfooter row — in flow, never an absolute overlay over the footer or a panel", () => {
    const host = readModuleTreeSource(join(shellHelpers, "..", "🏛️ShellHost", "🟦️.tsx"));
    const slot = host.slice(host.indexOf("subfooter={"), host.indexOf("panels={Object.fromEntries"));
    expect([host.split("subfooter={").length - 1, slot.includes("<TimeTravelBand "), slot.includes("<HistoryReprojectionStatus "), slot.includes("<LocalFolderReconnectBand"), slot.includes("data-semio-checkin-band"), /\babsolute\b|\bfixed\b/u.test(slot)], "every bottom band sits in the one subfooter slot, none positioned out of flow").toEqual([1, true, true, true, true, false]);
    expect(host.slice(0, host.indexOf("subfooter={")).includes("<TimeTravelBand "), "no second mount of the band outside the slot").toBe(false);
    expect(/focusedTimeTravel !== null \|\| focusedReprojection !== null \|\|/u.test(slot), "a replay with no session open still shows its status").toBe(true);
  });

  it("uses only ARIA attributes each role supports, references only ids that exist and names every control", () => {
    const view = mountBand(editingSession, "en");
    const body = mountHistoryBody(draftBody(true), () => undefined);
    expect(ariaFindings([view.container, body.container])).toEqual([]);
  });

  it("opens the finalize prompt with focus on its name field, a destructive Overwrite, Back, and no ARIA finding", () => {
    const choices = readJson(join(framework, "🔨️modules", "🛂️manifest", "🧪️tests/🧫️fixtures", "🧫️dialog-choices", "🔣️.json"));
    const field = (def: { readonly id: string }, value: unknown, change: (value: unknown) => void, binding?: { readonly id: string; readonly labelledBy?: string; readonly required?: boolean }) => createElement("input", { id: binding?.id, "aria-labelledby": binding?.labelledBy, required: binding?.required, value: String(value ?? ""), onChange: (event: { readonly target: { readonly value: string } }) => change(event.target.value), "data-field": def.id });
    const view = render(createElement(UIDialog, { dialog: choices.dialog, seedArgs: choices.seed, renderField: field as never, onSubmit: () => undefined, onChoose: () => undefined, onCancel: () => undefined } as never));
    const prompt = document.querySelector<HTMLElement>('[role="dialog"]')!;
    const first = timeTravelFocusElementV1(document, "dialog")!;
    expect([first.getAttribute("data-field"), computeAccessibleName(first)]).toEqual(["name", "Alternative name"]);
    const overwrite = prompt.querySelector<HTMLElement>('[data-dialog-choice="overwrite"]')!;
    expect([computeAccessibleName(overwrite), overwrite.getAttribute("data-destructive")]).toEqual(["Overwrite", "true"]);
    expect([...prompt.querySelectorAll("button")].some((button) => computeAccessibleName(button) === "Back")).toBe(true);
    expect(ariaFindings([prompt])).toEqual([]);
    view.unmount();
  });
});
//#endregion ⏪️DraftEditor
