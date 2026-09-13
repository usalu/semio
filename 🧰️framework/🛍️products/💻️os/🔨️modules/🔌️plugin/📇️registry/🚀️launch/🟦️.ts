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
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/LAUNCH-JSON-GENERATOR-FROM-PLAYGROUND-REGISTRY
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/REGISTRY-SCRIPT-REFACTOR-TO-VOCABULARY-DISCOVERY-LIBRARY
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { PlaygroundEntry } from "../🎮️playground/🔎️discovery/🟦️.ts";

const SEED_REL_PATH = ".vscode/🧩️launch.seed.jsonc";
/** @emoji 🖼️ The renderers whose `serve`/`dev` browser listeners every playground variant owns a
 * `0_dev` launcher for. `⚛️react` serves `🎯️targets/⚛️react`'s Vite host; `🧊️wgpu` serves
 * `🎯️targets/🧊️wgpu/🌐️server`'s, the one browser listener that consumes Nx-completed WGPU artifacts
 * (`@semio-tech/framework-os-dev:serve-<variant>-wgpu-<profile>`). Both target families are inferred
 * per variant from the playground registry, so a new plugin gets both rows with no seed edit. */
const RENDERER_ROWS = [
  { id: "react", badge: "⚛️react" },
  { id: "wgpu", badge: "🧊️wgpu" },
] as const;
/** @emoji 📄️ Repo-relative path of the generated output, shared with `📜️script.ts`'s freshness gate. */
export const LAUNCH_OUTPUT_REL_PATH = ".vscode/launch.json";
const DEV_LAUNCHERS_MARKER =
  ',\n\n  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated\n  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.\n  "devLaunchers": ';

//#region 🔖️DevLauncher
/** @emoji 🧩️ One `serverReadyAction` shape with a `"{PORT}"` token substituted at render time. */
type ServerReadyTemplate = { readonly pattern: string; readonly uriFormat: string };

/** @emoji 👥️ Multi-user expansion template for one playground variant's `@generated:<variant>:users`
 * placeholder: one launcher per registry `userPorts.react[]`/`userPorts.wgpu[]` slot (1-based `{N}`),
 * reusing the variant's own `command`/`reactServerReadyAction`/`wgpuServerReadyAction` and offsetting
 * its `order`/`wgpuOrder` by `0.01 * N`. `env` values may carry `"{N}"`, `"{PORT}"` and `"{EMAIL}"`
 * tokens; `SEMIO_RENDERER` is set programmatically per renderer, not part of this template. */
type DevLauncherUsersTemplate = {
  readonly namePrefixPattern: string;
  readonly emailPattern: string;
  readonly env: Readonly<Record<string, string>>;
};

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
  readonly users?: DevLauncherUsersTemplate;
};
//#endregion

//#region 🔖️SeedSplit
/** @emoji ✂️ Splits the seed file into the output skeleton (verbatim `configurations` text with
 * `"@generated:<variant>:<renderer>"` placeholders) and the parsed `devLaunchers` table. Both live in
 * one JSONC document; `DEV_LAUNCHERS_MARKER` is the exact, generator-authored boundary between them. */
function readSeed(repoRoot: string, readText?: (path: string) => string): { readonly skeleton: string; readonly devLaunchers: Readonly<Record<string, DevLauncherEntry>> } {
  const seedPath = join(repoRoot, SEED_REL_PATH);
  const raw = readText ? readText(SEED_REL_PATH) : readFileSync(seedPath, "utf8");
  try {
    Bun.JSONC.parse(raw);
  } catch {
    throw new Error(`🚀️launch/🟦️.ts: seed file ${seedPath} is not valid JSONC`);
  }
  const markerIndex = raw.indexOf(DEV_LAUNCHERS_MARKER);
  if (markerIndex === -1) throw new Error(`🚀️launch/🟦️.ts: seed file ${seedPath} is missing the devLaunchers marker`);
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
  if (!env || !sra || order === undefined) throw new Error(`🚀️launch/🟦️.ts: devLauncher "${name}" is missing ${renderer} fields`);
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

/** @emoji 🧽️ Supplies one zero-touch renderer entry when a discovered variant has no curated launcher surface. */
function renderDiscoveredEntry(playground: PlaygroundEntry, namePrefix: string, renderer: "react" | "wgpu" | "native", order: number, command?: string): object {
  const port = renderer === "react" ? playground.ports.react : playground.ports.wgpu;
  if (renderer === "native") {
    return {
      name: `🛠️dev${namePrefix}🧊️wgpu🖥️native`,
      type: "node-terminal",
      request: "launch",
      command: `bun nx run @semio-tech/framework-os-dev:run-${playground.variant}-native-dev`,
      cwd: "${workspaceFolder}",
      env: { SEMIO_PLUGIN: playground.pluginId, ...(playground.app ? { SEMIO_APP: playground.app } : {}) },
      presentation: { group: "3_dev", order },
    };
  }
  return {
    name: `🛠️dev${namePrefix}${renderer === "react" ? "⚛️react" : "🧊️wgpu🌐️wasm"}`,
    type: "node-terminal",
    request: "launch",
    command: command ?? `bun nx run workspace:dev -- ${playground.variant}`,
    cwd: "${workspaceFolder}",
    env: { S_OS_PORT: String(port), SEMIO_PLUGIN: playground.pluginId, SEMIO_RENDERER: renderer, ...(playground.app ? { SEMIO_APP: playground.app } : {}) },
    presentation: { group: "3_dev", order },
    serverReadyAction: { action: "openExternally", pattern: `(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):${port})`, uriFormat: "%s" },
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

/** @emoji 🔤️ Substitutes the `users` template's `"{N}"` / `"{PORT}"` / `"{EMAIL}"` tokens in `text`. */
function substituteUserTokens(text: string, n: number, port: number, email: string): string {
  return text.replaceAll("{N}", String(n)).replaceAll("{PORT}", String(port)).replaceAll("{EMAIL}", email);
}

/** @emoji 👥️ Renders one `users` launcher for user slot `n` (1-based) of one renderer, reusing the
 * variant's own `command`/`serverReadyAction` and offsetting its base `order` by `0.01 * n`. */
function renderUserEntry(users: DevLauncherUsersTemplate, launcher: DevLauncherEntry, renderer: "react" | "wgpu", n: number, port: number, baseOrder: number, sra: ServerReadyTemplate): object {
  const email = substituteUserTokens(users.emailPattern, n, port, "");
  const namePrefix = substituteUserTokens(users.namePrefixPattern, n, port, email);
  const name = `🛠️dev${namePrefix}${renderer === "react" ? "⚛️react" : "🧊️wgpu🌐️wasm"}`;
  const env: Record<string, string> = {};
  for (const [key, value] of Object.entries(users.env)) env[key] = substituteUserTokens(value, n, port, email);
  env.SEMIO_RENDERER = renderer;
  return {
    name,
    type: "node-terminal",
    request: "launch",
    command: launcher.command,
    cwd: "${workspaceFolder}",
    env,
    // ↕️ Rounded to 2dp: floating-point addition of `0.01 * n` onto a decimal `baseOrder` (e.g.
    // `386.2 + 0.02`) otherwise lands on an ugly `386.21999999999997` instead of the clean `386.22`.
    presentation: { group: "3_dev", order: Math.round((baseOrder + n * 0.01) * 100) / 100 },
    serverReadyAction: renderServerReadyAction(sra, port),
  };
}

/** @emoji 👥️ Renders every launcher for one variant's `@generated:<variant>:users` placeholder: one
 * per `playground.userPorts.react[]` slot, then one per `playground.userPorts.wgpu[]` slot (only when
 * the base launcher also declares `wgpuOrder`/`wgpuServerReadyAction`). */
function renderUserEntries(launcher: DevLauncherEntry, playground: PlaygroundEntry): object[] {
  const users = launcher.users;
  if (!users) return [];
  if (!playground.userPorts) throw new Error(`🚀️launch/🟦️.ts: devLaunchers["${playground.variant}"] declares "users" but the registry entry has no "userPorts"`);
  const entries: object[] = [];
  playground.userPorts.react.forEach((port, index) => entries.push(renderUserEntry(users, launcher, "react", index + 1, port, launcher.order, launcher.reactServerReadyAction)));
  if (launcher.wgpuOrder !== undefined && launcher.wgpuServerReadyAction) {
    playground.userPorts.wgpu.forEach((port, index) => entries.push(renderUserEntry(users, launcher, "wgpu", index + 1, port, launcher.wgpuOrder!, launcher.wgpuServerReadyAction!)));
  }
  return entries;
}
//#endregion

//#region 🔖️Generate
/** @emoji 🏗️ Renders the full `.vscode/launch.json` text: seed skeleton with every
 * `@generated:<variant>:<renderer>` placeholder substituted by a fresh, registry-ported entry. */
export function generateLaunchJson(repoRoot: string, playgrounds: readonly PlaygroundEntry[], components: readonly { project: string; pluginId: string }[], readText?: (path: string) => string): string {
  const { skeleton, devLaunchers } = readSeed(repoRoot, readText);
  const byVariant = new Map(playgrounds.map((entry) => [entry.variant, entry]));
  let out = skeleton;
  for (const [variant, launcher] of Object.entries(devLaunchers)) {
    const playground = byVariant.get(variant);
    if (!playground) throw new Error(`🚀️launch/🟦️.ts: devLaunchers["${variant}"] has no matching playground registry entry (renamed or removed plugin — update the seed)`);
    const reactPlaceholder = JSON.stringify(`@generated:${variant}:react`);
    if (!out.includes(reactPlaceholder)) throw new Error(`🚀️launch/🟦️.ts: seed is missing placeholder ${reactPlaceholder}`);
    const reactName = `🛠️dev${launcher.namePrefix}⚛️react`;
    out = out.replace(reactPlaceholder, reindent(JSON.stringify(renderEntry(reactName, launcher, "react", playground.ports.react), null, 2), 4));
    if (launcher.wgpuOrder !== undefined) {
      const wgpuPlaceholder = JSON.stringify(`@generated:${variant}:wgpu`);
      if (!out.includes(wgpuPlaceholder)) throw new Error(`🚀️launch/🟦️.ts: seed is missing placeholder ${wgpuPlaceholder}`);
      const wgpuName = `🛠️dev${launcher.namePrefix}🧊️wgpu🌐️wasm`;
      out = out.replace(wgpuPlaceholder, reindent(JSON.stringify(renderEntry(wgpuName, launcher, "wgpu", playground.ports.wgpu), null, 2), 4));
    }
    if (launcher.users) {
      const usersPlaceholder = JSON.stringify(`@generated:${variant}:users`);
      if (!out.includes(usersPlaceholder)) throw new Error(`🚀️launch/🟦️.ts: seed is missing placeholder ${usersPlaceholder}`);
      const userEntriesText = renderUserEntries(launcher, playground)
        .map((entry) => JSON.stringify(entry, null, 2))
        .join(",\n");
      out = out.replace(usersPlaceholder, reindent(userEntriesText, 4));
    }
  }
  if (out.includes("@generated:")) throw new Error("🚀️launch/🟦️.ts: an @generated placeholder was not resolved (devLaunchers table is missing an entry)");
  const synthesized: object[] = [];
  const printCatalogPath = "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json";
  const printCatalog = JSON.parse(readText ? readText(printCatalogPath) : readFileSync(join(repoRoot, printCatalogPath), "utf8")) as { documents: { id: string }[] };
  printCatalog.documents.forEach((document, index) => {
    synthesized.push({ name: `📦️build🖨️print ${document.id}`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/print:build-${document.id}`, cwd: "${workspaceFolder}", presentation: { group: "4_build", order: Math.round((231 + index * 0.001) * 1000) / 1000 } });
    synthesized.push({ name: `🛠️dev🖨️print ${document.id}`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/print:watch-${document.id}`, cwd: "${workspaceFolder}", presentation: { group: "0_dev", order: Math.round((391 + index * 0.001) * 1000) / 1000 } });
  });
  [...components].sort((a, b) => a.pluginId.localeCompare(b.pluginId)).forEach((component, index) => {
    for (const [offset, target] of ["component-dev", "component-release", "materialize-dev", "materialize-release"].entries()) synthesized.push({
      name: `📦️build🧩️${component.pluginId}⚙️${target}`,
      type: "node-terminal",
      request: "launch",
      command: `bun nx run ${component.project}:${target}`,
      cwd: "${workspaceFolder}",
      presentation: { group: "4_build", order: Math.round((230 + index * 0.01 + offset * 0.001) * 1000) / 1000 },
    });
  });
  synthesized.push({ name: "🎮️build inputs", type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/framework-os-dev:build-inputs", cwd: "${workspaceFolder}", presentation: { group: "4_build", order: 349 } });
  const ordered = [...playgrounds].sort((left, right) => left.variant.localeCompare(right.variant));
  ordered.forEach((playground, index) => {
    synthesized.push({ name: `🎮️generate🧩️${playground.variant} session`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/plugin-registry:session-${playground.variant}`, cwd: "${workspaceFolder}", presentation: { group: "4_build", order: Math.round((350 + index * 0.001) * 1000) / 1000 } });
    for (const [offset, profile] of ["dev", "release"].entries()) synthesized.push({ name: `🎮️prepare🧩️${playground.variant}⚛️react ${profile}`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/framework-os-dev:prepare-${playground.variant}-react-${profile}`, cwd: "${workspaceFolder}", presentation: { group: "4_build", order: Math.round((360 + index * 0.01 + offset * 0.001) * 1000) / 1000 } });
    for (const [offset, profile] of ["dev", "release"].entries()) for (const [commandIndex, command] of ["prepare", "run", "smoke"].entries()) synthesized.push({
      name: `🎮️${command}🧩️${playground.variant}🖥️native ${profile}`, type: "node-terminal", request: "launch",
      command: `bun nx run @semio-tech/framework-os-dev:${command}-${playground.variant}-native-${profile}`, cwd: "${workspaceFolder}",
      presentation: { group: command === "prepare" ? "4_build" : command === "smoke" ? "3_test" : "0_dev", order: Math.round((365 + index * 0.01 + offset * 0.003 + commandIndex * 0.001) * 1000) / 1000 },
    });
    synthesized.push({ name: `🎮️build🧩️${playground.variant}⚛️react release`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/framework-os-dev:build-${playground.variant}-react-release`, cwd: "${workspaceFolder}", presentation: { group: "4_build", order: Math.round((362 + index * 0.001) * 1000) / 1000 } });
    for (const [offset, profile] of ["dev", "release"].entries()) synthesized.push({ name: `🎮️activate🧩️${playground.variant}⚛️react ${profile}`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/framework-os-dev:activate-${playground.variant}-react-${profile}`, cwd: "${workspaceFolder}", presentation: { group: "4_build", order: Math.round((361 + index * 0.01 + offset * 0.001) * 1000) / 1000 } });
    for (const [rendererIndex, renderer] of RENDERER_ROWS.entries()) for (const [offset, profile] of ["dev", "release"].entries()) for (const [commandIndex, command] of ["serve", "dev"].entries()) synthesized.push({ name: `🎮️${command}🧩️${playground.variant}${renderer.badge} ${profile}`, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/framework-os-dev:${command}-${playground.variant}-${renderer.id}-${profile}`, cwd: "${workspaceFolder}", presentation: { group: "0_dev", order: Math.round((380 + index * 0.1 + rendererIndex * 0.05 + offset * 0.01 + commandIndex * 0.001) * 1000) / 1000 } });
    const launcher = devLaunchers[playground.variant];
    const prefix = launcher?.namePrefix ?? `🧩️${playground.variant}`;
    const order = Math.round((390 + index * 0.01) * 1_000) / 1_000;
    const reactName = `🛠️dev${prefix}⚛️react`;
    const wgpuName = `🛠️dev${prefix}🧊️wgpu🌐️wasm`;
    const nativeName = `🛠️dev${prefix}🧊️wgpu🖥️native`;
    if (!out.includes(JSON.stringify(reactName))) synthesized.push(renderDiscoveredEntry(playground, prefix, "react", order, launcher?.command));
    if (!out.includes(JSON.stringify(wgpuName))) synthesized.push(renderDiscoveredEntry(playground, prefix, "wgpu", order + 0.001, launcher?.command));
    if (!out.includes(JSON.stringify(nativeName))) synthesized.push(renderDiscoveredEntry(playground, prefix, "native", order + 0.002));
  });
  if (synthesized.length > 0) {
    const marker = '\n  ],\n  "compounds":';
    if (!out.includes(marker)) throw new Error("🚀️launch/🟦️.ts: generated skeleton lacks the configurations/compounds boundary");
    out = out.replace(marker, `\n    ,\n${synthesized.map((entry) => reindent(JSON.stringify(entry, null, 2), 4)).join(",\n")}\n  ],\n  "compounds":`);
  }
  try {
    Bun.JSONC.parse(out);
  } catch {
    throw new Error("🚀️launch/🟦️.ts: generated launch output is not valid JSONC");
  }
  return out;
}
//#endregion
