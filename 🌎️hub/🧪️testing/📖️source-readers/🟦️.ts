/** 🧭️ Hub Rust source readers shared by the hub gates: the `os-hub` binary closure, the test package list, the directory backend and trusted-catalog source trees.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { type ExactCargoLawGroup } from "@semio-tech/process/cargo-laws";
import { listModuleTree, readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

/** 🌳️ The crate roots that carry the `os-hub` binary's code, in dependency order; the binary's own root closes the list. */
const HUB_BIN_CRATE_ROOTS = ["⚙️runtime", "📒️ledgers", "🧱️models", "🧠️state", "🛰️session-routes", "📨️document-routes", "🛣️routes"] as const;

/** 🌳️ The module trees of every crate that carries the `os-hub` binary's code, concatenated in dependency order. */
export function readHubBinSource(repoRoot: string): string {
  return [...HUB_BIN_CRATE_ROOTS.map((dir) => join(repoRoot, "🌎️hub/🏗️bootstrap", dir, "🦀️.rs")), join(repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs")].map((root) => readModuleTreeSource(root)).join("\n");
}

/** 🧪️ Every hub package that owns tests: the `os-hub` verbs that run the whole suite select these. */
export const HUB_TEST_PACKAGES: readonly string[] = [
  "semio-hub",
  "semio-hub-artifact-authority",
  "semio-hub-artifact-checkin",
  "semio-hub-artifact-creation",
  "semio-hub-auth",
  "semio-hub-boot-laws",
  "semio-hub-component-codec-laws",
  "semio-hub-directory",
  "semio-hub-directory-backend-laws",
  "semio-hub-directory-laws",
  "semio-hub-directory-neo4j",
  "semio-hub-directory-postgres",
  "semio-hub-directory-route-laws",
  "semio-hub-directory-service",
  "semio-hub-document-routes",
  "semio-hub-hostile-input-laws",
  "semio-hub-inference-catalog",
  "semio-hub-inference-contract-laws",
  "semio-hub-inference-laws",
  "semio-hub-inference-ledger",
  "semio-hub-lag-rebootstrap",
  "semio-hub-local-bootstrap",
  "semio-hub-native-openable",
  "semio-hub-observability",
  "semio-hub-open-plan-laws",
  "semio-hub-presence-route-laws",
  "semio-hub-refusal",
  "semio-hub-route-models",
  "semio-hub-runtime",
  "semio-hub-session-route-laws",
  "semio-hub-session-routes",
  "semio-hub-stores",
  "semio-hub-trusted-catalog-contract",
  "semio-hub-trusted-catalog-contract-laws",
  "semio-hub-trusted-catalog-laws",
];

/** 🗺️ One exact-law group per owning crate for laws written as `[package, law]` pairs, in order of first appearance; `--features integration-fixtures` reaches only the crates that declare it. */
/** 📋️ Requires an existing authored Hub datum before consuming its fields. */
export function requireHubDatum<T>(value:T|undefined,label:string):T{if(value===undefined)throw new Error("Required Hub datum is missing: "+label);return value;}

/** 🔬️ Loads a real installed test oracle behind a checked function boundary. */
export async function hubOracleFunction(specifier:string):Promise<(...arguments_:unknown[])=>unknown>{
  const module:unknown=await import(specifier);
  if(module===null||typeof module!=="object"||!("default" in module)||typeof module.default!=="function")throw Error("Hub oracle module has no default function");
  const oracle=module.default;
  return (...arguments_)=>Reflect.apply(oracle,undefined,arguments_);
}
/** 🔢️ Validates the numeric result produced by a third-party oracle. */
export function hubOracleNumber(value:unknown):number{
  if(typeof value!=="number"||!Number.isFinite(value))throw Error("Hub oracle result must be a finite number");
  return value;
}
export function hubLawGroups(laws: ReadonlyArray<readonly [string, string]>, cargoArgs: readonly string[] = []): ExactCargoLawGroup[] {
  const owners = new Map<string, string[]>();
  for (const [owner, law] of laws) owners.set(owner, [...(owners.get(owner) ?? []), law]);
  const fixturePackages: ReadonlySet<string> = new Set(["semio-hub", "semio-hub-boot-laws", "semio-hub-component-codec-laws", "semio-hub-hostile-input-laws", "semio-hub-inference", "semio-hub-inference-contract-laws", "semio-hub-inference-runtime", "semio-hub-trusted-catalog", "semio-hub-trusted-catalog-contract", "semio-hub-trusted-catalog-laws"]);
  return [...owners].map(([owner, ls]) => {
    const feature = cargoArgs.indexOf("--features");
    const args = feature >= 0 && cargoArgs[feature + 1] === "integration-fixtures" && !fixturePackages.has(owner) ? cargoArgs.filter((_, index) => index !== feature && index !== feature + 1) : cargoArgs;
    return { package: owner, target: owner === "semio-hub" ? { kind: "bin" as const, name: "os-hub" } : { kind: "lib" as const }, ...(args.length ? { cargoArgs: args } : {}), laws: ls };
  });
}

/** 🔏️ The module trees of the trusted-catalog crate root and of the contract and descriptor roots it was carved into, each root followed by the children it mounts. */
export function readTrustedCatalogSource(repoRoot: string): string {
  return ["🦀️.rs", "📜️contract/🦀️.rs", "🧾️descriptor/🦀️.rs"].map((file) => readModuleTreeSource(join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog", file))).join("\n");
}

/** 🏃️ The inference runtime module trees together with the committer and contract crates it was carved into. */
export function readHubInferenceRuntimeSource(repoRoot: string): string {
  return ["🦀️.rs", "🔐️committer/🦀️.rs", "📜️contract/🦀️.rs"].map((file) => readModuleTreeSource(join(repoRoot, "🌎️hub/💡️inference/🏃️runtime", file))).join("\n");
}

/** 🗄️ One directory backend's production source the way the gates read it: the head of its root file (imports, the effect macro and the section mounts, without the testkit mount), then every section child in mount order with the section-only `pub(super)` visibility removed, and for neo4j the row decoders it was carved into. Every section child implements its capability traits directly, so each method reads once, exactly as it did in the unsplit file. */
export function readDirectoryBackendSource(repoRoot: string, backend: string): string {
  const [root, ...children] = listModuleTree(join(repoRoot, "🌎️hub/📇️directory", backend, "🦀️.rs")).map((file) => readFileSync(file, "utf8"));
  const rows = backend === "🌐️neo4j" ? [readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🧰️rows/🦀️.rs"), "utf8")] : [];
  const testkit = root!.indexOf('\n#[cfg(any(test, feature = "testkit"))]');
  return [testkit < 0 ? root! : root!.slice(0, testkit), ...children.map((text) => text.replaceAll("pub(super) ", "")), ...rows].join("\n");
}

/** 🌐️ The neo4j directory backend together with the row decoders it was carved into. */
export function readNeo4jDirectorySource(repoRoot: string): string {
  return readDirectoryBackendSource(repoRoot, "🌐️neo4j");
}

/** 🧪️ The shared fixtures and the unit laws of the `os-hub` route laws crates, concatenated. */
export function readHubBinTestSource(repoRoot: string): string {
  const base = join(repoRoot, "🌎️hub/🧪️tests/🔬️bin-unit");
  return ["🧰️testkit/🦀️.rs", "🚀️boot/🔬️unit/🦀️.rs", "🧭️open-plan/🔬️unit/🦀️.rs", "📇️directory/🔬️unit/🦀️.rs", "👥️presence/🔬️unit/🦀️.rs", "🔐️session/🔬️unit/🦀️.rs", "🎲️hostile-input/🔬️unit/🦀️.rs"].map((file) => readModuleTreeSource(join(base, file))).join("\n");
}

/** 🦀️ Every Rust file of one module, so a law stays findable wherever the test layout puts it —
 * production types and `#[cfg(test)]` law names are asserted against the module, not against a file. */
export function moduleRustSource(root: string): string {
  const files: string[] = [];
  const pending = [root];
  while (pending.length > 0) {
    const directory = pending.pop()!;
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (entry.isDirectory()) pending.push(join(directory, entry.name));
      else if (entry.name.endsWith(".rs")) files.push(join(directory, entry.name));
    }
  }
  return files.sort().map((file) => readFileSync(file, "utf8")).join("\n");
}
