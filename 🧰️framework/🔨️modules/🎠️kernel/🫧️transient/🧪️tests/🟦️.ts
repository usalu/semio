import { describe, expect, it, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020.js";
import { createRequire } from "node:module";

type ReferenceCache<K, V> = { has(key: K): boolean; get(key: K): V | undefined; set(key: K, value: V): void; reset(): void };
const LRUCache = createRequire(import.meta.url)("lru-cache") as new<K, V>(options: { max: number }) => ReferenceCache<K, V>;
import { build } from "esbuild";
import { TransientStore, ephemeralBox } from "../🟦️.ts";

interface Store {
  box<T>(key: string, init: T): { current: T };
  map<K, V>(key: string): Map<K, V>;
  set<T>(key: string): Set<T>;
  weakMap<K extends object, V>(key: string): WeakMap<K, V>;
  reset(): void;
}
type Provider = Readonly<{ create: () => Store; shared: Store; box: Store["box"]; map: Store["map"]; set: Store["set"]; weakMap: Store["weakMap"] }>;
const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8")) as { cases: readonly { id: string; expected: unknown }[] };
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));

class ReferenceStore implements Store {
  private readonly boxes = new LRUCache<string, { current: unknown }>({ max: 128 });
  private readonly maps = new LRUCache<string, Map<unknown, unknown>>({ max: 128 });
  private readonly sets = new LRUCache<string, Set<unknown>>({ max: 128 });
  private readonly weakMaps = new LRUCache<string, WeakMap<object, unknown>>({ max: 128 });
  box<T>(key: string, init: T): { current: T } {
    if (!this.boxes.has(key)) this.boxes.set(key, { current: init });
    return this.boxes.get(key)! as { current: T };
  }
  map<K, V>(key: string): Map<K, V> {
    if (!this.maps.has(key)) this.maps.set(key, new Map());
    return this.maps.get(key)! as Map<K, V>;
  }
  set<T>(key: string): Set<T> {
    if (!this.sets.has(key)) this.sets.set(key, new Set());
    return this.sets.get(key)! as Set<T>;
  }
  weakMap<K extends object, V>(key: string): WeakMap<K, V> {
    if (!this.weakMaps.has(key)) this.weakMaps.set(key, new WeakMap());
    return this.weakMaps.get(key)! as WeakMap<K, V>;
  }
  reset(): void { this.boxes.reset(); this.maps.reset(); this.sets.reset(); this.weakMaps.reset(); }
}

function observe(id: string, provider: Provider): unknown {
  const left = provider.create(), right = provider.create(), key = "same", object = {};
  switch (id) {
    case "keyed-box": {
      const box = left.box(key, 1); box.current = 2;
      return { same: box === left.box(key, 99), current: left.box(key, 99).current };
    }
    case "function-init": {
      let calls = 0;
      const identity = (value: string): string => { calls++; return value; }, box = left.box(key, identity), before = calls, result = box.current("ui.nav.back");
      return { same: box.current === identity, before, after: calls, result };
    }
    case "collection-keyspaces": {
      const box = left.box(key, 1), map = left.map<string, number>(key), set = left.set<string>(key), weak = left.weakMap<object, number>(key);
      map.set("value", 2); set.add("value"); weak.set(object, 4);
      return { box: box.current, map: map.get("value"), set: set.has("value"), weak: weak.get(object), reused: [box === left.box(key, 99), map === left.map(key), set === left.set(key), weak === left.weakMap(key)] };
    }
    case "reset": {
      const box = left.box(key, 1), map = left.map<string, number>(key), set = left.set<string>(key), weak = left.weakMap<object, number>(key);
      map.set("value", 2); set.add("value"); weak.set(object, 4); left.reset();
      return { replaced: [box !== left.box(key, 9), map !== left.map(key), set !== left.set(key), weak !== left.weakMap(key)], old: [box.current, map.get("value"), set.has("value"), weak.get(object)], fresh: [left.box(key, 9).current, left.map(key).size, left.set(key).size, left.weakMap(key).has(object)] };
    }
    case "isolation": {
      const box = left.box(key, 1), other = right.box(key, 2); box.current = 11;
      left.map(key).set("value", 1); left.set(key).add("value"); left.weakMap(key).set(object, 1);
      return { left: box.current, right: other.current, same: box === other, maps: [left.map(key).size, right.map(key).size], sets: [left.set(key).size, right.set(key).size], weak: [left.weakMap(key).has(object), right.weakMap(key).has(object)] };
    }
    case "default-helpers": {
      provider.shared.reset();
      return { shared: [provider.box(key, 1) === provider.shared.box(key, 2), provider.map(key) === provider.shared.map(key), provider.set(key) === provider.shared.set(key), provider.weakMap(key) === provider.shared.weakMap(key)] };
    }
    case "key-identities": {
      const values = [left.box("", 1), left.box("\0", 2), left.box("🌱️", 3)];
      return { values: values.map(box => box.current), distinct: [values[0] !== values[1], values[1] !== values[2], values[0] !== values[2]] };
    }
    case "retained-function-init": {
      const shared = provider.shared, identity = (value: string): string => value, box = shared.box("retained-function", identity);
      return { type: typeof box.current, value: box.current("ui.nav.back") };
    }
    case "retained-noop-init": {
      let calls = 0;
      const box = provider.shared.box("retained-noop", () => { calls++; }), type = typeof box.current, before = calls;
      box.current();
      return { type, before, after: calls };
    }
    case "retained-isolation-reset": {
      const box = left.box("cursor", { x: 1 }); box.current.x = 2;
      const same = left.box("cursor", { x: 99 }) === box, rightValue = right.box("cursor", { x: 3 }).current.x, old = left.map<string, number>("measurements"); old.set("width", 42); left.reset();
      return { same, rightValue, replaced: left.map("measurements") !== old, freshSize: left.map("measurements").size, retainedWidth: old.get("width") };
    }
    default: throw new Error(`Unowned transient case: ${id}`);
  }
}

test("transient identity vectors are closed and independently reproduced", () => {
  const validate = new Ajv({ strict: true }).compile(schema), shared = new ReferenceStore();
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(10);
  const provider: Provider = { create: () => new ReferenceStore(), shared, box: shared.box.bind(shared), map: shared.map.bind(shared), set: shared.set.bind(shared), weakMap: shared.weakMap.bind(shared) };
  for (const row of fixture.cases) expect(observe(row.id, provider), row.id).toEqual(row.expected);
});

test("the canonical transient leaf preserves every identity and owns no external runtime", async () => {
  const owner = await import("../🟦️.ts");
  const provider: Provider = { create: () => new owner.TransientStore(), shared: owner.defaultTransientStore, box: owner.ephemeralBox, map: owner.ephemeralMap, set: owner.ephemeralSet, weakMap: owner.ephemeralWeakMap };
  for (const row of fixture.cases) expect(observe(row.id, provider), row.id).toEqual(row.expected);
  const result = await build({ entryPoints: [fileURLToPath(new URL("../🟦️.ts", import.meta.url))], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
  expect(Object.keys(result.metafile!.inputs)).toHaveLength(1);
  expect(Object.values(result.metafile!.outputs).flatMap(output => output.imports)).toEqual([]);
});

  describe("ephemeralBox", () => {
    it("stores a function-typed init as the current value (not as a lazy factory)", () => {
      const identity = (id: string) => id;
      const box = ephemeralBox<(id: string) => string>(`test.ephemeralBox.fn.${Math.random()}`, identity);
      expect(typeof box.current).toBe("function");
      expect(box.current("ui.nav.back")).toBe("ui.nav.back");
    });

    it("stores a no-op function init without invoking it", () => {
      let calls = 0;
      const noop = () => {
        calls += 1;
      };
      const box = ephemeralBox<() => void>(`test.ephemeralBox.noop.${Math.random()}`, noop);
      expect(typeof box.current).toBe("function");
      expect(calls).toBe(0);
      box.current();
      expect(calls).toBe(1);
    });

    it("is owned by an isolatable, resettable TransientStore lane", () => {
      const left = new TransientStore();
      const right = new TransientStore();
      const leftBox = left.box("cursor", { x: 1 });
      leftBox.current.x = 2;
      expect(left.box("cursor", { x: 99 })).toBe(leftBox);
      expect(right.box("cursor", { x: 3 }).current.x).toBe(3);

      const oldMap = left.map<string, number>("measurements");
      oldMap.set("width", 42);
      left.reset();
      expect(left.map<string, number>("measurements")).not.toBe(oldMap);
      expect(left.map<string, number>("measurements").size).toBe(0);
      expect(oldMap.get("width")).toBe(42);
    });
  });

