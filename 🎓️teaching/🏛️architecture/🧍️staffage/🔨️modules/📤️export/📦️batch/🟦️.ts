/** 📦️ The complete 3D model set of the catalog as files: per asset one GLB per named pose (all seven views inside), one FBX per clip (one view, default `light-solid`), the neutral model JSON of every pose and clip and the ZIP of all of them, plus one manifest of the whole set.
 *
 * `repo tools exec bun -- 🔨️modules/📤️export/📦️batch/🟦️.ts --lang en|de [--out dir] [--scale 50] [--asset a,b,c] [--view light-solid] [--environment preset] [--span-k kelvin] [--catalog dir] [--parent pid] [--force]`
 *
 * Output layout under `--out` (default `📦️packages/🟦️typescript/📤️dist/🧊️models` of this package, which is not checked in):
 * `manifest.json` and `<asset>/<asset>-<pose>-all-model-1to<scale>.glb`, `<asset>/<asset>-<clip>-<view>-model-1to<scale>.fbx`, `<asset>/<asset>-<pose>-all-model-1to<scale>.pose.json`, `<asset>/<asset>-<clip>-all-model-1to<scale>.clip.json`, `<asset>/<asset>-all-model-1to<scale>.zip` (members `glb/`, `fbx/`, `model/`, byte-identical to {@link writeModelBundle}).
 * Every absolute path stays under 256 characters (the Windows limit), the names are the D14 names of {@link modelStem}.
 *
 * Properties: assets are streamed one at a time (one parsed asset and its files in memory), every file is written through a temporary name and renamed, the manifest is rewritten after every asset so an interrupted run leaves a consistent set, the bytes are deterministic (no time stamps anywhere, the manifest carries none), a run resumes (a file whose bytes still hash to the manifest and whose asset source and options are unchanged is kept; `--force` recomputes), and SIGINT, SIGTERM, SIGHUP (POSIX) or SIGBREAK (Windows) and the exit of `--parent` cancel between and inside files with exit code 130.
 * Progress goes to stderr: `[asset i/n] id`, then permille of the running asset with the phase.
 *
 * @see ../🗿️model/🟦️.ts — naming
 * @see ../🗜️zip/📦️models/🟦️.ts — the bundle the ZIP equals
 */
import { createHash } from "node:crypto";
import { mkdir, readFile, readdir, rename, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { LANGUAGES, type Asset, type Language, type Slug, type Text } from "../../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, assetPathsFromFiles, loadEnvironments, readStreamedAsset } from "../../🗂️catalog/📥️files/🟦️.ts";
import { resolveThermalEnvironment } from "../../🗂️catalog/🟦️.ts";
import { VIEWS, type JobControl, type View } from "../../🔗️contracts/🟦️.ts";
import { ModelRequestError } from "../../🧊️volume/🟦️.ts";
import { writeFbxDetailed } from "../🎬️fbx/🟦️.ts";
import { isEntry, watchParent } from "../🎭️cast/🟦️.ts";
import { writeGlbDetailed } from "../🧊️glb/🟦️.ts";
import { modelExtension, modelStem, writeModel } from "../🗿️model/🟦️.ts";
import { writeZip, type ZipEntry } from "../🗜️zip/🟦️.ts";
import { producerDigest } from "./🔏️producer/🟦️.ts";

/** 🏷️ The schema id of the manifest. */
export const BATCH_SCHEMA = "semio.staffage-models/v1";

/** 🔢️ The version of the file set: a change of the layout or of the names changes it, and a resume never keeps files of another version. */
export const BATCH_GENERATOR = "staffage-models/1";

/** 📏️ The longest absolute path a Windows machine accepts without long-path opt-in. */
export const MAX_PATH_CHARS = 256;

/** 🗂️ What a file is. */
export type BatchKind = "glb" | "fbx" | "model" | "zip";

/** 🔄️ One file of the set. `triangles` is the number of triangles of the geometry the file holds or describes (null for the ZIP). */
export type BatchFile = {
  readonly asset: Slug;
  readonly subject: "pose" | "clip" | "asset";
  readonly item: Slug | "all";
  readonly view: View | "all";
  readonly kind: BatchKind;
  readonly file: string;
  readonly bytes: number;
  readonly sha256: string;
  readonly triangles: number | null;
};

/** 📁️ The options a set is made with; a resume keeps files only when they are equal. `producer` is the content digest of the code, schema, catalog and kernel the files are made by (see `🔏️producer`). */
export type BatchSettings = { readonly generator: string; readonly producer: string; readonly scale: number; readonly view: View; readonly environment: string | null; readonly presets: string; readonly thermalSpanK: number | null };

/** 🗃️ One asset of the manifest: the SHA-256 of its catalog document the files were made from. */
export type BatchAsset = { readonly id: Slug; readonly source: string };

/** 🧾️ The manifest of a set: the settings every file was made with, the assets with their source digests, the files sorted by path. */
export type BatchManifest = { readonly $schema: typeof BATCH_SCHEMA; readonly settings: BatchSettings; readonly assets: readonly BatchAsset[]; readonly files: readonly BatchFile[] };

/** ⚙️ The options of a run. */
export type BatchOptions = {
  readonly out: string;
  readonly catalog?: string;
  readonly scale: number;
  readonly assets?: readonly Slug[];
  readonly view?: View;
  readonly environment?: Slug;
  readonly thermalSpanK?: number;
  readonly force?: boolean;
  readonly started?: (asset: Slug, index: number, count: number) => void;
};

/** 📊️ The totals of a run. */
export type BatchSummary = { readonly assets: number; readonly files: number; readonly bytes: number; readonly written: number; readonly skipped: number; readonly writtenBytes: number };

/** 🚫️ Thrown for a malformed command line: the message carries English then German, `reason` both. */
export class BatchUsageError extends Error {
  constructor(readonly reason: Text) {
    super(`${reason.en}\n${reason.de}`);
    this.name = "BatchUsageError";
  }
}

/** 📂️ The default output directory: `📤️dist/🧊️models` of the TypeScript package, ignored by version control. */
export const DEFAULT_BATCH_OUT = resolve(import.meta.dirname, "../../../📦️packages/🟦️typescript/📤️dist/🧊️models");

const MANIFEST = "manifest.json";
const PRODUCER_BASE = resolve(import.meta.dirname, "../../../../../..");
const PRODUCER_ROOTS: readonly string[] = [resolve(import.meta.dirname, "../.."), resolve(import.meta.dirname, "../../../🧬️schema"), resolve(PRODUCER_BASE, "🧰️framework/🔨️modules/🧊️3d/🗿️model")];
const sha256 = (data: Uint8Array | string): string => createHash("sha256").update(data).digest("hex");
const encoder = new TextEncoder();
const LABEL: Text = { en: "Exporting 3D models", de: "3D-Modelle werden exportiert" };

async function readIfExists(path: string): Promise<Uint8Array | undefined> {
  try {
    return new Uint8Array(await readFile(path));
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
}

async function writeAtomic(path: string, data: Uint8Array | string): Promise<void> {
  if (path.length > MAX_PATH_CHARS) throw new ModelRequestError("model-batch-path", path);
  const temporary = `${path}.${process.pid}.tmp`;
  try {
    await writeFile(temporary, data);
    await rename(temporary, path);
  } finally {
    await rm(temporary, { force: true });
  }
}

async function readManifest(out: string): Promise<BatchManifest | undefined> {
  const bytes = await readIfExists(join(out, MANIFEST));
  if (bytes === undefined) return undefined;
  try {
    const parsed = JSON.parse(new TextDecoder().decode(bytes)) as BatchManifest;
    return parsed.$schema === BATCH_SCHEMA ? parsed : undefined;
  } catch {
    return undefined;
  }
}

const sameSettings = (a: BatchSettings, b: BatchSettings): boolean => a.generator === b.generator && a.producer === b.producer && a.scale === b.scale && a.view === b.view && a.environment === b.environment && a.presets === b.presets && a.thermalSpanK === b.thermalSpanK;

type Planned = { readonly subject: BatchFile["subject"]; readonly item: BatchFile["item"]; readonly view: BatchFile["view"]; readonly kind: BatchKind; readonly name: string; readonly make?: () => Promise<{ readonly data: Uint8Array | string; readonly triangles: number | null }> };

/** 🗺️ The file names of an asset in bundle order: per pose a GLB then its JSON, per clip an FBX then its JSON, last the ZIP. */
export function plannedNames(asset: Asset, scale: number, view: View): readonly { readonly subject: BatchFile["subject"]; readonly item: Slug | "all"; readonly view: View | "all"; readonly kind: BatchKind; readonly name: string }[] {
  const out: { subject: BatchFile["subject"]; item: Slug | "all"; view: View | "all"; kind: BatchKind; name: string }[] = [];
  for (const pose of asset.poses.filter((p) => p.key !== true)) {
    out.push({ subject: "pose", item: pose.id, view: "all", kind: "glb", name: `${modelStem(asset.id, pose.id, "all", scale)}.glb` });
    out.push({ subject: "pose", item: pose.id, view: "all", kind: "model", name: `${modelStem(asset.id, pose.id, "all", scale)}.${modelExtension("pose")}` });
  }
  for (const clip of asset.clips) {
    out.push({ subject: "clip", item: clip.id, view, kind: "fbx", name: `${modelStem(asset.id, clip.id, view, scale)}.fbx` });
    out.push({ subject: "clip", item: clip.id, view: "all", kind: "model", name: `${modelStem(asset.id, clip.id, "all", scale)}.${modelExtension("clip")}` });
  }
  out.push({ subject: "asset", item: "all", view: "all", kind: "zip", name: `${asset.id}-all-model-1to${scale}.zip` });
  return out;
}

/** 📤️ The bundle member folder of a kind. */
export const MEMBER_FOLDER: Readonly<Record<Exclude<BatchKind, "zip">, string>> = { glb: "glb", fbx: "fbx", model: "model" };

/** ⏳️ Writes the model set of the catalog into `options.out`; returns the manifest and the totals. Rejects with the signal's reason when cancelled (the manifest then lists the assets finished so far). */
export async function exportModels(options: BatchOptions, control: JobControl = {}): Promise<{ readonly manifest: BatchManifest; readonly summary: BatchSummary }> {
  if (!(options.scale > 0) || !Number.isFinite(options.scale)) throw new ModelRequestError("model-bad-option", "scale");
  const out = resolve(options.out);
  const catalog = resolve(options.catalog ?? DEFAULT_CATALOG_ROOT);
  const view = options.view ?? "light-solid";
  const presetList = (await loadEnvironments(catalog, control.signal))?.presets ?? [];
  const presets = new Map(presetList.map((preset) => [preset.id, preset]));
  const presetsDigest = sha256(JSON.stringify(presetList));
  const entries = await assetPathsFromFiles(catalog, control.signal);
  const wanted = options.assets === undefined ? undefined : new Set(options.assets);
  if (wanted !== undefined) {
    const known = new Set(entries.map((e) => e.folder));
    const unknown = [...wanted].filter((id) => !known.has(id));
    if (unknown.length > 0) throw new BatchUsageError({ en: `unknown assets: ${unknown.join(", ")}`, de: `unbekannte Assets: ${unknown.join(", ")}` });
  }
  const selected = entries.filter((e) => wanted === undefined || wanted.has(e.folder));
  await mkdir(out, { recursive: true });
  const settings: BatchSettings = { generator: BATCH_GENERATOR, producer: await producerDigest(PRODUCER_ROOTS, PRODUCER_BASE, control.signal), scale: options.scale, view, environment: options.environment ?? null, presets: presetsDigest, thermalSpanK: options.thermalSpanK ?? null };
  const found = await readManifest(out);
  const previous = found !== undefined && sameSettings(found.settings, settings) ? found : undefined;
  const assets = new Map<Slug, BatchAsset>((previous?.assets ?? []).map((a) => [a.id, a]));
  const files = new Map<string, BatchFile>((previous?.files ?? []).map((f) => [f.file, f]));
  const known = new Set(entries.map((e) => e.folder));
  for (const [id] of assets) if (!known.has(id)) assets.delete(id);
  for (const [path, file] of files) if (!known.has(file.asset)) files.delete(path);
  let written = 0;
  let skipped = 0;
  let writtenBytes = 0;
  const save = async (): Promise<BatchManifest> => {
    const manifest: BatchManifest = { $schema: BATCH_SCHEMA, settings, assets: [...assets.values()].sort((a, b) => a.id.localeCompare(b.id)), files: [...files.values()].sort((a, b) => (a.file < b.file ? -1 : a.file > b.file ? 1 : 0)) };
    await writeAtomic(join(out, MANIFEST), `${JSON.stringify(manifest, null, 1)}\n`);
    return manifest;
  };

  for (const [index, entry] of selected.entries()) {
    control.signal?.throwIfAborted();
    options.started?.(entry.folder, index + 1, selected.length);
    const streamed = await readStreamedAsset(entry, control.signal);
    const asset = streamed.asset;
    if (asset.volume === undefined) throw new ModelRequestError("model-no-volume", asset.id);
    const environment = resolveThermalEnvironment(asset, presets, options.environment);
    const shared = { scale: options.scale, ...(environment === undefined ? {} : { environment }), ...(options.thermalSpanK === undefined ? {} : { thermalSpanK: options.thermalSpanK }) };
    const source = sha256(streamed.text);
    const before = assets.get(asset.id);
    const reusable = options.force !== true && before !== undefined && before.source === source;
    const folder = join(out, asset.id);
    await mkdir(folder, { recursive: true });
    const plan = plannedNames(asset, options.scale, view);
    const stage = (phase: number): JobControl => ({
      ...(control.signal ? { signal: control.signal } : {}),
      progress: (done, total, label) => control.progress?.(index * 1000 + Math.min(1000, Math.round(((phase + done / total) / plan.length) * 1000)), selected.length * 1000, label),
    });
    const produced = new Map<string, BatchFile>();
    const keep = async (planned: (typeof plan)[number]): Promise<BatchFile | undefined> => {
      const relative = `${asset.id}/${planned.name}`;
      const old = files.get(relative);
      if (!reusable || old === undefined) return undefined;
      const bytes = await readIfExists(join(folder, planned.name));
      return bytes !== undefined && bytes.length === old.bytes && sha256(bytes) === old.sha256 ? old : undefined;
    };
    const emit = async (planned: (typeof plan)[number], data: Uint8Array | string, triangles: number | null): Promise<BatchFile> => {
      const bytes = typeof data === "string" ? encoder.encode(data) : data;
      await writeAtomic(join(folder, planned.name), bytes);
      written += 1;
      writtenBytes += bytes.length;
      return { asset: asset.id, subject: planned.subject, item: planned.item, view: planned.view, kind: planned.kind, file: `${asset.id}/${planned.name}`, bytes: bytes.length, sha256: sha256(bytes), triangles };
    };

    const trianglesOf = new Map<string, number>();
    let regenerated = false;
    for (const [position, planned] of plan.entries()) {
      control.signal?.throwIfAborted();
      if (planned.kind === "zip") continue;
      const kept = await keep(planned);
      const stagedControl = stage(position);
      let file: BatchFile;
      if (kept !== undefined) {
        file = kept;
        skipped += 1;
      } else {
        regenerated = true;
        if (planned.kind === "glb") {
          const result = await writeGlbDetailed(asset, { ...shared, pose: planned.item as Slug }, stagedControl);
          file = await emit(planned, result.bytes, result.triangles);
        } else if (planned.kind === "fbx") {
          const result = await writeFbxDetailed(asset, { ...shared, clip: planned.item as Slug, view }, stagedControl);
          file = await emit(planned, result.bytes, result.triangles);
        } else {
          const text = await writeModel(asset, { ...shared, ...(planned.subject === "pose" ? { pose: planned.item as Slug } : { clip: planned.item as Slug }) }, stagedControl);
          file = await emit(planned, text, trianglesOf.get(`${planned.subject}:${planned.item}`) ?? null);
        }
      }
      if (planned.kind !== "model" && file.triangles !== null) trianglesOf.set(`${planned.subject}:${planned.item}`, file.triangles);
      produced.set(planned.name, file);
    }

    const zipPlan = plan[plan.length - 1]!;
    const keptZip = regenerated ? undefined : await keep(zipPlan);
    if (keptZip !== undefined) {
      produced.set(zipPlan.name, keptZip);
      skipped += 1;
    } else {
      const members: ZipEntry[] = [];
      for (const planned of plan) {
        if (planned.kind === "zip") continue;
        control.signal?.throwIfAborted();
        members.push({ name: `${MEMBER_FOLDER[planned.kind]}/${planned.name}`, bytes: (await readIfExists(join(folder, planned.name)))! });
      }
      produced.set(zipPlan.name, await emit(zipPlan, writeZip(members, `${asset.id} 3D models 1:${options.scale}`), null));
    }
    for (const stale of await readdir(folder)) if (!produced.has(stale)) await rm(join(folder, stale), { force: true, recursive: true });
    for (const [path, file] of files) if (file.asset === asset.id) files.delete(path);
    for (const file of produced.values()) files.set(file.file, file);
    assets.set(asset.id, { id: asset.id, source });
    await save();
    control.progress?.((index + 1) * 1000, selected.length * 1000, LABEL);
  }
  const manifest = await save();
  const all = [...files.values()];
  return { manifest, summary: { assets: assets.size, files: all.length, bytes: all.reduce((n, f) => n + f.bytes, 0), written, skipped, writtenBytes } };
}

const FLAGS = ["lang", "out", "scale", "asset", "view", "environment", "span-k", "catalog", "parent", "force"];
const FLAG_LIST = "--lang <en|de> [--out <dir>] [--scale <denominator>] [--asset <a,b,c>] [--view <view>] [--environment <preset>] [--span-k <kelvin>] [--catalog <dir>] [--parent <pid>] [--force]";
const USAGE: Text = { en: `usage: ${FLAG_LIST} (scale default 50, every asset by default, FBX view default light-solid)`, de: `Aufruf: ${FLAG_LIST} (Massstab standardmässig 50, standardmässig jedes Asset, FBX-Ansicht standardmässig light-solid)` };
const bad = (en: string, de: string): BatchUsageError => new BatchUsageError({ en: `${en}\n${USAGE.en}`, de: `${de}\n${USAGE.de}` });

/** 🔤️ The parsed command line. */
export type BatchCommand = BatchOptions & { readonly language: Language; readonly parent?: number };

/** 🔧️ Parses `--key value` arguments and the bare flag `--force`; every refusal names the offending flag. */
export function parseBatchArguments(argv: readonly string[]): BatchCommand {
  const values = new Map<string, string>();
  for (let index = 0; index < argv.length; index += 1) {
    const key = argv[index];
    if (!key?.startsWith("--")) throw bad(`bad argument ${key ?? ""}`, `ungültiges Argument ${key ?? ""}`);
    const name = key.slice(2);
    if (!FLAGS.includes(name)) throw bad(`unknown --${name}`, `unbekanntes --${name}`);
    if (name === "force") {
      values.set(name, "true");
      continue;
    }
    const value = argv[index + 1];
    if (value === undefined || value.startsWith("--")) throw bad(`--${name} needs a value`, `--${name} braucht einen Wert`);
    values.set(name, value);
    index += 1;
  }
  const language = values.get("lang");
  if (language === undefined || !(LANGUAGES as readonly string[]).includes(language)) throw bad(`--lang must be one of ${LANGUAGES.join(", ")}`, `--lang muss eines von ${LANGUAGES.join(", ")} sein`);
  const number = (key: string, fallback?: number): number | undefined => {
    const raw = values.get(key);
    const value = raw === undefined ? fallback : Number(raw);
    if (value !== undefined && (!(value > 0) || !Number.isFinite(value))) throw bad(`--${key} must be a positive number`, `--${key} muss eine positive Zahl sein`);
    return value;
  };
  const view = values.get("view");
  if (view !== undefined && !(VIEWS as readonly string[]).includes(view)) throw bad(`--view must be one of ${VIEWS.join(", ")}, got ${view}`, `--view muss eines von ${VIEWS.join(", ")} sein, erhalten ${view}`);
  const parent = number("parent");
  if (parent !== undefined && !Number.isInteger(parent)) throw bad("--parent must be a process id", "--parent muss eine Prozess-ID sein");
  const assets = values.get("asset");
  const span = number("span-k");
  return {
    language: language as Language,
    out: resolve(values.get("out") ?? DEFAULT_BATCH_OUT),
    scale: number("scale", 50)!,
    ...(assets === undefined ? {} : { assets: assets.split(",").filter((id) => id.length > 0) as Slug[] }),
    ...(view === undefined ? {} : { view: view as View }),
    ...(values.has("environment") ? { environment: values.get("environment")! as Slug } : {}),
    ...(span === undefined ? {} : { thermalSpanK: span }),
    ...(values.has("catalog") ? { catalog: resolve(values.get("catalog")!) } : {}),
    ...(parent === undefined ? {} : { parent }),
    ...(values.has("force") ? { force: true } : {}),
  };
}

async function main(): Promise<void> {
  const controller = new AbortController();
  for (const signal of process.platform === "win32" ? (["SIGINT", "SIGTERM", "SIGBREAK"] as const) : (["SIGINT", "SIGTERM", "SIGHUP"] as const)) process.on(signal, () => controller.abort());
  process.stderr.write(`[pid ${process.pid}] ppid ${process.ppid}\n`);
  let stopWatching = (): void => {};
  const started = Date.now();
  try {
    const command = parseBatchArguments(process.argv.slice(2));
    if (command.parent !== undefined) stopWatching = watchParent(command.parent, () => { controller.abort(); setTimeout(() => process.exit(130), 5000).unref(); });
    let last = -1;
    const { summary } = await exportModels({ ...command, started: (id, index, count) => process.stderr.write(`[asset ${index}/${count}] ${id}\n`) }, {
      signal: controller.signal,
      progress: (done, total, label) => {
        const percent = Math.floor((done / total) * 100);
        if (percent !== last) {
          last = percent;
          process.stderr.write(`[${percent} %] ${label[command.language]}\n`);
        }
      },
    });
    stopWatching();
    process.stdout.write(`${command.out}\n`);
    process.stderr.write(`[done] ${summary.assets} assets, ${summary.files} files, ${summary.bytes} bytes; wrote ${summary.written} (${summary.writtenBytes} bytes), kept ${summary.skipped}; ${((Date.now() - started) / 1000).toFixed(1)} s; [peak] ${Math.round(process.resourceUsage().maxRSS / 1024)} MB resident\n`);
  } catch (error) {
    stopWatching();
    if (controller.signal.aborted) {
      process.stderr.write("[cancelled / abgebrochen]\n");
      process.exitCode = 130;
    } else {
      process.stderr.write(`${(error as Error).message}\n`);
      process.exitCode = 1;
    }
  }
}

if (isEntry(import.meta.url, process.argv[1])) await main();
