/** 🗃️ Oracle registry records
 *
 * Oracle entries, linked packages, adapter registration and the no-oracle decisions of the oracle registry.
 */

//#region 🔌️Adapters
import { type ComparisonProfile, type Implementation, type PlatformId, type EngineFamily } from "../../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { type DiscoveredCase } from "../🧭️case-discovery/🟦️.ts";
import { type NativeSecondImplementationEvidence, type OracleKind } from "../📑️mutation-manifest/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📇️Registry
/** 📇️ One approved third-party reference implementation, test-only by construction. */
export type OracleEntry = Readonly<{
  id: string;
  /** 🎭️ What this reference IS. Only a `QUALIFYING_ORACLE_KINDS` member can discharge a mutation's external-oracle requirement; everything else is a required supplement. */
  kind?: OracleKind;
  ecosystem: string;
  package: string;
  packages?: readonly OracleLinkedPackage[];
  version?: string;
  lockDigest?: string;
  source?: { repository?: string; commit?: string; license?: string };
  /** ⚙️ The kernel this reference actually sits on. Independence is accounted per FAMILY: two wrappers around one kernel are one oracle. */
  engine?: EngineFamily;
  capabilities: readonly string[];
  comparisonProfiles: readonly ComparisonProfile[];
  license: string;
  testOnly: true;
  productionReachable?: boolean;
  networkDuringExecution?: boolean;
  platforms?: readonly PlatformId[];
  homepage?: string;
  rationale?: string;
  hostPath?: string;
  /** 🧭️ The case adapter language that drives a `native` reference (a third-party CLI spawned from that adapter's oracle handler). */
  hostImplementation?: Implementation;
  /** 🔧️ The commands a `third-party-cli` reference runs; a command missing from PATH makes the case's oracle role unavailable, recorded with the command it lacked. */
  executables?: readonly string[];
  productionDebt?: { reachableFrom: readonly string[]; owner: string; plan: string };
  /** 🌱️ Required, and checked by `nativeSecondImplementationBreaches`, when `kind` is `verified-native-second-implementation` — absent otherwise. */
  nativeSecondImplementation?: NativeSecondImplementationEvidence;
}>;

/**
 * 🧩️ One further third-party package a composed reference links beyond its primary one, pinned and
 * licensed in its own right. A secondary package that inherited the entry's version would be
 * recorded under a version it is not at, which is a worse record than none.
 */
export type OracleLinkedPackage = Readonly<{ package: string; version: string; license: string; homepage?: string; role: string }>;

/**
 * 🧩️ Every third-party package one registered oracle actually links, each with its own pin. Most name
 * a single package; a composed reference names several, because some formats have no single credible
 * crate — an OOXML container needs an archive reader AND an XML reader, and a spreadsheet needs a
 * reader AND a writer. Declaring only one of them would leave the other unclassified, which is
 * precisely what the dependency ratchet exists to prevent.
 */
export function oracleLinkedPackages(entry: OracleEntry): OracleLinkedPackage[] {
  const primary: OracleLinkedPackage = { package: entry.package, version: entry.version ?? "*", license: entry.license, homepage: entry.homepage, role: "primary reference" };
  const byName = new Map<string, OracleLinkedPackage>();
  for (const linked of [primary, ...(entry.packages ?? [])]) if (linked.package.length > 0 && !byName.has(linked.package)) byName.set(linked.package, linked);
  return [...byName.values()];
}

/** 🧩️ The names alone of every package one registered oracle links — what the import probe scans for. */
export function oraclePackages(entry: OracleEntry): string[] {
  return oracleLinkedPackages(entry).map((linked) => linked.package);
}

/** 🎯️ How each language's adapter registers a SUBJECT handler (`Adapter::subject`, `subject:` in a TS scenario, …). */
export const ADAPTER_SUBJECT_REGISTRATION: Readonly<Record<Implementation, RegExp>> = {
  rust: /\.subject\(/,
  typescript: /\bsubject\s*:/,
  go: /\.Subject\(/,
  python: /\.subject\(/,
  dotnet: /\.Subject\(/,
};

/** 🎯️ Whether the case's adapter in `implementation` registers any subject handler — read from its source, like its entry point. */
export function adapterRegistersSubject(repoRoot: string, discovered: DiscoveredCase, implementation: Implementation): boolean {
  const path = discovered.adapters[implementation];
  return path !== undefined && ADAPTER_SUBJECT_REGISTRATION[implementation].test(readFileSync(join(repoRoot, path), "utf8"));
}

/**
 * 🧭️ The case adapter language an oracle runs in: a `native` reference is driven from the adapter its
 * entry names in `hostImplementation`, a `javascript` package from the TypeScript adapter, anything
 * else from the adapter of its own ecosystem. `undefined` when a native reference names no host.
 */
export function oracleImplementation(entry: OracleEntry): Implementation | undefined {
  return (entry.ecosystem === "native" ? entry.hostImplementation : entry.ecosystem === "javascript" ? "typescript" : entry.ecosystem) as Implementation | undefined;
}

/**
 * 🚪️ The one entry point each language host loads an adapter through — the signature the generated
 * entrypoint calls (`🖥️host/🏗️materialization/🟦️.ts`) or the attribute the interpreted host reads
 * (`🖥️host/🐍️.py`, `🖥️host/🟦️.ts`). A file without it is a standalone script whose rows never
 * execute in either role.
 */
export const ADAPTER_ENTRY_POINTS: Readonly<Record<Implementation, { signature: string; pattern: RegExp }>> = {
  rust: { signature: "`pub fn adapter() -> Adapter`", pattern: /^pub fn adapter\(\)\s*->\s*(?:[A-Za-z_][\w]*::)*Adapter\b/m },
  typescript: { signature: "`export default defineTestAdapter({ implementation: \"typescript\", … })`", pattern: /^export default defineTestAdapter\(/m },
  go: { signature: "`func Adapter() *host.Adapter`", pattern: /^func Adapter\(\)\s*\*host\.Adapter\b/m },
  python: { signature: "`def adapter() -> Adapter`", pattern: /^def adapter\(\)/m },
  dotnet: { signature: "`static class Adapter { static Semio.Repo.Test.Adapter Create() }`", pattern: /\bstatic\s+Semio\.Repo\.Test\.Adapter\s+Create\(\)/ },
};

/** 📇️ A recorded decision that a capability legitimately has no credible reference implementation. */
export type NoOracleDecision = Readonly<{ id: string; capabilities: readonly string[]; rationale: string; substitutes: readonly string[]; coversMutations?: boolean; referenceSurvey?: ReferenceSurvey }>;

/** 🔍️ The recorded negative search behind a justified mutation reference gap. */
export type ReferenceSurvey = Readonly<{
  ecosystemsSearched: readonly string[];
  candidatesConsidered: readonly Readonly<{ package: string; ecosystem: string; verdict: "cannot-express-the-mutation" | "format-defined-by-this-repository" | "no-open-implementation" | "shares-the-production-engine"; reason: string }>[];
  whyNoneQualifies: string;
}>;

/** ⚖️ Whether a decision justifies a mutation reference gap: it claims mutations explicitly and carries a surveyed negative search. */
export function isJustifiedMutationDecision(decision: NoOracleDecision): boolean {
  const survey = decision.referenceSurvey;
  return decision.coversMutations === true && survey !== undefined && survey.ecosystemsSearched.length > 0 && survey.candidatesConsidered.length > 0 && survey.whyNoneQualifies.trim().length >= 40;
}

/**
 * 🧩️ A package an owner contributes so its adapters can reach their reference libraries.
 *
 * `path` is what separates the two kinds. WITH a path the entry names LOCAL source linked by path —
 * an in-repo crate, module or workspace package — which is not an external dependency and is
 * provisioned by generating a manifest that points at it. WITHOUT a path the entry names an EXTERNAL
 * distribution the generated host must obtain from that ecosystem's own registry, which is exactly
 * what `externalOracleHostPackages` hands to the dependency classification so a reference library
 * can never reach a host through a side door.
 *
 * `module` is the import name when it differs from the distribution name (`Pillow` → `PIL`);
 * `version` pins the distribution.
 */
export type OracleHostPackage = Readonly<{ implementation: Implementation; package: string; path?: string; version?: string; module?: string; rationale?: string }>;

/** 🧱️ A local package the generated SUBJECT host links beside the owner's own package — a subject spanning several packages. */
export type SubjectHostPackage = Readonly<{ implementation: Implementation; package: string; path: string; rationale: string }>;

/** 🧩️ The name a host imports a contributed package by — the declared module, else the package name. */
export function oracleHostModule(entry: OracleHostPackage): string {
  return entry.module ?? entry.package.replace(/-/g, "_");
}
//#endregion 📇️Registry
