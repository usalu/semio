/** 🪛️ Shared doubles, motion preference, render targets and screen sessions of the pet-companion suites (the topics sit in the sibling folders `🐾️pet-companions`, `🎪️pets-on-stage` and `🎛️pet-allowances`). */
import { act, screen } from "@testing-library/react";
import { useEffect, useImperativeHandle, type ReactElement } from "react";
import { afterEach, vi } from "vitest";
import { type Menagerie, type Species, type Text } from "@semio-tech/pets";
import { type PetLayerProps } from "@semio-tech/pets-react";
import type { CatalogView, RunView } from "@semio-tech/quiz";
import { QuizPets, QuizPetsProvider, type PetChoice, type QuizPetsSource, type QuizPetsStage, type QuizPreferences, type QuizState, type QuizStep } from "@semio-tech/quiz-react";

//#region 🧫️Doubles
export const text = (en: string, de: string): Text => ({ en, de });

function species(id: string, name: Text): Species {
  return {
    id,
    name,
    thing: name,
    grounds: [],
    size: { width: 40, height: 48 },
    palette: { body: "#ffcc33", accent: "#ff8800", detail: "#663300" },
    bones: [{ id: "root", x: 0, y: 0 }],
    parts: [],
    face: { eyes: [] },
    clips: [],
    repertoire: {},
    locomotion: { gait: "float", speed: 30, hover: 4 },
    temperament: { energy: 0.5, sociability: 0.5, curiosity: 0.5 },
    states: [{ id: "resting", name: text("Resting", "In Ruhe") }],
    tricks: [],
    purr: { clip: "purr" },
    emitters: [],
    gear: [],
    grip: 43,
    reach: 10,
    mood: "content",
  };
}

export const MENAGERIE: Menagerie = {
  schema: "semio.pets.menagerie/v1",
  id: "test",
  title: text("Test pets", "Testtierchen"),
  species: [species("sunny", text("Sunny", "Sonni")), species("housy", text("Housy", "Hausi")), species("windy", text("Windy", "Windi")), species("flamy", text("Flamy", "Flammi"))],
  bonds: [],
  casts: [
    { scene: "home", core: ["sunny", "housy"], rotation: ["windy", "sunny"] },
    { scene: "heating", core: ["flamy"], rotation: ["housy", "ghost"] },
  ],
  chemistry: [],
};

export const CATALOG: CatalogView = {
  id: "arch",
  title: text("Architecture", "Architektur"),
  introduction: { title: text("Welcome", "Willkommen"), paragraphs: [] },
  quizzes: [
    { id: "heating", emoji: "🔥", title: text("Heating", "Heizen"), description: text("Warmth.", "Wärme."), tasks: [] },
    { id: "cooling", emoji: "❄️", title: text("Cooling", "Kühlen"), description: text("Cold.", "Kälte."), tasks: [] },
  ],
  badges: [],
};

export const RUNS: QuizState["runs"] = { r1: { run: "r1", quiz: "heating" } as RunView, r2: { run: "r2", quiz: "cooling" } as RunView };

export const PREFERENCES: QuizPreferences = { theme: "system", textSize: "normal", showCursors: true, others: "submitted", animateIcons: true, iconsChosen: false, pets: "calm", petsLiveliness: "calm", petsChosen: false, petsPlay: true, petsMischief: true, challenges: {} };

const ON_STAGE: Readonly<Record<string, readonly string[]>> = { home: ["sunny", "windy"], heating: ["flamy", "ghost", "housy"] };

export const REDUCED_MOTION = "(prefers-reduced-motion: reduce)";

export function at(step: QuizStep): Pick<QuizState, "step" | "runs" | "catalog"> {
  return { step, runs: RUNS, catalog: CATALOG };
}

/** 🫧️ A render target double: its layer records what the quiz hands it, shows it as attributes, says, like the real
 * one, who is on stage — the species of `ON_STAGE` for the scene — and nobody once it is gone, and its handle writes
 * down every deed it is asked for (`played`, as `"<species> <deed>"`). */
export function fakeStage(): { readonly stage: QuizPetsStage & ReturnType<typeof vi.fn>; readonly handed: PetLayerProps[]; readonly played: string[] } {
  const handed: PetLayerProps[] = [];
  const played: string[] = [];
  const PetLayer = (props: PetLayerProps): ReactElement => {
    handed.push(props);
    const { scene, onCast, ref } = props;
    useImperativeHandle(ref, () => ({ play: (species, deed) => void played.push(`${species} ${deed}`) }), []);
    useEffect(() => {
      onCast?.(ON_STAGE[scene] ?? []);
      return () => onCast?.([]);
    }, [scene, onCast]);
    return <div data-testid="pets" aria-hidden="true" data-scene={props.scene} data-mode={props.mode} data-quiet={String(props.quiet)} />;
  };
  return { stage: vi.fn(async () => ({ PetLayer, PET_KEEPOUTS: "button, p" })), handed, played };
}

export function fakeSource(): QuizPetsSource & ReturnType<typeof vi.fn> {
  return vi.fn(async () => MENAGERIE);
}

/** ⏳️ Lets every settled fetch reach the screen. */
export async function settle(): Promise<void> {
  await act(async () => {
    await Promise.resolve();
    await Promise.resolve();
    await Promise.resolve();
  });
}

/** 🧘️ Emulates the device's motion preference; the returned function changes it while the page is open, and only who
 * asked about motion hears of it — every other media query stays unmatched and silent. */
export function motionPreference(reduce: boolean): (reduce: boolean) => void {
  let matches = reduce;
  const listeners = new Set<(event: MediaQueryListEvent) => void>();
  vi.stubGlobal("matchMedia", (query: string) => ({
    get matches(): boolean {
      return query === REDUCED_MOTION && matches;
    },
    media: query,
    onchange: null,
    addEventListener: (_type: string, listener: (event: MediaQueryListEvent) => void) => (query === REDUCED_MOTION ? listeners.add(listener) : listeners),
    removeEventListener: (_type: string, listener: (event: MediaQueryListEvent) => void) => listeners.delete(listener),
    addListener: () => undefined,
    removeListener: () => undefined,
    dispatchEvent: () => false,
  }));
  return (next) => {
    matches = next;
    act(() => {
      for (const listener of listeners) listener({ matches: next, media: REDUCED_MOTION } as MediaQueryListEvent);
    });
  };
}

export const layer = (): HTMLElement | null => screen.queryByTestId("pets");

export function Pets(props: { readonly source?: QuizPetsSource; readonly stage?: QuizPetsStage; readonly choice: PetChoice; readonly chosen?: boolean; readonly play?: boolean; readonly mischief?: boolean; readonly step?: QuizStep; readonly locale?: "en" | "de"; readonly children?: ReactElement }): ReactElement {
  return (
    <QuizPetsProvider source={props.source} stage={props.stage} choice={props.choice} chosen={props.chosen ?? false} play={props.play ?? true} mischief={props.mischief ?? true} state={at(props.step ?? { screen: "home" })} locale={props.locale ?? "en"}>
      <p>quiz</p>
      {props.children}
      <QuizPets />
    </QuizPetsProvider>
  );
}

/** 📛️ The name of a group named by another element. */
export function named(group: HTMLElement): string | undefined {
  return document.getElementById(group.getAttribute("aria-labelledby") ?? "")?.textContent ?? undefined;
}

afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});
//#endregion 🧫️Doubles
