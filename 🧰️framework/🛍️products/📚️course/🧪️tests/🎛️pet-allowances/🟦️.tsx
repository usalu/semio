import { usePetQuestion } from "@semio-tech/course-react";
import find from "lodash/find";
import flatMap from "lodash/flatMap";
import uniq from "lodash/uniq";
import { type Menagerie } from "@semio-tech/pets";
import { type LearnerView } from "@semio-tech/course";
import { CoursePets, CoursePetsProvider, evolveCourseState, shinyPetSpecies, type CoursePetsSource } from "@semio-tech/course-react";
import { RUNS, at, species } from "../🪛️pet-companions-kit/🟦️.tsx";
/** 🎛️ Pet companions: what the learner allows, the hand and the topics, and play with the pets. */
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { type ReactElement } from "react";
import { describe, expect, it, vi } from "vitest";
import { DEEDS } from "@semio-tech/pets";
import { type PetLayerProps } from "@semio-tech/pets-react";
import type { CourseView, RunResult, RunView, Sheet, SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/course";
import { CardIcon, ClassificationTaskView, HomeScreen, PET_DEEDS, PET_DEED_LABELS, PET_DEED_SAID, PetTopic, PreferencesPanel, COURSE_LOCALES, COURSE_PET_CONTROLS, COURSE_PET_PROPS, CourseCard, initialCourseState, localStore, memoryStorageOrigin, petPlayers, petProp, courseText, readPreferences, switchedPets, withPets, writePreferences, type PetChoice, type CoursePreferences, type CourseSession, type CourseState, type CourseStep } from "@semio-tech/course-react";
import { RunScreen } from "@semio-tech/course-react/modules/run/.tsx";
import { ResultsScreen } from "@semio-tech/course-react/modules/results/.tsx";
import sheets from "../../🧫️fixtures/🃏️sheet-assembly/🔣️.json";
import { COURSE, MENAGERIE, PREFERENCES, Pets, fakeSource, fakeStage, motionPreference, named, settle, text } from "../🪛️pet-companions-kit/🟦️.tsx";
import shinyCases from "../../🧫️fixtures/✨️shiny-pets/🔣️.json";
import contexts from "../../🧫️fixtures/🐾️pet-contexts/🔣️.json";

const SHEET = sheets.sheets.find((vector) => vector.id === "icons-1")!.sheet as unknown as Sheet;

const OPEN_RUN: RunView = { run: "run-1", learner: "learner-1", quiz: SHEET.quiz, status: "open", sheet: SHEET, answers: {}, startedAt: 1_000_000 };

const HOME_COURSE: CourseView = {
  ...COURSE,
  quizzes: [
    { ...COURSE.quizzes[0]!, tasks: [{ id: "u-values", kind: "classification", title: text("U-values", "U-Werte") }, { id: "heating-load-and-demand", kind: "sorting", title: text("Heating load", "Heizlast") }] },
    { ...COURSE.quizzes[1]!, tasks: [{ id: "air-change-rates", kind: "matching", title: text("Air changes", "Luftwechsel") }] },
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

/** 🏠️ The overview of a learner who knows the course, on `step`. */
function homeAt(step: CourseStep): CourseState {
  return { ...initialCourseState({ introduced: true, learner: { id: "a".repeat(32), identity: { kind: "pseudonym", handle: "Ada" } }, course: HOME_COURSE, learnerView: undefined, runs: {} }), step };
}

/** 🛋️ A session double for the screens: everything it is asked does nothing and answers at once. */
function idleSession(): CourseSession {
  const done = async (): Promise<undefined> => undefined;
  return { now: () => 2_000_000, open: () => undefined, chooseBoard: () => undefined, startRun: done, resumeRun: done, loadRun: done, forgetLearner: () => undefined, refreshLeaderboard: async () => ({ answered: true }), refreshCrowd: done, openTask: done, answer: () => undefined, askCrowd: () => undefined, unaskCrowd: () => undefined, submit: done } as unknown as CourseSession;
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
        expect(readPreferences(localStore(origin.tab(), "arch"))).toMatchObject({ pets: "lively", petsChosen: false, petsPlay, petsMischief });
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
      for (const locale of COURSE_LOCALES) {
        const onChange = vi.fn();
        const panel = (preferences: CoursePreferences): ReactElement => <PreferencesPanel preferences={preferences} locale={locale} text={courseText(locale)} onChange={onChange} />;
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
      const pets = (choice: PetChoice, play: boolean, mischief: boolean, step?: CourseStep): ReactElement => <Pets source={source} stage={stage} choice={choice} chosen play={play} mischief={mischief} step={step} />;
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

    it("lets calm pets keep playing on a device that asks for reduced motion", async () => {
      motionPreference(true);
      const { stage, handed } = fakeStage();
      render(
        <Pets source={fakeSource()} stage={stage} choice="calm">
          <PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={() => undefined} />
        </Pets>,
      );
      await settle();
      expect(handed.at(-1)).toMatchObject({ mode: "calm", play: true, mischief: true });
      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).disabled).toBe(false);
      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).disabled).toBe(false);
      expect(screen.queryByText("Only calm and lively pets play.")).toBeNull();
      expect(screen.getByRole("group", { name: "Play with the pets" })).not.toBeNull();
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
        <div className="course-app">
          <main id="course-main">
            <CourseCard id="task" card="task" icon={<CardIcon icon="info" />} title="Climates">
              <PetTopic topic={petProp("geo", "climates")}>
                <ClassificationTaskView task={task} answer={undefined} onAnswer={() => undefined} text={courseText("en")} locale="en" />
              </PetTopic>
            </CourseCard>
          </main>
        </div>,
      );
      const grips = [...document.querySelectorAll<HTMLElement>("[data-course-grip]")];
      expect(grips).toHaveLength(2);
      for (const grip of grips) expect(grip.closest(COURSE_PET_CONTROLS)).toBe(grip);
      const zones = [...document.querySelectorAll<HTMLElement>("[data-course-drop]")];
      expect(zones.map((zone) => zone.dataset.courseDrop)).toEqual(["pool", "category:hot"]);
      for (const zone of zones) expect(zone.closest(COURSE_PET_CONTROLS)).toBe(zone);
      expect(screen.getByRole("heading", { name: "Climates" }).closest(COURSE_PET_CONTROLS)).toBeNull();
      expect(marked(COURSE_PET_PROPS)).toEqual([
        ["LI", "UL", "geo/climates/desert"],
        ["LI", "UL", "geo/climates/fjord"],
      ]);
      expect(petProp(undefined, "desert")).toBeUndefined();

      fireEvent.pointerDown(grips[0]!, { button: 0, clientX: 10, clientY: 10 });
      act(() => void window.dispatchEvent(new PointerEvent("pointermove", { clientX: 60, clientY: 40 })));
      const ghost = document.querySelector<HTMLElement>(".course-drag-ghost");
      expect(ghost?.querySelector<HTMLElement>("[data-pet-prop]")?.dataset.petProp).toBe("geo/climates/desert");
      expect(document.querySelectorAll("[data-pet-prop]")).toHaveLength(3);
      expect(marked(COURSE_PET_PROPS)).toEqual([
        ["LI", "UL", "geo/climates/desert"],
        ["LI", "UL", "geo/climates/fjord"],
      ]);
      act(() => void window.dispatchEvent(new PointerEvent("pointerup", { clientX: 60, clientY: 40 })));
      expect(document.querySelector(".course-drag-ghost")).toBeNull();
    });

    it("gives every quiz card of the overview its quiz as a topic it is never lifted for — a press on a card is the card's — and marks every task of an opened quiz's page with `<quiz>/<task>`", () => {
      const home = (step: CourseStep): ReactElement => <HomeScreen session={idleSession()} state={homeAt(step)} text={courseText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />;
      const { rerender } = render(home({ screen: "home" }));
      const cards = [...document.querySelectorAll<HTMLElement>("[data-pet-topic]")];
      expect(cards.map((card) => card.dataset.petTopic).sort()).toEqual(["cooling", "heating"]);
      for (const card of cards) {
        expect(card.dataset.card).toBe(`quiz:${card.dataset.petTopic}`);
        expect(card.hasAttribute("data-pet-prop")).toBe(false);
        expect(card.closest(COURSE_PET_CONTROLS)).not.toBeNull();
        expect(card.closest(COURSE_PET_CONTROLS)).toBe(card.closest("[data-layered-card]"));
        expect(card.querySelectorAll("[data-pet-prop]")).toHaveLength(0);
      }
      expect([...document.querySelectorAll(COURSE_PET_PROPS)].filter((element) => element.closest("[data-layered-card]") !== null)).toEqual([]);

      rerender(home({ screen: "home", page: "heating" }));
      const page = document.querySelector<HTMLElement>('[data-page="quiz:heating"]')!;
      const rows = [...page.querySelectorAll<HTMLElement>(COURSE_PET_PROPS)];
      expect(rows.map((row) => [row.tagName, row.dataset.petProp, row.closest<HTMLElement>("[data-card]")?.dataset.card])).toEqual([
        ["LI", "heating/u-values", "course-tasks"],
        ["LI", "heating/heating-load-and-demand", "course-tasks"],
      ]);
      expect(rows.map((row) => row.textContent)).toEqual([expect.stringContaining("U-values"), expect.stringContaining("Heating load")]);
      expect(page.querySelector("[data-pet-topic]")).toBeNull();
    });

    it("marks the items of a run that are no table rows with `<quiz>/<task>/<item>` — a classification's chips, a matching's item rows (never its cards), a sorting's rows — and nothing else of the run", () => {
      const [sources, appliances, masses] = SHEET.tasks as readonly [SheetClassificationTask, SheetMatchingTask, SheetSortingTask];
      render(<RunScreen session={idleSession()} state={initialCourseState({ introduced: true, runs: { [OPEN_RUN.run]: OPEN_RUN } })} run={OPEN_RUN.run} text={courseText("en")} locale="en" />);
      const steps = within(screen.getByRole("navigation", { name: "Tasks" })).getAllByRole("button");
      expect(marked()).toEqual(sources.items.map((item) => ["LI", "UL", `${SHEET.quiz}/${sources.id}/${item.id}`]));
      for (const chip of document.querySelectorAll<HTMLElement>("[data-pet-prop]")) expect(chip.dataset.petProp).toBe(`${SHEET.quiz}/${sources.id}/${chip.dataset.courseItem}`);
      fireEvent.click(steps[1]!);
      expect(appliances.dimensions).toHaveLength(2);
      expect(marked()).toEqual(appliances.dimensions.flatMap(() => appliances.items.map((item) => ["LI", "UL", `${SHEET.quiz}/${appliances.id}/${item.id}`])));
      const valueCards = [...document.querySelectorAll("li[data-course-drag]")];
      expect(valueCards.length).toBeGreaterThan(0);
      for (const card of valueCards) expect(card.hasAttribute("data-pet-prop")).toBe(false);
      fireEvent.click(steps[2]!);
      expect(marked()).toEqual(masses.items.map((item) => ["LI", "OL", `${SHEET.quiz}/${masses.id}/${item.id}`]));
      expect(document.querySelectorAll("section[data-pet-prop], section[data-pet-topic], table [data-pet-prop], [data-pet-prop] [data-pet-prop]")).toHaveLength(0);
      expect(marked(COURSE_PET_PROPS)).toEqual(marked());
    });

    it("marks the true order of a sorting's results with `<quiz>/<task>/<item>`, never a row of a table", () => {
      const view = submittedRun();
      const masses = SHEET.tasks[2] as SheetSortingTask;
      render(<ResultsScreen session={idleSession()} state={initialCourseState({ introduced: true, runs: { [view.run]: view } })} run={view.run} others="never" text={courseText("en")} locale="en" />);
      expect(marked()).toEqual([...masses.items].reverse().map((item) => ["LI", "OL", `${SHEET.quiz}/${masses.id}/${item.id}`]));
      expect(document.querySelectorAll("tr").length).toBeGreaterThan(0);
      expect(document.querySelectorAll("table [data-pet-prop], tr[data-pet-prop], [data-pet-topic]")).toHaveLength(0);
      expect(marked(COURSE_PET_PROPS)).toEqual(marked());
    });
  });

  describe("play with the pets", () => {
    it("offers the stage's own deeds, worded differently in both languages", () => {
      expect([...PET_DEEDS]).toEqual([...DEEDS]);
      expect(Object.keys(PET_DEED_LABELS)).toEqual([...PET_DEEDS]);
      expect(Object.keys(PET_DEED_SAID)).toEqual([...PET_DEEDS]);
      const words = { en: ["Hello", "Trick", "Pet", "Toss", "Circle clockwise", "Circle counterclockwise", "Stroke", "Shake"], de: ["Hallo", "Kunststück", "Streicheln", "Hochwerfen", "Im Uhrzeigersinn kreisen", "Gegen den Uhrzeigersinn kreisen", "Streichen", "Schütteln"] };
      for (const locale of COURSE_LOCALES) expect(PET_DEEDS.map((deed) => courseText(locale)(PET_DEED_LABELS[deed]))).toEqual(words[locale]);
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
      for (const locale of COURSE_LOCALES) {
        const { stage, played } = fakeStage();
        const { unmount } = render(
          <Pets source={fakeSource()} stage={stage} choice="calm" locale={locale}>
            <PreferencesPanel preferences={PREFERENCES} locale={locale} text={courseText(locale)} onChange={() => undefined} />
          </Pets>,
        );
        expect(screen.queryByRole("group", { name: said[locale].group })).toBeNull();
        await settle();
        const play = screen.getByRole("group", { name: said[locale].group });
        expect(within(play).getAllByRole("group").map(named)).toEqual(said[locale].names);
        const [sunny, windy] = said[locale].names.map((name) => within(play).getByRole("group", { name }));
        for (const pet of [sunny!, windy!]) expect(within(pet).getAllByRole("button").map((button) => button.textContent)).toEqual(PET_DEEDS.map((deed) => courseText(locale)(PET_DEED_LABELS[deed])));
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
        for (const deed of ["circle", "countercircle", "stroke", "shake"] as const) {
          const button = within(windy!).getByRole("button", { name: courseText(locale)(PET_DEED_LABELS[deed]) });
          button.focus();
          await user.keyboard("{Enter}");
          await user.keyboard(" ");
          expect(document.activeElement).toBe(button);
          expect(played.slice(-2)).toEqual([`windy ${deed}`, `windy ${deed}`]);
          expect(status.textContent).toBe(courseText(locale)(PET_DEED_SAID[deed], { name: said[locale].names[1]! }));
        }
        unmount();
      }
    });

    it("is there only while the pets are calm or lively, play is allowed, somebody is on stage and the site has pets", async () => {
      const source = fakeSource();
      const { stage } = fakeStage();
      const panel = <PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={() => undefined} />;
      const group = (): HTMLElement | null => screen.queryByRole("group", { name: "Play with the pets" });
      const { rerender, unmount } = render(
        <Pets source={source} stage={stage} choice="calm">
          {panel}
        </Pets>,
      );
      await settle();
      expect(group()).not.toBeNull();
      const cases: readonly [props: { readonly choice: PetChoice; readonly play?: boolean; readonly mischief?: boolean; readonly step?: CourseStep }, names: readonly string[] | null][] = [
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


describe("badge-driven shiny pets", () => {
  const menagerie: Menagerie = { ...MENAGERIE, species: [...MENAGERIE.species, species("chilly", text("Chilly", "Frosti"))], casts: shinyCases.casts };
  const course: CourseView = { ...COURSE, quizzes: shinyCases.quizzes.map(quiz => ({ ...COURSE.quizzes[0], ...quiz, tasks: quiz.tasks.map(task => ({ ...task, kind: "sorting", title: text(task.id, task.id) })) })) };
  const learner = (badges: LearnerView["badges"]): LearnerView => ({ learner: "a", identity: { kind: "anonymous" }, runs: shinyCases.runs.map(run => ({ ...run, challenge: "easy", status: "submitted", startedAt: 0 })), badges, best: {}, total: 0 });
  for (const example of shinyCases.cases) {
    it(example.name, () => {
      const learnerView = learner(example.badges);
      const awards = (example.awards ?? {}) as CourseState["awards"];
      const localRuns = shinyCases.runs.map(run => ({ ...run, learner: example.localLearner ?? "a" })) as unknown as RunView[];
      const eligible = uniq([...example.badges.map(award => award.run), ...Object.entries(awards).filter(([, badges]) => badges.length > 0).map(([run]) => run)]);
      const scenes = uniq(flatMap(course.quizzes.filter(quiz => eligible.some(id => find(localRuns, { run: id, learner: "a" })?.quiz === quiz.id)), quiz => [quiz.id, ...quiz.tasks.map(task => `question-${quiz.id}-${task.id}`)]));
      const reference = uniq(flatMap(shinyCases.casts.filter(cast => scenes.includes(cast.scene)), cast => [...cast.core, ...cast.rotation]));
      expect(menagerie.species.filter(entry => reference.includes(entry.id)).map(entry => entry.id)).toEqual(example.shiny);
      expect(shinyPetSpecies(menagerie, { course, learnerView, learner: { id: "a" }, awards, runs: Object.fromEntries(localRuns.map(run => [run.run, run])) })).toEqual(example.shiny);
    });
  }
  it("hands updated awards to the layer and clears them when learners switch", async () => {
    const source: CoursePetsSource = async () => menagerie;
    const { stage, handed } = fakeStage();
    const earned = shinyCases.cases.find(example => example.name === "core, rotation and question pets")!;
    const show = (learnerView?: LearnerView): ReactElement => <CoursePetsProvider source={source} stage={stage} choice="still" chosen play mischief locale="en" state={{ ...at({ screen: "home" }), course, learnerView, learner: learnerView === undefined ? undefined : { id: learnerView.learner } }}><CoursePets /></CoursePetsProvider>;
    const view = render(show());
    await settle();
    expect(handed.at(-1)?.shiny).toEqual([]);
    view.rerender(show(learner(earned.badges)));
    expect(handed.at(-1)?.shiny).toEqual(earned.shiny);
    view.rerender(show(learner([])));
    expect(handed.at(-1)?.shiny).toEqual([]);
    view.rerender(show());
    expect(handed.at(-1)?.shiny).toEqual([]);
    expect(stage).toHaveBeenCalledTimes(1);
  });
  it("shines on the submission event before the learner view refreshes", async () => {
    const source: CoursePetsSource = async () => menagerie;
    const { stage, handed } = fakeStage();
    const current: CourseState = { ...initialCourseState({ introduced: true, runs: {} }), course, learner: { id: "a" }, runs: { r1: { ...RUNS.r1, quiz: "heat", learner: "a" } } };
    const show = (state: CourseState): ReactElement => <CoursePetsProvider source={source} stage={stage} choice="still" chosen play mischief locale="de" state={state}><CoursePets /></CoursePetsProvider>;
    const view = render(show(current));
    await settle();
    expect(handed.at(-1)?.shiny).toEqual([]);
    const submitted = evolveCourseState(current, { type: "run-submitted", run: "r1", result: {} as RunResult, badges: ["warm"], at: 1 });
    view.rerender(show(submitted));
    expect(handed.at(-1)?.shiny).toEqual(["sunny", "housy", "flamy"]);
    view.rerender(show(evolveCourseState(submitted, { type: "learner-forgotten" })));
    expect(handed.at(-1)?.shiny).toEqual([]);
  });
});

describe("pet question ownership", () => {
    it("restores a mounted owner's question after a followed question leaves", async () => {
      function Question(props: { run: string; task: string }): null {
        usePetQuestion(props.run, props.task);
        return null;
      }
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const tree = (run: string, following: boolean): ReactElement => <Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run }}><><Question key="own" run="r1" task="systems" />{following ? <Question key="followed" run="r2" task="air" /> : null}</></Pets>;
      const first = contexts.follow[0]!;
      const { rerender } = render(tree(first.run, first.following));
      await settle();
      for (const vector of contexts.follow) {
        rerender(tree(vector.run, vector.following));
        expect(handed.at(-1)?.scene).toBe(vector.expected);
        const oracle = find(contexts.cases, (candidate) => candidate.step.screen === "run" && "run" in candidate.step && candidate.step.run === vector.run && "question" in candidate && candidate.question !== undefined);
        expect(handed.at(-1)?.scene).toBe(oracle?.expected);
      }
    });
    it("tracks the actual question tabs of a run and returns to quiz scope on results", async () => {
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const state = { ...homeAt({ screen: "run", run: OPEN_RUN.run }), runs: { [OPEN_RUN.run]: OPEN_RUN } };
      const tree = (current: CourseState): ReactElement => <CoursePetsProvider source={source} stage={stage} choice="lively" chosen={true} play={true} mischief={true} state={current} locale="en">
        {current.step.screen === "run" ? <RunScreen session={idleSession()} state={current} run={OPEN_RUN.run} text={courseText("en")} locale="en" /> : null}
        <CoursePets />
      </CoursePetsProvider>;
      const { rerender } = render(tree(state));
      await settle();
      expect(handed.at(-1)).toMatchObject({ scene: `question-${OPEN_RUN.quiz}-${SHEET.tasks[0]!.id}`, quiet: true });
      for (const task of SHEET.tasks.slice(1)) {
        fireEvent.click(screen.getByRole("button", { name: new RegExp(task.title.en) }));
        expect(handed.at(-1)).toMatchObject({ scene: `question-${OPEN_RUN.quiz}-${task.id}`, quiet: true });
      }
      rerender(tree({ ...state, step: { screen: "results", run: OPEN_RUN.run } }));
      expect(handed.at(-1)).toMatchObject({ scene: OPEN_RUN.quiz, quiet: false });
      rerender(tree({ ...state, step: { screen: "home" } }));
      expect(handed.at(-1)).toMatchObject({ scene: "home", quiet: false });
      rerender(tree(state));
      expect(handed.at(-1)).toMatchObject({ scene: `question-${OPEN_RUN.quiz}-${SHEET.tasks[0]!.id}`, quiet: true });
    });

    it("ignores a question reporter belonging to another run and clears it when it leaves", async () => {
      function Question(props: { run: string; task: string }): null {
        usePetQuestion(props.run, props.task);
        return null;
      }
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const { rerender } = render(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r1" }}><Question run="r2" task="air" /></Pets>);
      await settle();
      expect(handed.at(-1)?.scene).toBe("heating");
      rerender(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r1" }}><Question run="r1" task="systems" /></Pets>);
      expect(handed.at(-1)?.scene).toBe("question-heating-systems");
      rerender(<Pets source={source} stage={stage} choice="calm" step={{ screen: "run", run: "r1" }} />);
      expect(handed.at(-1)?.scene).toBe("heating");
    });

});
