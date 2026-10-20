import { usePetQuestion } from "@semio-tech/course-react";
import find from "lodash/find";
import flatMap from "lodash/flatMap";
import uniq from "lodash/uniq";
import { type Menagerie } from "@semio-tech/pets";
import { type LearnerView } from "@semio-tech/course";
import { CoursePets, CoursePetsProvider, evolveCourseState, shinyPetSpecies, type CoursePetsSource } from "@semio-tech/course-react";
import { RUNS, at, species } from "../🪛️pet-companions-kit/🟦️.tsx";
/** 🎛️ Pet companions: what the learner allows, the hand and the topics, and play with the pets. */
import { fireEvent, render, screen } from "@testing-library/react";
import { type ReactElement } from "react";
import { describe, expect, it } from "vitest";
import type { CourseView, RunResult, RunView } from "@semio-tech/course";
import { initialCourseState, courseText, type CourseState } from "@semio-tech/course-react";
import { RunScreen } from "@semio-tech/course-react/modules/run/.tsx";
import { COURSE, MENAGERIE, Pets, fakeSource, fakeStage, settle, text } from "../🪛️pet-companions-kit/🟦️.tsx";
import shinyCases from "../../🧫️fixtures/✨️shiny-pets/🔣️.json";
import contexts from "../../🧫️fixtures/🐾️pet-contexts/🔣️.json";
import { SHEET, OPEN_RUN, homeAt, idleSession } from "./🟢️shared/🟦️.tsx";
import { registerWhatLearnerAllowsTests } from "./🔶️what-learner-allows/🟦️.tsx";
import { registerHandTopicsTests } from "./🔸️hand-topics/🟦️.tsx";
import { registerPlayPetsTests } from "./🟠️play-pets/🟦️.tsx";

describe("🐾️ pet companions", () => {
  registerWhatLearnerAllowsTests();
  registerHandTopicsTests();
  registerPlayPetsTests();
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
