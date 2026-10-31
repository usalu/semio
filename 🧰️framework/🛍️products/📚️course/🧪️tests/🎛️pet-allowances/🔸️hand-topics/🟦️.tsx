import { JSDOM } from "jsdom";
import taskTopics from "../../../🧫️fixtures/🐾️pet-contexts/📋️task-topics/🔣️.json";
/** 🔸️ The hand topics tests of the 🐾️ pet companions suite. */
import { act, fireEvent, render, screen, within } from "@testing-library/react";
import { type ReactElement } from "react";
import { describe, expect, it } from "vitest";
import type { SheetClassificationTask, SheetMatchingTask, SheetSortingTask } from "@semio-tech/course";
import { CardIcon, ClassificationTaskView, HomeScreen, PetTopic, COURSE_PET_CONTROLS, COURSE_PET_PROPS, CourseCard, initialCourseState, petProp, courseText, type CourseStep } from "@semio-tech/course-react";
import { RunScreen } from "@semio-tech/course-react/modules/run/.tsx";
import { ResultsScreen } from "@semio-tech/course-react/modules/results/.tsx";
import { PREFERENCES, text } from "../../🪛️pet-companions-kit/🟦️.tsx";
import { OPEN_RUN, SHEET, homeAt, idleSession, marked, submittedRun } from "../🟢️harness/🟦️.tsx";

/** 🔹 Registers the hand topics tests of the 🐾️ pet companions suite. */
export function registerHandTopicsTests(): void {
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

      expect({version:taskTopics.version,markerTag:taskTopics.markerTag,rowTag:taskTopics.rowTag,listTag:taskTopics.listTag,card:taskTopics.card,props:taskTopics.rows.map(row=>row.prop),cases:taskTopics.cases}).toEqual({version:1,markerTag:"DIV",rowTag:"LI",listTag:"OL",card:"course-tasks",props:["heating/u-values","heating/heating-load-and-demand"],cases:[{id:"open",locked:false},{id:"first-task-locked",locked:true}]});
      for (const vector of taskTopics.cases) {
        const state = homeAt({ screen: "home", page: "heating" });
        const course = vector.locked ? { ...state.course!, unlocks: { ...state.course!.unlocks, [taskTopics.rows[0]!.prop]: null } } : state.course;
        rerender(<HomeScreen session={idleSession()} state={{ ...state, course }} text={courseText("en")} locale="en" preferences={PREFERENCES} onPreferences={() => undefined} />);
        const page = [...document.querySelectorAll<HTMLElement>("[data-page]")].find((row) => row.dataset.page === "quiz:heating")!;
        const list = page.querySelector<HTMLOListElement>("[data-card=course-tasks] ol[role=list]")!;
        const rows = [...page.querySelectorAll<HTMLElement>(COURSE_PET_PROPS)];
        expect(rows.map((row) => [row.tagName, row.dataset.petProp, row.closest<HTMLElement>("[data-card]")?.dataset.card])).toEqual(taskTopics.rows.map((row) => [taskTopics.markerTag, row.prop, taskTopics.card]));
        expect(list.tagName).toBe(taskTopics.listTag);
        expect(within(list).getAllByRole("listitem")).toEqual([...list.children]);
        expect([...list.children].map((row) => row.tagName)).toEqual(taskTopics.rows.map(() => taskTopics.rowTag));
        expect(rows.map((row) => row.closest("li")?.parentElement === list)).toEqual(taskTopics.rows.map(() => true));
        expect(rows.map((row) => row.closest("li")?.hasAttribute("data-pet-prop"))).toEqual(taskTopics.rows.map(() => false));
        expect(rows.map((row) => row.textContent)).toEqual(taskTopics.rows.map((row) => expect.stringContaining(row.label)));
        for (const [index, row] of rows.entries()) expect(row.closest("[inert][aria-hidden=true]") !== null).toBe(vector.locked && index === 0);
        const oracle = new JSDOM(page.outerHTML);
        try {
          expect([...oracle.window.document.querySelectorAll<HTMLElement>(COURSE_PET_PROPS)].map((row) => [row.tagName, row.dataset.petProp, row.closest("li")?.tagName, row.closest("li")?.parentElement?.tagName])).toEqual(taskTopics.rows.map((row) => [taskTopics.markerTag, row.prop, taskTopics.rowTag, taskTopics.listTag]));
        } finally { oracle.window.close(); }
        expect(page.querySelector("[data-pet-topic]")).toBeNull();
      }
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
}
