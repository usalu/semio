//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: shared taxonomy vocabulary + repo-wide package discovery contract.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, dirname, relative } from "node:path";
import { fileURLToPath } from "node:url";
//#endregion 🔌️Adapters

const __dirname = dirname(fileURLToPath(import.meta.url));

//#region 🔣️Taxonomy
/**
 * 🔣️ Shape of `🔣️taxonomy.json` — the single source of truth for taxonomy directory-name/role/lang
 * vocabulary, replacing the two independently hand-maintained copies in framework/os/plugin/registry
 * script.ts (`TAXONOMY_ARTIFACT_COMPONENTS`/`TAXONOMY_WINDOW_CHILDREN`) and root script.ts
 * (`POLICY_ARTIFACT_COMPONENT_DIRS`/`POLICY_WINDOW_CHILD_DIRS`) — see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`.
 */
export interface Taxonomy {
  readonly _comment?: string;
  readonly roles: readonly string[];
  readonly langs: readonly string[];
  readonly artifactComponentDirs: readonly string[];
  readonly artifactChildDirsExtra: readonly string[];
  readonly windowChildDirs: readonly string[];
  readonly taxonomyLeafParentDirs: readonly string[];
  readonly taxonomyLeafFilename: string;
  readonly libWiringLineBudget: number;
  readonly forbiddenPathSegments: readonly string[];
  readonly packagesDirName: string;
  readonly areas: Readonly<Record<string, string>>;
}

let cachedTaxonomy: Taxonomy | undefined;

/** 📖️ Reads and parses `🔣️taxonomy.json` (sibling to this module, resolved via `import.meta.url` like `getLibRoot` in `📦️index.ts` of this same package) — cached after the first call. */
export function loadTaxonomy(): Taxonomy {
  if (cachedTaxonomy) return cachedTaxonomy;
  const path = join(__dirname, "🔣️taxonomy.json");
  cachedTaxonomy = JSON.parse(readFileSync(path, "utf8")) as Taxonomy;
  return cachedTaxonomy;
}

/** 🧭️ Longest-prefix match of a repo-relative path against `taxonomy.areas` keys — `undefined` outside every declared area. */
export function areaOf(repoRelPath: string, taxonomy: Taxonomy = loadTaxonomy()): string | undefined {
  const norm = repoRelPath.replaceAll("\\", "/").replace(/^\.\//, "");
  let bestKey: string | undefined;
  for (const key of Object.keys(taxonomy.areas)) {
    if (norm !== key && !norm.startsWith(`${key}/`)) continue;
    if (!bestKey || key.length > bestKey.length) bestKey = key;
  }
  return bestKey ? taxonomy.areas[bestKey] : undefined;
}
//#endregion 🔣️Taxonomy

//#region 🧭️Discovery
/** 🎭️ Package "kind" declared by `[package.metadata.semio] role`/`"semio": { "role" }` — see `readSemioMarker`. */
export type PackageRole = "plugin" | "framework" | "product" | "hub" | "s-module" | "extension" | "testkit" | "tool";

/** 🌐️ Ecosystem a discovered package's manifest belongs to. */
export type PackageLang = "🦀️rust" | "🟦️typescript" | "🐹️go" | "🐍️python";

/** 📦️ One package discovered under `<owner>/📦️packages/<lang>/`, with its role/id marker resolved. */
export interface DiscoveredPackage {
  readonly ownerRel: string;
  readonly lang: PackageLang;
  readonly manifestPath: string;
  readonly role: PackageRole;
  readonly id: string;
  readonly area: string;
}

const MANIFEST_FILENAME: Partial<Record<PackageLang, string>> = {
  "🦀️rust": "Cargo.toml",
  "🟦️typescript": "package.json",
  "🐹️go": "go.mod",
  "🐍️python": "pyproject.toml",
};

/** 🔍️ Extracts the `[package.metadata.semio]` table body from Cargo.toml text (stops at the next `[…]` header) — simple line-scan, no TOML parser dependency (none is a repo dependency for the TS side; `readSemioMarker` mirrors `hasSemioRole`'s approach in framework/os/plugin/registry script.ts). */
function rustSemioTableBody(text: string): string | undefined {
  const lines = text.split("\n");
  const start = lines.findIndex((line) => line.trim() === "[package.metadata.semio]");
  if (start === -1) return undefined;
  const body: string[] = [];
  for (let i = start + 1; i < lines.length; i++) {
    if (lines[i].trim().startsWith("[")) break;
    body.push(lines[i]);
  }
  return body.join("\n");
}

/** 🏷️ Package name from a Cargo.toml's `[package] name = "…"` line. */
function rustPackageName(text: string): string | undefined {
  return text.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
}

/**
 * 🏷️ Reads a package's role marker: rust — `[package.metadata.semio] role = "…"` (line-scan, matching
 * `hasSemioRole` in framework/os/plugin/registry script.ts); typescript — top-level `"semio": { "role": "…" }`
 * in `package.json`. go/python return `undefined` until those ecosystems join the taxonomy contract.
 */
export function readSemioMarker(manifestPath: string, lang: PackageLang): { role: PackageRole; id?: string } | undefined {
  if (!existsSync(manifestPath)) return undefined;
  if (lang === "🦀️rust") {
    const body = rustSemioTableBody(readFileSync(manifestPath, "utf8"));
    if (!body) return undefined;
    const role = body.match(/^role\s*=\s*"([^"]+)"/m)?.[1] as PackageRole | undefined;
    if (!role) return undefined;
    const id = body.match(/^id\s*=\s*"([^"]+)"/m)?.[1];
    return id ? { role, id } : { role };
  }
  if (lang === "🟦️typescript") {
    try {
      const pkg = JSON.parse(readFileSync(manifestPath, "utf8")) as { semio?: { role?: string; id?: string } };
      const role = pkg.semio?.role as PackageRole | undefined;
      if (!role) return undefined;
      return pkg.semio?.id ? { role, id: pkg.semio.id } : { role };
    } catch {
      return undefined;
    }
  }
  // TODO(W8): read from sibling 📋️project.json metadata.semio / pyproject.toml [tool.semio] once those
  // ecosystems are restructured.
  return undefined;
}

const DISCOVERY_SKIP_DIRS = new Set(["node_modules", "target", "dist", ".git", ".🦑️repo", "🤖️generated", "🔌️plugin-modules", "pkg", "storybook-static", "temp", "compose"]);

/** 📁️ `readdirSync(dir, { withFileTypes: true })`, defaulting to `[]` for an unreadable/missing dir — a helper (rather than an explicit `ReturnType<typeof readdirSync>` annotation) so the `Dirent<string>` element type infers unambiguously from this specific overload. */
function readdirSafe(absDir: string) {
  try {
    return readdirSync(absDir, { withFileTypes: true });
  } catch {
    return [];
  }
}

/** 🔤️ Drops every non-ASCII codepoint (emoji + variation selectors), e.g. `"✒️writer"` -> `"writer"` — mirrors `policyStripEmoji` in root script.ts. */
function stripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

/** 🆔️ Falls back to the manifest's own package name, else an emoji-stripped dash-joined owner path, when `readSemioMarker` found no explicit `id`. */
function fallbackPackageId(manifestPath: string, lang: PackageLang, ownerRel: string): string {
  try {
    if (lang === "🦀️rust") {
      const name = rustPackageName(readFileSync(manifestPath, "utf8"));
      if (name) return name;
    } else if (lang === "🟦️typescript") {
      const name = (JSON.parse(readFileSync(manifestPath, "utf8")) as { name?: string }).name;
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

/**
 * 🗺️ Walks `repoRoot` for every `<owner>/📦️packages/<lang>/` directory holding a lang manifest
 * (`Cargo.toml` for 🦀️rust, `package.json` for 🟦️typescript), reads its role marker via
 * `readSemioMarker`, and returns the flattened catalog — the repo-wide package discovery contract this
 * taxonomy vocabulary exists to serve (extending discovery beyond plugins to framework, hub, s kernels,
 * repo product, print, mit-bestand — see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`). A `📦️packages/<lang>/` dir whose
 * manifest carries no resolvable role marker is skipped (not yet migrated, or an ecosystem not yet
 * touched).
 */
export function discoverPackages(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredPackage[] {
  const packagesDirName = taxonomy.packagesDirName;
  const found: DiscoveredPackage[] = [];

  const walk = (absDir: string): void => {
    for (const entry of readdirSafe(absDir)) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || DISCOVERY_SKIP_DIRS.has(entry.name)) continue;
      const absChild = join(absDir, entry.name);
      if (entry.name !== packagesDirName) {
        walk(absChild);
        continue;
      }
      const ownerRel = relative(repoRoot, absDir).replaceAll("\\", "/");
      for (const langEntry of readdirSafe(absChild)) {
        if (!langEntry.isDirectory()) continue;
        const lang = langEntry.name as PackageLang;
        const manifestFilename = MANIFEST_FILENAME[lang];
        if (!manifestFilename) continue;
        const manifestAbs = join(absChild, langEntry.name, manifestFilename);
        const marker = readSemioMarker(manifestAbs, lang);
        if (!marker) continue;
        found.push({
          ownerRel,
          lang,
          manifestPath: relative(repoRoot, manifestAbs).replaceAll("\\", "/"),
          role: marker.role,
          id: marker.id ?? fallbackPackageId(manifestAbs, lang, ownerRel),
          area: areaOf(ownerRel, taxonomy) ?? "",
        });
      }
    }
  };
  walk(repoRoot);
  return found.sort((a, b) => a.ownerRel.localeCompare(b.ownerRel));
}
//#endregion 🧭️Discovery
