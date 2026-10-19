/** 🐾️ Pet companions: the learner's choice between no pets, motionless, calm and lively ones is lively until they
 * choose otherwise, kept on the device and offered in both languages, and a switch on the footer of every screen hides
 * the pets and brings them back as lively as they were; a device that asks for reduced motion does not hold them still,
 * and what a learner chose in the preferences or with the switch is stored as a fact of its own;
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
import { fireEvent, render, screen, within } from "@testing-library/react";
import uniq from "lodash/uniq";
import find from "lodash/find";
import { describe, expect, it, vi } from "vitest";
import { type Menagerie } from "@semio-tech/pets";
import { PET_KEEPOUTS } from "@semio-tech/pets-react";
import type { SheetClassificationTask } from "@semio-tech/course";
import { CardIcon, ClassificationTaskView, EMPTY_PRESENCE_VIEW, PET_CHOICES, PET_CHOICE_LABELS, PET_HOME_SCENE, PetsSwitch, PreferencesPanel, PresenceOverlay, COURSE_LOCALES, COURSE_PET_CONTROLS, COURSE_PET_KEEPOUTS, COURSE_PET_PROPS, COURSE_PET_SURFACES, CourseCard, CoursePetsProvider, effectivePetMode, localStore, memoryStorageOrigin, COURSE_PETS_TEMPO, peerGlances, petScene, petsTempo, courseText, readPreferences, switchedPets, withPets, writePreferences, type PeerCursor, type PetChoice, type CoursePetsSource, type CoursePetsStage, type CourseStep } from "@semio-tech/course-react";
import { COURSE, MENAGERIE, PREFERENCES, Pets, RUNS, at, fakeSource, fakeStage, layer, motionPreference, settle, text } from "../🪛️pet-companions-kit/🟦️.tsx";
import contexts from "../../🧫️fixtures/🐾️pet-contexts/🔣️.json";

function quiet(): readonly ReturnType<typeof vi.spyOn>[] {
  return (["log", "info", "warn", "error", "debug"] as const).map((level) => vi.spyOn(console, level).mockImplementation(() => undefined));
}

function boxed(element: Element, left: number, top: number): void {
  element.getBoundingClientRect = () => ({ left, top, right: left + 16, bottom: top + 16, width: 16, height: 16, x: left, y: top, toJSON: () => ({}) });
}

describe("🐾️ pet companions", () => {
  describe("the learner's choice", () => {
    it("is lively until the learner chooses otherwise, and stays on the device", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store).pets).toBe("lively");
      for (const choice of PET_CHOICES) {
        writePreferences(store, { ...readPreferences(store), pets: choice });
        expect(readPreferences(localStore(origin.tab(), "arch")).pets).toBe(choice);
      }
      store.write("preferences", { theme: "dark", pets: "wild" });
      expect(readPreferences(store)).toMatchObject({ theme: "dark", pets: "lively", petsLiveliness: "lively" });
    });

    it("remembers how lively the pets were while they are off, so the switch brings them back as they were", () => {
      const store = localStore(memoryStorageOrigin().tab(), "arch");
      expect(readPreferences(store).petsLiveliness).toBe("lively");
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
      expect(readPreferences(store)).toMatchObject({ pets: "off", petsLiveliness: "lively" });
      store.write("preferences", { pets: "lively", petsLiveliness: "still" });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively" });
    });

    it("knows whether the learner chose: a fact of its own that stays on the device, that only a choice for the pets states and that preferences stored without it do not carry", () => {
      const origin = memoryStorageOrigin();
      const store = localStore(origin.tab(), "arch");
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: false });
      writePreferences(store, { ...readPreferences(store), locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false });
      expect(readPreferences(localStore(origin.tab(), "arch"))).toEqual({ locale: "de", theme: "dark", textSize: "large", showCursors: false, others: "always", animateIcons: false, iconsChosen: false, pets: "lively", petsLiveliness: "lively", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} });
      for (const choice of PET_CHOICES) {
        const device = memoryStorageOrigin();
        const fresh = localStore(device.tab(), "arch");
        writePreferences(fresh, withPets(readPreferences(fresh), choice));
        expect(readPreferences(localStore(device.tab(), "arch"))).toMatchObject({ pets: choice, petsChosen: true });
        writePreferences(fresh, { ...readPreferences(fresh), theme: "light", animateIcons: false });
        expect(readPreferences(localStore(device.tab(), "arch"))).toMatchObject({ theme: "light", pets: choice, petsChosen: true });
      }
      const untouched = readPreferences(localStore(memoryStorageOrigin().tab(), "arch"));
      expect(withPets(untouched, switchedPets(false, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "off", petsLiveliness: "lively", petsChosen: true });
      expect(withPets(untouched, switchedPets(true, untouched.petsLiveliness))).toEqual({ ...untouched, pets: "lively", petsLiveliness: "lively", petsChosen: true });
      const before: readonly Record<string, unknown>[] = [{ pets: "lively", petsLiveliness: "lively" }, { pets: "off", petsLiveliness: "still" }, { theme: "dark" }, { pets: "still", petsChosen: "true" }, { pets: "calm", petsChosen: 1 }, { pets: "calm", petsChosen: null }];
      for (const stored of before) {
        store.write("preferences", stored);
        expect(readPreferences(store).petsChosen, JSON.stringify(stored)).toBe(false);
      }
      store.write("preferences", { pets: "lively", petsChosen: true });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: true });
      store.write("preferences", { pets: "wild", petsChosen: true });
      expect(readPreferences(store)).toMatchObject({ pets: "lively", petsChosen: true });
    });

    it("is the learner's own only through the pets' row: no other control of the preferences states it", () => {
      const onChange = vi.fn();
      render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={onChange} />);
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
      for (const locale of COURSE_LOCALES) {
        const onChange = vi.fn();
        const { unmount } = render(<PreferencesPanel preferences={PREFERENCES} locale={locale} text={courseText(locale)} onChange={onChange} />);
        const buttons = within(screen.getByRole("group", { name: labels[locale].group })).getAllByRole("button");
        expect(buttons.map((button) => button.textContent?.trim())).toEqual(labels[locale].choices);
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
      for (const locale of COURSE_LOCALES) {
        expect(courseText(locale)("course.preferences.petsShown")).toBe(names[locale]);
        for (const shown of [true, false]) {
          const onChange = vi.fn();
          const { unmount } = render(
            <CoursePetsProvider source={fakeSource()} stage={fakeStage().stage} choice="off" chosen={false} play mischief state={at({ screen: "home" })} locale={locale}>
              <PetsSwitch shown={shown} label={names[locale]} onChange={onChange} />
            </CoursePetsProvider>,
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
        <CoursePetsProvider source={undefined} choice="calm" chosen={false} play mischief state={at({ screen: "home" })} locale="en">
          <PetsSwitch shown label="Show pets" onChange={() => undefined} />
        </CoursePetsProvider>,
      );
      expect(container.innerHTML).toBe("");
      expect(render(<PetsSwitch shown label="Show pets" onChange={() => undefined} />).container.innerHTML).toBe("");
    });
  });

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
});
