/** 🟠️ The layer tests of the 🐾️ pet companions suite. */
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { type Menagerie } from "@semio-tech/pets";
import { PET_KEEPOUTS } from "@semio-tech/pets-react";
import type { SheetClassificationTask } from "@semio-tech/course";
import { CardIcon, ClassificationTaskView, EMPTY_PRESENCE_VIEW, PresenceOverlay, COURSE_PET_CONTROLS, COURSE_PET_KEEPOUTS, COURSE_PET_PROPS, COURSE_PET_SURFACES, CourseCard, COURSE_PETS_TEMPO, peerGlances, petsTempo, courseText, type PeerCursor, type CoursePetsSource, type CoursePetsStage } from "@semio-tech/course-react";
import { MENAGERIE, Pets, fakeSource, fakeStage, layer, settle, text } from "../../🪛️pet-companions-kit/🟦️.tsx";
import { boxed, quiet } from "../🟢️harness/🟦️.tsx";

/** 🟡 Registers the layer tests of the 🐾️ pet companions suite. */
export function registerLayerTests(): void {
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
        shiny: [],
        surfaces: COURSE_PET_SURFACES,
        keepouts: `button, p, ${COURSE_PET_KEEPOUTS}`,
        controls: COURSE_PET_CONTROLS,
        props: COURSE_PET_PROPS,
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
      const source = vi.fn<CoursePetsSource>().mockRejectedValueOnce(new Error("offline")).mockResolvedValue(MENAGERIE);
      const { stage } = fakeStage();
      const { rerender, container } = render(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      expect(container.innerHTML).toBe("<p>quiz</p>");
      rerender(<Pets source={source} stage={stage} choice="off" />);
      rerender(<Pets source={source} stage={stage} choice="calm" />);
      await settle();
      expect(source).toHaveBeenCalledTimes(2);
      expect(layer()).not.toBeNull();

      const broken = vi.fn<CoursePetsStage>(() => {
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
      const source = vi.fn<CoursePetsSource>(() => new Promise((resolve) => (deliver = resolve)));
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
      const stage: CoursePetsStage = async () => ({
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
        <div className="course-app">
          <header>
            <span>Quizzes</span>
          </header>
          <main id="course-main">
            <CourseCard id="task" card="task" icon={<CardIcon icon="info" />} title="Climates" footerRight={<button type="button">Next</button>}>
              <ClassificationTaskView task={task} answer={undefined} onAnswer={() => undefined} text={courseText("en")} locale="en" />
            </CourseCard>
          </main>
          <footer className="course-footer" />
          <CourseCard id="dialog" card="dialog" icon={<CardIcon icon="info" />} title="Submit?" />
        </div>,
      );
      const body = document.createElement("div");
      body.dataset.slot = "window-chrome-body-surface";
      document.querySelector('#course-main [data-slot="window-chrome-chip-cap"]')!.parentElement!.after(body);
      const surfaces = [...document.querySelectorAll<HTMLElement>(COURSE_PET_SURFACES)];
      expect(surfaces.map((part) => [part.dataset.slot ?? part.tagName, part.closest<HTMLElement>("[data-card]")?.dataset.card])).toEqual([
        ["window-chrome-chip-cap", "task"],
        ["window-chrome-body-surface", "task"],
        ["FOOTER", undefined],
      ]);
      expect(surfaces[0]!.textContent?.trim()).toBe("Climates");
      expect([surfaces[0]!.dataset.dock, surfaces[0]!.hasAttribute("data-window-silhouette-chip")]).toEqual(["top", true]);
      expect(surfaces[0]!.querySelector("button")).toBeNull();
      const clear = [...document.querySelectorAll<HTMLElement>(COURSE_PET_KEEPOUTS)];
      expect(clear.filter((element) => element.tagName === "HEADER")).toEqual([document.querySelector(".course-app > header")]);
      expect(clear.filter((element) => element.dataset.courseItem !== undefined).map((element) => element.dataset.courseItem)).toEqual(["desert"]);
      expect(clear.filter((element) => element.dataset.courseDrop !== undefined).map((element) => element.dataset.courseDrop)).toEqual(["pool", "category:hot"]);
      expect([...document.querySelectorAll(`${PET_KEEPOUTS}, ${COURSE_PET_KEEPOUTS}`)]).toEqual(expect.arrayContaining([...clear, screen.getByRole("combobox", { name: "Category for Desert" })]));
    });

    it("lives in real time unless the document root names another tempo: the seam of the end-to-end proof of an encounter", async () => {
      expect(COURSE_PETS_TEMPO).toBe("data-pets-tempo");
      expect(petsTempo()).toBe(1);
      try {
        for (const [said, tempo] of [["8", 8], ["0.5", 0.5], ["fast", 1], ["0", 1], ["-2", 1], ["Infinity", 1]] as const) {
          document.documentElement.setAttribute(COURSE_PETS_TEMPO, said);
          expect(petsTempo(), said).toBe(tempo);
        }
        document.documentElement.setAttribute(COURSE_PETS_TEMPO, "8");
        const { stage, handed } = fakeStage();
        render(<Pets source={fakeSource()} stage={stage} choice="lively" />);
        await settle();
        expect(handed.at(-1)).toMatchObject({ mode: "lively", tempo: 8 });
      } finally {
        document.documentElement.removeAttribute(COURSE_PETS_TEMPO);
      }
    });

    it("glances at the tips of the other learners' cursors while they show", () => {
      expect(peerGlances()).toEqual([]);
      const peers: readonly PeerCursor[] = [
        { session: "w1", tag: "0badc0de", label: "Mira K.", colour: 3, place: { screen: "home" }, home: undefined, cursor: { anchor: "run", x: 0.5, y: 0.5 }, focus: undefined, drag: undefined },
        { session: "w2", tag: "0badc0df", label: "Ben", colour: 4, place: { screen: "home" }, home: undefined, cursor: { anchor: "elsewhere", x: 0.1, y: 0.1 }, focus: "run", drag: undefined },
      ];
      const { rerender } = render(
        <>
          <div data-presence-anchor="run" ref={(element) => { if (element !== null) boxed(element, 0, 0); }} />
          <PresenceOverlay view={{ ...EMPTY_PRESENCE_VIEW, peers }} show />
        </>,
      );
      const marks = [...document.querySelectorAll<HTMLElement>("[data-presence-layer] .course-peer")];
      expect(marks.map((mark) => mark.hidden)).toEqual([false, true]);
      boxed(marks[0]!, 320, 180);
      boxed(marks[1]!, 0, 0);
      expect(peerGlances()).toEqual([{ x: 320, y: 180 }]);
      rerender(
        <>
          <div data-presence-anchor="run" ref={(element) => { if (element !== null) boxed(element, 0, 0); }} />
          <PresenceOverlay view={{ ...EMPTY_PRESENCE_VIEW, peers }} show={false} />
        </>,
      );
      expect(peerGlances()).toEqual([]);
    });
  });
}
