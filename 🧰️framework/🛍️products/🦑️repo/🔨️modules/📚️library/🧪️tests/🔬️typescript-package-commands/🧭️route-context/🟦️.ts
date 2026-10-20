/** 🧭️ The invocation, roots and re-entry of the test script that its route group functions receive instead of the script instance. */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

export type TestRouteContext = {
  readonly invocation: ScriptInvocation;
  readonly repoRoot: string;
  readonly root: string;
  readonly run: (segments: string[]) => Promise<void>;
};

/** 🛎️ Reads the one route family source of the test script that registers `route`, with its workspace-relative path. */
export function readRegisteredTestRoute(repoRoot: string, route: string): Readonly<{ path: string; text: string }> {
  const folder = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands";
  const marker = `segments[0] === "${route}"`;
  const families = readdirSync(join(repoRoot, folder), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => `${folder}/${entry.name}/🟦️.ts`).filter((path) => existsSync(join(repoRoot, path)));
  const matches = families.map((path) => ({ path, text: readFileSync(join(repoRoot, path), "utf8") })).filter((family) => family.text.includes(marker));
  if (matches.length !== 1) throw new Error(`Exactly one test route family must register ${route}`);
  return matches[0]!;
}
