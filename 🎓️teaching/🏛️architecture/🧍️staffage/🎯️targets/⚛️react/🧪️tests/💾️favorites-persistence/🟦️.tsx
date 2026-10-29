/** 💾️ Favorites, saved collections and the playground persist under one versioned key, event by event, and read defensively.
 * @see ./🥒️.feature — the scenarios */
import { describe, expect, it } from "vitest";
import { VIEWS } from "@teaching/staffage";
import { ArchiveStore, INITIAL_STATE, STORAGE_KEY, STORAGE_VERSION, memoryStorage, reduce, sanitize } from "../../🔨️modules/💾️persistence/🟦️.ts";

const placement = { id: "p1", asset: "park-bench", xM: 30, pick: null, offsetS: 0, mirrored: false };

describe("favorites", () => {
  it("survive a reload through the same storage", () => {
    const storage = memoryStorage();
    const first = new ArchiveStore(storage, VIEWS);
    first.dispatch({ type: "favorite-added", asset: "walking-commuter" });
    first.dispatch({ type: "favorite-added", asset: "park-bench" });
    first.dispatch({ type: "favorite-added", asset: "park-bench" });
    first.dispatch({ type: "favorite-removed", asset: "walking-commuter" });
    expect(new ArchiveStore(storage, VIEWS).getSnapshot().favorites).toEqual(["park-bench"]);
    expect(JSON.parse(storage.getItem(STORAGE_KEY)!).version).toBe(STORAGE_VERSION);
  });

  it("notify subscribers and observers of each event once", () => {
    const store = new ArchiveStore(memoryStorage(), VIEWS);
    const seen: string[] = [];
    let notified = 0;
    store.observe((event) => seen.push(event.type));
    store.subscribe(() => (notified += 1));
    store.dispatch({ type: "favorite-added", asset: "a-b" });
    store.dispatch({ type: "favorite-added", asset: "a-b" });
    expect(seen).toEqual(["favorite-added"]);
    expect(notified).toBe(1);
  });

  it("reach another tab through the storage event", () => {
    const storage = memoryStorage();
    const mine = new ArchiveStore(storage, VIEWS);
    const theirs = new ArchiveStore(storage, VIEWS);
    const target = new EventTarget();
    const stop = mine.watch(target as Window);
    theirs.dispatch({ type: "favorite-added", asset: "cyclist" });
    target.dispatchEvent(Object.assign(new Event("storage"), { key: STORAGE_KEY }));
    expect(mine.getSnapshot().favorites).toEqual(["cyclist"]);
    stop();
  });
});

describe("the folded state", () => {
  it("keeps saved collections by id and clamps placements to the ground", () => {
    let state = reduce(INITIAL_STATE, { type: "collection-saved", collection: { id: "c1", name: "Mine", assets: ["a-b"] } });
    state = reduce(state, { type: "collection-saved", collection: { id: "c1", name: "Renamed", assets: ["a-b", "c-d"] } });
    expect(state.collections).toEqual([{ id: "c1", name: "Renamed", assets: ["a-b", "c-d"] }]);
    state = reduce(state, { type: "placement-added", placement });
    expect(state.playground.placements[0]!.xM).toBe(INITIAL_STATE.playground.lengthM);
    state = reduce(state, { type: "playground-configured", patch: { lengthM: 10 } });
    expect(state.playground.placements[0]!.xM).toBe(10);
    state = reduce(state, { type: "placement-duplicated", id: "p1", newId: "p2" });
    expect(state.playground.placements.map((item) => item.id)).toEqual(["p1", "p2"]);
    expect(reduce(state, { type: "collection-removed", id: "c1" }).collections).toEqual([]);
  });

  it("forgets a stored language only on request", () => {
    let state = reduce(INITIAL_STATE, { type: "preferences-changed", patch: { language: "de" } });
    expect(state.preferences.language).toBe("de");
    state = reduce(state, { type: "language-cleared" });
    expect("language" in state.preferences).toBe(false);
  });
});

describe("reading storage", () => {
  it("falls back to the initial state for another version, broken JSON or malformed members", () => {
    expect(sanitize({ version: 999, state: {} }, VIEWS)).toEqual(INITIAL_STATE);
    const storage = memoryStorage();
    storage.setItem(STORAGE_KEY, "{not json");
    expect(new ArchiveStore(storage, VIEWS).getSnapshot()).toEqual(INITIAL_STATE);
    const state = sanitize({ version: STORAGE_VERSION, state: { preferences: { language: "fr", theme: "neon" }, favorites: ["ok-slug", "Bad Slug", 3, "ok-slug"], collections: [{ id: 1 }], playground: { scale: -4, view: "x", placements: [{ id: "q", asset: "a-b", xM: "far" }, "nope"] } } }, VIEWS);
    expect(state.preferences).toEqual({ theme: "system", motion: "system", textSize: "normal", playful: true });
    expect(state.favorites).toEqual(["ok-slug"]);
    expect(state.collections).toEqual([]);
    expect(state.playground.scale).toBe(1);
    expect(state.playground.view).toBe("dark-line");
    expect(state.playground.placements).toEqual([{ id: "q", asset: "a-b", xM: 0, pick: null, offsetS: 0, mirrored: false }]);
  });

  it("reports volatile persistence when storage refuses writes and keeps working", () => {
    const refusing = { getItem: () => null, setItem: () => { throw new Error("quota"); }, removeItem: () => undefined };
    const store = new ArchiveStore(refusing, VIEWS);
    store.dispatch({ type: "favorite-added", asset: "a-b" });
    expect(store.mode).toBe("volatile");
    expect(store.getSnapshot().favorites).toEqual(["a-b"]);
  });
});
