/** 🧪️ The React history-editing chrome against the language-neutral band corpus (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`):
 * the i18n catalogue pinned to the corpus's one `labels` table (every key, both tiers, both languages, and no other key),
 * every stage's lines in English and German, the controls with their corpus-owned ids and what disables them — a refused
 * one focusable and telling its reason — the reserved verbs they dispatch with the session generation, the remappable chords (never Escape, never from a form field) published on `aria-keyshortcuts`, the
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
import { createElement, Fragment, useCallback, useLayoutEffect, useReducer, useState } from "react";
import { createMachine, transition, type AnyMachineSnapshot } from "xstate";
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { CHROME_CONTROL_TOOLTIP_DELAY_MS, composeControlKeybindings, PresenceBar, SHELL_KEYBINDINGS, TREE_WINDOW_PATH_SEPARATOR, uiChromeTranslationBundles, UIDialog, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { encodePresenceHistoryEdit, encodePresenceInteraction, encodePresenceToolRun } from "@semio-tech/framework-replication";
import { createMemoryStoragePort, type ActionDescriptor, type BuiltNode, type HistoryEntry, type HistoryPatch, type HistoryTimeTravel, type UiIntent } from "@semio-tech/framework";
import type { PanelDock, PanelTabNode } from "@semio-tech/ui-react";
import { initialShellState, shellReducer, type ShellAction, type ShellState } from "../../../../🐚️Shell/🟦️.tsx";
import { builtNodeToSnapshot, UiDocumentStore } from "../../../../📃️UiDocumentStore/🟦️.tsx";
import { checkinSubmitMessageV1, checkpointGateV1, checkpointOnCloseKeyV1, presenceEphemeralPeerFieldsV1, EMPTY_SHELL_HISTORY_PROJECTION_V1, FRAMEWORK_CHECKIN_CONTROLLER_ID, HISTORY_REFUSAL_LABEL_KEYS, historyPatchShouldApplyV1, historyRefusalCodeV1, historyRefusalNoticeV1, historyRefusalOfOutputV1, operationProgressPartsV1, programHistoryProjectionsWithLoadV1, shellHistoryProjectionWithLoadV1, panelActionRoutesThroughHostV1, panelTabDefinitionToNode, shellHistoryCursorDomV1, shellHistoryProjectionAfterPatchV1, shellLabel, syncShellLabelLocale, useCheckpointOnCloseV1, type ShellHistoryProjectionV1 } from "../../../🟦️.tsx";
import { HISTORY_ROW_KEY_PREFIX, HistoryReprojectionStatus, historyReprojectionControlV1, revealHistoryPanelV1, scheduleTimeTravelFocusV1, TIME_TRAVEL_CHORD_IDS, TIME_TRAVEL_CONTROL_IDS, TIME_TRAVEL_ROW_ACTION_REFUSALS, TIME_TRAVEL_ROW_ACTION_VERBS, TimeTravelBand, timeTravelRowActionRefusalsV1, timeTravelBandControlsV1, timeTravelBandTextV1, timeTravelControlActionV1, timeTravelFocusElementV1, timeTravelFocusIsHeldV1, timeTravelIndicatorTextV1, timeTravelPeerPresenceV1, timeTravelTransitionV1, TimeTravelWindowIndicator, useTimeTravelRevealV1, type TimeTravelFocusTargetV1 } from "../../🟦️.tsx";
import { RowActionRefusalsContext, TREE_ROW_TONE_CLASSES, UiPresenceOverlayContext } from "../../../../🗣️Interpreter/🟦️.tsx";
import { TIME_TRAVEL_CODE_LABELS, TIME_TRAVEL_LABELS } from "../../../../../../../../../../🔨️modules/⏪️time-travel/🟦️.ts";
import { HISTORY_NOTICE_LABELS, historyReprojectionStatus } from "../../../../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { documentLoadCancelledV1, HISTORY_SILENT_REFUSALS_V1, historyFaultNoticeV1, historyLaneNoticeV1, historyOutputNoticeV1, historySilentRefusalOfFaultV1, historySilentRefusalOfOutputV1, IMPORT_ABORT_ACTION_ID, importOpenedFilesV1 } from "../../../🟦️.tsx";

const { computeAccessibleName, computeAccessibleDescription, getRole }: typeof AccessibilityOracle = createRequire(import.meta.url)("dom-accessibility-api");
const { roles: ariaRoles, aria: ariaProperties } = createRequire(import.meta.url)("aria-query") as { readonly roles: ReadonlyMap<string, { readonly props: Readonly<Record<string, unknown>> }>; readonly aria: ReadonlyMap<string, unknown> };
const here = dirname(fileURLToPath(import.meta.url));
const shellHelpers = join(here, "..", "..", "..");
const framework = join(here, "..", "..", "..", "..", "..", "..", "..", "..", "..", "..");
const readJson = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const corpus = readJson(join(shellHelpers, "🧫️fixtures", "🧫️time-travel-band", "🔣️.json"));
const rowActionCorpus = readJson(join(framework, "🔨️modules", "🖱️ui", "🧫️fixtures", "♿️disabled-row-action", "🔣️.json")) as readonly { readonly id: string; readonly label: string; readonly focusable: boolean; readonly actionable: boolean }[];
type Case = { readonly name: string; readonly session: HistoryTimeTravel; readonly text: Readonly<Record<"en" | "de", Record<string, string | null>>>; readonly controls: readonly { readonly control: string; readonly controlId: string; readonly action: string; readonly disabledBy: string | null; readonly args?: Readonly<Record<string, string>> }[]; readonly indicator: Readonly<Record<"en" | "de", string>> };
/** 🏷️ The corpus's one copy of every band, indicator, peer and refusal text: label key → tier → locale, placeholders in single braces. */
type LabelRow = Readonly<Record<"normal" | "beginner", Readonly<Record<"en" | "de", string>>>>;
const labels = corpus.labels as Readonly<Record<string, LabelRow>>;
/** 🧩️ A corpus label at the normal tier with its placeholders filled. */
const corpusLabel = (key: string, locale: "en" | "de", values: Readonly<Record<string, string | number>> = {}): string => labels[key]!.normal[locale].replace(/\{(\w+)\}/gu, (_, name: string) => String(values[name]));
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
/** 💬️ What `aria-describedby` of `element` reads: the text of every element it names, in order. */
const describedBy = (element: Element): string => (element.getAttribute("aria-describedby") ?? "").split(/\s+/u).filter(Boolean).map((id) => document.getElementById(id)?.textContent ?? "").join(" ");
/** 👁️ The disabled reason a sighted person reads right now (`DisabledReasonHint` revealed), with the id it carries. */
const revealedReason = (): { readonly id: string; readonly text: string } | null => {
  const shown = document.querySelector('[data-slot="row-action-reason"][data-revealed]');
  return shown === null ? null : { id: shown.id, text: shown.textContent ?? "" };
};

describe("⏪️ time-travel band corpus", () => {
  afterEach(() => cleanup());
  afterAll(() => syncShellLabelLocale("en"));

  it("validates every session and history patch against the kernel wire schema", () => {
    const ajv = new Ajv({ allErrors: true, strict: false });
    const schema = readJson(join(framework, "🔨️modules", "🎠️kernel", "🧬️schema", "🔣️history-patch", "🔣️.json"));
    ajv.addSchema(schema);
    const session = ajv.compile({ $ref: `${schema.$id}#/definitions/HistoryTimeTravel` });
    const patch = ajv.compile({ $ref: `${schema.$id}#/definitions/HistoryPatch` });
    for (const entry of cases) {
      expect(session(entry.session), `${entry.name}: ${JSON.stringify(session.errors)}`).toBe(true);
      expect(patch({ cursor: 0, timeTravel: entry.session }), `${entry.name}: ${JSON.stringify(patch.errors)}`).toBe(true);
    }
    expect(new Set(cases.map((entry) => entry.session.stage))).toEqual(new Set(["editing", "replaying", "reviewing", "choosing", "finalizing"]));
    for (const malformed of [{ ...cases[0]!.session, generation: -1 }, { ...cases[0]!.session, stage: "unknown" }, { ...cases[0]!.session, initialSnapshot: {} }, { ...cases[0]!.session, initialPack: "00" }, ...[-1, 4294967296, "128"].map(processed => ({ ...cases[0]!.session, processed }))]) expect(session(malformed)).toBe(false);
    expect(patch({ cursor: 0, actor: "actor:alice" })).toBe(false);
  });

  it("pins the i18n catalogue to the corpus's labels table: every key, both tiers, both languages, and no other key", () => {
    const flatten = (node: unknown, prefix: string, out: Map<string, { readonly normal: string; readonly beginner: string }>): void => {
      const entry = node as Readonly<Record<string, unknown>>;
      if (typeof entry.label === "object" && entry.label !== null && Object.keys(entry).length === 1) out.set(prefix, entry.label as { readonly normal: string; readonly beginner: string });
      else for (const [key, value] of Object.entries(entry)) flatten(value, `${prefix}.${key}`, out);
    };
    for (const locale of LOCALES) {
      const ui = (uiChromeTranslationBundles[locale].translation as unknown as { readonly ui: { readonly timeTravel: unknown; readonly history: unknown; readonly mutation: { readonly level: unknown } } }).ui;
      const catalogue = new Map<string, { readonly normal: string; readonly beginner: string }>();
      flatten(ui.timeTravel, "ui.timeTravel", catalogue);
      flatten(ui.history, "ui.history", catalogue);
      flatten(ui.mutation.level, "ui.mutation.level", catalogue);
      expect([...catalogue.keys()].sort(), `${locale}: the catalogue's keys are the table's`).toEqual(Object.keys(labels).sort());
      for (const [key, row] of Object.entries(labels)) for (const tier of ["normal", "beginner"] as const) expect(catalogue.get(key)?.[tier], `${key} (${tier}, ${locale})`).toBe(row[tier][locale].replace(/\{(\w+)\}/gu, "{{$1}}"));
    }
    for (const row of corpus.refusals as readonly { readonly code: string; readonly label: string; readonly text: Readonly<Record<"en" | "de", string>> }[]) expect([HISTORY_REFUSAL_LABEL_KEYS[row.code as keyof typeof HISTORY_REFUSAL_LABEL_KEYS], row.text], row.code).toEqual([row.label, labels[row.label]!.normal]);
    for (const entry of cases) {
      for (const locale of LOCALES) {
        const { session } = entry;
        const target = session.targetLabel === undefined ? null : session.targetLabel.native[locale];
        const progress = (session.stage === "editing" || session.stage === "replaying") && session.total !== undefined ? corpusLabel(session.stage === "editing" ? "ui.timeTravel.preparationProgress" : "ui.timeTravel.progress", locale, { done: session.done ?? 0, total: session.total }) : null;
        expect(entry.text[locale], `${entry.name} (${locale}): every line is a label of the table`).toEqual({
          stage: corpusLabel(`ui.timeTravel.stage.${session.stage}`, locale),
          target: target === null ? null : corpusLabel("ui.timeTravel.target", locale, { target }),
          progress: progress !== null && session.processed !== undefined ? `${progress} · ${corpusLabel("ui.timeTravel.processed", locale, { processed: session.processed })}` : progress,
          review: session.stage === "reviewing" && session.review !== undefined ? corpusLabel(`ui.timeTravel.review.${session.review}`, locale) : null,
          outcome: session.worst === undefined ? null : corpusLabel("ui.timeTravel.worst", locale, { level: corpusLabel(`ui.mutation.level.${session.worst}`, locale) }),
          fault: session.fault === undefined ? null : corpusLabel(HISTORY_REFUSAL_LABEL_KEYS[historyRefusalCodeV1(session.fault) ?? "timeTravel.replay-faulted"], locale),
          accepted: (session.acceptedCount ?? 0) > 0 ? corpusLabel("ui.timeTravel.accepted", locale, { count: session.acceptedCount ?? 0 }) : null,
        });
        expect(entry.indicator[locale], `${entry.name} (${locale})`).toBe(session.stage === "editing" && target !== null ? corpusLabel("ui.timeTravel.indicatorTarget", locale, { target }) : corpusLabel(`ui.timeTravel.stage.${session.stage}`, locale));
      }
    }
    for (const locale of LOCALES) for (const row of Object.values(labels)) for (const tier of ["normal", "beginner"] as const) expect(/time travel|zeitreise/iu.test(row[tier][locale]), `${row[tier][locale]}: one name for the concept — "History editing" / "Verlaufsbearbeitung"`).toBe(false);
  });

  it("derives every stage's lines, controls, verbs and indicator in English and German", () => {
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      for (const entry of cases) {
        expect(timeTravelBandTextV1(entry.session, { ...axes, locale }), `${entry.name} (${locale})`).toEqual(entry.text[locale]);
        expect(timeTravelIndicatorTextV1(entry.session, { ...axes, locale }), `${entry.name} (${locale})`).toBe(entry.indicator[locale]);
        const controls = timeTravelBandControlsV1(entry.session);
        expect(controls.map(({ control, controlId, disabledBy }) => ({ control, controlId, action: timeTravelControlActionV1(corpus.controllerId, entry.session, control).action, disabledBy })), entry.name).toEqual(entry.controls.map(({ control, controlId, action, disabledBy }) => ({ control, controlId, action, disabledBy })));
        for (const row of entry.controls) expect([TIME_TRAVEL_CONTROL_IDS[row.control as keyof typeof TIME_TRAVEL_CONTROL_IDS], timeTravelControlActionV1(corpus.controllerId, entry.session, row.control as keyof typeof TIME_TRAVEL_CONTROL_IDS)], `${entry.name}: ${row.control}`).toEqual([row.controlId, { controllerId: corpus.controllerId, action: row.action, args: row.args ?? { generation: entry.session.generation } }]);
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
        expect(buttons.map((button) => [button.dataset.semioTimeTravelControl, button.id]), entry.name).toEqual(entry.controls.map((row) => [row.control, row.controlId]));
        expect(buttons.map((button) => [button.textContent, button.getAttribute("aria-disabled"), describedBy(button) || null, computeAccessibleDescription(button) || null]), `${entry.name} (${locale})`).toEqual(entry.controls.map((row) => { const reason = row.disabledBy === null ? null : String(shellLabel(row.disabledBy as Parameters<typeof shellLabel>[0])); return [String(shellLabel(`ui.timeTravel.${row.control}` as Parameters<typeof shellLabel>[0])), reason === null ? null : "true", reason, reason]; }));
        expect(buttons.map((button) => [button.disabled, button.hasAttribute("title"), button.tabIndex >= 0]), `${entry.name} (${locale}): a refused control stays reachable — never the disabled attribute, never a title`).toEqual(entry.controls.map(() => [false, false, true]));
        if (entry.session.processed !== undefined) process.stdout.write(`[DEBUG] history processed React band locale=${locale} completed=${entry.session.processed} text=${status.textContent?.replace(/\s+/gu, " ").trim()}\n`);
        view.unmount();
      }
    }
  });

  it("dispatches each enabled control's reserved verb with the session generation, and nothing for a disabled one", () => {
    syncShellLabelLocale("en");
    for (const entry of cases) {
      const view = mountBand(entry.session, "en");
      for (const row of entry.controls) controlButton(view.container, row.control)!.click();
      expect(view.dispatched, entry.name).toEqual(entry.controls.filter((row) => row.disabledBy === null).map((row) => ({ controllerId: corpus.controllerId, action: row.action, args: row.args ?? { generation: entry.session.generation } })));
      view.unmount();
    }
  });

  it("keeps a refused control in the Tab order and shows its reason on hover, keyboard focus and press — hidden again on leave, blur and Escape — in both languages", async () => {
    const user = userEvent.setup();
    const blocked = cases.find((entry) => entry.controls.some((row) => row.control === "finalize" && row.disabledBy === "ui.timeTravel.refusal.blocked") && entry.controls.some((row) => row.control === "rerun" && row.disabledBy !== null))!;
    for (const locale of LOCALES) {
      syncShellLabelLocale(locale);
      const view = mountBand(blocked.session, locale);
      const reached: string[] = [];
      for (let stop = 0; stop < blocked.controls.length; stop += 1) {
        await act(async () => {
          await user.tab();
        });
        reached.push((document.activeElement as HTMLElement | null)?.dataset.semioTimeTravelControl ?? "");
      }
      expect(reached, `${locale}: Tab walks every control, the refused ones too`).toEqual(blocked.controls.map((row) => row.control));
      act(() => (document.activeElement as HTMLElement).blur());
      for (const row of blocked.controls.filter((entry) => entry.disabledBy !== null)) {
        const button = controlButton(view.container, row.control)!;
        const reason = String(shellLabel(row.disabledBy as Parameters<typeof shellLabel>[0]));
        const told = () => [revealedReason()?.text ?? null, describedBy(button), button.getAttribute("aria-describedby") === (revealedReason()?.id ?? button.getAttribute("aria-describedby"))];
        expect(told(), `${row.control} (${locale}): at rest the reason is the description only`).toEqual([null, reason, true]);
        fireEvent.pointerEnter(button, { pointerType: "mouse" });
        expect(revealedReason(), `${row.control} (${locale}): a hover reveals only after the tooltip delay`).toBeNull();
        await act(async () => {
          await new Promise((resolve) => setTimeout(resolve, CHROME_CONTROL_TOOLTIP_DELAY_MS + 50));
        });
        expect(told(), `${row.control} (${locale}): hover`).toEqual([reason, reason, true]);
        fireEvent.pointerLeave(button, { pointerType: "mouse" });
        expect(told(), `${row.control} (${locale}): leave`).toEqual([null, reason, true]);
        act(() => button.focus());
        expect([document.activeElement === button, ...told()], `${row.control} (${locale}): keyboard focus`).toEqual([true, reason, reason, true]);
        fireEvent.keyDown(button, { key: "Escape" });
        expect(told(), `${row.control} (${locale}): Escape`).toEqual([null, reason, true]);
        fireEvent.click(button);
        expect(told(), `${row.control} (${locale}): press`).toEqual([reason, reason, true]);
        act(() => button.blur());
        expect(told(), `${row.control} (${locale}): blur`).toEqual([null, reason, true]);
      }
      expect(view.dispatched, `${locale}: a refused control dispatches nothing, however it is pressed`).toEqual([]);
      expect(band(view.container).querySelector("[data-semio-time-travel-controls]")?.getAttribute("aria-live"), `${locale}: revealing a reason never re-announces the status`).toBe("off");
      expect(ariaFindings([view.container])).toEqual([]);
      view.unmount();
    }
  }, 30_000);

  it("keeps a control's element and the focus on it when the session makes it refused or runnable again", () => {
    syncShellLabelLocale("en");
    const ready = cases.find((entry) => entry.session.stage === "reviewing" && entry.controls.some((row) => row.control === "finalize" && row.disabledBy === null))!;
    const blocked = cases.find((entry) => entry.session.stage === "reviewing" && entry.controls.some((row) => row.control === "finalize" && row.disabledBy === "ui.timeTravel.refusal.blocked") && entry.session.nextProblem === undefined)!;
    const bindings = composeControlKeybindings(new Map(), {});
    const mounted = (session: HistoryTimeTravel) => createElement(UiKeybindingsProvider, { bindings, children: createElement(TimeTravelBand, { session, terminology: axes.terminology, locale: "en", controllerId: corpus.controllerId, onAction: () => undefined }) });
    const view = render(mounted(ready.session));
    const finalize = controlButton(view.container, "finalize")!;
    act(() => finalize.focus());
    view.rerender(mounted(blocked.session));
    const refused = controlButton(view.container, "finalize")!;
    expect([refused === finalize, document.activeElement === finalize, refused.getAttribute("aria-disabled"), revealedReason()?.text], "a focused Finalize that the session blocks keeps its element and says why at once").toEqual([true, true, "true", String(shellLabel("ui.timeTravel.refusal.blocked"))]);
    view.rerender(mounted(ready.session));
    expect([controlButton(view.container, "finalize") === finalize, document.activeElement === finalize, finalize.hasAttribute("aria-disabled"), revealedReason()]).toEqual([true, true, false, null]);
    view.unmount();
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
      ["ui.timeTravel.refusal.readOnly", "refusalReadOnly"],
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
type Transition = { readonly name: string; readonly from: HistoryTimeTravel | null; readonly to: HistoryTimeTravel | null; readonly reveal: boolean; readonly focus: TimeTravelFocusTargetV1 | null; readonly scrollTo: string | null };
/** 🗝️ The draft editor's node keys the corpus holds the Rust producer and every shell's focus resolution to. */
type EditorKeys = { readonly panel: string; readonly accept: { readonly control: string; readonly row: string }; readonly inputs: readonly { readonly pointer: string; readonly control: string; readonly row: string }[] };

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
    const host = readFileSync(join(shellHelpers, "..", "🏛️ShellHost", "🟦️.tsx"), "utf8");
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
    const host = readFileSync(join(shellHelpers, "..", "🏛️ShellHost", "🟦️.tsx"), "utf8");
    const load = host.slice(host.indexOf("const task = documentTransferRef.current.trackDocumentTransfer(file, baseSession.pluginId);"), host.indexOf("`Effect::LoadDocument` is pack+spr bytes only"));
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
