/** 🧩️ The React history-editing chrome against the language-neutral band corpus (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`):
 * the i18n catalogue pinned to the corpus's one `labels` table (every key, both tiers, both languages, and no other key),
 * every stage's lines in English and German, the controls with their corpus-owned ids and what disables them — a refused
 * one focusable and telling its reason — the reserved verbs they dispatch with the session generation, the remappable chords (never Escape, never from a form field) published on `aria-keyshortcuts`, the
 * window indicator's accessible name and the `⏪️time-travel` bundle texts. The history body, the draft editor, the frozen and restored histories, the shell reveal wiring, the
 * mutation pages, the list inputs, the lane notices, the document load and the reprojection status are the sibling suites under `🧪️tests/`, which share `🧰️history-test-support`. Third-party oracles:
 * Ajv validates the corpus against its schema and the kernel's own `HistoryTimeTravel` wire schema, `dom-accessibility-api`
 * names every control, `aria-query` (the WAI-ARIA role model) lists the attributes each role supports,
 * and the `⏪️time-travel` module's `TIME_TRAVEL_LABELS` is the independent source of every stage and refusal text. */
import { join } from "node:path";
import Ajv from "ajv";
import userEvent from "@testing-library/user-event";
import { afterAll, afterEach, describe, expect, it } from "vitest";
import { createElement } from "react";
import { act, cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { CHROME_CONTROL_TOOLTIP_DELAY_MS, composeControlKeybindings, SHELL_KEYBINDINGS, uiChromeTranslationBundles, UiKeybindingsProvider } from "@semio-tech/ui-react";
import { type HistoryPatch, type HistoryTimeTravel } from "@semio-tech/framework";
import {
  EMPTY_SHELL_HISTORY_PROJECTION_V1,
  historyPatchShouldApplyV1,
  operationProgressPartsV1,
  shellHistoryProjectionAfterPatchV1,
  type ShellHistoryProjectionV1
} from "../../../🔩️plumbing/🟦️.ts";
import {
  HISTORY_REFUSAL_LABEL_KEYS,
  historyRefusalCodeV1,
  historyRefusalNoticeV1,
  historyRefusalOfOutputV1
} from "../../../🛣️route-and-chrome/🟦️.tsx";
import { shellLabel, syncShellLabelLocale } from "../../../🏷️labels/🟦️.tsx";
import { TIME_TRAVEL_CHORD_IDS, TIME_TRAVEL_CONTROL_IDS, TimeTravelBand, timeTravelBandControlsV1, timeTravelBandTextV1, timeTravelControlActionV1, timeTravelIndicatorTextV1, TimeTravelWindowIndicator } from "../../🟦️.tsx";
import { TIME_TRAVEL_CODE_LABELS, TIME_TRAVEL_LABELS } from "@semio-tech/framework/time-travel";
import { LOCALES, ariaFindings, axes, band, cases, computeAccessibleDescription, computeAccessibleName, controlButton, corpus, corpusLabel, describedBy, framework, labels, mountBand, readJson, revealedReason } from "../🧰️history-test-support/🟦️.tsx";

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
        if (entry.session.processed !== undefined) {}
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
