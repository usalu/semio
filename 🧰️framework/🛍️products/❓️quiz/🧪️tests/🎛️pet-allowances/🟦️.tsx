/** 🎛️ Pet companions: what the learner allows, the hand and the topics, and play with the pets. */
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { type ReactElement } from "react";
import { describe, expect, it, vi } from "vitest";
import { DEEDS } from "@semio-tech/pets";
import { type PetLayerProps } from "@semio-tech/pets-react";
import type { CatalogView, RunResult, RunView, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/quiz";
import { CardIcon, ClassificationTaskView, HomeScreen, PET_DEEDS, PET_DEED_LABELS, PET_DEED_SAID, PetTopic, PreferencesPanel, QUIZ_LOCALES, QUIZ_PET_CONTROLS, QUIZ_PET_PROPS, QuizCard, ResultsScreen, RunScreen, initialQuizState, localStore, memoryStorageOrigin, petPlayers, petProp, quizText, readPreferences, switchedPets, withPets, writePreferences, type PetChoice, type QuizPreferences, type QuizSession, type QuizState, type QuizStep } from "@semio-tech/quiz-react";
import sheets from "../🧫️fixtures/🃏️sheet-assembly/🔣️.json";
import { CATALOG, MENAGERIE, PREFERENCES, Pets, fakeSource, fakeStage, motionPreference, named, settle, text } from "../🪛️pet-companions-kit/🟦️.tsx";

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

describe("🐾️ pet companions", () => {
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
