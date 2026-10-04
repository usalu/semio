/** 🐾️ Pet companions: the learner's choice between no pets, motionless, calm (unless chosen otherwise) and lively ones is
 * kept on the device and offered in both languages, and a switch on the footer of every screen hides the pets and
 * brings them back as lively as they were; a device that asks for reduced motion decides the default only — its learner
 * gets motionless pets and is told why until they choose, and what a learner chose in the preferences or with the
 * switch holds whatever the device asks for, which is why that the learner chose is stored as a fact of its own;
 * every step has its scene — the quiz on screen or home; the species and the layer are fetched only when pets are
 * wanted, a fetch that fails or a layer that throws leaves the quiz untouched and silent; the layer stands on the
 * cards of the screen, keeps clear of what a learner drags and glances at the other learners' cursors; and the
 * preferences name the pets the layer says are on stage right now. The learner lets the pets answer clicks and be
 * picked up and lets them play with the page — both yes until the learner says no, both off while the pets are off or
 * still —; a press on what of the quiz acts on one is never a pet's; the quiz marks the topics pets may play with
 * (the tasks of a quiz's page, the items of a run that are no table rows, the true order of a sorting's results) and
 * gives the cards of the overview their quiz as a topic they are never lifted for; and the settings play with every
 * pet on stage by the keyboard, saying politely what was asked. The whole client is driven with the sample menagerie
 * of the pets product and the real layer of `@semio-tech/pets-react`; the screens are those of the shared sheet
 * `icons-1`.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/🐾️pets/🟦️.tsx
 * @see ../../../🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json — the sample menagerie
 * @see ../../🧫️fixtures/🃏️sheet-assembly/🔣️.json — `icons-1`
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html
 * @see https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html
 */

import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import uniq from "lodash/uniq";
import { useEffect, useImperativeHandle, type ReactElement } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { decodeQueryEnvelope, encodeQueryResult, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { DEEDS, type Menagerie, type Species, type Text } from "@semio-tech/pets";
import { PET_KEEPOUTS, type PetLayerProps } from "@semio-tech/pets-react";
import type { CatalogView, Query, RunResult, RunView, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/quiz";
import {
  CardIcon,
  ClassificationTaskView,
  EMPTY_PRESENCE_VIEW,
  HomeScreen,
  PET_CHOICES,
  PET_CHOICE_LABELS,
  PET_DEEDS,
  PET_DEED_LABELS,
  PET_DEED_SAID,
  PET_HOME_SCENE,
  PetTopic,
  PetsSwitch,
  PreferencesPanel,
  PresenceOverlay,
  QUIZ_LOCALES,
  QUIZ_PET_CONTROLS,
  QUIZ_PET_KEEPOUTS,
  QUIZ_PET_PROPS,
  QUIZ_PET_SURFACES,
  QuizApp,
  QuizCard,
  QuizPets,
  QuizPetsProvider,
  ResultsScreen,
  RunScreen,
  effectivePetMode,
  initialQuizState,
  localStore,
  memoryStorageOrigin,
  QUIZ_PETS_TEMPO,
  peerGlances,
  petNames,
  petPlayers,
  petProp,
  petScene,
  petsTempo,
  quizInstance,
  quizText,
  readPreferences,
  switchedPets,
  withPets,
  writePreferences,
  type PeerCursor,
  type PetChoice,
  type PresenceConnect,
  type QuizPetsSource,
  type QuizPetsStage,
  type QuizPreferences,
  type QuizSession,
  type QuizState,
  type QuizStep,
} from "@semio-tech/quiz-react";
import vectors from "../../../🐾️pets/🧫️fixtures/🧬️schema-conformance/🔣️.json";
import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";

//#region 🧫️Doubles
const text = (en: string, de: string): Text => ({ en, de });

function species(id: string, name: Text): Species {
  return {
    id,
    name,
    thing: name,
    grounds: [],
    size: { width: 40, height: 48 },
    palette: { body: "#ffcc33", accent: "#ff8800", detail: "#663300" },
    bones: [{ id: "root", x: 0, y: 0 }],
    parts: [],
    face: { eyes: [] },
    clips: [],
    repertoire: {},
    locomotion: { gait: "float", speed: 30, hover: 4 },
    temperament: { energy: 0.5, sociability: 0.5, curiosity: 0.5 },
    states: [{ id: "resting", name: text("Resting", "In Ruhe") }],
    tricks: [],
    purr: { clip: "purr" },
    emitters: [],
    gear: [],
    grip: 43,
    reach: 10,
    mood: "content",
  };
}

const MENAGERIE: Menagerie = {
  schema: "semio.pets.menagerie/v1",
  id: "test",
  title: text("Test pets", "Testtierchen"),
  species: [species("sunny", text("Sunny", "Sonni")), species("housy", text("Housy", "Hausi")), species("windy", text("Windy", "Windi")), species("flamy", text("Flamy", "Flammi"))],
  bonds: [],
  casts: [
    { scene: "home", core: ["sunny", "housy"], rotation: ["windy", "sunny"] },
    { scene: "heating", core: ["flamy"], rotation: ["housy", "ghost"] },
  ],
  chemistry: [],
};

const CATALOG: CatalogView = {
  id: "arch",
  title: text("Architecture", "Architektur"),
  introduction: { title: text("Welcome", "Willkommen"), paragraphs: [] },
  quizzes: [
    { id: "heating", emoji: "🔥", title: text("Heating", "Heizen"), description: text("Warmth.", "Wärme."), tasks: [] },
    { id: "cooling", emoji: "❄️", title: text("Cooling", "Kühlen"), description: text("Cold.", "Kälte."), tasks: [] },
  ],
  badges: [],
};

const RUNS: QuizState["runs"] = { r1: { run: "r1", quiz: "heating" } as RunView, r2: { run: "r2", quiz: "cooling" } as RunView };
const PREFERENCES: QuizPreferences = { theme: "system", textSize: "normal", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "calm", petsLiveliness: "calm", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} };
const ON_STAGE: Readonly<Record<string, readonly string[]>> = { home: ["sunny", "windy"], heating: ["flamy", "ghost", "housy"] };
const REDUCED_MOTION = "(prefers-reduced-motion: reduce)";
const SAMPLE = vectors.menagerie as unknown as Menagerie;
const TIMING = { minMs: 1, maxMs: 4 };
const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });
const encoder = new TextEncoder();
const decoder = new TextDecoder();

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** 🛂️ A proctor that knows the catalog and nothing else. */
function catalogProctor(): HttpTransport {
  return {
    send: async (request) => {
      if (request.method === "GET" && request.path === "/instance") return reply(200, quizInstance());
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      if (request.path !== "/queries") return reply(404, { kind: "notFound", message: request.path });
      const query = JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query;
      if (query.type !== "catalog") return reply(404, { kind: "notFound", message: query.type });
      return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(CATALOG)), frontier: null }));
    },
  };
}

function at(step: QuizStep): Pick<QuizState, "step" | "runs" | "catalog"> {
  return { step, runs: RUNS, catalog: CATALOG };
}

/** 🫧️ A render target double: its layer records what the quiz hands it, shows it as attributes, says, like the real
 * one, who is on stage — the species of `ON_STAGE` for the scene — and nobody once it is gone, and its handle writes
 * down every deed it is asked for (`played`, as `"<species> <deed>"`). */
function fakeStage(): { readonly stage: QuizPetsStage & ReturnType<typeof vi.fn>; readonly handed: PetLayerProps[]; readonly played: string[] } {
  const handed: PetLayerProps[] = [];
  const played: string[] = [];
  const PetLayer = (props: PetLayerProps): ReactElement => {
    handed.push(props);
    const { scene, onCast, ref } = props;
    useImperativeHandle(ref, () => ({ play: (species, deed) => void played.push(`${species} ${deed}`) }), []);
    useEffect(() => {
      onCast?.(ON_STAGE[scene] ?? []);
      return () => onCast?.([]);
    }, [scene, onCast]);
    return <div data-testid="pets" aria-hidden="true" data-scene={props.scene} data-mode={props.mode} data-quiet={String(props.quiet)} />;
  };
  return { stage: vi.fn(async () => ({ PetLayer, PET_KEEPOUTS: "button, p" })), handed, played };
}

function fakeSource(): QuizPetsSource & ReturnType<typeof vi.fn> {
  return vi.fn(async () => MENAGERIE);
}

/** ⏳️ Lets every settled fetch reach the screen. */
async function settle(): Promise<void> {
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
  });
}

/** 🧘️ Emulates the device's motion preference; the returned function changes it while the page is open, and only who
 * asked about motion hears of it — every other media query stays unmatched and silent. */
function motionPreference(reduce: boolean): (reduce: boolean) => void {
  let matches = reduce;
  const listeners = new Set<(event: MediaQueryListEvent) => void>();
  vi.stubGlobal("matchMedia", (query: string) => ({
    get matches(): boolean {
      return query === REDUCED_MOTION && matches;
    },
    media: query,
    onchange: null,
    addEventListener: (_type: string, listener: (event: MediaQueryListEvent) => void) => (query === REDUCED_MOTION ? listeners.add(listener) : listeners),
    removeEventListener: (_type: string, listener: (event: MediaQueryListEvent) => void) => listeners.delete(listener),
    addListener: () => undefined,
    removeListener: () => undefined,
    dispatchEvent: () => false,
  }));
  return (next) => {
    matches = next;
    act(() => {
      for (const listener of listeners) listener({ matches: next, media: REDUCED_MOTION } as MediaQueryListEvent);
    });
  };
}

function quiet(): readonly ReturnType<typeof vi.spyOn>[] {
  return (["log", "info", "warn", "error", "debug"] as const).map((level) => vi.spyOn(console, level).mockImplementation(() => undefined));
}

function boxed(element: Element, left: number, top: number): void {
  element.getBoundingClientRect = () => ({ left, top, right: left + 16, bottom: top + 16, width: 16, height: 16, x: left, y: top, toJSON: () => ({}) });
}

const layer = (): HTMLElement | null => screen.queryByTestId("pets");

function Pets(props: { readonly source?: QuizPetsSource; readonly stage?: QuizPetsStage; readonly choice: PetChoice; readonly chosen?: boolean; readonly play?: boolean; readonly mischief?: boolean; readonly step?: QuizStep; readonly locale?: "en" | "de"; readonly children?: ReactElement }): ReactElement {
  return (
    <QuizPetsProvider source={props.source} stage={props.stage} choice={props.choice} chosen={props.chosen ?? false} play={props.play ?? true} mischief={props.mischief ?? true} state={at(props.step ?? { screen: "home" })} locale={props.locale ?? "en"}>
      <p>quiz</p>
      {props.children}
      <QuizPets />
    </QuizPetsProvider>
  );
}

const SHEET = sheets.sheets.find((vector) => vector.id === "icons-1")!.sheet as unknown as Sheet;
const OPEN_RUN: RunView = { run: "run-1", learner: "learner-1", quiz: SHEET.quiz, status: "open", sheet: SHEET, answers: {}, startedAt: 1_000_000 };
const HOME_CATALOG: CatalogView = {
  ...CATALOG,
  quizzes: [
    { ...CATALOG.quizzes[0]!, tasks: [{ id: "u-values", kind: "classification", title: text("U-values", "U-Werte") }, { id: "heating-load-and-demand", kind: "sorting", title: text("Heating load", "Heizlast") }] },
    { ...CATALOG.quizzes[1]!, tasks: [{ id: "air-change-rates", kind: "matching", title: text("Air changes", "Luftwechsel") }] },
  ],
};

/** 🧾️ The run of `SHEET` once submitted, with a result for each of its three tasks; the true order of its sorting is
 * the reverse of the sheet's. */
function submittedRun(): RunView {
  const [sources, appliances, masses] = SHEET.tasks as readonly [SheetClassificationTask, SheetMatchingTask, SheetSortingTask];
  const category = sources.categories[0]!.id;
  const result: RunResult = {
    quiz: SHEET.quiz,
    challenge: "medium",
    score: 0.5,
    points: 100,
    tasks: [
      { kind: "classification", task: sources.id, score: 1, items: sources.items.map((item) => ({ item: item.id, assigned: category, correct: category, credit: 1 })) },
      { kind: "matching", task: appliances.id, score: 0, dimensions: appliances.dimensions.map((dimension) => ({ dimension: dimension.id, score: 0, items: appliances.items.map((item) => ({ item: item.id, correct: 1 })) })) },
      { kind: "sorting", task: masses.id, score: 0, items: masses.items.map((item, index) => ({ item: item.id, value: 10 ** index, position: index, rank: masses.items.length - 1 - index })) },
    ],
  };
  return { ...OPEN_RUN, status: "submitted", result, submittedAt: 1_100_000 };
}

/** 🏠️ The overview of a learner who knows the catalog, on `step`. */
function homeAt(step: QuizStep): QuizState {
  return { ...initialQuizState({ introduced: true, learner: { id: "a".repeat(32), identity: { kind: "pseudonym", handle: "Ada" } }, catalog: HOME_CATALOG, learnerView: undefined, runs: {} }), step };
}

/** 🛋️ A session double for the screens: everything it is asked does nothing and answers at once. */
function idleSession(): QuizSession {
  const done = async (): Promise<undefined> => undefined;
  return { now: () => 2_000_000, open: () => undefined, chooseBoard: () => undefined, startRun: done, resumeRun: done, loadRun: done, forgetLearner: () => undefined, refreshLeaderboard: async () => ({ answered: true }), refreshCrowd: done, openTask: done, answer: () => undefined, askCrowd: () => undefined, unaskCrowd: () => undefined, submit: done } as unknown as QuizSession;
}

/** 🔑️ The tag, the parent's tag and the topic key of every element on screen that pets may play with. */
function marked(selector = "[data-pet-prop]"): readonly (readonly [string, string | undefined, string | undefined])[] {
  return [...document.querySelectorAll<HTMLElement>(selector)].map((element) => [element.tagName, element.parentElement?.tagName, element.dataset.petProp]);
}

/** 📛️ The name of a group named by another element. */
function named(group: HTMLElement): string | undefined {
  return document.getElementById(group.getAttribute("aria-labelledby") ?? "")?.textContent ?? undefined;
}

afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});
//#endregion 🧫️Doubles

describe("🐾️ pet companions", () => {
  describe("the learner's choice", () => {
    it("is calm unless the learner chose otherwise, and stays on the device", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store).pets).toBe("calm");
      for (const choice of PET_CHOICES) {
        writePreferences(store, { ...readPreferences(store), pets: choice });
        expect(readPreferences(localStore(origin.tab(), "arch")).pets).toBe(choice);
      }
      store.write("preferences", { theme: "dark", pets: "wild" });
      expect(readPreferences(store)).toMatchObject({ theme: "dark", pets: "calm", petsLiveliness: "calm" });
    });

    it("remembers how lively the pets were while they are off, so the switch brings them back as they were", () => {
      const store = localStore(memoryStorageOrigin().tab(), "arch");
      expect(readPreferences(store).petsLiveliness).toBe("calm");
      for (const choice of ["still", "calm", "lively"] as const) {
        const chosen = withPets(readPreferences(store), choice);
        expect(chosen).toMatchObject({ pets: choice, petsLiveliness: choice, petsChosen: true });
        const hidden = withPets(chosen, switchedPets(false, chosen.petsLiveliness));
        expect(hidden).toMatchObject({ pets: "off", petsLiveliness: choice, petsChosen: true });
        writePreferences(store, hidden);
        const read = readPreferences(store);
        expect(read).toMatchObject({ pets: "off", petsLiveliness: choice, petsChosen: true });
        expect(withPets(read, switchedPets(true, read.petsLiveliness))).toMatchObject({ pets: choice, petsLiveliness: choice, petsChosen: true });
      }
      store.write("preferences", { pets: "off", petsLiveliness: "off" });
      expect(readPreferences(store)).toMatchObject({ pets: "off", petsLiveliness: "calm" });
      store.write("preferences", { pets: "lively", petsLiveliness: "still" });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively" });
    });

    it("knows whether the learner chose: a fact of its own that stays on the device, that only a choice for the pets states and that preferences stored without it do not carry", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store)).toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: false });
      writePreferences(store, { ...readPreferences(store), locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false });
      expect(readPreferences(localStore(origin.tab(), "arch"))).toEqual({ locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false, iconsChosen: false, pets: "calm", petsLiveliness: "calm", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} });
      for (const choice of PET_CHOICES) {
        const device = memoryStorageOrigin();
        const fresh = localStore(device.tab(), "arch");
        writePreferences(fresh, withPets(readPreferences(fresh), choice));
        expect(readPreferences(localStore(device.tab(), "arch"))).toMatchObject({ pets: choice, petsChosen: true });
        writePreferences(fresh, { ...readPreferences(fresh), theme: "light", animateIcons: false });
        expect(readPreferences(localStore(device.tab(), "arch"))).toMatchObject({ theme: "light", pets: choice, petsChosen: true });
      }
      const untouched = readPreferences(localStore(memoryStorageOrigin().tab(), "arch"));
      expect(withPets(untouched, switchedPets(false, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "off", petsLiveliness: "calm", petsChosen: true });
      expect(withPets(untouched, switchedPets(true, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "calm", petsLiveliness: "calm", petsChosen: true });
      const before: readonly Record<string, unknown>[] = [{ pets: "lively", petsLiveliness: "lively" }, { pets: "off", petsLiveliness: "still" }, { theme: "dark" }, { pets: "still", petsChosen: "true" }, { pets: "calm", petsChosen: 1 }, { pets: "calm", petsChosen: null }];
      for (const stored of before) {
        store.write("preferences", stored);
        expect(readPreferences(store).petsChosen, JSON.stringify(stored)).toBe(false);
      }
      store.write("preferences", { pets: "lively", petsChosen: true });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: true });
      store.write("preferences", { pets: "wild", petsChosen: true });
      expect(readPreferences(store)).toMatchObject({ pets: "calm", petsChosen: true });
    });

    it("is the learner's own only through the pets' row: no other control of the preferences states it", () => {
      const onChange = vi.fn();
      render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={onChange} />);
      const row = screen.getByRole("group", { name: "Pets" });
      const others = [...screen.getAllByRole("button"), ...screen.getAllByRole("checkbox")].filter((control) => !row.contains(control));
      expect(others.length).toBeGreaterThanOrEqual(14);
      for (const control of others) {
        fireEvent.click(control);
        expect(onChange.mock.lastCall?.[0], control.closest("label")?.textContent ?? control.textContent ?? "").toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: false });
      }
      expect(onChange).toHaveBeenCalledTimes(others.length);
      for (const button of within(row).getAllByRole("button")) {
        fireEvent.click(button);
        expect(onChange.mock.lastCall?.[0], button.textContent ?? "").toMatchObject({ petsChosen: true });
      }
    });

    it("offers off, still, calm and lively in both languages", () => {
      expect([...PET_CHOICES]).toEqual(["off", "still", "calm", "lively"]);
      expect(Object.keys(PET_CHOICE_LABELS)).toEqual([...PET_CHOICES]);
      const labels = { en: { group: "Pets", choices: ["Off", "Still", "Calm", "Lively"] }, de: { group: "Tierchen", choices: ["Aus", "Reglos", "Ruhig", "Lebhaft"] } };
      for (const locale of QUIZ_LOCALES) {
        const onChange = vi.fn();
        const { unmount } = render(<PreferencesPanel preferences={PREFERENCES} locale={locale} text={quizText(locale)} onChange={onChange} />);
        const buttons = within(screen.getByRole("group", { name: labels[locale].group })).getAllByRole("button");
        expect(buttons.map((button) => button.textContent)).toEqual(labels[locale].choices);
        expect(buttons.map((button) => button.getAttribute("aria-pressed"))).toEqual(["false", "false", "true", "false"]);
        buttons.forEach((button, index) => {
          fireEvent.click(button);
          expect(onChange).toHaveBeenLastCalledWith({ ...PREFERENCES, pets: PET_CHOICES[index], petsLiveliness: PET_CHOICES[index] === "off" ? "calm" : PET_CHOICES[index], petsChosen: true });
        });
        unmount();
      }
    });

    it("has a switch for every screen: a checkbox named in both languages that the keyboard operates, and none on a site without pets", () => {
      const names = { en: "Show pets", de: "Tierchen anzeigen" };
      for (const locale of QUIZ_LOCALES) {
        expect(quizText(locale)("quiz.preferences.petsShown")).toBe(names[locale]);
        for (const shown of [true, false]) {
          const onChange = vi.fn();
          const { unmount } = render(
            <QuizPetsProvider source={fakeSource()} stage={fakeStage().stage} choice="off" chosen={false} play mischief state={at({ screen: "home" })} locale={locale}>
              <PetsSwitch shown={shown} label={names[locale]} onChange={onChange} />
            </QuizPetsProvider>,
          );
          const box = screen.getByRole("checkbox", { name: names[locale] }) as HTMLInputElement;
          expect(box.checked).toBe(shown);
          expect(box.tabIndex).toBe(0);
          expect(box.closest("label")?.textContent).toBe(names[locale]);
          fireEvent.click(box);
          expect(onChange).toHaveBeenLastCalledWith(!shown);
          unmount();
        }
      }
      const { container } = render(
        <QuizPetsProvider source={undefined} choice="calm" chosen={false} play mischief state={at({ screen: "home" })} locale="en">
          <PetsSwitch shown label="Show pets" onChange={() => undefined} />
        </QuizPetsProvider>,
      );
      expect(container.innerHTML).toBe("");
      expect(render(<PetsSwitch shown label="Show pets" onChange={() => undefined} />).container.innerHTML).toBe("");
    });
  });

  describe("the liveliness", () => {
    it("is what the learner chose whatever the device asks for, and until the learner chooses motionless on a device that asks for reduced motion", () => {
      const table: readonly (readonly [choice: PetChoice, chosen: boolean, reducedMotion: boolean, mode: PetChoice])[] = [
        ["off", false, false, "off"],
        ["off", false, true, "off"],
        ["off", true, false, "off"],
        ["off", true, true, "off"],
        ["still", false, false, "still"],
        ["still", false, true, "still"],
        ["still", true, false, "still"],
        ["still", true, true, "still"],
        ["calm", false, false, "calm"],
        ["calm", false, true, "still"],
        ["calm", true, false, "calm"],
        ["calm", true, true, "calm"],
        ["lively", false, false, "lively"],
        ["lively", false, true, "still"],
        ["lively", true, false, "lively"],
        ["lively", true, true, "lively"],
      ];
      expect(uniq(table.map(([choice, chosen, reducedMotion]) => `${choice} ${chosen} ${reducedMotion}`))).toHaveLength(PET_CHOICES.length * 2 * 2);
      expect(uniq(table.map(([choice]) => choice))).toEqual([...PET_CHOICES]);
      for (const [choice, chosen, reducedMotion, mode] of table) expect(effectivePetMode(choice, chosen, reducedMotion), `${choice}, chosen ${chosen}, reduced motion ${reducedMotion}`).toBe(mode);
    });

    it("follows the device's motion preference while the page is open until the learner chooses, and the learner's choice from then on", async () => {
      const prefer = motionPreference(true);
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const { rerender } = render(<Pets source={source} stage={stage} choice="lively" />);
      await settle();
      expect(layer()?.dataset.mode).toBe("still");
      prefer(false);
      expect(layer()?.dataset.mode).toBe("lively");
      prefer(true);
      expect(layer()?.dataset.mode).toBe("still");
      expect(handed.every((props) => props.mode === "still" || props.mode === "lively")).toBe(true);

      for (const choice of ["lively", "calm", "still"] as const) {
        rerender(<Pets source={source} stage={stage} choice={choice} chosen />);
        expect(layer()?.dataset.mode).toBe(choice);
        prefer(false);
        expect(layer()?.dataset.mode).toBe(choice);
        prefer(true);
        expect(layer()?.dataset.mode).toBe(choice);
      }
      rerender(<Pets source={source} stage={stage} choice="off" chosen />);
      expect(layer()).toBeNull();
      expect(source).toHaveBeenCalledTimes(1);
    });

    it("loads nothing for a learner who prefers reduced motion and switched the pets off", async () => {
      motionPreference(true);
      const source = fakeSource();
      const { stage } = fakeStage();
      render(<Pets source={source} stage={stage} choice="off" />);
      await settle();
      expect(layer()).toBeNull();
      expect(source).not.toHaveBeenCalled();
    });
  });

  describe("the scene", () => {
    it("is home wherever no quiz is on screen", () => {
      const steps: readonly QuizStep[] = [{ screen: "introduction" }, { screen: "identity" }, { screen: "home" }, { screen: "home", page: "learner" }, { screen: "home", page: "intro" }, { screen: "home", page: "board" }, { screen: "home", page: "badges" }, { screen: "home", page: "prefs" }, { screen: "home", page: "nowhere" }];
      for (const step of steps) expect(petScene(step, RUNS, CATALOG), JSON.stringify(step)).toBe(PET_HOME_SCENE);
      expect(PET_HOME_SCENE).toBe("home");
    });

    it("is the quiz whose page is opened, that is being played or whose results show", () => {
      expect(petScene({ screen: "home", page: "heating" }, RUNS, CATALOG)).toBe("heating");
      expect(petScene({ screen: "home", page: "cooling" }, RUNS, CATALOG)).toBe("cooling");
      expect(petScene({ screen: "run", run: "r1" }, RUNS, CATALOG)).toBe("heating");
      expect(petScene({ screen: "results", run: "r2" }, RUNS, CATALOG)).toBe("cooling");
    });

    it("stays home until the catalog and the run are known", () => {
      expect(petScene({ screen: "home", page: "heating" }, RUNS, undefined)).toBe("home");
      expect(petScene({ screen: "run", run: "r9" }, RUNS, CATALOG)).toBe("home");
      expect(petScene({ screen: "results", run: "r9" }, {}, undefined)).toBe("home");
    });
  });

  describe("the layer", () => {
    it("is fetched once pets are wanted and then lives in the scene of the step", async () => {
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const { rerender } = render(<Pets source={source} stage={stage} choice="calm" />);
      expect(layer()).toBeNull();
      await settle();
      expect(source).toHaveBeenCalledTimes(1);
      expect(stage).toHaveBeenCalledTimes(1);
      expect(handed.at(-1)).toEqual({
        menagerie: MENAGERIE,
        scene: "home",
        mode: "calm",
        quiet: false,
        play: true,
        mischief: true,
        surfaces: QUIZ_PET_SURFACES,
        keepouts: `button, p, ${QUIZ_PET_KEEPOUTS}`,
        controls: QUIZ_PET_CONTROLS,
        props: QUIZ_PET_PROPS,
        glances: peerGlances,
        onCast: expect.any(Function),
        tempo: 1,
        ref: { current: { play: expect.any(Function) } },
      });
      expect(new Set(handed.map((props) => props.onCast)).size).toBe(1);
      expect(new Set(handed.map((props) => props.ref)).size).toBe(1);
      expect(layer()?.getAttribute("aria-hidden")).toBe("true");

      rerender(<Pets source={source} stage={stage} choice="calm" step={{ screen: "home", page: "heating" }} />);
      expect(handed.at(-1)).toMatchObject({ scene: "heating", quiet: false });
      rerender(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r2" }} />);
      expect(handed.at(-1)).toMatchObject({ scene: "cooling", quiet: true });
      rerender(<Pets source={source} stage={stage} choice="lively" step={{ screen: "results", run: "r2" }} />);
      expect(handed.at(-1)).toMatchObject({ scene: "cooling", quiet: false, mode: "lively" });
      rerender(<Pets source={source} stage={stage} choice="still" step={{ screen: "identity" }} />);
      expect(handed.at(-1)).toMatchObject({ scene: "home", mode: "still" });
      expect(source).toHaveBeenCalledTimes(1);
      expect(stage).toHaveBeenCalledTimes(1);
    });

    it("neither loads nor renders anything while pets are off, and loads when they are switched on", async () => {
      const source = fakeSource();
      const { stage } = fakeStage();
      const { rerender, container } = render(<Pets source={source} stage={stage} choice="off" />);
      await settle();
      expect(source).not.toHaveBeenCalled();
      expect(stage).not.toHaveBeenCalled();
      expect(container.innerHTML).toBe("<p>quiz</p>");

      rerender(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      expect(layer()).not.toBeNull();
      rerender(<Pets source={source} stage={stage} choice="off" />);
      expect(container.innerHTML).toBe("<p>quiz</p>");
      rerender(<Pets source={source} stage={stage} choice="lively" />);
      expect(layer()?.dataset.mode).toBe("lively");
      expect(source).toHaveBeenCalledTimes(1);
    });

    it("loads nothing for a site without pets", async () => {
      const { stage } = fakeStage();
      const { container } = render(<Pets stage={stage} choice="lively" />);
      await settle();
      expect(stage).not.toHaveBeenCalled();
      expect(container.innerHTML).toBe("<p>quiz</p>");
    });

    it("stays away silently when the species or the layer cannot be fetched, and tries again when pets are chosen anew", async () => {
      const said = quiet();
      const source = vi.fn<QuizPetsSource>().mockRejectedValueOnce(new Error("offline")).mockResolvedValue(MENAGERIE);
      const { stage } = fakeStage();
      const { rerender, container } = render(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      expect(container.innerHTML).toBe("<p>quiz</p>");
      rerender(<Pets source={source} stage={stage} choice="off" />);
      rerender(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      expect(source).toHaveBeenCalledTimes(2);
      expect(layer()).not.toBeNull();

      const broken = vi.fn<QuizPetsStage>(() => {
        throw new Error("no chunk");
      });
      const other = render(<Pets source={fakeSource()} stage={broken} choice="calm" />);
      await settle();
      expect(broken).toHaveBeenCalledTimes(1);
      expect(other.container.innerHTML).toBe("<p>quiz</p>");
      for (const spy of said) expect(spy).not.toHaveBeenCalled();
    });

    it("drops a fetch that outlives the client or the wish for pets", async () => {
      const said = quiet();
      let deliver: (menagerie: Menagerie) => void = () => undefined;
      const source = vi.fn<QuizPetsSource>(() => new Promise((resolve) => (deliver = resolve)));
      const { stage, handed } = fakeStage();
      const { rerender, unmount } = render(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      const first = deliver;
      rerender(<Pets source={source} stage={stage} choice="off" />);
      first(MENAGERIE);
      await settle();
      rerender(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      expect(source).toHaveBeenCalledTimes(2);
      expect(handed).toHaveLength(0);
      unmount();
      deliver(MENAGERIE);
      await settle();
      expect(handed).toHaveLength(0);
      for (const spy of said) expect(spy).not.toHaveBeenCalled();
    });

    it("vanishes without taking the quiz along when it throws", async () => {
      vi.spyOn(console, "error").mockImplementation(() => undefined);
      const stage: QuizPetsStage = async () => ({
        PetLayer: () => {
          throw new Error("broken rig");
        },
        PET_KEEPOUTS: "button",
      });
      const { container } = render(<Pets source={fakeSource()} stage={stage} choice="calm" />);
      await settle();
      expect(container.innerHTML).toBe("<p>quiz</p>");
    });

    it("stands on the cards of the screen and keeps clear of what a learner drags and drops on", () => {
      const task: SheetClassificationTask = {
        kind: "classification",
        id: "climates",
        title: text("Climates", "Klimazonen"),
        prompt: text("Assign each place its climate.", "Ordne jedem Ort sein Klima zu."),
        categories: [{ id: "hot", label: text("Hot", "Heiß") }],
        items: [{ id: "desert", label: text("Desert", "Wüste") }],
      };
      render(
        <div className="quiz-app">
          <header>
            <span>Quizzes</span>
          </header>
          <main id="quiz-main">
            <QuizCard id="task" card="task" icon={<CardIcon icon="info" />} title="Climates" footerRight={<button type="button">Next</button>}>
              <ClassificationTaskView task={task} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />
            </QuizCard>
          </main>
          <footer className="quiz-footer" />
          <QuizCard id="dialog" card="dialog" icon={<CardIcon icon="info" />} title="Submit?" />
        </div>,
      );
      const body = document.createElement("div");
      body.dataset.slot = "window-chrome-body-surface";
      document.querySelector('#quiz-main [data-slot="window-chrome-chip-cap"]')!.parentElement!.after(body);
      const surfaces = [...document.querySelectorAll<HTMLElement>(QUIZ_PET_SURFACES)];
      expect(surfaces.map((part) => [part.dataset.slot ?? part.tagName, part.closest<HTMLElement>("[data-card]")?.dataset.card])).toEqual([
        ["window-chrome-chip-cap", "task"],
        ["window-chrome-body-surface", "task"],
        ["FOOTER", undefined],
      ]);
      expect(surfaces[0]!.textContent?.trim()).toBe("Climates");
      expect([surfaces[0]!.dataset.dock, surfaces[0]!.hasAttribute("data-window-silhouette-chip")]).toEqual(["top", true]);
      expect(surfaces[0]!.querySelector("button")).toBeNull();
      const clear = [...document.querySelectorAll<HTMLElement>(QUIZ_PET_KEEPOUTS)];
      expect(clear.filter((element) => element.tagName === "HEADER")).toEqual([document.querySelector(".quiz-app > header")]);
      expect(clear.filter((element) => element.dataset.quizItem !== undefined).map((element) => element.dataset.quizItem)).toEqual(["desert"]);
      expect(clear.filter((element) => element.dataset.quizDrop !== undefined).map((element) => element.dataset.quizDrop)).toEqual(["pool", "category:hot"]);
      expect([...document.querySelectorAll(`${PET_KEEPOUTS}, ${QUIZ_PET_KEEPOUTS}`)]).toEqual(expect.arrayContaining([...clear, screen.getByRole("combobox", { name: "Category for Desert" })]));
    });

    it("lives in real time unless the document root names another tempo: the seam of the end-to-end proof of an encounter", async () => {
      expect(QUIZ_PETS_TEMPO).toBe("data-pets-tempo");
      expect(petsTempo()).toBe(1);
      try {
        for (const [said, tempo] of [["8", 8], ["0.5", 0.5], ["fast", 1], ["0", 1], ["-2", 1], ["Infinity", 1]] as const) {
          document.documentElement.setAttribute(QUIZ_PETS_TEMPO, said);
          expect(petsTempo(), said).toBe(tempo);
        }
        document.documentElement.setAttribute(QUIZ_PETS_TEMPO, "8");
        const { stage, handed } = fakeStage();
        render(<Pets source={fakeSource()} stage={stage} choice="lively" />);
        await settle();
        expect(handed.at(-1)).toMatchObject({ mode: "lively", tempo: 8 });
      } finally {
        document.documentElement.removeAttribute(QUIZ_PETS_TEMPO);
      }
    });

    it("glances at the tips of the other learners' cursors while they show", () => {
      expect(peerGlances()).toEqual([]);
      const peers: readonly PeerCursor[] = [
        { session: "w1", tag: "0badc0de", label: "Mira K.", colour: 3, cursor: { anchor: "run", x: 0.5, y: 0.5 }, focus: undefined, drag: undefined },
        { session: "w2", tag: "0badc0df", label: "Ben", colour: 4, cursor: { anchor: "elsewhere", x: 0.1, y: 0.1 }, focus: "run", drag: undefined },
      ];
      const { rerender } = render(
        <>
          <div data-presence-anchor="run" />
          <PresenceOverlay view={{ ...EMPTY_PRESENCE_VIEW, peers }} show />
        </>,
      );
      const marks = [...document.querySelectorAll<HTMLElement>("[data-presence-layer] .quiz-peer")];
      expect(marks.map((mark) => mark.hidden)).toEqual([false, true]);
      boxed(marks[0]!, 320, 180);
      boxed(marks[1]!, 0, 0);
      expect(peerGlances()).toEqual([{ x: 320, y: 180 }]);
      rerender(
        <>
          <div data-presence-anchor="run" />
          <PresenceOverlay view={{ ...EMPTY_PRESENCE_VIEW, peers }} show={false} />
        </>,
      );
      expect(peerGlances()).toEqual([]);
    });
  });

  describe("the pets on stage", () => {
    it("are named in the learner's language, in the order the layer names them, and nobody the menagerie does not know", () => {
      expect(petNames(MENAGERIE, ["windy", "sunny"], "en")).toEqual(["Windy", "Sunny"]);
      expect(petNames(MENAGERIE, ["flamy", "ghost", "housy"], "de")).toEqual(["Flammi", "Hausi"]);
      expect(petNames(MENAGERIE, [], "en")).toEqual([]);
      expect(petNames(SAMPLE, ["blobby", "hoppy", "floaty"], "de")).toEqual(["Blobby, der Klecks", "Hoppy, die Sprungfeder", "Floaty, der Ballon"]);
      for (const locale of QUIZ_LOCALES) expect(petNames(SAMPLE, uniq(SAMPLE.casts.flatMap((cast) => [...cast.core, ...cast.rotation])), locale)).toEqual(uniq(SAMPLE.casts.flatMap((cast) => [...cast.core, ...cast.rotation])).map((id) => SAMPLE.species.find((entry) => entry.id === id)!.name[locale]));
    });

    it("are named under the choice as soon as the layer says who is there, in both languages, and no longer once they are gone", async () => {
      const lines = { en: { home: "Here right now: Sunny · Windy", heating: "Here right now: Flamy · Housy" }, de: { home: "Gerade hier: Sonni · Windi", heating: "Gerade hier: Flammi · Hausi" } };
      for (const locale of QUIZ_LOCALES) {
        const source = fakeSource();
        const { stage } = fakeStage();
        const panel = <PreferencesPanel preferences={PREFERENCES} locale={locale} text={quizText(locale)} onChange={() => undefined} />;
        const { rerender, unmount } = render(
          <Pets source={source} stage={stage} choice="calm" locale={locale}>
            {panel}
          </Pets>,
        );
        expect(screen.queryByText(/ · /u)).toBeNull();
        await settle();
        const group = screen.getByRole("group", { name: quizText(locale)("quiz.preferences.pets") });
        expect(group.nextElementSibling?.textContent).toBe(lines[locale].home);
        rerender(
          <Pets source={source} stage={stage} choice="still" locale={locale} step={{ screen: "home", page: "heating" }}>
            {panel}
          </Pets>,
        );
        expect(group.nextElementSibling?.textContent).toBe(lines[locale].heating);
        rerender(
          <Pets source={source} stage={stage} choice="off" locale={locale} step={{ screen: "home", page: "heating" }}>
            {panel}
          </Pets>,
        );
        expect(group.nextElementSibling).toBeNull();
        unmount();
      }
    });

    it("are not named where no pets were handed in", () => {
      render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={() => undefined} />);
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling).toBeNull();
    });

    it("stay still for a device that asks for reduced motion until the learner chooses: the preferences press what is in effect and say why in both languages, and say nothing once the learner chose", async () => {
      const notes = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };
      const prefer = motionPreference(true);
      for (const locale of QUIZ_LOCALES) {
        prefer(true);
        const source = fakeSource();
        const { stage } = fakeStage();
        const pets = (choice: PetChoice, chosen: boolean): ReactElement => (
          <Pets source={source} stage={stage} choice={choice} chosen={chosen} locale={locale}>
            <PreferencesPanel preferences={{ ...PREFERENCES, pets: choice, petsChosen: chosen }} locale={locale} text={quizText(locale)} onChange={() => undefined} />
          </Pets>
        );
        const { rerender, unmount } = render(pets("lively", false));
        const group = screen.getByRole("group", { name: quizText(locale)("quiz.preferences.pets") });
        const pressed = (): PetChoice[] => PET_CHOICES.filter((_, index) => within(group).getAllByRole("button")[index]?.getAttribute("aria-pressed") === "true");
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.tagName).toBe("P");
        expect(pressed()).toEqual(["still"]);
        await settle();
        expect(layer()?.dataset.mode).toBe("still");
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.nextElementSibling?.textContent).toContain(" · ");

        for (const choice of ["calm", "lively", "still"] as const) {
          rerender(pets(choice, true));
          expect(layer()?.dataset.mode, choice).toBe(choice);
          expect(pressed(), choice).toEqual([choice]);
          expect(screen.queryByText(notes[locale]), choice).toBeNull();
          expect(group.nextElementSibling?.textContent, choice).toContain(" · ");
          rerender(pets(choice, false));
          expect(layer()?.dataset.mode, choice).toBe("still");
          expect(pressed(), choice).toEqual(["still"]);
          expect(group.nextElementSibling?.textContent, choice).toBe(notes[locale]);
        }
        for (const chosen of [true, false]) {
          rerender(pets("off", chosen));
          expect(layer()).toBeNull();
          expect(pressed()).toEqual(["off"]);
          expect(screen.queryByText(notes[locale])).toBeNull();
        }
        prefer(false);
        rerender(pets("lively", false));
        expect(layer()?.dataset.mode).toBe("lively");
        expect(pressed()).toEqual(["lively"]);
        expect(screen.queryByText(notes[locale])).toBeNull();
        unmount();
      }
      prefer(true);
      render(
        <Pets choice="lively">
          <PreferencesPanel preferences={{ ...PREFERENCES, pets: "lively" }} locale="en" text={quizText("en")} onChange={() => undefined} />
        </Pets>,
      );
      expect(screen.queryByText(notes.en)).toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }).getAttribute("aria-pressed")).toBe("true");
    });

    it("show nobody for a device that forces its own colours, whatever the learner chose, and the preferences say that instead of the word on motion, in both languages", async () => {
      const notes = { en: "Your device uses its own contrast colours, so no pets show.", de: "Dein Gerät verwendet eigene Kontrastfarben, deshalb erscheinen keine Tierchen." };
      const still = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };
      const answers = ["(forced-colors: active)", REDUCED_MOTION];
      vi.stubGlobal("matchMedia", (query: string) => ({ matches: answers.includes(query), media: query, onchange: null, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, dispatchEvent: () => false }));
      for (const locale of QUIZ_LOCALES) {
        const panel = (choice: PetChoice, chosen = false): ReactElement => <PreferencesPanel preferences={{ ...PREFERENCES, pets: choice, petsChosen: chosen }} locale={locale} text={quizText(locale)} onChange={() => undefined} />;
        const source = fakeSource();
        const { stage } = fakeStage();
        const { rerender, unmount } = render(
          <Pets source={source} stage={stage} choice="calm" locale={locale}>
            {panel("calm")}
          </Pets>,
        );
        const group = screen.getByRole("group", { name: quizText(locale)("quiz.preferences.pets") });
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.tagName).toBe("P");
        expect(screen.queryByText(still[locale])).toBeNull();
        await settle();
        expect(layer()?.dataset.mode).toBe("still");
        for (const choice of ["still", "calm", "lively"] as const) {
          rerender(
            <Pets source={source} stage={stage} choice={choice} chosen locale={locale}>
              {panel(choice, true)}
            </Pets>,
          );
          expect(layer()?.dataset.mode, choice).toBe(choice);
          expect(group.nextElementSibling?.textContent, choice).toBe(notes[locale]);
          expect(screen.queryByText(still[locale]), choice).toBeNull();
        }
        rerender(
          <Pets source={source} stage={stage} choice="off" locale={locale}>
            {panel("off")}
          </Pets>,
        );
        expect(screen.queryByText(notes[locale])).toBeNull();
        unmount();
      }
      render(
        <Pets choice="calm">
          <PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={() => undefined} />
        </Pets>,
      );
      expect(screen.queryByText(notes.en)).toBeNull();
    });
  });

  describe("the client", () => {
    const client = (storage: ReturnType<ReturnType<typeof memoryStorageOrigin>["tab"]>, pets?: QuizPetsSource): ReactElement => (
      <QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => catalogProctor()} storage={storage} languages={["en"]} timing={TIMING} pets={pets} />
    );
    const app = (): HTMLElement => document.querySelector<HTMLElement>(".quiz-app")!;

    it("carries the choice, fetches the site's menagerie and the real layer, and lets go of both when pets are switched off", async () => {
      const origin = memoryStorageOrigin();
      const pets = vi.fn<QuizPetsSource>(async () => SAMPLE);
      render(client(origin.tab(), pets));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      expect(app().dataset.pets).toBe("calm");
      const shown = await waitFor(() => {
        const found = app().querySelector<HTMLElement>(".pet-layer");
        expect(found).not.toBeNull();
        return found!;
      });
      expect(pets).toHaveBeenCalledTimes(1);
      expect(shown.getAttribute("aria-hidden")).toBe("true");
      expect(shown).toBe(app().lastElementChild);
      expect(shown.querySelector("a, button, input, select, textarea, [tabindex]")).toBeNull();
      const parts = [...document.querySelectorAll<HTMLElement>(QUIZ_PET_SURFACES)].filter((part) => part.closest("[inert]") === null);
      const cards = uniq(parts.flatMap((part) => part.closest<HTMLElement>("[data-card]")?.dataset.card ?? []));
      expect(cards.length).toBeGreaterThanOrEqual(1);
      for (const card of cards) expect(parts.filter((part) => part.closest<HTMLElement>("[data-card]")?.dataset.card === card).map((part) => part.dataset.slot)).toEqual(["window-chrome-chip-cap"]);
      expect(parts.at(-1)).toBe(app().querySelector(":scope > footer"));
      expect(parts.at(-1)).not.toBeNull();
      expect([...document.querySelectorAll(QUIZ_PET_KEEPOUTS)]).toContain(app().querySelector(":scope > header"));
      const group = screen.getByRole("group", { name: "Pets" });

      fireEvent.click(within(group).getByRole("button", { name: "Off" }));
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(group.nextElementSibling).toBeNull();
      expect(readPreferences(localStore(origin.tab(), CATALOG.id)).pets).toBe("off");

      fireEvent.click(within(group).getByRole("button", { name: "Still" }));
      expect(app().dataset.pets).toBe("still");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(pets).toHaveBeenCalledTimes(1);
    });

    it("keeps a switch on the footer of every screen that hides the pets and brings them back as lively as they were", async () => {
      const origin = memoryStorageOrigin();
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      const footer = app().querySelector<HTMLElement>(":scope > footer")!;
      const box = within(footer).getByRole("checkbox", { name: "Show pets" }) as HTMLInputElement;
      expect(box.checked).toBe(true);
      expect(box.closest("nav")).toBeNull();
      expect(within(footer).getAllByRole("button").map((button) => button.textContent)).toEqual(["What is stored"]);
      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }));
      expect(app().dataset.pets).toBe("lively");

      fireEvent.click(box);
      expect(box.checked).toBe(false);
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Off" }).getAttribute("aria-pressed")).toBe("true");
      expect(readPreferences(localStore(origin.tab(), CATALOG.id))).toMatchObject({ pets: "off", petsLiveliness: "lively" });

      fireEvent.click(box);
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("lively");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(readPreferences(localStore(origin.tab(), CATALOG.id))).toMatchObject({ pets: "lively", petsLiveliness: "lively" });
    });

    it("holds the pets of a device that asks for reduced motion still only until the learner chooses in the preferences, and keeps that choice for the next visit", async () => {
      const note = "Your device asks for less motion, so the pets stay still until you choose.";
      motionPreference(true);
      const origin = memoryStorageOrigin();
      const stored = (): QuizPreferences => readPreferences(localStore(origin.tab(), CATALOG.id));
      const row = (): HTMLElement => screen.getByRole("group", { name: "Pets" });
      const pressed = (): (string | null)[] => within(row()).getAllByRole("button").filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.textContent);
      const first = render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("still");
      expect(pressed()).toEqual(["Still"]);
      expect(row().nextElementSibling?.textContent).toBe(note);

      fireEvent.click(within(screen.getByRole("group", { name: "Theme" })).getByRole("button", { name: "Dark" }));
      fireEvent.click(screen.getByRole("checkbox", { name: "Animate icons" }));
      expect(stored()).toMatchObject({ theme: "dark", animateIcons: true, iconsChosen: true, pets: "calm", petsLiveliness: "calm", petsChosen: false });
      expect(app().dataset.pets).toBe("still");
      expect(row().nextElementSibling?.textContent).toBe(note);

      fireEvent.click(within(row()).getByRole("button", { name: "Calm" }));
      expect(app().dataset.pets).toBe("calm");
      expect(pressed()).toEqual(["Calm"]);
      expect(screen.queryByText(note)).toBeNull();
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(stored()).toMatchObject({ theme: "dark", pets: "calm", petsLiveliness: "calm", petsChosen: true });

      first.unmount();
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("calm");
      expect(pressed()).toEqual(["Calm"]);
      expect(screen.queryByText(note)).toBeNull();
    });

    it("counts the switch as a choice: on a device that asks for reduced motion it hides the still pets and brings calm ones back", async () => {
      const note = "Your device asks for less motion, so the pets stay still until you choose.";
      motionPreference(true);
      const origin = memoryStorageOrigin();
      const stored = (): QuizPreferences => readPreferences(localStore(origin.tab(), CATALOG.id));
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      const box = within(app().querySelector<HTMLElement>(":scope > footer")!).getByRole("checkbox", { name: "Show pets" }) as HTMLInputElement;
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("still");
      expect(screen.getByText(note)).not.toBeNull();

      fireEvent.click(box);
      expect(box.checked).toBe(false);
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(screen.queryByText(note)).toBeNull();
      expect(stored()).toMatchObject({ pets: "off", petsLiveliness: "calm", petsChosen: true });

      fireEvent.click(box);
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("calm");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(screen.queryByText(note)).toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Calm" }).getAttribute("aria-pressed")).toBe("true");
      expect(stored()).toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: true });
    });

    it("takes a liveliness that was stored without the word that the learner chose it for the default: still on a device that asks for reduced motion, as stored on any other", async () => {
      const note = "Your device asks for less motion, so the pets stay still until you choose.";
      const prefer = motionPreference(true);
      const origin = memoryStorageOrigin();
      localStore(origin.tab(), CATALOG.id).write("preferences", { theme: "dark", pets: "lively", petsLiveliness: "lively" });
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("still");
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling?.textContent).toBe(note);
      prefer(false);
      expect(app().dataset.pets).toBe("lively");
      expect(screen.queryByText(note)).toBeNull();
      prefer(true);
      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }));
      expect(app().dataset.pets).toBe("lively");
      expect(screen.queryByText(note)).toBeNull();
      expect(readPreferences(localStore(origin.tab(), CATALOG.id))).toMatchObject({ theme: "dark", pets: "lively", petsLiveliness: "lively", petsChosen: true });
    });

    it("fetches nothing for a learner who switched the pets off", async () => {
      const origin = memoryStorageOrigin();
      writePreferences(localStore(origin.tab(), CATALOG.id), { ...PREFERENCES, pets: "off" });
      const pets = vi.fn<QuizPetsSource>(async () => SAMPLE);
      render(client(origin.tab(), pets));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await settle();
      expect(app().dataset.pets).toBe("off");
      expect(pets).not.toHaveBeenCalled();
      expect(document.querySelector(".pet-layer")).toBeNull();
    });

    it("carries the choice on a site without pets, and shows none", async () => {
      render(client(memoryStorageOrigin().tab()));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await settle();
      expect(app().dataset.pets).toBe("calm");
      expect(document.querySelector(".pet-layer")).toBeNull();
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling).toBeNull();
      expect(screen.queryByRole("checkbox", { name: "Show pets" })).toBeNull();
    });

    it("plays with the pets of the real layer from the settings, and keeps what the learner allows for the next visit", async () => {
      const origin = memoryStorageOrigin();
      const stored = (): QuizPreferences => readPreferences(localStore(origin.tab(), CATALOG.id));
      const first = render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      const play = await screen.findByRole("group", { name: "Play with the pets" });
      const pets = within(play).getAllByRole("group");
      const known = SAMPLE.species.map((entry) => entry.name.en);
      expect(pets.length).toBeGreaterThan(0);
      for (const pet of pets) expect(known).toContain(named(pet));
      fireEvent.click(within(pets[0]!).getByRole("button", { name: "Hello" }));
      expect(within(play).getByRole("status").textContent).toBe(`${named(pets[0]!)} says hello.`);
      fireEvent.click(within(pets[0]!).getByRole("button", { name: "Toss" }));
      expect(within(play).getByRole("status").textContent).toBe(`${named(pets[0]!)} is tossed up.`);

      fireEvent.click(screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }));
      expect(screen.queryByRole("group", { name: "Play with the pets" })).toBeNull();
      expect(stored()).toMatchObject({ pets: "calm", petsChosen: false, petsPlay: false, petsMischief: true });
      fireEvent.click(screen.getByRole("checkbox", { name: "Pets may play with the page" }));
      expect(stored()).toMatchObject({ petsPlay: false, petsMischief: false });
      expect(app().querySelector(".pet-layer")).not.toBeNull();

      first.unmount();
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).checked).toBe(false);
      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).checked).toBe(false);
      fireEvent.click(screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }));
      expect(await screen.findByRole("group", { name: "Play with the pets" })).not.toBeNull();
      expect(stored()).toMatchObject({ petsPlay: true, petsMischief: false, petsChosen: false });
    });
  });

  describe("what the learner allows", () => {
    it("lets the pets answer clicks, be picked up and play with the page until the learner says no, keeps that on the device, and never counts it as a choice of liveliness", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store)).toMatchObject({ petsPlay: true, petsMischief: true, petsChosen: false });
      for (const [petsPlay, petsMischief] of [
        [false, true],
        [true, false],
        [false, false],
        [true, true],
      ] as const) {
        writePreferences(store, { ...readPreferences(store), petsPlay, petsMischief });
        expect(readPreferences(localStore(origin.tab(), "arch"))).toMatchObject({ pets: "calm", petsChosen: false, petsPlay, petsMischief });
      }
      const before: readonly Record<string, unknown>[] = [{}, { petsPlay: "false", petsMischief: 0 }, { petsPlay: null, petsMischief: "no" }, { petsPlay: true, petsMischief: true }];
      for (const stored of before) {
        store.write("preferences", stored);
        expect(readPreferences(store), JSON.stringify(stored)).toMatchObject({ petsPlay: true, petsMischief: true });
      }
      store.write("preferences", { petsPlay: false, petsMischief: false });
      expect(readPreferences(store)).toMatchObject({ petsPlay: false, petsMischief: false, petsChosen: false });
      expect(withPets(readPreferences(store), "lively")).toMatchObject({ pets: "lively", petsChosen: true, petsPlay: false, petsMischief: false });
      expect(withPets(readPreferences(store), switchedPets(false, "calm"))).toMatchObject({ pets: "off", petsPlay: false, petsMischief: false });
    });

    it("is two checkboxes in both languages that change only themselves, off and disabled with the reason while the pets are off or still", async () => {
      const user = userEvent.setup();
      const words = {
        en: { play: "Pets react to clicks and can be picked up", mischief: "Pets may play with the page", resting: "Only calm and lively pets play." },
        de: { play: "Tierchen reagieren auf Klicks und lassen sich hochheben", mischief: "Tierchen dürfen mit der Seite spielen", resting: "Nur ruhige und lebhafte Tierchen spielen." },
      };
      for (const locale of QUIZ_LOCALES) {
        const onChange = vi.fn();
        const panel = (preferences: QuizPreferences): ReactElement => <PreferencesPanel preferences={preferences} locale={locale} text={quizText(locale)} onChange={onChange} />;
        const { rerender, unmount } = render(panel(PREFERENCES));
        const play = (): HTMLInputElement => screen.getByRole("checkbox", { name: words[locale].play }) as HTMLInputElement;
        const mischief = (): HTMLInputElement => screen.getByRole("checkbox", { name: words[locale].mischief }) as HTMLInputElement;
        const states = (): readonly boolean[] => [play().checked, play().disabled, mischief().checked, mischief().disabled];
        expect(states()).toEqual([true, false, true, false]);
        expect(screen.queryByText(words[locale].resting)).toBeNull();
        fireEvent.click(play());
        expect(onChange).toHaveBeenLastCalledWith({ ...PREFERENCES, petsPlay: false });
        fireEvent.click(mischief());
        expect(onChange).toHaveBeenLastCalledWith({ ...PREFERENCES, petsMischief: false });
        rerender(panel({ ...PREFERENCES, petsPlay: false, petsMischief: false }));
        expect(states()).toEqual([false, false, false, false]);
        fireEvent.click(play());
        expect(onChange).toHaveBeenLastCalledWith({ ...PREFERENCES, petsPlay: true, petsMischief: false });
        for (const pets of ["off", "still"] as const) {
          onChange.mockClear();
          rerender(panel({ ...PREFERENCES, pets, petsChosen: true }));
          expect(states(), pets).toEqual([false, true, false, true]);
          const note = screen.getByText(words[locale].resting);
          expect([play().getAttribute("aria-describedby"), mischief().getAttribute("aria-describedby")], pets).toEqual([note.id, note.id]);
          await user.click(play());
          await user.click(mischief());
          await user.keyboard(" ");
          expect(onChange, pets).not.toHaveBeenCalled();
        }
        rerender(panel({ ...PREFERENCES, pets: "lively", petsChosen: true }));
        expect(states()).toEqual([true, false, true, false]);
        expect(screen.queryByText(words[locale].resting)).toBeNull();
        unmount();
      }
    });

    it("is handed to the layer as it is while the pets are calm or lively, not at all while they are still, and in a run the same — the stage keeps a run quiet", async () => {
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const pets = (choice: PetChoice, play: boolean, mischief: boolean, step?: QuizStep): ReactElement => <Pets source={source} stage={stage} choice={choice} chosen play={play} mischief={mischief} step={step} />;
      const last = (): Pick<PetLayerProps, "mode" | "quiet" | "play" | "mischief"> => {
        const { mode, quiet, play, mischief } = handed.at(-1)!;
        return { mode, quiet, play, mischief };
      };
      const { rerender } = render(pets("calm", true, true));
      await settle();
      expect(last()).toEqual({ mode: "calm", quiet: false, play: true, mischief: true });
      rerender(pets("calm", false, true));
      expect(last()).toEqual({ mode: "calm", quiet: false, play: false, mischief: true });
      rerender(pets("lively", true, false));
      expect(last()).toEqual({ mode: "lively", quiet: false, play: true, mischief: false });
      rerender(pets("still", true, true));
      expect(last()).toEqual({ mode: "still", quiet: false, play: false, mischief: false });
      rerender(pets("calm", true, true, { screen: "run", run: "r1" }));
      expect(last()).toEqual({ mode: "calm", quiet: true, play: true, mischief: true });
      rerender(pets("lively", false, false, { screen: "run", run: "r1" }));
      expect(last()).toEqual({ mode: "lively", quiet: true, play: false, mischief: false });
      rerender(pets("lively", true, true, { screen: "results", run: "r1" }));
      expect(last()).toEqual({ mode: "lively", quiet: false, play: true, mischief: true });
      expect(new Set(handed.map((props) => props.ref)).size).toBe(1);
      expect(source).toHaveBeenCalledTimes(1);
    });

    it("lets nothing play while a device that asks for reduced motion holds the pets still by default", async () => {
      motionPreference(true);
      const { stage, handed } = fakeStage();
      render(
        <Pets source={fakeSource()} stage={stage} choice="calm">
          <PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={() => undefined} />
        </Pets>,
      );
      await settle();
      expect(handed.at(-1)).toMatchObject({ mode: "still", play: false, mischief: false });
      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).disabled).toBe(true);
      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).disabled).toBe(true);
      expect(screen.getByText("Only calm and lively pets play.")).not.toBeNull();
      expect(screen.queryByRole("group", { name: "Play with the pets" })).toBeNull();
    });
  });

  describe("the hand and the topics", () => {
    it("never takes a press from what of the quiz acts on one — a grip, a drop zone — and plays only with marked topics, never with the copy a drag carries", () => {
      const task: SheetClassificationTask = {
        kind: "classification",
        id: "climates",
        title: text("Climates", "Klimazonen"),
        prompt: text("Assign each place its climate.", "Ordne jedem Ort sein Klima zu."),
        categories: [{ id: "hot", label: text("Hot", "Heiß") }],
        items: [
          { id: "desert", label: text("Desert", "Wüste") },
          { id: "fjord", label: text("Fjord", "Fjord") },
        ],
      };
      render(
        <div className="quiz-app">
          <main id="quiz-main">
            <QuizCard id="task" card="task" icon={<CardIcon icon="info" />} title="Climates">
              <PetTopic topic={petProp("geo", "climates")}>
                <ClassificationTaskView task={task} answer={undefined} onAnswer={() => undefined} text={quizText("en")} locale="en" />
              </PetTopic>
            </QuizCard>
          </main>
        </div>,
      );
      const grips = [...document.querySelectorAll<HTMLElement>("[data-quiz-grip]")];
      expect(grips).toHaveLength(2);
      for (const grip of grips) expect(grip.closest(QUIZ_PET_CONTROLS)).toBe(grip);
      const zones = [...document.querySelectorAll<HTMLElement>("[data-quiz-drop]")];
      expect(zones.map((zone) => zone.dataset.quizDrop)).toEqual(["pool", "category:hot"]);
      for (const zone of zones) expect(zone.closest(QUIZ_PET_CONTROLS)).toBe(zone);
      expect(screen.getByRole("heading", { name: "Climates" }).closest(QUIZ_PET_CONTROLS)).toBeNull();
      expect(marked(QUIZ_PET_PROPS)).toEqual([
        ["LI", "UL", "geo/climates/desert"],
        ["LI", "UL", "geo/climates/fjord"],
      ]);
      expect(petProp(undefined, "desert")).toBeUndefined();

      fireEvent.pointerDown(grips[0]!, { button: 0, clientX: 10, clientY: 10 });
      act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX: 60, clientY: 40 })));
      const ghost = document.querySelector<HTMLElement>(".quiz-drag-ghost");
      expect(ghost?.querySelector<HTMLElement>("[data-pet-prop]")?.dataset.petProp).toBe("geo/climates/desert");
      expect(document.querySelectorAll("[data-pet-prop]")).toHaveLength(3);
      expect(marked(QUIZ_PET_PROPS)).toEqual([
        ["LI", "UL", "geo/climates/desert"],
        ["LI", "UL", "geo/climates/fjord"],
      ]);
      act(() => void window.dispatchEvent(new PointerEvent("pointerup", { clientX: 60, clientY: 40 })));
      expect(document.querySelector(".quiz-drag-ghost")).toBeNull();
    });

    it("gives every quiz card of the overview its quiz as a topic it is never lifted for — a press on a card is the card's — and marks every task of an opened quiz's page with `<quiz>/<task>`", () => {
      const home = (step: QuizStep): ReactElement => <HomeScreen session={idleSession()} state={homeAt(step)} text={quizText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />;
      const { rerender } = render(home({ screen: "home" }));
      const cards = [...document.querySelectorAll<HTMLElement>("[data-pet-topic]")];
      expect(cards.map((card) => card.dataset.petTopic).sort()).toEqual(["cooling", "heating"]);
      for (const card of cards) {
        expect(card.dataset.card).toBe(`quiz:${card.dataset.petTopic}`);
        expect(card.hasAttribute("data-pet-prop")).toBe(false);
        expect(card.closest(QUIZ_PET_CONTROLS)).not.toBeNull();
        expect(card.closest(QUIZ_PET_CONTROLS)).toBe(card.closest("[data-layered-card]"));
        expect(card.querySelectorAll("[data-pet-prop]")).toHaveLength(0);
      }
      expect([...document.querySelectorAll(QUIZ_PET_PROPS)].filter((element) => element.closest("[data-layered-card]") !== null)).toEqual([]);

      rerender(home({ screen: "home", page: "heating" }));
      const page = document.querySelector<HTMLElement>('[data-page="quiz:heating"]')!;
      const rows = [...page.querySelectorAll<HTMLElement>(QUIZ_PET_PROPS)];
      expect(rows.map((row) => [row.tagName, row.dataset.petProp, row.closest<HTMLElement>("[data-card]")?.dataset.card])).toEqual([
        ["LI", "heating/u-values", "quiz-tasks"],
        ["LI", "heating/heating-load-and-demand", "quiz-tasks"],
      ]);
      expect(rows.map((row) => row.textContent)).toEqual([expect.stringContaining("U-values"), expect.stringContaining("Heating load")]);
      expect(page.querySelector("[data-pet-topic]")).toBeNull();
    });

    it("marks the items of a run that are no table rows with `<quiz>/<task>/<item>` — a classification's chips, a matching's item rows (never its cards), a sorting's rows — and nothing else of the run", () => {
      const [sources, appliances, masses] = SHEET.tasks as readonly [SheetClassificationTask, SheetMatchingTask, SheetSortingTask];
      render(<RunScreen session={idleSession()} state={initialQuizState({ introduced: true, runs: { [OPEN_RUN.run]: OPEN_RUN } })} run={OPEN_RUN.run} text={quizText("en")} locale="en" />);
      const steps = within(screen.getByRole("navigation", { name: "Tasks" })).getAllByRole("button");
      expect(marked()).toEqual(sources.items.map((item) => ["LI", "UL", `${SHEET.quiz}/${sources.id}/${item.id}`]));
      for (const chip of document.querySelectorAll<HTMLElement>("[data-pet-prop]")) expect(chip.dataset.petProp).toBe(`${SHEET.quiz}/${sources.id}/${chip.dataset.quizItem}`);
      fireEvent.click(steps[1]!);
      expect(appliances.dimensions).toHaveLength(2);
      expect(marked()).toEqual(appliances.dimensions.flatMap(() => appliances.items.map((item) => ["LI", "UL", `${SHEET.quiz}/${appliances.id}/${item.id}`])));
      const valueCards = [...document.querySelectorAll("li[data-quiz-drag]")];
      expect(valueCards.length).toBeGreaterThan(0);
      for (const card of valueCards) expect(card.hasAttribute("data-pet-prop")).toBe(false);
      fireEvent.click(steps[2]!);
      expect(marked()).toEqual(masses.items.map((item) => ["LI", "OL", `${SHEET.quiz}/${masses.id}/${item.id}`]));
      expect(document.querySelectorAll("section[data-pet-prop], section[data-pet-topic], table [data-pet-prop], [data-pet-prop] [data-pet-prop]")).toHaveLength(0);
      expect(marked(QUIZ_PET_PROPS)).toEqual(marked());
    });

    it("marks the true order of a sorting's results with `<quiz>/<task>/<item>`, never a row of a table", () => {
      const view = submittedRun();
      const masses = SHEET.tasks[2] as SheetSortingTask;
      render(<ResultsScreen session={idleSession()} state={initialQuizState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others="never" text={quizText("en")} locale="en" />);
      expect(marked()).toEqual([...masses.items].reverse().map((item) => ["LI", "OL", `${SHEET.quiz}/${masses.id}/${item.id}`]));
      expect(document.querySelectorAll("tr").length).toBeGreaterThan(0);
      expect(document.querySelectorAll("table [data-pet-prop], tr[data-pet-prop], [data-pet-topic]")).toHaveLength(0);
      expect(marked(QUIZ_PET_PROPS)).toEqual(marked());
    });
  });

  describe("play with the pets", () => {
    it("offers the stage's own deeds, worded differently in both languages", () => {
      expect([...PET_DEEDS]).toEqual([...DEEDS]);
      expect(Object.keys(PET_DEED_LABELS)).toEqual([...PET_DEEDS]);
      expect(Object.keys(PET_DEED_SAID)).toEqual([...PET_DEEDS]);
      const words = { en: ["Hello", "Trick", "Pet", "Toss"], de: ["Hallo", "Kunststück", "Streicheln", "Hochwerfen"] };
      for (const locale of QUIZ_LOCALES) expect(PET_DEEDS.map((deed) => quizText(locale)(PET_DEED_LABELS[deed]))).toEqual(words[locale]);
      expect(words.en.filter((word) => words.de.includes(word))).toEqual([]);
      expect(petPlayers(MENAGERIE, ["windy", "ghost", "sunny"], "de")).toEqual([
        { species: "windy", name: "Windi" },
        { species: "sunny", name: "Sonni" },
      ]);
    });

    it("names every pet on stage in a group of its own whose buttons ask the layer for a deed by pointer or keyboard, and says politely what was asked, in both languages", async () => {
      const said = {
        en: { group: "Play with the pets", names: ["Sunny", "Windy"], hello: "Sunny says hello.", trick: "Windy does a trick.", pet: "Windy is petted.", toss: "Sunny is tossed up." },
        de: { group: "Mit den Tierchen spielen", names: ["Sonni", "Windi"], hello: "Sonni sagt Hallo.", trick: "Windi macht ein Kunststück.", pet: "Windi wird gestreichelt.", toss: "Sonni wird hochgeworfen." },
      };
      const user = userEvent.setup();
      for (const locale of QUIZ_LOCALES) {
        const { stage, played } = fakeStage();
        const { unmount } = render(
          <Pets source={fakeSource()} stage={stage} choice="calm" locale={locale}>
            <PreferencesPanel preferences={PREFERENCES} locale={locale} text={quizText(locale)} onChange={() => undefined} />
          </Pets>,
        );
        expect(screen.queryByRole("group", { name: said[locale].group })).toBeNull();
        await settle();
        const play = screen.getByRole("group", { name: said[locale].group });
        expect(within(play).getAllByRole("group").map(named)).toEqual(said[locale].names);
        const [sunny, windy] = said[locale].names.map((name) => within(play).getByRole("group", { name }));
        for (const pet of [sunny!, windy!]) expect(within(pet).getAllByRole("button").map((button) => button.textContent)).toEqual(PET_DEEDS.map((deed) => quizText(locale)(PET_DEED_LABELS[deed])));
        const status = within(play).getByRole("status");
        expect([status.textContent, status.getAttribute("aria-live")]).toEqual(["", "polite"]);

        await user.click(within(sunny!).getAllByRole("button")[0]!);
        expect([played, status.textContent]).toEqual([["sunny hello"], said[locale].hello]);
        const trick = within(windy!).getAllByRole("button")[1]!;
        trick.focus();
        await user.keyboard("{Enter}");
        await user.keyboard(" ");
        expect(document.activeElement).toBe(trick);
        expect([played, status.textContent]).toEqual([["sunny hello", "windy trick", "windy trick"], said[locale].trick]);
        await user.tab();
        expect(document.activeElement).toBe(within(windy!).getAllByRole("button")[2]);
        await user.keyboard("{Enter}");
        expect([played.at(-1), status.textContent]).toEqual(["windy pet", said[locale].pet]);
        await user.click(within(sunny!).getAllByRole("button")[3]!);
        expect([played.at(-1), status.textContent]).toEqual(["sunny toss", said[locale].toss]);
        unmount();
      }
    });

    it("is there only while the pets are calm or lively, play is allowed, somebody is on stage and the site has pets", async () => {
      const source = fakeSource();
      const { stage } = fakeStage();
      const panel = <PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={() => undefined} />;
      const group = (): HTMLElement | null => screen.queryByRole("group", { name: "Play with the pets" });
      const { rerender, unmount } = render(
        <Pets source={source} stage={stage} choice="calm">
          {panel}
        </Pets>,
      );
      await settle();
      expect(group()).not.toBeNull();
      const cases: readonly [props: { readonly choice: PetChoice; readonly play?: boolean; readonly mischief?: boolean; readonly step?: QuizStep }, names: readonly string[] | null][] = [
        [{ choice: "calm", play: false }, null],
        [{ choice: "calm", mischief: false }, ["Sunny", "Windy"]],
        [{ choice: "still" }, null],
        [{ choice: "lively", step: { screen: "home", page: "cooling" } }, null],
        [{ choice: "lively", step: { screen: "home", page: "heating" } }, ["Flamy", "Housy"]],
        [{ choice: "lively", step: { screen: "run", run: "r1" } }, ["Flamy", "Housy"]],
        [{ choice: "off" }, null],
      ];
      for (const [props, names] of cases) {
        rerender(
          <Pets source={source} stage={stage} chosen {...props}>
            {panel}
          </Pets>,
        );
        await settle();
        const found = group();
        expect(found === null ? null : within(found).getAllByRole("group").map(named), JSON.stringify(props)).toEqual(names);
      }
      unmount();
      render(<Pets choice="calm">{panel}</Pets>);
      await settle();
      expect(group()).toBeNull();
    });
  });
});
