import { expect, test } from "bun:test";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { componentWorkspacesV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";

/** 🧱️ The fields of one `cargo build --unit-graph` unit the census reads (https://doc.rust-lang.org/cargo/reference/unstable.html#unit-graph). */
export type CargoUnitV1 = { readonly pkg_id: string; readonly target: { readonly name: string; readonly kind: readonly string[] }; readonly mode: string; readonly platform: string | null; readonly features: readonly string[]; readonly dependencies: readonly { readonly index: number }[] };

/** 🗺️ One planned package's own unit graph, resolved alone as the release driver's per-package feature unification resolves it. */
export type CargoPackageUnitGraphV1 = { readonly package: string; readonly units: readonly CargoUnitV1[] };

/** 🌳️ One compile unit key across the plan: the feature sets that select it (with the packages selecting each) and its distinct closure signatures. */
export type CargoUnitCensusRowV1 = { readonly unit: string; readonly featureSets: Readonly<Record<string, readonly string[]>>; readonly signatures: number };

type FixtureV1 = {
  readonly schema: "semio.hub.unit-census/v1";
  readonly workspaces: readonly string[];
  readonly target: string;
  readonly profiles: readonly string[];
  readonly featureVarianceAllowed: readonly { readonly package: string; readonly reason: string; readonly until: string }[];
  readonly cases: readonly { readonly name: string; readonly graphs: readonly CargoPackageUnitGraphV1[]; readonly expected: CargoUnitCensusRowV1[]; readonly drift: string[] }[];
  readonly crateTypeCases: readonly { readonly name: string; readonly graphs: readonly CargoPackageUnitGraphV1[]; readonly nonComponentCdylibs: string[] }[];
  readonly firstPartyFeaturesAllowed: readonly FirstPartyFeatureAllowanceV1[];
  readonly featureCases: readonly { readonly name: string; readonly graphs: readonly CargoPackageUnitGraphV1[]; readonly allowed: readonly string[]; readonly violations: string[] }[];
};

/** 📜️ One documented exception to L2 (no consumer-dependent first-party features). The list is empty by default; every row names why the feature
 * still exists and who removes it. `test-only` rows may appear only in test units, never in a release unit. */
export type FirstPartyFeatureAllowanceV1 = { readonly package: string; readonly feature: string; readonly classification: "test-only" | "peer-owned" | "peer-blocked"; readonly reason: string };

const repoRoot = resolve(import.meta.dir, "../../..");
const fixture = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🌳️unit-census/🔣️.json"), "utf8")) as FixtureV1;
const unification = { CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: "package" };

/** 🏷️ `name@version` of a Cargo package id (`…#name@version`, or `…/name#version` when the name equals the directory). */
function packageKey(id: string): string {
  const [source, fragment = ""] = id.split("#");
  return fragment.includes("@") ? fragment : `${source!.slice(source!.lastIndexOf("/") + 1)}@${fragment}`;
}

/** 🌳️ Counts, per compile unit key, the feature sets and closure signatures across every planned package's unit graph. A build-script
 * compile unit is keyed by the platform of the run that executes it, so host and target scripts of one package stay apart. */
export function cargoUnitCensusV1(graphs: readonly CargoPackageUnitGraphV1[]): CargoUnitCensusRowV1[] {
  const sets = new Map<string, Map<string, Set<string>>>(), signatures = new Map<string, Set<string>>();
  for (const graph of graphs) {
    const runs = new Map<number, Set<string>>(), memo = new Map<number, string>();
    graph.units.forEach((unit) => { if (unit.mode === "run-custom-build") for (const dependency of unit.dependencies) (runs.get(dependency.index) ?? runs.set(dependency.index, new Set()).get(dependency.index)!).add(unit.platform ?? "host"); });
    const signature = (index: number): string => {
      const known = memo.get(index);
      if (known) return known;
      const unit = graph.units[index]!;
      const value = createHash("sha256").update(JSON.stringify([unit.pkg_id, unit.target.name, unit.mode, unit.platform, unit.features, unit.dependencies.map((dependency) => signature(dependency.index)).sort()])).digest("hex");
      memo.set(index, value);
      return value;
    };
    graph.units.forEach((unit, index) => {
      for (const platform of runs.get(index) ?? [unit.platform ?? "host"]) {
        const key = `${packageKey(unit.pkg_id)} ${unit.target.name} ${unit.mode} ${platform}`, features = [...unit.features].sort().join(",");
        const byFeatures = sets.get(key) ?? sets.set(key, new Map()).get(key)!;
        (byFeatures.get(features) ?? byFeatures.set(features, new Set()).get(features)!).add(graph.package);
        (signatures.get(key) ?? signatures.set(key, new Set()).get(key)!).add(signature(index));
      }
    });
  }
  return [...sets.keys()].sort().map((unit) => ({ unit, featureSets: Object.fromEntries([...sets.get(unit)!.entries()].sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)).map(([features, packages]) => [features, [...packages].sort()])), signatures: signatures.get(unit)!.size }));
}

/** 🚦️ The unit keys compiled with more than one feature set, apart from the packages allowed to vary. */
export function cargoUnitFeatureDriftV1(rows: readonly CargoUnitCensusRowV1[], allowed: readonly string[]): string[] {
  return rows.filter((row) => Object.keys(row.featureSets).length > 1 && !allowed.includes(row.unit.split("@")[0]!)).map((row) => row.unit);
}

/** 🧩️ L5, library XOR component: the unit keys whose target links a cdylib without being a component — a cdylib unit that also builds a
 * library crate type, or that another unit depends on (a component is a cdylib-only root nothing links).
 * @see https://doc.rust-lang.org/reference/linkage.html */
export function cargoNonComponentCdylibsV1(graphs: readonly CargoPackageUnitGraphV1[]): string[] {
  const keys = new Set<string>();
  for (const graph of graphs) {
    const depended = new Set(graph.units.flatMap((unit) => unit.dependencies.map((dependency) => dependency.index)));
    graph.units.forEach((unit, index) => {
      if (unit.target.kind.includes("cdylib") && (unit.target.kind.length !== 1 || depended.has(index))) keys.add(`${packageKey(unit.pkg_id)} ${unit.target.name} ${unit.mode} ${unit.platform ?? "host"}`);
    });
  }
  return [...keys].sort();
}

/** 🚫️ L2, G3: every `package/feature` a first-party (path) unit is compiled with that the allow-list does not name, as `name@version feature`.
 * Registry and git packages keep their features; a first-party crate carries none. */
export function cargoFirstPartyFeatureViolationsV1(graphs: readonly CargoPackageUnitGraphV1[], allowed: ReadonlySet<string>): string[] {
  const violations = new Set<string>();
  for (const graph of graphs) for (const unit of graph.units) {
    if (!unit.pkg_id.startsWith("path+")) continue;
    const key = packageKey(unit.pkg_id);
    for (const feature of unit.features) if (!allowed.has(`${key.split("@")[0]}/${feature}`)) violations.add(`${key} ${feature}`);
  }
  return [...violations].sort();
}

/** 🔍️ Independent oracle for G3: every feature a first-party package of the workspace declares (implicit optional-dependency features included), from
 * `cargo metadata`, as `package/feature`. */
async function cargoMetadataFirstPartyFeatures(manifest: string, signal: AbortSignal): Promise<string[]> {
  const metadata = JSON.parse(await cargo(["metadata", "--locked", "--offline", "--format-version", "1", "--manifest-path", manifest], signal)) as { packages: { name: string; source: string | null; features: Record<string, string[]> }[] };
  return [...new Set(metadata.packages.filter((pkg) => pkg.source === null).flatMap((pkg) => Object.keys(pkg.features).map((feature) => `${pkg.name}/${feature}`)))].sort();
}

/** 🔍️ Independent oracle for L5: every workspace member whose library target declares `cdylib`, from `cargo metadata`, with its crate types. */
async function cargoMetadataCdylibPackages(manifest: string, signal: AbortSignal): Promise<Map<string, readonly string[]>> {
  const metadata = JSON.parse(await cargo(["metadata", "--locked", "--offline", "--no-deps", "--format-version", "1", "--manifest-path", manifest], signal)) as { packages: { name: string; targets: { kind: string[]; crate_types: string[] }[] }[] };
  return new Map(metadata.packages.flatMap((pkg) => pkg.targets.filter((target) => target.crate_types.includes("cdylib")).map((target) => [pkg.name, target.crate_types] as const)));
}

/** 🏃️ Runs one Cargo query with per-package feature unification and returns its standard output; cancellable through `signal`. */
function cargo(args: readonly string[], signal: AbortSignal): Promise<string> {
  return new Promise((accept, reject) => {
    const child = spawn("cargo", args, { cwd: repoRoot, env: { ...process.env, ...unification }, signal, stdio: ["ignore", "pipe", "pipe"] });
    const out: Buffer[] = [], err: Buffer[] = [];
    child.stdout.on("data", (chunk: Buffer) => out.push(chunk));
    child.stderr.on("data", (chunk: Buffer) => err.push(chunk));
    child.once("error", reject);
    child.once("close", (code) => (code === 0 ? accept(Buffer.concat(out).toString("utf8")) : reject(new Error(`cargo ${args.join(" ")} exited ${code}: ${Buffer.concat(err).toString("utf8").slice(-2000)}`))));
  });
}

/** 🗺️ Reads every planned package's unit graph for one profile and target, reporting progress per package. */
export async function readCargoPackageUnitGraphsV1(manifest: string, packages: readonly string[], target: string, profile: string, signal: AbortSignal): Promise<CargoPackageUnitGraphV1[]> {
  const graphs: CargoPackageUnitGraphV1[] = [];
  for (const [index, name] of packages.entries()) {
    const graph = JSON.parse(await cargo(["build", "--locked", "--offline", "--manifest-path", manifest, "-p", name, "--lib", "--target", target, "--profile", profile, "-Z", "unstable-options", "--unit-graph"], signal)) as { units: CargoUnitV1[] };
    graphs.push({ package: name, units: graph.units });
    console.log(`[unit-census] ${profile} ${index + 1}/${packages.length} ${name}: ${graph.units.length} units`);
  }
  return graphs;
}

/** 🔍️ Independent oracle: the feature sets `cargo tree` resolves per `name@version` library node, against the census's library units. */
async function cargoTreeLibraryFeatures(manifest: string, name: string, target: string, signal: AbortSignal): Promise<Map<string, Set<string>>> {
  const text = await cargo(["tree", "--locked", "--offline", "--manifest-path", manifest, "-p", name, "--target", target, "-e", "normal,build", "--prefix", "none", "-f", "{p}|{f}"], signal);
  const nodes = new Map<string, Set<string>>();
  for (const line of text.split("\n")) {
    const [node, features] = line.replace(/ \(\*\)$/u, "").split("|");
    if (features === undefined) continue;
    const [crate, version] = node!.trim().split(" ");
    const key = `${crate}@${version!.slice(1)}`;
    (nodes.get(key) ?? nodes.set(key, new Set()).get(key)!).add(features.split(",").filter(Boolean).sort().join(","));
  }
  return nodes;
}

function libraryFeatures(graph: CargoPackageUnitGraphV1): Map<string, Set<string>> {
  const nodes = new Map<string, Set<string>>();
  for (const unit of graph.units) {
    if (unit.mode !== "build" || unit.target.kind.includes("custom-build")) continue;
    const key = packageKey(unit.pkg_id);
    (nodes.get(key) ?? nodes.set(key, new Set()).get(key)!).add([...unit.features].sort().join(","));
  }
  return nodes;
}

const plain = (nodes: Map<string, Set<string>>): Record<string, string[]> => Object.fromEntries([...nodes.entries()].sort(([a], [b]) => (a < b ? -1 : 1)).map(([key, value]) => [key, [...value].sort()]));

test("the library XOR component law reproduces every language-neutral crate-type case", () => {
  for (const row of fixture.crateTypeCases) expect(cargoNonComponentCdylibsV1(row.graphs), row.name).toEqual(row.nonComponentCdylibs);
});

test("G3 reproduces every language-neutral first-party feature case", () => {
  for (const row of fixture.featureCases) expect(cargoFirstPartyFeatureViolationsV1(row.graphs, new Set(row.allowed)), row.name).toEqual(row.violations);
});

test("every G3 allowance is documented and unique", () => {
  const keys = fixture.firstPartyFeaturesAllowed.map((row) => `${row.package}/${row.feature}`);
  expect(new Set(keys).size, "duplicate allowance").toBe(keys.length);
  for (const row of fixture.firstPartyFeaturesAllowed) expect(row.reason.length, `${row.package}/${row.feature} needs a reason`).toBeGreaterThan(0);
});

test("the census reproduces every language-neutral unit graph case", () => {
  expect(fixture.schema).toBe("semio.hub.unit-census/v1");
  for (const row of fixture.cases) {
    const rows = cargoUnitCensusV1(row.graphs);
    expect(rows, row.name).toEqual(row.expected);
    expect(cargoUnitFeatureDriftV1(rows, []), row.name).toEqual(row.drift);
  }
});

test("every crate of the hub release closure compiles once per profile, apart from the documented feature variance", async () => {
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  try {
    const workspaces = componentWorkspacesV1(repoRoot, fixture.workspaces);
    expect(workspaces.map((workspace) => workspace.manifest)).toEqual(fixture.workspaces.map((workspace) => join(repoRoot, workspace, "Cargo.toml")));
    const allowed = fixture.featureVarianceAllowed.map((row) => row.package);
    const allowances = new Set(fixture.firstPartyFeaturesAllowed.map((row) => `${row.package}/${row.feature}`));
    const releaseAllowances = new Set(fixture.firstPartyFeaturesAllowed.filter((row) => row.classification !== "test-only").map((row) => `${row.package}/${row.feature}`));
    for (const workspace of workspaces) {
      const declared = await cargoMetadataFirstPartyFeatures(workspace.manifest, controller.signal);
      expect(declared.filter((key) => !allowances.has(key)), `G3 cargo metadata oracle: ${workspace.manifest} declares first-party features outside the allow-list`).toEqual([]);
      expect([...allowances].filter((key) => !declared.includes(key)), "G3: these allowances name no declared feature any more; drop them").toEqual([]);
      for (const profile of fixture.profiles) {
        const graphs = await readCargoPackageUnitGraphsV1(workspace.manifest, workspace.packages, fixture.target, profile, controller.signal);
        const rows = cargoUnitCensusV1(graphs);
        const variance = rows.filter((row) => Object.keys(row.featureSets).length > 1).map((row) => row.unit.split("@")[0]!);
        console.log(`[unit-census] ${profile}: ${rows.length} unit keys, ${rows.reduce((sum, row) => sum + row.signatures, 0)} compile units, feature variance in ${[...new Set(variance)].sort().join(", ") || "none"}`);
        expect(cargoUnitFeatureDriftV1(rows, allowed), `${workspace.manifest} ${profile}`).toEqual([]);
        for (const name of allowed) expect(variance, `${name} is allowed to vary but compiles once; drop it from featureVarianceAllowed`).toContain(name);
        expect(cargoNonComponentCdylibsV1(graphs), `${workspace.manifest} ${profile} links a cdylib outside a component crate`).toEqual([]);
        expect(cargoFirstPartyFeatureViolationsV1(graphs, releaseAllowances), `G3: ${workspace.manifest} ${profile} compiles a release unit with a first-party feature`).toEqual([]);
        if (profile !== fixture.profiles[0]) continue;
        const cdylibs = await cargoMetadataCdylibPackages(workspace.manifest, controller.signal);
        for (const name of workspace.packages) expect(cdylibs.get(name), `cargo metadata oracle: component ${name} is cdylib-only`).toEqual(["cdylib"]);
        for (const [name, types] of cdylibs) expect({ name, types, component: workspace.packages.includes(name) }, "cargo metadata oracle: only component crates are cdylibs").toEqual({ name, types: ["cdylib"], component: true });
        const linked = new Set(graphs.flatMap((graph) => graph.units.filter((unit) => unit.target.kind.includes("cdylib")).map((unit) => packageKey(unit.pkg_id).split("@")[0]!)));
        expect([...linked].sort(), "the unit graphs link exactly the planned component crates").toEqual([...workspace.packages].sort());
        for (const graph of graphs) expect(plain(libraryFeatures(graph)), `cargo tree oracle for ${graph.package}`).toEqual(plain(await cargoTreeLibraryFeatures(workspace.manifest, graph.package, fixture.target, controller.signal)));
      }
    }
  } finally {
    process.removeListener("SIGINT", cancel);
  }
}, 1_800_000);
