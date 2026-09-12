import { builtinModules } from "node:module";
import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, extname, join } from "node:path";
import { runProbe } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { canonicalFilenameForKind } from "../../🔍️discovery/🟦️.ts";

const DEPENDENCY_SKIP_DIRS = new Set(["compose", "node_modules", ".git", ".🧬semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);

/** 📖️ Reads one repository-relative dependency input without racing a concurrent removal. */
export function dependencyReadFileSafe(repoRoot: string, ...parts: string[]): string {
  try { return readFileSync(join(repoRoot, ...parts), "utf8"); } catch { return ""; }
}

function dependencyDiscoverNamedFiles(repoRoot: string, filename: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    let entries: ReturnType<typeof readdirSync>;
    try { entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true }); } catch { return; }
    for (const entry of entries) {
      const child = relDir ? `${relDir}/${entry.name}` : entry.name;
      if (entry.isDirectory()) { if (!DEPENDENCY_SKIP_DIRS.has(entry.name)) walk(child); }
      else if (entry.name === filename) found.push(child);
    }
  };
  walk("");
  return found.sort();
}

export function dependencyDiscoverCargoTomlFiles(repoRoot: string): string[] { return dependencyDiscoverNamedFiles(repoRoot, "Cargo.toml"); }
export function dependencyDiscoverScriptTsFiles(repoRoot: string): string[] { return dependencyDiscoverNamedFiles(repoRoot, "📜️script.ts"); }

function dependencyTaxonomy(repoRoot: string): any {
  return JSON.parse(dependencyReadFileSafe(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
}

/** 🔒️Ecosystems the dependency freeze tracks — all five the repository actually ships. */
export type DependencyEcosystem = "rust" | "js" | "go" | "python" | "dotnet";
export const DEPENDENCY_ECOSYSTEMS: readonly DependencyEcosystem[] = ["rust", "js", "go", "python", "dotnet"];
/**
 * 🔒️Which phase a dependency is pulled in for. A dependency can serve more than one, so this is a
 * set per baseline entry. The two `production-*` classes are the only ones the purity gate cares
 * about; the target final state is zero of them. `test-oracle` is reserved for packages an approved
 * entry in the oracle registry claims — nothing else may enter under that class.
 */
export type DependencyKind = "production-runtime" | "production-build" | "repository-tooling" | "test-runner" | "test-oracle";

/** 🔒️Maps a manifest section's intent onto the phase vocabulary above. */
function dependencyKindOf(intent: "runtime" | "build" | "test" | "tooling"): DependencyKind {
  switch (intent) {
    case "runtime":
      return "production-runtime";
    case "build":
      return "production-build";
    case "test":
      return "test-runner";
    default:
      return "repository-tooling";
  }
}

/** 🔒️One third-party dependency's baseline record — the unit the freeze ratchet compares by `${ecosystem}:${name}` identity (version excluded from identity so routine patch bumps don't trip the gate; recorded for information only). */
export type DependencyDeclaration = { user: string; version: string; kind: DependencyKind };
export type DependencyBaselineEntry = { ecosystem: DependencyEcosystem; name: string; version: string; kinds: DependencyKind[]; users: string[]; productionReachable: boolean; declarations?: DependencyDeclaration[]; oracleIds?: string[]; oracleConflictUsers?: string[] };

/** 🔒️The committed freeze baseline file's shape — `🔒️dependencies.json` at the repo root. */
type DependencyBaseline = { schemaVersion: number; generatedAt: string; commit: string; entries: DependencyBaselineEntry[] };

/** 🔒️Repo-relative path of the committed dependency-freeze baseline. Root-level, alongside `📋️project.json`/`📜️script.ts`/`🧪️tests/🟦️.ts` — no existing convention for a repo-wide *hand-ratcheted* generated inventory exists yet (the `🤖️generated/` folders next to owning modules are build-regenerated and gitignored — see `.gitignore` — the opposite of what a freeze baseline needs). */
export const DEPENDENCY_BASELINE_REL_PATH = "🔒️dependencies.json";

/** 🔒️Rust crate name prefixes/exact names treated as first-party even without a `path =` key (defensive fallback — `path =` is the primary signal). */
function dependencyIsInternalRustName(name: string): boolean {
  return name.startsWith("semio-") || name.startsWith("semio_") || name === "ports" || /^db(?:_[a-z0-9]+)*$/.test(name);
}

type DependencyRustEntry = { name: string; version: string; kind: DependencyKind; internal: boolean };

/** 🔒️Parses `[workspace.dependencies]` from the root `Cargo.toml` into a name → {path?, version?} map, used to resolve `name.workspace = true` / `name = { workspace = true }` references in member manifests. */
function dependencyParseWorkspaceDeps(repoRoot: string): Map<string, { path?: string; version?: string }> {
  const map = new Map<string, { path?: string; version?: string }>();
  const content = dependencyReadFileSafe(repoRoot, "Cargo.toml");
  const lines = content.split(/\r?\n/);
  let inSection = false;
  for (let i = 0; i < lines.length; i += 1) {
    const trimmed = lines[i]!.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    if (trimmed.startsWith("[[")) {
      inSection = false;
      continue;
    }
    const sectionMatch = trimmed.match(/^\[([^\]]+)\]$/);
    if (sectionMatch) {
      inSection = sectionMatch[1] === "workspace.dependencies";
      continue;
    }
    if (!inSection) continue;
    const kv = trimmed.match(/^["']?([A-Za-z0-9_-]+)["']?\s*=\s*(.*)$/);
    if (!kv) continue;
    const name = kv[1]!;
    let full = kv[2]!;
    if (full.trim().startsWith("{")) {
      let depth = (full.match(/\{/g) ?? []).length - (full.match(/\}/g) ?? []).length;
      let j = i;
      while (depth > 0 && j + 1 < lines.length) {
        j += 1;
        full += ` ${lines[j]}`;
        depth += (lines[j]!.match(/\{/g) ?? []).length - (lines[j]!.match(/\}/g) ?? []).length;
      }
      i = j;
    }
    const pathMatch = full.match(/\bpath\s*=\s*"([^"]+)"/);
    const versionMatch = full.match(/(?:^|[{,]\s*)version\s*=\s*"([^"]+)"/) ?? full.match(/^"([^"]+)"/);
    map.set(name, { path: pathMatch?.[1], version: versionMatch?.[1] });
  }
  return map;
}

const DEPENDENCY_CARGO_SECTION_RE = /(?:^|\.)(dependencies|dev-dependencies|build-dependencies)$/;

/** 🔒️True when a `Cargo.toml`'s `[lib]` table sets `proc-macro = true` — such a crate's `[dependencies]` are compiler plugins linked into the compiler at build time, never into the target binary. */
function dependencyCargoTomlIsProcMacro(content: string): boolean {
  let inLib = false;
  for (const raw of content.split(/\r?\n/)) {
    const trimmed = raw.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const sectionMatch = trimmed.match(/^\[([^\]]+)\]$/);
    if (sectionMatch) {
      inLib = sectionMatch[1] === "lib";
      continue;
    }
    if (inLib && /^proc-macro\s*=\s*true\s*$/.test(trimmed)) return true;
  }
  return false;
}

/** 🔒️Parses every `[dependencies]`/`[dev-dependencies]`/`[build-dependencies]` table (including `[target.'cfg(...)'.…]` variants) in one `Cargo.toml`, resolving `workspace = true` refs against `workspaceDeps`. */
function dependencyParseCargoToml(repoRoot: string, relPath: string, workspaceDeps: Map<string, { path?: string; version?: string }>): DependencyRustEntry[] {
  const content = dependencyReadFileSafe(repoRoot, relPath);
  if (!content) return [];
  const lines = content.split(/\r?\n/);
  const results: DependencyRustEntry[] = [];
  type CargoDepSection = "dependencies" | "dev-dependencies" | "build-dependencies";
  let currentSection: CargoDepSection | null = null;
  const isProcMacro = dependencyCargoTomlIsProcMacro(content);
  const kindFor = (section: string): DependencyKind => dependencyKindOf(section === "dev-dependencies" ? "test" : section === "build-dependencies" || (section === "dependencies" && isProcMacro) ? "build" : "runtime");

  for (let i = 0; i < lines.length; i += 1) {
    const trimmed = lines[i]!.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    if (trimmed.startsWith("[[")) {
      currentSection = null;
      continue;
    }
    const sectionMatch = trimmed.match(/^\[([^\]]+)\]$/);
    if (sectionMatch) {
      const depMatch = sectionMatch[1] === "workspace.dependencies" ? undefined : (sectionMatch[1]!.match(DEPENDENCY_CARGO_SECTION_RE)?.[1] as CargoDepSection | undefined);
      currentSection = depMatch ?? null;
      continue;
    }
    if (!currentSection) continue;

    // `name.workspace = true` dotted shorthand — must be checked before the generic key=value parse below (its "name" would otherwise swallow the ".workspace" suffix).
    const dottedWorkspace = trimmed.match(/^["']?([A-Za-z0-9_-]+)["']?\.workspace\s*=\s*true\s*$/);
    if (dottedWorkspace) {
      const name = dottedWorkspace[1]!;
      const ws = workspaceDeps.get(name);
      results.push({ name, version: ws?.version ?? "*", kind: kindFor(currentSection), internal: Boolean(ws?.path) || dependencyIsInternalRustName(name) });
      continue;
    }

    const kv = trimmed.match(/^["']?([A-Za-z0-9_-]+)["']?\s*=\s*(.*)$/);
    if (!kv) continue;
    const name = kv[1]!;
    let full = kv[2]!;
    if (full.trim().startsWith("{")) {
      let depth = (full.match(/\{/g) ?? []).length - (full.match(/\}/g) ?? []).length;
      let j = i;
      while (depth > 0 && j + 1 < lines.length) {
        j += 1;
        full += ` ${lines[j]}`;
        depth += (lines[j]!.match(/\{/g) ?? []).length - (lines[j]!.match(/\}/g) ?? []).length;
      }
      i = j;
    }
    const hasPath = /\bpath\s*=\s*"/.test(full);
    const isWorkspaceRef = /\bworkspace\s*=\s*true/.test(full);
    let version = (full.match(/(?:^|[{,]\s*)version\s*=\s*"([^"]+)"/) ?? full.match(/^"([^"]+)"/))?.[1] ?? "*";
    let internal = hasPath || dependencyIsInternalRustName(name);
    if (isWorkspaceRef) {
      const ws = workspaceDeps.get(name);
      if (ws?.path) internal = true;
      if (ws?.version) version = ws.version;
    }
    results.push({ name, version, kind: kindFor(currentSection), internal });
  }
  return results;
}

type DependencyJsEntry = { name: string; version: string; kind: DependencyKind; internal: boolean };

type DependencyJsParityEvidence = { file: string; line: number; kind: "config" | "import" | "script" };
type DependencyJsParityRow = { dependency: string; manifest: string; scope: string; evidence: DependencyJsParityEvidence[] };
type DependencyJsParityImport = { dependency: string; file: string; line: number; manifest: string };
type DependencyJsManifestSection = "dependencies" | "devDependencies" | "optionalDependencies" | "peerDependencies";
export type DependencyJsLockMismatchKind = "invalid-lockfile" | "missing-in-lock" | "stale-in-lock" | "version-mismatch" | "workspace-missing";
type DependencyJsLockMismatch = { dependency?: string; kind: DependencyJsLockMismatchKind; lockVersion?: string; manifest: string; manifestVersion?: string; section?: DependencyJsManifestSection };
export type DependencyJsParityReport = { manifests: number; externalRows: number; evidencedRows: number; unownedRows: DependencyJsParityRow[]; undeclaredImports: DependencyJsParityImport[]; lockMismatches: DependencyJsLockMismatch[]; lockFixtureChecks: number; lockWorkspaces: number };

const DEPENDENCY_JS_SOURCE_EXTENSIONS = new Set([".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs", ".css", ".mdx"]);
const DEPENDENCY_JS_BUILTIN_PREFIXES = ["node:", "bun:"];
const DEPENDENCY_JS_BUILTINS = new Set(builtinModules.flatMap((name) => [name, name.replace(/^node:/, "")]));
const DEPENDENCY_JS_MANIFEST_SECTIONS: readonly DependencyJsManifestSection[] = ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"];

/** 🧪️ Compares package manifest dependency tables with Bun's live workspace snapshots. */
export function dependencyJsLockWorkspaceMismatches(manifestValues: ReadonlyMap<string, unknown>, workspaces: unknown): DependencyJsLockMismatch[] {
  const workspaceRecords = workspaces && typeof workspaces === "object" ? (workspaces as Record<string, unknown>) : {};
  const mismatches: DependencyJsLockMismatch[] = [];
  const table = (value: unknown, section: DependencyJsManifestSection): Record<string, string> => {
    if (!value || typeof value !== "object") return {};
    const candidate = (value as Record<string, unknown>)[section];
    if (!candidate || typeof candidate !== "object") return {};
    return Object.fromEntries(Object.entries(candidate as Record<string, unknown>).map(([name, version]) => [name, String(version)]));
  };
  for (const [manifest, manifestValue] of manifestValues) {
    const normalizedManifest = manifest.replaceAll("\\", "/");
    const workspaceKey = normalizedManifest === "package.json" ? "" : normalizedManifest.replace(/\/package\.json$/u, "");
    const lockValue = workspaceRecords[workspaceKey];
    if (!lockValue || typeof lockValue !== "object") {
      mismatches.push({ kind: "workspace-missing", manifest });
      continue;
    }
    for (const section of DEPENDENCY_JS_MANIFEST_SECTIONS) {
      const manifestTable = table(manifestValue, section);
      const lockTable = table(lockValue, section);
      for (const dependency of [...new Set([...Object.keys(manifestTable), ...Object.keys(lockTable)])].sort()) {
        const manifestVersion = manifestTable[dependency];
        const lockVersion = lockTable[dependency];
        if (manifestVersion === undefined) mismatches.push({ dependency, kind: "stale-in-lock", lockVersion, manifest, section });
        else if (lockVersion === undefined) mismatches.push({ dependency, kind: "missing-in-lock", manifest, manifestVersion, section });
        else if (manifestVersion !== lockVersion) mismatches.push({ dependency, kind: "version-mismatch", lockVersion, manifest, manifestVersion, section });
      }
    }
  }
  return mismatches.sort((left, right) => left.manifest.localeCompare(right.manifest) || String(left.section).localeCompare(String(right.section)) || String(left.dependency).localeCompare(String(right.dependency)) || left.kind.localeCompare(right.kind));
}



/** 🔒️ Reads Bun's lockfile and audits configured in-scope workspace snapshots against their manifests. */
function dependencyJsLockMismatches(repoRoot: string, manifests: readonly string[]): { mismatches: DependencyJsLockMismatch[]; workspaces: number } {
  const contents = dependencyReadFileSafe(repoRoot, "bun.lock");
  if (!contents) return { mismatches: [{ kind: "invalid-lockfile", manifest: "bun.lock" }], workspaces: 0 };
  try {
    const lock = Bun.JSONC.parse(contents) as { workspaces?: unknown };
    const rootManifest = Bun.JSONC.parse(dependencyReadFileSafe(repoRoot, "package.json")) as { workspaces?: unknown };
    const configuredWorkspaces = new Set(Array.isArray(rootManifest.workspaces) ? rootManifest.workspaces.filter((value): value is string => typeof value === "string").map((value) => value.replaceAll("\\", "/").replace(/\/$/u, "")) : []);
    const auditedManifests = manifests.filter((manifest) => manifest === "package.json" || configuredWorkspaces.has(manifest.replaceAll("\\", "/").replace(/\/package\.json$/u, "")));
    const manifestValues = new Map<string, unknown>();
    for (const manifest of auditedManifests) manifestValues.set(manifest, Bun.JSONC.parse(dependencyReadFileSafe(repoRoot, manifest)));
    return { mismatches: dependencyJsLockWorkspaceMismatches(manifestValues, lock.workspaces), workspaces: auditedManifests.length };
  } catch {
    return { mismatches: [{ kind: "invalid-lockfile", manifest: "bun.lock" }], workspaces: 0 };
  }
}

/** 🔎️ Resolves source ownership for a JavaScript manifest. A technology package owns its taxonomy unit; a nested target does too when no canonical technology manifest exists, and otherwise owns only its target directory. */
function dependencyJsOwnershipScope(manifest: string, manifests: readonly string[]): string {
  const marker = "/📦️packages/🟦️typescript/";
  const markerIndex = manifest.indexOf(marker);
  if (markerIndex >= 0) {
    const taxonomyScope = manifest.slice(0, markerIndex);
    const canonicalManifest = `${taxonomyScope}${marker}package.json`;
    if (manifest === canonicalManifest || !manifests.includes(canonicalManifest)) return taxonomyScope;
  }
  const directory = dirname(manifest);
  return directory === "." ? "" : directory;
}

/** 🔎️ Discovers JavaScript/config sources under the same exclusions as the dependency freeze. */
function dependencyDiscoverJsSourceFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      const child = relDir ? `${relDir}/${entry.name}` : entry.name;
      if (entry.isDirectory()) {
        if ((DEPENDENCY_SKIP_DIRS.has(entry.name) && entry.name !== ".storybook") || entry.name === "compose" || entry.name === ".🧬semio") continue;
        walk(child);
      } else if (entry.name !== "package.json" && (DEPENDENCY_JS_SOURCE_EXTENSIONS.has(extname(entry.name)) || dependencyJsIsConfigFile(child))) {
        found.push(child);
      }
    }
  };
  walk("");
  return found.sort();
}

/** 🔧️ Recognizes executable or package-loading configuration without scanning arbitrary JSON fixtures as dependency evidence. */
function dependencyJsIsConfigFile(file: string): boolean {
  const name = file.slice(file.lastIndexOf("/") + 1);
  return name === "nx.json" || name === "project.json" || /^tsconfig(?:\.[^.]+)*\.json$/u.test(name) || /(?:^|\/)\.storybook\//u.test(file) || /(?:^|[.\-🧰️⚙️])config\.[cm]?[jt]sx?$/u.test(name) || name === ".dependency-cruiser.cjs";
}

function dependencyJsPackageName(specifier: string): string {
  if (specifier.startsWith("@")) return specifier.split("/").slice(0, 2).join("/");
  return specifier.split("/", 1)[0]!;
}

function dependencyJsIsPackageName(name: string): boolean {
  return /^(?:@[A-Za-z0-9][A-Za-z0-9._-]*\/)?[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(name);
}

type DependencyJsToken = { kind: "identifier" | "string" | "punctuation"; value: string; line: number };

function dependencyJsTokens(contents: string): DependencyJsToken[] {
  const tokens: DependencyJsToken[] = [];
  let index = 0;
  let line = 1;
  const advance = (): string => {
    const character = contents[index++]!;
    if (character === "\n") line += 1;
    return character;
  };
  const skipQuoted = (quote: string): string => {
    let value = "";
    advance();
    while (index < contents.length) {
      const character = advance();
      if (character === "\\" && index < contents.length) {
        value += advance();
      } else if (character === quote) {
        break;
      } else {
        value += character;
      }
    }
    return value;
  };
  function skipTemplate(): void {
    advance();
    while (index < contents.length) {
      const character = contents[index]!;
      if (character === "\\") {
        advance();
        if (index < contents.length) advance();
      } else if (character === "`") {
        advance();
        return;
      } else if (contents.startsWith("${", index)) {
        advance();
        advance();
        let depth = 1;
        while (index < contents.length && depth > 0) {
          const expressionCharacter = contents[index]!;
          if (expressionCharacter === "'" || expressionCharacter === '"') {
            skipQuoted(expressionCharacter);
          } else if (expressionCharacter === "`") {
            skipTemplate();
          } else if (contents.startsWith("//", index)) {
            while (index < contents.length && advance() !== "\n") {}
          } else if (contents.startsWith("/*", index)) {
            advance();
            advance();
            let commentDepth = 1;
            while (index < contents.length && commentDepth > 0) {
              if (contents.startsWith("/*", index)) {
                advance();
                advance();
                commentDepth += 1;
              } else if (contents.startsWith("*/", index)) {
                advance();
                advance();
                commentDepth -= 1;
              } else {
                advance();
              }
            }
          } else {
            const consumed = advance();
            if (consumed === "{") depth += 1;
            if (consumed === "}") depth -= 1;
          }
        }
      } else {
        advance();
      }
    }
  }
  while (index < contents.length) {
    const character = contents[index]!;
    if (/\s/u.test(character)) {
      advance();
    } else if (contents.startsWith("//", index)) {
      while (index < contents.length && advance() !== "\n") {}
    } else if (contents.startsWith("/*", index)) {
      advance();
      advance();
      let depth = 1;
      while (index < contents.length && depth > 0) {
        if (contents.startsWith("/*", index)) {
          advance();
          advance();
          depth += 1;
        } else if (contents.startsWith("*/", index)) {
          advance();
          advance();
          depth -= 1;
        } else {
          advance();
        }
      }
    } else if (character === "'" || character === '"') {
      const tokenLine = line;
      tokens.push({ kind: "string", value: skipQuoted(character), line: tokenLine });
    } else if (character === "`") {
      skipTemplate();
    } else if (/[A-Za-z_$]/u.test(character)) {
      const tokenLine = line;
      let value = advance();
      while (index < contents.length && /[A-Za-z0-9_$]/u.test(contents[index]!)) value += advance();
      tokens.push({ kind: "identifier", value, line: tokenLine });
    } else {
      tokens.push({ kind: "punctuation", value: advance(), line });
    }
  }
  return tokens;
}

/** 🔎️ Extracts statically named bare module imports without mistaking fixture strings for code. */
function dependencyJsImports(contents: string): { dependency: string; line: number }[] {
  const tokens = dependencyJsTokens(contents);
  const imports: { dependency: string; line: number }[] = [];
  const record = (token: DependencyJsToken | undefined): void => {
    if (!token || token.kind !== "string") return;
    const specifier = token.value;
    if (specifier.startsWith(".") || specifier.startsWith("/") || specifier.startsWith("#") || specifier.startsWith("@/") || DEPENDENCY_JS_BUILTIN_PREFIXES.some((prefix) => specifier.startsWith(prefix)) || DEPENDENCY_JS_BUILTINS.has(specifier)) return;
    const dependency = dependencyJsPackageName(specifier);
    if (dependencyJsIsPackageName(dependency)) imports.push({ dependency, line: token.line });
  };
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index]!;
    if (token.kind !== "identifier") continue;
    if (token.value === "require" && tokens[index + 1]?.value === "(") record(tokens[index + 2]);
    if (token.value === "import") {
      if (tokens[index + 1]?.kind === "string") record(tokens[index + 1]);
      else if (tokens[index + 1]?.value === "(") record(tokens[index + 2]);
      else {
        for (let cursor = index + 1; cursor < Math.min(tokens.length, index + 64); cursor += 1) {
          if (tokens[cursor]!.value === ";") break;
          if (tokens[cursor]!.value === "from") {
            record(tokens[cursor + 1]);
            break;
          }
        }
      }
    }
    if (token.value === "export") {
      for (let cursor = index + 1; cursor < Math.min(tokens.length, index + 64); cursor += 1) {
        if (tokens[cursor]!.value === ";") break;
        if (tokens[cursor]!.value === "from") {
          record(tokens[cursor + 1]);
          break;
        }
      }
    }
  }
  for (const [lineIndex, sourceLine] of contents.split(/\r?\n/u).entries()) {
    const statement = sourceLine.match(/^\s*(?:import|export)\b.*\bfrom\s+["']([^"']+)["']\s*;?(?:\s*\/\/.*)?$/u) ?? sourceLine.match(/^\s*import\s+["']([^"']+)["']\s*;?(?:\s*\/\/.*)?$/u);
    if (!statement) continue;
    const specifier = statement[1]!;
    if (specifier.startsWith(".") || specifier.startsWith("/") || specifier.startsWith("#") || specifier.startsWith("@/") || DEPENDENCY_JS_BUILTIN_PREFIXES.some((prefix) => specifier.startsWith(prefix)) || DEPENDENCY_JS_BUILTINS.has(specifier)) continue;
    const dependency = dependencyJsPackageName(specifier);
    const sourceLineNumber = lineIndex + 1;
    if (dependencyJsIsPackageName(dependency) && !imports.some((item) => item.dependency === dependency && item.line === sourceLineNumber)) imports.push({ dependency, line: sourceLineNumber });
  }
  return imports;
}

/** 🔧️ Extracts exact package references from recognized config strings while retaining token line evidence and ignoring comments. */
function dependencyJsConfigReferences(contents: string): { dependency: string; line: number }[] {
  const references: { dependency: string; line: number }[] = [];
  for (const token of dependencyJsTokens(contents)) {
    if (token.kind !== "string" || token.value.startsWith(".") || token.value.startsWith("/") || token.value.startsWith("#")) continue;
    const dependency = dependencyJsPackageName(token.value);
    if (dependencyJsIsPackageName(dependency)) references.push({ dependency, line: token.line });
  }
  return references;
}

/** 🔎️ Audits direct JavaScript rows against taxonomy-owned source/import/script evidence. */
export function dependencyJsParity(repoRoot: string, lockParitySelfTests: () => number): DependencyJsParityReport {
  const lockFixtureChecks = lockParitySelfTests();
  const manifests = dependencyDiscoverPackageJsonFiles(repoRoot);
  const internalNames = dependencyInternalJsPackageNames(repoRoot, manifests);
  const sources = dependencyDiscoverJsSourceFiles(repoRoot);
  const sourceContents = new Map(sources.map((file) => [file, dependencyReadFileSafe(repoRoot, file)]));
  const sourceImports = new Map([...sourceContents].map(([file, contents]) => [file, dependencyJsImports(contents)]));
  const configReferences = new Map([...sourceContents].filter(([file]) => dependencyJsIsConfigFile(file)).map(([file, contents]) => [file, dependencyJsConfigReferences(contents)]));
  const owners = manifests.map((manifest) => ({ manifest, scope: dependencyJsOwnershipScope(manifest, manifests) })).sort((left, right) => right.scope.length - left.scope.length);
  const sourceOwners = new Map(sources.map((file) => [file, owners.find(({ scope }) => !scope || file === scope || file.startsWith(`${scope}/`))?.manifest]));
  const declared = new Map<string, Set<string>>();
  const engineProvided = new Map<string, Set<string>>();
  const rows: DependencyJsParityRow[] = [];
  for (const manifest of manifests) {
    const scope = dependencyJsOwnershipScope(manifest, manifests);
    const dependencies = dependencyParsePackageJson(repoRoot, manifest, internalNames);
    declared.set(manifest, new Set(dependencies.map((dependency) => dependency.name)));
    let scripts: Record<string, string> = {};
    try {
      const packageManifest = JSON.parse(dependencyReadFileSafe(repoRoot, manifest)) as { engines?: Record<string, string>; scripts?: Record<string, string> };
      scripts = packageManifest.scripts ?? {};
      engineProvided.set(manifest, new Set(Object.keys(packageManifest.engines ?? {}).filter(dependencyJsIsPackageName)));
    } catch {
      scripts = {};
      engineProvided.set(manifest, new Set());
    }
    for (const dependency of dependencies.filter((entry) => !entry.internal)) {
      const evidence: DependencyJsParityEvidence[] = [];
      for (const [file, imports] of sourceImports) {
        if (sourceOwners.get(file) !== manifest) continue;
        for (const imported of imports) {
          if (imported.dependency === dependency.name) evidence.push({ file, line: imported.line, kind: "import" });
          if (evidence.length >= 4) break;
        }
        if (evidence.length >= 4) break;
      }
      if (evidence.length < 4) {
        for (const [file, references] of configReferences) {
          if (sourceOwners.get(file) !== manifest) continue;
          for (const reference of references) {
            if (reference.dependency === dependency.name) evidence.push({ file, line: reference.line, kind: "config" });
            if (evidence.length >= 4) break;
          }
          if (evidence.length >= 4) break;
        }
      }
      if (evidence.length < 4) {
        for (const [name, command] of Object.entries(scripts)) {
          if (new RegExp(`(^|[^A-Za-z0-9@/_-])${dependency.name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}($|[^A-Za-z0-9@/_-])`).test(command)) evidence.push({ file: manifest, line: 0, kind: "script" });
          if (evidence.length >= 4) break;
        }
      }
      rows.push({ dependency: dependency.name, manifest, scope, evidence });
    }
  }
  const undeclaredImports: DependencyJsParityImport[] = [];
  for (const [file, imports] of sourceImports) {
    const owner = owners.find(({ scope }) => !scope || file === scope || file.startsWith(`${scope}/`));
    if (!owner) continue;
    for (const imported of imports) {
      if (internalNames.has(imported.dependency) || imported.dependency.startsWith("@semio-tech/") || declared.get(owner.manifest)?.has(imported.dependency) || engineProvided.get(owner.manifest)?.has(imported.dependency)) continue;
      undeclaredImports.push({ ...imported, file, manifest: owner.manifest });
    }
  }
  undeclaredImports.sort((left, right) => left.file.localeCompare(right.file) || left.line - right.line || left.dependency.localeCompare(right.dependency));
  const unownedRows = rows.filter((row) => row.evidence.length === 0).sort((left, right) => left.manifest.localeCompare(right.manifest) || left.dependency.localeCompare(right.dependency));
  const lockParity = dependencyJsLockMismatches(repoRoot, manifests);
  return { manifests: manifests.length, externalRows: rows.length, evidencedRows: rows.length - unownedRows.length, unownedRows, undeclaredImports, lockMismatches: lockParity.mismatches, lockFixtureChecks, lockWorkspaces: lockParity.workspaces };
}

/** 🔒️Every internal (workspace-owned) JS package name — the `"name"` field of every non-compose, non-node_modules `package.json`. Used to classify a dependency as first-party even when it isn't `@semio-tech/…`-scoped or `workspace:`-versioned. */
function dependencyInternalJsPackageNames(repoRoot: string, manifests: readonly string[]): Set<string> {
  const names = new Set<string>();
  for (const relPath of manifests) {
    try {
      const pkg = JSON.parse(dependencyReadFileSafe(repoRoot, relPath)) as { name?: string };
      if (pkg.name) names.add(pkg.name);
    } catch {
      /* ignore unparsable manifest */
    }
  }
  return names;
}

/** 🔒️Parses one `package.json`'s `dependencies`/`devDependencies`/`peerDependencies`/`optionalDependencies`. */
function dependencyParsePackageJson(repoRoot: string, relPath: string, internalNames: ReadonlySet<string>): DependencyJsEntry[] {
  let pkg: Record<string, unknown>;
  try {
    pkg = JSON.parse(dependencyReadFileSafe(repoRoot, relPath)) as Record<string, unknown>;
  } catch {
    return [];
  }
  const results: DependencyJsEntry[] = [];
  const sections: { key: string; kind: DependencyKind }[] = [
    { key: "dependencies", kind: dependencyKindOf("runtime") },
    { key: "peerDependencies", kind: dependencyKindOf("runtime") },
    { key: "optionalDependencies", kind: dependencyKindOf("runtime") },
    { key: "devDependencies", kind: dependencyKindOf("tooling") },
  ];
  for (const { key, kind } of sections) {
    const table = pkg[key];
    if (!table || typeof table !== "object") continue;
    for (const [name, versionRaw] of Object.entries(table as Record<string, string>)) {
      const version = String(versionRaw);
      const internal = internalNames.has(name) || name.startsWith("@semio-tech/") || version.startsWith("workspace:") || version.startsWith("file:") || version.startsWith("link:");
      results.push({ name, version, kind, internal });
    }
  }
  return results;
}

/** 🔒️Repo-wide `package.json` file paths (repo-relative), skipping policy, ticket, and composition trees. */
function dependencyDiscoverPackageJsonFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (DEPENDENCY_SKIP_DIRS.has(ent.name) || ent.name === "compose" || ent.name === ".🧬semio") continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "package.json") found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🧱️ Keeps both canonical and generated composition trees outside the governing refactor boundary. */
export function dependencyIsCompositionManifest(relPath: string): boolean {
  return relPath.startsWith("compose/") || relPath.startsWith("temp/compose/");
}

/** 🔒️Merges every third-party (non-internal) dependency across the whole workspace (Rust + JS, `compose/` excluded) into one baseline-entry list, keyed by `${ecosystem}:${name}`. */
export function dependencyFreezeCurrentThirdParty(repoRoot: string): DependencyBaselineEntry[] {
  const byKey = new Map<string, DependencyBaselineEntry>();
  const record = (ecosystem: DependencyEcosystem, name: string, version: string, kind: DependencyKind, user: string): void => {
    const key = `${ecosystem}:${name}`;
    const existing = byKey.get(key);
    if (existing) {
      if (!existing.kinds.includes(kind)) existing.kinds.push(kind);
      if (!existing.users.includes(user)) existing.users.push(user);
      if (!existing.declarations!.some((declaration) => declaration.user === user && declaration.version === version && declaration.kind === kind)) existing.declarations!.push({ user, version, kind });
      return;
    }
    byKey.set(key, { ecosystem, name, version, kinds: [kind], users: [user], productionReachable: false, declarations: [{ user, version, kind }] });
  };

  const cargoManifests = dependencyDiscoverCargoTomlFiles(repoRoot).filter((path) => !dependencyIsCompositionManifest(path));
  const workspaceDeps = dependencyParseWorkspaceDeps(repoRoot);
  for (const manifest of cargoManifests) {
    for (const dep of dependencyParseCargoToml(repoRoot, manifest, workspaceDeps)) {
      if (dep.internal) continue;
      record("rust", dep.name, dep.version, dep.kind, manifest);
    }
  }

  const jsManifests = dependencyDiscoverPackageJsonFiles(repoRoot);
  const internalJsNames = dependencyInternalJsPackageNames(repoRoot, jsManifests);
  for (const manifest of jsManifests) {
    for (const dep of dependencyParsePackageJson(repoRoot, manifest, internalJsNames)) {
      if (dep.internal) continue;
      record("js", dep.name, dep.version, dep.kind, manifest);
    }
  }

  dependencyCollectGo(repoRoot, record);
  dependencyCollectPython(repoRoot, record);
  dependencyCollectDotnet(repoRoot, record);

  const oracles = dependencyOracleRegistryPackages(repoRoot);
  const contributionTaxonomy = dependencyTaxonomy(repoRoot);
  for (const entry of byKey.values()) {
    dependencyClassifyOracleEntry(entry, oracles.get(entry.name), String(contributionTaxonomy.testOraclesDirName));
    entry.declarations?.sort((left, right) => left.user.localeCompare(right.user) || left.kind.localeCompare(right.kind) || left.version.localeCompare(right.version));
    entry.kinds.sort();
    entry.users.sort();
    entry.productionReachable = entry.kinds.some((kind) => kind === "production-runtime" || kind === "production-build");
  }
  return [...byKey.values()].sort((a, b) => (a.ecosystem === b.ecosystem ? a.name.localeCompare(b.name) : a.ecosystem.localeCompare(b.ecosystem)));
}

/** 📇️ Identifies canonical test cases, testing fixtures and the repository's semantic test domain. */
const DEPENDENCY_TEST_DOMAIN_PATH_RE = /(?:^|\/)(?:🧪️test|🧪️tests|🧫️fixtures)\//u;

/** 📇️An oracle name changes classification only when every declaration is owned by the test/oracle domain OR is itself a non-production declaration (`dev-dependencies`/`devDependencies`, i.e. `test-runner`/`repository-tooling` kind) — per the ticket's own definition of done, a third-party dependency kept ONLY behind `[dev-dependencies]` is compliant from ANY directory, not just a test-domain one. A genuine conflict requires an actual production-runtime/production-build declaration outside the test domain; those remain honest and become conflicts. */
export function dependencyClassifyOracleEntry(entry: DependencyBaselineEntry, oracleIds: readonly string[] | undefined, oracleDirectoryName: string): void {
  if (!oracleIds) return;
  entry.oracleIds = [...oracleIds].sort();
  const declarations = entry.declarations ?? entry.users.map((user) => ({ user, version: entry.version, kind: entry.kinds[0] ?? "repository-tooling" }));
  const isContribution = (path: string): boolean => {
    const parts = path.split("/");
    return parts.slice(0, -1).includes(oracleDirectoryName);
  };
  const productDeclarations = declarations.filter((declaration) => !DEPENDENCY_TEST_DOMAIN_PATH_RE.test(declaration.user) && !isContribution(declaration.user) && (declaration.kind === "production-runtime" || declaration.kind === "production-build"));
  if (productDeclarations.length === 0) entry.kinds = ["test-oracle"];
  else entry.oracleConflictUsers = [...new Set(productDeclarations.map((declaration) => declaration.user))].sort();
}

/** 📇️Package name → approved oracle ids, from the registry that claims them as test-only references. */
function dependencyOracleRegistryPackages(repoRoot: string): Map<string, string[]> {
  const map = new Map<string, string[]>();
  const taxonomy = dependencyTaxonomy(repoRoot);
  const oracleRegistry = taxonomy.testOracleRegistryLocation;
  const oracleRegistryPath = oracleRegistry
    ? `${oracleRegistry.directoryPath}/${canonicalFilenameForKind(oracleRegistry.fileKindId, taxonomy)}`
    : "";
  const contributionFilename = taxonomy.testContributionFileKindId
    ? canonicalFilenameForKind(taxonomy.testContributionFileKindId, taxonomy)
    : "";
  const manifests = [oracleRegistryPath, ...dependencyDiscoverContributionManifests(repoRoot, String(taxonomy.testOraclesDirName), contributionFilename)];
  for (const manifest of manifests) {
    if (manifest === "") continue;
    const content = dependencyReadFileSafe(repoRoot, manifest);
    if (!content) continue;
    try {
      for (const entry of (JSON.parse(content) as { oracles?: { id: string; package: string }[] }).oracles ?? []) {
        map.set(entry.package, [...(map.get(entry.package) ?? []), entry.id]);
      }
    } catch {
      /* an unreadable manifest means no package is excused as an oracle */
    }
  }
  return map;
}

/** 🧩️Every `<owner>/<contributionDir>/<contributionFile>` manifest, found by convention. */
function dependencyDiscoverContributionManifests(repoRoot: string, dirName: string, fileName: string): string[] {
  if (dirName === "" || fileName === "") return [];
  const found: string[] = [];
  const walk = (relDir: string): void => {
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(join(repoRoot, relDir || "."), { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (!entry.isDirectory()) continue;
      const childRel = relDir ? `${relDir}/${entry.name}` : entry.name;
      if (entry.name === "node_modules" || entry.name === ".git" || entry.name === ".nx" || entry.name === "target" || entry.name === "dist" || childRel.startsWith(".🧬semio")) continue;
      if (entry.name === dirName) {
        if (existsSync(join(repoRoot, childRel, fileName))) found.push(`${childRel}/${fileName}`);
        continue;
      }
      walk(childRel);
    }
  };
  walk("");
  return found;
}

/** 🐹️Module directories `go.work` declares, minus any `compose/` path — the hard forbidden area. */
function dependencyParseGoModuleDirs(content: string): string[] {
  const dirs: string[] = [];
  const single = content.match(/^\s*use\s+(\S+)\s*$/gm) ?? [];
  for (const line of single) dirs.push(line.replace(/^\s*use\s+/, "").trim());
  const block = content.match(/use\s*\(([\s\S]*?)\)/);
  if (block) for (const line of block[1]!.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith("//")) dirs.push(trimmed);
  }
  return [...new Set(dirs.map((dir) => dir.replace(/^\.\//, "")).filter((dir) => dir !== "" && !dir.startsWith("compose/") && dir !== "compose"))].sort();
}

function dependencyGoModuleDirs(repoRoot: string): string[] {
  return dependencyParseGoModuleDirs(dependencyReadFileSafe(repoRoot, "go.work"));
}

type DependencyGoRequirement = { name: string; version: string; kind: DependencyKind };

/** 🐹️Reads module identity and local replacements without invoking the Go toolchain. */
export function dependencyParseGoModule(content: string): { module?: string; localReplaces: string[]; requirements: DependencyGoRequirement[] } {
  const module = content.match(/^\s*module\s+(\S+)\s*$/mu)?.[1];
  const localReplaces: string[] = [];
  const requirements: DependencyGoRequirement[] = [];
  let block: "require" | "replace" | null = null;
  for (const raw of content.split(/\r?\n/u)) {
    const line = raw.trim();
    if (/^(require|replace)\s*\($/u.test(line)) {
      block = line.startsWith("require") ? "require" : "replace";
      continue;
    }
    if (block && line === ")") {
      block = null;
      continue;
    }
    const requireBody = block === "require" ? line : line.startsWith("require ") ? line.slice("require ".length) : "";
    const requirement = requireBody.match(/^([^\s]+)\s+([^\s]+)(\s*\/\/\s*indirect)?/u);
    if (requirement) requirements.push({ name: requirement[1]!, version: requirement[2]!, kind: dependencyKindOf(requirement[3] ? "build" : "runtime") });
    const replaceBody = block === "replace" ? line : line.startsWith("replace ") ? line.slice("replace ".length) : "";
    const replacement = replaceBody.match(/^([^\s]+)(?:\s+v[^\s]+)?\s+=>\s+([^\s]+)/u);
    if (replacement && /^(?:\.{1,2}[\\/]|[\\/]|[A-Za-z]:[\\/])/u.test(replacement[2]!)) localReplaces.push(replacement[1]!);
  }
  return { module, localReplaces: [...new Set(localReplaces)].sort(), requirements };
}

/** 🐹️First-party module paths proven by `go.work` membership or a local `replace`. */
function dependencyGoInternalModules(repoRoot: string, dirs = dependencyGoModuleDirs(repoRoot)): Set<string> {
  const internal = new Set<string>();
  for (const replaced of dependencyParseGoModule(dependencyReadFileSafe(repoRoot, "go.work")).localReplaces) internal.add(replaced);
  for (const dir of dirs) {
    const parsed = dependencyParseGoModule(dependencyReadFileSafe(repoRoot, `${dir}/go.mod`));
    if (parsed.module) internal.add(parsed.module);
    for (const replaced of parsed.localReplaces) internal.add(replaced);
  }
  return internal;
}

/** 🐹️Third-party requirements of every non-compose Go module. `// indirect` requirements are transitive
 * evidence rather than a declared use, so they are classed as build-phase rather than runtime. */
export function dependencyCollectGo(repoRoot: string, record: (ecosystem: DependencyEcosystem, name: string, version: string, kind: DependencyKind, user: string) => void, recordFirstParty?: (entry: DependencyBaselineEntry) => void): void {
  const dirs = dependencyGoModuleDirs(repoRoot);
  const internal = dependencyGoInternalModules(repoRoot, dirs);
  for (const dir of dirs) {
    const relPath = `${dir}/go.mod`.replace(/^\/+/, "");
    for (const requirement of dependencyParseGoModule(dependencyReadFileSafe(repoRoot, relPath)).requirements) {
      const { name, version, kind } = requirement;
      if (!name.includes(".") || name.startsWith("semio.tech/")) continue;
      if (internal.has(name)) {
        recordFirstParty?.({ ecosystem: "go", name, version, kinds: [kind], users: [relPath], productionReachable: kind === "production-runtime" || kind === "production-build" });
        continue;
      }
      record("go", name, version, kind, relPath);
    }
  }
}

/** 🐍️Declared Python requirements of every non-compose `pyproject.toml`. */
function dependencyCollectPython(repoRoot: string, record: (ecosystem: DependencyEcosystem, name: string, version: string, kind: DependencyKind, user: string) => void): void {
  const manifests: string[] = [];
  const walk = (relDir: string): void => {
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(join(repoRoot, relDir || "."), { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (ent.name === "node_modules" || ent.name === ".git" || ent.name === ".venv" || ent.name === ".nx" || ent.name === "target" || ent.name === "dist" || ent.name === "compose" || childRel.startsWith(".🧬semio")) continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "pyproject.toml") manifests.push(childRel);
    }
  };
  walk("");
  const requirement = /^\s*"([A-Za-z0-9._-]+)\s*([^"]*)"\s*,?\s*$/;
  for (const manifest of manifests) {
    const content = dependencyReadFileSafe(repoRoot, manifest);
    if (!content) continue;
    let section: DependencyKind | null = null;
    for (const raw of content.split(/\r?\n/)) {
      const line = raw.trim();
      if (line.startsWith("[")) {
        section = null;
        continue;
      }
      if (/^dependencies\s*=\s*\[/.test(line)) section = dependencyKindOf("runtime");
      else if (/^(dev|test|lint|docs)\s*=\s*\[/.test(line)) section = dependencyKindOf("test");
      else if (/^\]/.test(line)) section = null;
      if (section === null) continue;
      const match = line.match(requirement);
      if (!match) continue;
      const name = match[1]!;
      if (name.startsWith("semio")) continue;
      record("python", name, match[2]!.trim() || "*", section, manifest);
    }
  }
}

/** 🔷️`PackageReference`s of every non-compose .NET project; test projects contribute test-runner deps only. */
function dependencyCollectDotnet(repoRoot: string, record: (ecosystem: DependencyEcosystem, name: string, version: string, kind: DependencyKind, user: string) => void): void {
  const projects: string[] = [];
  const walk = (relDir: string): void => {
    let entries: ReturnType<typeof readdirSync>;
    try {
      entries = readdirSync(join(repoRoot, relDir || "."), { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (ent.name === "node_modules" || ent.name === ".git" || ent.name === ".nx" || ent.name === "bin" || ent.name === "obj" || ent.name === "target" || ent.name === "compose" || childRel.startsWith(".🧬semio")) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".csproj")) projects.push(childRel);
    }
  };
  walk("");
  for (const project of projects) {
    const content = dependencyReadFileSafe(repoRoot, project);
    if (!content) continue;
    const references = [...content.matchAll(/<PackageReference\s+Include="([^"]+)"(?:[^>]*Version="([^"]*)")?/g)];
    const isTestProject = /<IsTestProject>\s*true\s*<\/IsTestProject>/i.test(content) || references.some(([, name]) => name!.startsWith("xunit") || name!.startsWith("Microsoft.NET.Test") || name!.startsWith("NUnit"));
    for (const [, name, version] of references) record("dotnet", name!, version ?? "*", dependencyKindOf(isTestProject ? "test" : "runtime"), project);
  }
}

/** 🔒️Reads the committed baseline, or `null` if it doesn't exist yet (first run — `verify dependencies write-baseline` creates it). */
function dependencyFreezeLoadBaseline(repoRoot: string): DependencyBaseline | null {
  const content = dependencyReadFileSafe(repoRoot, DEPENDENCY_BASELINE_REL_PATH);
  if (!content) return null;
  try {
    return JSON.parse(content) as DependencyBaseline;
  } catch {
    return null;
  }
}

/** 🔒️Writes the current third-party dependency inventory as the new committed baseline (repo root, `🔒️dependencies.json`). */
export function dependencyFreezeWriteBaseline(repoRoot: string): DependencyBaseline {
  const probe = runProbe("git", ["rev-parse", "HEAD"], { cwd: repoRoot });
  const commit = probe.status === 0 ? probe.stdout.trim() : "unknown";
  const entries = dependencyFreezeCurrentThirdParty(repoRoot).map((entry) => ({ ecosystem: entry.ecosystem, name: entry.name, version: entry.version, kinds: entry.kinds, users: entry.users, productionReachable: entry.productionReachable, ...(entry.oracleIds ? { oracleIds: entry.oracleIds } : {}) }));
  const baseline: DependencyBaseline = { schemaVersion: 2, generatedAt: new Date().toISOString(), commit, entries };
  writeFileSync(join(repoRoot, DEPENDENCY_BASELINE_REL_PATH), `${JSON.stringify(baseline, null, 2)}\n`, "utf8");
  return baseline;
}

export type DependencyFreezeCheckResult = { baseline: DependencyBaseline; current: DependencyBaselineEntry[]; newDeps: DependencyBaselineEntry[]; removedDeps: DependencyBaselineEntry[] };

/** 🔒️Compares the current third-party inventory against the committed baseline. Only NEW dependencies (present now, absent from baseline) are a failure — removals always pass, so the check only ever ratchets tighter. */
export function dependencyFreezeCheck(repoRoot: string): DependencyFreezeCheckResult {
  const baseline = dependencyFreezeLoadBaseline(repoRoot);
  const current = dependencyFreezeCurrentThirdParty(repoRoot);
  if (!baseline) return { baseline: { schemaVersion: 2, generatedAt: "", commit: "", entries: [] }, current, newDeps: current, removedDeps: [] };
  const baselineKeys = new Set(baseline.entries.map((e) => `${e.ecosystem}:${e.name}`));
  const currentKeys = new Set(current.map((e) => `${e.ecosystem}:${e.name}`));
  const newDeps = current.filter((e) => !baselineKeys.has(`${e.ecosystem}:${e.name}`));
  const removedDeps = baseline.entries.filter((e) => !currentKeys.has(`${e.ecosystem}:${e.name}`));
  return { baseline, current, newDeps, removedDeps };
}
