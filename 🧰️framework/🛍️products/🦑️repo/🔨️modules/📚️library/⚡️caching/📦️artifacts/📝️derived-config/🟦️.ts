/** 📝️ Checked-in configuration that is a pure function of one canonical file. `generate config` writes it and `generate config --check` (and the
 * workspace policy) fails on any drift, so no second copy is ever edited by hand.
 *   1. The devcontainer editor block (`customizations.vscode.settings` / `extensions`) is `.vscode/settings.json` and
 *      `.vscode/extensions.json` minus the omitted host-only entries plus the declared overlay (`.devcontainer/editor-overlay.json`),
 *      with `${workspaceFolder}` rewritten to `${containerWorkspaceFolder}`.
 *   2. Every cargo-nextest scope file (`<workspace>/.config/nextest.toml`) is the root file followed by that workspace's own
 *      `[[profile.quick.overrides]]` tables.
 *   3. `.github/dependabot.yml` names only directories that exist and hold the ecosystem's manifest, and covers every `go.work` module.
 * https://containers.dev/implementors/json_reference/#vscode https://nexte.st/docs/configuration/ https://docs.github.com/en/code-security/dependabot */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";

/** 🧩️ What the devcontainer does not inherit from the host editor files, and what only it adds. */
export type EditorOverlay = Readonly<{
  settings: Readonly<{ omit: readonly string[]; set: Readonly<Record<string, unknown>> }>;
  extensions: Readonly<{ omit: readonly string[]; add: readonly string[] }>;
}>;

/** 🖊️ The editor surface of one host: workspace settings and recommended extensions. */
export type EditorSurface = Readonly<{ settings: Readonly<Record<string, unknown>>; extensions: readonly string[] }>;

const EDITOR_OVERLAY = ".devcontainer/editor-overlay.json";
export const DEVCONTAINER = ".devcontainer/devcontainer.json";
export const NEXTEST = ".config/nextest.toml";
export const NEXTEST_SCOPES = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⏱️process-budgets/native-profile.json";
const HOST_FOLDER = "${workspaceFolder}";
const CONTAINER_FOLDER = "${containerWorkspaceFolder}";
const QUICK_OVERRIDES = /^\[\[profile\.quick\.overrides\]\]/m;

/** 🧮️ Derives the devcontainer editor surface from the host canonical surface and the declared overlay. */
export function deriveContainerEditor(canonical: EditorSurface, overlay: EditorOverlay): EditorSurface {
  const inherited = JSON.parse(JSON.stringify(canonical.settings).replaceAll(HOST_FOLDER, CONTAINER_FOLDER)) as Record<string, unknown>;
  const settings: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(inherited)) if (!overlay.settings.omit.includes(key)) settings[key] = value;
  Object.assign(settings, overlay.settings.set);
  const extensions = [...canonical.extensions.filter((id) => !overlay.extensions.omit.includes(id)), ...overlay.extensions.add];
  return { settings, extensions: [...new Set(extensions)] };
}

function readJsonc<T>(workspace: string, path: string): T {
  return Bun.JSONC.parse(readFileSync(join(workspace, path), "utf8")) as T;
}

/** 📖️ Reads the host canonical editor surface. */
export function readCanonicalEditor(workspace: string): EditorSurface {
  return { settings: readJsonc(workspace, ".vscode/settings.json"), extensions: readJsonc<{ recommendations: string[] }>(workspace, ".vscode/extensions.json").recommendations };
}

/** 📖️ Reads the container editor surface currently checked into the devcontainer. */
export function readContainerEditor(workspace: string): EditorSurface {
  const vscode = readJsonc<{ customizations: { vscode: { settings: Record<string, unknown>; extensions: string[] } } }>(workspace, DEVCONTAINER).customizations.vscode;
  return { settings: vscode.settings, extensions: vscode.extensions };
}

/** 🧮️ The editor surface the devcontainer must hold for this checkout. */
export function expectedContainerEditor(workspace: string): EditorSurface {
  return deriveContainerEditor(readCanonicalEditor(workspace), readJsonc<EditorOverlay>(workspace, EDITOR_OVERLAY));
}

/** 🔎️ Names the editor entries that differ between two surfaces. */
export function describeEditorDrift(expected: EditorSurface, actual: EditorSurface): string[] {
  const problems: string[] = [];
  const keys = [...new Set([...Object.keys(expected.settings), ...Object.keys(actual.settings)])];
  const settings = keys.filter((key) => !isDeepStrictEqual(expected.settings[key], actual.settings[key]));
  if (settings.length) problems.push(`customizations.vscode.settings differs from .vscode/settings.json plus ${EDITOR_OVERLAY} at: ${settings.join(", ")}`);
  if (!isDeepStrictEqual(expected.extensions, actual.extensions)) problems.push(`customizations.vscode.extensions differs from .vscode/extensions.json plus ${EDITOR_OVERLAY}`);
  return problems;
}

/** 🧭️ Skips whitespace, commas and comments from `index`. */
function skip(text: string, index: number): number {
  for (;;) {
    const newline = text.indexOf("\n", index);
    if (/[\s,]/.test(text[index] ?? "x")) index++;
    else if (text.startsWith("//", index)) index = newline === -1 ? text.length : newline + 1;
    else if (text.startsWith("/*", index)) index = text.indexOf("*/", index) + 2;
    else return index;
  }
}

/** 🔪️ Returns the index just after the JSONC value starting at `start`. */
function valueEnd(text: string, start: number): number {
  let depth = 0;
  for (let index = start; index < text.length; index++) {
    const character = text[index]!;
    if (character === '"') {
      for (index++; text[index] !== '"'; index++) if (text[index] === "\\") index++;
      if (depth === 0) return index + 1;
    } else if (character === "{" || character === "[") depth++;
    else if (character === "}" || character === "]") {
      if (depth === 0) return index;
      if (--depth === 0) return index + 1;
    } else if (depth === 0 && /[,\s]/.test(character)) return index;
  }
  return text.length;
}

/** 🧭️ Locates the value range of `path` (object keys only) inside JSONC text. */
function locate(text: string, path: readonly string[]): { start: number; end: number } {
  let start = skip(text, 0), end = valueEnd(text, start);
  for (const key of path) {
    let index = skip(text, start + 1), found = false;
    while (index < end && text[index] === '"') {
      const keyEnd = valueEnd(text, index), name = JSON.parse(text.slice(index, keyEnd)) as string;
      const valueStart = skip(text, text.indexOf(":", keyEnd) + 1), valueFinish = valueEnd(text, valueStart);
      if (name === key) {
        [start, end, found] = [valueStart, valueFinish, true];
        break;
      }
      index = skip(text, valueFinish);
    }
    if (!found) throw new Error(`no property ${path.join(".")}`);
  }
  return { start, end };
}

/** 🖨️ Prints JSON with objects expanded and arrays of scalars on one line while they fit, as Prettier does. */
function printJson(value: unknown, indent = "", width = 120): string {
  if (Array.isArray(value)) {
    const inline = `[${value.map((item) => JSON.stringify(item)).join(", ")}]`;
    if (value.every((item) => item === null || typeof item !== "object") && indent.length + inline.length <= width) return inline;
    return value.length ? `[\n${value.map((item) => `${indent}  ${printJson(item, `${indent}  `, width)}`).join(",\n")}\n${indent}]` : "[]";
  }
  if (value !== null && typeof value === "object") {
    const entries = Object.entries(value);
    return entries.length ? `{\n${entries.map(([key, item]) => `${indent}  ${JSON.stringify(key)}: ${printJson(item, `${indent}  `, width)}`).join(",\n")}\n${indent}}` : "{}";
  }
  return JSON.stringify(value);
}

/** ✍️ Replaces one JSONC property value in place and keeps every comment and sibling untouched. */
export function replaceJsoncValue(text: string, path: readonly string[], value: unknown): string {
  const { start, end } = locate(text, path);
  const lineStart = text.lastIndexOf("\n", start) + 1, indent = /^[ \t]*/.exec(text.slice(lineStart))![0];
  return text.slice(0, start) + printJson(value, indent) + text.slice(end);
}

function normalize(text: string): string {
  return text.replaceAll("\r\n", "\n");
}

/** 📚️ The workspaces that own a cargo-nextest scope file besides the root one. */
export function nextestScopes(workspace: string): string[] {
  return (JSON.parse(readFileSync(join(workspace, NEXTEST_SCOPES), "utf8")) as { workspaces: { directory: string }[] }).workspaces.map((row) => row.directory).filter((directory) => directory !== ".");
}

/** 🧮️ A scope file is the root file followed by the scope's own `[[profile.quick.overrides]]` tables. */
export function deriveNextest(root: string, scope: string): string {
  const base = normalize(root).trimEnd(), own = normalize(scope), from = own.startsWith(base) ? base.length : 0;
  const first = own.slice(from).search(QUICK_OVERRIDES);
  return first === -1 ? `${base}\n` : `${base}\n\n${own.slice(from + first).trimEnd()}\n`;
}

export function nextestProblems(workspace: string): string[] {
  const root = readFileSync(join(workspace, NEXTEST), "utf8"), problems: string[] = [];
  for (const scope of nextestScopes(workspace)) {
    const path = `${scope}/${NEXTEST}`, own = readFileSync(join(workspace, path), "utf8");
    if (normalize(own) !== deriveNextest(root, own)) problems.push(`${path} must be ${NEXTEST} followed by its own [[profile.quick.overrides]] tables`);
  }
  return problems;
}

const DEPENDABOT_MANIFESTS: Readonly<Record<string, (folder: string) => boolean>> = {
  bun: (folder) => existsSync(join(folder, "package.json")),
  cargo: (folder) => existsSync(join(folder, "Cargo.toml")),
  gomod: (folder) => existsSync(join(folder, "go.mod")),
  uv: (folder) => existsSync(join(folder, "pyproject.toml")),
  nuget: (folder) => readdirSync(folder).some((name) => name.endsWith(".csproj") || name.endsWith(".sln")),
  "github-actions": (folder) => existsSync(join(folder, ".github", "workflows")),
};

function dependabotDirectories(workspace: string, pattern: string): string[] {
  const relative = pattern.replace(/^\/+/, "");
  if (!relative.includes("*")) return existsSync(join(workspace, relative)) ? [relative] : [];
  return [...new Bun.Glob(relative).scanSync({ cwd: workspace, onlyFiles: false })].sort();
}

export function dependabotProblems(workspace: string): string[] {
  const document = Bun.YAML.parse(readFileSync(join(workspace, ".github/dependabot.yml"), "utf8")) as { updates: { "package-ecosystem": string; directory?: string; directories?: string[] }[] };
  const problems: string[] = [], covered = new Map<string, Set<string>>();
  for (const update of document.updates) {
    const ecosystem = update["package-ecosystem"], manifest = DEPENDABOT_MANIFESTS[ecosystem];
    if (!manifest) {
      problems.push(`.github/dependabot.yml: ecosystem ${ecosystem} has no manifest rule`);
      continue;
    }
    for (const pattern of [...(update.directories ?? []), ...(update.directory ? [update.directory] : [])]) {
      const found = dependabotDirectories(workspace, pattern);
      if (!found.length) problems.push(`.github/dependabot.yml: ${ecosystem} directory ${pattern} does not exist`);
      for (const directory of found) {
        covered.set(ecosystem, (covered.get(ecosystem) ?? new Set()).add(directory));
        if (!manifest(join(workspace, directory))) problems.push(`.github/dependabot.yml: ${ecosystem} directory ${pattern} holds no manifest`);
      }
    }
  }
  const modules = [...readFileSync(join(workspace, "go.work"), "utf8").matchAll(/^\t\.\/(.+)$/gm)].map((match) => match[1]!.trim());
  for (const module of modules) if (!covered.get("gomod")?.has(module)) problems.push(`.github/dependabot.yml: go.work module ${module} has no gomod entry`);
  return problems;
}

/** 🔎️ Every drift between checked-in configuration and its canonical source; empty when all of it is derived. */
export function checkDerivedConfig(workspace: string): string[] {
  return [...describeEditorDrift(expectedContainerEditor(workspace), readContainerEditor(workspace)).map((problem) => `${DEVCONTAINER}: ${problem}`), ...nextestProblems(workspace), ...dependabotProblems(workspace)];
}

/** ✍️ Rewrites every derived file from its canonical source; returns the files that changed. */
export async function writeDerivedConfig(workspace: string): Promise<string[]> {
  const changed: string[] = [];
  const write = async (path: string, next: string): Promise<void> => {
    const file = join(workspace, path);
    if (normalize(readFileSync(file, "utf8")) === normalize(next)) return;
    await Bun.write(file, next);
    changed.push(path);
  };
  const expected = expectedContainerEditor(workspace), devcontainer = readFileSync(join(workspace, DEVCONTAINER), "utf8");
  await write(DEVCONTAINER, replaceJsoncValue(replaceJsoncValue(devcontainer, ["customizations", "vscode", "settings"], expected.settings), ["customizations", "vscode", "extensions"], expected.extensions));
  const root = readFileSync(join(workspace, NEXTEST), "utf8");
  for (const scope of nextestScopes(workspace)) {
    const path = `${scope}/${NEXTEST}`;
    await write(path, deriveNextest(root, readFileSync(join(workspace, path), "utf8")));
  }
  return changed;
}
