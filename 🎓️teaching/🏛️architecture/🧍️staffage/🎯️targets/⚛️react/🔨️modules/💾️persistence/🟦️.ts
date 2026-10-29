/** 💾️ Local-first persistence of the archive app: an event-sourced store whose folded state is saved under one versioned key.
 *
 * Commands become events ({@link ArchiveEvent}); a pure reducer folds them into {@link ArchiveState}; every event is written through to storage as a `{ version, state }` snapshot
 * and published to subscribers, so the UI is a projection of the event stream. Storage is injected: the browser's `localStorage`, or memory when storage is blocked or full
 * (the store then reports `persistence: "volatile"` and the UI says so). Another tab's write arrives as a `storage` event and replaces the folded state.
 * Persisted local-only: preferences, favorites, saved collections, the playground. Ephemeral shared: everything in the URL. Ephemeral local-only: clock, hover, drag.
 *
 * @see ../🗺️routing/🟦️.ts — the ephemeral shared state
 * @see https://developer.mozilla.org/docs/Web/API/Window/storage_event — cross-tab events
 */
import { useSyncExternalStore } from "react";
import type { Language, Slug, View } from "@teaching/staffage";
import { isLanguage } from "../🌐️i18n/🟦️.ts";

/** 🔖️ The version of the stored document; a different version is dropped, not migrated. */
export const STORAGE_VERSION = 1;

/** 🔑️ The one key the folded state lives under. */
export const STORAGE_KEY = `semio.staffage.v${STORAGE_VERSION}`;

/** 🗄️ The storage surface the store needs. */
export interface StorageArea {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

/** 🧠️ A storage area that lives and dies with the page. */
export function memoryStorage(): StorageArea {
  const items = new Map<string, string>();
  return { getItem: (key) => items.get(key) ?? null, setItem: (key, value) => void items.set(key, value), removeItem: (key) => void items.delete(key) };
}

/** 🌐️ The browser's `localStorage` when it works, else memory. */
export function browserStorage(): { readonly area: StorageArea; readonly durable: boolean } {
  try {
    const probe = `${STORAGE_KEY}.probe`;
    localStorage.setItem(probe, "1");
    localStorage.removeItem(probe);
    return { area: localStorage, durable: true };
  } catch {
    return { area: memoryStorage(), durable: false };
  }
}

/** 🌓️ A colour theme choice. */
export type Theme = "system" | "light" | "dark";

/** 🎞️ Whether animation follows the device, is reduced, or runs fully. */
export type Motion = "system" | "reduced" | "full";

/** 🔠️ A text size choice. */
export type TextSize = "normal" | "large";

/** 🎛️ The reader's preferences; `language` stays unset until the reader or the browser's list decides. */
export type Preferences = { readonly language?: Language; readonly theme: Theme; readonly motion: Motion; readonly textSize: TextSize; readonly playful: boolean };

/** 🖼️ The ground of the playground. */
export const BACKGROUNDS = ["paper", "dark", "light", "transparent"] as const;

/** 🖼️ One of {@link BACKGROUNDS}. */
export type Background = (typeof BACKGROUNDS)[number];

/** 🎯️ What a placed or compared figure shows: a static pose, or a clip with a phase offset in seconds. */
export type Selection = { readonly kind: "pose"; readonly pose: Slug } | { readonly kind: "clip"; readonly clip: Slug };

/** 📍️ One asset on the playground's ground line, `xM` metres from its left end. */
export type Placement = { readonly id: string; readonly asset: Slug; readonly xM: number; readonly pick: Selection | null; readonly offsetS: number; readonly mirrored: boolean };

/** 🧪️ The playground composition: shared scale, view and ground, plus its placements. */
export type Playground = { readonly scale: number; readonly view: View; readonly background: Background; readonly lengthM: number; readonly placements: readonly Placement[] };

/** 📚️ A collection the reader saved. */
export type SavedCollection = { readonly id: string; readonly name: string; readonly assets: readonly Slug[] };

/** 🗃️ Everything persisted. */
export type ArchiveState = { readonly preferences: Preferences; readonly favorites: readonly Slug[]; readonly collections: readonly SavedCollection[]; readonly playground: Playground };

/** 🎬️ Every change the archive can undergo. */
export type ArchiveEvent =
  | { readonly type: "preferences-changed"; readonly patch: Partial<Preferences> }
  | { readonly type: "language-cleared" }
  | { readonly type: "favorite-added"; readonly asset: Slug }
  | { readonly type: "favorite-removed"; readonly asset: Slug }
  | { readonly type: "collection-saved"; readonly collection: SavedCollection }
  | { readonly type: "collection-removed"; readonly id: string }
  | { readonly type: "playground-configured"; readonly patch: Partial<Pick<Playground, "scale" | "view" | "background" | "lengthM">> }
  | { readonly type: "placement-added"; readonly placement: Placement }
  | { readonly type: "placement-changed"; readonly id: string; readonly patch: Partial<Omit<Placement, "id">> }
  | { readonly type: "placement-duplicated"; readonly id: string; readonly newId: string }
  | { readonly type: "placement-removed"; readonly id: string }
  | { readonly type: "playground-cleared" }
  | { readonly type: "playground-replaced"; readonly playground: Playground }
  | { readonly type: "everything-cleared" };

/** 📏️ The playground's default ground length in metres. */
export const DEFAULT_LENGTH_M = 24;

/** 🌱️ The state of a reader who has chosen nothing. */
export const INITIAL_STATE: ArchiveState = {
  preferences: { theme: "system", motion: "system", textSize: "normal", playful: true },
  favorites: [],
  collections: [],
  playground: { scale: 100, view: "dark-line", background: "paper", lengthM: DEFAULT_LENGTH_M, placements: [] },
};

function clampX(playground: Playground, xM: number): number {
  return Math.min(Math.max(xM, 0), playground.lengthM);
}

/** 🧮️ Folds one event into the state; pure and total. */
export function reduce(state: ArchiveState, event: ArchiveEvent): ArchiveState {
  const { playground } = state;
  switch (event.type) {
    case "preferences-changed":
      return { ...state, preferences: { ...state.preferences, ...event.patch } };
    case "language-cleared": {
      const { language: _language, ...rest } = state.preferences;
      return { ...state, preferences: rest };
    }
    case "favorite-added":
      return state.favorites.includes(event.asset) ? state : { ...state, favorites: [...state.favorites, event.asset] };
    case "favorite-removed":
      return { ...state, favorites: state.favorites.filter((asset) => asset !== event.asset) };
    case "collection-saved":
      return { ...state, collections: [...state.collections.filter((collection) => collection.id !== event.collection.id), event.collection] };
    case "collection-removed":
      return { ...state, collections: state.collections.filter((collection) => collection.id !== event.id) };
    case "playground-configured": {
      const next = { ...playground, ...event.patch };
      return { ...state, playground: { ...next, placements: next.placements.map((placement) => ({ ...placement, xM: clampX(next, placement.xM) })) } };
    }
    case "placement-added":
      return { ...state, playground: { ...playground, placements: [...playground.placements, { ...event.placement, xM: clampX(playground, event.placement.xM) }] } };
    case "placement-changed":
      return { ...state, playground: { ...playground, placements: playground.placements.map((placement) => (placement.id === event.id ? { ...placement, ...event.patch, xM: clampX(playground, event.patch.xM ?? placement.xM) } : placement)) } };
    case "placement-duplicated": {
      const source = playground.placements.find((placement) => placement.id === event.id);
      return source === undefined ? state : { ...state, playground: { ...playground, placements: [...playground.placements, { ...source, id: event.newId, xM: clampX(playground, source.xM + 0.6) }] } };
    }
    case "placement-removed":
      return { ...state, playground: { ...playground, placements: playground.placements.filter((placement) => placement.id !== event.id) } };
    case "playground-cleared":
      return { ...state, playground: { ...playground, placements: [] } };
    case "playground-replaced":
      return { ...state, playground: event.playground };
    case "everything-cleared":
      return INITIAL_STATE;
  }
}

const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);
const isSlug = (value: unknown): value is Slug => typeof value === "string" && /^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value);
const finite = (value: unknown, fallback: number): number => (typeof value === "number" && Number.isFinite(value) ? value : fallback);
const oneOf = <T extends string>(value: unknown, options: readonly T[], fallback: T): T => (typeof value === "string" && (options as readonly string[]).includes(value) ? (value as T) : fallback);

function sanitizePick(value: unknown): Selection | null {
  if (!isRecord(value)) return null;
  if (value.kind === "pose" && isSlug(value.pose)) return { kind: "pose", pose: value.pose };
  if (value.kind === "clip" && isSlug(value.clip)) return { kind: "clip", clip: value.clip };
  return null;
}

/** 🧼️ Reads a stored document defensively: unknown versions and malformed members fall back to the initial state, nothing throws. */
export function sanitize(value: unknown, views: readonly View[]): ArchiveState {
  if (!isRecord(value) || value.version !== STORAGE_VERSION || !isRecord(value.state)) return INITIAL_STATE;
  const { preferences, favorites, collections, playground } = value.state;
  const prefs = isRecord(preferences) ? preferences : {};
  const ground = isRecord(playground) ? playground : {};
  const lengthM = Math.min(Math.max(finite(ground.lengthM, DEFAULT_LENGTH_M), 4), 200);
  return {
    preferences: {
      ...(isLanguage(prefs.language) ? { language: prefs.language } : {}),
      theme: oneOf(prefs.theme, ["system", "light", "dark"], "system"),
      motion: oneOf(prefs.motion, ["system", "reduced", "full"], "system"),
      textSize: oneOf(prefs.textSize, ["normal", "large"], "normal"),
      playful: prefs.playful !== false,
    },
    favorites: Array.isArray(favorites) ? [...new Set(favorites.filter(isSlug))] : [],
    collections: Array.isArray(collections)
      ? collections.flatMap((item) => (isRecord(item) && typeof item.id === "string" && typeof item.name === "string" && Array.isArray(item.assets) ? [{ id: item.id, name: item.name, assets: item.assets.filter(isSlug) }] : []))
      : [],
    playground: {
      scale: Math.max(1, finite(ground.scale, 100)),
      view: oneOf(ground.view, views, "dark-line"),
      background: oneOf(ground.background, BACKGROUNDS, "paper"),
      lengthM,
      placements: Array.isArray(ground.placements)
        ? ground.placements.flatMap((item) =>
            isRecord(item) && typeof item.id === "string" && isSlug(item.asset)
              ? [{ id: item.id, asset: item.asset, xM: Math.min(Math.max(finite(item.xM, 0), 0), lengthM), pick: sanitizePick(item.pick), offsetS: finite(item.offsetS, 0), mirrored: item.mirrored === true }]
              : [],
          )
        : [],
    },
  };
}

/** 🗃️ Whether what the store holds survives a reload. */
export type PersistenceMode = "durable" | "volatile";

/** 🏪️ The event-sourced store: `dispatch` an event, read the folded state, subscribe to changes. */
export class ArchiveStore {
  #state: ArchiveState;
  #mode: PersistenceMode;
  readonly #listeners = new Set<() => void>();
  readonly #observers = new Set<(event: ArchiveEvent) => void>();
  readonly #storage: StorageArea;
  readonly #views: readonly View[];

  constructor(storage: StorageArea, views: readonly View[], durable = true) {
    this.#storage = storage;
    this.#views = views;
    this.#mode = durable ? "durable" : "volatile";
    this.#state = this.#read();
  }

  get mode(): PersistenceMode {
    return this.#mode;
  }

  getSnapshot = (): ArchiveState => this.#state;

  subscribe = (listener: () => void): (() => void) => {
    this.#listeners.add(listener);
    return () => void this.#listeners.delete(listener);
  };

  observe(observer: (event: ArchiveEvent) => void): () => void {
    this.#observers.add(observer);
    return () => void this.#observers.delete(observer);
  }

  dispatch(event: ArchiveEvent): void {
    const next = reduce(this.#state, event);
    if (next === this.#state) return;
    this.#state = next;
    this.#write();
    for (const observer of [...this.#observers]) observer(event);
    for (const listener of [...this.#listeners]) listener();
  }

  reload(): void {
    this.#state = this.#read();
    for (const listener of [...this.#listeners]) listener();
  }

  watch(target: Pick<Window, "addEventListener" | "removeEventListener">): () => void {
    const onStorage = (event: Event): void => {
      if ((event as StorageEvent).key === STORAGE_KEY || (event as StorageEvent).key === null) this.reload();
    };
    target.addEventListener("storage", onStorage);
    return () => target.removeEventListener("storage", onStorage);
  }

  #read(): ArchiveState {
    try {
      const raw = this.#storage.getItem(STORAGE_KEY);
      return raw === null ? INITIAL_STATE : sanitize(JSON.parse(raw), this.#views);
    } catch {
      return INITIAL_STATE;
    }
  }

  #write(): void {
    try {
      this.#storage.setItem(STORAGE_KEY, JSON.stringify({ version: STORAGE_VERSION, state: this.#state }));
    } catch {
      this.#mode = "volatile";
    }
  }
}

/** 🪝️ Subscribes a component to the folded state. */
export function useArchiveState(store: ArchiveStore): ArchiveState {
  return useSyncExternalStore(store.subscribe, store.getSnapshot, store.getSnapshot);
}
