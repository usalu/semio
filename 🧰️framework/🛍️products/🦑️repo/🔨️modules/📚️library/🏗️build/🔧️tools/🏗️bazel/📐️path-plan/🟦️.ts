import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,readlinkSync,rmSync,symlinkSync} from "node:fs";
import {parse,win32} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import component from "../🧬️schema/🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

const policy = schema.definitions.Policy.const, budget = component.definitions.PathBudget.const;
if (validateJsonSchemaSubset(schema.definitions.PolicyShape, policy, schema).length) throw new Error("Closed Bazel path plan required");

/** 🧾️ The authored path plan: output roots, hermetic roots and the fillers the worst-case law measures with. */
export const BAZEL_PATH_PLAN_V1 = policy;
/** 📏️ One declared target the law measures: its kind, its declared source paths (repository relative, slash separated) and, for a hermetic target, the longest paths inside its dependency tree. */
export interface BazelPathTargetV1 {readonly id: string; readonly kind: "hermetic" | "host"; readonly sources: readonly string[]; readonly tree?: readonly string[]}
/** 🖥️ The host facts the law needs; `outputBase` and `checkout` may be absent when the caller cannot know them, which skips the families that depend on them. */
export interface BazelPathHostV1 {readonly platform: "darwin" | "linux" | "win32"; readonly arch: "arm64" | "x64"; readonly longPaths: "enabled" | "disabled" | "unprobed"; readonly outputBase?: string; readonly checkout?: string}
export interface BazelPathFamilyV1 {readonly name: string; readonly units: number}
export interface BazelPathRowV1 {readonly id: string; readonly kind: "hermetic" | "host"; readonly families: readonly BazelPathFamilyV1[]; readonly worst: BazelPathFamilyV1 | null; readonly limit: number; readonly slack: number; readonly fits: boolean}
export interface BazelPathLawV1 {readonly limit: number; readonly rows: readonly BazelPathRowV1[]; readonly over: readonly string[]; readonly fits: boolean}

/** 🔢️ Counts UTF-16 code units, the unit Windows MAX_PATH and the repository's 256 rule are stated in. */
export const bazelPathUnitsV1 = (text: string): number => text.length;

/** 🧭️ Names the short fixed output root of one checkout on a host whose path limit needs it, or nothing: `--output_user_root` is the root, `--output_base` is `<root><separator><slot>`, and the directory holding the real output bases is linked at the root so the data stays inside the finite products partition. The name is the first hex digits of the SHA-256 of the checkout path, so it is short, ASCII, stable and distinct per checkout. */
export function bazelOutputRootV1(platform: string, checkout: string, rules: typeof policy = policy): {readonly root: string; readonly base: (slot: string) => string} | undefined {
  const row = (rules.outputRoot as Readonly<Record<string, {template: string; checkoutDigits: number} | null>>)[platform];
  if (!row) return undefined;
  const windows = platform === "win32", separator = windows ? "\\" : "/", normalized = windows ? checkout.replaceAll("/", "\\").toLowerCase() : checkout;
  const drive = windows ? /^[A-Za-z]:/u.exec(checkout.replaceAll("/", "\\"))?.[0].toUpperCase() ?? rules.fillers.drive : rules.fillers.drive;
  const root = row.template.replace("{drive}", drive).replace("{checkout}", createHash("sha256").update(normalized).digest("hex").slice(0, row.checkoutDigits));
  return {root, base: slot => root + separator + slot};
}

/** 🔗️ Makes the short output root a link to the real directory of the output bases (a junction on Windows, a directory link elsewhere), idempotently: a root that already points there is kept, a stale link is replaced and a real directory is refused. */
export function linkBazelOutputRootV1(root: string, target: string, platform: string = process.platform): void {
  let current: string | undefined;
  try {current = lstatSync(root).isSymbolicLink() ? readlinkSync(root) : "directory";} catch (error) {if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;}
  if (current === "directory") {
    if (!existsSync(root)) return;
    throw new Error(`Short Bazel output root is a real directory: ${root}`);
  }
  if (current === target || current !== undefined && win32.normalize(current) === win32.normalize(target)) return;
  if (current !== undefined) rmSync(root);
  mkdirSync(parse(root).dir, {recursive: true});
  symlinkSync(target, root, platform === "win32" ? "junction" : "dir");
}

/** 🏠️ Renders the private root a hermetic action builds its isolated workspace in, for the law: the declared base and a work name of the authored width, then the workspace directory; `base` is joined to the execution root when it is relative, which only the caller that knows the output base can measure. */
export function bazelHermeticRootV1(platform: string, outputBase: string | undefined, rules: typeof policy = policy): {readonly units: number} | undefined {
  const row = (rules.hermeticRoot as Readonly<Record<string, {base: string; workDigits: number; workspace: string; separator: string}>>)[platform];
  if (!row) throw new Error(`Hermetic root required for platform: ${platform}`);
  const base = row.base.replace("{drive}", rules.fillers.drive), relative = !/^[A-Za-z]:|^[\\/]/u.test(base);
  if (relative && outputBase === undefined) return undefined;
  const text = (relative ? outputBase + "/" + rules.fillers.execroot : "") + base + row.separator + "0".repeat(row.workDigits) + row.separator + row.workspace + row.separator;
  return {units: bazelPathUnitsV1(text)};
}

/** 🚧️ The host limit in UTF-16 units: the repository rule, and on Windows the MAX_PATH of the host (the extended limit when long paths are enabled), less the terminating NUL. */
export function bazelPathLimitV1(host: Pick<BazelPathHostV1, "platform" | "longPaths">): number {
  return Math.min(budget.repositoryUnits, host.platform === "win32" ? (host.longPaths === "enabled" ? budget.platforms.win32.extendedMax : budget.platforms.win32.pathMax) - 1 : Infinity);
}

/** ⚖️ Computes, for every declared target, the longest path of each family a host tool opens and fails the targets over budget: the checkout path of a host task's sources, the aliased source label beneath the execution root (and beneath a sandbox where the host has one), the hermetic private root plus the repository path, the snapshot file (and its copy in a sandbox stash) and the longest dependency-tree entry. Families that need the output base or the checkout are skipped when the caller cannot name them. Independent Python implementation and language-agnostic corpus: [fixture](./🧪️tests/🧫️fixtures/🔣️.json). */
export function bazelPathLawV1(host: BazelPathHostV1, targets: readonly BazelPathTargetV1[], aliasLabel: (path: string) => string, rules: typeof policy = policy): BazelPathLawV1 {
  const row = budget.platforms[host.platform], limit = bazelPathLimitV1(host), configuration = row.configuration[host.arch], base = host.outputBase, fillers = rules.fillers;
  const execroot = fillers.execroot, longestSandbox = row.sandboxes.reduce((longest, name) => name.length > longest.length ? name : longest, "");
  const hermeticRoot = bazelHermeticRootV1(host.platform, base, rules), snapshot = `${execroot}bazel-out/${configuration}/bin/${budget.task}.snapshot/files/${budget.file}`, treePrefix = `${execroot}bazel-out/${configuration}/bin/${fillers.tree}/`;
  const rows = targets.map(target => {
    const families: BazelPathFamilyV1[] = [], longest = (values: readonly string[], measure: (value: string) => number) => values.reduce((best, value) => Math.max(best, measure(value)), 0);
    if (target.kind === "host" && host.checkout !== undefined && target.sources.length) families.push({name: "host-source", units: bazelPathUnitsV1(host.checkout) + 1 + longest(target.sources, bazelPathUnitsV1)});
    if (base !== undefined && target.sources.length) {
      families.push({name: "alias-source", units: bazelPathUnitsV1(base) + 1 + bazelPathUnitsV1(execroot) + longest(target.sources, path => bazelPathUnitsV1(aliasLabel(path)))});
      if (row.sandboxes.length) families.push({name: "sandbox-alias-source", units: bazelPathUnitsV1(base) + 1 + bazelPathUnitsV1(`sandbox/${longestSandbox}/${"0".repeat(budget.sandboxDigits)}/${execroot}`) + longest(target.sources, path => bazelPathUnitsV1(aliasLabel(path)))});
    }
    if (target.kind === "hermetic" && base !== undefined && row.sandboxes.length) families.push({name: "sandbox-stash", units: bazelPathUnitsV1(base) + 1 + bazelPathUnitsV1(`sandbox/sandbox_stash/${fillers.stash}/${"0".repeat(budget.sandboxDigits)}/${snapshot}`)});
    if (target.kind === "hermetic" && hermeticRoot && target.sources.length) families.push({name: "hermetic-root", units: hermeticRoot.units + longest(target.sources, bazelPathUnitsV1)});
    if (base !== undefined) families.push({name: "snapshot", units: bazelPathUnitsV1(base) + 1 + bazelPathUnitsV1(snapshot)});
    if (target.kind === "hermetic" && base !== undefined && target.tree?.length) families.push({name: "tree", units: bazelPathUnitsV1(base) + 1 + bazelPathUnitsV1(treePrefix) + longest(target.tree, bazelPathUnitsV1)});
    const worst = families.reduce<BazelPathFamilyV1 | null>((best, family) => best === null || family.units > best.units ? family : best, null), slack = worst === null ? limit : limit - worst.units;
    return {id: target.id, kind: target.kind, families, worst, limit, slack, fits: slack >= 0};
  });
  return Object.freeze({limit, rows, over: rows.filter(entry => !entry.fits).map(entry => entry.id), fits: rows.every(entry => entry.fits)});
}
