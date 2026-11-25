/** 📊️ Generates the multiview coverage matrix and survives long runs: `bun 📊️matrix.ts --language en|de --out <dir> [options]`.
 *
 * Every asset runs in a process of its own and writes one record `<out>/cells/<asset>.json` when it ends ({@link ./🗃️cells.ts}). Nothing is held until the end, so a cut loses only the assets that were running.
 * No process holds the catalog: an asset process reads its own document (and an alternative asset when it is asked for), the generator keeps a head per asset (id, category, digest, scale entries) and the merge streams asset by asset into `matrix.json`.
 *
 * - `--language en|de` (required): the language of the progress and summary lines; no language is assumed.
 * - `--out <dir>` (required): the run folder.
 * - `--assets a,b` and `--categories humans,vegetation`: the selection (default: the whole catalog).
 * - `--shard i/n`: runs only every n-th asset of the selection starting at i (0-based), for invocations on several machines; a shard does not merge.
 * - `--jobs N`: asset processes at a time (default 2). An asset process held 0.26 GB resident for a rigid asset, 0.37 GB for a person and 1.2 GB for a tree (finned-heat-exchanger, jogger, olive-tree on 2026-11-25); the generator itself reads one document at a time and keeps its head.
 * - `--progress <file>`: where the progress is written (default `<out>/progress.json`); invocations that run side by side in one run folder (the lanes of `full.sh`) each name their own.
 * - `--timeout-s N`: the most seconds one asset may take (default 7200); an asset that takes longer is stopped and recorded as the named failure `asset-timeout`.
 * - Resume: a rerun skips every asset that has a current record. `--retry-failed` also reruns the assets recorded as `asset-crashed` or `asset-timeout`; `--fresh` reruns the whole selection. A record of an asset document that changed since is stale and is run again.
 * - `--merge-only`: runs nothing and merges the records. `--from dirA,dirB`: further run folders (shards produced by separate invocations) to merge with `--out`.
 *
 * The merge writes `matrix.json` (every cell of the selection exactly once) and `summary.json` (counts per category, view and check, and the assets that are done, failed by name or missing) into `--out`.
 * A crash or a timeout inside one asset fails its cells as `asset-crashed` or `asset-timeout` with the message; only an asset no folder holds is `cell-missing`.
 * Progress goes to stderr (one line per ended asset with its seconds, the peak resident memory of its process and the estimated remaining time) and to `<out>/progress.json`, which is rewritten when an asset starts or ends and when a running asset reaches its next scale entry; SIGINT and SIGTERM cancel: the running assets are stopped, the finished records stay and the next run resumes. An asset process whose generator vanished (SIGKILL, a crash) ends by itself at its next scale entry and leaves no record. Exit code 0 when the run ended, 130 when it was cancelled, 2 on wrong usage.
 *
 * @see ./🧰️.ts — the engine
 * @see ./🗃️cells.ts — the records
 * @see ./🔎️scan.ts — the authors' gate: one asset in seconds
 */
import { spawn, type ChildProcess } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync, writeSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import type { Language } from "../../🧬️schema/🟦️.ts";
import { EXTREME_PROBES, MODE_LIST, UNIFORM_INSTANTS, VIEWPOINT_LIST, catalogLookup, loadAsset, runAsset } from "./🧰️.ts";
import { catalogHeads, failedRecord, pendingAssets, recordPath, runGuarded, writeMatrix, writeRecord, type AssetHead } from "./🗃️cells.ts";

const TEXT = {
  usage: { en: "usage: bun 📊️matrix.ts --language en|de --out <dir> [--assets a,b] [--categories c,d] [--shard i/n] [--jobs N] [--timeout-s N] [--progress <file>] [--retry-failed] [--fresh] [--merge-only] [--from dirA,dirB]", de: "Aufruf: bun 📊️matrix.ts --language en|de --out <Ordner> [--assets a,b] [--categories c,d] [--shard i/n] [--jobs N] [--timeout-s N] [--progress <Datei>] [--retry-failed] [--fresh] [--merge-only] [--from OrdnerA,OrdnerB]" },
  plan: { en: (n: number, skipped: number, jobs: number) => `${n} assets to run, ${skipped} finished ones skipped, ${jobs} at a time`, de: (n: number, skipped: number, jobs: number) => `${n} Assets zu rechnen, ${skipped} fertige übersprungen, ${jobs} gleichzeitig` },
  done: { en: "done", de: "fertig" },
  failed: { en: "failed", de: "fehlgeschlagen" },
  remaining: { en: "remaining about", de: "verbleibend etwa" },
  peak: { en: "peak", de: "Spitze" },
  running: { en: "running", de: "läuft" },
  cancelled: { en: "cancelled: the finished records stay, the next run resumes", de: "abgebrochen: die fertigen Datensätze bleiben, der nächste Lauf setzt fort" },
  timeout: { en: (s: number, at: string) => `the asset did not end within ${s} s (${at})`, de: (s: number, at: string) => `das Asset wurde nicht innerhalb von ${s} s fertig (${at})` },
  crashed: { en: (how: string, at: string, last: string) => `the asset process ended with ${how} (${at})${last ? `: ${last}` : ""}`, de: (how: string, at: string, last: string) => `der Asset-Prozess endete mit ${how} (${at})${last ? `: ${last}` : ""}` },
  entry: { en: (i: number, n: number, scale: string) => `scale entry ${i}/${n} ${scale}`, de: (i: number, n: number, scale: string) => `Massstabseintrag ${i}/${n} ${scale}` },
  noEntry: { en: "before the first scale entry", de: "vor dem ersten Massstabseintrag" },
  shard: { en: "shard finished; merge with --merge-only (and --from for the folders of other invocations)", de: "Shard fertig; zusammenführen mit --merge-only (und --from für die Ordner anderer Aufrufe)" },
  merged: { en: (cells: number, ok: number, unavailable: number, fail: number, done: number, failed: number, missing: number) => `${cells} cells: ${ok} ok, ${unavailable} unavailable, ${fail} fail; assets: ${done} done, ${failed} failed by name, ${missing} missing`, de: (cells: number, ok: number, unavailable: number, fail: number, done: number, failed: number, missing: number) => `${cells} Zellen: ${ok} ok, ${unavailable} nicht verfügbar, ${fail} fehlgeschlagen; Assets: ${done} fertig, ${failed} namentlich fehlgeschlagen, ${missing} fehlend` },
} as const;

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const has = (name: string): boolean => args.includes(`--${name}`);
const language = flag("language") as Language | undefined;
if ((language !== "en" && language !== "de") || flag("out") === undefined) {
  console.error(`${TEXT.usage.en}\n${TEXT.usage.de}`);
  process.exit(2);
}
const out = resolve(flag("out")!);

const single = flag("asset");
if (single !== undefined) {
  const asset = loadAsset(single);
  if (asset === undefined) {
    console.error(`no asset "${single}"`);
    process.exit(2);
  }
  const parent = process.ppid;
  const record = runGuarded(asset, (target) =>
    runAsset(target, catalogLookup(), (index, count, scale) => {
      if (process.ppid !== parent) process.exit(1);
      writeSync(1, `entry ${index + 1} ${count} ${scale}\n`);
    }),
  );
  writeRecord(out, record);
  writeSync(1, `peak ${Math.round(process.resourceUsage().maxRSS / 1024)}\n`);
  process.exit(0);
}

const only = flag("assets")?.split(",");
const categories = flag("categories")?.split(",");
const selection = catalogHeads(only).filter((head) => categories === undefined || categories.includes(head.category));
const clock = (seconds: number): string => `${String(Math.floor(seconds / 3600)).padStart(2, "0")}:${String(Math.floor((seconds % 3600) / 60)).padStart(2, "0")}:${String(Math.floor(seconds % 60)).padStart(2, "0")}`;

mkdirSync(out, { recursive: true });
let cancelled = false;

if (!has("merge-only")) {
  const shard = flag("shard");
  const [index, count] = shard === undefined ? [0, 1] : (shard.split("/").map(Number) as [number, number]);
  const mine = selection.filter((_, i) => i % count === index);
  const todo = has("fresh") ? mine : pendingAssets(out, mine, has("retry-failed"));
  const jobs = Math.max(1, Math.min(Number(flag("jobs") ?? 2), Math.max(1, todo.length)));
  const timeoutS = Number(flag("timeout-s") ?? 7200);
  const script = fileURLToPath(import.meta.url);
  const progressFile = resolve(flag("progress") ?? join(out, "progress.json"));
  const started = Date.now();
  const running = new Map<ChildProcess, { asset: AssetHead; started: number; entry: readonly [number, number, string] | null; peakMb?: number }>();
  const entryText = (entry: readonly [number, number, string] | null, lang: Language): string => (entry === null ? TEXT.noEntry[lang] : TEXT.entry[lang](...entry));
  const ended: { asset: string; outcome: "done" | "failed"; seconds: number }[] = [];
  const queue = [...todo];
  const progress = (): void => {
    writeFileSync(`${progressFile}.part`, JSON.stringify({ total: todo.length, done: ended.filter((e) => e.outcome === "done").length, failed: ended.filter((e) => e.outcome === "failed").map((e) => e.asset), pending: queue.length, running: [...running.values()].map((r) => ({ asset: r.asset.id, entry: entryText(r.entry, language), seconds: Math.round((Date.now() - r.started) / 1000) })), elapsedS: Math.round((Date.now() - started) / 1000), cancelled }, null, 2));
    renameSync(`${progressFile}.part`, progressFile);
  };
  console.error(`[matrix] ${TEXT.plan[language](todo.length, mine.length - todo.length, jobs)}`);
  await new Promise<void>((finish) => {
    const next = (): void => {
      while (!cancelled && running.size < jobs && queue.length > 0) start(queue.shift()!);
      progress();
      if (running.size === 0 && (cancelled || queue.length === 0)) finish();
    };
    const start = (asset: AssetHead): void => {
      const child = spawn(process.execPath, [script, "--language", language, "--out", out, "--asset", asset.id], { stdio: ["ignore", "pipe", "pipe"] });
      const state: { asset: AssetHead; started: number; entry: readonly [number, number, string] | null; peakMb?: number } = { asset, started: Date.now(), entry: null };
      running.set(child, state);
      let timedOut = false;
      let tail = "";
      const timer = setTimeout(() => {
        timedOut = true;
        child.kill("SIGKILL");
      }, timeoutS * 1000);
      child.stdout!.on("data", (chunk: Buffer) => {
        for (const line of chunk.toString("utf8").split("\n")) {
          const [word, i, n, ...scale] = line.split(" ");
          if (word === "peak") state.peakMb = Number(i);
          if (word !== "entry") continue;
          state.entry = [Number(i), Number(n), scale.join(" ")];
          progress();
        }
      });
      child.stderr!.on("data", (chunk: Buffer) => void (tail = (tail + chunk.toString("utf8")).slice(-2000)));
      child.on("exit", (code, signal) => {
        clearTimeout(timer);
        running.delete(child);
        const seconds = (Date.now() - state.started) / 1000;
        if (!cancelled) {
          const path = recordPath(out, asset.id);
          const written = existsSync(path) && statSync(path).mtimeMs >= state.started - 1000;
          if (timedOut && !written) writeRecord(out, failedRecord(asset, "asset-timeout", TEXT.timeout.en(timeoutS, entryText(state.entry, "en")), seconds));
          else if (!written) writeRecord(out, failedRecord(asset, "asset-crashed", TEXT.crashed.en(signal !== null ? `signal ${signal}` : `exit code ${code}`, entryText(state.entry, "en"), tail.trim().split("\n").pop()?.slice(0, 300) ?? ""), seconds));
          const outcome = written && code === 0 && !existsFailure(path) ? "done" : "failed";
          ended.push({ asset: asset.id, outcome, seconds });
          const mean = ended.reduce((sum, e) => sum + e.seconds, 0) / ended.length;
          const left = ((queue.length + running.size) * mean) / jobs;
          console.error(`[matrix] [${ended.length}/${todo.length}] ${TEXT[outcome][language]} ${asset.id} ${seconds.toFixed(0)} s${state.peakMb === undefined ? "" : ` · ${TEXT.peak[language]} ${state.peakMb} MB`} · ${TEXT.running[language]} ${running.size} · ${TEXT.remaining[language]} ${clock(left)}`);
        }
        next();
      });
    };
    for (const signal of ["SIGINT", "SIGTERM"] as const)
      process.on(signal, () => {
        if (cancelled) return;
        cancelled = true;
        for (const child of running.keys()) child.kill("SIGKILL");
        if (running.size === 0) finish();
      });
    next();
  });
  progress();
  if (cancelled) {
    console.error(`[matrix] ${TEXT.cancelled[language]}`);
    process.exit(130);
  }
  if (shard !== undefined) {
    console.error(`[matrix] ${TEXT.shard[language]}`);
    process.exit(0);
  }
}

function existsFailure(path: string): boolean {
  try {
    return (JSON.parse(readFileSync(path, "utf8")) as { outcome?: string }).outcome === "failed";
  } catch {
    return true;
  }
}

const coverage = {
  exhaustive: ["representation decisions of every preset and declared key", "valid-range band edges of every authored representation", "unavailable declarations: reason, limiting feature and size, alternatives and their targets", "geometry of every pose in the three viewpoints", "cross-view synchrony of every evaluated instant (forward, height, lateral)", "default-pose dimensions against widthM, depthM, heightM", "loop closure of every looping clip", "all seven modes of every pose at every scale entry"],
  sampled: [
    `clip instants: start, every keyframe, mid of every key interval, extremes of rig bounds and view extents (${EXTREME_PROBES} probes), ${UNIFORM_INSTANTS} uniform intermediates (16 for clips longer than 600 s), end; all seven modes at every instant`,
    "transitions: every pose to the rest pose plus three seeded random pairs (mulberry32 seeded by the FNV-1a hash of the asset id) at 0, 0.25, 0.5, 0.75 and 1 of one second; all seven modes",
    "languages: English at every subject; German additionally at every pose and at the start, key, extreme and end instants (not at uniform, mid or transition instants)",
    "band edges (validRange ends, up to 1:1000): poses and the start and end instants of every clip",
    "alternatives: scale alternatives reuse the target cells, symbols render every pose, asset alternatives are checked for an authored target at the scale",
  ],
  viewpoints: VIEWPOINT_LIST,
  modes: MODE_LIST,
};
const current = catalogHeads(selection.map((head) => head.id));
const { summary, run, cells } = writeMatrix(out, current, [out, ...(flag("from")?.split(",").map((folder) => resolve(folder)) ?? [])], coverage);
console.error(`[matrix] ${TEXT.merged[language](cells, summary.total.ok, summary.total.unavailable, summary.total.fail, run.done, run.failed.length, run.missing.length)}`);
