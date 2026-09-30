import type { SemioTestDependencies } from "../../🟦️.ts";
import type { Vec3 } from "@semio-tech/s-3d-js";

type TestSource = { readonly url: string };

/** 🔁️ `s.stdio.semio.brep.affine-transforms/v1` — the kernel-neutral affine-transform vectors every implementation answers
 * (Rust kernel, `brep_invoke` bridge, this TS kernel, OpenCascade as the third-party oracle).
 * @see ../../../../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧫️fixtures/🔁️affine-transforms/🔣️.json */
type AffineSolid = { readonly kind: "box"; readonly width: number; readonly depth: number; readonly height: number } | { readonly kind: "sphere"; readonly radius: number } | { readonly kind: "cylinder" | "cone"; readonly radius: number; readonly height: number };
type AffineStep =
  | { readonly kind: "translate"; readonly offset: Vec3 }
  | { readonly kind: "rotate"; readonly axis: Vec3; readonly angle: number }
  | { readonly kind: "rotateAbout"; readonly origin: Vec3; readonly axis: Vec3; readonly angle: number }
  | { readonly kind: "scale"; readonly factor: number; readonly center: Vec3 }
  | { readonly kind: "mirror"; readonly origin: Vec3; readonly normal: Vec3 };
type AffineBounds = { readonly min: Vec3; readonly max: Vec3 };
type AffineCase = { readonly id: string; readonly solid: AffineSolid; readonly steps: readonly AffineStep[]; readonly expect: { readonly volume: number; readonly centerOfMass: Vec3; readonly bounds: AffineBounds; readonly faceNormals?: readonly Vec3[] } };
type AffineRefusal = { readonly id: string; readonly solid: AffineSolid; readonly step: AffineStep };
type AffineFixture = { readonly schema: "s.stdio.semio.brep.affine-transforms/v1"; readonly tessellationTolerance: number; readonly volumeRelativeTolerance: number; readonly centerOfMassTolerance: number; readonly boundsTolerance: number; readonly normalTolerance: number; readonly cases: readonly AffineCase[]; readonly refusals: readonly AffineRefusal[] };
type AffineMeasure = { readonly volume: number; readonly centerOfMass: Vec3; readonly bounds: AffineBounds; readonly faceNormals: readonly Vec3[]; readonly faceCount: number; readonly edgeCount: number };
/** 🧩️ One implementation under the vectors: build the primitive, apply one step, measure the result. */
type AffineKernelOps<S> = { readonly close?: () => Promise<void>; readonly make: (solid: AffineSolid) => Promise<S>; readonly apply: (shape: S, step: AffineStep) => Promise<S>; readonly measure: (shape: S) => Promise<AffineMeasure> };

/** 🧠️ The CAD runtime's own path: the Rust `BrepKernel` over the `brep_invoke` wire (`invokeBrep`, Semio session wasm). */
async function semioAffineOps(fixture: AffineFixture): Promise<AffineKernelOps<string>> {
  const { SemioGeometrySession } = await import("../../🌊️session/🟦️.ts");
  const session = new SemioGeometrySession();
  const invokeBrep = session.invoke.bind(session);
  const handle = async (method: string, args: Record<string, unknown>) => (await invokeBrep<{ readonly handle: string }>(method, args)).handle;
  return {
    close: () => session.close(),
    make: async ({ kind, ...args }) => handle(kind, args),
    apply: async (shape, { kind, ...args }) => handle(kind, { shape, ...args }),
    measure: async (shape) => {
      const { value: volume } = await invokeBrep<{ readonly value: number }>("volume", { shape });
      const { value: centerOfMass } = await invokeBrep<{ readonly value: Vec3 }>("centerOfMass", { shape });
      const mesh = await invokeBrep<{ readonly position: readonly number[]; readonly face_infos: readonly { readonly normal: Vec3 }[] }>("tessellate", { shape, tolerance: fixture.tessellationTolerance });
      const topology = await invokeBrep<{ readonly faces: readonly string[]; readonly edges: readonly string[] }>("deconstruct", { shape });
      const min: [number, number, number] = [Infinity, Infinity, Infinity];
      const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
      for (let index = 0; index < mesh.position.length; index += 3) {
        for (let axis = 0; axis < 3; axis++) {
          min[axis] = Math.min(min[axis]!, mesh.position[index + axis]!);
          max[axis] = Math.max(max[axis]!, mesh.position[index + axis]!);
        }
      }
      return { volume, centerOfMass, bounds: { min, max }, faceNormals: mesh.face_infos.map((info) => info.normal), faceCount: topology.faces.length, edgeCount: topology.edges.length };
    },
  };
}

const BREP_INVOKE_CATALOG_PATH = "🌊️session/";

/** 🧾️ One `brep_invoke` verb as the catalog declares it. */
type BrepInvokeVerb = { readonly method: string; readonly kernelOperation: string; readonly args: readonly { readonly name: string; readonly type: string; readonly default?: number | boolean }[]; readonly result: string; readonly label: { readonly en: string; readonly de: string } };

/** ✂️ Splits an object literal's body at its top-level commas (commas inside `[]`/`()`/`{}` belong to a value). */
function topLevelEntries(body: string): string[] {
  const entries: string[] = [];
  let depth = 0;
  let current = "";
  for (const character of body) {
    if ("[({".includes(character)) depth++;
    if ("])}".includes(character)) depth--;
    if (character === "," && depth === 0) {
      entries.push(current);
      current = "";
    } else current += character;
  }
  return [...entries, current];
}

/** 🔍️ Every `invokeBrep("<method>", { … })` call in the kernel source, with the argument keys its object literal passes. */
function brepInvokeCalls(source: string): { readonly method: string; readonly keys: readonly string[] }[] {
  return [...source.matchAll(/invokeBrep(?:<[^>]*>)?\("([A-Za-z]+)", \{([^{}]*)\}\)/g)].map((match) => ({
    method: match[1]!,
    keys: topLevelEntries(match[2]!)
      .map((entry) => entry.split(":")[0]!.trim())
      .filter((key) => key.length > 0),
  }));
}

const AFFINE_FIXTURE_PATH = "../../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧫️fixtures/🔁️affine-transforms/🔣️.json";

/** 📥️ Reads the fixture and refuses a renamed contract or an empty case list, so a drift never passes over zero vectors. */
async function affineFixture(source: TestSource): Promise<AffineFixture> {
  const { readFile } = await import("node:fs/promises");
  const root = JSON.parse(await readFile(new URL(AFFINE_FIXTURE_PATH, source.url), "utf8")) as AffineFixture;
  if (root.schema !== "s.stdio.semio.brep.affine-transforms/v1" || root.cases.length === 0 || root.refusals.length === 0) throw new Error(`affine-transforms fixture contract drifted: ${root.schema}`);
  return root;
}

/** 📏️ Every mismatch between one implementation's measurement and the fixture, as readable lines (empty = agreement). */
function affineMismatches(fixture: AffineFixture, row: AffineCase, before: AffineMeasure, after: AffineMeasure): string[] {
  const out: string[] = [];
  const near = (label: string, got: number, want: number, tolerance: number) => {
    if (!(Math.abs(got - want) <= tolerance)) out.push(`${row.id}: ${label} ${got} != ${want} (±${tolerance})`);
  };
  near("volume", after.volume, row.expect.volume, fixture.volumeRelativeTolerance * Math.abs(row.expect.volume));
  row.expect.centerOfMass.forEach((want, axis) => near(`centerOfMass[${axis}]`, after.centerOfMass[axis]!, want, fixture.centerOfMassTolerance));
  row.expect.bounds.min.forEach((want, axis) => near(`bounds.min[${axis}]`, after.bounds.min[axis]!, want, fixture.boundsTolerance));
  row.expect.bounds.max.forEach((want, axis) => near(`bounds.max[${axis}]`, after.bounds.max[axis]!, want, fixture.boundsTolerance));
  if (before.faceCount !== after.faceCount || before.edgeCount !== after.edgeCount) out.push(`${row.id}: topology ${before.faceCount}/${before.edgeCount} -> ${after.faceCount}/${after.edgeCount}`);
  if (row.expect.faceNormals) {
    const unmatched = [...after.faceNormals];
    for (const want of row.expect.faceNormals) {
      const index = unmatched.findIndex((got) => got.every((value, axis) => Math.abs(value - want[axis]!) <= fixture.normalTolerance));
      if (index < 0) out.push(`${row.id}: outward face normal ${JSON.stringify(want)} missing from ${JSON.stringify(after.faceNormals)}`);
      else unmatched.splice(index, 1);
    }
    if (unmatched.length > 0) out.push(`${row.id}: unexpected face normals ${JSON.stringify(unmatched)}`);
  }
  return out;
}

/** 🔁️ Runs every fixture vector and refusal through one implementation and returns every disagreement. */
async function affineDisagreements<S>(fixture: AffineFixture, ops: AffineKernelOps<S>): Promise<string[]> {
  const out: string[] = [];
  for (const row of fixture.cases) {
    let shape = await ops.make(row.solid);
    const before = await ops.measure(shape);
    for (const step of row.steps) shape = await ops.apply(shape, step);
    out.push(...affineMismatches(fixture, row, before, await ops.measure(shape)));
  }
  for (const refusal of fixture.refusals) {
    const shape = await ops.make(refusal.solid);
    const accepted = await ops.apply(shape, refusal.step).then(
      () => true,
      () => false,
    );
    if (accepted) out.push(`${refusal.id}: accepted a degenerate ${refusal.step.kind}`);
  }
  return out;
}

/** 🔮️ OpenCascade through the owned `brepjs` boundary — the third-party answer to the same vectors. */
async function openCascadeAffineOps(): Promise<AffineKernelOps<object>> {
  const occt = await import("../../../../../../🔌️plugins/📐️cad/⚙️engine/🧱️brepjs/🟦️.ts");
  const wasmFile = await occt.resolveOwnedOpenCascadeWasmFileUrl();
  await occt.initializeOwnedOpenCascade((path) => (path === "brepjs_single.wasm" ? wasmFile : path));
  const degrees = (radians: number) => (radians * 180) / Math.PI;
  return {
    make: async (solid) => (solid.kind === "box" ? occt.box(solid.width, solid.depth, solid.height) : solid.kind === "sphere" ? occt.sphere(solid.radius) : solid.kind === "cylinder" ? occt.cylinder(solid.radius, solid.height) : occt.cone(solid.radius, 0, solid.height)),
    apply: async (shape, step) => {
      if (step.kind === "translate") return occt.translate(shape, step.offset);
      if (step.kind === "rotate") return occt.rotate(shape, degrees(step.angle), { axis: step.axis, at: [0, 0, 0] });
      if (step.kind === "rotateAbout") return occt.rotate(shape, degrees(step.angle), { axis: step.axis, at: step.origin });
      if (step.kind === "scale") return occt.scale(shape, step.factor, { center: step.center });
      return occt.mirror(shape, { normal: step.normal, at: step.origin });
    },
    measure: async (shape) => {
      const props = occt.unwrap(occt.measureVolumeProps(shape));
      const bounds = occt.getBounds(shape);
      const faces = occt.getFaces(shape);
      return {
        volume: props.volume,
        centerOfMass: [props.centerOfMass[0], props.centerOfMass[1], props.centerOfMass[2]],
        bounds: { min: [bounds.xMin, bounds.yMin, bounds.zMin], max: [bounds.xMax, bounds.yMax, bounds.zMax] },
        faceNormals: faces.map((face) => occt.normalAt(face)),
        faceCount: faces.length,
        edgeCount: occt.getEdges(shape).length,
      };
    },
  };
}

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: SemioTestDependencies, source: TestSource): Promise<void> {
  const { SemioBrepKernel } = dependencies;

  const { beforeEach, describe, expect, it } = vitest;
  const { bootstrapCadModules } = await import("../../../../../../🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts");
  bootstrapCadModules();

  describe("@semio-tech/cad-js/spatial-kernel/semio", () => {
    const kernel = new SemioBrepKernel();

    beforeEach(async () => {
      await kernel.resetDerivedPipelineForTest();
    });

    it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
      const cell = await kernel.createBoxFromCorners({ cornerA: [0, 0, 0], cornerB: [2, 3, 0], height: 4 });
      expect(await kernel.volume(cell)).toBeCloseTo(24, 3);
    });

    it("createBoxFromCornersDiff includes one face bucket and matching FaceRef entity ids on tessellate", async () => {
      const r = await kernel.createBoxFromCornersDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 });
      expect(Object.keys(r.diff.faces?.added ?? {}).length).toBe(6);
      const mesh = await kernel.tessellate(r.solid, 1e-3);
      expect(mesh.index.length).toBeGreaterThan(0);
      const modelFaceIds = new Set((r.diff.faces?.added ?? []).map((f) => String(f.id)));
      for (const info of mesh.faceInfos) expect(modelFaceIds.has(String(info.entityId))).toBe(true);
    });

    it("solid.sphere command creates a solid with the expected volume", async () => {
      const res = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 2 });
      const added = res.diff.solids?.added?.[0];
      expect(added).toBeTruthy();
      const vol = await kernel.solidVolume(added!.id);
      expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 0);
    });

    it("solid.booleanDifference cuts a sphere out of a box", async () => {
      const box = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 5 });
      const boxId = box.diff.solids!.added![0]!.id;
      const sphere = await kernel.executeCommandDiff("solid.sphere", { center: [0, 0, 0], radius: 1 });
      const sphereId = sphere.diff.solids!.added![0]!.id;
      const res = await kernel.executeCommandDiff("solid.booleanDifference", { baseObjects: [{ id: boxId }], cutterObjects: [{ id: sphereId }] });
      const resultId = res.diff.solids?.added?.[0]?.id;
      expect(resultId).toBeTruthy();
      const vol = await kernel.solidVolume(resultId!);
      expect(vol).toBeGreaterThan(0);
      expect(vol).toBeLessThan((4 / 3) * Math.PI * 125);
    });

    it("curve.arc places start/end vertices on the requested circle", async () => {
      const res = await kernel.executeCommandDiff("curve.arc", { center: [0, 0, 0], start: [1, 0, 0], angle: 90 });
      const verts = res.diff.vertices?.added ?? [];
      expect(verts.length).toBe(2);
      expect(verts[0]!.position).toEqual([1, 0, 0]);
    });

    it("energy wall command (…From2PointsAndHeight) builds a box solid", async () => {
      const res = await kernel.executeCommandDiff("energy.energy.constructExternalWallFrom2PointsAndHeight", { pointA: [0, 0, 0], pointB: [4, 0, 0], height: 2.7 });
      expect(res.diff.solids?.added?.length).toBe(1);
      const vol = await kernel.solidVolume(res.diff.solids!.added![0]!.id);
      expect(vol).toBeGreaterThan(0);
    });
  });

  describe("@semio-tech/cad-js/spatial-kernel/semio affine transforms", () => {
    it("OpenCascade (brepjs, the third-party oracle) answers every affine-transform vector and refuses every degenerate one", async () => {
      const fixture = await affineFixture(source);
      expect(await affineDisagreements(fixture, await openCascadeAffineOps())).toEqual([]);
    });

    it("the semio brep kernel answers every affine-transform vector and refuses every degenerate one", async () => {
      const fixture = await affineFixture(source);
      const ops = await semioAffineOps(fixture);
      try { expect(await affineDisagreements(fixture, ops)).toEqual([]); } finally { await ops.close?.(); }
    });

    it("every brep_invoke call the kernel makes is a declared verb with its declared arguments", async () => {
      const { readFile } = await import("node:fs/promises");
      const { default: Ajv2020 } = await import("ajv");
      const catalog = JSON.parse(await readFile(new URL(`${BREP_INVOKE_CATALOG_PATH}🔣️.json`, source.url), "utf8")) as { readonly verbs: readonly BrepInvokeVerb[] };
      const schema = JSON.parse(await readFile(new URL(`${BREP_INVOKE_CATALOG_PATH}🧬️schema/🔣️.json`, source.url), "utf8")) as object;
      const validate = new Ajv2020({ strict: true, allErrors: true }).addKeyword("x-semio-formats").compile({ $ref: "https://json.schemas.assets.semio-tech.com/s/modules/spatial-kernel/engines/semio/session/component.json#/$defs/SemioGeometryVerbsV1", ...schema });
      expect(validate(catalog), JSON.stringify(validate.errors)).toBe(true);
      const verbs = new Map(catalog.verbs.map((verb) => [verb.method, verb]));
      expect(verbs.size).toBe(catalog.verbs.length);
      const calls = brepInvokeCalls(await readFile(new URL("🟦️.ts", source.url), "utf8"));
      expect(calls.map((call) => call.method)).toEqual(expect.arrayContaining(["translate", "rotate", "box", "tessellate"]));
      const violations = calls.flatMap(({ method, keys }) => {
        const verb = verbs.get(method);
        if (!verb) return [`${method}: not a declared brep_invoke verb`];
        const declared = new Set(verb.args.map((arg) => arg.name));
        return [...keys.filter((key) => !declared.has(key)).map((key) => `${method}: undeclared argument ${key}`), ...verb.args.filter((arg) => arg.default === undefined && !keys.includes(arg.name)).map((arg) => `${method}: missing required argument ${arg.name}`)];
      });
      expect(violations).toEqual([]);
    });
  });

}
