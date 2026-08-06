/**
 * 🖥️ Renders `.vscode/launch.json` from the hand-maintained seed (`.vscode/🧩️launch.seed.jsonc`) plus
 * the playground registry — the single source of truth for per-plugin dev-server ports. Never
 * hand-edit `.vscode/launch.json` directly: edit the seed file (for keyboard/mouse shortcuts, bespoke
 * tooling launchers, fixture/native variants, build/publish groups, and the `devLaunchers`
 * per-playground-variant templates), or a plugin's `[[package.metadata.semio.playground]]` block (for
 * ports), then regenerate.
 *
 * 🚪️ Module only — `📜️script.ts` owns the CLI: `generate` writes this output alongside the registry
 * catalog and `check` verifies its freshness (CLAUDE.md: one `script.ts` per bundle). The playground
 * catalog is passed IN rather than imported, so this module never depends on `📜️script.ts` at runtime.
 *
 * @see .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/LAUNCH-JSON-GENERATOR-FROM-PLAYGROUND-REGISTRY
 * @see .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/REGISTRY-SCRIPT-REFACTOR-TO-VOCABULARY-DISCOVERY-LIBRARY
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { PlaygroundEntry } from "./📜️script.ts";

const SEED_REL_PATH = ".vscode/🧩️launch.seed.jsonc";
/** @emoji 📄️ Repo-relative path of the generated output, shared with `📜️script.ts`'s freshness gate. */
export const LAUNCH_OUTPUT_REL_PATH = ".vscode/launch.json";
const DEV_LAUNCHERS_MARKER =
  ',\n\n  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated\n  // output); see 🖥️launch.ts readSeed() for the exact split contract this marker line supports.\n  "devLaunchers": ';

//#region 🔖️DevLauncher
/** @emoji 🧩️ One `serverReadyAction` shape with a `"{PORT}"` token substituted at render time. */
type ServerReadyTemplate = { readonly pattern: string; readonly uriFormat: string };

/** @emoji 🎮️ Hand-curated parts of one playground variant's `3_dev` launch entries that the plugin
 * registry cannot supply (display name, launch command, VS Code presentation order, env/serverReadyAction
 * shape). Registry ports fill any `"{PORT}"` token in `reactEnv`/`wgpuEnv`/`*ServerReadyAction`. */
type DevLauncherEntry = {
  readonly namePrefix: string;
  readonly order: number;
  readonly command: string;
  readonly reactEnv: Readonly<Record<string, string>>;
  readonly reactServerReadyAction: ServerReadyTemplate;
  readonly wgpuOrder?: number;
  readonly wgpuEnv?: Readonly<Record<string, string>>;
  readonly wgpuServerReadyAction?: ServerReadyTemplate;
};
//#endregion

//#region 🔖️SeedSplit
/** @emoji ✂️ Splits the seed file into the output skeleton (verbatim `configurations` text with
 * `"@generated:<variant>:<renderer>"` placeholders) and the parsed `devLaunchers` table. Both live in
 * one JSONC document; `DEV_LAUNCHERS_MARKER` is the exact, generator-authored boundary between them. */
function readSeed(repoRoot: string): { readonly skeleton: string; readonly devLaunchers: Readonly<Record<string, DevLauncherEntry>> } {
  const seedPath = join(repoRoot, SEED_REL_PATH);
  const raw = readFileSync(seedPath, "utf8");
  const markerIndex = raw.indexOf(DEV_LAUNCHERS_MARKER);
  if (markerIndex === -1) throw new Error(`🖥️launch.ts: seed file ${seedPath} is missing the devLaunchers marker`);
  const skeleton = `${raw.slice(0, markerIndex)}}\n`;
  const devLaunchersJsonText = raw.slice(markerIndex + DEV_LAUNCHERS_MARKER.length, raw.length - "\n}\n".length);
  const devLaunchers = JSON.parse(devLaunchersJsonText) as Record<string, DevLauncherEntry>;
  return { skeleton, devLaunchers };
}
//#endregion

//#region 🔖️Render
function renderEnv(template: Readonly<Record<string, string>>, port: number): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [key, value] of Object.entries(template)) out[key] = value === "{PORT}" ? String(port) : value;
  return out;
}

function renderServerReadyAction(template: ServerReadyTemplate, port: number): { action: string; pattern: string; uriFormat: string } {
  return { action: "openExternally", pattern: template.pattern.replaceAll("{PORT}", String(port)), uriFormat: template.uriFormat.replaceAll("{PORT}", String(port)) };
}

/** @emoji 🧱️ Builds one `3_dev` launch config object for a variant+renderer, matching the field order
 * and shape of every hand-authored playground launcher in `.vscode/launch.json` today. */
function renderEntry(name: string, launcher: DevLauncherEntry, renderer: "react" | "wgpu", port: number): object {
  const env = renderer === "react" ? launcher.reactEnv : launcher.wgpuEnv;
  const sra = renderer === "react" ? launcher.reactServerReadyAction : launcher.wgpuServerReadyAction;
  const order = renderer === "react" ? launcher.order : launcher.wgpuOrder;
  if (!env || !sra || order === undefined) throw new Error(`🖥️launch.ts: devLauncher "${name}" is missing ${renderer} fields`);
  return {
    name,
    type: "node-terminal",
    request: "launch",
    command: launcher.command,
    cwd: "${workspaceFolder}",
    env: renderEnv(env, port),
    presentation: { group: "3_dev", order },
    serverReadyAction: renderServerReadyAction(sra, port),
  };
}

/** @emoji ↔️ Re-indents a `JSON.stringify(obj, null, 2)` block (0-based) to sit at the seed's 4-space
 * `configurations` array-item depth; only line 1 needs no shift since it replaces an inline placeholder. */
function reindent(jsonText: string, extraSpaces: number): string {
  const pad = " ".repeat(extraSpaces);
  return jsonText
    .split("\n")
    .map((line, i) => (i === 0 ? line : pad + line))
    .join("\n");
}
//#endregion

//#region 🔖️Generate
/** @emoji 🏗️ Renders the full `.vscode/launch.json` text: seed skeleton with every
 * `@generated:<variant>:<renderer>` placeholder substituted by a fresh, registry-ported entry. */
export function generateLaunchJson(repoRoot: string, playgrounds: readonly PlaygroundEntry[]): string {
  const { skeleton, devLaunchers } = readSeed(repoRoot);
  const byVariant = new Map(playgrounds.map((entry) => [entry.variant, entry]));
  let out = skeleton;
  for (const [variant, launcher] of Object.entries(devLaunchers)) {
    const playground = byVariant.get(variant);
    if (!playground) throw new Error(`🖥️launch.ts: devLaunchers["${variant}"] has no matching playground registry entry (renamed or removed plugin — update the seed)`);
    const reactPlaceholder = JSON.stringify(`@generated:${variant}:react`);
    if (!out.includes(reactPlaceholder)) throw new Error(`🖥️launch.ts: seed is missing placeholder ${reactPlaceholder}`);
    const reactName = `🛠️dev${launcher.namePrefix}⚛️react`;
    out = out.replace(reactPlaceholder, reindent(JSON.stringify(renderEntry(reactName, launcher, "react", playground.ports.react), null, 2), 4));
    if (launcher.wgpuOrder !== undefined) {
      const wgpuPlaceholder = JSON.stringify(`@generated:${variant}:wgpu`);
      if (!out.includes(wgpuPlaceholder)) throw new Error(`🖥️launch.ts: seed is missing placeholder ${wgpuPlaceholder}`);
      const wgpuName = `🛠️dev${launcher.namePrefix}🧊️wgpu🌐️wasm`;
      out = out.replace(wgpuPlaceholder, reindent(JSON.stringify(renderEntry(wgpuName, launcher, "wgpu", playground.ports.wgpu), null, 2), 4));
    }
  }
  if (out.includes("@generated:")) throw new Error("🖥️launch.ts: an @generated placeholder was not resolved (devLaunchers table is missing an entry)");
  return out;
}
//#endregion
