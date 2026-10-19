/** 📝️ Derives cargo-nextest scope configurations and validates dependency-update manifest ownership.
 * @see https://nexte.st/docs/configuration/ */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

export const NEXTEST = ".config/nextest.toml";
export const NEXTEST_SCOPES = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/⏱️process-budgets/native-profile.json";
const HOST_FOLDER = "${workspaceFolder}";
const CONTAINER_FOLDER = "${containerWorkspaceFolder}";
const QUICK_OVERRIDES = /^\[\[profile\.quick\.overrides\]\]/m;

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
  nuget: (folder) => readdirSync(folder).some((name) => name.endsWith(".csproj")),
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
  return [...nextestProblems(workspace), ...dependabotProblems(workspace)];
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
  const root = readFileSync(join(workspace, NEXTEST), "utf8");
  for (const scope of nextestScopes(workspace)) {
    const path = `${scope}/${NEXTEST}`;
    await write(path, deriveNextest(root, readFileSync(join(workspace, path), "utf8")));
  }
  return changed;
}
