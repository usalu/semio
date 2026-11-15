/** 📦️ The complete 3D model set of the catalog as files: per asset one GLB per named pose (every authored scale and mode as glTF scenes, all seven views inside), the multi-scene neutral model JSON of every pose and clip, one FBX per clip, authored scale and view, and the ZIP of all of them, plus one manifest of the whole set.
 *
 * `repo tools exec bun -- 🔨️modules/📤️export/📦️batch/🟦️.ts --lang en|de [--out dir] [--scale 50,100] [--view light-solid,thermal] [--asset a,b,c] [--environment preset] [--span-k kelvin] [--catalog dir] [--parent pid] [--force]`
 *
 * Output layout under `--out` (default `📦️packages/🟦️typescript/📤️dist/🧊️models` of this package, which is not checked in):
 * `manifest.json` and `<asset>/<asset>.<pose>.glb`, `<asset>/<asset>.<pose>.pose.json`, `<asset>/<asset>.<clip>.clip.json`, `<asset>/<asset>.<clip>.s<scale>.<view>.fbx`, `<asset>/<asset>.zip` (members `glb/`, `fbx/`, `model/`, every planned file that exists), the names of {@link plannedFiles}.
 * Every absolute path stays under 256 characters (the Windows limit).
 *
 * Scales: one run exports every authored scale of an asset (`--scale` and `--view` restrict only the FBX files of this run); a run never deletes a file of another scale, whatever its key: the only files removed from an asset folder are those the asset's complete plan no longer names. An asset with no authored scale is not exported and is listed in the summary; naming it with `--asset` is refused.
 * Properties: assets are streamed one at a time (one parsed asset and its files in memory), every file is written through a temporary name and renamed, the manifest is rewritten after every asset so an interrupted run leaves a consistent set, the bytes are deterministic (no time stamps anywhere, the manifest carries none), and a run resumes: a file is kept when the key it was written with (the content digest of the asset document, the producing code, schema and kernel, the environment presets and the thermal options) equals the current key and its bytes still hash to the manifest; `--force` recomputes.
 * SIGINT, SIGTERM, SIGHUP (POSIX) or SIGBREAK (Windows) and the exit of `--parent` cancel between and inside files with exit code 130. Progress goes to stderr: `[asset i/n] id`, then permille of the running asset with the phase.
 *
 * @see ../🗿️model/🟦️.ts — naming and `plannedFiles`
 * @see ../🗜️zip/📦️models/🟦️.ts — the bundle the ZIP equals
 */
import { createHash } from "node:crypto";
import { mkdir, readFile, readdir, rename, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { LANGUAGES, type Asset, type Language, type Slug, type Text } from "../../../🧬️schema/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, assetPathsFromFiles, loadEnvironments, readStreamedAsset } from "../../🗂️catalog/📥️files/🟦️.ts";
import { resolveThermalEnvironment } from "../../🗂️catalog/🟦️.ts";
import { VIEWS, type JobControl, type View } from "../../🔗️contracts/🟦️.ts";
import { ModelRequestError, authoredScales, modeOf } from "../../🧊️volume/🟦️.ts";
import { writeFbxViews } from "../🎬️fbx/🟦️.ts";
import { isEntry, watchParent } from "../🎭️cast/🟦️.ts";
import { writeGlbDetailed } from "../🧊️glb/🟦️.ts";
import { type FileKind, type PlannedFile, MEMBER_FOLDER, plannedFiles, writeModel } from "../🗿️model/🟦️.ts";
import { writeZip, type ZipEntry } from "../🗜️zip/🟦️.ts";
import { producerDigest } from "./🔏️producer/🟦️.ts";

export { plannedFiles, MEMBER_FOLDER, type PlannedFile } from "../🗿️model/🟦️.ts";

/** 🏷️ The schema id of the manifest. */
export const BATCH_SCHEMA = "semio.staffage-models/v2";

/** 🔢️ The version of the file set: a change of the layout or of the names changes it, and a resume never keeps files of another version. */
export const BATCH_GENERATOR = "staffage-models/2";

/** 📏️ The longest absolute path a Windows machine accepts without long-path opt-in. */
export const MAX_PATH_CHARS = 256;

/** 🗂️ What a file is. */
export type BatchKind = FileKind;

/** 🔄️ One file of the set. `scale` is the denominator of an FBX and `null` for the files that hold every authored scale; `key` is the content digest the file was written with (see the module description); `triangles` is the number of triangles of the geometry the file holds or describes (null where unknown). */
export type BatchFile = {
  readonly asset: Slug;
  readonly subject: "pose" | "clip" | "asset";
  readonly item: Slug | "all";
  readonly scale: number | null;
  readonly view: View | "all";
  readonly kind: BatchKind;
  readonly file: string;
  readonly bytes: number;
  readonly sha256: string;
  readonly triangles: number | null;
  readonly key: string;
};

/** 📁️ What the last run was made with (informational: the resume decision is each file's own `key`). `producer` is the content digest of the code, schema and kernel the files are made by (see `🔏️producer`). */
export type BatchSettings = { readonly generator: string; readonly producer: string; readonly environment: string | null; readonly presets: string; readonly thermalSpanK: number | null };

/** 🗃️ One asset of the manifest: the SHA-256 of its catalog document the files were made from. */
export type BatchAsset = { readonly id: Slug; readonly source: string };

/** 🧾️ The manifest of a set: the settings of the last run, the assets with their source digests, the files sorted by path. */
export type BatchManifest = { readonly $schema: typeof BATCH_SCHEMA; readonly settings: BatchSettings; readonly assets: readonly BatchAsset[]; readonly files: readonly BatchFile[] };

/** ⚙️ The options of a run: `scales` and `views` restrict the FBX files of this run (default every authored scale and all seven views); `producer` fixes the content digest of the producing code (default: computed from the tree, see `🔏️producer`) so that a tree edited while a run is in flight cannot change its resume keys. */
export type BatchOptions = {
  readonly out: string;
  readonly catalog?: string;
  readonly scales?: readonly number[];
  readonly views?: readonly View[];
  readonly assets?: readonly Slug[];
  readonly environment?: Slug;
  readonly thermalSpanK?: number;
  readonly force?: boolean;
  readonly producer?: string;
  readonly started?: (asset: Slug, index: number, count: number) => void;
};

/** 📊️ The totals of a run over the whole manifest, and the assets that have no authored scale yet and were not exported. */
export type BatchSummary = { readonly assets: number; readonly files: number; readonly bytes: number; readonly written: number; readonly skipped: number; readonly writtenBytes: number; readonly unauthored: readonly Slug[] };

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
    return parsed.$schema === BATCH_SCHEMA && parsed.settings.generator === BATCH_GENERATOR ? parsed : undefined;
  } catch {
    return undefined;
  }
}

/** 🗺️ The file names of an asset in bundle order, restricted by the run's scales and views; see {@link plannedFiles}. */
export const plannedNames = (asset: Asset, options: { readonly scales?: readonly number[]; readonly views?: readonly View[] } = {}): readonly PlannedFile[] => plannedFiles(asset, options);

/** 🔑️ The content digest a file is written with: the producing code, the asset document, the environment presets and the thermal options (the one resume key of every file of an asset, whatever its scale). */
export const keyOf = (producer: string, source: string, presets: string, environment: string | null, thermalSpanK: number | null): string => sha256(JSON.stringify([BATCH_GENERATOR, producer, source, presets, environment, thermalSpanK]));

/** ⏳️ Writes the model set of the catalog into `options.out`; returns the manifest and the totals. Rejects with the signal's reason when cancelled (the manifest then lists the assets finished so far). */
export async function exportModels(options: BatchOptions, control: JobControl = {}): Promise<{ readonly manifest: BatchManifest; readonly summary: BatchSummary }> {
  const out = resolve(options.out);
  const catalog = resolve(options.catalog ?? DEFAULT_CATALOG_ROOT);
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
  const producer = options.producer ?? (await producerDigest(PRODUCER_ROOTS, PRODUCER_BASE, control.signal));
  const settings: BatchSettings = { generator: BATCH_GENERATOR, producer, environment: options.environment ?? null, presets: presetsDigest, thermalSpanK: options.thermalSpanK ?? null };
  const key = (source: string): string => keyOf(producer, source, presetsDigest, settings.environment, settings.thermalSpanK);
  const previous = await readManifest(out);
  const assets = new Map<Slug, BatchAsset>((previous?.assets ?? []).map((a) => [a.id, a]));
  const files = new Map<string, BatchFile>((previous?.files ?? []).map((f) => [f.file, f]));
  const known = new Set(entries.map((e) => e.folder));
  for (const [id] of assets) if (!known.has(id)) assets.delete(id);
  for (const [path, file] of files) if (!known.has(file.asset)) files.delete(path);
  let written = 0;
  let skipped = 0;
  let writtenBytes = 0;
  const unauthored: Slug[] = [];
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
    if (authoredScales(asset.volumes).length === 0) {
      if (wanted !== undefined) throw new ModelRequestError("model-no-authored-scale", asset.id);
      unauthored.push(asset.id);
      continue;
    }
    const environment = resolveThermalEnvironment(asset, presets, options.environment);
    const shared = { ...(environment === undefined ? {} : { environment }), ...(options.thermalSpanK === undefined ? {} : { thermalSpanK: options.thermalSpanK }) };
    const source = sha256(streamed.text);
    const current = key(source);
    const folder = join(out, asset.id);
    await mkdir(folder, { recursive: true });
    const full = plannedFiles(asset);
    const plan = plannedFiles(asset, { ...(options.scales === undefined ? {} : { scales: options.scales }), ...(options.views === undefined ? {} : { views: options.views }) });
    const stage = (position: number): JobControl => ({
      ...(control.signal ? { signal: control.signal } : {}),
      progress: (done, total, label) => control.progress?.(index * 1000 + Math.min(1000, Math.round(((position + done / total) / plan.length) * 1000)), selected.length * 1000, label),
    });
    const relative = (planned: PlannedFile): string => `${asset.id}/${planned.name}`;
    const keep = async (planned: PlannedFile, expected: string): Promise<BatchFile | undefined> => {
      const old = files.get(relative(planned));
      if (options.force === true || old === undefined || old.key !== expected) return undefined;
      const bytes = await readIfExists(join(folder, planned.name));
      return bytes !== undefined && bytes.length === old.bytes && sha256(bytes) === old.sha256 ? old : undefined;
    };
    const emit = async (planned: PlannedFile, data: Uint8Array | string, triangles: number | null, fileKey: string): Promise<BatchFile> => {
      const bytes = typeof data === "string" ? encoder.encode(data) : data;
      await writeAtomic(join(folder, planned.name), bytes);
      written += 1;
      writtenBytes += bytes.length;
      return { asset: asset.id, subject: planned.subject, item: planned.item, scale: planned.scale, view: planned.view, kind: planned.kind, file: relative(planned), bytes: bytes.length, sha256: sha256(bytes), triangles, key: fileKey };
    };

    const produced = new Map<string, BatchFile>();
    const trianglesOfPose = new Map<string, number>();
    const fbxGroups = new Map<string, { clip: Slug; scale: number; views: View[]; positions: number[] }>();
    for (const [position, planned] of plan.entries()) {
      if (planned.kind !== "fbx") continue;
      const group = fbxGroups.get(`${planned.item}|${planned.scale}|${modeOf(planned.view as View)}`) ?? { clip: planned.item as Slug, scale: planned.scale!, views: [], positions: [] };
      group.views.push(planned.view as View);
      group.positions.push(position);
      fbxGroups.set(`${planned.item}|${planned.scale}|${modeOf(planned.view as View)}`, group);
    }
    const fbxByName = new Map<string, BatchFile>();
    for (const group of fbxGroups.values()) {
      control.signal?.throwIfAborted();
      const todo: View[] = [];
      for (const [offset, view] of group.views.entries()) {
        const planned = plan[group.positions[offset]!]!;
        const kept = await keep(planned, current);
        if (kept !== undefined) {
          fbxByName.set(planned.name, kept);
          skipped += 1;
        } else todo.push(view);
      }
      if (todo.length === 0) continue;
      const results = await writeFbxViews(asset, { ...shared, clip: group.clip, scale: group.scale, views: todo }, stage(group.positions[0]!));
      for (const { view, result } of results) {
        const planned = plan[group.positions[group.views.indexOf(view)]!]!;
        fbxByName.set(planned.name, await emit(planned, result.bytes, result.triangles, current));
      }
    }

    for (const [position, planned] of plan.entries()) {
      control.signal?.throwIfAborted();
      if (planned.kind === "zip") continue;
      let file: BatchFile;
      if (planned.kind === "fbx") file = fbxByName.get(planned.name)!;
      else {
        const kept = await keep(planned, current);
        if (kept !== undefined) {
          file = kept;
          skipped += 1;
        } else if (planned.kind === "glb") {
          const result = await writeGlbDetailed(asset, { ...shared, pose: planned.item as Slug }, stage(position));
          file = await emit(planned, result.bytes, result.triangles, current);
        } else {
          const text = await writeModel(asset, { ...shared, ...(planned.subject === "pose" ? { pose: planned.item as Slug } : { clip: planned.item as Slug }) }, stage(position));
          file = await emit(planned, text, planned.subject === "pose" ? (trianglesOfPose.get(planned.item as string) ?? null) : null, current);
        }
      }
      if (planned.kind === "glb" && file.triangles !== null) trianglesOfPose.set(planned.item as string, file.triangles);
      produced.set(planned.name, file);
    }
    for (const [name, file] of fbxByName) produced.set(name, file);

    const zipPlan = full[full.length - 1]!;
    const members: { planned: PlannedFile; record: BatchFile }[] = [];
    for (const planned of full) {
      if (planned.kind === "zip") continue;
      const record = produced.get(planned.name) ?? files.get(relative(planned));
      if (record !== undefined && (await readIfExists(join(folder, planned.name))) !== undefined) members.push({ planned, record });
    }
    const zipKey = sha256(JSON.stringify([current, members.map((m) => [m.planned.name, m.record.sha256])]));
    const keptZip = await keep(zipPlan, zipKey);
    if (keptZip !== undefined) {
      produced.set(zipPlan.name, keptZip);
      skipped += 1;
    } else {
      const zipEntries: ZipEntry[] = [];
      for (const { planned } of members) {
        control.signal?.throwIfAborted();
        zipEntries.push({ name: `${MEMBER_FOLDER[planned.kind as "glb"]}/${planned.name}`, bytes: (await readIfExists(join(folder, planned.name)))! });
      }
      produced.set(zipPlan.name, await emit(zipPlan, writeZip(zipEntries, `${asset.id} 3D models`), null, zipKey));
    }
    const owned = new Set(full.map((f) => f.name));
    for (const stale of await readdir(folder)) if (!owned.has(stale)) await rm(join(folder, stale), { force: true, recursive: true });
    for (const [path, file] of files) if (file.asset === asset.id && !owned.has(path.slice(asset.id.length + 1))) files.delete(path);
    for (const file of produced.values()) files.set(file.file, file);
    assets.set(asset.id, { id: asset.id, source });
    await save();
    control.progress?.((index + 1) * 1000, selected.length * 1000, LABEL);
  }
  const manifest = await save();
  const all = [...files.values()];
  return { manifest, summary: { assets: assets.size, files: all.length, bytes: all.reduce((n, f) => n + f.bytes, 0), written, skipped, writtenBytes, unauthored } };
}

const FLAGS = ["lang", "out", "scale", "asset", "view", "environment", "span-k", "catalog", "parent", "force"];
const FLAG_LIST = "--lang <en|de> [--out <dir>] [--scale <50,100,200,500>] [--asset <a,b,c>] [--view <view,view>] [--environment <preset>] [--span-k <kelvin>] [--catalog <dir>] [--parent <pid>] [--force]";
const USAGE: Text = { en: `usage: ${FLAG_LIST} (every authored scale, every asset and all seven FBX views by default; --scale and --view restrict the FBX files of this run and never delete the files of other scales)`, de: `Aufruf: ${FLAG_LIST} (standardmässig jeder erstellte Massstab, jedes Asset und alle sieben FBX-Ansichten; --scale und --view beschränken die FBX-Dateien dieses Laufs und löschen nie die Dateien anderer Massstäbe)` };
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
  const list = <T extends string>(key: string, allowed: readonly string[] | undefined, parse: (raw: string) => T): T[] | undefined => {
    const raw = values.get(key);
    if (raw === undefined) return undefined;
    const parts = raw.split(",").filter((p) => p.length > 0).map(parse);
    if (parts.length === 0) throw bad(`--${key} needs at least one value`, `--${key} braucht mindestens einen Wert`);
    if (allowed !== undefined) for (const part of parts) if (!allowed.includes(part)) throw bad(`--${key} must be among ${allowed.join(", ")}, got ${part}`, `--${key} muss unter ${allowed.join(", ")} sein, erhalten ${part}`);
    return parts;
  };
  const views = list<View>("view", VIEWS, (raw) => raw as View);
  const scales = list<string>("scale", ["50", "100", "200", "500"], (raw) => raw)?.map(Number);
  const parent = number("parent");
  if (parent !== undefined && !Number.isInteger(parent)) throw bad("--parent must be a process id", "--parent muss eine Prozess-ID sein");
  const assets = values.get("asset");
  const span = number("span-k");
  return {
    language: language as Language,
    out: resolve(values.get("out") ?? DEFAULT_BATCH_OUT),
    ...(assets === undefined ? {} : { assets: assets.split(",").filter((id) => id.length > 0) as Slug[] }),
    ...(scales === undefined ? {} : { scales }),
    ...(views === undefined ? {} : { views }),
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
    process.stderr.write(`[done] ${summary.assets} assets, ${summary.files} files, ${summary.bytes} bytes; wrote ${summary.written} (${summary.writtenBytes} bytes), kept ${summary.skipped}; ${summary.unauthored.length} assets without an authored scale not exported; ${((Date.now() - started) / 1000).toFixed(1)} s; [peak] ${Math.round(process.resourceUsage().maxRSS / 1024)} MB resident\n`);
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
