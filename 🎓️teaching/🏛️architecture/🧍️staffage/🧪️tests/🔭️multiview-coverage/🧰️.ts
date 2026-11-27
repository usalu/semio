/** 🔭️ The multiview coverage engine: the matrix asset × scale (preset, valid-range band edges, declared alternatives) × viewpoint (elevation, front, plan) × mode (seven) × pose, clip instant and transition.
 *
 * Every cell is `ok` (rendered and checked), `unavailable` (the asset declares the scale unavailable with a complete reason, limiting feature and alternatives), `range-end`, `group-drift` (reported tiers) or `fail` with named checks.
 * The universe of cells is enumerated from the declarations alone ({@link universe}); a cell that no runner produced is a `fail` named `cell-missing` and the cells of an asset whose run crashed or timed out fail under that name ({@link failedAsset}), so nothing is ever skipped silently.
 * Declarations, geometry of every pose, clip instant and transition, and the cross-view synchrony of every instant are checked exhaustively; the seven modes are rendered for every pose
 * and for the clip instants of {@link clipInstants} (keyframes, mid-transitions, extremes of the extents, start, end and at least eight uniform intermediates).
 *
 * The check functions are exported one by one ({@link geometryFindings}, {@link paperFindings}, {@link cueFinding}, {@link derivedFindings}) so that the authors' gate checks one asset with exactly the checks of the matrix.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🟦️.ts — the cases
 * The catalog is 375 documents of 615 MB together: a caller that works on single assets reads single documents ({@link loadAsset}, {@link mapCatalog}, {@link catalogLookup}) and never holds the catalog ({@link loadCatalog} does, 2.1 GB resident).
 *
 * @see ./📊️matrix.ts — the generator: one process and one record per asset, resume, merge of shards
 * @see ./🔎️scan.ts — the authors' gate: one asset in seconds
 * @see ../../🧫️fixtures/🔭️multiview-coverage/🔣️.json — the vectors
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { CATEGORIES, SCALE_DENOMINATORS, PLANAR_VIEWPOINTS, type Alternative, type Asset, type AssetDocument, type AuthoredRepresentation, type Language, type SymbolRepresentation, type PlanarViewpoint } from "../../🧬️schema/🟦️.ts";
import { VIEWS, type DrawingScene, type EvaluatedShape, type EvaluatedState, type EvaluationQuery, type RepresentationSelector, type View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { defaultDepictionOptions, textlessDepictionOptions } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, pickRepresentation } from "../../🔨️modules/🖨️representation/🟦️.ts";
import { resolveViews, type DrawOptions, type DrawingSource, type MultiView } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { STAFFAGE_ROOT } from "../🧰️support/🧰️.ts";
import { derivedFlags } from "./🪞️derived.ts";
import { anchorLandmarks, boneSync } from "./⚓️landmarks.ts";

/** 👁️ The three supported viewpoints: the side (elevation), the front and the top (plan). */
export const VIEWPOINT_LIST: readonly PlanarViewpoint[] = PLANAR_VIEWPOINTS;

/** 🎨️ The seven modes. */
export const MODE_LIST: readonly View[] = VIEWS;

/** 🔢️ The preset scale keys every asset must decide. */
export const PRESET_KEYS: readonly string[] = SCALE_DENOMINATORS.map(String);

/** 🧾️ The status of one cell. */
export type Status = "ok" | "unavailable" | "range-end" | "group-drift" | "fail";

/** 🧩️ The kind of a scale entry of the matrix. */
export type ScaleKind = "preset" | "declared" | "band-min" | "band-max" | "alternative";

/** 🔍️ The failures of one check in one cell: how many subjects failed and the first messages. */
export type Failures = { readonly count: number; readonly first: readonly { readonly subject: string; readonly message: string }[]; readonly worst?: { readonly deltaMm: number; readonly subject: string; readonly pair: string }; readonly byKey?: Readonly<Record<string, { readonly deltaMm: number; readonly subject: string }>> };

/** 🧱️ One cell of the matrix. */
export type Cell = {
  readonly asset: string;
  readonly category: string;
  readonly scale: string;
  readonly scaleKind: ScaleKind;
  readonly requested: number | null;
  readonly view: PlanarViewpoint;
  readonly mode: View;
  readonly status: Status;
  readonly checked: number;
  readonly reason?: string;
  readonly failures: Readonly<Record<string, Failures>>;
  readonly rangeEnd?: Readonly<Record<string, Failures>>;
  readonly groupDrift?: Readonly<Record<string, Failures>>;
};

/** ⏱️ A sampled instant of a clip. */
export type Instant = { readonly tag: "start" | "key" | "mid" | "extreme" | "uniform" | "end"; readonly t: number; readonly name: string };

/** ▶️ One evaluated subject: a pose, a clip instant or a transition instant. */
export type Subject = { readonly id: string; readonly query: EvaluationQuery; readonly isDefault: boolean; readonly modes: readonly View[]; readonly languages: readonly Language[] };

/** 📏️ The extent of drawn shapes. */
export type Extent = { readonly minX: number; readonly maxX: number; readonly minY: number; readonly maxY: number };

/** 📤️ The count of uniform intermediate instants per clip, and per clip longer than {@link LONG_CLIP_S}. */
export const UNIFORM_INSTANTS = 8;

/** ⏳️ The duration above which a clip counts as long and is sampled at {@link LONG_UNIFORM_INSTANTS} uniform instants. */
export const LONG_CLIP_S = 600;

/** 📝️ The uniform instants of a long clip. */
export const LONG_UNIFORM_INSTANTS = 16;

/** 🌍️ The languages rendered at poses and at the start, key, extreme and end instants; the other instants render English. */
export const LANGUAGES: readonly Language[] = ["en", "de"];

/** 🌡️ The representative modes rendered at the cheaper instants (uniform, mid, transitions): one line mode, the thermal and the moments mode. */
export const SAMPLED_MODES: readonly View[] = ["light-line", "thermal", "moments"];

/** 📦️ The probes of the extreme scan of a clip. */
export const EXTREME_PROBES = 48;

/** 🎯️ The D18 tolerance of a dimension: the larger of 15 mm and 2 % of it. */
export const d18 = (nominalM: number): number => Math.max(0.015, 0.02 * nominalM);

/** 🪙️ A length in whole micrometres: the unit every dimension and extent comparison is decided in (D50), so a difference that equals its tolerance is the same integer on both sides whatever the unit or the scale it was computed in. */
export const micrometres = (lengthM: number): number => Math.round(lengthM * 1e6);

/** ⚖️ Whether a measured length lies within D18 of its nominal one, decided in whole micrometres (D50): a difference of exactly the tolerance is within it, one micrometre more is not. */
export const withinD18 = (measuredM: number, nominalM: number): boolean => Math.abs(micrometres(measuredM) - micrometres(nominalM)) <= micrometres(d18(nominalM));

/** 💾️ The D18 tolerance of the synchrony checks: D18 of the largest dimension of the asset. */
export const toleranceM = (asset: Asset): number => d18(Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM));

const CATALOG_ROOT = join(STAFFAGE_ROOT, "🗂️catalog");

const groupsOf = (root: string): string[] => readdirSync(root, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => join(root, entry.name));

const readAsset = (file: string): Asset => (JSON.parse(readFileSync(file, "utf8")) as AssetDocument).asset;

/** 📁️ The asset document files of the catalog in folder order, unread. */
export function catalogFiles(root = CATALOG_ROOT): string[] {
  return groupsOf(root).flatMap((group) => readdirSync(group, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => join(group, entry.name, "🔣️.json")).filter((file) => existsSync(file)));
}

/** 🗂️ The catalog asset documents in id order, without validating them. It holds every document at once (2.1 GB resident for the 375 documents of 2026-11): only for a caller that needs them all together. */
export function loadCatalog(root = CATALOG_ROOT): Asset[] {
  return catalogFiles(root).map(readAsset).sort((a, b) => a.id.localeCompare(b.id));
}

/** 🚰️ The catalog one asset document at a time in folder order: the caller keeps what it takes of a document, the document itself is released with its turn. */
export function* streamCatalog(root = CATALOG_ROOT): Generator<Asset> {
  for (const file of catalogFiles(root)) yield readAsset(file);
}

/** 🎣️ One asset by id, or undefined when the catalog has none. It reads the one document of the folder the contract gives it (`<category>/<id>/🔣️.json`); only an id that no folder is named after is searched in the stream of the catalog. */
export function loadAsset(id: string, root = CATALOG_ROOT): Asset | undefined {
  for (const group of groupsOf(root)) {
    const file = join(group, id, "🔣️.json");
    if (!existsSync(file)) continue;
    const asset = readAsset(file);
    if (asset.id === id) return asset;
  }
  for (const asset of streamCatalog(root)) if (asset.id === id) return asset;
  return undefined;
}

/** 🪡️ What `project` takes of the assets of the catalog, in id order, one document at a time: of the named `ids` (each read by {@link loadAsset}, unknown ones left out) or of the whole catalog; a projection that returns undefined leaves its asset out. No document outlives its turn unless the projection keeps it. */
export function mapCatalog<T extends { readonly id: string }>(project: (asset: Asset) => T | undefined, ids?: readonly string[], root = CATALOG_ROOT): T[] {
  const taken: T[] = [];
  const take = (asset: Asset | undefined): void => {
    const projected = asset === undefined ? undefined : project(asset);
    if (projected !== undefined) taken.push(projected);
  };
  if (ids === undefined) for (const asset of streamCatalog(root)) take(asset);
  else for (const id of new Set(ids)) take(loadAsset(id, root));
  return taken.sort((a, b) => a.id.localeCompare(b.id));
}

/** 📇️ The catalog by asset id as the matrix asks for it: whether an asset exists, and its document. A `Map` of the catalog is one; {@link catalogLookup} is the one that does not hold the catalog. */
export type CatalogLookup = Pick<ReadonlyMap<string, Asset>, "has" | "get">;

/** 🔦️ A {@link CatalogLookup} that reads the document of an asset when it is asked for and keeps the last answer only, so the run of one asset holds its own document and at most one alternative asset. */
export function catalogLookup(root = CATALOG_ROOT): CatalogLookup {
  let last: { readonly id: string; readonly asset: Asset | undefined } | undefined;
  const get = (id: string): Asset | undefined => (last?.id === id ? last : (last = { id, asset: loadAsset(id, root) })).asset;
  return { has: (id) => get(id) !== undefined, get };
}

/** 📐️ The extent of the non-ground shapes, or undefined when nothing is drawn. */
export function extentOf(shapes: readonly EvaluatedShape[]): Extent | undefined {
  const points = shapes.filter((evaluated) => evaluated.shape.role !== "ground").flatMap((evaluated) => evaluated.rings.flat());
  if (points.length === 0) return undefined;
  const xs = points.map((p) => p.x);
  const ys = points.map((p) => p.y);
  return { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
}

/** 🪜️ The bounding box in paper millimetres of the path items of a scene whose id is in `ids` (all path items of the body layers when `ids` is undefined). */
export function sceneBox(scene: DrawingScene, ids?: ReadonlySet<string>): Extent | undefined {
  const xs: number[] = [];
  const ys: number[] = [];
  for (const layer of scene.layers)
    for (const item of layer.items) {
      if (item.kind !== "path" || (ids !== undefined && !ids.has(item.id))) continue;
      if (ids === undefined && !["fill", "field", "body", "detail", "texture", "glazing"].includes(layer.id)) continue;
      for (const c of item.commands) {
        if (c.op === "Z") continue;
        xs.push(c.x);
        ys.push(c.y);
        if (c.op === "C") xs.push(c.x1, c.x2), ys.push(c.y1, c.y2);
      }
    }
  return xs.length === 0 ? undefined : { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
}

/** 🧻️ The ids of the path items that draw the non-ground shapes of a view: the fill of a filled closed shape (`shape:<id>`) and every visible outline run of a shape (`outline:<id>:<n>`), so a part drawn unfilled is measured too. */
export function paperIds(scene: DrawingScene, shapes: readonly EvaluatedShape[]): Set<string> {
  const wanted = new Set(shapes.filter((evaluated) => evaluated.shape.role !== "ground").map((evaluated) => evaluated.shape.id));
  const ids = new Set<string>();
  for (const layer of scene.layers)
    for (const item of layer.items) {
      if (item.kind !== "path") continue;
      const shape = item.id.startsWith("shape:") ? item.id.slice(6) : item.id.startsWith("outline:") ? item.id.slice(8, item.id.lastIndexOf(":")) : undefined;
      if (shape !== undefined && wanted.has(shape)) ids.add(item.id);
    }
  return ids;
}

/** 📃️ The paper box in millimetres of the shapes of a view: fills and outline runs ({@link paperIds}), or undefined when none is drawn. */
export const paperBox = (scene: DrawingScene, shapes: readonly EvaluatedShape[]): Extent | undefined => sceneBox(scene, paperIds(scene, shapes));

/** 🔑️ The key of a cell. */
export const cellKey = (asset: string, scale: string, view: PlanarViewpoint, mode: View): string => `${asset}|${scale}|${view}|${mode}`;

/** 🧭️ One scale entry of an asset's matrix. */
export type ScaleEntry = { readonly scale: string; readonly kind: ScaleKind; readonly requested: number | null; readonly representation: string };

/** ✅️ The scale entries of an asset: every preset and every declared key, plus the edges of the valid range of every authored representation; alternatives follow their unavailable entry. */
export function scaleEntries(asset: Asset): ScaleEntry[] {
  const keys = [...new Set([...PRESET_KEYS, ...Object.keys(asset.representations)])].sort((a, b) => Number(b) - Number(a));
  const entries: ScaleEntry[] = [];
  for (const key of keys) {
    entries.push({ scale: key, kind: PRESET_KEYS.includes(key) ? "preset" : "declared", requested: Number(key), representation: key });
    const decision = asset.representations[key as keyof Asset["representations"]];
    if (decision?.status === "authored") {
      const { minDenominator, maxDenominator } = decision.validRange;
      if (minDenominator !== Number(key)) entries.push({ scale: `${key}:band-min`, kind: "band-min", requested: minDenominator, representation: key });
      if (maxDenominator !== Number(key)) entries.push({ scale: `${key}:band-max`, kind: "band-max", requested: maxDenominator, representation: key });
    }
    if (decision?.status === "unavailable") decision.alternatives.forEach((alternative, i) => entries.push({ scale: `${key}>${alternativeId(alternative, i)}`, kind: "alternative", requested: Number(key), representation: key }));
  }
  return entries;
}

/** 🏷️ The id of an alternative inside a scale key. */
export const alternativeId = (alternative: Alternative, index: number): string => `alt${index}:${alternative.kind === "scale" ? `scale:${alternative.denominator}` : alternative.kind === "symbol" ? `symbol:${alternative.symbol}` : `asset:${alternative.asset}`}`;

/** 🪪️ What the matrix keeps of an asset to place its cells: the id, the category and the scale entries. The universe of cells follows from these alone, so a merge over the catalog holds no document. */
export type Declared = { readonly id: string; readonly category: string; readonly entries: readonly ScaleEntry[] };

/** 📋️ The {@link Declared} of an asset document. */
export const declaredOf = (asset: Asset): Declared => ({ id: asset.id, category: asset.category, entries: scaleEntries(asset) });

/** 🔐️ Every cell key of one asset, in the order of the matrix: scale entry, viewpoint, mode. */
export const keysOf = (declared: Declared): string[] => declared.entries.flatMap((entry) => VIEWPOINT_LIST.flatMap((view) => MODE_LIST.map((mode) => cellKey(declared.id, entry.scale, view, mode))));

/** 🌌️ Every cell key the matrix must contain for the assets, from the declarations alone. */
export function universe(assets: readonly Asset[]): string[] {
  return assets.flatMap((asset) => keysOf(declaredOf(asset)));
}

/** 🗺️ The sampled instants of a clip: start, every keyframe, the middle of every key interval, the extremes of the extents (`probe` maps a time to the extents of the three views and the rig bounds), `UNIFORM_INSTANTS` uniform intermediates and the end. Looping clips wrap, non-looping ones stay inside. */
export function clipInstants(asset: Asset, clipId: string, probe?: (t: number) => readonly number[] | undefined): Instant[] {
  const clip = asset.clips.find((candidate) => candidate.id === clipId)!;
  const last = clip.loop ? clip.durationS : clip.durationS * 0.999;
  const clamp = (t: number): number => (clip.loop ? (t >= clip.durationS ? t - clip.durationS : t) : Math.min(t, last));
  const found: Instant[] = [];
  const add = (tag: Instant["tag"], raw: number, label: string): void => {
    const t = Math.round(clamp(raw) * 1e6) / 1e6;
    if (!found.some((instant) => Math.abs(instant.t - t) < 1e-6)) found.push({ tag, t, name: `${label} @${t.toFixed(3)} s` });
  };
  add("start", 0, "start");
  const keys = [...clip.keys].sort((a, b) => a.t - b.t);
  keys.forEach((key, i) => {
    add("key", key.t, `key ${key.pose}`);
    const next = keys[i + 1];
    if (next !== undefined) add("mid", (key.t + next.t) / 2, `mid ${key.pose}>${next.pose}`);
  });
  if (probe !== undefined) {
    const samples = Array.from({ length: EXTREME_PROBES + 1 }, (_, i) => {
      const t = (clip.durationS * 0.999 * i) / EXTREME_PROBES;
      return { t, values: probe(t) };
    }).filter((sample): sample is { t: number; values: readonly number[] } => sample.values !== undefined);
    for (let channel = 0; channel < (samples[0]?.values.length ?? 0); channel++) {
      const sorted = [...samples].sort((a, b) => a.values[channel]! - b.values[channel]!);
      add("extreme", sorted[0]!.t, `min channel ${channel}`);
      add("extreme", sorted[sorted.length - 1]!.t, `max channel ${channel}`);
    }
  }
  const uniform = clip.durationS > LONG_CLIP_S ? LONG_UNIFORM_INSTANTS : UNIFORM_INSTANTS;
  const free = (t: number): boolean => !found.some((instant) => Math.abs(instant.t - Math.round(clamp(t) * 1e6) / 1e6) < 1e-6);
  for (let i = 1; i <= uniform; i++) {
    let t = (clip.durationS * i) / (uniform + 1);
    for (let nudge = 1; !free(t) && nudge < 50; nudge++) t += clip.durationS / (4 * (uniform + 1) * nudge);
    add("uniform", t, `uniform ${i}/${uniform + 1}`);
  }
  add("end", last, "end");
  return found.sort((a, b) => a.t - b.t);
}

/** 🔬️ The channels of the extreme scan: the rig bounds and the extents of the three views. */
export function probeChannels(state: EvaluatedState): readonly number[] | undefined {
  const e = extentOf(state.shapes.elevation);
  const f = extentOf(state.shapes.front);
  const p = extentOf(state.shapes.plan);
  const b = state.bounds;
  return [b.minX, b.maxX, b.minY, b.maxY, ...(e ? [e.minX, e.maxX, e.minY, e.maxY] : [0, 0, 0, 0]), ...(f ? [f.minX, f.maxX] : [0, 0]), ...(p ? [p.minY, p.maxY] : [0, 0])];
}

/** 🌐️ The selector of a requested denominator. */
export const scaleSelector = (denominator: number): RepresentationSelector => ({ kind: "scale", scale: { denominator } });

/** 🎲️ A deterministic generator (mulberry32) so the seeded transition pairs are the same in every run and language. */
export function seeded(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** 🔤️ The 32 bit FNV-1a hash of a text, the seed of an asset's transition pairs. */
export function hash(text: string): number {
  let h = 2166136261;
  for (let i = 0; i < text.length; i++) h = Math.imul(h ^ text.charCodeAt(i), 16777619);
  return h >>> 0;
}

/** 🔀️ The transition pairs of an asset: every pose to the rest pose (the default pose), plus three seeded random pairs. */
export function transitionPairs(asset: Asset): (readonly [string, string])[] {
  const rest = asset.presentation.defaultPose;
  const ids = asset.poses.map((pose) => pose.id);
  const pairs: (readonly [string, string])[] = ids.filter((id) => id !== rest).map((id) => [id, rest] as const);
  const random = seeded(hash(asset.id));
  for (let i = 0; i < 3 && ids.length > 1; i++) {
    const from = ids[Math.floor(random() * ids.length)]!;
    const others = ids.filter((id) => id !== from);
    pairs.push([from, others[Math.floor(random() * others.length)]!]);
  }
  return pairs;
}

/** 🖨️ The fractions of a transition that are sampled. */
export const TRANSITION_FRACTIONS: readonly number[] = [0, 0.25, 0.5, 0.75, 1];

/** 🧩️ A measurement that is the same at every scale of one authored representation, made once per representation: the first requested denominator that draws it makes it, the others read it. */
export function perRepresentation<T>(asset: Asset, make: (requested: number) => T): (requested: number) => T {
  const known = new Map<string, T>();
  return (requested) => {
    const picked = pickRepresentation(asset, requested);
    const key = picked.kind === "authored" ? `${picked.denominator}` : `unavailable ${requested}`;
    if (!known.has(key)) known.set(key, make(requested));
    return known.get(key)!;
  };
}

/** 🖱️ Every pose, clip instant and transition instant of an asset as subjects; the seven modes render at poses and at the start, key, extreme and end instants, the sampled modes elsewhere. */
export function subjects(asset: Asset, requested: number): Subject[] {
  const poses: Subject[] = asset.poses.map((pose) => ({ id: `pose ${pose.id}`, query: { kind: "pose", pose: pose.id }, isDefault: pose.id === asset.presentation.defaultPose, modes: MODE_LIST, languages: LANGUAGES }));
  const probeScale = scaleSelector(requested);
  const clips = asset.clips.flatMap((clip) => {
    const probe = (t: number): readonly number[] | undefined => {
      try {
        return probeChannels(evaluate(asset, { kind: "clip", clip: clip.id, timeS: t }, probeScale));
      } catch {
        return undefined;
      }
    };
    return clipInstants(asset, clip.id, probe).map((instant): Subject => ({ id: `clip ${clip.id} ${instant.name}`, query: { kind: "clip", clip: clip.id, timeS: instant.t }, isDefault: false, modes: MODE_LIST, languages: instant.tag === "uniform" || instant.tag === "mid" ? ["en"] : LANGUAGES }));
  });
  const transitions = transitionPairs(asset).flatMap(([from, to]) => TRANSITION_FRACTIONS.map((fraction): Subject => ({ id: `transition ${from}>${to} @${fraction}`, query: { kind: "transition", from, to, durationS: 1, timeS: fraction }, isDefault: false, modes: MODE_LIST, languages: ["en"] })));
  return [...poses, ...clips, ...transitions];
}

class Collector {
  readonly cells = new Map<string, { checked: number; failures: Map<string, { count: number; first: { subject: string; message: string }[]; worst?: { deltaMm: number; subject: string; pair: string }; byKey?: Record<string, { deltaMm: number; subject: string }> }> }>();

  constructor(private readonly keyOf: (view: PlanarViewpoint, mode: View) => string) {
    for (const view of VIEWPOINT_LIST) for (const mode of MODE_LIST) this.cells.set(keyOf(view, mode), { checked: 0, failures: new Map() });
  }

  checked(views: readonly PlanarViewpoint[], modes: readonly View[] = MODE_LIST): void {
    for (const view of views) for (const mode of modes) this.cells.get(this.keyOf(view, mode))!.checked++;
  }

  fail(views: readonly PlanarViewpoint[], modes: readonly View[], check: string, subject: string, message: string, delta?: { readonly deltaMm: number; readonly pair: string }): void {
    for (const view of views)
      for (const mode of modes) {
        const cell = this.cells.get(this.keyOf(view, mode))!;
        const entry = cell.failures.get(check) ?? { count: 0, first: [] as { subject: string; message: string }[] };
        entry.count++;
        if (delta !== undefined && (entry.worst === undefined || delta.deltaMm > entry.worst.deltaMm)) entry.worst = { deltaMm: delta.deltaMm, subject, pair: delta.pair };
        if (delta !== undefined && mode === "light-line") {
          const byKey = (entry.byKey ??= {});
          const known = byKey[delta.pair];
          if (known === undefined || delta.deltaMm > known.deltaMm) byKey[delta.pair] = { deltaMm: delta.deltaMm, subject };
        }
        if (entry.first.length < 3) entry.first.push({ subject, message });
        cell.failures.set(check, entry);
      }
  }
}

const finite = (extent: Extent): boolean => Object.values(extent).every(Number.isFinite);

const messageOf = (error: unknown): string => (error instanceof Error ? error.message : String(error)).split("\n")[0]!.slice(0, 220);

/** 🧷️ One finding of the geometry of one evaluated state: the views it concerns, the check, the message and, where measured, the delta with its pair, the bone or the anchor; `tier` is `reported` for the group-drift tier of D41. */
export type GeometryFinding = { readonly views: readonly PlanarViewpoint[]; readonly check: string; readonly tier: "hard" | "reported"; readonly message: string; readonly delta?: { readonly deltaMm: number; readonly pair: string }; readonly bone?: string; readonly anchor?: string; readonly anchorKind?: string };

/** 🧲️ The geometry findings of one evaluated state: empty or non-finite views, the synchrony of the extents of the three views, the per-bone synchrony ({@link boneSync}), the anchor landmarks ({@link anchorLandmarks}) and, for the default pose, the extents against the dimensions. */
export function geometryFindings(asset: Asset, state: EvaluatedState, isDefault: boolean): GeometryFinding[] {
  const tolerance = toleranceM(asset);
  const findings: GeometryFinding[] = [];
  const extents = Object.fromEntries(VIEWPOINT_LIST.map((view) => [view, extentOf(state.shapes[view])])) as Record<PlanarViewpoint, Extent | undefined>;
  for (const view of VIEWPOINT_LIST) {
    const extent = extents[view];
    if (extent === undefined) findings.push({ views: [view], check: `${view}-empty`, tier: "hard", message: `${view} evaluates to no geometry` });
    else if (!finite(extent)) findings.push({ views: [view], check: `${view}-bounds`, tier: "hard", message: `${view} has non-finite bounds ${JSON.stringify(extent)}` });
  }
  const { elevation: e, front: f, plan: p } = extents;
  const agree = (check: string, views: readonly PlanarViewpoint[], label: string, a: readonly [number, number], b: readonly [number, number]): void => {
    const delta = Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1]));
    if (!(micrometres(delta) <= micrometres(tolerance))) findings.push({ views, check, tier: "hard", message: `${label} differ: [${a[0].toFixed(3)}, ${a[1].toFixed(3)}] vs [${b[0].toFixed(3)}, ${b[1].toFixed(3)}] m (tolerance ${tolerance.toFixed(3)})`, delta: { deltaMm: delta * 1000, pair: views.join("/") } });
  };
  if (e && p) agree("sync-forward", ["elevation", "plan"], "elevation x and plan x", [e.minX, e.maxX], [p.minX, p.maxX]);
  if (e && f) agree("sync-height", ["elevation", "front"], "elevation y and front y", [e.minY, e.maxY], [f.minY, f.maxY]);
  if (f && p) agree("sync-lateral", ["front", "plan"], "front x and plan y", [f.minX, f.maxX], [p.minY, p.maxY]);
  for (const finding of boneSync(state, tolerance)) findings.push({ views: finding.pair.split("/") as PlanarViewpoint[], check: finding.check, tier: finding.tier === "bone" ? "hard" : "reported", message: finding.message, delta: { deltaMm: finding.deltaMm, pair: `${finding.pair} ${finding.bone}` }, bone: finding.bone });
  for (const finding of anchorLandmarks(state, tolerance)) findings.push({ views: [finding.view], check: "anchor-landmark", tier: "hard", message: finding.message, delta: { deltaMm: finding.deltaMm, pair: `${finding.view} ${finding.anchor}` }, bone: finding.bone, anchor: finding.anchor, anchorKind: finding.kind });
  if (!isDefault) return findings;
  const size = (check: string, view: PlanarViewpoint, label: string, measured: number, nominal: number): void => {
    if (!withinD18(measured, nominal)) findings.push({ views: [view], check, tier: "hard", message: `${label} is ${measured.toFixed(3)} m but the dimensions say ${nominal.toFixed(3)} m`, delta: { deltaMm: Math.abs(measured - nominal) * 1000, pair: `${view} vs dimensions` } });
  };
  const d = asset.dimensions;
  if (e) {
    size("dimension-elevation-width", "elevation", "elevation x extent (widthM)", e.maxX - e.minX, d.widthM);
    size("dimension-elevation-height", "elevation", "elevation height (heightM)", e.maxY - e.minY, d.heightM);
  }
  if (f) {
    size("dimension-front-width", "front", "front lateral extent (depthM)", f.maxX - f.minX, d.depthM);
    size("dimension-front-height", "front", "front height (heightM)", f.maxY - f.minY, d.heightM);
  }
  if (p) {
    size("dimension-plan-length", "plan", "plan x extent (widthM)", p.maxX - p.minX, d.widthM);
    size("dimension-plan-width", "plan", "plan lateral extent (depthM)", p.maxY - p.minY, d.depthM);
  }
  return findings;
}

function checkGeometry(asset: Asset, subject: Subject, state: EvaluatedState, collector: Collector): void {
  for (const finding of geometryFindings(asset, state, subject.isDefault)) collector.fail(finding.views, MODE_LIST, finding.check, subject.id, finding.message, finding.delta);
}

/** 🕳️ Why a scene draws none of the shapes of a view: per shape its role, pen (the shape's own, else the pen of its role in the representation or the symbol that is drawn) and filled flag, then the reason class (`zero-pen`, `degenerate`, `occluded-or-unexplained`). */
export function blankReason(state: EvaluatedState, view: PlanarViewpoint, mode: View): string {
  const drawn = state.representation;
  const rep = drawn.kind === "scale" ? state.asset.representations[`${drawn.denominator}` as keyof Asset["representations"]] : undefined;
  const source = drawn.kind === "symbol" ? state.asset.symbols.find((symbol) => symbol.id === drawn.symbol) : rep?.status === "authored" ? rep : undefined;
  const pens = source?.pens as unknown as Record<string, number | null> | undefined;
  const lines = state.shapes[view]
    .filter((e) => e.shape.role !== "ground")
    .map((e) => {
      const points = e.rings.flat();
      const w = points.length === 0 ? 0 : Math.max(...points.map((p) => p.x)) - Math.min(...points.map((p) => p.x));
      const h = points.length === 0 ? 0 : Math.max(...points.map((p) => p.y)) - Math.min(...points.map((p) => p.y));
      const pen = e.shape.penMm ?? pens?.[e.shape.role] ?? null;
      const reason = points.length === 0 || (w < 1e-9 && h < 1e-9) ? "degenerate" : (pen === null || pen === 0) && !e.shape.filled ? "zero-pen" : "occluded-or-unexplained";
      return `${e.shape.id}[role ${e.shape.role}, pen ${pen === null ? "none" : pen} mm, ${e.shape.filled ? "filled" : "open"}, ${reason}]`;
    });
  return `${mode}: ${lines.length} shapes: ${lines.slice(0, 6).join("; ")}`;
}

/** 🫥️ The blank finding of a scene of a line, solid or thermal mode: no non-ground shape of the view is drawn, neither by its fill nor by an outline run ({@link paperIds}), so a part or a symbol drawn as an unfilled outline is drawn; the forces and moments modes draw the structure and are not asked. */
export function blankFinding(state: EvaluatedState, view: PlanarViewpoint, mode: View, scene: DrawingScene): { readonly check: "blank-shapes"; readonly message: string } | undefined {
  if (mode === "forces" || mode === "moments" || paperIds(scene, state.shapes[view]).size > 0) return undefined;
  return { check: "blank-shapes", message: `no ${view} shape of the representation is drawn: ${blankReason(state, view, mode)}` };
}

/** 📰️ The paper findings of the default pose of one view: the box of the drawn fills and outline runs ({@link paperBox}) against the dimensions at the scale of the scene, within D18 as {@link withinD18} decides it. */
export function paperFindings(asset: Asset, view: PlanarViewpoint, scene: DrawingScene, shapes: readonly EvaluatedShape[]): { readonly check: "paper-width" | "paper-height"; readonly message: string; readonly delta: { readonly deltaMm: number; readonly pair: string } }[] {
  const box = paperBox(scene, shapes);
  if (box === undefined) return [];
  const k = scene.metadata.mmPerMetre;
  const d = asset.dimensions;
  const [w, h] = view === "elevation" ? [d.widthM, d.heightM] : view === "front" ? [d.depthM, d.heightM] : [d.widthM, d.depthM];
  const gotW = box.maxX - box.minX;
  const gotH = box.maxY - box.minY;
  const pair = `${view} paper vs dimensions`;
  return [
    ...(!withinD18(gotW / k, w) ? [{ check: "paper-width" as const, message: `drawn width ${gotW.toFixed(2)} mm but ${w}·${k.toFixed(3)} = ${(w * k).toFixed(2)} mm`, delta: { deltaMm: (Math.abs(gotW - w * k) / k) * 1000, pair } }] : []),
    ...(!withinD18(gotH / k, h) ? [{ check: "paper-height" as const, message: `drawn height ${gotH.toFixed(2)} mm but ${h}·${k.toFixed(3)} = ${(h * k).toFixed(2)} mm`, delta: { deltaMm: (Math.abs(gotH - h * k) / k) * 1000, pair } }] : []),
  ];
}

function checkScene(asset: Asset, subject: Subject, state: EvaluatedState, view: PlanarViewpoint, mode: View, scene: DrawingScene, requested: number, collector: Collector): void {
  const fail = (check: string, message: string): void => collector.fail([view], [mode], check, subject.id, message);
  const m = scene.metadata;
  if (m.viewpoint !== view) fail("scene-viewpoint", `scene is the ${m.viewpoint} viewpoint`);
  if (m.view !== mode) fail("scene-mode", `scene is the ${m.view} mode`);
  if (m.timeS !== state.timeS) fail("scene-time", `scene time ${m.timeS} differs from the state time ${state.timeS}`);
  if (JSON.stringify(m.query) !== JSON.stringify(state.query)) fail("scene-query", "scene query differs from the state query");
  if (m.scale.denominator !== requested) fail("scene-scale", `scene scale 1:${m.scale.denominator} differs from the requested 1:${requested}`);
  if (!(Number.isFinite(scene.widthMm) && Number.isFinite(scene.heightMm) && scene.widthMm > 0 && scene.heightMm > 0)) fail("scene-size", `scene size ${scene.widthMm} x ${scene.heightMm} mm`);
  const box = sceneBox(scene);
  if (box === undefined) return fail("blank", "the scene draws no path");
  if (!finite(box)) return fail("scene-bounds", "the scene has non-finite coordinates");
  const blank = blankFinding(state, view, mode, scene);
  if (blank !== undefined) fail(blank.check, blank.message);
  if (subject.isDefault) for (const finding of paperFindings(asset, view, scene, state.shapes[view])) collector.fail([view], [mode], finding.check, subject.id, finding.message, finding.delta);
}

export function drawOptions(mode: View, denominator: number, textless = false, language: Language = "en"): DrawOptions {
  const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...rest } = textless ? textlessDepictionOptions(mode, denominator) : defaultDepictionOptions(mode, language, denominator);
  return rest as DrawOptions;
}

/** 📐️ The shape paths of a scene by id, each as its command ops and its model-space coordinates in metres (paper coordinates relative to `metadata.origin`, divided by `mmPerMetre`): a label that widens the sheet moves the origin and translates every path, never the geometry. */
export const shapeGeometry = (scene: DrawingScene): [string, string, number[]][] => {
  const { origin, mmPerMetre } = scene.metadata;
  return scene.layers.flatMap((layer) =>
    layer.items.flatMap((item): [string, string, number[]][] =>
      item.kind === "path" && item.id.startsWith("shape:")
        ? [[item.id, item.commands.map((c) => c.op).join(""), item.commands.flatMap((c) => (c.op === "Z" ? [] : c.op === "C" ? [c.x1, c.y1, c.x2, c.y2, c.x, c.y] : [c.x, c.y]).map((v, i) => (i % 2 === 0 ? v - origin.xMm : origin.yMm - v) / mmPerMetre))]]
        : [],
    ),
  );
};

/** 🟰️ Whether two shape geometries ({@link shapeGeometry}) are the same shapes in the same order within a nanometre of model space. */
export const sameShapeGeometry = (a: ReturnType<typeof shapeGeometry>, b: ReturnType<typeof shapeGeometry>): boolean =>
  a.length === b.length && a.every(([id, ops, xs], i) => id === b[i]![0] && ops === b[i]![1] && xs.length === b[i]![2].length && xs.every((v, j) => Math.abs(v - b[i]![2][j]!) <= 1e-9));

/** 🦓️ Whether a scene carries a colour-independent cue: a hatch pattern or a dashed stroke. */
export const hasCue = (scene: DrawingScene): boolean => scene.layers.some((layer) => layer.items.some((item) => item.kind === "path" && (item.fill?.pattern !== undefined || (item.stroke?.dashMm?.length ?? 0) > 0)));

/** 🚦️ The analysis modes and the share of the range (per cent) below which each leaves a value unhatched: within 35 % of the centre in the thermal mode, up to 15 % of its side of the range in the forces and moments modes. */
export const UNHATCHED_PERCENT: Readonly<Partial<Record<View, number>>> = { thermal: 35, forces: 15, moments: 15 };

/** 🪧️ The cue finding of an analysis scene, one rule for the three analysis modes: a scene without hatch pattern and without dashed stroke is accepted only when its legend states, in English and German, the share below which a value is drawn unhatched, so the unhatched state is itself a reading. */
export function cueFinding(mode: View, scene: DrawingScene): { readonly check: "analysis-cue-legend"; readonly message: string } | undefined {
  const percent = UNHATCHED_PERCENT[mode];
  if (percent === undefined || hasCue(scene)) return undefined;
  const legend = JSON.stringify(scene.legends.map((l) => [l.convention, l.interpretation]));
  if (new RegExp(`${percent}\\s?%`, "u").test(legend) && /unhatched|without hatch/iu.test(legend) && /schraffiert|ohne Schraffur/iu.test(legend)) return undefined;
  return { check: "analysis-cue-legend", message: `the ${mode} scene has neither a hatch pattern nor a dashed stroke under default options and its legend does not state that a value within ${percent} % is drawn unhatched` };
}

function renderSubject(asset: Asset, subject: Subject, source: DrawingSource, collector: Collector, symbol?: SymbolRepresentation): void {
  let geometryDone = false;
  const compared = subject.languages.length > 1;
  const english = new Map<string, ReturnType<typeof shapeGeometry>>();
  for (const language of subject.languages)
    for (const mode of subject.modes) {
      let multi: MultiView;
      try {
        multi = resolveViews(asset, subject.query, source, mode, drawOptions(mode, source.denominator, false, language));
      } catch (error) {
        collector.checked(VIEWPOINT_LIST, [mode]);
        collector.fail(VIEWPOINT_LIST, [mode], "resolve-error", `${subject.id} ${language}`, messageOf(error));
        continue;
      }
      if (multi.state !== null && !geometryDone && symbol === undefined) {
        checkGeometry(asset, subject, multi.state, collector);
        geometryDone = true;
      }
      for (const view of VIEWPOINT_LIST) {
        const drawing = multi.views[view];
        collector.checked([view], [mode]);
        if (drawing.kind === "unavailable") {
          collector.fail([view], [mode], `unavailable-${drawing.code}`, subject.id, drawing.reason[language]);
          continue;
        }
        if (drawing.state !== multi.state) collector.fail([view], [mode], "state-identity", subject.id, "the view was projected from another evaluated state");
        if (symbol !== undefined && drawing.fidelity === "conventional-symbol" && !CONVENTIONAL_NOTE.test(`${symbol.dimensionalNote.en} ${symbol.dimensionalNote.de}`)) collector.fail([view], [mode], "symbol-conventional-note", subject.id, "a conventional symbol does not say that it is a conventional sign");
        checkScene(asset, subject, drawing.state, view, mode, drawing.scene, source.denominator, collector);
        const cue = cueFinding(mode, drawing.scene);
        if (cue !== undefined) collector.fail([view], [mode], cue.check, subject.id, cue.message);
        const key = `${view}|${mode}`;
        if (language === "en" && compared) english.set(key, shapeGeometry(drawing.scene));
        else if (english.get(key) !== undefined && !sameShapeGeometry(english.get(key)!, shapeGeometry(drawing.scene))) collector.fail([view], [mode], "language-geometry", subject.id, `the ${language} drawing has other shape geometry than the English one`);
      }
    }
}

const CONVENTIONAL_NOTE = /conventional|not to scale|nicht ma(ss|ß)st|konvention/iu;

function toCells(asset: Asset, entry: ScaleEntry, collector: Collector, reason?: string): Cell[] {
  return VIEWPOINT_LIST.flatMap((view) =>
    MODE_LIST.map((mode): Cell => {
      const raw = collector.cells.get(cellKeyOf(view, mode, asset, entry))!;
      const all = Object.fromEntries([...raw.failures].map(([check, value]) => [check, value]));
      const rangeEndCheck = (entry.kind === "band-min" || entry.kind === "band-max") && all["unavailable-scale-unsuitable"] !== undefined;
      const { "unavailable-scale-unsuitable": rangeEnd, ...rest } = all;
      const groupChecks = Object.keys(rest).filter((check) => check.startsWith("group-drift-"));
      const hard = Object.fromEntries(Object.entries(rangeEndCheck ? rest : all).filter(([check]) => !check.startsWith("group-drift-")));
      const failures = hard;
      const groupDrift = Object.fromEntries(groupChecks.map((check) => [check, rest[check]!]));
      if (raw.checked === 0 && Object.keys(failures).length === 0) failures["cell-unchecked"] = { count: 1, first: [{ subject: "cell", message: "no subject was checked in this cell" }] };
      const status: Status = Object.keys(failures).length > 0 ? "fail" : groupChecks.length > 0 ? "group-drift" : rangeEndCheck ? "range-end" : reason !== undefined ? "unavailable" : "ok";
      return { asset: asset.id, category: asset.category, scale: entry.scale, scaleKind: entry.kind, requested: entry.requested, view, mode, status, checked: raw.checked, ...(reason !== undefined ? { reason } : {}), failures, ...(rangeEndCheck ? { rangeEnd: { "range-end-unsuitable": rangeEnd! } } : {}), ...(groupChecks.length > 0 ? { groupDrift } : {}) };
    }),
  );
}

const cellKeyOf = (view: PlanarViewpoint, mode: View, asset: Asset, entry: ScaleEntry): string => cellKey(asset.id, entry.scale, view, mode);
const collectorOf = (asset: Asset, entry: ScaleEntry): Collector => new Collector((view, mode) => cellKeyOf(view, mode, asset, entry));

const filled = (text: { readonly en: string; readonly de: string } | undefined): boolean => text !== undefined && text.en.trim().length > 0 && text.de.trim().length > 0;

function declaration(asset: Asset, entry: ScaleEntry, catalogById: CatalogLookup): { collector: Collector; reason: string } {
  const collector = collectorOf(asset, entry);
  const decision = asset.representations[entry.representation as keyof Asset["representations"]];
  const fail = (check: string, message: string): void => collector.fail(VIEWPOINT_LIST, MODE_LIST, check, `declaration 1:${entry.requested}`, message);
  collector.checked(VIEWPOINT_LIST);
  if (decision?.status !== "unavailable") return { collector, reason: "" };
  if (!filled(decision.reason)) fail("unavailable-reason", "the reason lacks an English or German text");
  if (!filled(decision.limitingFeature)) fail("unavailable-limiting-feature", "the limiting feature lacks an English or German text");
  if (!(decision.limitingSizeM > 0)) fail("unavailable-limiting-size", `limitingSizeM ${decision.limitingSizeM} is not positive`);
  if (decision.alternatives.length === 0) fail("unavailable-alternatives", "no alternative is offered");
  if (decision.cause !== "below-printable-feature" && decision.cause !== "no-physical-depiction") fail("unavailable-cause", `cause is ${JSON.stringify(decision.cause)}`);
  try {
    const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator: Number(entry.representation) }, "light-line", drawOptions("light-line", Number(entry.representation)));
    for (const view of VIEWPOINT_LIST) {
      const drawing = multi.views[view];
      if (drawing.kind !== "unavailable" || drawing.code !== "scale-unavailable") collector.fail([view], MODE_LIST, "unavailable-resolver", "declaration", `the resolver does not report ${view} as scale-unavailable`);
      else if (drawing.alternatives.length === 0 || !filled(drawing.reason)) collector.fail([view], MODE_LIST, "unavailable-resolver", "declaration", `the resolver answer for ${view} lacks reason or alternatives`);
    }
  } catch (error) {
    fail("unavailable-resolver", messageOf(error));
  }
  const picked = pickRepresentation(asset, Number(entry.representation));
  if (picked.kind !== "unavailable") fail("unavailable-resolution", `the resolver returns ${picked.kind} for a declared unavailable scale`);
  decision.alternatives.forEach((alternative, i) => {
    if (alternative.kind === "scale") {
      const target = asset.representations[`${alternative.denominator}` as keyof Asset["representations"]];
      if (target?.status !== "authored") fail("alternative-scale", `alternative ${i}: 1:${alternative.denominator} is not authored`);
    } else if (alternative.kind === "symbol") {
      if (!asset.symbols.some((symbol) => symbol.id === alternative.symbol)) fail("alternative-symbol", `alternative ${i}: the asset has no symbol "${alternative.symbol}"`);
    } else if (!catalogById.has(alternative.asset)) fail("alternative-asset", `alternative ${i}: the catalog has no asset "${alternative.asset}"`);
  });
  return { collector, reason: filled(decision.reason) ? decision.reason.en : "declared unavailable without reason" };
}

/** 🧷️ What a run computes once per representation: the subjects with their extreme instants and the loop findings. */
type Measured = { readonly subjects: (requested: number) => Subject[]; readonly loops: (requested: number) => LoopFinding[] };

const measuredOf = (asset: Asset): Measured => ({ subjects: perRepresentation(asset, (requested) => subjects(asset, requested)), loops: perRepresentation(asset, (requested) => loopFindings(asset, requested)) });

function runAuthored(asset: Asset, entry: ScaleEntry, measured: Measured): Collector {
  const collector = collectorOf(asset, entry);
  const requested = entry.requested!;
  const source: DrawingSource = { kind: "scale", denominator: requested };
  const reduced = entry.kind === "band-min" || entry.kind === "band-max";
  const all = measured.subjects(requested);
  for (const subject of reduced ? all.filter((candidate) => candidate.query.kind === "pose" || candidate.id.includes(" start @") || candidate.id.includes(" end @")) : all) renderSubject(asset, subject, source, collector);
  for (const finding of measured.loops(requested)) collector.fail(finding.views, MODE_LIST, "loop-closure", finding.subject, finding.message);
  if (entry.kind === "preset" || entry.kind === "declared") checkDerived(asset, entry, collector);
  return collector;
}

/** 📷️ One allowlisted symmetric view: the asset, the viewpoint and the physical reason in English and German (a body of revolution has the same front and side by nature); with `bands` (one or more scale keys) the allowance holds in those bands only (D51: a slender body whose two views differ by less than the measure can resolve in the coarse bands), without it in every band. */
export type DerivedAllowance = { readonly asset: string; readonly view: PlanarViewpoint; readonly bands?: readonly string[]; readonly reason: { readonly en: string; readonly de: string } };

/** 🧱️ One allowlisted band copy: the asset, the viewpoint, the scale keys (two or more different ones, in any order) whose drawing of that view is truthfully the same, so a copy between any two of them is allowed, and the physical reason in English and German (a prismatic or plain part, such as a rendered wall face, has nothing a finer band could add). */
export type BandCopyAllowance = { readonly asset: string; readonly view: PlanarViewpoint; readonly bands: readonly string[]; readonly reason: { readonly en: string; readonly de: string } };

/** 📜️ The two allowlists of the derived-view checks. */
export type Allowlists = { readonly derived: readonly DerivedAllowance[]; readonly bandCopy: readonly BandCopyAllowance[] };

/** 🗝️ The allowlists read from the fixture of the vectors: `derivedViewAllowlist` (genuinely symmetric bodies) and `bandCopyAllowlist` (views that are truthfully the same drawing in two bands). */
export function allowlists(): Allowlists {
  const file = join(STAFFAGE_ROOT, "🧫️fixtures", "🔭️multiview-coverage", "🔣️.json");
  if (!existsSync(file)) return { derived: [], bandCopy: [] };
  const document = JSON.parse(readFileSync(file, "utf8")) as { derivedViewAllowlist?: DerivedAllowance[]; bandCopyAllowlist?: BandCopyAllowance[] };
  return { derived: document.derivedViewAllowlist ?? [], bandCopy: document.bandCopyAllowlist ?? [] };
}

/** 🧬️ The derived-view findings of the default pose of one representation (or symbol) at a requested denominator: a front or plan that equals the same view of another authored band (`derived-view-band-copy`) or another view of its own band (`derived-view-in-band`), unless an allowlist names the asset and view (for a band copy the two bands, for an in-band view optionally the bands it holds in) with a physical reason in English and German; an entry without both texts allows nothing. */
export function derivedFindings(asset: Asset, representation: string, requested: number, symbol?: string, allow: Allowlists = allowlists()): { readonly view: PlanarViewpoint; readonly check: "derived-view-band-copy" | "derived-view-in-band"; readonly against: string; readonly message: string }[] {
  const select = (denominator: number): RepresentationSelector => (symbol === undefined ? scaleSelector(denominator) : { kind: "symbol", symbol });
  const shapesAt = (denominator: number) => evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, select(denominator)).shapes;
  const others = symbol !== undefined ? {} : Object.fromEntries(Object.keys(asset.representations).filter((key) => key !== representation && authoredOf(asset, key) !== undefined).map((key) => [`1:${key}`, shapesAt(Number(key))]));
  const reasoned = (allowance: DerivedAllowance | BandCopyAllowance): boolean => allowance.asset === asset.id && filled(allowance.reason);
  const symmetric = allow.derived.filter(reasoned);
  const copies = allow.bandCopy.filter(reasoned);
  const allowed = (flag: { readonly view: PlanarViewpoint; readonly against: string }): boolean =>
    flag.against.startsWith("1:") ? copies.some((allowance) => allowance.view === flag.view && representation !== flag.against.slice(2).split(" ")[0] && allowance.bands.includes(representation) && allowance.bands.includes(flag.against.slice(2).split(" ")[0]!)) : symmetric.some((allowance) => allowance.view === flag.view && (allowance.bands === undefined || allowance.bands.includes(representation)));
  return derivedFlags(shapesAt(requested), others)
    .filter((flag) => !allowed(flag))
    .map((flag) => ({ view: flag.view, check: flag.against.startsWith("1:") ? ("derived-view-band-copy" as const) : ("derived-view-in-band" as const), against: flag.against, message: `the ${flag.view} view equals the ${flag.against} under ${flag.transform} (normalised Hausdorff ${flag.distance})` }));
}

function checkDerived(asset: Asset, entry: ScaleEntry, collector: Collector, symbol?: string): void {
  try {
    for (const finding of derivedFindings(asset, entry.representation, entry.requested!, symbol)) collector.fail([finding.view], MODE_LIST, finding.check, "default pose", finding.message);
    collector.checked(VIEWPOINT_LIST);
  } catch (error) {
    collector.fail(VIEWPOINT_LIST, MODE_LIST, "derived-view", "default pose", messageOf(error));
  }
}

type LoopFinding = { readonly views: readonly PlanarViewpoint[]; readonly subject: string; readonly message: string };

function loopFindings(asset: Asset, requested: number): LoopFinding[] {
  const findings: LoopFinding[] = [];
  for (const clip of asset.clips) {
    if (!clip.loop) continue;
    try {
      const a = evaluate(asset, { kind: "clip", clip: clip.id, timeS: 0 }, scaleSelector(requested));
      const b = evaluate(asset, { kind: "clip", clip: clip.id, timeS: clip.durationS * (1 - 1e-6) }, scaleSelector(requested));
      for (const view of VIEWPOINT_LIST) {
        const ea = extentOf(a.shapes[view]);
        const eb = extentOf(b.shapes[view]);
        if (ea && eb && Math.max(Math.abs(ea.maxY - eb.maxY), Math.abs(ea.minY - eb.minY)) > toleranceM(asset)) findings.push({ views: [view], subject: `clip ${clip.id}`, message: `the ${view} view at the end of the loop differs from its start` });
      }
    } catch (error) {
      findings.push({ views: VIEWPOINT_LIST, subject: `clip ${clip.id}`, message: messageOf(error) });
    }
  }
  return findings;
}

function runSymbol(asset: Asset, entry: ScaleEntry, symbolId: string): Collector {
  const collector = collectorOf(asset, entry);
  const requested = entry.requested!;
  const symbol = asset.symbols.find((candidate) => candidate.id === symbolId);
  if (symbol === undefined) {
    collector.checked(VIEWPOINT_LIST);
    collector.fail(VIEWPOINT_LIST, MODE_LIST, "alternative-symbol", `symbol ${symbolId}`, `the asset has no symbol "${symbolId}"`);
    return collector;
  }
  for (const view of VIEWPOINT_LIST) {
    const geometry = symbol.geometry[view];
    if (geometry === undefined || geometry.length === 0) {
      collector.checked([view]);
      collector.fail([view], MODE_LIST, `symbol-${view}-missing`, `symbol ${symbolId}`, `the symbol authors no ${view} geometry`);
    }
  }
  checkDerived(asset, entry, collector, symbolId);
  const source: DrawingSource = { kind: "symbol", symbol: symbolId, denominator: requested };
  for (const pose of asset.poses) renderSubject(asset, { id: `pose ${pose.id}`, query: { kind: "pose", pose: pose.id }, isDefault: false, modes: MODE_LIST, languages: ["en"] }, source, collector, symbol);
  return collector;
}

/** 🧮️ Runs every cell of one asset: every scale entry with its declarations, geometry, instants, renders and alternatives. `catalogById` answers the asset alternatives ({@link catalogLookup} reads them on demand). `onEntry` reports the progress before each scale entry (its index, the count and its scale) and stops the run with the error it throws, which is how a caller cancels. */
export function runAsset(asset: Asset, catalogById: CatalogLookup, onEntry?: (index: number, count: number, scale: string) => void): Cell[] {
  const cells: Cell[] = [];
  const byScale = new Map<string, Cell[]>();
  const measured = measuredOf(asset);
  const ordered = scaleEntries(asset);
  const sequence = [...ordered.filter((candidate) => candidate.kind !== "alternative"), ...ordered.filter((candidate) => candidate.kind === "alternative")];
  for (const [position, entry] of sequence.entries()) {
    onEntry?.(position, sequence.length, entry.scale);
    const decision = asset.representations[entry.representation as keyof Asset["representations"]];
    let produced: Cell[];
    if (entry.kind === "alternative") {
      const index = Number(entry.scale.split(">alt")[1]!.split(":")[0]);
      const alternative = (decision as Extract<typeof decision, { status: "unavailable" }>).alternatives[index]!;
      if (alternative.kind === "symbol") produced = toCells(asset, entry, runSymbol(asset, entry, alternative.symbol));
      else if (alternative.kind === "scale") {
        const target = byScale.get(`${alternative.denominator}`);
        produced = target === undefined ? toCells(asset, entry, failAll(asset, entry, "alternative-scale", `1:${alternative.denominator} is not authored`)) : target.map((cell) => ({ ...cell, scale: entry.scale, scaleKind: "alternative" as const, requested: entry.requested }));
      } else {
        const other = catalogById.get(alternative.asset);
        const picked = other === undefined ? undefined : pickRepresentation(other, entry.requested!);
        produced = toCells(asset, entry, other === undefined ? failAll(asset, entry, "alternative-asset", `no asset "${alternative.asset}"`) : picked!.kind !== "authored" ? failAll(asset, entry, "alternative-asset-scale", `"${alternative.asset}" has no authored 1:${entry.requested}`) : okAll(asset, entry));
      }
    } else if (decision === undefined) produced = toCells(asset, entry, failAll(asset, entry, "representation-missing", `the asset decides no representation for 1:${entry.scale}`));
    else if (decision.status === "unavailable") {
      const { collector, reason } = declaration(asset, entry, catalogById);
      produced = toCells(asset, entry, collector, reason);
    } else {
      const collector = runAuthored(asset, entry, measured);
      const declared = declaration(asset, entry, catalogById);
      for (const [key, value] of declared.collector.cells) collector.cells.get(key)!.checked += value.checked;
      produced = toCells(asset, entry, collector);
    }
    byScale.set(entry.scale, produced);
    cells.push(...produced);
  }
  return cells;
}

function failAll(asset: Asset, entry: ScaleEntry, check: string, message: string): Collector {
  const collector = collectorOf(asset, entry);
  collector.checked(VIEWPOINT_LIST);
  collector.fail(VIEWPOINT_LIST, MODE_LIST, check, `scale ${entry.scale}`, message);
  return collector;
}

function okAll(asset: Asset, entry: ScaleEntry): Collector {
  const collector = collectorOf(asset, entry);
  collector.checked(VIEWPOINT_LIST);
  return collector;
}

/** 🧯️ The cells of an asset whose run did not end: every cell of its universe as a failure named `code` (`asset-crashed`, `asset-timeout`) that carries the message, so a crash or a timeout inside one asset is never a silent `cell-missing`. */
export function failedAsset(asset: Asset, code: string, message: string): Cell[] {
  return failedCells(declaredOf(asset), code, message);
}

/** 🪦️ {@link failedAsset} from the {@link Declared} of the asset. */
export function failedCells(declared: Declared, code: string, message: string): Cell[] {
  return declared.entries.flatMap((entry) => VIEWPOINT_LIST.flatMap((view) => MODE_LIST.map((mode): Cell => ({ asset: declared.id, category: declared.category, scale: entry.scale, scaleKind: entry.kind, requested: entry.requested, view, mode, status: "fail", checked: 0, failures: { [code]: { count: 1, first: [{ subject: declared.id, message }] } } }))));
}

/** 🧶️ The cells of one asset in the order of the matrix: every universe cell of it exactly once, taken from `produced` by key, a missing one as a `cell-missing` failure. */
export function placeCells(declared: Declared, produced: ReadonlyMap<string, Cell>): Cell[] {
  return keysOf(declared).map((key) => {
    const found = produced.get(key);
    if (found !== undefined) return found;
    const [assetId, scale, view, mode] = key.split("|") as [string, string, PlanarViewpoint, View];
    return { asset: assetId, category: declared.category, scale, scaleKind: "preset" as const, requested: null, view, mode, status: "fail" as const, checked: 0, failures: { "cell-missing": { count: 1, first: [{ subject: key, message: "no runner produced this cell" }] } } };
  });
}

/** 🪢️ The cells by their key. */
export const cellsByKey = (cells: readonly Cell[]): Map<string, Cell> => new Map(cells.map((cell) => [cellKey(cell.asset, cell.scale, cell.view, cell.mode), cell]));

/** 🚨️ The result of the matrix: every universe cell exactly once, missing ones as `cell-missing` failures. */
export function assemble(assets: readonly Asset[], produced: readonly Cell[]): Cell[] {
  const byKey = cellsByKey(produced);
  return assets.flatMap((asset) => placeCells(declaredOf(asset), byKey));
}

/** 🏗️ Builds the matrix of the assets sequentially. */
export function buildMatrix(assets: readonly Asset[], catalogAssets: readonly Asset[] = assets): Cell[] {
  const catalogById = new Map(catalogAssets.map((asset) => [asset.id, asset]));
  return assemble(assets, assets.flatMap((asset) => runAsset(asset, catalogById)));
}

/** 📊️ The counts of one group of cells. */
export type Counts = { ok: number; unavailable: number; "range-end": number; "group-drift": number; fail: number };

/** 📮️ The summary of a matrix: counts by category, then by viewpoint, and the failing checks by number of cells. */
export type Summary = { total: Counts; byCategory: Record<string, { total: Counts; byView: Record<PlanarViewpoint, Counts>; assets: number; failingAssets: number }>; checks: Record<string, number> };

/** 🧺️ Summarises cells as they come: `add` counts a batch and keeps none of it, `result` is the {@link Summary} of everything added, the same as {@link summarise} of the batches in one array. */
export function summariser(): { readonly add: (cells: readonly Cell[]) => void; readonly result: () => Summary } {
  const zero = (): Counts => ({ ok: 0, unavailable: 0, "range-end": 0, "group-drift": 0, fail: 0 });
  const total = zero();
  const byCategory: Summary["byCategory"] = {};
  const checks: Record<string, number> = {};
  const assetsIn = new Map<string, Set<string>>();
  const failingIn = new Map<string, Set<string>>();
  const add = (cells: readonly Cell[]): void => {
    for (const cell of cells) {
      const group = (byCategory[cell.category] ??= { total: zero(), byView: { elevation: zero(), front: zero(), plan: zero() }, assets: 0, failingAssets: 0 });
      total[cell.status]++;
      group.total[cell.status]++;
      group.byView[cell.view][cell.status]++;
      (assetsIn.get(cell.category) ?? assetsIn.set(cell.category, new Set()).get(cell.category)!).add(cell.asset);
      if (cell.status === "fail") (failingIn.get(cell.category) ?? failingIn.set(cell.category, new Set()).get(cell.category)!).add(cell.asset);
      for (const check of Object.keys(cell.failures)) checks[check] = (checks[check] ?? 0) + 1;
    }
  };
  const result = (): Summary => {
    for (const [category, group] of Object.entries(byCategory)) {
      group.assets = assetsIn.get(category)?.size ?? 0;
      group.failingAssets = failingIn.get(category)?.size ?? 0;
    }
    return { total, byCategory, checks: Object.fromEntries(Object.entries(checks).sort((a, b) => b[1] - a[1])) };
  };
  return { add, result };
}

/** 🛑️ Counts by category, then by viewpoint, and the failing checks by number of cells. */
export function summarise(cells: readonly Cell[]): Summary {
  const summary = summariser();
  summary.add(cells);
  return summary.result();
}

/** 📥️ The categories in schema order. */
export const CATEGORY_ORDER: readonly string[] = CATEGORIES;

/** 🔖️ The authored representation of a scale key, or undefined. */
export const authoredOf = (asset: Asset, key: string): AuthoredRepresentation | undefined => {
  const decision = asset.representations[key as keyof Asset["representations"]];
  return decision?.status === "authored" ? decision : undefined;
};
