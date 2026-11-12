/** 🎭️ Cast exporter: renders the named, figure-only casts of a cast manifest into one folder per cast (`<out>/<cast-id>/🔣️.json` and `NNNN.svg`).
 * `repo tools exec bun -- 🟦️.ts --manifest file --ids a,b --out dir --lod full|preview|poster --output-fps 30 [--catalog dir]`. The level of detail (frames sampled at canonical times, shape roles, transport density; see 🧊️lod) is explicit and part of every index and provenance; the whole run sees one level. Casts are language-independent: they print no text, so no language is asked for or part of any cache key.
 *
 * A cast of the `axonometric` viewpoint draws the 3D volume through the default isometric camera. Each frame is `evaluate` → `depict` (every annotation, legend and title off, transparent paper) → `writeSvg`. All frames of a cast share one
 * viewBox and one world-origin alignment (the union of the frame extents, frames translated into it), so playing them back never jumps; `mirror`
 * flips the figure about the vertical axis of the world origin. Thermal clips and poses under an environment timeline solve through `solveThermalAt`,
 * so the temperatures at a frame equal those reached by playing; one `thermalSession` per cast lets the frames share the input history and the integrations from the same start, which changes no value. `thermalSpanK` colours the thermal view on a symmetric ±spanK range around the air temperature of the cast environment instead of the absolute shared range of −20…60 °C, so moderate sources (a 35 °C laptop in a 21 °C room) stay readable; the course casts use 25 K, one value per scene. Casts render one after the other; each frame is kept as a compact UTF-8 buffer until the shared viewBox is known, then placed, hashed and written one at a time (sixteen writes in flight) and released, so memory
 * stays bounded by one cast and by the frames of the cast, never by their text copies; identical frames (equal SHA-256) are written once and repeated by name in the index; schedules longer than 600 frames are thinned to 600 samples
 * with a fractional `fps`. A cast is written to a temporary folder and renamed into place. Progress goes to
 * stderr (`[pid]`, `[cast i/n] id`, `[done/total]` every 50 frames). `--parent <pid>` cancels the same way when that process vanishes (a hard-killed caller). SIGINT, SIGTERM, SIGHUP (POSIX) and SIGBREAK (Windows) cancel: every frame yields to the
 * event loop so a signal is seen between frames, the temporary folder is removed, an interrupted cast is never renamed into place, and the exit code is 130.
 * @see ./🧬️schema/🔣️.schema.json — the manifest schema
 * @see ../🎬️cli/🟦️.ts — the general export command */
import { createHash } from "node:crypto";
import { readFileSync, realpathSync } from "node:fs";
import { mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { JobControl } from "../../🔗️contracts/🟦️.ts";
import { sectionOf } from "../../🎨️depiction/✂️section/🟦️.ts";
import { type Asset, type AuthoredRepresentation, type Environment, type EnvironmentPreset, SCALE_DENOMINATORS } from "../../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, type LoadedAsset, loadCatalogFromFiles, loadEnvironments } from "../../🗂️catalog/📥️files/🟦️.ts";
import { evaluate, poseOf } from "../../🧮️evaluation/🟦️.ts";
import { depict, dimensionExtent, figureOf, loadAxonometric, textlessDepictionOptions } from "../../🎨️depiction/🟦️.ts";
import { prepare, sourceOf } from "../../🎨️depiction/🧩️shapes/🟦️.ts";
import { solveThermalAt, thermalSession } from "../../🌡️thermal/🟦️.ts";
import { solveTransport } from "../../💨️transport/🟦️.ts";
import { solveStructure } from "../../🏗️structure/🟦️.ts";
import { type StructuralSummary, structureIndex, structuralSummary } from "../../🏗️structure/🧾️summary/🟦️.ts";
import { type EnvironmentInput, isTimeline } from "../../🌡️thermal/🟦️.ts";
import { type FrameSchedule, frameSchedule } from "../🎞️frames/🟦️.ts";
import { writeSvg } from "../🖼️svg/🟦️.ts";
import { canonicalJson, castProvenance, presetRevision, revisionOfText } from "../../🏷️revision/🟦️.ts";
import { type Cast, type CastIndex, type CastLogical, type CastManifest, type Revision, parseCastManifest } from "./🧬️schema/🟦️.ts";
import { type ResolvedLod, lodSchedule, resolveLod } from "./🧊️lod/🟦️.ts";

const MARGIN_MM = 1;
const MAX_FRAMES = 600;

const frameFile = (index: number): string => `${String(index).padStart(4, "0")}.svg`;
const round = (value: number, digits = 6): number => Math.round(value * 10 ** digits) / 10 ** digits;

export { canonicalJson, presetRevision, revisionOfText };

/** ⚙️ What a cast export needs besides the manifest: the catalog folder, the level of detail (absent: `full` at 30 fps; the command line names it explicitly), and progress and cancellation. */
export type CastOptions = JobControl & { readonly catalog?: string; readonly lod?: ResolvedLod; readonly started?: (id: string, index: number, count: number) => void };

const FULL = resolveLod("full", 30);

const yieldToLoop = (): Promise<void> => new Promise((done) => setImmediate(done));

type Frames = { readonly schedule: FrameSchedule; readonly queries: readonly ({ readonly kind: "pose"; readonly pose: string; readonly timeS: number } | { readonly kind: "clip"; readonly clip: string; readonly timeS: number })[] };

/** 🪶 Thins a schedule longer than {@link MAX_FRAMES} to that many evenly spaced samples (a possibly fractional `fps`), so slow transients such as a 25 minute vacuum run stay cheap in time, disk and memory; the playback still maps scene time to frames through `fps`. A loop whose `durationS · fps` is not whole gets `fps = count / durationS`, so `count` frames span the period exactly and the `playbackFrame` wrap lands on frame 0 without a seam; whole periods stay unchanged. */
export function thinned(schedule: FrameSchedule): FrameSchedule {
  if (schedule.count <= MAX_FRAMES) {
    const fps = schedule.loop ? schedule.count / schedule.durationS : schedule.fps;
    return fps === schedule.fps ? schedule : { ...schedule, fps, times: Array.from({ length: schedule.count }, (_, frame) => frame / fps) };
  }
  const fps = (schedule.loop ? MAX_FRAMES : MAX_FRAMES - 1) / schedule.durationS;
  return { ...schedule, fps, count: MAX_FRAMES, times: Array.from({ length: MAX_FRAMES }, (_, frame) => frame / fps) };
}

function queriesOf(cast: Cast, asset: Asset, schedule: FrameSchedule): Frames["queries"] {
  if (cast.clip !== undefined) {
    const warm = cast.transportAgeS ?? 0;
    return schedule.times.map((timeS) => ({ kind: "clip", clip: cast.clip!, timeS: warm + timeS }));
  }
  const pose = poseOf(asset, cast.pose!).id;
  return schedule.times.map((timeS) => ({ kind: "pose", pose, timeS }));
}

/** 🕰️ The canonical clock of a cast, identical for every level of detail: the schedule of its clip (thinned to 600 samples) or of its environment timeline, and nothing for a plain pose. */
export function canonicalSchedule(cast: Cast, asset: Asset): FrameSchedule {
  if (cast.clip !== undefined) {
    const clip = asset.clips.find((candidate) => candidate.id === cast.clip);
    if (!clip) throw new Error(`asset ${asset.id} has no clip ${cast.clip}; clips: ${asset.clips.map((candidate) => candidate.id).join(", ")}`);
    return thinned(frameSchedule(clip, cast.fps!));
  }
  poseOf(asset, cast.pose!);
  const end = cast.timeline ? Math.max(...cast.timeline.map((key) => key.t)) : 0;
  return end > 0 ? thinned(frameSchedule({ durationS: end, loop: false }, cast.fps!)) : { fps: 1, durationS: 0, loop: false, count: 1, times: [0] };
}

function framesOf(cast: Cast, asset: Asset, lod: ResolvedLod): Frames & { readonly canonical: FrameSchedule } {
  const canonical = canonicalSchedule(cast, asset);
  const schedule = lodSchedule(canonical, lod);
  return { canonical, schedule, queries: queriesOf(cast, asset, schedule) };
}

/** 🗺️ The authored representation a cast draws: exactly the authored preset scale, with geometry for the viewpoint (the axonometric viewpoint needs the asset's volume instead of authored shapes) and, for the thermal view, thermal regions. Unavailable, derived and empty combinations throw. */
function authoredRepresentation(id: string, asset: Asset, cast: Cast): AuthoredRepresentation {
  const authored = SCALE_DENOMINATORS.filter((denominator) => asset.representations[`${denominator}`]?.status === "authored");
  const representation = asset.representations[`${cast.scale}`];
  if (representation?.status !== "authored") throw new Error(`cast ${id}: asset ${asset.id} has no authored representation at 1:${cast.scale}; authored scales: ${authored.join(", ")}`);
  if (cast.viewpoint !== "axonometric" && representation.geometry[cast.viewpoint].length === 0) throw new Error(`cast ${id}: asset ${asset.id} has no ${cast.viewpoint} geometry at 1:${cast.scale}`);
  if (cast.view === "thermal" && asset.thermal.regions.length === 0) throw new Error(`cast ${id}: asset ${asset.id} has no thermal regions for the thermal view`);
  return representation;
}

function environmentOf(cast: Cast, presets: ReadonlyMap<string, EnvironmentPreset>): EnvironmentInput | undefined {
  const preset = (slug: string): Environment => {
    const found = presets.get(slug);
    if (!found) throw new Error(`unknown environment preset ${slug}; presets: ${[...presets.keys()].join(", ")}`);
    return found.environment;
  };
  if (cast.timeline) return { keys: [...cast.timeline].sort((a, b) => a.t - b.t).map((key) => ({ t: key.t, environment: preset(key.environment), ease: key.ease })) };
  return cast.environment === undefined ? undefined : preset(cast.environment);
}

function placed(svg: string, ox: number, oy: number, left: number, top: number, width: number, height: number, mirror: boolean): string {
  const transform = mirror ? `matrix(-1 0 0 1 ${round(left + ox, 4)} ${round(top - oy, 4)})` : `translate(${round(left - ox, 4)} ${round(top - oy, 4)})`;
  const start = svg.indexOf("</metadata>\n");
  const end = svg.lastIndexOf("\n</svg>");
  const framed = (text: string): string =>
    text
      .replace(/width="[^"]*" height="[^"]*" viewBox="[^"]*"/u, `width="${round(width, 4)}mm" height="${round(height, 4)}mm" viewBox="0 0 ${round(width, 4)} ${round(height, 4)}"`)
      .replace(/origin-x-mm="[^"]*" origin-y-mm="[^"]*"/u, `origin-x-mm="${round(left, 4)}" origin-y-mm="${round(top, 4)}"`);
  const bodyStart = start + "</metadata>\n".length;
  if (start < 0 || end < bodyStart) return framed(svg);
  return `${framed(svg.slice(0, start))}</metadata>\n<g id="staffage-cast" transform="${transform}">${svg.slice(bodyStart, end)}</g>${svg.slice(end)}`;
}

type Drawn = { readonly originX: number; readonly originY: number; readonly width: number; readonly height: number; svg: Buffer | null };

const WRITES_IN_FLIGHT = 16;

/** 💾 Writes files with a bounded number in flight, so the latency of many small files on a scanned disk overlaps instead of adding up; `flush` waits for all and rethrows the first failure. */
function writer(): { readonly write: (path: string, text: string) => Promise<void>; readonly flush: () => Promise<void> } {
  const pending = new Set<Promise<void>>();
  let failure: unknown;
  return {
    async write(path, text) {
      while (pending.size >= WRITES_IN_FLIGHT) await Promise.race(pending);
      if (failure !== undefined) throw failure;
      const job: Promise<void> = patiently(() => writeFile(path, text)).catch((error: unknown) => void (failure ??= error)).finally(() => void pending.delete(job));
      pending.add(job);
    },
    async flush() {
      await Promise.all(pending);
      if (failure !== undefined) throw failure;
    },
  };
}

/** 🧹 Runs a filesystem step again a few times when Windows reports a transient lock (`EPERM`, `EBUSY`, `ENOTEMPTY`, as virus scanners and indexers hold a just written folder). */
async function patiently<T>(step: () => Promise<T>): Promise<T> {
  for (let attempt = 1; ; attempt++) {
    try {
      return await step();
    } catch (error) {
      const code = (error as NodeJS.ErrnoException).code;
      if (attempt >= 8 || (code !== "EPERM" && code !== "EBUSY" && code !== "ENOTEMPTY")) throw error;
      await new Promise((done) => setTimeout(done, 50 * attempt));
    }
  }
}

const ROUND_PART = 5;

/** 📏️ Accumulates the level-independent physical truth of a cast over states of its canonical clock: the figure height, the paper extent of the figure and, for the first state, the polygons of its filled shapes by id. `evaluate` and `prepare` only: no thermal solve, no drawing. An axonometric cast measures that truth on the side elevation. */
function logicalOf(asset: Asset, cast: Cast): { readonly add: (state: ReturnType<typeof evaluate>) => void; readonly done: (maxPaths: number) => CastLogical } {
  const k = 1000 / cast.scale;
  const mirror = cast.mirror === true;
  const viewpoint = cast.viewpoint === "axonometric" ? "elevation" : cast.viewpoint;
  let heightM = 0;
  let box = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  let parts: Record<string, readonly (readonly [number, number])[]> | undefined;
  return {
    add(state) {
      heightM = Math.max(heightM, state.bounds.maxY - state.bounds.minY);
      const everything = viewpoint === "elevation" && parts !== undefined ? [] : prepare(state, viewpoint, sourceOf(state), k);
      const extent = dimensionExtent(viewpoint, state.bounds, figureOf(everything), k);
      if (extent !== null) box = { minX: Math.min(box.minX, extent.minX), minY: Math.min(box.minY, extent.minY), maxX: Math.max(box.maxX, extent.maxX), maxY: Math.max(box.maxY, extent.maxY) };
      if (parts === undefined)
        parts = Object.fromEntries(
          everything.flatMap((p) => (p.closed && p.shape.filled && p.rings[0] !== undefined ? [[p.shape.id, p.rings[0].map((q): [number, number] => [round((mirror ? -q.x : q.x) / k, ROUND_PART), round(-q.y / k, ROUND_PART)])]] : [])),
        );
    },
    done(maxPaths) {
      const figure = mirror ? { minX: -box.maxX, minY: box.minY, maxX: -box.minX, maxY: box.maxY } : box;
      return { heightM: round(heightM), figureBoundsM: { minX: round(figure.minX), minY: round(figure.minY), maxX: round(figure.maxX), maxY: round(figure.maxY) }, partsM: parts ?? {}, maxPaths };
    },
  };
}

/** 🎬 Draws the frames of one cast one at a time, each kept as a compact UTF-8 buffer (one byte per character instead of a two-byte string), then places them into the shared viewBox one at a time and writes each frame as soon as it is placed,
 * only when its placed text is new (compared by SHA-256); every buffer is released after its frame is placed. Returns the index. */
async function render(id: string, cast: Cast, asset: Asset, revision: Revision, presets: ReadonlyMap<string, EnvironmentPreset>, lod: ResolvedLod, control: JobControl, report: () => void, directory: string): Promise<CastIndex> {
  const { canonical, schedule, queries } = framesOf(cast, asset, lod);
  const environment = environmentOf(cast, presets);
  const selector = { kind: "scale", scale: { denominator: cast.scale } } as const;
  const base = { ...textlessDepictionOptions(cast.view, cast.scale, cast.viewpoint, cast.theme), patterns: lod.level.patterns, roles: lod.level.roles, transportDensity: lod.level.transportDensity, ...(cast.thermalSpanK === undefined ? {} : { range: { mode: "span", spanK: cast.thermalSpanK } as const }), transparent: true, marginMm: MARGIN_MM, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false } as const };
  const options = { ...base, transport: cast.transport ?? [], ...(cast.transportStrokeMm === undefined ? {} : { transportStrokeMm: cast.transportStrokeMm }), ...(cast.transportTrailS === undefined ? {} : { transportTrailS: cast.transportTrailS }),...(environment === undefined ? {} : isTimeline(environment) ? { environmentTimeline: environment } : { environment }) };
  const drawn: Drawn[] = [];
  const session = thermalSession();
  const summaries: StructuralSummary[] = [];
  let mmPerMetre = 0;
  const logical = logicalOf(asset, cast);
  const shared = schedule === canonical;
  if (!shared)
    for (const query of queriesOf(cast, asset, canonical)) {
      await yieldToLoop();
      control.signal?.throwIfAborted();
      logical.add(evaluate(asset, query, selector));
    }
  for (const query of queries) {
    await yieldToLoop();
    control.signal?.throwIfAborted();
    const state = evaluate(asset, query, selector);
    if (shared) logical.add(state);
    const thermal = cast.view === "thermal" ? solveThermalAt(asset, query, environment, { ...(control.signal ? { signal: control.signal } : {}), session }) : undefined;
    const transport = options.transport.length > 0 && cast.transportAgeS !== undefined ? solveTransport(state.asset, state.query, options.environmentTimeline ?? options.environment!, { maxAgeS: cast.transportAgeS, ...(schedule.loop ? { periodS: schedule.durationS } : {}) }) : undefined;
    const structure = cast.view === "forces" || cast.view === "moments" ? solveStructure(state) : undefined;
    if (structure) summaries.push(structuralSummary(structure, cast.mirror === true));
    const scene = depict(state, options, { ...(thermal ? { thermal } : {}), ...(transport ? { transport } : {}), ...(structure ? { structure } : {}) }, control);
    mmPerMetre ||= scene.metadata.mmPerMetre;
    drawn.push({ originX: scene.metadata.origin.xMm, originY: scene.metadata.origin.yMm, width: scene.widthMm, height: scene.heightMm, svg: Buffer.from(writeSvg(scene, { compact: true }), "utf8") });
    report();
  }
  const section = sectionOf(authoredRepresentation(id, asset, cast), cast.viewpoint);
  const mirror = cast.mirror === true;
  const near = Math.max(...drawn.map((frame) => frame.originX));
  const far = Math.max(...drawn.map((frame) => frame.width - frame.originX));
  const top = Math.max(...drawn.map((frame) => frame.originY));
  const bottom = Math.max(...drawn.map((frame) => frame.height - frame.originY));
  const left = mirror ? far : near;
  const width = near + far;
  const height = top + bottom;
  const k = mmPerMetre;
  const named = new Map<string, string>();
  const frames: string[] = [];
  let maxPaths = 0;
  const files = writer();
  for (const [position, frame] of drawn.entries()) {
    control.signal?.throwIfAborted();
    const svg = placed(frame.svg!.toString("utf8"), frame.originX, frame.originY, left, top, width, height, mirror);
    frame.svg = null;
    maxPaths = Math.max(maxPaths, svg.split("<path").length - 1);
    const digest = createHash("sha256").update(svg).digest("hex");
    const known = named.get(digest);
    if (known !== undefined) {
      frames.push(known);
      continue;
    }
    const name = frameFile(position);
    named.set(digest, name);
    frames.push(name);
    await files.write(join(directory, name), svg);
  }
  await files.flush();
  return {
    version: 1,
    cast: id,
    asset: asset.id,
    view: cast.view,
    viewpoint: cast.viewpoint,
    ...(section === undefined ? {} : { section }),
    frames,
    fps: schedule.fps,
    durationS: schedule.durationS,
    loop: schedule.loop,
    mmPerMetre: k,
    originMm: [round(left, 4), round(top, 4)],
    boundsM: { minX: round(-left / k), minY: round(-bottom / k), maxX: round((width - left) / k), maxY: round(top / k) },
    lod: { id: lod.id, revision: lod.revision, canonicalFrames: canonical.count, canonicalFps: canonical.fps },
    logical: logical.done(maxPaths),
    ...(summaries.length > 0 ? { structure: structureIndex(summaries) } : {}),
    provenance: castProvenance(cast, asset, revision, presets, { fps: schedule.fps, durationS: schedule.durationS, loop: schedule.loop, frames: frames.length }, lod),
  };
}

async function exportOne(directory: string, out: string, plan: { readonly id: string; readonly cast: Cast; readonly asset: Asset; readonly revision: Revision }, presets: ReadonlyMap<string, EnvironmentPreset>, lod: ResolvedLod, control: JobControl, report: () => void): Promise<CastIndex> {
  const temporary = join(out, `.${plan.id}.tmp-${process.pid}`);
  try {
    await patiently(() => rm(temporary, { recursive: true, force: true }));
    await mkdir(temporary, { recursive: true });
    const index = await render(plan.id, plan.cast, plan.asset, plan.revision, presets, lod, control, report, temporary);
    await writeFile(join(temporary, "🔣️.json"), `${JSON.stringify(index, null, 2)}\n`);
    control.signal?.throwIfAborted();
    await patiently(() => rm(directory, { recursive: true, force: true }));
    await patiently(() => rename(temporary, directory));
    return index;
  } finally {
    await patiently(() => rm(temporary, { recursive: true, force: true }));
  }
}

/** 📦️ Renders the casts `ids` of `manifest` into `<out>/<id>/`; returns their indexes in `ids` order. Throws before writing anything when an id, asset, pose, clip or preset is unknown. */
export async function exportCast(manifest: unknown, ids: readonly string[], out: string, options: CastOptions = {}): Promise<readonly CastIndex[]> {
  const checked: CastManifest = parseCastManifest(manifest);
  const unknown = ids.filter((id) => !(id in checked.casts));
  if (unknown.length > 0) throw new Error(`unknown cast ids: ${unknown.join(", ")}`);
  if (ids.some((id) => checked.casts[id]!.viewpoint === "axonometric")) await loadAxonometric();
  const root = resolve(options.catalog ?? DEFAULT_CATALOG_ROOT);
  const lod = options.lod ?? FULL;
  const catalog = await loadCatalogFromFiles(root, options.signal, new Set(ids.map((id) => checked.casts[id]!.asset)));
  const presets = new Map(((await loadEnvironments(root, options.signal))?.presets ?? []).map((preset) => [preset.id, preset]));
  const plans = ids.map((id) => {
    const cast = checked.casts[id]!;
    const found = catalog.loaded.filter((candidate) => candidate.asset.id === cast.asset);
    if (found.length === 0) throw new Error(`cast ${id}: asset ${cast.asset} is not in the catalog ${root}`);
    if (found.length > 1) throw new Error(`cast ${id}: asset ${cast.asset} is in the catalog ${root} ${found.length} times: ${found.map((candidate) => candidate.path).join(", ")}`);
    const { asset, path, folder } = found[0] as LoadedAsset;
    if (folder !== asset.id) throw new Error(`cast ${id}: asset ${asset.id} lives in the folder ${folder}, the folder must carry the asset id`);
    environmentOf(cast, presets);
    authoredRepresentation(id, asset, cast);
    return { id, cast, asset, revision: revisionOfText(readFileSync(path, "utf8")), count: framesOf(cast, asset, lod).schedule.count };
  });
  const total = plans.reduce((sum, plan) => sum + plan.count, 0);
  let done = 0;
  const control: JobControl = options.signal ? { signal: options.signal } : {};
  const report = (): void => options.progress?.(++done, total, { en: "Rendering cast frames", de: "Besetzungsbilder werden gerendert" });
  await mkdir(resolve(out),{ recursive: true });
  const indexes: CastIndex[] = [];
  for (const [position, plan] of plans.entries()) {
    options.signal?.throwIfAborted();
    options.started?.(plan.id, position + 1, plans.length);
    indexes.push(await exportOne(join(resolve(out), plan.id), resolve(out), plan, presets, lod, control, report));
  }
  return indexes;
}

const FLAGS = ["manifest", "ids", "out", "catalog", "parent", "lod", "output-fps"];
const USAGE = "usage: --manifest <file> [--ids a,b,c] --out <dir> --lod full|preview|poster --output-fps <fps> [--catalog <dir>] [--parent <pid>]";

/** 🔧️ Parses `--key value` arguments; every refusal names the offending flag. */
export function parseArguments(argv: readonly string[]): { readonly manifest: string; readonly ids: readonly string[] | undefined; readonly out: string; readonly catalog: string; readonly parent: number | undefined; readonly lod: ResolvedLod } {
  const values = new Map<string, string>();
  for (let index = 0; index < argv.length; index += 2) {
    const key = argv[index];
    const value = argv[index + 1];
    if (!key?.startsWith("--") || value === undefined) throw new Error(`bad argument ${key ?? ""}\n${USAGE}`);
    values.set(key.slice(2), value);
  }
  for (const key of values.keys()) if (!FLAGS.includes(key)) throw new Error(`unknown --${key}\n${USAGE}`);
  const need = (key: string): string => values.get(key) ?? (() => { throw new Error(`missing --${key}\n${USAGE}`); })();
  const ids = values.get("ids");
  return { manifest: resolve(need("manifest")), ids: ids === undefined ? undefined : ids.split(",").filter((id) => id.length > 0), out: resolve(need("out")), catalog: resolve(values.get("catalog") ?? DEFAULT_CATALOG_ROOT), parent: parentOf(values.get("parent")), lod: resolveLod(need("lod"), Number(need("output-fps"))) };
}

function parentOf(value: string | undefined): number | undefined {
  if (value === undefined) return undefined;
  const pid = Number(value);
  if (!Number.isInteger(pid) || pid <= 0) throw new Error(`--parent must be a process id, got ${value}\n${USAGE}`);
  return pid;
}

/** 🔭 Tells whether a process exists; `EPERM` means it exists but belongs to someone else. */
export function processAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/** 🐕 Calls `gone` once when the watched process has vanished, so a hard-killed caller never leaves this exporter running; returns the function that stops watching. */
export function watchParent(pid: number, gone: () => void, interval = 500, alive: (pid: number) => boolean = processAlive): () => void {
  const timer = setInterval(() => {
    if (alive(pid)) return;
    clearInterval(timer);
    gone();
  }, interval);
  timer.unref();
  return () => clearInterval(timer);
}

async function main(): Promise<void> {
  const controller = new AbortController();
  for (const signal of process.platform === "win32" ? (["SIGINT", "SIGTERM", "SIGBREAK"] as const) : (["SIGINT", "SIGTERM", "SIGHUP"] as const)) process.on(signal, () => controller.abort());
  process.stderr.write(`[pid ${process.pid}] ppid ${process.ppid}\n`);
  let stopWatching = (): void => {};
  try {
    const args = parseArguments(process.argv.slice(2));
    if (args.parent !== undefined) stopWatching = watchParent(args.parent, () => { controller.abort(); setTimeout(() => process.exit(130), 5000).unref(); });
    const manifest = JSON.parse(await readFile(args.manifest, "utf8")) as CastManifest;
    const ids = args.ids ?? Object.keys(manifest.casts ?? {});
    const indexes = await exportCast(manifest, ids, args.out, { catalog: args.catalog, lod: args.lod, signal: controller.signal, started: (id, index, count) => process.stderr.write(`[cast ${index}/${count}] ${id}\n`), progress: (done, total, label) => { if (done % 50 === 0 || done === total) process.stderr.write(`[${done}/${total}] ${label.en} / ${label.de}\n`); } });
    for (const index of indexes) process.stdout.write(`${join(args.out, index.cast)}\n`);
    process.stderr.write(`[peak] ${Math.round(process.resourceUsage().maxRSS / 1024)} MB resident\n`);
  } catch (error) {
    stopWatching();
    if (controller.signal.aborted) {
      process.stderr.write("[cancelled]\n");
      process.exitCode = 130;
    } else {
      process.stderr.write(`${(error as Error).message}\n`);
      process.exitCode = 1;
    }
  }
}

/** 🚪 Tells whether this module is the program being run; both paths are resolved through symbolic links and case-folded where the file system is case-insensitive, so a symlinked checkout (`/workspaces`, `/var` → `/private/var`) or a lower-case drive letter never turns the command into a silent no-op. */
export function isEntry(moduleUrl: string, entry: string | undefined): boolean {
  if (entry === undefined) return false;
  const real = (path: string): string => {
    try {
      return realpathSync.native(path);
    } catch {
      return resolve(path);
    }
  };
  return real(fileURLToPath(moduleUrl)) === real(resolve(entry));
}

if (isEntry(import.meta.url, process.argv[1])) await main();
