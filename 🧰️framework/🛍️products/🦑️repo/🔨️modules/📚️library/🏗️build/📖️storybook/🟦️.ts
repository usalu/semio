import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { readBuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";

//#region 🔖️StorybookDiscoveryGuard
const UI_STORYBOOK_DISCOVERY_BASELINE = { entries: 231, stories: 170, docs: 61, inputs: 61 } as const;
const STORYBOOK_MDX_SKIP_DIRS = new Set(["node_modules", "compose", ".🧬semio", ".git", ".nx", "target", "dist", "storybook-static"]);

/** 📖️ Rejects MDX inputs inside the retired-transform non-Compose ownership boundary. */
export function assertNoOwnedStorybookMdx(root: string): void {
  const mdx: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory() && STORYBOOK_MDX_SKIP_DIRS.has(entry.name)) continue;
      const full = join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.isFile() && entry.name.endsWith(".mdx")) mdx.push(relative(root, full));
    }
  };
  walk(root);
  if (mdx.length > 0)
    throw new Error(
      `[storybook] owned MDX transforms are retired, but ${mdx.length} owned MDX input(s) exist:\n${mdx
        .sort()
        .map((file) => `  ${file}`)
        .join("\n")}`,
    );
}

/** 📊️ Freezes the UI Storybook's TS/TSX discovery after its owned MDX-transform retirement. */
export function assertUiStorybookDiscovery(root: string): void {
  if ((process.env.STORYBOOK_SCOPE ?? "") !== "ui") return;
  const index = JSON.parse(readFileSync(join(root, process.env.STORYBOOK_OUTPUT_DIR ?? `dist/${readBuildMode(process.env.SEMIO_BUILD_MODE)}/storybook`, "index.json"), "utf8")) as { entries?: Record<string, { type?: string; importPath?: string }> };
  const entries = Object.values(index.entries ?? {});
  const stories = entries.filter((entry) => entry.type === "story").length;
  const docs = entries.filter((entry) => entry.type === "docs").length;
  const inputs = new Set(entries.map((entry) => entry.importPath).filter((path): path is string => Boolean(path)));
  const unsupported = [...inputs].filter((path) => !/\.(?:ts|tsx)$/u.test(path));
  const actual = { entries: entries.length, stories, docs, inputs: inputs.size };
  if (unsupported.length > 0 || Object.entries(UI_STORYBOOK_DISCOVERY_BASELINE).some(([key, value]) => actual[key as keyof typeof actual] !== value)) {
    throw new Error(`[storybook] UI discovery changed: expected ${JSON.stringify(UI_STORYBOOK_DISCOVERY_BASELINE)}, got ${JSON.stringify(actual)}${unsupported.length > 0 ? `; non-TS/TSX inputs: ${unsupported.join(", ")}` : ""}`);
  }
  console.log(`[storybook] UI discovery guard passed (${stories} stories, ${docs} docs, ${inputs.size} TS/TSX inputs, 0 MDX).`);
}

