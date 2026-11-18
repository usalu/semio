import { spawnSync } from "node:child_process";
import { appendFileSync, readFileSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { attachedDiskImagesV1 } from "../🖼️images/🟦️.ts";

const protocol = schema.definitions.Protocol.const;

/** 🪪️ Names the process that owns a sandbox: its pid and the start time the system reports for it, so a recycled pid is never taken for the owner. */
export type SandboxOwnerV1 = Readonly<{ version: 1; pid: number; start: string | null }>;

/** 📒️ One attached image registered in a sandbox ledger. */
export type SandboxLedgerRowV1 = Readonly<{ version: 1; device: string; image: string; pid: number; start: string | null; atMs: number }>;

/** 🏷️ Names who an attached image belongs to. */
export type ImageKindV1 = "ours" | "leaked" | "foreign";

/** 🔎️ One attached image with its owner verdict and, for a sandbox image, the sandbox root. */
export type ClassifiedImageV1 = Readonly<{ image: string; target: string; device: string; kind: ImageKindV1; sandbox?: string }>;

/** 📊️ What one sweep saw and did. */
export type ImageSweepReportV1 = Readonly<{ total: number; ours: number; leaked: number; foreign: number; detached: number; failed: number; sandboxesRemoved: number; sandboxesKept: number }>;

/** ⏱️ Reads the start time the system reports for a process, or null when it is gone or unreadable. See https://man7.org/linux/man-pages/man1/ps.1.html */
export function processStartV1(pid: number): string | null {
  if (process.platform === "win32") return null;
  const run = spawnSync("/bin/ps", ["-o", "lstart=", "-p", String(pid)], { encoding: "utf8", env: { PATH: "/usr/bin:/bin", LC_ALL: "C" } });
  const text = run.status === 0 ? run.stdout.trim() : "";
  return text === "" ? null : text;
}

/** 🙋️ Writes the owner row of a sandbox root for the current process. */
export function writeSandboxOwnerV1(root: string, pid: number = process.pid, start: string | null = processStartV1(pid)): SandboxOwnerV1 {
  const owner: SandboxOwnerV1 = { version: 1, pid, start };
  writeFileSync(join(root, protocol.owner), JSON.stringify(owner));
  return owner;
}

/** 📖️ Reads the owner row of a sandbox root; a bare pid number is read as an owner without a start time, and anything else as no owner. */
export function readSandboxOwnerV1(root: string): SandboxOwnerV1 | undefined {
  let text: string;
  try { text = readFileSync(join(root, protocol.owner), "utf8").trim(); } catch { return undefined; }
  if (/^[0-9]+$/u.test(text)) return Number(text) > 0 ? { version: 1, pid: Number(text), start: null } : undefined;
  try {
    const row = JSON.parse(text) as SandboxOwnerV1;
    return row.version === 1 && Number.isSafeInteger(row.pid) && row.pid > 0 && (row.start === null || typeof row.start === "string") ? row : undefined;
  } catch { return undefined; }
}

/** ❤️ Tells whether the owner still runs: the pid exists and, when a start time was recorded, the process has that start time. */
export function sandboxOwnerAliveV1(owner: SandboxOwnerV1, start: (pid: number) => string | null = processStartV1): boolean {
  try { process.kill(owner.pid, 0); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "EPERM") return false; }
  return owner.start === null || start(owner.pid) === owner.start;
}

/** 🧾️ Appends the attached images below a sandbox root that its ledger does not list yet, with the registering process. */
export function recordSandboxImagesV1(root: string, images: readonly Readonly<{ image: string; device: string }>[] = attachedDiskImagesV1(), pid: number = process.pid, now: number = Date.now()): number {
  let real = root;
  try { real = realpathSync(root); } catch {}
  const path = join(real, protocol.ledger), known = new Set<string>();
  try { for (const line of readFileSync(path, "utf8").split("\n")) if (line) known.add(`${(JSON.parse(line) as SandboxLedgerRowV1).device}\0${(JSON.parse(line) as SandboxLedgerRowV1).image}`); } catch {}
  const start = processStartV1(pid);
  let added = 0;
  for (const row of images) {
    if (!row.image.startsWith(real + "/") || row.device === "" || known.has(`${row.device}\0${row.image}`)) continue;
    appendFileSync(path, JSON.stringify({ version: 1, device: row.device, image: row.image, pid, start, atMs: now } satisfies SandboxLedgerRowV1) + "\n");
    added++;
  }
  return added;
}

/** 🏷️ Sorts the attached images into ours (below a `.cache/b` of a checkout or a live sandbox), leaked (below a sandbox whose owner is gone) and foreign (everything else). */
export function classifyAttachedImagesV1(images: readonly Readonly<{ image: string; target: string; device: string }>[], alive: (root: string) => boolean = root => { const owner = readSandboxOwnerV1(root); return owner !== undefined && sandboxOwnerAliveV1(owner); }): readonly ClassifiedImageV1[] {
  const name = new RegExp(protocol.sandboxName, "u");
  return images.map(row => {
    const at = row.image.indexOf(protocol.sandboxSegment);
    if (at >= 0) {
      const rest = row.image.slice(at + protocol.sandboxSegment.length), first = rest.split("/")[0]!;
      if (name.test(first) && rest.length > first.length) {
        const sandbox = row.image.slice(0, at + protocol.sandboxSegment.length) + first;
        return { ...row, kind: alive(sandbox) ? "ours" as const : "leaked" as const, sandbox };
      }
    }
    return { ...row, kind: row.image.includes(protocol.ourSegment) ? "ours" as const : "foreign" as const };
  });
}

/** 🧹️ Detaches the images of dead test sandboxes, nested ones first, and removes the dead sandbox directories below the given parents; images of live sandboxes, of the checkout and foreign ones are never touched. */
export function sweepLeakedSandboxImagesV1(options: Readonly<{ parents?: readonly string[]; images?: () => readonly Readonly<{ image: string; target: string; device: string }>[]; detach?: (target: string) => boolean; under?: string; alive?: (root: string) => boolean; remove?: boolean }> = {}): ImageSweepReportV1 {
  const all = options.images ?? attachedDiskImagesV1, list = () => all().filter(row => options.under === undefined || row.image.startsWith(options.under + "/")), alive = options.alive ?? (root => { const owner = readSandboxOwnerV1(root); return owner !== undefined && sandboxOwnerAliveV1(owner); });
  const detach = options.detach ?? (target => spawnSync("/usr/bin/hdiutil", ["detach", target], { encoding: "utf8" }).status === 0 || spawnSync("/usr/bin/hdiutil", ["detach", "-force", target], { encoding: "utf8" }).status === 0);
  const before = classifyAttachedImagesV1(list(), alive);
  let detached = 0, failed = 0;
  for (const row of [...before].filter(item => item.kind === "leaked").sort((left, right) => right.image.length - left.image.length)) {
    if (detach(row.device === "" ? row.target : row.device)) detached++; else failed++;
  }
  const after = classifyAttachedImagesV1(list(), alive), busy = new Set(after.filter(item => item.sandbox !== undefined).map(item => item.sandbox!));
  let removed = 0, kept = 0;
  if (options.remove !== false) for (const parent of options.parents ?? []) {
    let names: string[];
    try { names = readdirSync(parent); } catch { continue; }
    for (const entry of names) {
      const root = join(parent, entry);
      if (!new RegExp(protocol.sandboxName, "u").test(entry)) continue;
      let real: string;
      try { real = realpathSync(root); } catch { continue; }
      if (dirname(real) !== realpathSync(parent) || alive(root) || busy.has(root) || [...busy].some(path => path === real)) { kept++; continue; }
      try { rmSync(real, { recursive: true, force: true, maxRetries: 3 }); removed++; } catch { kept++; }
    }
  }
  const count = (kind: ImageKindV1) => before.filter(item => item.kind === kind).length;
  return { total: before.length, ours: count("ours"), leaked: count("leaked"), foreign: count("foreign"), detached, failed, sandboxesRemoved: removed, sandboxesKept: kept };
}

/** 🗣️ Words the split of the attached images in the chosen language. */
export function describeImageSplitV1(language: "en" | "de", report: Readonly<{ total: number; ours: number; leaked: number; foreign: number }>): string {
  return protocol.messages[language].replace(/\{(total|ours|leaked|foreign)\}/gu, (_, key: "total" | "ours" | "leaked" | "foreign") => String(report[key]));
}

/** 🗣️ Words what one sweep detached in the chosen language. */
export function describeImageSweepV1(language: "en" | "de", detached: number, removed = 0, failed = 0): string {
  return protocol.messages.swept[language].replace("{detached}", String(detached)).replace("{removed}", String(removed)).replace("{failed}", String(failed));
}
