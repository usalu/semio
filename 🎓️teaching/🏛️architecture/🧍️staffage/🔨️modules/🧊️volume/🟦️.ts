/** 🧊️ The 3D volume of a staffage asset as neutral `semio.model/v1` models, one individually authored model per scale and mode.
 *
 * `asset.volumes[scale]` holds three lists, each drawn against the 2D shapes of its own scale: `body` carries the tone views (`dark-line`, `dark-solid`, `light-line`, `light-solid`), `thermal` the `thermal` view (one material per thermal region and layer, painted from the solved field, animated per clip; with it the developed air flow of the static instant as stream tubes, arrow heads, vapour and fog puffs, see ./🌊️flow) and `structure` the `forces` and `moments` views (member solids painted by N or M per member, plus the generated glyphs).
 * {@link modelOf} builds the procedural model of one scale and one view: the nodes are the rig bones at rest in the neutral frame (right-handed, +Y up, +Z ahead, +X left, metres, radians, quaternions) with the joints as mechanism, the solids are the list of the view's mode carried from the bone-local `(u, v, w)` frame (see ./🧊️solids),
 * the poses are the asset poses, the animations the asset clips sampled with `evaluate`, and the variants are the views of that mode as looks of one geometry. {@link modelOfAsset} builds one document with one kernel scene per authored scale and mode (`s50-body`, `s50-thermal`, `s50-structure`, `s100-body`, …) over the one skeleton, the seven views as variants over all scenes.
 * A scale that is not authored is refused by name (`model-scale-unavailable`), never answered by another scale.
 * One source of truth: bone frames, thermal fields, structural results and the flow field come from `evaluate`, `solveThermalAt`, `solveStructure` and `developTransport`, and the colours from the shared `thermalPaint`, `memberPaint` and `flowPaint` of the 2D views; nothing is solved twice or differently, and the analyses are solved once per instant for every scale and mode.
 *
 * Progress and cancellation flow through the kernel `Operation`: every evaluated frame and every slice of the developing air flow reports and checks, and the work yields to the event loop between them.
 * A clip carries the flow of its first instant as static geometry, placed on the figure of that instant: the tubes do not follow the figure while the clip plays (the field of every frame would be a mesh per frame, which neither glTF nor FBX animates).
 *
 * @see ../../🧬️schema/🟦️.ts — `Volumes`
 * @see ../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts — the neutral kernel
 */
import type { Asset, AuthoredVolume, Slug, Text } from "../../🧬️schema/🟦️.ts";
import { CancelledError, silentOperation, type Animation, type Channel, type Material, type ModelDocument, type Node, type Operation, type Pose, type Primitive, type Scene, type Solid, type TransformPatch, type Variant } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { scopedId } from "./🪪️ids/🟦️.ts";
import { MODES, VIEWS, VIEWS_OF, modeOf, sceneIdOf, type DisplayRange, type EvaluatedState, type EvaluationQuery, type FlowField, type ForceQuantity, type Mode, type ThermalQuantity, type Vec3, type View } from "../🔗️contracts/🟦️.ts";
import { THEMES, type Theme } from "../🎨️depiction/🖍️palette/🟦️.ts";
import { FLOW_DEVELOP_S, VIEW_TITLES } from "../🎨️depiction/🟦️.ts";
import { eligibleBones, longestArrowMm } from "../🎨️depiction/🏗️structural-view/🟦️.ts";
import { solveStructure } from "../🏗️structure/🟦️.ts";
import { solveThermalAt, thermalSession, type EnvironmentInput } from "../🌡️thermal/🟦️.ts";
import { developTransport, developedTimeS } from "../💨️transport/🟦️.ts";
import { evaluate } from "../🧮️evaluation/🟦️.ts";
import { type Local, animationOf, colourChannel, frameTimes, nodeChannels } from "./🎞️clips/🟦️.ts";
import { type LookSpec, type Paint, type Surface, baseMaterial, linearOf, lineLook, materialIds, materialOf, solidLook, surfacesOf, variantOf } from "./🎨️looks/🟦️.ts";
import { type Triple } from "./🧭️frame/🟦️.ts";
import { FLOW_TAG, flowParts } from "./🌊️flow/🟦️.ts";
import { type Analytic, type FrameData, type FrameSetup, Slots, frameDataOf, glyphOf, glyphPaintOf } from "./🧷️frames/🟦️.ts";
import { ModelRequestError } from "./🚫️refusal/🟦️.ts";
import { HIDDEN, glyphEdgeOf, modelSizing } from "./🪧️glyphs/🟦️.ts";
import { authoredAt, authoredScales, listOf, solidOf } from "./🧊️solids/🟦️.ts";
import { skeletonOf, placementsOf, type Placement } from "./🦴️skeleton/🟦️.ts";
import { memberLook } from "./🏗️structure/🟦️.ts";
import { thermalLook } from "./🌡️thermal/🟦️.ts";

export { ModelRequestError, MODEL_REFUSALS, type ModelRefusal } from "./🚫️refusal/🟦️.ts";
export { frameTimes, eventsOf } from "./🎞️clips/🟦️.ts";
export { SCALE_KEYS, authoredAt, authoredScales, listOf, solidOf, type ListSolid } from "./🧊️solids/🟦️.ts";
export { MODES, VIEWS_OF, modeOf, sceneIdOf, type Mode } from "../🔗️contracts/🟦️.ts";
export { toNeutral, pseudoToNeutral, basisOf, quaternionOf } from "./🧭️frame/🟦️.ts";
export { skeletonOf, restPlacements, placementsOf, outOfPlaneBones, jointOf } from "./🦴️skeleton/🟦️.ts";
export { hexOf, linearOf, surfacesOf, type Surface, type Paint } from "./🎨️looks/🟦️.ts";
export { FLOW_MODEL, FLOW_TAG, flowParts, flowUnitOf, tubeStations, tubesOf, type FlowMaterial, type FlowParts } from "./🌊️flow/🟦️.ts";

/** ⚙️ What the analytic looks of any model read besides the asset.
 * - `at`: the static state of the document (colours of the analytic variants, glyph placements): a pose or a clip instant; default the first named pose.
 * - `poses`, `clips`: which named poses and clips become poses and animations; default every one (`"all"`), `[]` none.
 * - `environment`, `thermalSpanK`: the environment of the thermal view (a preset environment or a timeline) and the span ±K around the air temperature (absent: the shared absolute range), as in the 2D views.
 * - `thermalQuantity` (default surface temperature), `forceQuantity` (default axial force) and `theme` (default light) of the analytic views. */
export type SharedModelOptions = {
  readonly at?: EvaluationQuery;
  readonly poses?: readonly Slug[] | "all";
  readonly clips?: readonly Slug[] | "all";
  readonly environment?: EnvironmentInput;
  readonly thermalSpanK?: number;
  readonly thermalQuantity?: ThermalQuantity;
  readonly forceQuantity?: ForceQuantity;
  readonly theme?: Theme;
  /** 🌊️ Whether the thermal scenes carry the developed air flow of the static instant as geometry (default true); a viewer that animates the flow itself asks for none and saves developing it. */
  readonly flowGeometry?: boolean;
};

/** ⚙️ What one model contains: the preset denominator `scale` (1:`scale`, a key of `asset.volumes`; it sizes the glyphs in paper millimetres) and the `view` whose mode, and with it whose list of solids, the model is made of; the model carries every view of that mode as a variant. */
export type ModelOptions = SharedModelOptions & { readonly scale: number; readonly view: View };

/** ⚙️ What the one document of an asset contains: the authored `scales` to include (default every authored scale; an unavailable one is refused) besides the {@link SharedModelOptions}. */
export type AssetModelOptions = SharedModelOptions & { readonly scales?: readonly number[] };

const yieldToLoop = (): Promise<void> => new Promise((done) => setTimeout(done, 0));

const withNeutral = (patch: TransformPatch | undefined, base: Local): Local => ({
  translation: (patch?.translation as Triple | undefined) ?? base.translation,
  rotation: (patch?.rotation as Local["rotation"] | undefined) ?? base.rotation,
  scale: (patch?.scale as Triple | undefined) ?? base.scale,
});

const rgba = (paint: Paint): number[] => [...linearOf(paint.hex), paint.alpha];

const ANALYTIC_VIEWS: ReadonlySet<View> = new Set(["thermal", "forces", "moments"]);

type Instant = { readonly query: EvaluationQuery; readonly state: EvaluatedState; readonly analytic: Analytic };
type Instants = {
  readonly at: EvaluationQuery;
  readonly poseIds: readonly Slug[];
  readonly clipIds: readonly Slug[];
  readonly staticInstant: Instant;
  readonly flow: DevelopedFlow | null;
  readonly poses: readonly { readonly pose: Slug; readonly state: EvaluatedState }[];
  readonly clips: readonly { readonly clip: Slug; readonly times: readonly number[]; readonly instants: readonly Instant[] }[];
};

type Resolved = { readonly at: EvaluationQuery; readonly poseIds: readonly Slug[]; readonly clipIds: readonly Slug[] };

function resolveRequest(asset: Asset, options: SharedModelOptions): Resolved {
  const named = asset.poses.filter((p) => p.key !== true);
  const first = named[0];
  const at: EvaluationQuery | undefined = options.at ?? (first === undefined ? undefined : { kind: "pose", pose: first.id });
  if (at === undefined) throw new ModelRequestError("model-bad-option", "at");
  const poseIds = options.poses === undefined || options.poses === "all" ? named.map((p) => p.id) : options.poses;
  for (const id of poseIds) if (!asset.poses.some((p) => p.id === id)) throw new ModelRequestError("model-unknown-pose", id);
  const clipIds = options.clips === undefined || options.clips === "all" ? asset.clips.map((c) => c.id) : options.clips;
  for (const id of clipIds) if (!asset.clips.some((c) => c.id === id)) throw new ModelRequestError("model-unknown-clip", id);
  return { at, poseIds, clipIds };
}

const boneFramesOf = (state: EvaluatedState): Record<Slug, EvaluatedState["bones"][Slug]["frame"]> => Object.fromEntries(Object.entries(state.bones).map(([bone, b]) => [bone, b.frame]));

const FLOW_STEPS = 8;

/** 🌬️ A developed flow field and what the rig root travelled while it developed (world metres): the field lies around the figure of the developed time, the model shows the figure of the static instant. */
type DevelopedFlow = { readonly field: FlowField; readonly travel: Vec3 };

/** 💨️ The flow field of the static instant, developed as the drawings develop theirs: `FLOW_DEVELOP_S` seconds of history before the instant (whole periods of a looping clip, so the pose stays; none for a clip that plays once, whose air is still at its first instant), in the environment of the thermal colours, slice by slice with `report` and the cancellation of the operation. */
async function developedFlow(asset: Asset, at: EvaluationQuery, state: EvaluatedState, environment: EnvironmentInput, operation: Operation, report: (fraction: number) => void): Promise<DevelopedFlow | null> {
  const clip = at.kind === "clip" ? asset.clips.find((c) => c.id === at.clip) : undefined;
  const periodS = clip?.loop === true && clip.durationS > 0 ? clip.durationS : 0;
  const query = { ...at, timeS: developedTimeS(Math.max(0, at.timeS ?? 0), clip !== undefined && periodS === 0 ? 0 : FLOW_DEVELOP_S, periodS) } as EvaluationQuery;
  const pause = async (): Promise<void> => {
    await yieldToLoop();
    if (operation.cancelled()) throw new CancelledError();
  };
  const field = (await developTransport(asset, query, environment, { maxAgeS: FLOW_DEVELOP_S, ...(periodS > 0 ? { periodS } : {}), flow: true, pathlines: false, pause, progress: (done, total) => report(done / total) })).flow;
  if (field === null) return null;
  const from = state.bones[asset.rig.root]!.frame.o;
  const to = evaluate(asset, query).bones[asset.rig.root]!.frame.o;
  return { field, travel: { x: to.x - from.x, y: to.y - from.y, z: to.z - from.z } };
}

/** ⏱️ Evaluates and analyses every instant the models share (static state, poses, clip frames) and develops the air flow of the static instant: the analyses are solved once whatever the number of scales and modes. */
async function instantsOf(asset: Asset, request: Resolved, options: SharedModelOptions, wants: { readonly thermal: boolean; readonly structure: boolean }, operation: Operation): Promise<Instants> {
  const session = thermalSession();
  const flowing = wants.thermal && options.flowGeometry !== false && asset.thermal.emitters.length > 0;
  const total = (flowing ? FLOW_STEPS : 0) + 1 + request.poseIds.length + request.clipIds.reduce((n, id) => n + frameTimes(asset.clips.find((c) => c.id === id)!).length, 0);
  let done = 0;
  const step = async (): Promise<void> => {
    if (operation.cancelled()) throw new CancelledError();
    operation.report(Math.min(1, done / total));
    await yieldToLoop();
    if (operation.cancelled()) throw new CancelledError();
    done += 1;
  };
  const analyse = (query: EvaluationQuery, state: EvaluatedState): Instant => {
    const analytic: Analytic = {};
    if (wants.thermal) analytic.thermal = solveThermalAt(asset, query, options.environment, { session });
    if (wants.structure) analytic.structure = solveStructure(state);
    return { query, state, analytic };
  };
  await step();
  const staticInstant = analyse(request.at, evaluate(asset, request.at));
  const flow = flowing ? await developedFlow(asset, request.at, staticInstant.state, options.environment ?? asset.thermal.environment, operation, (fraction) => operation.report(Math.min(1, (done + FLOW_STEPS * fraction) / total))) : null;
  if (flowing) done += FLOW_STEPS;
  const poses: { pose: Slug; state: EvaluatedState }[] = [];
  for (const id of request.poseIds) {
    await step();
    const isStatic = request.at.kind === "pose" && request.at.pose === id;
    poses.push({ pose: id, state: isStatic ? staticInstant.state : evaluate(asset, { kind: "pose", pose: id }) });
  }
  const clips: { clip: Slug; times: number[]; instants: Instant[] }[] = [];
  for (const id of request.clipIds) {
    const clip = asset.clips.find((c) => c.id === id)!;
    const times = frameTimes(clip);
    const instants: Instant[] = [];
    for (const timeS of times) {
      await step();
      const query: EvaluationQuery = { kind: "clip", clip: id, timeS };
      instants.push(analyse(query, evaluate(asset, query)));
    }
    clips.push({ clip: id, times, instants });
  }
  return { ...request, staticInstant, flow, poses, clips };
}

/** 🧩️ The geometry, materials, variants, glyphs, glyph poses and colour channels of one scale and mode, in the ids of a single model (no scene prefix yet). */
type Fragment = {
  readonly mode: Mode;
  readonly scale: number;
  readonly solids: Solid[];
  readonly nodes: Node[];
  readonly materials: Material[];
  readonly variants: readonly Variant[];
  readonly poses: ReadonlyMap<Slug, Record<Slug, TransformPatch>>;
  readonly channels: ReadonlyMap<Slug, Channel[]>;
  readonly notes: readonly Text[];
};

function titleOfSurface(asset: Asset, surface: Surface): Text {
  const bone = asset.rig.bones.find((b) => b.id === surface.bone)?.title;
  if (bone === undefined) throw new ModelRequestError("model-unknown-bone", `${asset.id}/${surface.bone}`);
  const suffix = surface.part === undefined ? surface.role : `${surface.part}`;
  return { en: `${bone.en} (${suffix})`, de: `${bone.de} (${suffix})` };
}

/** 🏭️ The fragment of a scale and mode: the list of solids, its surfaces and base materials, the looks of the mode's views bound as variants, the glyphs the mode's analyses need and their placements at every instant, and in the thermal mode the flow parts of the scale. */
function fragmentOf(asset: Asset, volume: AuthoredVolume, mode: Mode, instants: Instants, options: SharedModelOptions): Fragment {
  const scale = volume.denominator;
  const theme: Theme = options.theme ?? "light";
  const views = VIEWS_OF[mode];
  const list = listOf(volume, mode);
  const { surfaces, of } = surfacesOf(list);
  const mm = scale / 1000;
  const range: DisplayRange = options.thermalSpanK === undefined ? { mode: "shared" } : { mode: "span", spanK: options.thermalSpanK };
  const eligible = eligibleBones(asset, asset.dimensions.heightM);
  const longestMm = longestArrowMm(asset.dimensions.heightM / mm);
  const fixed: Partial<Record<"dark-solid" | "light-solid" | "dark-line" | "light-line", LookSpec>> = mode === "body" ? { "dark-solid": solidLook("dark", "dark-solid", surfaces), "light-solid": solidLook("light", "light-solid", surfaces), "dark-line": lineLook("dark", "dark-line"), "light-line": lineLook("light", "light-line") } : {};
  const thermal = mode === "thermal" ? thermalLook(asset, { quantity: options.thermalQuantity ?? "surface-temperature", range, theme }, surfaces) : undefined;
  const forces = mode === "structure" ? memberLook(asset, { view: "forces", quantity: options.forceQuantity ?? "axial", range: { mode: "shared" }, theme, eligible }, surfaces) : undefined;
  const moments = mode === "structure" ? memberLook(asset, { view: "moments", quantity: "moment", range: { mode: "shared" }, theme, eligible }, surfaces) : undefined;
  const slots = new Slots();
  const setup: FrameSetup = { asset, views, theme, mm, longestMm, sizing: modelSizing(mm, Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM), longestMm), looks: { thermal, forces, moments, fixed }, slots };
  const frameOf = (instant: Instant): FrameData => frameDataOf(setup, instant.state, instant.analytic);

  const staticFrame = frameOf(instants.staticInstant);
  const clipFrames = instants.clips.map((c) => ({ clip: c.clip, times: c.times, frames: c.instants.map(frameOf) }));

  const nodes: Node[] = [];
  const solids: Solid[] = list.map((s) => solidOf(s, of.get(s.id)!.id));
  const materials: Material[] = surfaces.map((s) => baseMaterial(s, titleOfSurface(asset, s)));
  const hidden = { hex: "#000000", alpha: 0 } as const;
  const addGlyphMaterial = (id: Slug, title: Text): void => {
    if (!materials.some((m) => m.id === id)) materials.push(materialOf(id, title, hidden, true));
  };
  for (const root of slots.order) {
    const s = slots.map.get(root)!;
    addGlyphMaterial(s.material, { en: `Glyph ${s.kind} ${s.detail}`, de: `Glyphe ${s.kind} ${s.detail}` });
    addGlyphMaterial(glyphEdgeOf(s.material), { en: `Glyph edge ${s.kind} ${s.detail}`, de: `Glyphenkante ${s.kind} ${s.detail}` });
    const made = glyphOf(root, s, setup.sizing!, staticFrame.glyphs);
    nodes.push(...made.nodes);
    solids.push(...made.solids);
  }
  const flow = mode === "thermal" && instants.flow !== null ? flowParts(instants.flow.field, scale, Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM), theme, range, instants.flow.travel) : null;
  if (flow !== null) {
    nodes.push(...flow.nodes);
    solids.push(...flow.solids);
    for (const m of flow.materials) materials.push(materialOf(m.id, m.title, hidden, true));
  }

  const edgeBase = "edge";
  materials.push(materialOf(edgeBase, { en: "Feature edge", de: "Kante" }, { hex: THEMES.light.ink, alpha: scale <= 100 ? 1 : 0 }, true));

  const lookOf = (view: View): LookSpec<never> => (view === "thermal" ? thermal : view === "forces" ? forces : view === "moments" ? moments : fixed[view as "dark-solid"]) as unknown as LookSpec<never>;
  const variants = views.map((view) => {
    const bindings: Record<Slug, Slug> = {};
    const paints = staticFrame.paints.get(view);
    const look = lookOf(view);
    const ids = materialIds(view, surfaces, look);
    for (const s of surfaces) bindings[s.id] = ids.get(look.keyOf(s))!;
    for (const [key, id] of ids) {
      const rep = surfaces.find((s) => look.keyOf(s) === key)!;
      materials.push(materialOf(id, look.titleOf(key, rep), paints!.get(key)!, look.unlit));
    }
    const edgeVisible = view.endsWith("-line") || ANALYTIC_VIEWS.has(view) || scale <= 100;
    const edgeTheme: Theme = view.startsWith("dark") ? "dark" : view.startsWith("light") ? "light" : theme;
    const edgeId = `${view}-edge`;
    materials.push(materialOf(edgeId, { en: "Feature edge", de: "Kante" }, { hex: THEMES[edgeTheme].ink, alpha: edgeVisible ? 1 : 0 }, true));
    bindings[edgeBase] = edgeId;
    for (const root of slots.order) {
      const s = slots.map.get(root)!;
      const mine = (view === "forces" && (s.kind === "arrow" || s.kind === "mass")) || (view === "moments" && s.kind === "arc");
      if (!mine) continue;
      const id = `${view}-${s.material}`;
      if (!materials.some((m) => m.id === id)) materials.push(materialOf(id, { en: `Glyph ${s.detail}`, de: `Glyphe ${s.detail}` }, glyphPaintOf(s, theme)));
      bindings[s.material] = id;
      bindings[glyphEdgeOf(s.material)] = edgeId;
    }
    if (view === "thermal" && flow !== null)
      for (const m of flow.materials) {
        materials.push(materialOf(`${view}-${m.id}`, m.title, m.paint, true));
        bindings[m.id] = `${view}-${m.id}`;
      }
    return variantOf(view, VIEW_TITLES[view], bindings);
  });

  const glyphIds = nodes.filter((n) => n.tags?.includes("glyph") === true && !(n.tags.includes(FLOW_TAG) && n.parent !== null)).map((n) => n.id);
  const baseOf = new Map(nodes.map((n) => [n.id, n]));
  const localOf = (id: Slug): Local => ({ translation: baseOf.get(id)!.translation as Triple, rotation: baseOf.get(id)!.rotation as Local["rotation"], scale: baseOf.get(id)!.scale as Triple });

  const poses = new Map<Slug, Record<Slug, TransformPatch>>();
  for (const { pose } of instants.poses) {
    const isStatic = instants.at.kind === "pose" && instants.at.pose === pose;
    const patches: Record<Slug, TransformPatch> = {};
    if (!isStatic) for (const id of glyphIds) patches[id] = { scale: HIDDEN };
    poses.set(pose, patches);
  }

  const channels = new Map<Slug, Channel[]>();
  for (const { clip, times, frames } of clipFrames) {
    const out: Channel[] = [];
    for (const node of glyphIds) {
      const base = localOf(node);
      out.push(...nodeChannels(node, times, frames.map((f) => withNeutral(f.glyphs.get(node), base)), base));
    }
    for (const view of views) {
      if (!ANALYTIC_VIEWS.has(view)) continue;
      const look = lookOf(view);
      const ids = materialIds(view, surfaces, look);
      const staticPaints = staticFrame.paints.get(view)!;
      for (const [key, materialId] of ids) {
        const channel = colourChannel(materialId, times, frames.map((f) => rgba(f.paints.get(view)?.get(key) ?? staticPaints.get(key)!)), rgba(staticPaints.get(key)!));
        if (channel !== null) out.push(channel);
      }
    }
    channels.set(clip, out);
  }
  return { mode, scale, solids, nodes, materials, variants, poses, channels, notes: volume.decisions };
}

/** 🦴️ The skeleton shared by every scene: bone nodes, the bone patches of every pose and the bone channels of every clip (root translation always, as the root moves). */
function skeletonModel(asset: Asset, instants: Instants): { readonly nodes: Node[]; readonly poseBones: ReadonlyMap<Slug, Record<Slug, TransformPatch>>; readonly clipBones: ReadonlyMap<Slug, Channel[]> } {
  const nodes = skeletonOf(asset);
  const boneIds = nodes.map((n) => n.id);
  const baseOf = new Map(nodes.map((n) => [n.id, n]));
  const localsOf = (state: EvaluatedState): Record<Slug, Placement> => placementsOf(asset, boneFramesOf(state));
  const poseBones = new Map<Slug, Record<Slug, TransformPatch>>();
  for (const { pose, state } of instants.poses) {
    const locals = localsOf(state);
    const patches: Record<Slug, TransformPatch> = {};
    for (const id of boneIds) patches[id] = { translation: locals[id]!.translation, rotation: locals[id]!.rotation };
    poseBones.set(pose, patches);
  }
  const clipBones = new Map<Slug, Channel[]>();
  for (const { clip, times, instants: frames } of instants.clips) {
    const locals = frames.map((f) => localsOf(f.state));
    const out: Channel[] = [];
    for (const bone of boneIds) {
      const node = baseOf.get(bone)!;
      const base: Local = { translation: node.translation as Triple, rotation: node.rotation as Local["rotation"], scale: node.scale as Triple };
      out.push(...nodeChannels(bone, times, locals.map((l) => ({ translation: l[bone]!.translation, rotation: l[bone]!.rotation, scale: base.scale })), base, bone === asset.rig.root ? ["translation"] : []));
    }
    clipBones.set(clip, out);
  }
  return { nodes, poseBones, clipBones };
}

/** 🪪️ A fragment in the ids of scene `scene`: every solid, material, glyph node and colour channel target gets the prefix `<scene>-`, and every solid rides on an identity child node `<scene>-<bone>` of its bone (the kernel's scene-specific mesh node), so scenes share the skeleton and nothing else. */
function scoped(fragment: Fragment, scene: Slug, bones: ReadonlySet<Slug>): { readonly nodes: Node[]; readonly solids: Solid[]; readonly materials: Material[]; readonly variants: readonly Variant[]; readonly poses: ReadonlyMap<Slug, Record<Slug, TransformPatch>>; readonly channels: ReadonlyMap<Slug, Channel[]> } {
  const p = (id: Slug): Slug => scopedId(scene, id);
  const used = new Set<Slug>();
  const solids: Solid[] = fragment.solids.map((s) => {
    used.add(s.node);
    return { ...s, id: p(s.id), node: p(s.node), material: p(s.material) } as Solid;
  });
  const meshNodes: Node[] = [...used].filter((n) => bones.has(n)).map((bone) => ({ id: p(bone), parent: bone, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], tags: ["scene-mesh", scene] }));
  const glyphs: Node[] = fragment.nodes.map((n) => ({ ...n, id: p(n.id), parent: n.parent === null ? null : p(n.parent) }));
  const materials: Material[] = fragment.materials.map((m) => ({ ...m, id: p(m.id) }));
  const variants: Variant[] = fragment.variants.map((v) => ({ ...v, bindings: Object.fromEntries(Object.entries(v.bindings).map(([from, to]) => [p(from), p(to)])) }));
  const rekey = (patches: Record<Slug, TransformPatch>): Record<Slug, TransformPatch> => Object.fromEntries(Object.entries(patches).map(([id, patch]) => [p(id), patch]));
  const poses = new Map([...fragment.poses].map(([id, patches]) => [id, rekey(patches)] as const));
  const retarget = (channel: Channel): Channel => ("node" in channel.target ? { ...channel, target: { ...channel.target, node: p(channel.target.node) } } : { ...channel, target: { ...channel.target, material: p(channel.target.material) } });
  const channels = new Map([...fragment.channels].map(([id, list]) => [id, list.map(retarget)] as const));
  return { nodes: [...meshNodes, ...glyphs], solids, materials, variants, poses, channels };
}

function titleOfPose(asset: Asset, id: Slug): Text {
  return asset.poses.find((p) => p.id === id)!.title;
}

const MODE_TITLES: Readonly<Record<Mode, Text>> = { body: { en: "body", de: "Körper" }, thermal: { en: "thermal", de: "thermisch" }, structure: { en: "structure", de: "Tragwerk" } };

/** 🗿️ The procedural neutral model of one scale and mode of an asset; see {@link ModelOptions}. Throws {@link ModelRequestError} for a scale that is not authored, an unknown pose or clip, `CancelledError` when the operation was cancelled. */
export async function modelOf(asset: Asset, options: ModelOptions, operation: Operation = silentOperation): Promise<ModelDocument> {
  if (!(options.scale > 0) || !Number.isFinite(options.scale)) throw new ModelRequestError("model-bad-option", "scale");
  const volume = authoredAt(asset.volumes, options.scale, asset.id);
  const mode = modeOf(options.view);
  const request = resolveRequest(asset, options);
  const instants = await instantsOf(asset, request, options, { thermal: mode === "thermal", structure: mode === "structure" }, operation);
  const skeleton = skeletonModel(asset, instants);
  const fragment = fragmentOf(asset, volume, mode, instants, options);
  const poses: Pose[] = instants.poses.map(({ pose }) => ({ id: pose, title: titleOfPose(asset, pose), nodes: { ...skeleton.poseBones.get(pose)!, ...fragment.poses.get(pose)! } }));
  const animations: Animation[] = instants.clips.map(({ clip }) => animationOf(asset.clips.find((c) => c.id === clip)!, [...skeleton.clipBones.get(clip)!, ...(fragment.channels.get(clip) ?? [])]));
  operation.report(1);
  return {
    version: "semio.model/v1",
    id: asset.id,
    title: asset.title,
    provenance: { generator: "staffage volume/v2", source: asset.id, notes: fragment.notes },
    nodes: [...skeleton.nodes, ...fragment.nodes],
    solids: fragment.solids,
    meshes: [],
    skins: [],
    materials: fragment.materials,
    variants: [...fragment.variants],
    poses,
    animations,
  };
}

/** 🏛️ The one procedural model of an asset with every requested authored scale: one kernel scene per scale and mode (`s50-body`, `s50-thermal`, `s50-structure`, …) over the one skeleton, all seven views as variants over every scene (a view recolours the scenes of its mode and leaves the other scenes at their base look), the poses and clips once. Throws {@link ModelRequestError} for a requested scale that is not authored (`model-scale-unavailable`) and for an asset with no authored scale (`model-no-authored-scale`). */
export async function modelOfAsset(asset: Asset, options: AssetModelOptions = {}, operation: Operation = silentOperation): Promise<ModelDocument> {
  const scales = options.scales ?? authoredScales(asset.volumes);
  if (scales.length === 0) throw new ModelRequestError("model-no-authored-scale", asset.id);
  const volumes = scales.map((scale) => authoredAt(asset.volumes, scale, asset.id));
  const request = resolveRequest(asset, options);
  const instants = await instantsOf(asset, request, options, { thermal: true, structure: true }, operation);
  const skeleton = skeletonModel(asset, instants);
  const bones = new Set(skeleton.nodes.map((n) => n.id));
  const scenes: Scene[] = [];
  const nodes: Node[] = [...skeleton.nodes];
  const glyphNodes: Node[] = [];
  const solids: Solid[] = [];
  const materials: Material[] = [];
  const variantBindings = new Map<View, { title: Text; bindings: Record<Slug, Slug> }>(VIEWS.map((view) => [view, { title: VIEW_TITLES[view], bindings: {} }]));
  const poseGlyphs = new Map<Slug, Record<Slug, TransformPatch>>(instants.poses.map(({ pose }) => [pose, {}]));
  const clipChannels = new Map<Slug, Channel[]>(instants.clips.map(({ clip }) => [clip, []]));
  const notes: Text[] = [];
  for (const volume of volumes) {
    for (const mode of MODES) {
      if (operation.cancelled()) throw new CancelledError();
      await yieldToLoop();
      const id = sceneIdOf(volume.denominator, mode);
      const fragment = fragmentOf(asset, volume, mode, instants, options);
      const scene = scoped(fragment, id, bones);
      for (const node of scene.nodes) (node.tags?.includes("glyph") === true ? glyphNodes : nodes).push(node);
      solids.push(...scene.solids);
      materials.push(...scene.materials);
      for (const variant of scene.variants) Object.assign(variantBindings.get(variant.id as View)!.bindings, variant.bindings);
      for (const [pose, patches] of scene.poses) Object.assign(poseGlyphs.get(pose)!, patches);
      for (const [clip, list] of scene.channels) clipChannels.get(clip)!.push(...list);
      scenes.push({ id, title: { en: `1:${volume.denominator} ${MODE_TITLES[mode].en}`, de: `1:${volume.denominator} ${MODE_TITLES[mode].de}` }, nodes: [...new Set(scene.solids.map((s) => s.node))], tags: [`scale-${volume.denominator}`, mode] });
      if (mode === "body") notes.push(...fragment.notes.map((t) => ({ en: `1:${volume.denominator}: ${t.en}`, de: `1:${volume.denominator}: ${t.de}` })));
    }
  }
  nodes.push(...glyphNodes);
  const poses: Pose[] = instants.poses.map(({ pose }) => ({ id: pose, title: titleOfPose(asset, pose), nodes: { ...skeleton.poseBones.get(pose)!, ...poseGlyphs.get(pose)! } }));
  const animations: Animation[] = instants.clips.map(({ clip }) => animationOf(asset.clips.find((c) => c.id === clip)!, [...skeleton.clipBones.get(clip)!, ...clipChannels.get(clip)!]));
  operation.report(1);
  return {
    version: "semio.model/v1",
    id: asset.id,
    title: asset.title,
    provenance: { generator: "staffage volume/v2", source: asset.id, notes },
    nodes,
    solids,
    meshes: [],
    skins: [],
    materials,
    variants: VIEWS.map((view) => variantOf(view, variantBindings.get(view)!.title, variantBindings.get(view)!.bindings)),
    poses,
    animations,
    scenes,
  };
}

/** 🌟️ The edge material of the feature-edge pass for the primitives of a procedural document: the base `edge` material of the primitive's scene (`<scene>-edge`; plain `edge` without scenes), except for the primitives of glyph solids, whose edges follow their own glyph material ({@link glyphEdgeOf}), and of the flow solids, whose edges (the rims of tube ends and arrow heads) keep the material of their solid and so show and hide with it. */
export function edgeMaterialOf(document: ModelDocument): (source: Primitive) => Slug {
  const glyphs = new Set(document.solids.filter((solid) => solid.tags?.includes("glyph") === true).map((solid) => solid.material));
  const prefixes = (document.scenes ?? []).map((scene) => `${scene.id}-`);
  const flow = new Set(document.solids.filter((solid) => solid.tags?.includes(FLOW_TAG) === true).map((solid) => solid.material));
  return (source) => (flow.has(source.material) ? source.material : glyphs.has(source.material) ? glyphEdgeOf(source.material) : `${prefixes.find((prefix) => source.material.startsWith(prefix)) ?? ""}edge`);
}
