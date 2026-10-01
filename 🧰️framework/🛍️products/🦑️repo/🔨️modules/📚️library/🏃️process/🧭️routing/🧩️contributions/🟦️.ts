import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { discoverPackages } from "../../../🔍️discovery/🟦️.ts";
import { runOwnedCommand } from "../../🎛️owned-execution/🟦️.ts";

/** 🧾️ One present owner's declarative workspace command contribution. */
export type OwnedScriptRoute = Readonly<{ command: readonly string[]; project: string; target: string; packageRoot: string }>;

/** 🔎️ Reads command contributions only from admitted present package projects. */
export function ownedScriptRoutes(repoRoot: string): OwnedScriptRoute[] {
  const routes: OwnedScriptRoute[] = [], keys = new Set<string>();
  for (const owner of discoverPackages(repoRoot)) {
    const projectPath = join(repoRoot, owner.packageRel, "📋️project.json");
    if (!existsSync(projectPath)) continue;
    const project = JSON.parse(readFileSync(projectPath, "utf8"));
    for (const [target, raw] of Object.entries(project.targets ?? {})) {
      const declaration = (raw as { metadata?: { semio?: { workspaceCommand?: unknown } } }).metadata?.semio?.workspaceCommand;
      if (declaration === undefined) continue;
      if (!Array.isArray(declaration) || declaration.length < 2 || declaration.some((word) => typeof word !== "string" || !/^[a-zA-Z0-9-]+$/.test(word))) throw new Error(`Invalid owned command ${projectPath}:${target}`);
      if (typeof project.name !== "string" || !project.name || typeof target !== "string" || !target) throw new Error(`Invalid owned command identity ${projectPath}:${target}`);
      const key = declaration.join(" ");
      if (keys.has(key)) throw new Error(`Duplicate owned command ${key}`);
      keys.add(key);
      routes.push({ command: declaration, project: project.name, target, packageRoot: owner.packageRel });
    }
  }
  return routes.sort((left, right) => right.command.length - left.command.length || left.command.join(" ").localeCompare(right.command.join(" ")));
}

/** 🚦️ Resolves the longest admitted command prefix without importing a test implementation. */
export function resolveOwnedScriptRoute(routes: readonly OwnedScriptRoute[], segments: readonly string[]): Readonly<{ route: OwnedScriptRoute; args: string[] }> | undefined {
  const matches = routes.filter((route) => route.command.every((word, index) => segments[index] === word)).sort((left, right) => right.command.length - left.command.length);
  if (!matches.length) return undefined;
  if (matches[1]?.command.length === matches[0]!.command.length) throw new Error(`Ambiguous owned command ${segments.join(" ")}`);
  return { route: matches[0]!, args: segments.slice(matches[0]!.command.length) };
}

/** ▶️ Executes an admitted owner's Nx target with progress, cancellation and inherited output ownership. */
export async function dispatchOwnedScriptRoute(repoRoot: string, segments: readonly string[]): Promise<boolean> {
  const selected = resolveOwnedScriptRoute(ownedScriptRoutes(repoRoot), segments);
  if (!selected) return false;
  const { route, args } = selected;
  await runOwnedCommand(process.execPath, ["nx", "run", `${route.project}:${route.target}`, "--skip-nx-cache", ...(args.length ? ["--", ...args] : [])], repoRoot, `owned-command:${route.project}:${route.target}`);
  return true;
}

