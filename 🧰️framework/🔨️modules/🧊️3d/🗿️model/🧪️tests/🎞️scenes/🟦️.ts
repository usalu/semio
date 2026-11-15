/** 🎞️ Scenes of the kernel against the language-agnostic vector `🧫️fixtures/🎞️scenes`: schema (Ajv) and validation rules, implicit scene, the scene cut (`selectScene`, node closure, solids, materials), tessellation per scene set (shared skins, skinned groups), posing, bounds (three `Box3` is the oracle in the GLB suite), material sampling, feature edges, progress and cancellation. */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { directoryNear } from "../🧰️support/🧫️reader/🟦️.ts";
import { boundsOf, CancelledError, featureEdges, materialsOfScene, nodesOfScene, poseModel, sampleMaterials, scenesOf, selectScene, solidsOfScene, tessellate, UnknownSceneError, validateModel } from "../../🟦️.ts";
import type { ModelDocument } from "../../🧬️schema/🟦️.ts";
import { fixture, validateSchema } from "../🧰️support/🟦️.ts";

const data = fixture("🎞️scenes");
const document = data.document as ModelDocument;
const quiet = { report: () => undefined, cancelled: () => false };
const errors = (d: ModelDocument) => validateModel(d).filter((i) => i.severity === "error").map((i) => ({ code: i.code, path: i.path }));
const ids = ["body", "structure", "thermal"] as const;

describe("🎞️ scene schema and validation", () => {
  it("the vector validates against the JSON schema and the validator", () => {
    expect(validateSchema(document), JSON.stringify(validateSchema.errors)).toBe(true);
    expect(errors(document)).toEqual([]);
  });

  it.each(data.invalid as { name: string; scenes: unknown; issues: unknown }[])("$name is refused with the recorded issues", (c) => {
    expect(errors({ ...document, scenes: c.scenes } as ModelDocument)).toEqual(c.issues);
  });

  it("the schema refuses a scene without nodes, with extra members or a bad slug", () => {
    expect(validateSchema({ ...document, scenes: [{ id: "a", title: { en: "a", de: "a" }, nodes: [] }] })).toBe(false);
    expect(validateSchema({ ...document, scenes: [{ id: "a", title: { en: "a", de: "a" }, nodes: ["spine"], extra: 1 }] })).toBe(false);
    expect(validateSchema({ ...document, scenes: [{ id: "A", title: { en: "a", de: "a" }, nodes: ["spine"] }] })).toBe(false);
  });

  it("a document without scenes, or with an empty list, is one implicit scene of every carrier", () => {
    const bare = { ...document } as { scenes?: unknown };
    delete bare.scenes;
    for (const doc of [bare as ModelDocument, { ...document, scenes: [] }]) {
      expect(errors(doc)).toEqual([]);
      const scenes = scenesOf(doc);
      expect(scenes.map((s) => s.id)).toEqual(["scenes"]);
      expect(scenes[0]!.nodes).toEqual(["hull-body", "hull-arm", "frame-body", "frame-arm", "skin-shared", "region-a", "skin-thermal"]);
      expect(selectScene(doc, "scenes")).toBe(doc);
      expect(solidsOfScene(doc, "scenes")).toEqual(document.solids.map((s) => s.id));
    }
    expect(() => selectScene(document, "nope")).toThrow(UnknownSceneError);
  });
});

describe("🎞️ the scene cut", () => {
  it.each(ids)("%s: node closure, solid ids and a valid self-contained document", (id) => {
    expect([...nodesOfScene(document, id)].sort()).toEqual([...data.expected.closure[id]].sort());
    expect(solidsOfScene(document, id)).toEqual(data.expected.solids[id]);
    const cut = selectScene(document, id);
    expect(cut.scenes!.map((s) => s.id)).toEqual([id]);
    expect(cut.nodes.map((n) => n.id)).toEqual(document.nodes.filter((n) => data.expected.closure[id].includes(n.id)).map((n) => n.id));
    expect(cut.solids.map((s) => s.id)).toEqual(data.expected.solids[id]);
    expect(errors(cut)).toEqual([]);
    expect(validateSchema(cut)).toBe(true);
  });

  it("keeps the shared skeleton, animation channels of its nodes and filters poses", () => {
    const cut = selectScene(document, "thermal");
    expect(cut.animations[0]!.channels.length).toBe(2);
    expect(cut.poses[0]!.nodes).toHaveProperty("arm");
  });

  it("materials of a scene include the variant targets of what it draws", () => {
    expect([...materialsOfScene(document, "body")].sort()).toEqual(["hull", "hull-dark", "skin"]);
    expect([...materialsOfScene(document, "structure")].sort()).toEqual(["frame", "skin"]);
    expect([...materialsOfScene(document, "thermal")]).toEqual(["heat"]);
  });
});

describe("🎞️ tessellation per scene", () => {
  it("joins every skinned group to the scenes of its node and shares one skin", async () => {
    const resolved = await tessellate(document, data.quality, quiet);
    expect(resolved.solids).toEqual([]);
    expect(Object.fromEntries(resolved.scenes!.map((s) => [s.id, s.nodes]))).toEqual(data.expected.resolvedSceneNodes);
    expect(resolved.nodes.filter((n) => n.skin).map((n) => n.id)).toEqual(data.expected.skinnedNodes);
    expect(resolved.skins.length).toBe(1);
    expect(errors(resolved)).toEqual([]);
    expect(validateSchema(resolved), JSON.stringify(validateSchema.errors)).toBe(true);
  });

  it.each(ids)("%s: tessellating the scene alone equals the scene of the whole", async (id) => {
    const whole = await tessellate(document, data.quality, quiet);
    const alone = await tessellate(document, data.quality, quiet, { scene: id });
    expect(alone.scenes!.map((s) => s.id)).toEqual([id]);
    expect(errors(alone)).toEqual([]);
    const drawn = (d: ModelDocument) => d.nodes.filter((n) => n.mesh).flatMap((n) => d.meshes.find((m) => m.id === n.mesh)!.primitives.flatMap((p) => (p.solids ?? []).map((r) => r.id))).sort();
    expect(drawn(alone)).toEqual([...data.expected.solids[id]].sort());
    const sceneOfWhole = selectScene(whole, id);
    expect(drawn(sceneOfWhole)).toEqual(drawn(alone));
    const positionsOf = (d: ModelDocument, solid: string) => d.meshes.flatMap((m) => m.primitives).filter((p) => p.solids?.some((r) => r.id === solid)).map((p) => p.positions.length);
    for (const solid of data.expected.solids[id].filter((s: string) => !s.endsWith("skin"))) expect(positionsOf(alone, solid).length).toBeGreaterThan(0);
  });

  it("is deterministic, reports progress and rejects when cancelled", async () => {
    const a = JSON.stringify(await tessellate(document, data.quality, quiet));
    expect(JSON.stringify(await tessellate(document, data.quality, quiet))).toBe(a);
    const seen: number[] = [];
    await tessellate(document, data.quality, { report: (f) => seen.push(f), cancelled: () => false }, { scene: "body" });
    expect(seen.at(-1)).toBe(1);
    expect(seen).toEqual([...seen].sort((x, y) => x - y));
    await expect(tessellate(document, data.quality, { report: () => undefined, cancelled: () => true })).rejects.toBeInstanceOf(CancelledError);
  });

  it("a document without scenes keeps the single skinned node", async () => {
    const bare = { ...document } as { scenes?: unknown };
    delete bare.scenes;
    const resolved = await tessellate(bare as ModelDocument, data.quality, quiet);
    expect(resolved.nodes.filter((n) => n.skin).map((n) => n.id)).toEqual(["skinned"]);
    expect("scenes" in resolved).toBe(false);
  });
});

describe("🎞️ pose, material sampling, bounds and edges per scene", () => {
  it.each(ids)("%s: poseModel poses exactly the closure, with the matrices of the whole", (id) => {
    const whole = poseModel(document, { animation: "wave", timeS: 0.25 });
    const part = poseModel(document, { animation: "wave", timeS: 0.25 }, { scene: id });
    expect(Object.keys(part.world).sort()).toEqual([...data.expected.closure[id]].sort());
    for (const [node, m] of Object.entries(part.world)) expect(m).toEqual(whole.world[node]);
    expect(poseModel(document, "bent", { scene: id }).issues).toEqual([]);
  });

  it("samples only the colour channels of the materials a scene draws", () => {
    expect(Object.keys(sampleMaterials(document, "wave", 0.5))).toEqual(["hull"]);
    expect(Object.keys(sampleMaterials(document, "wave", 0.5, { scene: "body" }))).toEqual(["hull"]);
    expect(sampleMaterials(document, "wave", 0.5, { scene: "structure" })).toEqual({});
    expect(sampleMaterials(document, "wave", 0.5, { scene: "thermal" })).toEqual({});
  });

  it.each(ids)("%s: bounds at rest and bent equal the hand-derived vector", async (id) => {
    const resolved = await tessellate(document, data.quality, quiet);
    for (const [pose, key] of [[undefined, "rest"], ["bent", "bent"]] as const) {
      const b = boundsOf(resolved, pose, { scene: id })!;
      for (let k = 0; k < 3; k++) {
        expect(b.min[k]!).toBeCloseTo(data.expected.bounds[id][key].min[k], 6);
        expect(b.max[k]!).toBeCloseTo(data.expected.bounds[id][key].max[k], 6);
      }
    }
    expect(() => boundsOf(document)).toThrow();
  });

  it("the bounds of all scenes together are the union of the scenes", async () => {
    const resolved = await tessellate(document, data.quality, quiet);
    const whole = boundsOf(resolved)!;
    for (let k = 0; k < 3; k++) {
      expect(whole.min[k]!).toBeCloseTo(Math.min(...ids.map((id) => boundsOf(resolved, undefined, { scene: id })!.min[k]!)), 9);
      expect(whole.max[k]!).toBeCloseTo(Math.max(...ids.map((id) => boundsOf(resolved, undefined, { scene: id })!.max[k]!)), 9);
    }
  });

  it("feature edges of a scene touch only its meshes", async () => {
    const resolved = await tessellate(document, data.quality, quiet);
    const all = await featureEdges(resolved, Math.PI / 6, {}, quiet);
    const part = await featureEdges(resolved, Math.PI / 6, { scene: "thermal" }, quiet);
    expect(part.scenes!.map((s) => s.id)).toEqual(["thermal"]);
    const lines = (d: ModelDocument) => Object.fromEntries(d.meshes.map((m) => [m.id, m.primitives.filter((p) => p.mode === "lines").map((p) => p.indices.length)]));
    const mine = lines(part);
    expect(Object.keys(mine).sort()).toEqual(part.meshes.map((m) => m.id).sort());
    for (const [mesh, counts] of Object.entries(mine)) expect(counts).toEqual(lines(all)[mesh]);
    expect(part.meshes.some((m) => m.id === "hull-body")).toBe(false);
    await expect(featureEdges(resolved, 0.5, { scene: "thermal" }, { report: () => undefined, cancelled: () => true })).rejects.toBeInstanceOf(CancelledError);
  });
});

describe("🎯️ identical bytes under bun and node", () => {
  it("no kernel, encoder or viewer source outside the portable module and the tests calls an engine-dependent Math function", () => {
    const root = directoryNear(import.meta.url, "../..");
    const files: string[] = [];
    const walk = (dir: string): void => {
      for (const name of readdirSync(dir)) {
        const path = join(dir, name);
        if (name === "node_modules" || name.includes("tests") || name === "🎯️portable") continue;
        if (statSync(path).isDirectory()) walk(path);
        else if (name.endsWith(".ts")) files.push(path);
      }
    };
    walk(root);
    expect(files.length).toBeGreaterThan(40);
    const banned = /Math\.(sin|cos|tan|atan2?|acos|asin|exp|expm1|log\w*|pow|cbrt|hypot|sinh|cosh|tanh)\b/u;
    expect(files.filter((f) => banned.test(readFileSync(f, "utf8")))).toEqual([]);
  });
});
