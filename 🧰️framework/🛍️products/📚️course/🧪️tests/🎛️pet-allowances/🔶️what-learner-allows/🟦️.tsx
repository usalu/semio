/** 🔶️ The what learner allows tests of the 🐾️ pet companions suite. */
import { fireEvent, render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { type ReactElement } from "react";
import { describe, expect, it, vi } from "vitest";
import { type PetLayerProps } from "@semio-tech/pets-react";
import { PreferencesPanel, COURSE_LOCALES, localStore, memoryStorageOrigin, courseText, readPreferences, switchedPets, withPets, writePreferences, type PetChoice, type CoursePreferences, type CourseStep } from "@semio-tech/course-react";
import { PREFERENCES, Pets, fakeSource, fakeStage, motionPreference, settle, text } from "../../🪛️pet-companions-kit/🟦️.tsx";

/** 🔷 Registers the what learner allows tests of the 🐾️ pet companions suite. */
export function registerWhatLearnerAllowsTests(): void {
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
}
