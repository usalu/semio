/** 🔶️ The pets stage tests of the 🐾️ pet companions suite. */
import { render, screen, within } from "@testing-library/react";
import uniq from "lodash/uniq";
import { type ReactElement } from "react";
import { describe, expect, it, vi } from "vitest";
import { PET_CHOICES, PreferencesPanel, COURSE_LOCALES, petNames, courseText, type PetChoice } from "@semio-tech/course-react";
import { MENAGERIE, PREFERENCES, Pets, REDUCED_MOTION, fakeSource, fakeStage, layer, motionPreference, settle } from "../../🪛️pet-companions-kit/🟦️.tsx";
import { SAMPLE } from "../🟠️harness/🟦️.tsx";

/** 🔷 Registers the pets stage tests of the 🐾️ pet companions suite. */
export function registerPetsStageTests(): void {
  describe("the pets on stage", () => {
    it("are named in the learner's language, in the order the layer names them, and nobody the menagerie does not know", () => {
      expect(petNames(MENAGERIE, ["windy", "sunny"], "en")).toEqual(["Windy", "Sunny"]);
      expect(petNames(MENAGERIE, ["flamy", "ghost", "housy"], "de")).toEqual(["Flammi", "Hausi"]);
      expect(petNames(MENAGERIE, [], "en")).toEqual([]);
      expect(petNames(SAMPLE, ["blobby", "hoppy", "floaty"], "de")).toEqual(["Blobby, der Klecks", "Hoppy, die Sprungfeder", "Floaty, der Ballon"]);
      for (const locale of COURSE_LOCALES) expect(petNames(SAMPLE, uniq(SAMPLE.casts.flatMap((cast) => [...cast.core, ...cast.rotation])), locale)).toEqual(uniq(SAMPLE.casts.flatMap((cast) => [...cast.core, ...cast.rotation])).map((id) => SAMPLE.species.find((entry) => entry.id === id)!.name[locale]));
    });

    it("are named under the choice as soon as the layer says who is there, in both languages, and no longer once they are gone", async () => {
      const lines = { en: { home: "Here right now: Sunny · Windy", heating: "Here right now: Flamy · Housy" }, de: { home: "Gerade hier: Sonni · Windi", heating: "Gerade hier: Flammi · Hausi" } };
      for (const locale of COURSE_LOCALES) {
        const source = fakeSource();
        const { stage } = fakeStage();
        const panel = <PreferencesPanel preferences={PREFERENCES} locale={locale} text={courseText(locale)} onChange={() => undefined} />;
        const { rerender, unmount } = render(
          <Pets source={source} stage={stage} choice="calm" locale={locale}>
            {panel}
          </Pets>,
        );
        expect(screen.queryByText(/ · /u)).toBeNull();
        await settle();
        const group = screen.getByRole("group", { name: courseText(locale)("course.preferences.pets") });
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
      render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={() => undefined} />);
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling).toBeNull();
    });

    it("keeps the learner's liveliness on a device that asks for reduced motion and does not say the device held them still", async () => {
      const notes = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };
      const prefer = motionPreference(true);
      for (const locale of COURSE_LOCALES) {
        prefer(true);
        const source = fakeSource();
        const { stage } = fakeStage();
        const pets = (choice: PetChoice, chosen: boolean): ReactElement => (
          <Pets source={source} stage={stage} choice={choice} chosen={chosen} locale={locale}>
            <PreferencesPanel preferences={{ ...PREFERENCES, pets: choice, petsChosen: chosen }} locale={locale} text={courseText(locale)} onChange={() => undefined} />
          </Pets>
        );
        const { rerender, unmount } = render(pets("lively", false));
        const group = screen.getByRole("group", { name: courseText(locale)("course.preferences.pets") });
        const pressed = (): PetChoice[] => PET_CHOICES.filter((_, index) => within(group).getAllByRole("button")[index]?.getAttribute("aria-pressed") === "true");
        expect(screen.queryByText(notes[locale])).toBeNull();
        expect(pressed()).toEqual(["lively"]);
        await settle();
        expect(layer()?.dataset.mode).toBe("lively");
        for (const choice of ["calm", "still", "off"] as const) {
          rerender(pets(choice, true));
          expect(pressed(), choice).toEqual([choice]);
          expect(screen.queryByText(notes[locale]), choice).toBeNull();
          if (choice === "off") expect(layer()).toBeNull();
          else expect(layer()?.dataset.mode, choice).toBe(choice);
        }
        prefer(false);
        rerender(pets("lively", false));
        expect(layer()?.dataset.mode).toBe("lively");
        expect(pressed()).toEqual(["lively"]);
        unmount();
      }
    });

    it("show nobody for a device that forces its own colours, whatever the learner chose, and the preferences say that instead of the word on motion, in both languages", async () => {
      const notes = { en: "Your device uses its own contrast colours, so no pets show.", de: "Dein Gerät verwendet eigene Kontrastfarben, deshalb erscheinen keine Tierchen." };
      const still = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };
      const answers = ["(forced-colors: active)", REDUCED_MOTION];
      vi.stubGlobal("matchMedia", (query: string) => ({ matches: answers.includes(query), media: query, onchange: null, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, dispatchEvent: () => false }));
      for (const locale of COURSE_LOCALES) {
        const panel = (choice: PetChoice, chosen = false): ReactElement => <PreferencesPanel preferences={{ ...PREFERENCES, pets: choice, petsChosen: chosen }} locale={locale} text={courseText(locale)} onChange={() => undefined} />;
        const source = fakeSource();
        const { stage } = fakeStage();
        const { rerender, unmount } = render(
          <Pets source={source} stage={stage} choice="calm" locale={locale}>
            {panel("calm")}
          </Pets>,
        );
        const group = screen.getByRole("group", { name: courseText(locale)("course.preferences.pets") });
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.tagName).toBe("P");
        expect(screen.queryByText(still[locale])).toBeNull();
        await settle();
        expect(layer()?.dataset.mode).toBe("calm");
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
          <PreferencesPanel preferences={PREFERENCES} locale="en" text={courseText("en")} onChange={() => undefined} />
        </Pets>,
      );
      expect(screen.queryByText(notes.en)).toBeNull();
    });
  });
}
