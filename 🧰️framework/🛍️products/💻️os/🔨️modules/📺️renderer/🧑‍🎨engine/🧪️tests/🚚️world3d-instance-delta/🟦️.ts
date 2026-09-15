/** 🚚️ The consumer half of the world-3d instance DELTA lane: `advanceWorldInstanceResidency` must
 * substitute exactly the changed records by id, keep every untouched record's object IDENTITY (that
 * identity is what lets the downstream instanced-mesh memos skip the instances that did not move),
 * and fall back to the authoritative `instancesJson` in every case where an in-place apply could not
 * be proven safe.
 *
 * 🧫️ Every expectation is read from `🌐️World3dHost/🧫️fixtures/🚚️instance-delta.json`, the same neutral
 * declaration the Rust producer (`Puzzle3dInstanceResidency`) is pinned against, so neither side can
 * drift alone. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B44.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧱️elements/🌐️World3dHost/🧫️fixtures/🚚️instance-delta.json" with { type: "json" };
import { advanceWorldInstanceResidency, parseWorldInstanceDelta, type WorldInstanceRecord, type WorldInstanceResidencyV1 } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";

/** 🧾️ The retained set a case starts from, built the way a cold consumer builds it: straight off the
 * declared `instancesJson`, so the record objects the law checks identity on are the real ones. */
const retainedOf = (retained: (typeof fixture)["cases"][number]["retained"]): WorldInstanceResidencyV1 | null =>
  retained ? { revision: retained.revision, records: JSON.parse(retained.instancesJson) as WorldInstanceRecord[] } : null;

describe("🚚️ world-3d instance delta lane", () => {
  it("declares the lane the Rust and TypeScript lane tables both carry", () => {
    expect(fixture.field).toBe("instancesDeltaJson");
    expect(fixture.bodyKey).toBe("framework.scene.world3d.instancesDelta");
    expect(fixture.contract.authoritativeField).toBe("instancesJson");
    expect(fixture.contract.fallbackIsAlwaysCorrect).toBe(true);
  });

  for (const scenario of fixture.cases) {
    it(scenario.name, () => {
      const previous = retainedOf(scenario.retained);
      const advanced = advanceWorldInstanceResidency(previous, scenario.instancesJson, scenario.instancesDeltaJson);
      expect(advanced.revision).toBe(scenario.expect.revision);
      expect(advanced.records.map((record) => record.id)).toEqual(scenario.expect.ids);
      expect(advanced.records.map((record) => record.position)).toEqual(scenario.expect.positions);
      // 🪪️ Object identity is the whole point of the in-place path: a record the delta did not name must
      // be the SAME object the consumer already held, and a fallback must have re-parsed everything.
      const reused = advanced.records.filter((record) => previous?.records.includes(record)).map((record) => record.id);
      expect(reused).toEqual(scenario.expect.reusedIds);
      // 🪪️ Identity is only ever kept for a record whose value did not change: on the in-place path the delta did not
      // name it, on a fallback its re-parsed authoritative record serializes identically.
      const authoritative = new Map((JSON.parse(scenario.instancesJson) as WorldInstanceRecord[]).map((record) => [record.id, record]));
      for (const record of advanced.records.filter((candidate) => previous?.records.includes(candidate))) expect(record).toEqual(authoritative.get(record.id));
      if (scenario.expect.applied) expect(reused.length).toBeGreaterThan(0);
    });
  }

  it("parses only a delta that declares every key the producer publishes", () => {
    for (const key of fixture.producer.deltaKeys) {
      expect(typeof key).toBe("string");
    }
    const complete = parseWorldInstanceDelta('{"base":1,"revision":2,"count":1,"changed":[{"id":"a"}],"removed":[]}');
    expect(complete).not.toBeNull();
    expect(complete?.changed).toHaveLength(fixture.producer.oneMovedObject.changedLength);
    expect(complete?.removed).toHaveLength(fixture.producer.oneMovedObject.removedLength);
    expect((complete?.revision ?? 0) - (complete?.base ?? 0)).toBe(fixture.producer.oneMovedObject.revisionAdvancesBy);
    expect(parseWorldInstanceDelta(null)).toBeNull();
    expect(parseWorldInstanceDelta("not json")).toBeNull();
    expect(parseWorldInstanceDelta('{"revision":2}')).toBeNull();
  });

  it("never applies a delta in place when the retained count disagrees with the producer's, keeping only value-equal records", () => {
    const previous: WorldInstanceResidencyV1 = { revision: 1, records: [{ id: "a", position: [0, 0, 0] }, { id: "b" }] };
    const advanced = advanceWorldInstanceResidency(previous, '[{"id":"a","position":[1,0,0]},{"id":"b"},{"id":"c"}]', '{"base":1,"revision":2,"count":3,"changed":[{"id":"a","position":[1,0,0]}],"removed":[]}');
    expect(advanced.records.map((record) => record.id)).toEqual(["a", "b", "c"]);
    expect(advanced.records.map((record) => previous.records.includes(record))).toEqual([false, true, false]);
    expect(advanced.records[0]?.position).toEqual([1, 0, 0]);
  });
});
