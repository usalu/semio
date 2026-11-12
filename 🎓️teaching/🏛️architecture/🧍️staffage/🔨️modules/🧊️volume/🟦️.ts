/** 🧊️ The 3D volume of a staffage asset as a neutral `semio.model/v1` model.
 *
 * `modelOf(asset, options, operation)` builds the procedural {@link ModelDocument} of an asset: the nodes are the rig bones at rest in the neutral frame (right-handed, +Y up, +Z ahead, +X left, metres, radians, quaternions) with the joints as mechanism, the solids are the `asset.volume` solids carried from the bone-local `(u, v, w)` frame (see ./🧊️solids),
 * the poses are the asset poses and the animations the asset clips sampled with `evaluate`, and the variants are the seven views of the 2D depiction as looks of one geometry (see ./🎨️looks, ./🌡️thermal, ./🏗️structure, ./🪧️glyphs).
 * One source of truth: bone frames, thermal fields, structural results and emitter states come from `evaluate`, `solveThermalAt`, `solveStructure` and `solveTransport`, and the colours from the shared `thermalPaint` and `memberPaint` of the 2D views; nothing is solved twice or differently.
 *
 * Progress and cancellation flow through the kernel `Operation`: every evaluated frame reports and checks, and the work yields to the event loop between frames.
 *
 * @see ../../🧬️schema/🟦️.ts — `Volume`
 * @see ../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts — the neutral kernel
 */
import type { Asset, Slug, Text } from "../../🧬️schema/🟦️.ts";
import { CancelledError, silentOperation, type Animation, type Channel, type Material, type ModelDocument, type Node, type Operation, type Pose, type Solid, type TransformPatch } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { VIEWS, type DisplayRange, type EvaluatedState, type EvaluationQuery, type ForceQuantity, type ThermalQuantity, type View } from "../🔗️contracts/🟦️.ts";
import { THEMES, type Theme } from "../🎨️depiction/🖍️palette/🟦️.ts";
import { VIEW_TITLES } from "../🎨️depiction/🟦️.ts";
import { eligibleBones, longestArrowMm } from "../🎨️depiction/🏗️structural-view/🟦️.ts";
import { solveStructure } from "../🏗️structure/🟦️.ts";
import { solveThermalAt, thermalSession, type EnvironmentInput } from "../🌡️thermal/🟦️.ts";
import { solveTransport } from "../💨️transport/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { type Local, animationOf, colourChannel, frameTimes, nodeChannels } from "./🎞️clips/🟦️.ts";
import { type LookSpec, type Paint, type Surface, baseMaterial, linearOf, lineLook, materialIds, materialOf, solidLook, surfacesOf, variantOf } from "./🎨️looks/🟦️.ts";
import { type Triple } from "./🧭️frame/🟦️.ts";
import { type Analytic, type FrameData, type FrameSetup, Slots, frameDataOf, glyphOf, glyphPaintOf } from "./🧷️frames/🟦️.ts";
import { ModelRequestError } from "./🚫️refusal/🟦️.ts";
import { HIDDEN, glyphEdgeOf } from "./🪧️glyphs/🟦️.ts";
import { solidOf, shownSolids } from "./🧊️solids/🟦️.ts";
import { skeletonOf, placementsOf } from "./🦴️skeleton/🟦️.ts";
import { memberLook } from "./🏗️structure/🟦️.ts";
import { thermalLook } from "./🌡️thermal/🟦️.ts";

export { ModelRequestError, MODEL_REFUSALS, type ModelRefusal } from "./🚫️refusal/🟦️.ts";
export { frameTimes, eventsOf } from "./🎞️clips/🟦️.ts";
export { regionsOfSolid, shownAt, shownSolids, solidOf } from "./🧊️solids/🟦️.ts";
export { toNeutral, pseudoToNeutral, basisOf, quaternionOf } from "./🧭️frame/🟦️.ts";
export { skeletonOf, restPlacements, placementsOf, outOfPlaneBones, jointOf } from "./🦴️skeleton/🟦️.ts";
export { hexOf, linearOf, surfacesOf, type Surface, type Paint } from "./🎨️looks/🟦️.ts";
export { edgeMaterialOf, glyphEdgeOf } from "./🪧️glyphs/🟦️.ts";

/** ⚙️ What one model contains.
 * - `scale`: the preset or custom denominator (1:`scale`) the model is made for: it filters the solids by `maxDenominator` and sizes the glyphs in paper millimetres.
 * - `at`: the static state of the document (colours of the analytic variants, glyph placements): a pose or a clip instant; default the first named pose.
 * - `poses`, `clips`: which named poses and clips become poses and animations; default every one (`"all"`), `[]` none.
 * - `views`: which of the seven views become variants; default all.
 * - `environment`, `thermalSpanK`: the environment of the thermal view (a preset environment or a timeline) and the span ±K around the air temperature of its scale (absent: the shared absolute range), as in the 2D views.
 * - `thermalQuantity` (default surface temperature), `forceQuantity` (default axial force) and `theme` (default light) of the analytic views. */
export type ModelOptions = {
  readonly scale: number;
  readonly at?: EvaluationQuery;
  readonly poses?: readonly Slug[] | "all";
  readonly clips?: readonly Slug[] | "all";
  readonly views?: readonly View[];
  readonly environment?: EnvironmentInput;
  readonly thermalSpanK?: number;
  readonly thermalQuantity?: ThermalQuantity;
  readonly forceQuantity?: ForceQuantity;
  readonly theme?: Theme;
};

const yieldToLoop = (): Promise<void> => new Promise((done) => setTimeout(done, 0));

const withNeutral = (patch: TransformPatch | undefined, base: Local): Local => ({
  translation: (patch?.translation as Triple | undefined) ?? base.translation,
  rotation: (patch?.rotation as Local["rotation"] | undefined) ?? base.rotation,
  scale: (patch?.scale as Triple | undefined) ?? base.scale,
});

const rgba = (paint: Paint): number[] => [...linearOf(paint.hex), paint.alpha];

/** 🗿️ The procedural neutral model of an asset; see {@link ModelOptions}. Throws {@link ModelRequestError} for an asset without volume or an unknown pose or clip, `CancelledError` when the operation was cancelled. */
export async function modelOf(asset: Asset, options: ModelOptions, operation: Operation = silentOperation): Promise<ModelDocument> {
  const volume = asset.volume;
  if (volume === undefined) throw new ModelRequestError("model-no-volume", asset.id);
  if (!(options.scale > 0) || !Number.isFinite(options.scale)) throw new ModelRequestError("model-bad-option", "scale");
  const views = options.views ?? VIEWS;
  const theme: Theme = options.theme ?? "light";
  const named = asset.poses.filter((p) => p.key !== true);
  const first = named[0];
  const at: EvaluationQuery | undefined = options.at ?? (first === undefined ? undefined : { kind: "pose", pose: first.id });
  if (at === undefined) throw new ModelRequestError("model-bad-option", "at");
  const poseIds = options.poses === undefined || options.poses === "all" ? named.map((p) => p.id) : options.poses;
  for (const id of poseIds) if (!asset.poses.some((p) => p.id === id)) throw new ModelRequestError("model-unknown-pose", id);
  const clipIds = options.clips === undefined || options.clips === "all" ? asset.clips.map((c) => c.id) : options.clips;
  for (const id of clipIds) if (!asset.clips.some((c) => c.id === id)) throw new ModelRequestError("model-unknown-clip", id);

  const mm = options.scale / 1000;
  const solids = shownSolids(volume, options.scale);
  const { surfaces, of } = surfacesOf(solids);
  const range: DisplayRange = options.thermalSpanK === undefined ? { mode: "shared" } : { mode: "span", spanK: options.thermalSpanK };
  const heightM = asset.dimensions.heightM;
  const wantsThermal = views.includes("thermal");
  const wantsForces = views.includes("forces");
  const wantsMoments = views.includes("moments");
  const wantsStructure = wantsForces || wantsMoments;
  const eligible = eligibleBones(asset, heightM);
  const session = thermalSession();
  const paperHeightMm = heightM / mm;
  const longestMm = longestArrowMm(paperHeightMm);

  const staticLooks: Partial<Record<"dark-solid" | "light-solid" | "dark-line" | "light-line", LookSpec>> = {
    "dark-solid": solidLook("dark", "dark-solid", surfaces),
    "light-solid": solidLook("light", "light-solid", surfaces),
    "dark-line": lineLook("dark", "dark-line"),
    "light-line": lineLook("light", "light-line"),
  };
  const thermal = wantsThermal ? thermalLook(asset, { quantity: options.thermalQuantity ?? "surface-temperature", range, theme }, surfaces) : undefined;
  const forces = wantsForces ? memberLook(asset, { view: "forces", quantity: options.forceQuantity ?? "axial", range: { mode: "shared" }, theme, eligible }, surfaces) : undefined;
  const moments = wantsMoments ? memberLook(asset, { view: "moments", quantity: "moment", range: { mode: "shared" }, theme, eligible }, surfaces) : undefined;

  const slots = new Slots();
  const frameSetup: FrameSetup = { asset, views, theme, mm, longestMm, looks: { thermal, forces, moments, fixed: staticLooks }, slots };

  async function analyse(query: EvaluationQuery, state: EvaluatedState): Promise<Analytic> {
    const out: Analytic = {};
    if (wantsThermal) out.thermal = solveThermalAt(asset, query, options.environment, { session });
    if (wantsStructure) out.structure = solveStructure(state);
    if (wantsThermal && asset.thermal.emitters.length > 0) out.emitters = solveTransport(asset, query, out.thermal!.environment, { maxAgeS: 0.5, pathlines: false, streamlines: false }).emitters;
    return out;
  }

  const frameOf = (state: EvaluatedState, analytic: Analytic): FrameData => frameDataOf(frameSetup, state, analytic);

  const total = 1 + poseIds.length + clipIds.reduce((n, id) => n + frameTimes(asset.clips.find((c) => c.id === id)!).length, 0);
  let done = 0;
  const step = async (): Promise<void> => {
    if (operation.cancelled()) throw new CancelledError();
    operation.report(Math.min(1, done / total));
    await yieldToLoop();
    if (operation.cancelled()) throw new CancelledError();
    done += 1;
  };

  await step();
  const staticState = evaluate(asset, at);
  const staticFrame = frameOf(staticState, await analyse(at, staticState));

  const poseFrames: { pose: Slug; frame: FrameData }[] = [];
  for (const id of poseIds) {
    await step();
    const query: EvaluationQuery = { kind: "pose", pose: id };
    const isStatic = at.kind === "pose" && at.pose === id;
    if (isStatic) {
      poseFrames.push({ pose: id, frame: staticFrame });
      continue;
    }
    const state = evaluate(asset, query);
    poseFrames.push({ pose: id, frame: { locals: placementsOf(asset, Object.fromEntries(Object.entries(state.bones).map(([bone, b]) => [bone, b.frame]))), paints: new Map(), glyphs: new Map(), glyphPaints: new Map() } });
  }

  const clipFrames: { clip: Slug; times: number[]; frames: FrameData[] }[] = [];
  for (const id of clipIds) {
    const clip = asset.clips.find((c) => c.id === id)!;
    const times = frameTimes(clip);
    const frames: FrameData[] = [];
    for (const timeS of times) {
      await step();
      const query: EvaluationQuery = { kind: "clip", clip: id, timeS };
      const state = evaluate(asset, query);
      frames.push(frameOf(state, await analyse(query, state)));
    }
    clipFrames.push({ clip: id, times, frames });
  }

  const skeleton = skeletonOf(asset);
  const nodes: Node[] = [...skeleton];
  const modelSolids: Solid[] = solids.map((s) => solidOf(s, of.get(s.id)!.id));
  const materials: Material[] = surfaces.map((s) => baseMaterial(s, titleOfSurface(asset, s)));
  const hidden = { hex: "#000000", alpha: 0 } as const;
  const addGlyphMaterial = (id: Slug, title: Text): void => {
    if (!materials.some((m) => m.id === id)) materials.push(materialOf(id, title, hidden, true));
  };
  for (const root of slots.order) {
    const s = slots.map.get(root)!;
    addGlyphMaterial(s.material, { en: `Glyph ${s.kind} ${s.detail}`, de: `Glyphe ${s.kind} ${s.detail}` });
    addGlyphMaterial(glyphEdgeOf(s.material), { en: `Glyph edge ${s.kind} ${s.detail}`, de: `Glyphenkante ${s.kind} ${s.detail}` });
    const made = glyphOf(root, s, mm, staticFrame.glyphs);
    nodes.push(...made.nodes);
    modelSolids.push(...made.solids);
  }

  const edgeBase = "edge";
  materials.push(materialOf(edgeBase, { en: "Feature edge", de: "Kante" }, { hex: THEMES.light.ink, alpha: options.scale <= 100 ? 1 : 0 }, true));

  const variants = views.map((view) => {
    const bindings: Record<Slug, Slug> = {};
    const paints = staticFrame.paints.get(view);
    const look = (view === "thermal" ? thermal : view === "forces" ? forces : view === "moments" ? moments : staticLooks[view]) as LookSpec<never> | undefined;
    const ids = materialIds(view, surfaces, look!);
    for (const s of surfaces) bindings[s.id] = ids.get(look!.keyOf(s))!;
    for (const [key, id] of ids) {
      const rep = surfaces.find((s) => look!.keyOf(s) === key)!;
      materials.push(materialOf(id, look!.titleOf(key, rep), paints!.get(key)!, look!.unlit));
    }
    const edgeVisible = view.endsWith("-line") || view === "thermal" || view === "forces" || view === "moments" || options.scale <= 100;
    const edgeTheme: Theme = view.startsWith("dark") ? "dark" : view.startsWith("light") ? "light" : theme;
    const edgeId = `${view}-edge`;
    materials.push(materialOf(edgeId, { en: "Feature edge", de: "Kante" }, { hex: THEMES[edgeTheme].ink, alpha: edgeVisible ? 1 : 0 }, true));
    bindings[edgeBase] = edgeId;
    for (const root of slots.order) {
      const s = slots.map.get(root)!;
      const mine = (view === "forces" && (s.kind === "arrow" || s.kind === "mass")) || (view === "moments" && s.kind === "arc") || (view === "thermal" && s.kind === "emitter");
      if (!mine) continue;
      const id = `${view}-${s.material}`;
      if (!materials.some((m) => m.id === id)) {
        const paint = glyphPaintOf(s, theme, staticFrame.glyphPaints);
        materials.push(materialOf(id, { en: `Glyph ${s.detail}`, de: `Glyphe ${s.detail}` }, paint));
      }
      bindings[s.material] = id;
      bindings[glyphEdgeOf(s.material)] = edgeId;
    }
    return variantOf(view, VIEW_TITLES[view], bindings);
  });

  const baseOf = new Map(nodes.map((n) => [n.id, n]));
  const localOf = (id: Slug): Local => ({ translation: baseOf.get(id)!.translation as Triple, rotation: baseOf.get(id)!.rotation as Local["rotation"], scale: baseOf.get(id)!.scale as Triple });
  const glyphIds = nodes.filter((n) => n.tags?.includes("glyph")).map((n) => n.id);
  const boneIds = skeleton.map((n) => n.id);

  const poses: Pose[] = poseFrames.map(({ pose, frame }) => {
    const source = asset.poses.find((p) => p.id === pose)!;
    const patches: Record<Slug, TransformPatch> = {};
    for (const id of boneIds) patches[id] = { translation: frame.locals[id]!.translation, rotation: frame.locals[id]!.rotation };
    const isStatic = frame === staticFrame;
    for (const id of glyphIds) if (!isStatic) patches[id] = { scale: HIDDEN };
    return { id: pose, title: source.title, nodes: patches };
  });

  const animations: Animation[] = clipFrames.map(({ clip: id, times, frames }) => {
    const clip = asset.clips.find((c) => c.id === id)!;
    const channels: Channel[] = [];
    for (const bone of boneIds) {
      const base = localOf(bone);
      channels.push(...nodeChannels(bone, times, frames.map((f) => ({ translation: f.locals[bone]!.translation, rotation: f.locals[bone]!.rotation, scale: base.scale })), base, bone === asset.rig.root ? ["translation"] : []));
    }
    for (const node of glyphIds) {
      const base = localOf(node);
      channels.push(...nodeChannels(node, times, frames.map((f) => withNeutral(f.glyphs.get(node), base)), base));
    }
    for (const view of views) {
      if (view !== "thermal" && view !== "forces" && view !== "moments") continue;
      const look = (view === "thermal" ? thermal : view === "forces" ? forces : moments) as LookSpec<never>;
      const ids = materialIds(view, surfaces, look);
      const staticPaints = staticFrame.paints.get(view)!;
      for (const [key, materialId] of ids) {
        const channel = colourChannel(materialId, times, frames.map((f) => rgba(f.paints.get(view)?.get(key) ?? staticPaints.get(key)!)), rgba(staticPaints.get(key)!));
        if (channel !== null) channels.push(channel);
      }
      if (view === "thermal") {
        for (const root of slots.order) {
          const s = slots.map.get(root)!;
          if (s.kind !== "emitter") continue;
          const staticPaint = glyphPaintOf(s, theme, staticFrame.glyphPaints);
          const channel = colourChannel(`thermal-${s.material}`, times, frames.map((f) => rgba(f.glyphPaints.get(s.material) ?? staticPaint)), rgba(staticPaint));
          if (channel !== null) channels.push(channel);
        }
      }
    }
    return animationOf(clip, channels);
  });

  operation.report(1);
  return {
    version: "semio.model/v1",
    id: asset.id,
    title: asset.title,
    provenance: { generator: "staffage volume/v1", source: asset.id, notes: volume.decisions },
    nodes,
    solids: modelSolids,
    meshes: [],
    skins: [],
    materials,
    variants,
    poses,
    animations,
  };
}

function titleOfSurface(asset: Asset, surface: Surface): Text {
  const bone = asset.rig.bones.find((b) => b.id === surface.bone)?.title;
  if (bone === undefined) throw new ModelRequestError("model-unknown-bone", `${asset.id}/${surface.bone}`);
  return { en: `${bone.en} (${surface.role})`, de: `${bone.de} (${surface.role})` };
}
