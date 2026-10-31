import { spawnSync } from "bun";
import { existsSync, readFileSync } from "node:fs";
import { sep } from "node:path";

/** 🚧️ One production manifest line that enables a testkit outside `[dev-dependencies]`. */
export interface TestkitLeak {
  readonly manifest: string;
  readonly line: number;
  readonly section: string;
  readonly text: string;
  readonly subject: string;
}

const PRODUCTION_SECTION = /^(workspace\.)?(dependencies|build-dependencies)$|^target\..+\.(dependencies|build-dependencies)$/;
const TESTKIT_FEATURE = /["']testkit["']/;
const TEST_DIRECTORIES = new Set(["🧪️tests", "🧷️companions", "🧰️testing", "🧰️fixtures", "🧫️fixtures", "🧰️test-support"]);
const TEST_PACKAGE = /^name\s*=\s*"[^"]*(testkit|test-support|testing|tests?|fixtures|laws)(-[0-9a-z]+)*"/m;
const TESTKIT_CRATE = /^[A-Za-z0-9_-]*(testkit|testing-caller)\s*=/;

/** 🔎️ A manifest is a test crate when it lives under a tests directory or is itself a testkit crate. */
export function manifestIsTestCrate(manifestPath: string, source: string): boolean {
  const parts = manifestPath.split(sep);
  return parts.some((part) => TEST_DIRECTORIES.has(part)) || TEST_PACKAGE.test(source);
}

/** 🧮️ The optional testkit candidates a production feature must never activate: the dependency names a feature graph is checked against. */
export type FeatureGraph = Readonly<Record<string, readonly string[]>>;

/**
 * 🕸️ The candidates a non-`testkit` feature transitively activates (`dep:X`, the bare name `X`, or `X/feature`; `X?/feature` activates nothing), plus `"default"` when `default` reaches the `testkit` feature.
 * A candidate reached solely through the feature named `testkit` is allowed: the walk never enters it.
 */
export function activatedTestkitCandidates(features: FeatureGraph, candidates: ReadonlySet<string>): ReadonlyMap<string, string> {
  const found = new Map<string, string>();
  for (const root of Object.keys(features)) {
    if (root === "testkit") continue;
    const seen = new Set<string>();
    const walk = (feature: string): void => {
      if (seen.has(feature) || feature === "testkit") return;
      seen.add(feature);
      for (const item of features[feature] ?? []) {
        if (item === "testkit" && root === "default") {
          if (!found.has("default")) found.set("default", root);
          continue;
        }
        const dependency = /^dep:(.+)$/.exec(item)?.[1] ?? (item.includes("?/") ? undefined : item.split("/")[0]);
        if (dependency !== undefined && candidates.has(dependency) && !found.has(dependency)) found.set(dependency, root);
        if (!item.startsWith("dep:") && !item.includes("/") && features[item] !== undefined) walk(item);
      }
    };
    walk(root);
  }
  return found;
}

/** 📜️ Reads the `[features]` table of a manifest, multi-line arrays included, with the line of each feature. */
function parseFeatures(lines: readonly string[]): { readonly graph: Record<string, string[]>; readonly lines: Record<string, number> } {
  const graph: Record<string, string[]> = {};
  const at: Record<string, number> = {};
  let section = "";
  let open: string | undefined;
  lines.forEach((raw, index) => {
    const text = raw.trim();
    const header = /^\[{1,2}([^\[\]]+)\]{1,2}$/.exec(text);
    if (header && open === undefined) {
      section = header[1]!;
      return;
    }
    if (section !== "features" || text === "" || text.startsWith("#")) return;
    if (open === undefined) {
      const start = /^([A-Za-z0-9_.-]+)\s*=\s*\[(.*)$/.exec(text);
      if (!start) return;
      open = start[1]!;
      graph[open] = [];
      at[open] = index + 1;
      collect(graph[open]!, start[2]!);
      if (start[2]!.includes("]")) open = undefined;
      return;
    }
    collect(graph[open]!, text);
    if (text.includes("]")) open = undefined;
  });
  return { graph, lines: at };
  function collect(into: string[], fragment: string): void {
    for (const match of fragment.split("]")[0]!.matchAll(/["']([^"']+)["']/g)) into.push(match[1]!);
  }
}

/** 🧪️ Finds every `testkit` feature, testkit crate or fixture-caller crate (`testing-caller`, which enables the job testkit) a production manifest enables in a normal dependency table, its default feature set, or any non-`testkit` feature that activates an optional one. */
export function findTestkitLeaks(manifestPath: string, source: string): readonly TestkitLeak[] {
  if (manifestIsTestCrate(manifestPath, source)) return [];
  const leaks: TestkitLeak[] = [];
  const candidates = new Set<string>();
  const sourceLines = source.split("\n");
  let section = "";
  sourceLines.forEach((raw, index) => {
    const text = raw.trim();
    const header = /^\[{1,2}([^\[\]]+)\]{1,2}$/.exec(text);
    if (header) {
      section = header[1]!;
      return;
    }
    if (text === "" || text.startsWith("#")) return;
    const name = /^([A-Za-z0-9_-]+)\s*=/.exec(text)?.[1] ?? "";
    const production = PRODUCTION_SECTION.test(section);
    if (production && /\boptional\s*=\s*true\b/.test(text) && (TESTKIT_FEATURE.test(text) || TESTKIT_CRATE.test(text))) {
      if (!section.startsWith("workspace.")) candidates.add(name);
      return;
    }
    const leaked = (production && (TESTKIT_FEATURE.test(text) || (TESTKIT_CRATE.test(text) && !section.startsWith("workspace.")))) || (section === "features" && /^default\s*=/.test(text) && TESTKIT_FEATURE.test(text));
    if (leaked) leaks.push({ manifest: manifestPath, line: index + 1, section, text, subject: section === "features" ? "default" : name });
  });
  const { graph, lines } = parseFeatures(sourceLines);
  for (const [subject, via] of activatedTestkitCandidates(graph, candidates)) {
    if (leaks.some((leak) => leak.subject === subject)) continue;
    leaks.push({ manifest: manifestPath, line: lines[via] ?? 0, section: "features", text: `${via} = [${(graph[via] ?? []).map((item) => `"${item}"`).join(", ")}] activates ${subject}`, subject });
  }
  return leaks;
}

/** 🌳️ Scans every tracked or untracked `Cargo.toml` of the checkout (ticket scratch excluded) for production testkit leaks. */
export function testkitLeaksOfRepository(root: string): { readonly leaks: readonly TestkitLeak[]; readonly manifests: readonly string[] } {
  const listing = spawnSync(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*Cargo.toml"], { cwd: root, stdout: "pipe" });
  const manifests = new TextDecoder().decode(listing.stdout).split("\0").filter((entry) => entry.endsWith("Cargo.toml") && !entry.startsWith(".") && existsSync(`${root}/${entry}`));
  return { manifests, leaks: manifests.flatMap((manifest) => findTestkitLeaks(manifest, readFileSync(`${root}/${manifest}`, "utf8"))) };
}

interface CargoDependency {
  readonly name: string;
  readonly kind: string | null;
  readonly optional: boolean;
  readonly rename?: string | null;
  readonly features: readonly string[];
}

interface CargoPackage {
  readonly manifest_path: string;
  readonly dependencies: readonly CargoDependency[];
  readonly features: Readonly<Record<string, readonly string[]>>;
}

/** 🦀️ The independent verdict: leaking `manifest:subject` rows read from `cargo metadata --no-deps` JSON — normal and build dependencies, the default feature set, and optional testkit dependencies any non-`testkit` feature activates — of non-test packages. */
export function cargoMetadataTestkitVerdict(metadata: { readonly packages: readonly CargoPackage[] }, root: string): readonly string[] {
  const rows: string[] = [];
  for (const pkg of metadata.packages) {
    const manifest = pkg.manifest_path.startsWith(`${root}/`) ? pkg.manifest_path.slice(root.length + 1) : pkg.manifest_path;
    if (manifestIsTestCrate(manifest, readFileSync(pkg.manifest_path, "utf8"))) continue;
    const candidates = new Set<string>();
    for (const dependency of pkg.dependencies) {
      if (dependency.kind === "dev") continue;
      const testkit = dependency.features.includes("testkit") || /(testkit|testing-caller)$/.test(dependency.name);
      if (!testkit) continue;
      if (dependency.optional) candidates.add(dependency.rename ?? dependency.name);
      else rows.push(`${manifest}:${dependency.rename ?? dependency.name}`);
    }
    for (const subject of activatedTestkitCandidates(pkg.features, candidates).keys()) rows.push(`${manifest}:${subject}`);
    if ((pkg.features.default ?? []).includes("testkit") && !rows.includes(`${manifest}:default`)) rows.push(`${manifest}:default`);
  }
  return rows.sort();
}

/** 🧾️ The scanner's verdict projected onto the same `manifest:subject` rows, restricted to the manifests `cargo metadata` covered. */
export function scannerRows(leaks: readonly TestkitLeak[], covered: ReadonlySet<string>): readonly string[] {
  return leaks.filter((leak) => covered.has(leak.manifest)).map((leak) => `${leak.manifest}:${leak.subject}`).sort();
}
