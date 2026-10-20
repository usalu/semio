/** 🔸️ The liveliness tests of the 🐾️ pet companions suite. */
import { render } from "@testing-library/react";
import uniq from "lodash/uniq";
import find from "lodash/find";
import { describe, expect, it } from "vitest";
import { PET_CHOICES, PET_HOME_SCENE, effectivePetMode, petScene, type PetChoice, type CourseStep } from "@semio-tech/course-react";
import { COURSE, Pets, RUNS, fakeSource, fakeStage, layer, motionPreference, settle, text } from "../../🪛️pet-companions-kit/🟦️.tsx";
import contexts from "../../../🧫️fixtures/🐾️pet-contexts/🔣️.json";

/** 🔹 Registers the liveliness tests of the 🐾️ pet companions suite. */
export function registerLivelinessTests(): void {
  describe("the liveliness", () => {
    it("is the learner's choice whatever the device asks for", () => {
      const table: readonly (readonly [choice: PetChoice, chosen: boolean, reducedMotion: boolean])[] = [
        ["off", false, false],
        ["off", false, true],
        ["off", true, false],
        ["off", true, true],
        ["still", false, false],
        ["still", false, true],
        ["still", true, false],
        ["still", true, true],
        ["calm", false, false],
        ["calm", false, true],
        ["calm", true, false],
        ["calm", true, true],
        ["lively", false, false],
        ["lively", false, true],
        ["lively", true, false],
        ["lively", true, true],
      ];
      expect(uniq(table.map(([choice, chosen, reducedMotion]) => `${choice} ${chosen} ${reducedMotion}`))).toHaveLength(PET_CHOICES.length * 2 * 2);
      expect(uniq(table.map(([choice]) => choice))).toEqual([...PET_CHOICES]);
      for (const [choice, chosen, reducedMotion] of table) expect(effectivePetMode(choice), `${choice}, chosen ${chosen}, reduced motion ${reducedMotion}`).toBe(choice);
    });

    it("keeps the learner's choice while the device's motion preference changes", async () => {
      const prefer = motionPreference(true);
      const source = fakeSource();
      const { stage, handed } = fakeStage();
      const { rerender } = render(<Pets source={source} stage={stage} choice="lively" />);
      await settle();
      expect(layer()?.dataset.mode).toBe("lively");
      prefer(false);
      expect(layer()?.dataset.mode).toBe("lively");
      prefer(true);
      expect(layer()?.dataset.mode).toBe("lively");
      expect(handed.every((props) => props.mode === "lively")).toBe(true);

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
    it("narrows the committed contexts identically to independent course lookups", () => {
      const course = { ...COURSE, topics: contexts.topics.map((id) => ({ id, emoji: "📚", title: text(id, id), quizzes: id === "energy" ? ["heating", "cooling"] : [] })) };
      for (const vector of contexts.cases) {
        const step = vector.step as CourseStep;
        const question = "question" in vector ? vector.question : undefined;
        expect(petScene(step, RUNS, course, question)).toBe(vector.expected);
        const quiz = step.screen === "run" || step.screen === "results" ? RUNS[step.run]?.quiz : step.screen === "home" ? find(course.quizzes, { id: step.page })?.id : undefined;
        const topic = step.screen === "home" ? find(course.topics, { id: step.page })?.id : undefined;
        const oracle = quiz === undefined ? topic === undefined ? "home" : `topic-${topic}` : step.screen === "run" && question !== undefined ? `question-${quiz}-${question}` : quiz;
        expect(petScene(step, RUNS, course, question)).toBe(oracle);
      }
    });

    it("is home wherever no quiz is on screen", () => {
      const steps: readonly CourseStep[] = [{ screen: "introduction" }, { screen: "identity" }, { screen: "home" }, { screen: "home", page: "learner" }, { screen: "home", page: "board" }, { screen: "home", page: "badges" }, { screen: "home", page: "prefs" }];
      for (const step of steps) expect(petScene(step, RUNS, COURSE), JSON.stringify(step)).toBe(PET_HOME_SCENE);
      expect(PET_HOME_SCENE).toBe("home");
    });

    it("is the quiz whose page is opened, that is being played or whose results show", () => {
      expect(petScene({ screen: "home", page: "heating" }, RUNS, COURSE)).toBe("heating");
      expect(petScene({ screen: "home", page: "cooling" }, RUNS, COURSE)).toBe("cooling");
      expect(petScene({ screen: "run", run: "r1" }, RUNS, COURSE)).toBe("heating");
      expect(petScene({ screen: "results", run: "r2" }, RUNS, COURSE)).toBe("cooling");
    });

    it("keeps unresolved pages and runs outside the general ensemble", () => {
      expect(petScene({ screen: "home", page: "heating" }, RUNS, undefined)).toBe("page-heating");
      expect(petScene({ screen: "home", page: "nowhere" }, RUNS, COURSE)).toBe("page-nowhere");
      expect(petScene({ screen: "run", run: "r9" }, RUNS, COURSE)).toBe("run-r9");
      expect(petScene({ screen: "results", run: "r9" }, {}, undefined)).toBe("run-r9");
    });
  });
}
