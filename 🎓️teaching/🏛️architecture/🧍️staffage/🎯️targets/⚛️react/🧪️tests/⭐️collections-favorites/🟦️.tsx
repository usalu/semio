/** ⭐️ Favorites and collections: persistence across stores on one storage, saved-collection events and the playground placement spacing.
 * @see ../../🔨️modules/⭐️collections/🧮️actions/🟦️.ts
 * @see ../../🔨️modules/💾️persistence/🟦️.ts
 */
import { VIEWS } from "@teaching/staffage";
import { describe, expect, it } from "vitest";
import { compareFirstFour, placeAllEvents, savedCollection, withAsset } from "../../🔨️modules/⭐️collections/🧮️actions/🟦️.ts";
import { ArchiveStore, INITIAL_STATE, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { compareFromParams } from "../../🔨️modules/🗺️routing/🟦️.ts";

const asset = (id: string, widthM: number) => ({ id, dimensions: { widthM } }) as never;

describe("favorites", () => {
  it("survive a reload through a new store on the same storage", () => {
    const storage = memoryStorage();
    const first = new ArchiveStore(storage, VIEWS);
    first.dispatch({ type: "favorite-added", asset: "cyclist" });
    first.dispatch({ type: "favorite-added", asset: "cyclist" });
    first.dispatch({ type: "favorite-added", asset: "park-bench" });
    first.dispatch({ type: "favorite-removed", asset: "park-bench" });
    expect(new ArchiveStore(storage, VIEWS).getSnapshot().favorites).toEqual(["cyclist"]);
  });
});

describe("saved collections", () => {
  it("are saved, edited and removed as events and persisted", () => {
    const storage = memoryStorage();
    const store = new ArchiveStore(storage, VIEWS);
    const created = savedCollection("  Street  ", ["a", "b", "a"], "saved-1");
    expect(created).toEqual({ id: "saved-1", name: "Street", assets: ["a", "b"] });
    store.dispatch({ type: "collection-saved", collection: created });
    store.dispatch({ type: "collection-saved", collection: withAsset(withAsset(created, "c", true), "a", false) });
    expect(new ArchiveStore(storage, VIEWS).getSnapshot().collections).toEqual([{ id: "saved-1", name: "Street", assets: ["b", "c"] }]);
    store.dispatch({ type: "collection-removed", id: "saved-1" });
    expect(new ArchiveStore(storage, VIEWS).getSnapshot().collections).toEqual([]);
  });
});

describe("curated collections", () => {
  it("place side by side separated by width plus 0.6 m and grow the ground", () => {
    let count = 0;
    const events = placeAllEvents([asset("a", 1), asset("b", 2)], INITIAL_STATE.playground, () => `p${count++}`);
    const store = new ArchiveStore(memoryStorage(), VIEWS);
    events.forEach((event) => store.dispatch(event));
    const [a, b] = store.getSnapshot().playground.placements;
    expect(a!.xM).toBeCloseTo(0.6 + 0.5, 9);
    expect(b!.xM - a!.xM).toBeCloseTo(0.5 + 0.6 + 1, 9);
    const long = placeAllEvents(Array.from({ length: 10 }, (_, index) => asset(`x${index}`, 3)), INITIAL_STATE.playground, () => "id");
    expect(long[0]).toMatchObject({ type: "playground-configured" });
  });

  it("opens the comparison with the first four assets or none for fewer than two", () => {
    expect(compareFirstFour(["a"])).toBeNull();
    expect(compareFromParams(compareFirstFour(["a", "b", "c", "d", "e"])!).panes.map((pane) => pane.asset)).toEqual(["a", "b", "c", "d"]);
  });
});
