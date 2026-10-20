/** 🟠️ The play pets tests of the 🐾️ pet companions suite. */
import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { DEEDS } from "@semio-tech/pets";
import { PET_DEEDS, PET_DEED_LABELS, PET_DEED_SAID, PreferencesPanel, COURSE_LOCALES, petPlayers, courseText, type PetChoice, type CourseStep } from "@semio-tech/course-react";
import { MENAGERIE, PREFERENCES, Pets, fakeSource, fakeStage, named, settle, text } from "../../🪛️pet-companions-kit/🟦️.tsx";

/** 🟡 Registers the play pets tests of the 🐾️ pet companions suite. */
export function registerPlayPetsTests(): void {
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
}
