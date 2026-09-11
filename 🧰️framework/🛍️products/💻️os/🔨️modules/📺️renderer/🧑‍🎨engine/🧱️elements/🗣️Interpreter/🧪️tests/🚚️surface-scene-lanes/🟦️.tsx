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
    const { leafBytes, childrenMax, packChunks } = contract.carrier;
    const records: AnyRecord[] = [];
    let level: number[] = [];
    const slices: string[] = [];
    for (let start = 0; start < payload.length || slices.length === 0; start += leafBytes) {
      slices.push(payload.slice(start, start + leafBytes));
      if (start + leafBytes >= payload.length) break;
    }
    const pack = packChunks ?? 1;
    for (let offset = 0; offset < slices.length; offset += pack) {
      const group = slices.slice(offset, offset + pack);
      const id = nextId.value++;
      const attributes: Record<string, string> = {};
      for (let index = 1; index < group.length; index += 1) attributes[String(index).padStart(2, "0")] = group[index]!;
      const leaf = textLeaf(id, `c${level.length}`, group[0]!);
      if (Object.keys(attributes).length > 0) leaf.component.dataAttributes = attributes;
      records.push(leaf);
      level.push(id);
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
      expect(arriving.has(declared.bodyKey)).toBe(false);
      const sameHash = world3dSurfaceLaneTexts(partial.record, partial.store.getState(), [{ lane: "instances", bytes: utf8ByteLength(complete), hash: "aaaaaaaaaaaaaaaa" }]);
      expect(sameHash.get(declared.bodyKey)).toBe(complete);
      surfaceSceneLaneCache.clear();
      const cold = world3dSurfaceLaneTexts(partial.record, partial.store.getState(), [{ lane: "instances", bytes: 999, hash: "bbbbbbbbbbbbbbbb" }]);
      expect(cold.has(declared.bodyKey)).toBe(false);
      console.info("[DEBUG] a successor hash that has not finished arriving is omitted; the previous complete text is kept only for the same hash");
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

    it("a completed successor hash replaces the previous lane on the same surface node", () => {
      const declared = WORLD3D_SCENE_LANES.find((entry: any) => entry.lane === "instances")!;
      const forest = "[{\"id\":\"forest-0\"}]";
      const nakagin = "[{\"id\":\"capsule-0000\"},{\"id\":\"capsule-0001\"}]";
      surfaceSceneLaneCache.clear();
      const first = surfaceWithLanes({ [declared.bodyKey]: forest });
      expect(world3dSurfaceLaneTexts(first.record, first.store.getState(), [{ lane: "instances", bytes: utf8ByteLength(forest), hash: "forest-hash000000" }]).get(declared.bodyKey)).toBe(forest);
      const next = surfaceWithLanes({ [declared.bodyKey]: nakagin });
      const collected = world3dSurfaceLaneTexts({ ...next.record, id: first.record.id }, next.store.getState(), [{ lane: "instances", bytes: utf8ByteLength(nakagin), hash: "nakagin-hash00000" }]);
      expect(collected.get(declared.bodyKey)).toBe(nakagin);
      console.info("[DEBUG] a completed successor hash replaced the previous instance lane on the same surface node");
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

    /** 🚚️ The joint neither half's laws covered: a window body arrives at the shell as a `BuiltNode`
     * (`PluginRuntime`'s `projectOwnedUiSurface` read of the retained surface) and is loaded into the
     * per-window `UiDocumentStore` by `builtNodeToSnapshot` — the exact function `ShellHost`'s
     * `createBuiltNodeStoreCacheV1` calls — before `PagedSurfaceView` ever walks a lane. The suite
     * above loads hand-minted `UiNodeRecord`s straight into the store and so never crosses that
     * projection; the intake laws in `📃️UiDocumentStore/🧪️tests/🧪️typedwire` stop at the retained
     * surface and use UNPACKED 512-byte leaves. Nakagin is the first document that packs at all:
     * measured on the real guest (`nakagin_world3d_surface_fits_reconcile_node_cap`, 2026-09-10) its
     * `instances` lane is 55 154 B in FOUR `packChunks`-slice leaves, every other lane one leaf. */
    it("projects a Nakagin-scale packed window body through builtNodeToSnapshot into 180 assembled instances", async () => {
      const { builtNodeToSnapshot } = await import("../../../📃️UiDocumentStore/🟦️.tsx");
      const { leafBytes, packChunks } = contract.carrier;
      const kinds = ["capsule_J", "capsule_L", "capsule_p", "capsule_s", "capsule_slash", "capsule_backslash", "base", "bridge", "capital", "tambour", "tambour_first-storey", "tambour_last-storey"];
      const meshIdFor = (index: number) => `mesh:🧊️${kinds[index % kinds.length]}`;
      const instances = Array.from({ length: 180 }, (_, index) => ({
        disabled: false,
        id: `0189${String(index).padStart(4, "0")}-66f2-4544-98f0-b6f0c0615492`,
        label: `Capsule With Balcony J · cs_sl${index % 13}_d0_t_f${index % 7}_b_c${index % 3}`,
        meshId: meshIdFor(index),
        objectKind: "Capsule With Balcony J",
        position: [-23.45 + (index % 17) * 1.35, -12.55 + (index % 11) * 1.05, (index % 24) * 1.6417],
        rotation: [0, 0, 0.7071067811865475, 0.7071067811865475],
        scale: [1, 1, 1],
      }));
      const laneTexts: Record<string, string> = {
        "framework.scene.world3d.meshes": JSON.stringify([{ id: "box", kind: "box" }, { id: "vortex-marker", kind: "vortex-marker" }, ...kinds.map((kind) => ({ id: `mesh:🧊️${kind}`, url: `/mesh/🧊️${kind}.glb` }))]),
        "framework.scene.world3d.instances": JSON.stringify(instances),
        "framework.scene.world3d.selection": "{\"method\":\"rectangle\",\"mode\":\"replace\",\"ids\":[],\"granularity\":\"mesh\",\"selectionMode\":\"mesh\"}",
        "framework.scene.world3d.interaction": "{\"hoveredId\":null,\"gridFactor\":1}",
        "framework.scene.world3d.lod": "{\"gridFactor\":1,\"gridSnapEnabled\":false,\"showLodGrid\":true,\"automaticLod\":true,\"depthVariableLod\":false,\"manualLod\":1}",
        "framework.scene.world3d.environment": "{\"ambient\":{\"intensity\":1.15}}",
      };
      expect(utf8ByteLength(laneTexts["framework.scene.world3d.instances"]!)).toBeGreaterThan(contract.carrier.measuredFaultBytes / 2);

      const built = (key: string, component: AnyRecord, children: readonly AnyRecord[] = []): AnyRecord => ({ key, component, layout: { kind: "leaf", width: "hug", height: "hug" }, style: TEST_STYLE, activity: "idle", disabled: false, accessibility: TEST_ACCESSIBILITY, bindings: [], menu: null, children: [...children] });
      /** 🧩️ The exact tree `paged_text_carrier` publishes: `packChunks` `leafBytes`-slices per text
       * leaf (`value` plus ascending `dataAttributes`) under a `childrenMax`-ary page tree. */
      const builtCarrier = (rootKey: string, payload: string): AnyRecord => {
        const slices: string[] = [];
        for (let start = 0; start < payload.length || slices.length === 0; start += leafBytes) slices.push(payload.slice(start, start + leafBytes));
        let level: AnyRecord[] = [];
        for (let offset = 0; offset < slices.length; offset += packChunks) {
          const group = slices.slice(offset, offset + packChunks);
          const attributes: Record<string, string> = {};
          for (let index = 1; index < group.length; index += 1) attributes[String(index).padStart(2, "0")] = group[index]!;
          level.push(built(`c${level.length}`, { type: "text", value: group[0]!, emphasize: null, dataAttributes: Object.keys(attributes).length > 0 ? attributes : null }));
        }
        for (let generation = 0; level.length > contract.carrier.childrenMax; generation += 1) {
          const paged: AnyRecord[] = [];
          for (let start = 0; start < level.length; start += contract.carrier.childrenMax) paged.push(built(`p${generation}-${paged.length}`, { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, level.slice(start, start + contract.carrier.childrenMax)));
          level = paged;
        }
        return built(rootKey, { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null }, level);
      };

      const body = built("puzzle3d-main-perspective", { type: "surface", kind: "world-3d", docSchema: contract.schema, doc: { bytes: [] }, bindings: [] }, Object.entries(laneTexts).map(([key, payload]) => builtCarrier(key, payload)));
      const store = new UiDocumentStore("window:puzzle3d-main-perspective");
      store.loadSnapshot(builtNodeToSnapshot("window:puzzle3d-main-perspective", body as any));
      const state = store.getState();
      const record = state.nodes.get(state.root!)!;
      expect(record.component.type).toBe("surface");

      const declared = Object.entries(laneTexts).map(([key, payload]) => ({ lane: world3dSceneLaneForBodyKey(key)!.lane, bytes: utf8ByteLength(payload), hash: `nakagin-${key.slice(-4)}` }));
      surfaceSceneLaneCache.clear();
      const collected = world3dSurfaceLaneTexts(record, state, declared);
      expect(Object.fromEntries(collected), "every declared lane must settle against its byte count — a leaf that drops its packed slices leaves the lane short and omitted").toEqual(laneTexts);
      const spine = { cameraJson: "{}", meshesJson: "", instancesJson: "", selectionJson: "", domainId: "puzzle3d.vortex", lanes: declared };
      const scene = world3dSceneFromLanes(spine, collected);
      const assembled = JSON.parse(scene.instancesJson) as { readonly meshId: string }[];
      expect(assembled).toHaveLength(180);
      const meshIds = new Set((JSON.parse(scene.meshesJson) as { readonly id: string }[]).map((mesh) => mesh.id));
      expect(assembled.filter((instance) => !meshIds.has(instance.meshId))).toEqual([]);

      const instancesRoot = state.nodes.get(record.children[Object.keys(laneTexts).indexOf("framework.scene.world3d.instances")]!)!;
      const leaves = (instancesRoot.children as readonly number[]).map((id: number) => state.nodes.get(id)! as AnyRecord).filter((child: AnyRecord) => child.component.type === "text");
      expect(leaves).toHaveLength(Math.ceil(Math.ceil(utf8ByteLength(laneTexts["framework.scene.world3d.instances"]!) / leafBytes) / packChunks));
      expect(leaves.every((packed: AnyRecord) => Object.keys(packed.component.dataAttributes ?? {}).length <= packChunks - 1)).toBe(true);
      console.info(`[DEBUG] a ${utf8ByteLength(laneTexts["framework.scene.world3d.instances"]!)}-byte instances lane in ${leaves.length} packed leaves projected to ${assembled.length} instances across ${collected.size} lanes`);
    });
  it("keeps a spine brush preview when no lane text arrives", () => {
      const preview = JSON.stringify({ targetVortexFullId: "seed-left-001:v0", objectKindId: "Capsule", origin: [0, 0, 0], orientation: [0, 0, 0, 1] });
      const assembled = world3dSceneFromLanes({ cameraJson: "{}", meshesJson: "", instancesJson: "", selectionJson: "{}", brushPreviewJson: preview }, new Map());
      expect(assembled.brushPreviewJson).toBe(preview);
    });

    it("does not clobber a spine brush preview with an empty leftover lane", () => {
      const preview = JSON.stringify({ targetVortexFullId: "seed-left-001:v0", objectKindId: "Capsule", origin: [0, 0, 0], orientation: [0, 0, 0, 1] });
      const declared = WORLD3D_SCENE_LANES.find((entry: any) => entry.lane === "brushPreview")!;
      expect(declared.optional).toBe(true);
      const assembled = world3dSceneFromLanes(
        { cameraJson: "{}", meshesJson: "", instancesJson: "", selectionJson: "{}", brushPreviewJson: preview },
        new Map([[declared.bodyKey, ""]]),
      );
      expect(assembled.brushPreviewJson).toBe(preview);
    });
  });
}
