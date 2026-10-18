//#region Constants
const METRICS_LOCK_FILES = new Set(["package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock", "bun.lockb", "cargo.lock"]);

const METRICS_LICENSE_TEMPLATE_BASENAMES = new Set(["LICENSE", "LICENSE.md", "LICENSE.txt", "COPYING", "COPYING.md", "NOTICE", "NOTICE.md", "UNLICENSE", "UNLICENSE.md"]);

const LANG_EMOJI: Record<string, string> = {
  TypeScript: "🟦️",
  JavaScript: "🟨️",
  Go: "🔵️",
  "C#": "🟣️",
  Python: "🐍️",
  Rust: "🦀️",
  Shell: "🐚️",
  Dockerfile: "🐳️",
  Makefile: "🔧️",
  CSS: "🎨️",
  SQL: "🛢️",
  HTML: "🌐️",
  Markdown: "📝️",
  TeX: "📐️",
  JSON: "🧾️",
  YAML: "📋️",
  TOML: "⚙️",
  XML: "📄️",
  CSV: "📑️",
  "Bourne Shell": "🐚️",
  "Bourne Again Shell": "🐚️",
  PowerShell: "💠️",
  Docker: "🐳️",
};

const ULOC_EXCLUDE_DIRS = [".🧬semio", "node_modules", "dist", "build", "target", ".git", ".nx", "coverage", ".cache", ".turbo", ".next", "out", "vendor", "third_party", "Carthage"];

export const MAX_METRICS_FILE_BYTES = 8 * 1024 * 1024;

//#endregion Constants

export function normalizeRepoPath(path: string): string {
  return path.replace(/\\/g, "/").replace(/^\.\//, "");
}

function metricsPathBasename(rel: string): string {
  return rel.slice(rel.lastIndexOf("/") + 1);
}

function hasHiddenDotPathSegment(rel: string): boolean {
  for (const seg of rel.split("/")) {
    if (!seg || seg === "." || seg === "..") continue;
    if (seg.startsWith(".")) return true;
  }
  return false;
}

function isMetricsLockOrGenerated(rel: string): boolean {
  const base = metricsPathBasename(rel);
  if (METRICS_LOCK_FILES.has(base)) return true;
  if (base.endsWith(".generated.go") || base.endsWith(".pb.go")) return true;
  return false;
}

function isMetricsLicenseTemplateFile(rel: string): boolean {
  const base = metricsPathBasename(rel);
  if (METRICS_LICENSE_TEMPLATE_BASENAMES.has(base)) return true;
  if (base.startsWith("LICENSE.")) return true;
  return false;
}

/** 🗂️Whether paths must be excluded from uloc/metrics (dot paths, license templates, vendor, lockfiles). */
export function shouldSkipPathForUloc(root: string, relPath: string): boolean {
  const rel = normalizeRepoPath(relPath);
  if (!rel || rel === ".🧬semio" || rel.startsWith(".🧬semio")) return true;
  if (hasHiddenDotPathSegment(rel)) return true;
  if (isMetricsLicenseTemplateFile(rel)) return true;
  if (isMetricsLockOrGenerated(rel)) return true;
  for (const dir of ULOC_EXCLUDE_DIRS) {
    if (rel === dir || rel.startsWith(`${dir}/`)) return true;
  }
  return false;
}

/** 🏷️Maps a repo path to a metrics language bucket (code langs + JSON/YAML/… formats). */
export function classifyPathForMetrics(path: string): string {
  const rel = normalizeRepoPath(path);
  const base = rel.slice(rel.lastIndexOf("/") + 1).toLowerCase();
  if (base === "dockerfile" || base.startsWith("dockerfile.")) return "Dockerfile";
  if (base === "makefile" || base === "justfile") return "Makefile";
  const ext = rel.slice(rel.lastIndexOf(".")).toLowerCase();
  switch (ext) {
    case ".ts":
    case ".tsx":
    case ".cts":
    case ".mts":
    case ".mtsx":
      return "TypeScript";
    case ".js":
    case ".mjs":
    case ".cjs":
      return "JavaScript";
    case ".go":
      return "Go";
    case ".cs":
      return "C#";
    case ".py":
      return "Python";
    case ".rs":
      return "Rust";
    case ".sh":
    case ".bash":
    case ".zsh":
      return "Shell";
    case ".ps1":
      return "PowerShell";
    case ".css":
    case ".scss":
    case ".sass":
      return "CSS";
    case ".sql":
      return "SQL";
    case ".html":
    case ".htm":
    case ".xhtml":
      return "HTML";
    case ".md":
    case ".markdown":
    case ".mdown":
    case ".mkd":
    case ".mdx":
    case ".mdc":
    case ".svx":
      return "Markdown";
    case ".tex":
    case ".sty":
    case ".cls":
    case ".ltx":
    case ".bib":
      return "TeX";
    case ".json":
    case ".jsonc":
      return "JSON";
    case ".yaml":
    case ".yml":
      return "YAML";
    case ".toml":
      return "TOML";
    case ".csv":
      return "CSV";
    case ".xml":
      return "XML";
    default:
      return "";
  }
}

/** 🎨️Emoji for a metrics language row. */
export function langMetricsEmoji(lang: string): string {
  return LANG_EMOJI[lang] ?? "📎️";
}
