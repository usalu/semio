/** 🔸️ The client tests of the 🐾️ pet companions suite. */
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import uniq from "lodash/uniq";
import { type ReactElement } from "react";
import { beforeAll, describe, expect, it, vi } from "vitest";
import { COURSE_PET_KEEPOUTS, COURSE_PET_SURFACES, CourseApp, localStore, memoryStorageOrigin, readPreferences, writePreferences, type CoursePetsSource, type CoursePreferences } from "@semio-tech/course-react";
import { COURSE, PREFERENCES, motionPreference, named, settle } from "../../🪛️pet-companions-kit/🟦️.tsx";
import { QUIET_PRESENCE, SAMPLE, TIMING, courseProctor } from "../🟠️harness/🟦️.tsx";

/** 🔹 Registers the client tests of the 🐾️ pet companions suite. */
export function registerClientTests(): void {
  describe("the client", () => {
    beforeAll(async () => {
      await import("@semio-tech/pets-react");
    }, 120_000);

    const client = (storage: ReturnType<ReturnType<typeof memoryStorageOrigin>["tab"]>, pets?: CoursePetsSource): ReactElement => {
      localStore(storage, COURSE.id).write("introduced", true);
      return <CourseApp proctor="" tenant={COURSE.id} presence={QUIET_PRESENCE} transport={() => courseProctor()} storage={storage} languages={["en"]} timing={TIMING} pets={pets} />;
    };
    const settings = async (): Promise<void> => {
      await waitFor(() => expect(document.querySelector('[data-course-grid="root"]')).not.toBeNull());
      if (window.location.hash !== "#prefs") fireEvent.click(screen.getByRole("link", { name: "Settings" }));
      await screen.findByRole("group", { name: "Pets" });
    };
    const app = (): HTMLElement => document.querySelector<HTMLElement>(".course-app")!;

    it("carries the choice, fetches the site's menagerie and the real layer, and lets go of both when pets are switched off", async () => {
      const origin = memoryStorageOrigin();
      const pets = vi.fn<CoursePetsSource>(async () => SAMPLE);
      render(client(origin.tab(), pets));
      await settings();
      expect(app().dataset.pets).toBe("lively");
      const shown = await waitFor(() => {
        const found = app().querySelector<HTMLElement>(".pet-layer");
        expect(found).not.toBeNull();
        return found!;
      });
      expect(pets).toHaveBeenCalledTimes(1);
      expect(shown.getAttribute("aria-hidden")).toBe("true");
      expect(shown).toBe(app().lastElementChild);
      expect(shown.querySelector("a, button, input, select, textarea, [tabindex]")).toBeNull();
      const parts = [...document.querySelectorAll<HTMLElement>(COURSE_PET_SURFACES)].filter((part) => part.closest("[inert]") === null);
      const cards = uniq(parts.flatMap((part) => part.closest<HTMLElement>("[data-card]")?.dataset.card ?? []));
      expect(cards.length).toBeGreaterThanOrEqual(1);
      for (const card of cards) expect(parts.filter((part) => part.closest<HTMLElement>("[data-card]")?.dataset.card === card).map((part) => part.dataset.slot)).toEqual(["window-chrome-chip-cap"]);
      expect(parts.at(-1)).toBe(app().querySelector(":scope > footer"));
      expect(parts.at(-1)).not.toBeNull();
      expect([...document.querySelectorAll(COURSE_PET_KEEPOUTS)]).toContain(app().querySelector(":scope > header"));
      const group = screen.getByRole("group", { name: "Pets" });

      fireEvent.click(within(group).getByRole("button", { name: "Off" }));
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(group.nextElementSibling).toBeNull();
      expect(readPreferences(localStore(origin.tab(), COURSE.id)).pets).toBe("off");

      fireEvent.click(within(group).getByRole("button", { name: "Still" }));
      expect(app().dataset.pets).toBe("still");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(pets).toHaveBeenCalledTimes(1);
    });

    it("keeps a switch on the footer of every screen that hides the pets and brings them back as lively as they were", async () => {
      const origin = memoryStorageOrigin();
      render(client(origin.tab(), async () => SAMPLE));
      await settings();
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      const footer = app().querySelector<HTMLElement>(":scope > footer")!;
      const box = within(footer).getByRole("checkbox", { name: "Show pets" }) as HTMLInputElement;
      expect(box.checked).toBe(true);
      expect(box.closest("nav")).toBeNull();
      expect(within(footer).getAllByRole("button").map((button) => button.textContent)).toEqual(["What is stored"]);
      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }));
      expect(app().dataset.pets).toBe("lively");

      fireEvent.click(box);
      expect(box.checked).toBe(false);
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Off" }).getAttribute("aria-pressed")).toBe("true");
      expect(readPreferences(localStore(origin.tab(), COURSE.id))).toMatchObject({ pets: "off", petsLiveliness: "lively" });

      fireEvent.click(box);
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("lively");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(readPreferences(localStore(origin.tab(), COURSE.id))).toMatchObject({ pets: "lively", petsLiveliness: "lively" });
    });

    it("starts lively on a device that asks for reduced motion, and keeps a later choice for the next visit", async () => {
      motionPreference(true);
      const origin = memoryStorageOrigin();
      const stored = (): CoursePreferences => readPreferences(localStore(origin.tab(), COURSE.id));
      const row = (): HTMLElement => screen.getByRole("group", { name: "Pets" });
      const pressed = (): (string | null)[] => within(row()).getAllByRole("button").filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.textContent?.trim() ?? null);
      const first = render(client(origin.tab(), async () => SAMPLE));
      await settings();
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("lively");
      expect(app().dataset.iconMotion).toBe("on");
      expect(pressed()).toEqual(["Lively"]);
      expect((screen.getByRole("checkbox", { name: "Animate icons" }) as HTMLInputElement).checked).toBe(true);

      fireEvent.click(within(screen.getByRole("group", { name: "Theme" })).getByRole("button", { name: "Dark" }));
      fireEvent.click(screen.getByRole("checkbox", { name: "Animate icons" }));
      expect(stored()).toMatchObject({ theme: "dark", animateIcons: false, iconsChosen: true, pets: "lively", petsLiveliness: "lively", petsChosen: false });
      expect(app().dataset.pets).toBe("lively");
      expect(app().dataset.iconMotion).toBe("off");

      fireEvent.click(within(row()).getByRole("button", { name: "Calm" }));
      expect(app().dataset.pets).toBe("calm");
      expect(pressed()).toEqual(["Calm"]);
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(stored()).toMatchObject({ theme: "dark", pets: "calm", petsLiveliness: "calm", petsChosen: true });

      first.unmount();
      render(client(origin.tab(), async () => SAMPLE));
      await settings();
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("calm");
      expect(pressed()).toEqual(["Calm"]);
      expect(app().dataset.iconMotion).toBe("off");
    });

    it("counts the switch as a choice: it hides the lively pets and brings them back as lively as they were", async () => {
      motionPreference(true);
      const origin = memoryStorageOrigin();
      const stored = (): CoursePreferences => readPreferences(localStore(origin.tab(), COURSE.id));
      render(client(origin.tab(), async () => SAMPLE));
      await settings();
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      const box = within(app().querySelector<HTMLElement>(":scope > footer")!).getByRole("checkbox", { name: "Show pets" }) as HTMLInputElement;
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("lively");

      fireEvent.click(box);
      expect(box.checked).toBe(false);
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(stored()).toMatchObject({ pets: "off", petsLiveliness: "lively", petsChosen: true });

      fireEvent.click(box);
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("lively");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }).getAttribute("aria-pressed")).toBe("true");
      expect(stored()).toMatchObject({ pets: "lively", petsLiveliness: "lively", petsChosen: true });
    });

    it("keeps a liveliness that was stored without the word that the learner chose it, including on a device that asks for reduced motion", async () => {
      const prefer = motionPreference(true);
      const origin = memoryStorageOrigin();
      localStore(origin.tab(), COURSE.id).write("preferences", { theme: "dark", pets: "lively", petsLiveliness: "lively" });
      render(client(origin.tab(), async () => SAMPLE));
      await settings();
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("lively");
      prefer(false);
      expect(app().dataset.pets).toBe("lively");
      prefer(true);
      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Still" }));
      expect(app().dataset.pets).toBe("still");
      expect(readPreferences(localStore(origin.tab(), COURSE.id))).toMatchObject({ theme: "dark", pets: "still", petsLiveliness: "still", petsChosen: true });
    });

    it("fetches nothing for a learner who switched the pets off", async () => {
      const origin = memoryStorageOrigin();
      writePreferences(localStore(origin.tab(), COURSE.id), { ...PREFERENCES, pets: "off" });
      const pets = vi.fn<CoursePetsSource>(async () => SAMPLE);
      render(client(origin.tab(), pets));
      await settings();
      await settle();
      expect(app().dataset.pets).toBe("off");
      expect(pets).not.toHaveBeenCalled();
      expect(document.querySelector(".pet-layer")).toBeNull();
    });

    it("carries the choice on a site without pets, and shows none", async () => {
      render(client(memoryStorageOrigin().tab()));
      await settings();
      await settle();
      expect(app().dataset.pets).toBe("lively");
      expect(document.querySelector(".pet-layer")).toBeNull();
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling).toBeNull();
      expect(screen.queryByRole("checkbox", { name: "Show pets" })).toBeNull();
    });

    it("plays with the pets of the real layer from the settings, and keeps what the learner allows for the next visit", async () => {
      const origin = memoryStorageOrigin();
      const stored = (): CoursePreferences => readPreferences(localStore(origin.tab(), COURSE.id));
      const first = render(client(origin.tab(), async () => SAMPLE));
      await settings();
      const play = await screen.findByRole("group", { name: "Play with the pets" });
      const pets = within(play).getAllByRole("group");
      const known = SAMPLE.species.map((entry) => entry.name.en);
      expect(pets.length).toBeGreaterThan(0);
      for (const pet of pets) expect(known).toContain(named(pet));
      fireEvent.click(within(pets[0]!).getByRole("button", { name: "Hello" }));
      expect(within(play).getByRole("status").textContent).toBe(`${named(pets[0]!)} says hello.`);
      fireEvent.click(within(pets[0]!).getByRole("button", { name: "Toss" }));
      expect(within(play).getByRole("status").textContent).toBe(`${named(pets[0]!)} is tossed up.`);

      fireEvent.click(screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }));
      expect(screen.queryByRole("group", { name: "Play with the pets" })).toBeNull();
      expect(stored()).toMatchObject({ pets: "lively", petsChosen: false, petsPlay: false, petsMischief: true });
      fireEvent.click(screen.getByRole("checkbox", { name: "Pets may play with the page" }));
      expect(stored()).toMatchObject({ petsPlay: false, petsMischief: false });
      expect(app().querySelector(".pet-layer")).not.toBeNull();

      first.unmount();
      render(client(origin.tab(), async () => SAMPLE));
      await settings();
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).checked).toBe(false);
      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).checked).toBe(false);
      fireEvent.click(screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }));
      expect(await screen.findByRole("group", { name: "Play with the pets" })).not.toBeNull();
      expect(stored()).toMatchObject({ petsPlay: true, petsMischief: false, petsChosen: false });
    });
  });
}
