/** 🎞️ Deterministic clip sampling: eases, time wrapping, key interpolation, scalar tracks, contacts, phases, thermal bindings, load scales, transitions and planted root motion.
 *
 * Sampling is a pure function of (asset, clip, time). A key's `ease` shapes the way to the next key: `linear`, `in`, `out`, `step` blend the two key states by the eased weight (angles linearly in degrees, translations linearly); `smooth` is a cubic Hermite (Catmull-Rom) segment through the neighbouring keys, so offsets, root and scalar tracks have continuous velocity across keys (zero end tangents for non-looping clips, wrap-around tangents for looping ones). Thermal bindings and loads blend by the smoothstep weight.
 * looping clips wrap past the last key towards the first (at `first.t + duration`) unless they end with a closing key at the duration, which is the end of the last segment and may differ from the first key by whole turns of an unlimited revolute joint (each cycle then adds that difference, so wheels keep spinning); others clamp. Root motion `plant` integrates at {@link PLANT_HZ} from t = 0 over the span boundaries:
 * the earliest-planted active contact anchor is held at the world x it had when its span began, and the root x follows; when a later-planted span outlasts it, the root crossfades (smoothstep over the overlap) to the new anchor, so x and its velocity stay continuous at handovers; free spans continue with the last velocity. Looping clips advance
 * by `advanceM` per cycle. `ground: "auto"` puts the lowest active contact anchor on y = 0.
 *
 * @see ../../🧬️schema/🟦️.ts — `Clip`, `ClipKey`, `ScalarTrack`, `ThermalBinding`
 * @see ../🔗️contracts/🟦️.ts — `ThermalInputs`
 * @see https://en.wikipedia.org/wiki/Smoothstep — the `smooth` ease
 */
import type { Anchor, AppliedLoad, Asset, BoneOffset, Clip, ContactSpan, Ease, Pose, Slug, ThermalBinding } from "../../🧬️schema/🟦️.ts";
import type { Affine, ThermalInputs } from "../🔗️contracts/🟦️.ts";
import { apply, solveRig } from "../🦴️rig/🟦️.ts";

/** ⏱️ The fixed integration rate of planted root motion, in hertz. */
export const PLANT_HZ = 240;

/** 🧾️ One bone offset with all components present. */
export type FullOffset = { readonly angleDeg: number; readonly dx: number; readonly dy: number };

/** 🎬️ Everything a clip or pose defines at one instant. `timeS` is the wrapped or clamped clip time; `loads` are blended but not yet multiplied by `loadScales`. */
export type Sample = {
  readonly timeS: number;
  readonly frame: number;
  readonly offsets: Readonly<Record<Slug, FullOffset>>;
  readonly root: { readonly x: number; readonly y: number; readonly angleDeg: number };
  readonly contacts: readonly Slug[];
  readonly thermal: ThermalInputs;
  readonly loadScales: Readonly<Record<Slug, number>>;
  readonly loads: readonly AppliedLoad[];
  readonly phase: Slug | null;
};

/** 📈️ The eased weight of a linear fraction: linear `t`, smooth `3t² − 2t³`, in `t²`, out `1 − (1 − t)²`, step `0` until `1`. */
export function ease(kind: Ease, t: number): number {
  const u = t <= 0 ? 0 : t >= 1 ? 1 : t;
  switch (kind) {
    case "linear": return u;
    case "smooth": return u * u * (3 - 2 * u);
    case "in": return u * u;
    case "out": return 1 - (1 - u) * (1 - u);
    case "step": return u >= 1 ? 1 : 0;
  }
}

/** 🔁️ The clip time for a query time: looping clips wrap into `[0, duration)`, others clamp to `[0, duration]`. */
export function wrapTime(timeS: number, durationS: number, loop: boolean): number {
  if (!loop) return timeS <= 0 ? 0 : timeS >= durationS ? durationS : timeS;
  const wrapped = timeS - Math.floor(timeS / durationS) * durationS;
  return wrapped >= durationS || wrapped < 0 ? 0 : wrapped;
}

/** 🧮️ The whole cycle a query time of a looping clip lies in (0 for other clips and the first cycle). */
export function cycleOf(timeS: number, durationS: number, loop: boolean): number {
  return loop ? Math.floor(timeS / durationS) : 0;
}

/** 🎞️ The frame at a time: `floor(t · fps + 1e-9)`. */
export function frameAt(timeS: number, fps: number): number {
  return Math.floor(timeS * fps + 1e-9);
}

/** 🔎️ The pose with the id, or an error naming the asset and the poses it has. */
export function poseOf(asset: Asset, id: Slug): Pose {
  const pose = asset.poses.find((candidate) => candidate.id === id);
  if (pose === undefined) throw new Error(`asset "${asset.id}" has no pose "${id}"; poses: ${asset.poses.map((p) => p.id).join(", ")}`);
  return pose;
}

/** 🔎️ The clip with the id, or an error naming the asset and the clips it has. */
export function clipOf(asset: Asset, id: Slug): Clip {
  const clip = asset.clips.find((candidate) => candidate.id === id);
  if (clip === undefined) throw new Error(`asset "${asset.id}" has no clip "${id}"; clips: ${asset.clips.map((c) => c.id).join(", ")}`);
  return clip;
}

/** 🔑️ The keys sampling uses: for a looping clip a last key at or beyond the duration repeats the first key (at t = 0) and is dropped, so the wrap segment is the only way back. */
export function effectiveKeys<T extends { readonly t: number }>(keys: readonly T[], loop: boolean, durationS: number): readonly T[] {
  return loop && keys.length > 1 && keys[0]!.t === 0 && keys[keys.length - 1]!.t >= durationS ? keys.slice(0, -1) : keys;
}

/** 📍️ A key segment: the indices `a`, `b`, the linear fraction `w` and the times `ta`, `tb` of the two keys (shifted by one duration across the loop seam). */
export type Segment = { readonly a: number; readonly b: number; readonly w: number; readonly ta: number; readonly tb: number };

/** 📍️ Where a time falls among ascending key times. Looping lists continue from the last key to the first one cycle later. */
export function locate(times: readonly number[], timeS: number, loop: boolean, durationS: number): Segment {
  const n = times.length;
  const last = n - 1;
  if (n <= 1) return { a: 0, b: 0, w: 0, ta: times[0] ?? 0, tb: times[0] ?? 0 };
  if (timeS >= times[last]!) {
    const tb = times[0]! + durationS;
    return loop && tb > times[last]! ? { a: last, b: 0, w: (timeS - times[last]!) / (tb - times[last]!), ta: times[last]!, tb } : { a: last, b: last, w: 0, ta: times[last]!, tb: times[last]! };
  }
  if (timeS < times[0]!) {
    const ta = times[last]! - durationS;
    return loop && times[0]! > ta ? { a: last, b: 0, w: (timeS - ta) / (times[0]! - ta), ta, tb: times[0]! } : { a: 0, b: 0, w: 0, ta: times[0]!, tb: times[0]! };
  }
  let low = 0;
  let high = last;
  while (high - low > 1) {
    const mid = (low + high) >> 1;
    if (times[mid]! <= timeS) low = mid;
    else high = mid;
  }
  return { a: low, b: high, w: (timeS - times[low]!) / (times[high]! - times[low]!), ta: times[low]!, tb: times[high]! };
}

/** 🔚️ Whether a looping key list ends with a closing key at or beyond the duration: that key is the end of the last segment and may differ from the first by whole turns on unlimited revolute joints. */
export function hasClosingKey(times: readonly number[], loop: boolean, durationS: number): boolean {
  return loop && times.length > 1 && times[times.length - 1]! >= durationS;
}

/** 🧭️ A neighbouring key for tangents: its index, its time shifted consistently across the loop seam and the number of cycles (`wrap`, −1, 0 or 1) whose closing-key difference its value carries; absent at the ends of a non-looping clip. */
export type Neighbour = { readonly i: number; readonly t: number; readonly wrap: -1 | 0 | 1 };

/** 🧭️ The keys before and after a segment. With a closing key the neighbour across the seam is the key before the closing one (wrap −1) or the second key (wrap +1), so tangents are continuous through the seam including whole turns. */
export function neighbours(times: readonly number[], segment: Segment, loop: boolean, durationS: number): { readonly prev: Neighbour | null; readonly next: Neighbour | null } {
  const n = times.length;
  const closing = hasClosingKey(times, loop, durationS);
  const shiftA = segment.ta - times[segment.a]!;
  const shiftB = segment.tb - times[segment.b]!;
  const prev: Neighbour | null = segment.a > 0 ? { i: segment.a - 1, t: times[segment.a - 1]! + shiftA, wrap: 0 } : !loop ? null : closing ? { i: n - 2, t: times[n - 2]! - durationS + shiftA, wrap: -1 } : { i: n - 1, t: times[n - 1]! - durationS + shiftA, wrap: 0 };
  const next: Neighbour | null = segment.b < n - 1 ? { i: segment.b + 1, t: times[segment.b + 1]! + shiftB, wrap: 0 } : !loop ? null : closing ? { i: 1, t: times[1]! + durationS + shiftB, wrap: 1 } : { i: 0, t: times[0]! + durationS + shiftB, wrap: 0 };
  return { prev, next };
}

/** 🌊️ The cubic Hermite value at fraction `u` between `va` at `ta` and `vb` at `tb`; tangents are the finite differences across the neighbouring keys (Catmull-Rom for uneven spacing) and zero where a neighbour is absent, so the value and its slope are continuous across keys. */
export function hermite(va: number, vb: number, ta: number, tb: number, u: number, prev: { readonly t: number; readonly v: number } | null, next: { readonly t: number; readonly v: number } | null): number {
  const dt = tb - ta;
  const ma = prev === null ? 0 : (vb - prev.v) / (tb - prev.t);
  const mb = next === null ? 0 : (next.v - va) / (next.t - ta);
  const u2 = u * u;
  const u3 = u2 * u;
  return (2 * u3 - 3 * u2 + 1) * va + (u3 - 2 * u2 + u) * dt * ma + (-2 * u3 + 3 * u2) * vb + (u3 - u2) * dt * mb;
}

/** 📉️ A scalar track sampled at a clip time: `smooth` segments are cubic Hermite through the neighbouring keys, other eases blend by their weight; ends hold (wrap for looping clips); `undefined` for an empty track. */
export function sampleTrack(track: Clip["tracks"][number], timeS: number, loop: boolean, durationS: number): number | undefined {
  const keys = effectiveKeys(track.keys, loop, durationS);
  if (keys.length === 0) return undefined;
  const times = keys.map((key) => key.t);
  const segment = locate(times, timeS, loop, durationS);
  const from = keys[segment.a]!.v;
  if (segment.a === segment.b) return from;
  const to = keys[segment.b]!.v;
  if (track.ease !== "smooth") return from + (to - from) * ease(track.ease, segment.w);
  const { prev, next } = neighbours(times, segment, loop, durationS);
  return hermite(from, to, segment.ta, segment.tb, segment.w, prev === null ? null : { t: prev.t, v: keys[prev.i]!.v }, next === null ? null : { t: next.t, v: keys[next.i]!.v });
}

/** 🦶️ Whether a contact span is active at a clip time: half-open `[fromS, toS)`, closed at the end of the clip. */
export function spanActive(span: ContactSpan, timeS: number, durationS: number): boolean {
  return timeS >= span.fromS && (timeS < span.toS || (timeS >= durationS && span.toS >= durationS));
}

/** 🦶️ The anchors in contact at a clip time, in declaration order without repeats. */
export function contactsAt(clip: Clip, timeS: number): readonly Slug[] {
  const ids: Slug[] = [];
  for (const span of clip.contacts) if (spanActive(span, timeS, clip.durationS) && !ids.includes(span.anchor)) ids.push(span.anchor);
  return ids;
}

/** 🎬️ The id of the first phase containing the clip time, or `null`. */
export function phaseAt(clip: Clip, timeS: number): Slug | null {
  for (const phase of clip.phases) if (timeS >= phase.fromS && (timeS < phase.toS || (timeS >= clip.durationS && phase.toS >= clip.durationS))) return phase.id;
  return null;
}

const ZERO_OFFSET: FullOffset = { angleDeg: 0, dx: 0, dy: 0 };

function lerp(a: number, b: number, w: number): number {
  return a + (b - a) * w;
}

function fullOffset(offset: BoneOffset | undefined): FullOffset {
  return offset === undefined ? ZERO_OFFSET : { angleDeg: offset.angleDeg ?? 0, dx: offset.dx ?? 0, dy: offset.dy ?? 0 };
}

function keyOffsets(asset: Asset, pose: Pose, overrides: Readonly<Record<Slug, BoneOffset>> | undefined): Record<Slug, FullOffset> {
  const result: Record<Slug, FullOffset> = {};
  for (const bone of asset.rig.bones) {
    const base = pose.bones[bone.id];
    const over = overrides?.[bone.id];
    result[bone.id] = fullOffset(over === undefined ? base : { ...base, ...over });
  }
  return result;
}

function blendOffsets(a: Readonly<Record<Slug, FullOffset>>, b: Readonly<Record<Slug, FullOffset>>, w: number): Record<Slug, FullOffset> {
  const result: Record<Slug, FullOffset> = {};
  for (const id of Object.keys(a)) {
    const x = a[id]!;
    const y = b[id]!;
    result[id] = { angleDeg: lerp(x.angleDeg, y.angleDeg, w), dx: lerp(x.dx, y.dx, w), dy: lerp(x.dy, y.dy, w) };
  }
  return result;
}

function blendOptional(a: number | undefined, b: number | undefined, w: number): number | undefined {
  return a === undefined ? b : b === undefined ? a : lerp(a, b, w);
}

function blendRecord(a: Readonly<Record<Slug, number>> | undefined, b: Readonly<Record<Slug, number>> | undefined, w: number, keys: readonly Slug[]): Record<Slug, number> {
  const result: Record<Slug, number> = {};
  for (const key of keys) result[key] = lerp(a?.[key] ?? 0, b?.[key] ?? 0, w);
  return result;
}

const REGION_FIELDS = ["surfaceTempC", "coreTempC", "wettedness"] as const;

/** 🌡️ The thermal inputs of two bindings blended by weight `w` (absent operating levels and emitter strengths count as 0, absent exposure as 1, absent region temperatures stay absent); every declared region, emitter and operating key is present. */
export function blendThermal(asset: Asset, a: ThermalBinding, b: ThermalBinding, w: number): ThermalInputs {
  const operatingKeys = new Set<Slug>([...Object.keys(a.operating ?? {}), ...Object.keys(b.operating ?? {})]);
  for (const region of asset.thermal.regions) if (region.operatingKey !== undefined) operatingKeys.add(region.operatingKey);
  const emitterKeys = new Set<Slug>([...asset.thermal.emitters.map((emitter) => emitter.id), ...Object.keys(a.emitters ?? {}), ...Object.keys(b.emitters ?? {})]);
  const regionKeys = new Set<Slug>([...asset.thermal.regions.map((region) => region.id), ...Object.keys(a.regions ?? {}), ...Object.keys(b.regions ?? {})]);
  const regions: Record<Slug, { exposedFraction: number; surfaceTempC?: number; coreTempC?: number; wettedness?: number }> = {};
  for (const id of [...regionKeys].sort()) {
    const x = a.regions?.[id];
    const y = b.regions?.[id];
    const entry: { exposedFraction: number; surfaceTempC?: number; coreTempC?: number; wettedness?: number } = { exposedFraction: lerp(x?.exposedFraction ?? 1, y?.exposedFraction ?? 1, w) };
    for (const field of REGION_FIELDS) {
      const value = blendOptional(x?.[field], y?.[field], w);
      if (value !== undefined) entry[field] = value;
    }
    regions[id] = entry;
  }
  return {
    activityMet: blendOptional(a.activityMet, b.activityMet, w) ?? asset.thermal.baseMet ?? 1,
    operating: blendRecord(a.operating, b.operating, w, [...operatingKeys].sort()),
    regions,
    emitters: blendRecord(a.emitters, b.emitters, w, [...emitterKeys].sort()),
  };
}

function applyTracks(clip: Clip, timeS: number, thermal: ThermalInputs): { readonly thermal: ThermalInputs; readonly loadScales: Record<Slug, number> } {
  let activityMet = thermal.activityMet;
  const operating = { ...thermal.operating };
  const emitters = { ...thermal.emitters };
  const regions: Record<Slug, { exposedFraction: number; surfaceTempC?: number; coreTempC?: number; wettedness?: number }> = {};
  for (const [id, region] of Object.entries(thermal.regions)) regions[id] = { ...region };
  const loadScales: Record<Slug, number> = {};
  for (const track of clip.tracks) {
    const value = sampleTrack(track, timeS, clip.loop, clip.durationS);
    if (value === undefined) continue;
    const [kind, id, field] = track.target.split(":");
    if (track.target === "activityMet") activityMet = value;
    else if (kind === "emitter" && id !== undefined) emitters[id] = value;
    else if (kind === "operating" && id !== undefined) operating[id] = value;
    else if (kind === "load" && id !== undefined) loadScales[id] = value;
    else if (kind === "region" && id !== undefined && field !== undefined) {
      const region = (regions[id] ??= { exposedFraction: 1 });
      if (field === "exposedFraction") region.exposedFraction = value;
      else if (field === "surfaceTempC" || field === "coreTempC" || field === "wettedness") region[field] = value;
    }
  }
  return { thermal: { activityMet, operating, regions, emitters }, loadScales };
}

function scaleLoad(load: AppliedLoad, k: number): AppliedLoad {
  const distributed = load.distributed === undefined ? {} : { distributed: { ...load.distributed, wxNPerM: load.distributed.wxNPerM * k, wyNPerM: load.distributed.wyNPerM * k } };
  return { ...load, fxN: load.fxN * k, fyN: load.fyN * k, ...(load.momentNm === undefined ? {} : { momentNm: load.momentNm * k }), ...distributed };
}

function mixLoad(a: AppliedLoad, b: AppliedLoad, w: number): AppliedLoad {
  const distributed = a.distributed !== undefined && b.distributed !== undefined
    ? { distributed: { bone: a.distributed.bone, fromM: lerp(a.distributed.fromM, b.distributed.fromM, w), toM: lerp(a.distributed.toM, b.distributed.toM, w), wxNPerM: lerp(a.distributed.wxNPerM, b.distributed.wxNPerM, w), wyNPerM: lerp(a.distributed.wyNPerM, b.distributed.wyNPerM, w) } }
    : a.distributed === undefined ? {} : { distributed: a.distributed };
  const x = blendOptional(a.x, b.x, w);
  const y = blendOptional(a.y, b.y, w);
  const moment = blendOptional(a.momentNm, b.momentNm, w);
  return { ...a, fxN: lerp(a.fxN, b.fxN, w), fyN: lerp(a.fyN, b.fyN, w), ...(x === undefined ? {} : { x }), ...(y === undefined ? {} : { y }), ...(moment === undefined ? {} : { momentNm: moment }), ...distributed };
}

/** 🧲️ Loads of two poses blended by weight `w`: loads in both interpolate, loads in one fade in or out by their weight. */
export function blendLoads(a: readonly AppliedLoad[], b: readonly AppliedLoad[], w: number): readonly AppliedLoad[] {
  if (w <= 0) return a;
  if (w >= 1) return b;
  const result: AppliedLoad[] = [];
  const seen = new Set<Slug>();
  for (const load of a) {
    seen.add(load.id);
    const other = b.find((candidate) => candidate.id === load.id);
    result.push(other === undefined ? scaleLoad(load, 1 - w) : mixLoad(load, other, w));
  }
  for (const load of b) if (!seen.has(load.id)) result.push(scaleLoad(load, w));
  return result;
}

type Core = {
  readonly offsets: Record<Slug, FullOffset>;
  readonly authoredRoot: { readonly x: number; readonly y: number; readonly angleDeg: number };
  readonly weight: number;
  readonly from: Pose;
  readonly to: Pose;
};

function authoredRoot(pose: Pose): { x: number; y: number; angleDeg: number } {
  return { x: pose.root?.x ?? 0, y: pose.root?.y ?? 0, angleDeg: pose.root?.angleDeg ?? 0 };
}

function core(asset: Asset, clip: Clip, timeS: number, cycle = 0): Core {
  const keys = clip.keys;
  const times = keys.map((key) => key.t);
  const segment = locate(times, timeS, clip.loop, clip.durationS);
  const keyA = keys[segment.a]!;
  const keyB = keys[segment.b]!;
  const from = poseOf(asset, keyA.pose);
  const to = poseOf(asset, keyB.pose);
  const weight = segment.a === segment.b ? 0 : ease(keyA.ease, segment.w);
  const dataOf = (key: (typeof keys)[number]) => {
    const pose = poseOf(asset, key.pose);
    return { offsets: keyOffsets(asset, pose, key.bones), root: authoredRoot(pose) };
  };
  type Data = ReturnType<typeof dataOf>;
  const closing = hasClosingKey(times, clip.loop, clip.durationS);
  const first = closing ? dataOf(keys[0]!) : null;
  const last = closing ? dataOf(keys[keys.length - 1]!) : null;
  const turn = (pick: (d: Data) => number): number => (first === null || last === null ? 0 : pick(last) - pick(first));
  let offsets: Record<Slug, FullOffset>;
  let root: { x: number; y: number; angleDeg: number };
  if (segment.a === segment.b || keyA.ease !== "smooth") {
    offsets = blendOffsets(keyOffsets(asset, from, keyA.bones), keyOffsets(asset, to, keyB.bones), weight);
    const ra = authoredRoot(from);
    const rb = authoredRoot(to);
    root = { x: lerp(ra.x, rb.x, weight), y: lerp(ra.y, rb.y, weight), angleDeg: lerp(ra.angleDeg, rb.angleDeg, weight) };
  } else {
    const { prev, next } = neighbours(times, segment, clip.loop, clip.durationS);
    const da = dataOf(keyA);
    const db = dataOf(keyB);
    const dp = prev === null ? null : dataOf(keys[prev.i]!);
    const dn = next === null ? null : dataOf(keys[next.i]!);
    const mix = (pick: (d: Data) => number): number => {
      const shift = turn(pick);
      return hermite(pick(da), pick(db), segment.ta, segment.tb, segment.w, dp === null ? null : { t: prev!.t, v: pick(dp) + prev!.wrap * shift }, dn === null ? null : { t: next!.t, v: pick(dn) + next!.wrap * shift });
    };
    offsets = {};
    for (const bone of asset.rig.bones) {
      const id = bone.id;
      offsets[id] = { angleDeg: mix((d) => d.offsets[id]!.angleDeg), dx: mix((d) => d.offsets[id]!.dx), dy: mix((d) => d.offsets[id]!.dy) };
    }
    root = { x: mix((d) => d.root.x), y: mix((d) => d.root.y), angleDeg: mix((d) => d.root.angleDeg) };
  }
  if (cycle !== 0 && first !== null) {
    const turned: Record<Slug, FullOffset> = {};
    for (const bone of asset.rig.bones) {
      const id = bone.id;
      const o = offsets[id]!;
      turned[id] = { angleDeg: o.angleDeg + cycle * turn((d) => d.offsets[id]!.angleDeg), dx: o.dx + cycle * turn((d) => d.offsets[id]!.dx), dy: o.dy + cycle * turn((d) => d.offsets[id]!.dy) };
    }
    offsets = turned;
    root = { x: root.x + cycle * turn((d) => d.root.x), y: root.y + cycle * turn((d) => d.root.y), angleDeg: root.angleDeg + cycle * turn((d) => d.root.angleDeg) };
  }
  return { offsets, authoredRoot: root, weight, from, to };
}

function anchorOf(asset: Asset, id: Slug): Anchor {
  const anchor = asset.anchors.find((candidate) => candidate.id === id);
  if (anchor === undefined) throw new Error(`asset "${asset.id}" has no anchor "${id}"`);
  return anchor;
}

function anchorLocal(asset: Asset, world: Readonly<Record<Slug, Affine>>, id: Slug): { readonly x: number; readonly y: number } {
  const anchor = anchorOf(asset, id);
  const matrix = world[anchor.bone];
  if (matrix === undefined) throw new Error(`anchor "${id}" of asset "${asset.id}" sits on unknown bone "${anchor.bone}"`);
  return apply(matrix, anchor.x, anchor.y);
}

type PlantTable = {
  readonly times: Float64Array;
  readonly xs: Float64Array;
  readonly plants: readonly (number | undefined)[];
};

const TABLES = new WeakMap<Clip, WeakMap<Asset, PlantTable>>();

function relativeXAt(asset: Asset, clip: Clip, timeS: number): (anchor: Slug) => number {
  let world: Record<Slug, Affine> | undefined;
  return (anchor) => {
    if (world === undefined) {
      const c = core(asset, clip, timeS);
      world = solveRig(asset.rig, c.offsets, { x: 0, y: 0, angleDeg: c.authoredRoot.angleDeg });
    }
    return anchorLocal(asset, world, anchor).x;
  };
}

function rootAt(clip: Clip, timeS: number, plants: readonly (number | undefined)[], relative: (anchor: Slug) => number): number | null {
  const active: number[] = [];
  for (let index = 0; index < clip.contacts.length; index++) if (plants[index] !== undefined && spanActive(clip.contacts[index]!, timeS, clip.durationS)) active.push(index);
  if (active.length === 0) return null;
  active.sort((p, q) => clip.contacts[p]!.fromS - clip.contacts[q]!.fromS || p - q);
  const first = active[0]!;
  const a = clip.contacts[first]!;
  const xFirst = plants[first]! - relative(a.anchor);
  const next = active.find((index) => clip.contacts[index]!.fromS > a.fromS && clip.contacts[index]!.toS >= a.toS);
  if (next === undefined) return xFirst;
  const b = clip.contacts[next]!;
  const xNext = plants[next]! - relative(b.anchor);
  return xFirst + (xNext - xFirst) * ease("smooth", (timeS - b.fromS) / (a.toS - b.fromS));
}

function buildTable(asset: Asset, clip: Clip): PlantTable {
  const steps = Math.ceil(clip.durationS * PLANT_HZ);
  const instants = new Set<number>();
  for (let k = 0; k <= steps; k++) instants.add(Math.min(k / PLANT_HZ, clip.durationS));
  for (const span of clip.contacts) for (const t of [span.fromS, span.toS]) if (t >= 0 && t <= clip.durationS) instants.add(t);
  const times = Float64Array.from([...instants].sort((p, q) => p - q));
  const xs = new Float64Array(times.length);
  const plants: (number | undefined)[] = clip.contacts.map(() => undefined);
  let x = core(asset, clip, 0).authoredRoot.x;
  let velocity = 0;
  for (let i = 0; i < times.length; i++) {
    const t = times[i]!;
    const previous = i === 0 ? t : times[i - 1]!;
    const relative = relativeXAt(asset, clip, t);
    const estimate = rootAt(clip, t, plants, relative) ?? x + velocity * (t - previous);
    for (let index = 0; index < clip.contacts.length; index++) {
      const span = clip.contacts[index]!;
      if (plants[index] === undefined && spanActive(span, t, clip.durationS)) plants[index] = estimate + relative(span.anchor);
    }
    const solved = rootAt(clip, t, plants, relative);
    if (solved !== null && t > previous) velocity = (solved - x) / (t - previous);
    x = solved ?? estimate;
    xs[i] = x;
  }
  return { times, xs, plants };
}

function tableOf(asset: Asset, clip: Clip): PlantTable {
  let byAsset = TABLES.get(clip);
  if (byAsset === undefined) TABLES.set(clip, (byAsset = new WeakMap()));
  let table = byAsset.get(asset);
  if (table === undefined) byAsset.set(asset, (table = buildTable(asset, clip)));
  return table;
}

function plantedX(asset: Asset, clip: Clip, timeS: number): number {
  const table = tableOf(asset, clip);
  const driven = rootAt(clip, timeS, table.plants, relativeXAt(asset, clip, timeS));
  if (driven !== null) return driven;
  const { times, xs } = table;
  let low = 0;
  let high = times.length - 1;
  while (high - low > 1) {
    const mid = (low + high) >> 1;
    if (times[mid]! <= timeS) low = mid;
    else high = mid;
  }
  if (times[high]! === times[low]!) return xs[low]!;
  return lerp(xs[low]!, xs[high]!, (timeS - times[low]!) / (times[high]! - times[low]!));
}

function groundedY(asset: Asset, offsets: Record<Slug, FullOffset>, angleDeg: number, contacts: readonly Slug[], authoredY: number): number {
  if (contacts.length === 0) return authoredY;
  const world = solveRig(asset.rig, offsets, { x: 0, y: 0, angleDeg });
  let lowest = Number.POSITIVE_INFINITY;
  for (const id of contacts) lowest = Math.min(lowest, anchorLocal(asset, world, id).y);
  return -lowest;
}

/** 🎬️ The full sample of a clip at a time: blended offsets and thermal state, tracks, contacts, phase, loads, load scales, and the root with planted motion and automatic ground applied. */
export function sampleClip(asset: Asset, clip: Clip, timeS: number): Sample {
  const t = wrapTime(timeS, clip.durationS, clip.loop);
  const c = core(asset, clip, t, cycleOf(timeS, clip.durationS, clip.loop));
  const blended = blendThermal(asset, c.from.thermal, c.to.thermal, c.weight);
  const { thermal, loadScales } = applyTracks(clip, t, blended);
  const contacts = contactsAt(clip, t);
  const auto = c.from.ground === "auto" && c.to.ground === "auto";
  const advance = clip.rootMotion === "plant" && clip.loop ? cycleOf(timeS, clip.durationS, clip.loop) * (clip.advanceM ?? 0) : 0;
  const x = clip.rootMotion === "plant" ? plantedX(asset, clip, t) + advance : c.authoredRoot.x;
  const y = auto ? groundedY(asset, c.offsets, c.authoredRoot.angleDeg, contacts, c.authoredRoot.y) : c.authoredRoot.y;
  return { timeS: t, frame: frameAt(t, clip.fps), offsets: c.offsets, root: { x, y, angleDeg: c.authoredRoot.angleDeg }, contacts, thermal, loadScales, loads: blendLoads(c.from.loads, c.to.loads, c.weight), phase: phaseAt(clip, t) };
}

/** 🧍️ The sample of a static pose: its offsets, authored root (ground `auto` puts the lowest listed contact on y = 0), thermal binding, loads and contacts. */
export function samplePose(asset: Asset, pose: Pose): Sample {
  const offsets = keyOffsets(asset, pose, undefined);
  const root = authoredRoot(pose);
  const y = pose.ground === "auto" ? groundedY(asset, offsets, root.angleDeg, pose.contacts, root.y) : root.y;
  return { timeS: 0, frame: 0, offsets, root: { x: root.x, y, angleDeg: root.angleDeg }, contacts: pose.contacts, thermal: blendThermal(asset, pose.thermal, pose.thermal, 0), loadScales: {}, loads: pose.loads, phase: null };
}

/** 🔀️ A synthetic non-looping two-key clip from pose `from` to pose `to` over `durationS` with smooth easing; contacts held by both poses last the whole time, those of one pose half of it. */
export function transition(asset: Asset, from: Slug, to: Slug, durationS: number): Clip {
  const a = poseOf(asset, from);
  const b = poseOf(asset, to);
  const half = durationS / 2;
  const contacts: ContactSpan[] = [];
  for (const anchor of a.contacts) contacts.push({ anchor, fromS: 0, toS: b.contacts.includes(anchor) ? durationS : half });
  for (const anchor of b.contacts) if (!a.contacts.includes(anchor)) contacts.push({ anchor, fromS: half, toS: durationS });
  return {
    id: `transition-${from}-${to}`,
    title: { en: `Transition ${from} to ${to}`, de: `Übergang ${from} nach ${to}` },
    description: { en: `Synthetic blend from ${from} to ${to}`, de: `Synthetische Überblendung von ${from} nach ${to}` },
    durationS,
    fps: asset.clips[0]?.fps ?? 30,
    loop: false,
    rootMotion: "none",
    keys: [{ t: 0, pose: from, ease: "smooth" }, { t: durationS, pose: to, ease: "smooth" }],
    contacts,
    phases: [],
    events: [],
    tracks: [],
  };
}
