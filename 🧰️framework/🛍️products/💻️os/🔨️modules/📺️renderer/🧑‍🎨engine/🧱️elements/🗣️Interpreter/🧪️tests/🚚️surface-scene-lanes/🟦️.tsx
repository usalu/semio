type TestSource = { readonly url: string };

/** 🚚️ The React half of the world-3d paged scene carrier's laws, read from the SAME language-neutral
 * declaration the Rust producer is pinned against
 * (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`). */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { UiDocumentStore, surfaceSceneLaneCache, utf8ByteLength, world3dSurfaceLaneTexts, world3dSceneFromLanes, WORLD3D_SCENE_LANES, WORLD3D_SCENE_LANE_KEY_PREFIX, world3dSceneLaneForBodyKey } = dependencies;
  const { describe, expect, it } = vitest;
  void source;

  const { default: contract } = await import("../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json");

  type AnyRecord = Record<string, any>;

  const TEST_STYLE = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
  const TEST_ACCESSIBILITY = { label: null, description: null, live: "off", shortcut: null, hidden: false };

  function node(id: number, key: string, component: AnyRecord, children: readonly number[] = []): AnyRecord {
    return { id, key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, transition: null, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: [...children] };
  }

  function container(id: number, key: string, children: readonly number[]): AnyRecord {
    return node(id, key, { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, children);
  }

  function textLeaf(id: number, key: string, value: string): AnyRecord {
    return node(id, key, { type: "text", value, emphasize: null, dataAttributes: null });
  }

  /** 🧩️ Rebuilds the exact carrier the Rust `paged_text_carrier` publishes: `leafBytes`-capped text
   * leaves under a balanced `childrenMax`-ary tree of page containers, rooted at the lane's key. */
  function carrier(nextId: { value: number }, rootKey: string, payload: string): { readonly records: AnyRecord[]; readonly rootId: number } {
    const { leafBytes, childrenMax } = contract.carrier;
    const records: AnyRecord[] = [];
    let level: number[] = [];
    for (let start = 0; start < payload.length || level.length === 0; start += leafBytes) {
      const id = nextId.value++;
      records.push(textLeaf(id, `c${level.length}`, payload.slice(start, start + leafBytes)));
      level.push(id);
      if (start + leafBytes >= payload.length) break;
    }
    let generation = 0;
    while (level.length > childrenMax) {
      const next: number[] = [];
      for (let start = 0; start < level.length; start += childrenMax) {
        const id = nextId.value++;
        records.push(container(id, `p${generation}-${next.length}`, level.slice(start, start + childrenMax)));
        next.push(id);
      }
      level = next;
      generation += 1;
    }
    const rootId = nextId.value++;
    records.push(container(rootId, rootKey, level));
    return { records, rootId };
  }

  /** 🗺️ One world-3d surface node with a carrier per lane, loaded into a real `UiDocumentStore`. */
  function surfaceWithLanes(laneTexts: Readonly<Record<string, string>>): { readonly store: any; readonly record: AnyRecord } {
    const nextId = { value: 1 };
    const records: AnyRecord[] = [];
    const children: number[] = [];
    for (const [key, payload] of Object.entries(laneTexts)) {
      const built = carrier(nextId, key, payload);
      records.push(...built.records);
      children.push(built.rootId);
    }
    const surface = node(0, "viewport", { type: "surface", kind: "world-3d", docSchema: contract.schema, doc: { bytes: [] }, bindings: [] }, children);
    const store = new UiDocumentStore("s");
    store.loadSnapshot({ surface: "s", revision: 0, root: 0, nodes: [surface, ...records], layoutEpoch: 0n });
    return { store, record: surface };
  }

  describe("world-3d paged scene carrier", () => {
    it("mirrors the language-neutral lane declaration exactly", () => {
      expect(WORLD3D_SCENE_LANE_KEY_PREFIX).toBe(contract.laneKeyPrefix);
      expect(WORLD3D_SCENE_LANES).toEqual(contract.lanes);
      for (const lane of contract.lanes) {
        expect(lane.bodyKey).toBe(`${contract.laneKeyPrefix}${lane.lane}`);
        expect(world3dSceneLaneForBodyKey(lane.bodyKey)).toEqual(lane);
      }
      expect(world3dSceneLaneForBodyKey("puzzle3d-main-top")).toBeUndefined();
      console.info(`[DEBUG] ${WORLD3D_SCENE_LANES.length} world-3d scene lanes pinned against the neutral declaration`);
    });

    it("reassembles a spine and its lane carriers back into the scene the host reads", () => {
      const { spine, laneTexts, assembled } = contract.roundTrip;
      const collected = new Map<string, string>(Object.entries(laneTexts));
      expect(world3dSceneFromLanes(spine, collected)).toEqual({ ...assembled, lanes: spine.lanes });
      console.info(`[DEBUG] round-tripped ${collected.size} lanes back onto the spine`);
    });

    it("walks each lane's retained carrier back to its exact payload", () => {
      const { spine, laneTexts, assembled } = contract.roundTrip;
      surfaceSceneLaneCache.clear();
      const { store, record } = surfaceWithLanes(laneTexts);
      const collected = world3dSurfaceLaneTexts(record, store.getState(), spine.lanes);
      expect(Object.fromEntries(collected)).toEqual(laneTexts);
      expect(world3dSceneFromLanes(spine, collected)).toEqual({ ...assembled, lanes: spine.lanes });
      console.info(`[DEBUG] ${collected.size} lane carriers walked out of the retained document`);
    });

    it("pages one oversized lane instead of faulting, and still reproduces it byte-exactly", () => {
      const { lane, unit, repeat, payloadBytes, expectedLeaves, expectedLeafDepth } = contract.paging;
      const payload = unit.repeat(repeat);
      expect(utf8ByteLength(payload)).toBe(payloadBytes);
      const declared = WORLD3D_SCENE_LANES.find((entry: any) => entry.lane === lane)!;
      surfaceSceneLaneCache.clear();
      const { store, record } = surfaceWithLanes({ [declared.bodyKey]: payload });
      const state = store.getState();
      const root = state.nodes.get(record.children[0]);
      let leaves = 0;
      let leafDepth = 0;
      const walk = (id: number, depth: number): void => {
        const current = state.nodes.get(id);
        if (!current) return;
        expect(current.children.length).toBeLessThanOrEqual(contract.carrier.childrenMax);
        if (current.component.type === "text") {
          leaves += 1;
          leafDepth = Math.max(leafDepth, depth);
          expect(utf8ByteLength(current.component.value)).toBeLessThanOrEqual(contract.carrier.leafBytes);
        }
        for (const child of current.children) walk(child, depth + 1);
      };
      for (const child of root.children) walk(child, 1);
      expect(leaves).toBe(expectedLeaves);
      expect(leafDepth).toBe(expectedLeafDepth);
      const collected = world3dSurfaceLaneTexts(record, state, [{ lane, bytes: payloadBytes, hash: "0000000000000000" }]);
      expect(collected.get(declared.bodyKey)).toBe(payload);
      console.info(`[DEBUG] oversized ${lane} lane paged into ${leaves} leaves at depth ${leafDepth}, ${payloadBytes} bytes reproduced`);
    });

    it("keeps the last complete text for a lane that has not finished arriving", () => {
      const declared = WORLD3D_SCENE_LANES.find((entry: any) => entry.lane === "instances")!;
      const complete = "[{\"id\":\"a\"},{\"id\":\"b\"}]";
      surfaceSceneLaneCache.clear();
      const first = surfaceWithLanes({ [declared.bodyKey]: complete });
      const settled = world3dSurfaceLaneTexts(first.record, first.store.getState(), [{ lane: "instances", bytes: utf8ByteLength(complete), hash: "aaaaaaaaaaaaaaaa" }]);
      expect(settled.get(declared.bodyKey)).toBe(complete);
      const partial = surfaceWithLanes({ [declared.bodyKey]: "[{\"id\":\"a\"}" });
      const arriving = world3dSurfaceLaneTexts(partial.record, partial.store.getState(), [{ lane: "instances", bytes: 999, hash: "bbbbbbbbbbbbbbbb" }]);
      expect(arriving.get(declared.bodyKey)).toBe(complete);
      surfaceSceneLaneCache.clear();
      const cold = world3dSurfaceLaneTexts(partial.record, partial.store.getState(), [{ lane: "instances", bytes: 999, hash: "bbbbbbbbbbbbbbbb" }]);
      expect(cold.has(declared.bodyKey)).toBe(false);
      console.info("[DEBUG] a partially arrived lane falls back to its last complete text, and is omitted when there is none");
    });

    it("reassembles a Nakagin-scale multi-lane carrier without dropping a leaf", () => {
      const instances = Array.from(
        { length: 720 },
        (_, index) => `{"id":"capsule-${String(index).padStart(4, "0")}","meshId":"box","position":[${index},0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}`,
      ).join(",");
      const vortices = Array.from({ length: 80 }, (_, index) => `{"id":"vortex-${index}","position":[0,${index},0],"strength":1}`).join(",");
      const laneTexts: Record<string, string> = {
        "framework.scene.world3d.meshes": "[{\"id\":\"box\",\"kind\":\"box\"}]",
        "framework.scene.world3d.instances": `[${instances}]`,
        "framework.scene.world3d.selection": "{\"method\":\"rectangle\",\"mode\":\"replace\",\"ids\":[],\"hoveredId\":null}",
        "framework.scene.world3d.vortices": `[${vortices}]`,
        "framework.scene.world3d.interaction": "{\"hoveredId\":null}",
      };
      const total = Object.values(laneTexts).reduce((sum, text) => sum + utf8ByteLength(text), 0);
      expect(total).toBeGreaterThan(contract.carrier.measuredFaultBytes);
      surfaceSceneLaneCache.clear();
      const { store, record } = surfaceWithLanes(laneTexts);
      const declared = Object.entries(laneTexts).map(([key, payload]) => {
        const lane = world3dSceneLaneForBodyKey(key)!;
        return { lane: lane.lane, bytes: utf8ByteLength(payload), hash: "nakagin-scale" };
      });
      const collected = world3dSurfaceLaneTexts(record, store.getState(), declared);
      expect(Object.fromEntries(collected)).toEqual(laneTexts);
      const spine = { cameraJson: "{}", meshesJson: "", instancesJson: "", selectionJson: "", lanes: declared };
      expect(world3dSceneFromLanes(spine, collected).instancesJson).toBe(laneTexts["framework.scene.world3d.instances"]);
      console.info(`[DEBUG] Nakagin-scale ${total} lane bytes reassembled from ${collected.size} carriers`);
    });

    it("a hover-only interaction change republishes only that lane", () => {
      const { spine, laneTexts, assembled } = contract.roundTrip;
      const hovered: Record<string, string> = { ...laneTexts, "framework.scene.world3d.interaction": "{\"hoveredId\":\"capsule-0001\"}" };
      const collected = new Map<string, string>(Object.entries(hovered));
      const next = world3dSceneFromLanes(spine, collected);
      expect(next.interactionJson).toBe(hovered["framework.scene.world3d.interaction"]);
      expect(next.instancesJson).toBe(assembled.instancesJson);
      expect(next.meshesJson).toBe(assembled.meshesJson);
      expect(next.selectionJson).toBe(assembled.selectionJson);
      const changed = Object.keys(hovered).filter((key) => hovered[key] !== (laneTexts as Record<string, string>)[key]);
      expect(changed).toEqual(["framework.scene.world3d.interaction"]);
      console.info("[DEBUG] hover republished only the interaction lane");
    });

    it("counts lane bytes in UTF-8 so a non-ASCII payload still settles", () => {
      const payload = "[\"Nakagin Kapselturm — Grundriss\",\"日本\"]";
      const bytes = new TextEncoder().encode(payload).length;
      expect(utf8ByteLength(payload)).toBe(bytes);
      expect(utf8ByteLength("🧩")).toBe(4);
      const declared = WORLD3D_SCENE_LANES.find((entry: any) => entry.lane === "references")!;
      surfaceSceneLaneCache.clear();
      const { store, record } = surfaceWithLanes({ [declared.bodyKey]: payload });
      const collected = world3dSurfaceLaneTexts(record, store.getState(), [{ lane: "references", bytes, hash: "cccccccccccccccc" }]);
      expect(collected.get(declared.bodyKey)).toBe(payload);
      console.info(`[DEBUG] a ${bytes}-byte UTF-8 lane payload settled against its declared byte length`);
    });
  });
}
