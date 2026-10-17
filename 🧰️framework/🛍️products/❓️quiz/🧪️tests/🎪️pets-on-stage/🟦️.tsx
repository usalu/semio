/** 🎪️ Pet companions: the pets on stage and the client around them. */
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import uniq from "lodash/uniq";
import { type ReactElement } from "react";
import { describe, expect, it, vi } from "vitest";
import { decodeQueryEnvelope, encodeQueryResult, type HttpResponse, type HttpTransport } from "@semio-tech/framework-server";
import { type Menagerie } from "@semio-tech/pets";
import type { Query } from "@semio-tech/quiz";
import { PET_CHOICES, PreferencesPanel, QUIZ_LOCALES, QUIZ_PET_KEEPOUTS, QUIZ_PET_SURFACES, QuizApp, localStore, memoryStorageOrigin, petNames, quizInstance, quizText, readPreferences, writePreferences, type PetChoice, type PresenceConnect, type QuizPetsSource, type QuizPreferences } from "@semio-tech/quiz-react";
import { schemaConformanceFixture as vectors } from "@semio-tech/pets/testing/fixtures/schema-conformance";
import { CATALOG, MENAGERIE, PREFERENCES, Pets, REDUCED_MOTION, fakeSource, fakeStage, layer, motionPreference, named, settle } from "../🪛️pet-companions-kit/🟦️.tsx";

const SAMPLE = vectors.menagerie as unknown as Menagerie;

const TIMING = { minMs: 1, maxMs: 4 };

const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });

const encoder = new TextEncoder();

const decoder = new TextDecoder();

function reply(status: number, body: unknown): HttpResponse {
  const payload = JSON.stringify(body);
  return { status, text: async () => payload, bytes: async () => encoder.encode(payload) };
}

/** 🛂️ A proctor that knows the catalog and nothing else. */
function catalogProctor(): HttpTransport {
  return {
    send: async (request) => {
      if (request.method === "GET" && request.path === "/instance") return reply(200, quizInstance());
      const body = JSON.parse(typeof request.body === "string" ? request.body : decoder.decode(request.body)) as unknown;
      if (request.path !== "/queries") return reply(404, { kind: "notFound", message: request.path });
      const query = JSON.parse(decoder.decode(decodeQueryEnvelope(body).arguments)) as Query;
      if (query.type !== "catalog") return reply(404, { kind: "notFound", message: query.type });
      return reply(200, encodeQueryResult({ kind: "snapshot", value: encoder.encode(JSON.stringify(CATALOG)), frontier: null }));
    },
  };
}

describe("🐾️ pet companions", () => {
  describe("the pets on stage", () => {
    it("are named in the learner's language, in the order the layer names them, and nobody the menagerie does not know", () => {
      expect(petNames(MENAGERIE, ["windy", "sunny"], "en")).toEqual(["Windy", "Sunny"]);
      expect(petNames(MENAGERIE, ["flamy", "ghost", "housy"], "de")).toEqual(["Flammi", "Hausi"]);
      expect(petNames(MENAGERIE, [], "en")).toEqual([]);
      expect(petNames(SAMPLE, ["blobby", "hoppy", "floaty"], "de")).toEqual(["Blobby, der Klecks", "Hoppy, die Sprungfeder", "Floaty, der Ballon"]);
      for (const locale of QUIZ_LOCALES) expect(petNames(SAMPLE, uniq(SAMPLE.casts.flatMap((cast) => [...cast.core, ...cast.rotation])), locale)).toEqual(uniq(SAMPLE.casts.flatMap((cast) => [...cast.core, ...cast.rotation])).map((id) => SAMPLE.species.find((entry) => entry.id === id)!.name[locale]));
    });

    it("are named under the choice as soon as the layer says who is there, in both languages, and no longer once they are gone", async () => {
      const lines = { en: { home: "Here right now: Sunny · Windy", heating: "Here right now: Flamy · Housy" }, de: { home: "Gerade hier: Sonni · Windi", heating: "Gerade hier: Flammi · Hausi" } };
      for (const locale of QUIZ_LOCALES) {
        const source = fakeSource();
        const { stage } = fakeStage();
        const panel = <PreferencesPanel preferences={PREFERENCES} locale={locale} text={quizText(locale)} onChange={() => undefined} />;
        const { rerender, unmount } = render(
          <Pets source={source} stage={stage} choice="calm" locale={locale}>
            {panel}
          </Pets>,
        );
        expect(screen.queryByText(/ · /u)).toBeNull();
        await settle();
        const group = screen.getByRole("group", { name: quizText(locale)("quiz.preferences.pets") });
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
      render(<PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={() => undefined} />);
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling).toBeNull();
    });

    it("stay still for a device that asks for reduced motion until the learner chooses: the preferences press what is in effect and say why in both languages, and say nothing once the learner chose", async () => {
      const notes = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };
      const prefer = motionPreference(true);
      for (const locale of QUIZ_LOCALES) {
        prefer(true);
        const source = fakeSource();
        const { stage } = fakeStage();
        const pets = (choice: PetChoice, chosen: boolean): ReactElement => (
          <Pets source={source} stage={stage} choice={choice} chosen={chosen} locale={locale}>
            <PreferencesPanel preferences={{ ...PREFERENCES, pets: choice, petsChosen: chosen }} locale={locale} text={quizText(locale)} onChange={() => undefined} />
          </Pets>
        );
        const { rerender, unmount } = render(pets("lively", false));
        const group = screen.getByRole("group", { name: quizText(locale)("quiz.preferences.pets") });
        const pressed = (): PetChoice[] => PET_CHOICES.filter((_, index) => within(group).getAllByRole("button")[index]?.getAttribute("aria-pressed") === "true");
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.tagName).toBe("P");
        expect(pressed()).toEqual(["still"]);
        await settle();
        expect(layer()?.dataset.mode).toBe("still");
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.nextElementSibling?.textContent).toContain(" · ");

        for (const choice of ["calm", "lively", "still"] as const) {
          rerender(pets(choice, true));
          expect(layer()?.dataset.mode, choice).toBe(choice);
          expect(pressed(), choice).toEqual([choice]);
          expect(screen.queryByText(notes[locale]), choice).toBeNull();
          expect(group.nextElementSibling?.textContent, choice).toContain(" · ");
          rerender(pets(choice, false));
          expect(layer()?.dataset.mode, choice).toBe("still");
          expect(pressed(), choice).toEqual(["still"]);
          expect(group.nextElementSibling?.textContent, choice).toBe(notes[locale]);
        }
        for (const chosen of [true, false]) {
          rerender(pets("off", chosen));
          expect(layer()).toBeNull();
          expect(pressed()).toEqual(["off"]);
          expect(screen.queryByText(notes[locale])).toBeNull();
        }
        prefer(false);
        rerender(pets("lively", false));
        expect(layer()?.dataset.mode).toBe("lively");
        expect(pressed()).toEqual(["lively"]);
        expect(screen.queryByText(notes[locale])).toBeNull();
        unmount();
      }
      prefer(true);
      render(
        <Pets choice="lively">
          <PreferencesPanel preferences={{ ...PREFERENCES, pets: "lively" }} locale="en" text={quizText("en")} onChange={() => undefined} />
        </Pets>,
      );
      expect(screen.queryByText(notes.en)).toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }).getAttribute("aria-pressed")).toBe("true");
    });

    it("show nobody for a device that forces its own colours, whatever the learner chose, and the preferences say that instead of the word on motion, in both languages", async () => {
      const notes = { en: "Your device uses its own contrast colours, so no pets show.", de: "Dein Gerät verwendet eigene Kontrastfarben, deshalb erscheinen keine Tierchen." };
      const still = { en: "Your device asks for less motion, so the pets stay still until you choose.", de: "Dein Gerät bittet um weniger Bewegung, deshalb bleiben die Tierchen reglos, bis du selbst wählst." };
      const answers = ["(forced-colors: active)", REDUCED_MOTION];
      vi.stubGlobal("matchMedia", (query: string) => ({ matches: answers.includes(query), media: query, onchange: null, addEventListener: () => undefined, removeEventListener: () => undefined, addListener: () => undefined, removeListener: () => undefined, dispatchEvent: () => false }));
      for (const locale of QUIZ_LOCALES) {
        const panel = (choice: PetChoice, chosen = false): ReactElement => <PreferencesPanel preferences={{ ...PREFERENCES, pets: choice, petsChosen: chosen }} locale={locale} text={quizText(locale)} onChange={() => undefined} />;
        const source = fakeSource();
        const { stage } = fakeStage();
        const { rerender, unmount } = render(
          <Pets source={source} stage={stage} choice="calm" locale={locale}>
            {panel("calm")}
          </Pets>,
        );
        const group = screen.getByRole("group", { name: quizText(locale)("quiz.preferences.pets") });
        expect(group.nextElementSibling?.textContent).toBe(notes[locale]);
        expect(group.nextElementSibling?.tagName).toBe("P");
        expect(screen.queryByText(still[locale])).toBeNull();
        await settle();
        expect(layer()?.dataset.mode).toBe("still");
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
          <PreferencesPanel preferences={PREFERENCES} locale="en" text={quizText("en")} onChange={() => undefined} />
        </Pets>,
      );
      expect(screen.queryByText(notes.en)).toBeNull();
    });
  });

  describe("the client", () => {
    const client = (storage: ReturnType<ReturnType<typeof memoryStorageOrigin>["tab"]>, pets?: QuizPetsSource): ReactElement => (
      <QuizApp proctor="" tenant={CATALOG.id} presence={QUIET_PRESENCE} transport={() => catalogProctor()} storage={storage} languages={["en"]} timing={TIMING} pets={pets} />
    );
    const app = (): HTMLElement => document.querySelector<HTMLElement>(".quiz-app")!;

    it("carries the choice, fetches the site's menagerie and the real layer, and lets go of both when pets are switched off", async () => {
      const origin = memoryStorageOrigin();
      const pets = vi.fn<QuizPetsSource>(async () => SAMPLE);
      render(client(origin.tab(), pets));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      expect(app().dataset.pets).toBe("calm");
      const shown = await waitFor(() => {
        const found = app().querySelector<HTMLElement>(".pet-layer");
        expect(found).not.toBeNull();
        return found!;
      });
      expect(pets).toHaveBeenCalledTimes(1);
      expect(shown.getAttribute("aria-hidden")).toBe("true");
      expect(shown).toBe(app().lastElementChild);
      expect(shown.querySelector("a, button, input, select, textarea, [tabindex]")).toBeNull();
      const parts = [...document.querySelectorAll<HTMLElement>(QUIZ_PET_SURFACES)].filter((part) => part.closest("[inert]") === null);
      const cards = uniq(parts.flatMap((part) => part.closest<HTMLElement>("[data-card]")?.dataset.card ?? []));
      expect(cards.length).toBeGreaterThanOrEqual(1);
      for (const card of cards) expect(parts.filter((part) => part.closest<HTMLElement>("[data-card]")?.dataset.card === card).map((part) => part.dataset.slot)).toEqual(["window-chrome-chip-cap"]);
      expect(parts.at(-1)).toBe(app().querySelector(":scope > footer"));
      expect(parts.at(-1)).not.toBeNull();
      expect([...document.querySelectorAll(QUIZ_PET_KEEPOUTS)]).toContain(app().querySelector(":scope > header"));
      const group = screen.getByRole("group", { name: "Pets" });

      fireEvent.click(within(group).getByRole("button", { name: "Off" }));
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(group.nextElementSibling).toBeNull();
      expect(readPreferences(localStore(origin.tab(), CATALOG.id)).pets).toBe("off");

      fireEvent.click(within(group).getByRole("button", { name: "Still" }));
      expect(app().dataset.pets).toBe("still");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(pets).toHaveBeenCalledTimes(1);
    });

    it("keeps a switch on the footer of every screen that hides the pets and brings them back as lively as they were", async () => {
      const origin = memoryStorageOrigin();
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
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
      expect(readPreferences(localStore(origin.tab(), CATALOG.id))).toMatchObject({ pets: "off", petsLiveliness: "lively" });

      fireEvent.click(box);
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("lively");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(readPreferences(localStore(origin.tab(), CATALOG.id))).toMatchObject({ pets: "lively", petsLiveliness: "lively" });
    });

    it("holds the pets of a device that asks for reduced motion still only until the learner chooses in the preferences, and keeps that choice for the next visit", async () => {
      const note = "Your device asks for less motion, so the pets stay still until you choose.";
      motionPreference(true);
      const origin = memoryStorageOrigin();
      const stored = (): QuizPreferences => readPreferences(localStore(origin.tab(), CATALOG.id));
      const row = (): HTMLElement => screen.getByRole("group", { name: "Pets" });
      const pressed = (): (string | null)[] => within(row()).getAllByRole("button").filter((button) => button.getAttribute("aria-pressed") === "true").map((button) => button.textContent);
      const first = render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("still");
      expect(pressed()).toEqual(["Still"]);
      expect(row().nextElementSibling?.textContent).toBe(note);

      fireEvent.click(within(screen.getByRole("group", { name: "Theme" })).getByRole("button", { name: "Dark" }));
      fireEvent.click(screen.getByRole("checkbox", { name: "Animate icons" }));
      expect(stored()).toMatchObject({ theme: "dark", animateIcons: true, iconsChosen: true, pets: "calm", petsLiveliness: "calm", petsChosen: false });
      expect(app().dataset.pets).toBe("still");
      expect(row().nextElementSibling?.textContent).toBe(note);

      fireEvent.click(within(row()).getByRole("button", { name: "Calm" }));
      expect(app().dataset.pets).toBe("calm");
      expect(pressed()).toEqual(["Calm"]);
      expect(screen.queryByText(note)).toBeNull();
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(stored()).toMatchObject({ theme: "dark", pets: "calm", petsLiveliness: "calm", petsChosen: true });

      first.unmount();
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("calm");
      expect(pressed()).toEqual(["Calm"]);
      expect(screen.queryByText(note)).toBeNull();
    });

    it("counts the switch as a choice: on a device that asks for reduced motion it hides the still pets and brings calm ones back", async () => {
      const note = "Your device asks for less motion, so the pets stay still until you choose.";
      motionPreference(true);
      const origin = memoryStorageOrigin();
      const stored = (): QuizPreferences => readPreferences(localStore(origin.tab(), CATALOG.id));
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      const box = within(app().querySelector<HTMLElement>(":scope > footer")!).getByRole("checkbox", { name: "Show pets" }) as HTMLInputElement;
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("still");
      expect(screen.getByText(note)).not.toBeNull();

      fireEvent.click(box);
      expect(box.checked).toBe(false);
      expect(app().dataset.pets).toBe("off");
      expect(app().querySelector(".pet-layer")).toBeNull();
      expect(screen.queryByText(note)).toBeNull();
      expect(stored()).toMatchObject({ pets: "off", petsLiveliness: "calm", petsChosen: true });

      fireEvent.click(box);
      expect(box.checked).toBe(true);
      expect(app().dataset.pets).toBe("calm");
      expect(app().querySelector(".pet-layer")).not.toBeNull();
      expect(screen.queryByText(note)).toBeNull();
      expect(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Calm" }).getAttribute("aria-pressed")).toBe("true");
      expect(stored()).toMatchObject({ pets: "calm", petsLiveliness: "calm", petsChosen: true });
    });

    it("takes a liveliness that was stored without the word that the learner chose it for the default: still on a device that asks for reduced motion, as stored on any other", async () => {
      const note = "Your device asks for less motion, so the pets stay still until you choose.";
      const prefer = motionPreference(true);
      const origin = memoryStorageOrigin();
      localStore(origin.tab(), CATALOG.id).write("preferences", { theme: "dark", pets: "lively", petsLiveliness: "lively" });
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect(app().dataset.pets).toBe("still");
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling?.textContent).toBe(note);
      prefer(false);
      expect(app().dataset.pets).toBe("lively");
      expect(screen.queryByText(note)).toBeNull();
      prefer(true);
      fireEvent.click(within(screen.getByRole("group", { name: "Pets" })).getByRole("button", { name: "Lively" }));
      expect(app().dataset.pets).toBe("lively");
      expect(screen.queryByText(note)).toBeNull();
      expect(readPreferences(localStore(origin.tab(), CATALOG.id))).toMatchObject({ theme: "dark", pets: "lively", petsLiveliness: "lively", petsChosen: true });
    });

    it("fetches nothing for a learner who switched the pets off", async () => {
      const origin = memoryStorageOrigin();
      writePreferences(localStore(origin.tab(), CATALOG.id), { ...PREFERENCES, pets: "off" });
      const pets = vi.fn<QuizPetsSource>(async () => SAMPLE);
      render(client(origin.tab(), pets));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await settle();
      expect(app().dataset.pets).toBe("off");
      expect(pets).not.toHaveBeenCalled();
      expect(document.querySelector(".pet-layer")).toBeNull();
    });

    it("carries the choice on a site without pets, and shows none", async () => {
      render(client(memoryStorageOrigin().tab()));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await settle();
      expect(app().dataset.pets).toBe("calm");
      expect(document.querySelector(".pet-layer")).toBeNull();
      expect(screen.getByRole("group", { name: "Pets" }).nextElementSibling).toBeNull();
      expect(screen.queryByRole("checkbox", { name: "Show pets" })).toBeNull();
    });

    it("plays with the pets of the real layer from the settings, and keeps what the learner allows for the next visit", async () => {
      const origin = memoryStorageOrigin();
      const stored = (): QuizPreferences => readPreferences(localStore(origin.tab(), CATALOG.id));
      const first = render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
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
      expect(stored()).toMatchObject({ pets: "calm", petsChosen: false, petsPlay: false, petsMischief: true });
      fireEvent.click(screen.getByRole("checkbox", { name: "Pets may play with the page" }));
      expect(stored()).toMatchObject({ petsPlay: false, petsMischief: false });
      expect(app().querySelector(".pet-layer")).not.toBeNull();

      first.unmount();
      render(client(origin.tab(), async () => SAMPLE));
      await screen.findByRole("heading", { level: 1, name: "Welcome" });
      await waitFor(() => expect(app().querySelector(".pet-layer")).not.toBeNull());
      expect((screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }) as HTMLInputElement).checked).toBe(false);
      expect((screen.getByRole("checkbox", { name: "Pets may play with the page" }) as HTMLInputElement).checked).toBe(false);
      fireEvent.click(screen.getByRole("checkbox", { name: "Pets react to clicks and can be picked up" }));
      expect(await screen.findByRole("group", { name: "Play with the pets" })).not.toBeNull();
      expect(stored()).toMatchObject({ petsPlay: true, petsMischief: false, petsChosen: false });
    });
  });
});
