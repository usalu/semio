/** 📊️ Generates the multiview coverage matrix: `bun 📊️matrix.ts --out <dir> [--jobs N] [--assets a,b] [--shard i/n]`.
 *
 * Without `--shard` it spawns `--jobs` shard processes (default 8), merges their cells, writes `matrix.json` (every cell) and `summary.json` (counts per category, view and check) into `--out`
 * and fills in `cell-missing` for any cell a crashed shard did not produce. Progress goes to stderr; SIGINT and SIGTERM cancel the shards.
 *
 * @see ./🧰️.ts — the engine
 */
import { spawn } from "node:child_process";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { EXTREME_PROBES, MODE_LIST, UNIFORM_INSTANTS, VIEWPOINT_LIST, assemble, loadCatalog, runAsset, summarise, type Cell } from "./🧰️.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const out = resolve(flag("out") ?? ".");
const only = flag("assets")?.split(",");
const catalog = loadCatalog();
const selected = catalog.filter((asset) => only === undefined || only.includes(asset.id));
const catalogById = new Map(catalog.map((asset) => [asset.id, asset]));

const shard = flag("shard");
if (shard !== undefined) {
  const [index, count] = shard.split("/").map(Number) as [number, number];
  const mine = selected.filter((_, i) => i % count === index);
  const cells: Cell[] = [];
  mine.forEach((asset, i) => {
    console.error(`[shard ${shard}] ${i + 1}/${mine.length} ${asset.id}`);
    try {
      cells.push(...runAsset(asset, catalogById));
    } catch (error) {
      console.error(`[shard ${shard}] ${asset.id} crashed: ${(error as Error).message}`);
    }
  });
  writeFileSync(join(out, `shard-${index}.json`), JSON.stringify(cells));
  process.exit(0);
}

mkdirSync(out, { recursive: true });
const jobs = Math.max(1, Math.min(Number(flag("jobs") ?? 8), selected.length));
const script = fileURLToPath(import.meta.url);
const children = Array.from({ length: jobs }, (_, i) => spawn(process.execPath, [script, "--out", out, "--shard", `${i}/${jobs}`, ...(only ? ["--assets", only.join(",")] : [])], { stdio: ["ignore", "inherit", "inherit"] }));
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => (children.forEach((child) => child.kill()), process.exit(130)));
await Promise.all(children.map((child) => new Promise<void>((done) => child.on("exit", () => done()))));
const produced = Array.from({ length: jobs }, (_, i) => {
  try {
    return JSON.parse(readFileSync(join(out, `shard-${i}.json`), "utf8")) as Cell[];
  } catch {
    return [];
  }
}).flat();
const cells = assemble(selected, produced);
const summary = summarise(cells);
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
writeFileSync(join(out, "matrix.json"), JSON.stringify({ version: "semio.staffage.coverage/v1", assets: selected.length, coverage, summary, cells }));
writeFileSync(join(out, "summary.json"), JSON.stringify({ coverage, summary }, null, 2));
console.error(`[done] ${cells.length} cells: ${summary.total.ok} ok, ${summary.total.unavailable} unavailable, ${summary.total.fail} fail`);
