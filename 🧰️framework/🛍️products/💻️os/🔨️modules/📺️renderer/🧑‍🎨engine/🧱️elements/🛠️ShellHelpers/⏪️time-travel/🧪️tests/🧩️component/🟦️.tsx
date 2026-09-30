/** 🧪️ The React time-travel chrome against the language-neutral band corpus (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`):
 * every stage's lines in English and German, the controls with what disables them, the reserved verbs they dispatch with the
 * session generation, the remappable chords (never Escape, never from a form field) published on `aria-keyshortcuts`, the
 * window indicator's accessible name, the framework history body rendered through the interpreter instead of a
 * host-built tab, and peers' open history edits against the peers corpus the wgpu shell asserts too
 * (`🧫️time-travel-peers`). Third-party oracles: Ajv validates the corpus against its schema and the kernel's own
 * `HistoryTimeTravel` wire schema, `dom-accessibility-api` names the indicator, `@testing-library/user-event` types the
 * chords, and the `⏪️time-travel` module's `TIME_TRAVEL_LABELS` is the independent source of every stage and refusal text. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { createRequire } from "node:module";
import type * as AccessibilityOracle from "dom-accessibility-api" with { "resolution-mode": "require" };
import userEvent from "@testing-library/user-event";
import { afterAll, afterEach, describe, expect, it } from "vitest";
import { createElement, Fragment } from "react";
import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { composeControlKeybindings, PresenceBar, SHELL_KEYBINDINGS, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { encodePresenceHistoryEdit, encodePresenceInteraction, encodePresenceToolRun } from "@semio-tech/framework-replication";
import type { ActionDescriptor, BuiltNode, HistoryPatch, HistoryTimeTravel, UiIntent } from "@semio-tech/framework";
import "../../../../🐚️Shell/🟦️.tsx";
import { builtNodeToSnapshot, UiDocumentStore } from "../../../../📃️UiDocumentStore/🟦️.tsx";
import { checkinSubmitMessageV1, presenceEphemeralPeerFieldsV1, EMPTY_SHELL_HISTORY_PROJECTION_V1, FRAMEWORK_CHECKIN_CONTROLLER_ID, HISTORY_REFUSAL_LABEL_KEYS, historyPatchShouldApplyV1, historyRefusalCodeV1, historyRefusalNoticeV1, historyRefusalOfOutputV1, operationProgressPartsV1, panelActionRoutesThroughHostV1, panelTabDefinitionToNode, shellHistoryProjectionAfterPatchV1, shellLabel, syncShellLabelLocale, type ShellHistoryProjectionV1 } from "../../../🟦️.tsx";
import { TIME_TRAVEL_CHORD_IDS, TimeTravelBand, timeTravelBandControlsV1, timeTravelBandTextV1, timeTravelControlActionV1, timeTravelIndicatorTextV1, timeTravelPeerPresenceV1, TimeTravelWindowIndicator } from "../../🟦️.tsx";
import { UiPresenceOverlayContext } from "../../../../🗣️Interpreter/🟦️.tsx";
import { TIME_TRAVEL_LABELS } from "../../../../../../../../../../🔨️modules/⏪️time-travel/🟦️.ts";

const { computeAccessibleName }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
const here = dirname(fileURLToPath(import.meta.url));
const shellHelpers = join(here, "..", "..", "..");
const framework = join(here, "..", "..", "..", "..", "..", "..", "..", "..", "..", "..");
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const corpus = readJson(join(shellHelpers, "🧫️fixtures", "🧫️time-travel-band", "🔣️.json"));
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
    expect([historyRefusalOfOutputV1({ rejected: "timeTravel.name-invalid" }), historyRefusalOfOutputV1({ rejected: "timeTravel.busy" }), historyRefusalOfOutputV1({ timeTravel: "reviewing" }), historyRefusalOfOutputV1(null)]).toEqual(["timeTravel.name-invalid", null, null, null]);
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
});

describe("🕰️ the framework history body renders through the interpreter", () => {
  afterEach(() => cleanup());
  const STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };
  const LEAF = { kind: "leaf", width: "hug", height: "hug" };
  const node = (key: string, component: Record<string, unknown>, children: readonly BuiltNode[] = [], bindings: readonly unknown[] = []): BuiltNode => ({ key, component, layout: LEAF, style: STYLE, activity: "idle", disabled: false, accessibility: ACCESSIBILITY, bindings: [...bindings], menu: null, children: [...children] }) as unknown as BuiltNode;
  const bind = (scope: string, name: string, args: Record<string, unknown> | null = null) => ({ trigger: "activate", action: { scope, name, version: 1 }, args });
  const treeItem = (key: string, label: string, children: readonly BuiltNode[]) => node(key, { type: "treeItem", label, description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null }, children);
  const section = (key: string, label: string, children: readonly BuiltNode[]) => node(key, { type: "treeSection", label, defaultOpen: true, headerToolbar: null, window: null }, children);
  const button = (key: string, label: string, binding: unknown) => node(key, { type: "button", label, icon: "" }, [], [binding]);
  const controller = "toy.controller";
  const body = node("framework.history", { type: "tree", interactionDomain: null }, [
    section("framework.history.actions", "Actions", [
      treeItem("framework.history.undo", "Undo", [button("framework.history.undo.run", "Undo", bind(controller, "undo"))]),
      treeItem("framework.history.checkin", "Check In", [button("s-checkin", "Check In", bind(FRAMEWORK_CHECKIN_CONTROLLER_ID, "submit"))]),
    ]),
    section("framework.history.commands", "Commands", [
      treeItem("framework.history.entry.4", "Drag selection", [button("framework.history.entry.4.edit", "Edit", bind(controller, "historyEditBegin", { mutationId: "m-2" }))]),
    ]),
  ]);
  const overlay = { windowKindLabels: {}, panelTabLabels: {}, modeLabels: {}, actionLabels: {}, utilityLabels: {}, exampleLabels: {}, actionArgLabels: {}, dialogLabels: {}, introductionLabels: {}, groupLabels: {} };

  it("mounts the guest's body on the History leaf and dispatches its authored verbs, a row's lone button becoming the row's activation", () => {
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
    for (const name of ["Undo", "Check In", "Drag selection"]) (view.getByRole("button", { name }) as HTMLButtonElement).click();
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
    for (const name of ["Undo", "Check In", "Drag selection"]) (view.getByRole("button", { name }) as HTMLButtonElement).click();
    expect(hosted.map(({ controllerId, action }) => `${controllerId}:${action}`)).toEqual([`${controller}:undo`, `${FRAMEWORK_CHECKIN_CONTROLLER_ID}:submit`]);
    expect(actor).toEqual(["framework.panel.history:historyEditBegin"]);
    expect([panelActionRoutesThroughHostV1({ controllerId: controller, action: "redo" }), panelActionRoutesThroughHostV1({ controllerId: controller, action: "historyEditRerun" })]).toEqual([false, false]);
  });

  it("marks the rows and chips of peers editing in time travel exactly as the shared peers corpus says, in both languages", () => {
    const localized = (text: { readonly en: string; readonly de: string }) => ({ native: text, reuse: text });
    const rows = peersCorpus.rows as readonly PeersCorpusRow[];
    const entries = rows.map((row) => ({ seq: row.seq, editId: row.editId, actionId: "apply", label: localized({ en: "Apply", de: "Anwenden" }), kind: "mutation", timestamp: "t", mutations: row.mutations.map((mutation, index) => ({ mutationId: mutation.mutationId, position: index, opIndex: index, label: localized(mutation.label) })) })) as unknown as Parameters<typeof timeTravelPeerPresenceV1>[1];
    const peerBody = node("framework.history", { type: "tree", interactionDomain: null }, [
      section("framework.history.commands", "Commands", rows.map((row) => treeItem(`framework.history.entry.${row.seq}`, row.mutations[0]!.label.en, [button(`framework.history.entry.${row.seq}.edit`, "Edit", bind(controller, "historyEditBegin", { mutationId: row.mutations[0]!.mutationId }))]))),
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
