import { ephemeralMap } from "../../../../../../🔨️modules/🫧️transient/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import type { PackageMaturity } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";

/** 🎭️ Package "kind" declared by the ecosystem's role marker — see `readSemioMarker` and `taxonomy.roles`. */
export type PackageRole = "plugin" | "framework" | "product" | "hub" | "s-module" | "extension" | "tool" | "library" | "artifact" | "test";

/** 🌐️ Ecosystem a discovered package's manifest belongs to (`taxonomy.langs`). */
export type PackageLang = "🦀️rust" | "🟦️typescript" | "🟨️javascript" | "🐹️go" | "🐍️python" | "🔷️dotnet";

/** 🎯️ A registered render/build target owning `<target>/📦️packages/<lang>/`. */
export type PackageTarget = string;

/** 📦️ One package discovered under `<owner>/📦️packages/<lang>/`, where the owner may be a registered target. */
export interface DiscoveredPackage {
  readonly ownerRel: string;
  readonly lang: PackageLang;
  /** 🎯️ Set when `ownerRel` ends in `🎯️targets/<registered-target>`. */
  readonly target?: PackageTarget;
  /** 📁️ Repo-relative dir holding the manifest (i.e. `dirname(manifestPath)`). */
  readonly packageRel: string;
  readonly manifestPath: string;
  readonly role: PackageRole;
  readonly id: string;
  /** 🗺️ Declared state of the enclosing area (`taxonomy.areas`), `""` outside every declared area. */
  readonly area: string;
  /** 📈️ Derived state of this package's OWNER: `mixed` while the owner still carries forbidden-segment dirs or an owner-root entry file, else `clean`. */
  readonly maturity: PackageMaturity;
}

/** 🏠️ One owner (a dir carrying a `📦️packages` folder) with every package it ships plus its derived migration state. */
export interface DiscoveredOwner {
  readonly ownerRel: string;
  readonly area: string;
  readonly maturity: PackageMaturity;
  readonly langs: readonly PackageLang[];
  readonly targets: readonly PackageTarget[];
  readonly roles: readonly PackageRole[];
  readonly packages: readonly DiscoveredPackage[];
  /** 🔥️ Residual `⚡️implementations`/`⚡️implementation` dirs still inside this owner (burn-down counter — must decrease monotonically to 0). */
  readonly residualImplDirs: number;
  /** 🚪️ Shape V1 leftovers: entry files still sitting at the owner root instead of in `📦️packages/<lang>/`. */
  readonly entryFilesAtOwnerRoot: readonly string[];
}

/** ⚠️ A discovery-time problem `discoverPackages` does not fail on but must not stay silent about — see `discoverPackageProblems`. */
export interface DiscoveryProblem {
  readonly kind: "target-inside-package-boundary" | "target-without-manifest" | "manifest-without-marker" | "unknown-lang" | "unknown-role" | "packaging-violation" | "package-role-unresolved" | "package-implementation";
  readonly path: string;
  readonly message: string;
}

/** 🔥️ Migration burn-down snapshot: everything that still has to shrink to zero before the finalization flip, derived from disk on every call (no hand-maintained lists). */
export interface DiscoveryBurndown {
  readonly ownersTotal: number;
  readonly packagesTotal: number;
  readonly cleanOwners: number;
  readonly mixedOwners: readonly DiscoveredOwner[];
  /** 🔥️ Every forbidden-segment dir repo-wide, including those outside any migrated owner. */
  readonly implDirsTotal: number;
  readonly implDirsByArea: Readonly<Record<string, number>>;
  /** 🏷️ `📦️packages/<lang>/` manifests carrying no role marker — invisible to `discoverPackages` until they get one. */
  readonly unmarkedManifests: readonly { readonly path: string; readonly area: string }[];
  /** 📦️ Files inside a package dir that are neither packaging code nor its entry file (data/docs belong at the owner root under Shape V2). */
  readonly packagingViolations: readonly { readonly path: string; readonly ownerRel: string }[];
}

/** 🔍️ Extracts a dotted TOML table's body from manifest text (stops at the next `[…]` header) — simple line-scan, no TOML parser dependency (none is a repo dependency for the TS side; mirrors `hasSemioRole` in framework/os/plugin/registry script.ts). */
export function tomlTableBody(text: string, table: string): string | undefined {
  const header = `[${table}]`;
  const lines = text.split("\n");
  const start = lines.findIndex((line) => line.trim() === header);
  if (start === -1) return undefined;
  const body: string[] = [];
  for (const line of lines.slice(start + 1)) {
    if (line.trim().startsWith("[")) break;
    body.push(line);
  }
  return body.join("\n");
}

/** 🧩️ Reads the authored component protocol independently of package ownership. */
export function declaredComponentKind(text: string): "plugin" | "extension" | undefined {
  const body = tomlTableBody(text, "package.metadata.semio");
  if (body === undefined) return undefined;
  const declarations = body.split(/\r?\n/u).filter((line) => /^\s*component-kind\s*=/u.test(line));
  if (!declarations.length) return undefined;
  const kind = declarations.length === 1 ? /^\s*component-kind\s*=\s*"(plugin|extension)"\s*(?:#.*)?$/u.exec(declarations[0]!)?.[1] : undefined;
  if (kind !== "plugin" && kind !== "extension") throw new Error("Component kind must declare exactly one plugin or extension protocol");
  return kind;
}

/** 🔍️ Walks a dotted key path (`"metadata.semio"`, `"semio"`) into a parsed JSON manifest. */
export function jsonTable(value: unknown, table: string): Record<string, unknown> | undefined {
  let current: unknown = value;
  for (const key of table.split(".")) {
    if (typeof current !== "object" || current === null) return undefined;
    current = (current as Record<string, unknown>)[key];
  }
  return typeof current === "object" && current !== null ? (current as Record<string, unknown>) : undefined;
}

/** 🏷️ Package name from a Cargo.toml's `[package] name = "…"` line. */
function rustPackageName(text: string): string | undefined {
  return text.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
}

/** 🏷️ Resolves role metadata from supplied bytes, including transaction preimage views. */
export function parseSemioMarker(content: string, lang: PackageLang, taxonomy: Taxonomy): { role: PackageRole; id?: string } | undefined {
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec) return undefined;
  let role: string | undefined;
  let id: string | undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(content, spec.table);
    if (!body) return undefined;
    role = body.match(new RegExp(`^${spec.roleKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
    id = body.match(new RegExp(`^${spec.idKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
  } else {
    let parsed: unknown;
    try {
      parsed = JSON.parse(content);
    } catch {
      return undefined;
    }
    const table = jsonTable(parsed, spec.table);
    if (!table) return undefined;
    role = typeof table[spec.roleKey] === "string" ? (table[spec.roleKey] as string) : undefined;
    id = typeof table[spec.idKey] === "string" ? (table[spec.idKey] as string) : undefined;
  }
  if (!role) return undefined;
  return id ? { role: role as PackageRole, id } : { role: role as PackageRole };
}

/** 🔧️ Minimal TOML table-body decoder: plain string values (`key = "..."`) and flat string arrays
 * (`key = ["a", "b"]`) — sufficient for opt-in sub-tables (see `readSemioMarkerSubTable`); deliberately
 * NOT a general TOML parser (no numbers/bools/nested-inline-tables/multiline strings — reach for a real
 * TOML dependency if a future consumer needs more than this). */
export function tomlTableValues(body: string): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  for (const rawLine of body.split("\n")) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) continue;
    const arrayMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\[([^\]]*)\]\s*$/);
    if (arrayMatch) {
      const [, key, values] = arrayMatch;
      if (key === undefined || values === undefined) throw new Error("TOML array captures required");
      result[key] = [...values.matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((match) => { const value = match[1]; if (value === undefined) throw new Error("TOML string capture required"); return value; });
      continue;
    }
    const scalarMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$/);
    if (scalarMatch) { const [, key, value] = scalarMatch; if (key === undefined || value === undefined) throw new Error("TOML scalar captures required"); result[key] = value; }
  }
  return result;
}

const DISCOVERY_SKIP_DIRS = new Set(["node_modules", "target", "dist", "📤️dist", ".git", ".🧬semio", "🤖️generated", "🗑️generated", "🔌️plugin-modules", "pkg", "storybook-static", "temp", ".venv", "coverage", "__pycache__", "client", "client_bin"]);

export const CARGO_TARGET_DIR_PATTERN = /^target(?:-[a-z0-9]+)*$/u;

/** 🎯️ Cargo target roots (`target`, `target-<slug>`) are build output that concurrent lanes create and prune mid-walk; discovery never enters them.
 *
 * `🗑️generated` is tool and test output by AGENTS.md's own rule and is gitignored (`.gitignore:22`), so
 * it is untracked by construction and can never hold authored content. The same set is already
 * declared opaque in `ignoredPathPatterns` beside `target`/`dist`/`node_modules`, and
 * `🖨️describe/🧾️source-epoch/🟦️.ts:165` refuses it by name; omitting it here let the package-boundary
 * walk judge one hub test run's leftovers as 595 authored package-purity breaches. */
export function isDiscoverySkipDirectory(name: string): boolean {
  return DISCOVERY_SKIP_DIRS.has(name) || CARGO_TARGET_DIR_PATTERN.test(name);
}

/** 🔁️ Retries a filesystem syscall interrupted by `EINTR`. Signal delivery can interrupt `scandir`/`lstat` mid-flight whenever many processes run concurrently against this repo, and the raw `node:fs` sync wrappers surface that as a thrown `EINTR` rather than retrying it. Retrying is the only correct response: the call had not failed, it had not yet run. @see https://man7.org/linux/man-pages/man7/signal.7.html */
export function retryOnEintr<T>(operation: () => T): T {
  for (let attempt = 0; ; attempt += 1) {
    try {
      return operation();
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EINTR" || attempt >= 64) throw error;
    }
  }
}

/** 📁️ `readdirSync(dir, { withFileTypes: true })`, defaulting to `[]` for an unreadable/missing dir — a helper (rather than an explicit `ReturnType<typeof readdirSync>` annotation) so the `Dirent<string>` element type infers unambiguously from this specific overload. */
/** 🌪️ Enumerates a directory that a concurrent rename may remove between discovery and enumeration; a vanished directory has no children. */
export function readdirVanishing(absDir: string): string[] {
  try {
    return retryOnEintr(() => readdirSync(absDir));
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
    return [];
  }
}

export function readdirSafe(absDir: string) {
  try {
    return retryOnEintr(() => readdirSync(absDir, { withFileTypes: true }));
  } catch {
    return [];
  }
}

/** 🔤️ Drops every non-ASCII codepoint (emoji + variation selectors), e.g. `"✒️writer"` -> `"writer"` — mirrors `policyStripEmoji` in root script.ts. */
export function stripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

/** 🆔️ Falls back to the manifest's own package name, else an emoji-stripped dash-joined owner path, when `readSemioMarker` found no explicit `id`. */
export function fallbackPackageId(manifestPath: string, lang: PackageLang, ownerRel: string, content?: string): string {
  try {
    if (lang === "🦀️rust") {
      const name = rustPackageName(content ?? readFileSync(manifestPath, "utf8"));
      if (name) return name;
    } else {
      const name = (JSON.parse(content ?? readFileSync(manifestPath, "utf8")) as { name?: string }).name;
      if (name) return name;
    }
  } catch {
    /* fall through to path-derived id */
  }
  return ownerRel
    .replaceAll("\\", "/")
    .split("/")
    .map(stripEmoji)
    .filter(Boolean)
    .join("-");
}

export interface OwnerAccumulator {
  ownerRel: string;
  area: string;
  packages: DiscoveredPackage[];
  residualImplDirs: number;
  entryFilesAtOwnerRoot: string[];
}

export interface DiscoveryScan {
  readonly packages: readonly DiscoveredPackage[];
  readonly owners: readonly DiscoveredOwner[];
  readonly problems: readonly DiscoveryProblem[];
  readonly burndown: DiscoveryBurndown;
}

export const scanCache = ephemeralMap<string, DiscoveryScan>("framework.products.repo.modules.lib.discovery.component.ts.scanCache");
