/** 🧪️ The React time-travel chrome against the language-neutral band corpus (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`):
 * every stage's lines in English and German, the controls with what disables them, the reserved verbs they dispatch with the
 * session generation, the remappable chords (never Escape, never from a form field) published on `aria-keyshortcuts`, the
 * window indicator's accessible name, the framework history body rendered through the interpreter instead of a
 * host-built tab, and peers' open history edits against the peers corpus the wgpu shell asserts too
 * (`🧫️time-travel-peers`). The corpus's `transitions` pin when the History panel is revealed and where focus moves; the
 * Rust-shaped draft editor (band, editor and input rows holding a stepper, a slider with snaps, a select and a reference
 * list) is operated by keyboard alone, every control is named, its outcome rows say their severity in words, icon and tone,
 * and every ARIA attribute is one its role allows; no checkpoint reaches a frozen history (e2e R2-6). Third-party oracles:
 * Ajv validates the corpus against its schema and the kernel's own `HistoryTimeTravel` wire schema, `dom-accessibility-api`
 * names every control, `aria-query` (the WAI-ARIA role model) lists the attributes each role supports,
 * `@testing-library/user-event` types the chords and tabs through the editor, and the `⏪️time-travel` module's
 * `TIME_TRAVEL_LABELS` is the independent source of every stage and refusal text. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { createRequire } from "node:module";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import userEvent from "@testing-library/user-event";
import { afterAll, afterEach, describe, expect, it } from "vitest";
import { createElement, Fragment, useCallback, useReducer, useState } from "react";
import { createMachine, transition, type AnyMachineSnapshot } from "xstate";
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { composeControlKeybindings, PresenceBar, SHELL_KEYBINDINGS, TREE_WINDOW_PATH_SEPARATOR, UIDialog, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { encodePresenceHistoryEdit, encodePresenceInteraction, encodePresenceToolRun } from "@semio-tech/framework-replication";
import { createMemoryStoragePort, type ActionDescriptor, type BuiltNode, type HistoryEntry, type HistoryPatch, type HistoryTimeTravel, type UiIntent } from "@semio-tech/framework";
import type { PanelDock, PanelTabNode } from "@semio-tech/ui-react";
import { initialShellState, shellReducer, type ShellAction, type ShellState } from "../../../../🐚️Shell/🟦️.tsx";
import { builtNodeToSnapshot, UiDocumentStore } from "../../../../📃️UiDocumentStore/🟦️.tsx";
import { checkinSubmitMessageV1, checkpointGateV1, checkpointOnCloseKeyV1, presenceEphemeralPeerFieldsV1, EMPTY_SHELL_HISTORY_PROJECTION_V1, FRAMEWORK_CHECKIN_CONTROLLER_ID, HISTORY_REFUSAL_LABEL_KEYS, historyPatchShouldApplyV1, historyRefusalCodeV1, historyRefusalNoticeV1, historyRefusalOfOutputV1, operationProgressPartsV1, panelActionRoutesThroughHostV1, panelTabDefinitionToNode, shellHistoryCursorDomV1, shellHistoryProjectionAfterPatchV1, shellLabel, syncShellLabelLocale, useCheckpointOnCloseV1, type ShellHistoryProjectionV1 } from "../../../🟦️.tsx";
import { HISTORY_ROW_KEY_PREFIX, HistoryReprojectionStatus, historyReprojectionControlV1, revealHistoryPanelV1, scheduleTimeTravelFocusV1, TIME_TRAVEL_CHORD_IDS, TimeTravelBand, timeTravelBandControlsV1, timeTravelBandTextV1, timeTravelControlActionV1, timeTravelFocusElementV1, timeTravelFocusIsHeldV1, timeTravelIndicatorTextV1, timeTravelPeerPresenceV1, timeTravelTransitionV1, TimeTravelWindowIndicator, useTimeTravelRevealV1, type TimeTravelFocusTargetV1 } from "../../🟦️.tsx";
import { TREE_ROW_TONE_CLASSES, UiPresenceOverlayContext } from "../../../../🗣️Interpreter/🟦️.tsx";
import { TIME_TRAVEL_CODE_LABELS, TIME_TRAVEL_LABELS } from "../../../../../../../../../../🔨️modules/⏪️time-travel/🟦️.ts";
import { HISTORY_NOTICE_LABELS } from "../../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { documentLoadCancelledV1, historyFaultNoticeV1, historyLaneNoticeV1, historyOutputNoticeV1, IMPORT_ABORT_ACTION_ID, importOpenedFilesV1 } from "../../../🟦️.tsx";

const { computeAccessibleName, computeAccessibleDescription, getRole }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
const { roles: ariaRoles, aria: ariaProperties } = createRequire(import.meta.url)("aria-query") as { readonly roles: ReadonlyMap<string, { readonly props: Readonly<Record<string, unknown>> }>; readonly aria: ReadonlyMap<string, unknown> };
const here = dirname(fileURLToPath(import.meta.url));
const shellHelpers = join(here, "..", "..", "..");
const framework = join(here, "..", "..", "..", "..", "..", "..", "..", "..", "..", "..");
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const corpus = readJson(join(shellHelpers, "🧫️fixtures", "🧫️time-travel-band", "🔣️.json"));
const rowActionCorpus = readJson(join(framework, "🔨️modules", "🖱️ui", "🧫️fixtures", "♿️disabled-row-action", "🔣️.json")) as readonly { readonly id: string; readonly label: string; readonly focusable: boolean; readonly actionable: boolean }[];
type Case = { readonly name: string; readonly session: HistoryTimeTravel; readonly text: Readonly<Record<"en" | "de", Record<string, string | null>>>; readonly controls: readonly { readonly control: string; readonly action: string; readonly disabledBy: string | null }[]; readonly indicator: Readonly<Record<"en" | "de", string>> };
const cases = corpus.cases as readonly Case[];
/** 👥️ The peers' history-edit corpus (`🧫️time-travel-peers`) both shells assert: the rows a replica holds, the peers on
 * its roster, and per locale every editing peer's chip and every history-body node's note. */
const peersCorpus = readJson(join(shellHelpers, "🧫️fixtures", "🧫️time-travel-peers", "🔣️.json"));
type PeersCorpusRow = { readonly seq: number; readonly editId: string; readonly mutations: readonly { readonly mutationId: string; readonly label: { readonly en: string; readonly de: string } }[] };
type PeersCorpusCase = {
  readonly name: string;
  readonly peers: readonly { readonly actor: string; readonly label: string; readonly historyEdit?: { readonly mutationId: string; readonly stage: "editing" | "replaying" | "reviewing" | "choosing" | "finalizing"; readonly drafts: number } }[];
  readonly expect: Readonly<Record<"en" | "de", { readonly chips: readonly { readonly actor: string; readonly text: string; readonly badge: string }[]; readonly notes: readonly { readonly key: string; readonly text: string }[] }>>;
};
const axes = { terminology: corpus.axes.terminology as string };
const LOCALES = ["en", "de"] as const;

function mountBand(session: HistoryTimeTravel, locale: string, overrides: Readonly<Record<string, string>> = {}) {
  const dispatched: ActionDescriptor[] = [];
  const bindings = composeControlKeybindings(new Map(), overrides);
  const view = render(createElement(UiKeybindingsProvider, { bindings, children: createElement(TimeTravelBand, { session, terminology: axes.terminology, locale, controllerId: corpus.controllerId, onAction: (action: ActionDescriptor) => dispatched.push(action) }) }));
  return { ...view, dispatched };
}

const band = (container: HTMLElement): HTMLElement => container.querySelector("[data-semio-time-travel]") as HTMLElement;
const controlButton = (container: HTMLElement, control: string): HTMLButtonElement | null => container.querySelector(`[data-semio-time-travel-control="${control}"]`);

describe("⏪️ time-travel band corpus", () => {
  afterEach(() => cleanup());
  afterAll(() => syncShellLabelLocale("en"));

  it("validates against its schema and every session against the kernel's HistoryTimeTravel wire schema", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    ajv.addSchema(readJson(join(framework, "🔨️modules", "🎠️kernel", "🧬️schema", "🔣️history-patch", "🔣️.json")));
    const validate = ajv.compile(readJson(join(shellHelpers, "🧬️schema", "🔣️time-travel-band", "🔣️.json")));
    expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
    expect(new Set(cases.map((entry) => entry.session.stage))).toEqual(new Set(["editing", "replaying", "reviewing", "choosing", "finalizing"]));
  });

  it("derives every stage's lines, controls, verbs and indicator in English and German", () => {
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const entry of cases) {
        expect(timeTravelBandTextV1(entry.session, { ...axes, locale }), `${entry.name} (${locale})`).toEqual(entry.text[locale]);
        expect(timeTravelIndicatorTextV1(entry.session, { ...axes, locale }), `${entry.name} (${locale})`).toBe(entry.indicator[locale]);
        const controls = timeTravelBandControlsV1(entry.session);
        expect(controls.map(({ control, disabledBy }) => ({ control, action: timeTravelControlActionV1(corpus.controllerId, entry.session, control).action, disabledBy })), entry.name).toEqual(entry.controls);
        for (const { control } of controls) expect(timeTravelControlActionV1(corpus.controllerId, entry.session, control)).toEqual({ controllerId: corpus.controllerId, action: entry.controls.find((row) => row.control === control)!.action, args: { generation: entry.session.generation } });
      }
    }
  });

  it("renders a polite status band per stage whose visible lines, progress and controls are the corpus's, in both languages", () => {
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const entry of cases) {
        const view = mountBand(entry.session, locale);
        const status = band(view.container);
        expect([status.getAttribute("role"), status.getAttribute("aria-live"), status.dataset.semioTimeTravel], entry.name).toEqual(["status", "polite", entry.session.stage]);
        for (const line of Object.values(entry.text[locale])) if (line !== null) expect(status.textContent, `${entry.name} (${locale})`).toContain(line);
        const progress = status.querySelector("progress");
        expect(progress === null, entry.name).toBe(entry.text[locale].progress === null);
        if (progress !== null) expect([progress.value, progress.max, progress.getAttribute("aria-label")]).toEqual([entry.session.done, entry.session.total, entry.text[locale].progress]);
        const buttons = [...status.querySelectorAll<HTMLButtonElement>("button[data-semio-time-travel-control]")];
        expect(buttons.map((button) => button.dataset.semioTimeTravelControl), entry.name).toEqual(entry.controls.map((row) => row.control));
        expect(buttons.map((button) => [button.textContent, button.disabled, button.title || null]), `${entry.name} (${locale})`).toEqual(entry.controls.map((row) => [String(shellLabel(`ui.timeTravel.${row.control}` as Parameters<typeof shellLabel>[0])), row.disabledBy !== null, row.disabledBy === null ? null : String(shellLabel(row.disabledBy as Parameters<typeof shellLabel>[0]))]));
        const review = status.querySelector<HTMLElement>("[data-semio-time-travel-review]");
        const finalize = controlButton(view.container, "finalize");
        if (finalize !== null) expect(finalize.getAttribute("aria-describedby"), entry.name).toBe(finalize.disabled && review !== null ? review.id : null);
        view.unmount();
      }
    }
  });

  it("dispatches each enabled control's reserved verb with the session generation, and nothing for a disabled one", () => {
    syncShellLabelLocale("en");
    for (const entry of cases) {
      const view = mountBand(entry.session, "en");
      for (const row of entry.controls) controlButton(view.container, row.control)!.click();
      expect(view.dispatched, entry.name).toEqual(entry.controls.filter((row) => row.disabledBy === null).map((row) => ({ controllerId: corpus.controllerId, action: row.action, args: { generation: entry.session.generation } })));
      view.unmount();
    }
  });

  it("tells every history-edit refusal of the corpus in both languages, coded by its own code, with its default severity", () => {
    const refusals = corpus.refusals as readonly { readonly code: string; readonly text: Readonly<Record<"en" | "de", string>>; readonly severity: string }[];
    expect(new Set(refusals.map((row) => row.code))).toEqual(new Set(Object.keys(HISTORY_REFUSAL_LABEL_KEYS)));
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const row of refusals) expect(historyRefusalNoticeV1(historyRefusalCodeV1(row.code)!), `${row.code} (${locale})`).toEqual({ text: row.text[locale], kind: row.severity, code: row.code });
    }
    expect([historyRefusalOfOutputV1({ rejected: "timeTravel.name-invalid" }), historyRefusalOfOutputV1({ rejected: "timeTravel.busy" }), historyRefusalOfOutputV1({ rejected: "timeTravel.unheard-of" }), historyRefusalOfOutputV1({ timeTravel: "reviewing" }), historyRefusalOfOutputV1(null)]).toEqual(["timeTravel.name-invalid", "timeTravel.busy", null, null, null]);
  });

  it("names the window indicator by what the window shows, in words and not by colour", () => {
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const entry of cases) {
        const view = render(createElement(TimeTravelWindowIndicator, { stage: entry.session.stage, description: timeTravelIndicatorTextV1(entry.session, { ...axes, locale }) }));
        const indicator = view.container.querySelector<HTMLElement>("[data-semio-time-travel-indicator]")!;
        expect([indicator.getAttribute("role"), computeAccessibleName(indicator), indicator.dataset.semioTimeTravelIndicator, indicator.textContent?.trim()], `${entry.name} (${locale})`).toEqual(["note", entry.indicator[locale], entry.session.stage, String(shellLabel("ui.timeTravel.indicator"))]);
        view.unmount();
      }
    }
  });
});

describe("⌨️ time-travel chords", () => {
  afterEach(() => cleanup());
  const editing = cases.find((entry) => entry.session.stage === "editing" && entry.session.fault === undefined)!;
  const replaying = cases.find((entry) => entry.session.stage === "replaying")!;
  const finalizing = cases.find((entry) => entry.session.stage === "finalizing")!;

  it("ships the corpus's default chords, none of them Escape, and publishes them on aria-keyshortcuts", () => {
    for (const control of ["accept", "discard", "exit"] as const) {
      expect(SHELL_KEYBINDINGS[TIME_TRAVEL_CHORD_IDS[control]]).toBe(corpus.chords[control].keys);
      expect(corpus.chords[control].controlId).toBe(TIME_TRAVEL_CHORD_IDS[control]);
      expect(corpus.chords[control].keys.split(/[+,]/u)).not.toContain("escape");
    }
    const view = mountBand(editing.session, "en");
    expect(["accept", "discard", "exit"].map((control) => controlButton(view.container, control)!.getAttribute("aria-keyshortcuts"))).toEqual([corpus.chords.accept.aria, corpus.chords.discard.aria, corpus.chords.exit.aria]);
  });

  it("accepts, discards and exits by chord while editing, and Escape dispatches nothing", async () => {
    const user = userEvent.setup();
    const view = mountBand(editing.session, "en");
    await act(async () => {
      await user.keyboard("{Escape}");
      await user.keyboard("{Alt>}{Enter}{/Alt}");
      await user.keyboard("{Alt>}{Backspace}{/Alt}");
      await user.keyboard("{Alt>}{Shift>}{Backspace}{/Shift}{/Alt}");
    });
    const args = { generation: editing.session.generation };
    expect(view.dispatched).toEqual([
      { controllerId: corpus.controllerId, action: "historyEditAccept", args },
      { controllerId: corpus.controllerId, action: "historyEditDiscard", args },
      { controllerId: corpus.controllerId, action: "historyEditExit", args },
    ]);
  });

  it("fires only the chords the stage offers, and never from a form field", async () => {
    const user = userEvent.setup();
    const replayView = mountBand(replaying.session, "en");
    await act(async () => {
      await user.keyboard("{Alt>}{Enter}{/Alt}");
      await user.keyboard("{Alt>}{Shift>}{Backspace}{/Shift}{/Alt}");
    });
    expect(replayView.dispatched.map((action) => action.action)).toEqual(["historyEditExit"]);
    replayView.unmount();
    const finalizingView = mountBand(finalizing.session, "en");
    await act(async () => {
      await user.keyboard("{Alt>}{Shift>}{Backspace}{/Shift}{/Alt}");
    });
    expect(finalizingView.dispatched).toEqual([]);
    finalizingView.unmount();
    const typing = mountBand(editing.session, "en");
    const field = document.createElement("input");
    document.body.append(field);
    field.focus();
    await act(async () => {
      await user.keyboard("{Alt>}{Enter}{/Alt}");
    });
    field.remove();
    expect(typing.dispatched).toEqual([]);
  });

  it("follows a user's remapped chord and republishes it", async () => {
    const user = userEvent.setup();
    const view = mountBand(editing.session, "en", { [TIME_TRAVEL_CHORD_IDS.accept]: "alt+a" });
    expect(controlButton(view.container, "accept")!.getAttribute("aria-keyshortcuts")).toBe("Alt+A");
    await act(async () => {
      await user.keyboard("{Alt>}{Enter}{/Alt}");
      await user.keyboard("{Alt>}a{/Alt}");
    });
    expect(view.dispatched.map((action) => action.action)).toEqual(["historyEditAccept"]);
  });
});

describe("🎞️ unsolicited replay progress folds into the band while the replay runs", () => {
  it("folds each progress frame's history patch into the program's projection and refreshes only a scope a frame carried", () => {
    const replaying = cases.find((entry) => entry.session.stage === "replaying")!.session;
    const frames: readonly { readonly uiScope?: { readonly kind: "full" }; readonly historyPatch?: HistoryPatch }[] = [
      { historyPatch: { cursor: 10, timeTravel: { ...replaying, done: 1, total: 8 } } },
      { uiScope: { kind: "full" }, historyPatch: { cursor: 11, timeTravel: { ...replaying, done: 5, total: 8 } } },
      {},
      { historyPatch: { cursor: 12, timeTravel: { ...replaying, stage: "reviewing", done: undefined, total: undefined, review: "ready", rerunnable: false } } },
      { historyPatch: { cursor: 13 } },
    ];
    let projection: ShellHistoryProjectionV1 = { ...EMPTY_SHELL_HISTORY_PROJECTION_V1, cursor: 9 };
    const seen: (readonly [string, string | null, number | null])[] = [];
    for (const frame of frames) {
      const { scope, historyPatch } = operationProgressPartsV1(frame);
      if (historyPatch !== undefined && historyPatchShouldApplyV1(projection.cursor, historyPatch)) projection = shellHistoryProjectionAfterPatchV1(projection, historyPatch, false);
      seen.push([scope.kind, projection.timeTravel?.stage ?? null, projection.timeTravel?.done ?? null]);
    }
    expect(seen).toEqual([
      ["none", "replaying", 1],
      ["full", "replaying", 5],
      ["none", "replaying", 5],
      ["none", "reviewing", null],
      ["none", null, null],
    ]);
  });
});

describe("🗣️ the time-travel bundle texts are the ⏪️time-travel module's own", () => {
  afterAll(() => syncShellLabelLocale("en"));

  it("matches every stage and refusal text in both languages", () => {
    const pairs: readonly (readonly [Parameters<typeof shellLabel>[0], keyof typeof TIME_TRAVEL_LABELS])[] = [
      ["ui.timeTravel.stage.editing", "stageEditing"],
      ["ui.timeTravel.stage.replaying", "stageReplaying"],
      ["ui.timeTravel.stage.reviewing", "stageReviewing"],
      ["ui.timeTravel.stage.choosing", "stageChoosing"],
      ["ui.timeTravel.stage.finalizing", "stageFinalizing"],
      ["ui.timeTravel.refusal.frozen", "frozen"],
      ["ui.timeTravel.refusal.illegal", "refusalIllegal"],
      ["ui.timeTravel.refusal.stale", "refusalStale"],
      ["ui.timeTravel.refusal.blocked", "refusalBlocked"],
      ["ui.timeTravel.refusal.empty", "refusalEmpty"],
      ["ui.timeTravel.refusal.cancelled", "replayCancelled"],
      ["ui.timeTravel.review.noChanges", "noChanges"],
      ["ui.timeTravel.review.needsReplay", "needsReplay"],
      ["ui.timeTravel.review.blocked", "reportBlocking"],
      ["ui.timeTravel.review.ready", "readyToFinalize"],
      ["ui.timeTravel.rerun", "actionRerun"],
    ];
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const [key, label] of pairs) expect(String(shellLabel(key)), `${key} (${locale})`).toBe(TIME_TRAVEL_LABELS[label][locale]);
    }
  });

  it("names every timeTravel.* code the vocabulary lists, byte for byte in both languages", () => {
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const [code, label] of TIME_TRAVEL_CODE_LABELS) {
        const known = historyRefusalCodeV1(code);
        expect(known, code).not.toBeNull();
        expect(historyRefusalNoticeV1(known!).text, `${code} (${locale})`).toBe(TIME_TRAVEL_LABELS[label][locale]);
      }
    }
    expect(Object.keys(HISTORY_REFUSAL_LABEL_KEYS).filter((code) => code.startsWith("timeTravel.")).sort()).toEqual(TIME_TRAVEL_CODE_LABELS.map(([code]) => code).sort());
  });
});

/** 🌲️ Rust-shaped history-body nodes (`🔌️plugin/🦀️.rs` `ui_history_panel`, `⏪️time-travel` band and editor sections), built
 * the way the guest's `BuiltNode` arrives: every input row holds one control, every button row one button. */
const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
const LEAF = { kind: "leaf", width: "hug", height: "hug" };
type NodeExtra = { readonly tone?: string; readonly label?: string; readonly disabled?: boolean };
const node = (key: string, component: Record<string, unknown>, children: readonly BuiltNode[] = [], bindings: readonly unknown[] = [], extra: NodeExtra = {}): BuiltNode => ({ key, component, layout: LEAF, style: { ...STYLE, tone: extra.tone ?? "neutral" }, activity: "idle", disabled: extra.disabled ?? false, accessibility: { ...ACCESSIBILITY, label: extra.label ?? null }, bindings: [...bindings], menu: null, children: [...children] }) as unknown as BuiltNode;
const bind = (scope: string, name: string, args: Record<string, unknown> | null = null, trigger = "activate") => ({ trigger, action: { scope, name, version: 1 }, args });
const treeItem = (key: string, label: string, children: readonly BuiltNode[], row: { readonly description?: string; readonly icon?: string; readonly tone?: string; readonly open?: boolean } = {}) => node(key, { type: "treeItem", label, description: row.description ?? null, icon: row.icon ?? null, defaultOpen: row.open ?? null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null }, children, [], { tone: row.tone });
const section = (key: string, label: string, children: readonly BuiltNode[]) => node(key, { type: "treeSection", label, defaultOpen: true, headerToolbar: null, window: null }, children);
const button = (key: string, label: string, binding: unknown, extra: NodeExtra = {}) => node(key, { type: "button", label, icon: "" }, [], [binding], extra);
const controller = "toy.controller";
const overlay = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} };
const historyTab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: { native: { en: "History", de: "Verlauf" }, reuse: { en: "History", de: "Verlauf" } }, group: "settings" as const, bodyKey: "framework.body.history", children: [] };

/** 🧱️ Renders `body` as the History leaf exactly as the shell mounts it, dispatching into `onAction`. */
function mountHistoryBody(body: BuiltNode, onAction: (action: ActionDescriptor) => void) {
  const store = new UiDocumentStore("panel:framework.panel.history");
  store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", body));
  const leaf = panelTabDefinitionToNode(historyTab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, onAction, 1, overlay);
  if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
  const source = leaf.trees[0]!.tree;
  const config = "resolveTree" in source ? source.resolveTree() : source;
  return render(createElement(Fragment, null, config.emptyState));
}

describe("🕰️ the framework history body renders through the interpreter", () => {
  afterEach(() => cleanup());
  const body = node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.actions", "Actions", [
      treeItem("framework.history.undo", "Undo", [button("framework.history.undo.run", "Undo", bind(controller, "undo"))]),
      treeItem("framework.history.checkin", "Check In", [button("s-checkin", "Check In", bind(FRAMEWORK_CHECKIN_CONTROLLER_ID, "submit"))]),
    ]),
    section("framework.history.commands", "Commands", [
      treeItem("framework.history.entry.4", "Drag selection", [button("framework.history.entry.4.edit", "Edit", bind(controller, "historyEditBegin", { mutationId: "m-2" }))]),
    ]),
  ]);

  it("mounts the guest's body on the History leaf and dispatches its authored verbs, a row's lone button becoming the row's activation only where the row says what it does", () => {
    const store = new UiDocumentStore("panel:framework.panel.history");
    store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", body));
    const dispatched: ActionDescriptor[] = [];
    const tab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: { native: { en: "History", de: "Verlauf" }, reuse: { en: "History", de: "Verlauf" } }, group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const leaf = panelTabDefinitionToNode(tab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, (action) => dispatched.push(action), 1, overlay);
    expect([leaf.kind, leaf.id]).toEqual(["leaf", "framework.panel.history"]);
    if (leaf.kind !== "leaf") return;
    const source = leaf.trees[0]!.tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const view = render(createElement(Fragment, null, config.emptyState));
    for (const name of ["Undo", "Check In", "Edit"]) (view.getByRole("button", { name }) as HTMLButtonElement).click();
    expect(dispatched.map(({ controllerId, action, args }) => ({ controllerId, action, args: args ?? null }))).toEqual([
      { controllerId: controller, action: "undo", args: null },
      { controllerId: FRAMEWORK_CHECKIN_CONTROLLER_ID, action: "submit", args: null },
      { controllerId: controller, action: "historyEditBegin", args: { mutationId: "m-2" } },
    ]);
    expect([checkinSubmitMessageV1(undefined), checkinSubmitMessageV1({ message: "  release  " }), checkinSubmitMessageV1({ message: " " })]).toEqual(["check-in", "release", "check-in"]);
  });

  it("routes an actor-bound document's Undo and check-in through the host, and every other gesture to its actor", () => {
    const store = new UiDocumentStore("panel:framework.panel.history");
    store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", body));
    const hosted: ActionDescriptor[] = [];
    const actor: string[] = [];
    const tab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: { native: { en: "History", de: "Verlauf" }, reuse: { en: "History", de: "Verlauf" } }, group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const actorPanels = { stores: new Map([["framework.panel.history", store]]), onIntent: (tabId: string, intent: UiIntent) => void actor.push(`${tabId}:${intent.action.name}`), onAction: (action: ActionDescriptor) => void hosted.push(action) };
    const leaf = panelTabDefinitionToNode(tab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", {}, () => undefined, 1, overlay, "native", "en", null, undefined, actorPanels);
    if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
    const source = leaf.trees[0]!.tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const view = render(createElement(Fragment, null, config.emptyState));
    for (const name of ["Undo", "Check In", "Edit"]) (view.getByRole("button", { name }) as HTMLButtonElement).click();
    expect(hosted.map(({ controllerId, action }) => `${controllerId}:${action}`)).toEqual([`${controller}:undo`, `${FRAMEWORK_CHECKIN_CONTROLLER_ID}:submit`]);
    expect(actor).toEqual(["framework.panel.history:historyEditBegin"]);
    expect([panelActionRoutesThroughHostV1({ controllerId: controller, action: "redo" }), panelActionRoutesThroughHostV1({ controllerId: controller, action: "historyEditRerun" })]).toEqual([false, false]);
  });

  it("marks the rows and chips of peers editing in time travel exactly as the shared peers corpus says, in both languages", () => {
    const localized = (text: { readonly en: string; readonly de: string }) => ({ native: text, reuse: text });
    const rows = peersCorpus.rows as readonly PeersCorpusRow[];
    const entries = rows.map((row) => ({ seq: row.seq, editId: row.editId, actionId: "apply", label: localized({ en: "Apply", de: "Anwenden" }), kind: "mutation", timestamp: "t", mutations: row.mutations.map((mutation, index) => ({ mutationId: mutation.mutationId, position: index, opIndex: index, label: localized(mutation.label) })) })) as unknown as Parameters<typeof timeTravelPeerPresenceV1>[1];
    const peerBody = node("framework.history", { type: "tree", interactionDomain: null }, [
      section("framework.history.commands", "Commands", rows.map((row) => treeItem(`framework.history.entry.${row.seq}`, row.mutations[0]!.label.en, [node(`framework.history.mutation.${row.mutations[0]!.mutationId}`, { type: "treeItem", label: row.mutations[0]!.label.en, description: null, icon: "circle", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [{ icon: "edit", label: "Edit", verb: "historyEditBegin", placement: "row", disabled: false }], target: { scope: controller, version: 1, args: { mutationId: row.mutations[0]!.mutationId }, activation: "historyEditBegin" } })]))),
    ]);
    for (const peerCase of peersCorpus.cases as readonly PeersCorpusCase[]) {
      for (const locale of LOCALES) {
        syncShellLabelLocale(locale);
        const expected = peerCase.expect[locale];
        const presence = timeTravelPeerPresenceV1(peerCase.peers, entries, { terminology: "native", locale });
        expect(presence.peers.some((peer) => "historyEdit" in peer), `${peerCase.name} (${locale}): the wire field never reaches the roster`).toBe(false);
        expect(presence.peers.flatMap((peer) => (peer.activity === undefined ? [] : [{ actor: peer.actor, text: peer.activity.text, badge: peer.activity.badge }])), `${peerCase.name} (${locale}): chips`).toEqual(expected.chips);
        expect([...presence.overlay.byKey].map(([key, entry]) => ({ key, text: (entry.notes ?? []).join(" · ") })).sort((a, b) => (a.key < b.key ? -1 : a.key > b.key ? 1 : 0)), `${peerCase.name} (${locale}): notes`).toEqual(expected.notes);
        const store = new UiDocumentStore("panel:framework.panel.history");
        store.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", peerBody));
        const tab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: localized({ en: "History", de: "Verlauf" }), group: "settings" as const, bodyKey: "framework.body.history", children: [] };
        const leaf = panelTabDefinitionToNode(tab as Parameters<typeof panelTabDefinitionToNode>[0], "settings", { "framework.panel.history": store }, () => undefined, 1, overlay);
        if (leaf.kind !== "leaf") throw new Error("history tab is a leaf");
        const source = leaf.trees[0]!.tree;
        const config = "resolveTree" in source ? source.resolveTree() : source;
        const view = render(createElement(UiPresenceOverlayContext.Provider, { value: presence.overlay, children: createElement(Fragment, null, config.emptyState) }));
        for (const note of expected.notes.filter((entry) => entry.key.startsWith("framework.history.entry."))) expect(view.container.textContent, `${peerCase.name} (${locale}): ${note.key}`).toContain(note.text);
        const roster = render(createElement(PresenceBar, { peers: presence.peers }));
        for (const peer of peerCase.peers) {
          const chip = roster.container.querySelector<HTMLElement>(`[data-row-id="peer:${peer.actor}"]`)!;
          const activity = expected.chips.find((entry) => entry.actor === peer.actor);
          expect([chip.hasAttribute("data-presence-activity"), activity === undefined || computeAccessibleName(chip).includes(activity.text), roster.container.querySelector(`[data-row-id="peer-activity-badge:${peer.actor}"]`)?.textContent ?? null], `${peerCase.name} (${locale}): ${peer.actor}`).toEqual([activity !== undefined, true, activity?.badge ?? null]);
        }
        view.unmount();
        roster.unmount();
      }
    }
    syncShellLabelLocale("en");
  });

  it("publishes this human's own history edit, tool run and interaction in the presence heartbeat, and nothing it does not carry", () => {
    const historyEdit = { mutationId: "m-2", stage: "choosing", drafts: 2 } as const;
    const toolRun = { toolId: "fill", state: "paused", stage: 0, completed: 3 } as const;
    const interaction = { app_id: "app", domains: [] };
    expect(presenceEphemeralPeerFieldsV1({ historyEdit: encodePresenceHistoryEdit(historyEdit), toolRun: encodePresenceToolRun(toolRun), interaction: encodePresenceInteraction(interaction) })).toEqual({ historyEdit, toolRun, interaction });
    expect([presenceEphemeralPeerFieldsV1({ historyEdit: [], toolRun: [], interaction: [] }), presenceEphemeralPeerFieldsV1(undefined)]).toEqual([{}, {}]);
  });

  it("keeps no host-built history tab or frozen history labels in the shell", () => {
    const shellHost = readFileSync(join(shellHelpers, "..", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const helpers = readFileSync(join(shellHelpers, "🟦️.tsx"), "utf8");
    for (const gone of ["frameworkUtilitiesHistoryTab", "historyPanelText", "checkinDialog", "s-checkin-message"]) expect(shellHost.includes(gone), gone).toBe(false);
    for (const gone of ["HISTORY_PANEL_LABELS", "SHELL_OWNED_PANEL_TAB_IDS", "shellRendersPanelTabItself"]) expect(helpers.includes(gone), gone).toBe(false);
  });
});

//#region ⏪️DraftEditor
/** ♿️ The axe-level structural findings under `roots`: an unknown `aria-*` attribute, one the element's role does not support
 * (the WAI-ARIA role model of `aria-query`), a `labelledby`/`describedby` naming no element, a control without an accessible
 * name (`dom-accessibility-api`) and a repeated id. */
function ariaFindings(roots: readonly ParentNode[]): readonly string[] {
  const findings: string[] = [];
  const ids = new Map<string, number>();
  for (const root of roots) {
    for (const element of root.querySelectorAll<HTMLElement>("*")) {
      if (element.id) ids.set(element.id, (ids.get(element.id) ?? 0) + 1);
      const role = getRole(element);
      for (const { name, value } of [...element.attributes]) {
        if (!name.startsWith("aria-")) continue;
        if (!ariaProperties.has(name)) findings.push(`${element.tagName}#${element.id}: unknown ${name}`);
        else if (role !== null && ariaRoles.get(role) !== undefined && !(name in ariaRoles.get(role)!.props) && name !== "aria-hidden") findings.push(`${role}#${element.id}: ${name} not supported`);
        if ((name === "aria-describedby" || name === "aria-labelledby") && value.split(/\s+/u).some((id) => id !== "" && document.getElementById(id) === null)) findings.push(`${role}#${element.id}: ${name} → missing ${value}`);
      }
      const interactive = element.matches('button, input, select, textarea, [role="slider"], [role="combobox"], [role="spinbutton"]') && element.closest('[aria-hidden="true"]') === null;
      if (interactive && computeAccessibleName(element).trim() === "") findings.push(`${role}#${element.id}: no accessible name`);
    }
  }
  for (const [id, count] of ids) if (count > 1) findings.push(`duplicate id ${id}`);
  return findings;
}

/** 🔢️ The session generation every draft verb of the editor body is stamped with. */
const GENERATION = 7;
const draftArgs = (path: string, value?: unknown) => ({ generation: GENERATION, path, ...(value === undefined ? {} : { value }) });
const container = (key: string, role: string, label: string | null, children: readonly BuiltNode[]) => node(key, { type: "container", role, label, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, children);
/** ✏️ The Rust-shaped body while a draft is open (`time_travel_band_section`, `time_travel_editor_sections`,
 * `ui_history_panel`): the band and editor sections, one row per input — a stepper, a slider with snaps, a select, a vector
 * and a reference list with "Use selection" — the alternatives, and a history row whose mutation rows carry their outcome. */
const draftBody = (inputs: boolean): BuiltNode =>
  node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.timeTravel", "History editing", [
      treeItem("framework.history.timeTravel.status", "Editing a mutation: Drag selection · Error", [], { icon: "clock", tone: "danger" }),
      treeItem("framework.history.timeTravel.exit.row", "Exit time travel", [button("framework.history.timeTravel.exit", "Exit time travel", bind(controller, "historyEditExit"))]),
    ]),
    section("framework.history.editor", "Drag selection", [
      treeItem("framework.history.editor.target", "Draft: Drag selection · Error: Target missing", [], { icon: "alert-circle", tone: "danger" }),
      ...(["Accept", "Discard", "Withdraw"] as const).map((verb) => treeItem(`framework.history.editor.${verb.toLowerCase()}.row`, verb, [button(`framework.history.editor.${verb.toLowerCase()}`, verb, bind(controller, `historyEdit${verb}`, { generation: GENERATION }))])),
    ]),
    ...(inputs
      ? [
          section("framework.history.editor.inputs", "Inputs", [
            treeItem("framework.history.editor.input.dx.row", "dx", [node("framework.history.editor.input.dx", { type: "numberStepper", value: 80, step: 1, uniform: true, min: -1000, max: 1000, precision: null }, [], [bind(controller, "historyEditInput", draftArgs("/dx"), "change")], { label: "dx" })]),
            treeItem("framework.history.editor.input.angle.row", "Angle", [node("framework.history.editor.input.angle", { type: "slider", value: 0, min: -180, max: 180, step: 1, unit: "°", snaps: [-90, 0, 90] }, [], [bind(controller, "historyEditInput", draftArgs("/angle"), "change")], { label: "Angle" })]),
            treeItem("framework.history.editor.input.mode.row", "Mode", [node("framework.history.editor.input.mode", { type: "select", value: "pivot", items: [{ value: "pivot", label: "Pivot" }, { value: "centroid", label: "Centroid" }], placeholder: null }, [], [bind(controller, "historyEditInput", draftArgs("/mode"), "change")], { label: "Mode" })]),
            treeItem("framework.history.editor.input.pivot.row", "Pivot", [
              container("framework.history.editor.input.pivot", "group", "Pivot", (["x", "y"] as const).map((axis, index) =>
                node(`framework.history.editor.input.pivot.${index}.axis`, { type: "container", role: "field", label: axis, description: "mm", required: null, error: null, defaultOpen: null, dropOverlay: null }, [
                  node(`framework.history.editor.input.pivot.${index}`, { type: "input", kind: "number", value: String(index * 10), placeholder: null, commit: "blur", min: -500, max: 500, step: 0.5, accept: null, precision: 1, snaps: [0] }, [], [bind(controller, "historyEditInput", draftArgs(`/pivot/${index}`), "commit")], { label: axis }),
                ]),
              )),
            ]),
            treeItem("framework.history.editor.input.targets.row", "Targets", [
              container("framework.history.editor.input.targets", "group", "Targets", [
                container("framework.history.editor.input.targets.chips", "toolbar", "Targets", [
                  button("framework.history.editor.input.targets.chip.0", "node-1", bind(controller, "historyEditInput", draftArgs("/targets", ["node-2"])), { label: "Remove node-1" }),
                  button("framework.history.editor.input.targets.chip.1", "node-2", bind(controller, "historyEditInput", draftArgs("/targets", ["node-1"])), { label: "Remove node-2" }),
                ]),
                container("framework.history.editor.input.targets.actions", "plain", null, [button("framework.history.editor.input.targets.useSelection", "Use selection", bind(controller, "historyEditUseSelection", draftArgs("/targets")), { label: "Use selection" })]),
              ]),
            ]),
          ]),
        ]
      : []),
    section("framework.history.alternatives", "Alternatives", [
      treeItem("framework.history.alternative.trunk", "Main line", [], { description: "Current", icon: "check" }),
      node("framework.history.alternative.alt-1", { type: "treeItem", label: "Variant", description: "Branched by Ada at 2026-10-01 09:12 UTC · Edited history", icon: "git-branch", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [{ icon: "git-branch", label: "Switch", verb: "switchAlternative", placement: "row", disabled: false }], target: { scope: controller, version: 1, args: { alternativeId: "alt-1" }, activation: "switchAlternative" } }),
    ]),
    section("framework.history.commands", "Commands", [
      treeItem(
        "framework.history.entry.4",
        "Drag 2 items by (80, 40)",
        [
          treeItem("framework.history.mutation.m-2", "Drag selection", [], { description: "Edited", icon: "edit" }),
          treeItem("framework.history.mutation.m-3", "Drag selection", [], { description: "Warning: Partially applied", icon: "triangle-alert", tone: "warning" }),
          treeItem("framework.history.mutation.m-4", "Drag selection", [], { description: "Error: Target missing", icon: "alert-circle", tone: "danger" }),
        ],
        { open: true },
      ),
    ]),
  ]);
const editingSession = cases.find((entry) => entry.session.stage === "editing" && entry.session.fault === undefined)!.session;
const byIdSuffix = (root: ParentNode, key: string): HTMLElement | null => [...root.querySelectorAll<HTMLElement>("[id]")].find((element) => element.id.endsWith(`/${key}`)) ?? null;
const nextFrames = (count: number) => act(async () => {
  for (let frame = 0; frame < count; frame += 1) await new Promise((resolve) => requestAnimationFrame(() => resolve(null)));
});
type Transition = { readonly name: string; readonly from: HistoryTimeTravel | null; readonly to: HistoryTimeTravel | null; readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null };
/** 🗝️ The draft editor's node keys the corpus holds the Rust producer and every shell's focus resolution to. */
type EditorKeys = { readonly panel: string; readonly accept: { readonly control: string; readonly row: string }; readonly inputs: readonly { readonly pointer: string; readonly control: string; readonly row: string }[] };

describe("🧭️ a session change reveals the History panel and moves focus where the session continues", () => {
  afterEach(() => cleanup());

  it("reveals and focuses exactly as the shared corpus's transitions say", () => {
    const transitions = corpus.transitions as readonly Transition[];
    expect(new Set(transitions.map((row) => row.focus))).toEqual(new Set(["editor", "band", "dialog", null]));
    for (const row of transitions) expect(timeTravelTransitionV1(row.from, row.to), row.name).toEqual({ reveal: row.reveal, focus: row.focus });
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

  it("fits a phone: the band wraps within 90 % of the viewport and every control is a touch-size target", () => {
    for (const entry of cases) {
      const view = mountBand(entry.session, "en");
      const status = band(view.container);
      expect([status.classList.contains("max-w-[90vw]"), status.classList.contains("flex-wrap")], entry.name).toEqual([true, true]);
      for (const control of status.querySelectorAll("button")) expect(control.classList.contains("min-h-medium"), `${entry.name}: ${control.textContent}`).toBe(true);
      view.unmount();
    }
  });

  it("uses only ARIA attributes each role supports, references only ids that exist and names every control", () => {
    const view = mountBand(editingSession, "en");
    const body = mountHistoryBody(draftBody(true), () => undefined);
    expect(ariaFindings([view.container, body.container])).toEqual([]);
  });

  it("opens the finalize prompt with focus on its name field, a destructive Overwrite, Back, and no ARIA finding", () => {
    const choices = readJson(join(framework, "🔨️modules", "🛂️manifest", "🧫️fixtures", "🧫️dialog-choices", "🔣️.json"));
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
    const reload = readJson(join(shellHelpers, "..", "..", "..", "..", "🔌️plugin", "🧫️fixtures", "🧫️history-label-reload", "🔣️.json")) as { readonly cases: readonly { readonly id: string; readonly expected: { readonly en: string; readonly de: string } }[] };
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
  const [state, dispatchState] = useReducer(shellReducer, undefined, () => initialShellState({ plugins: [], storage: createMemoryStoragePort() }));
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

  it("agrees with an xstate model of the reveal-and-focus statechart on every corpus transition", () => {
    const stages = ["editing", "replaying", "reviewing", "choosing", "finalizing"] as const;
    const focusOf = { editing: "editor", replaying: "band", reviewing: "band", choosing: "dialog", finalizing: null } as const;
    type Move = { readonly type: "session"; readonly stage: (typeof stages)[number] | null; readonly fresh: boolean; readonly retarget: boolean };
    const toward = (from: string) => [
      { guard: ({ event }: { event: Move }) => event.stage === null, target: "#chrome.inactive", actions: [{ type: "answer", params: { reveal: false, focus: null } }] },
      ...stages.flatMap((stage) => [
        { guard: ({ event }: { event: Move }) => event.stage === stage && event.fresh, target: `#chrome.${stage}`, actions: [{ type: "answer", params: { reveal: true, focus: focusOf[stage] } }] },
        { guard: ({ event }: { event: Move }) => event.stage === stage && (from !== stage || (stage === "editing" && event.retarget)), target: `#chrome.${stage}`, actions: [{ type: "answer", params: { reveal: false, focus: focusOf[stage] } }] },
        { guard: ({ event }: { event: Move }) => event.stage === stage, target: `#chrome.${stage}`, actions: [{ type: "answer", params: { reveal: false, focus: null } }] },
      ]),
    ];
    const machine = createMachine({ id: "chrome", initial: "inactive", states: Object.fromEntries(["inactive", ...stages].map((state) => [state, { on: { session: toward(state) } }])) } as never);
    for (const row of corpus.transitions as readonly Transition[]) {
      const snapshot = machine.resolveState({ value: row.from?.stage ?? "inactive" } as never) as AnyMachineSnapshot;
      const [, actions] = transition(machine, snapshot, { type: "session", stage: row.to?.stage ?? null, fresh: row.to !== null && (row.from === null || row.from.sessionId !== row.to.sessionId), retarget: row.from?.target !== row.to?.target } as never);
      const answer = (actions as unknown as readonly { readonly type: string; readonly params: { readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null } }[]).find((action) => action.type === "answer")!.params;
      expect(answer, `${row.name}: the statechart`).toEqual({ reveal: row.reveal, focus: row.focus });
      expect(timeTravelTransitionV1(row.from, row.to), `${row.name}: the shell`).toEqual(answer);
    }
  });
});
//#endregion 🛰️ShellReveal

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

describe("🪟️ every mutation of a history row is reachable, and a refused Edit says why", () => {
  afterEach(() => cleanup());

  it("windows a history row over all its mutations under the path the runtime files its page request under", () => {
    const view = mountHistoryBody(pagedHistoryBody(null, "Edit"), () => undefined);
    const group = view.container.querySelector<HTMLElement>(`[data-tree-window-path="${`framework.history.commands${TREE_WINDOW_PATH_SEPARATOR}${HISTORY_ROW_KEY_PREFIX}4`}"]`);
    expect([group?.getAttribute("data-tree-window-total"), group?.getAttribute("data-tree-window-length"), group?.querySelector('[data-tree-window-spacer="trailing"]')?.getAttribute("data-tree-window-rows")]).toEqual(["40", "8", "32"]);
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
    const fixture = readJson(join(framework, "🔨️modules", "🎠️kernel", "🧫️fixtures", "🧫️history-notices", "🔣️.json")) as { readonly notices: readonly { readonly code: string; readonly en: string; readonly de: string }[] };
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

//#region 📡️ReprojectionStatus
describe("📡️ a history change replaying before adoption is announced outside the History panel, in the kernel's own words", () => {
  afterEach(() => cleanup());
  type ReprojectionCase = { readonly name: string; readonly reprojection: NonNullable<HistoryPatch["reprojection"]>; readonly title: Readonly<Record<"en" | "de", string>>; readonly text: Readonly<Record<"en" | "de", string>>; readonly paused: boolean; readonly fault: string | null };
  const reprojectionCases = (readJson(join(framework, "🔨️modules", "🎠️kernel", "🧫️fixtures", "🧫️history-reprojection", "🔣️.json")) as { readonly cases: readonly ReprojectionCase[] }).cases;
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
    expect(announced()).toBe("History step: Replaying history: 1 of 240 mutations");
    view.rerender(createElement(HistoryReprojectionStatus, { reprojection: { done: 120, total: 240, kind: "step" }, locale: "en", sessionOpen: false, controllerId: controller, onAction: () => {} }));
    expect([announced(), view.container.querySelector("[data-semio-history-reprojection-text]")?.textContent, view.container.querySelector("progress")?.getAttribute("aria-valuetext")]).toEqual(["History step: Replaying history: 1 of 240 mutations", "Replaying history: 120 of 240 mutations", "Replaying history: 120 of 240 mutations"]);
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

  it("folds the waiting reprojection from every patch — a patch without one clears it", () => {
    const replaying = shellHistoryProjectionAfterPatchV1(EMPTY_SHELL_HISTORY_PROJECTION_V1, { cursor: 4, reprojection: { done: 1, total: 9, kind: "remote" } }, true);
    expect([EMPTY_SHELL_HISTORY_PROJECTION_V1.reprojection, replaying.reprojection, shellHistoryProjectionAfterPatchV1(replaying, { cursor: 5 }, false).reprojection]).toEqual([null, { done: 1, total: 9, kind: "remote" }, null]);
  });
});
//#endregion 📡️ReprojectionStatus
