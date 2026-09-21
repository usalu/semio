/**
 * 🖥️ Renders `.vscode/launch.json` from the hand-maintained seed (`.vscode/🧩️launch.seed.jsonc`) plus
 * the playground registry — the single source of truth for per-plugin dev-server ports. Never
 * hand-edit `.vscode/launch.json` directly: edit the seed file (for keyboard/mouse shortcuts, bespoke
 * tooling launchers, fixture/native variants, build/publish groups, and the `devLaunchers`
 * per-playground-variant presentation rows), or a plugin's `[[package.metadata.semio.playground]]`
 * block (for ports), then regenerate.
 *
 * 🔒️ Every playground variant gets exactly one `⚛️react` and one `🧊️wgpu🌐️wasm` dev launcher, whether
 * or not the seed curates it, and their command/port/env come from the registry entry alone — a seed
 * row can only choose the display name, the presentation order and extra env keys.
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
import { normalizeDevLaunchConfigurationNames, playgroundLaunchNamePrefix } from "./🏷️name-prefix/🟦️.ts";

const SEED_REL_PATH = ".vscode/🧩️launch.seed.jsonc";
/** @emoji 📄️ Repo-relative path of the generated output, shared with `📜️script.ts`'s freshness gate. */
export const LAUNCH_OUTPUT_REL_PATH = ".vscode/launch.json";
const DEV_LAUNCHERS_MARKER =
  ',\n\n  // 🎮️devLaunchers — per-playground-variant dev-launcher metadata (not part of the generated\n  // output); see 🚀️launch/🟦️.ts readSeed() for the exact split contract this marker line supports.\n  "devLaunchers": ';

//#region 🔖️DevLauncher
/** @emoji 🧩️ One `serverReadyAction` shape with a `"{PORT}"` token substituted at render time. */
type ServerReadyTemplate = { readonly pattern: string; readonly uriFormat: string };

/** @emoji 👥️ Multi-user expansion template for one playground variant's `@generated:<variant>:users`
 * placeholder: one launcher per registry `userPorts.react[]`/`userPorts.wgpu[]` slot (1-based `{N}`),
 * reusing the variant's own command/serverReadyAction and offsetting its `order`/`wgpuOrder` by
 * `0.01 * N`. `env` values may carry `"{N}"`, `"{PORT}"` and `"{EMAIL}"` tokens and are merged OVER
 * the registry-owned base env; `SEMIO_RENDERER` is set programmatically per renderer. */
type DevLauncherUsersTemplate = {
  readonly namePrefixPattern: string;
  readonly emailPattern: string;
  readonly env: Readonly<Record<string, string>>;
};

/** @emoji 🎮️ Hand-curated parts of one playground variant's `3_dev` launch entries that the plugin
 * registry cannot supply: display name and VS Code presentation order, plus any launcher-specific
 * `env` extras merged over the registry-owned base. Command, port, plugin/app/renderer env and the
 * `serverReadyAction` are all derived from the registry entry — a seed row can never drift from the
 * variant it launches. */
type DevLauncherEntry = {
  readonly namePrefix: string;
  readonly order: number;
  readonly wgpuOrder?: number;
  readonly env?: Readonly<Record<string, string>>;
  readonly users?: DevLauncherUsersTemplate;
};

/** @emoji 🌐️ The one `serverReadyAction` shape every playground dev server matches — Vite and the
 * wgpu Trunk server both print `http://<host>:<port>`, with `0.0.0.0` in a devcontainer. */
const DEV_SERVER_READY: ServerReadyTemplate = { pattern: "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):{PORT})", uriFormat: "%s" };

/** @emoji 🚀️ The `workspace:dev` invocation for one playground variant. `resolveFrameworkOsPlaygroundPlugin`
 * matches the variant id (or an alias) as the leading segment, so the variant id is always accepted. */
export function playgroundDevCommand(variant: string): string {
  return `bun nx run workspace:dev -- ${variant}`;
}

/** @emoji 🔌️ Registry-owned launch env for one variant+renderer. `SEMIO_PLUGIN` carries the **variant**,
 * exactly as `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts` and `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` read it, and
 * `S_OS_PORT` is the only port variable any dev server binds. */
export function playgroundDevEnv(playground: PlaygroundEntry, renderer: "react" | "wgpu", port: number): Record<string, string> {
  return { S_OS_PORT: String(port), SEMIO_PLUGIN: playground.variant, SEMIO_RENDERER: renderer, ...(playground.app ? { SEMIO_APP: playground.app } : {}) };
}
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
function renderEntry(name: string, launcher: DevLauncherEntry, playground: PlaygroundEntry, renderer: "react" | "wgpu", port: number): object {
  const order = renderer === "react" ? launcher.order : launcher.wgpuOrder;
  if (order === undefined) throw new Error(`🚀️launch/🟦️.ts: devLauncher "${name}" is missing its ${renderer} presentation order`);
  return {
    name,
    type: "node-terminal",
    request: "launch",
    command: playgroundDevCommand(playground.variant),
    cwd: "${workspaceFolder}",
    env: { ...playgroundDevEnv(playground, renderer, port), ...renderEnv(launcher.env ?? {}, port) },
    presentation: { group: "3_dev", order },
    serverReadyAction: renderServerReadyAction(DEV_SERVER_READY, port),
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
function renderUserEntry(users: DevLauncherUsersTemplate, playground: PlaygroundEntry, renderer: "react" | "wgpu", n: number, port: number, baseOrder: number, sra: ServerReadyTemplate): object {
  const email = substituteUserTokens(users.emailPattern, n, port, "");
  const namePrefix = substituteUserTokens(users.namePrefixPattern, n, port, email);
  const name = `🛠️dev${namePrefix}${renderer === "react" ? "⚛️react" : "🧊️wgpu🌐️wasm"}`;
  const env: Record<string, string> = playgroundDevEnv(playground, renderer, port);
  for (const [key, value] of Object.entries(users.env)) env[key] = substituteUserTokens(value, n, port, email);
  env.SEMIO_RENDERER = renderer;
  return {
    name,
    type: "node-terminal",
    request: "launch",
    command: playgroundDevCommand(playground.variant),
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
  playground.userPorts.react.forEach((port, index) => entries.push(renderUserEntry(users, playground, "react", index + 1, port, launcher.order, DEV_SERVER_READY)));
  if (launcher.wgpuOrder !== undefined) {
    playground.userPorts.wgpu.forEach((port, index) => entries.push(renderUserEntry(users, playground, "wgpu", index + 1, port, launcher.wgpuOrder!, DEV_SERVER_READY)));
  }
  return entries;
}

/** @emoji 🧩️ Supplies registry-owned dev launcher metadata when a variant has no curated seed row, or
 * when a curated row covers only one of the two browser renderers. */
function defaultDevLauncher(playground: PlaygroundEntry, order: number, repoRoot: string, playgrounds: readonly PlaygroundEntry[]): DevLauncherEntry {
  return { namePrefix: playgroundLaunchNamePrefix(playground, repoRoot, playgrounds), order, wgpuOrder: Math.round((order + 0.001) * 1000) / 1000 };
}
//#endregion

//#region 🔖️Generate
/** @emoji 🔒️ Every variant must own its launch name: the synthesis pass below adds a launcher only
 * when the skeleton does not already carry that name, so two variants resolving to the same
 * {@link playgroundLaunchNamePrefix} would leave the second one with NO dev launcher at all. Refuse
 * loudly here instead of emitting a `launch.json` that silently drops a playground. */
function assertDistinctLaunchNamePrefixes(playgrounds: readonly PlaygroundEntry[], repoRoot: string): void {
  const owners = new Map<string, string>();
  for (const playground of playgrounds) {
    const prefix = playgroundLaunchNamePrefix(playground, repoRoot, playgrounds);
    const owner = owners.get(prefix);
    if (owner !== undefined) throw new Error(`🚀️launch/🟦️.ts: playground variants ${JSON.stringify(owner)} and ${JSON.stringify(playground.variant)} both resolve to the launch name prefix ${JSON.stringify(prefix)} — one of them would get no dev launcher`);
    owners.set(prefix, playground.variant);
  }
}

/** @emoji 🏗️ Renders the full `.vscode/launch.json` text: seed skeleton with every
 * `@generated:<variant>:<renderer>` placeholder substituted by a fresh, registry-ported entry. */
export function generateLaunchJson(repoRoot: string, playgrounds: readonly PlaygroundEntry[], _components: readonly { project: string; pluginId: string }[], readText?: (path: string) => string): string {
  const { skeleton, devLaunchers } = readSeed(repoRoot, readText);
  const byVariant = new Map(playgrounds.map((entry) => [entry.variant, entry]));
  assertDistinctLaunchNamePrefixes(playgrounds, repoRoot);
  let out = skeleton;
  for (const [variant, launcher] of Object.entries(devLaunchers)) {
    const playground = byVariant.get(variant);
    if (!playground) throw new Error(`🚀️launch/🟦️.ts: devLaunchers["${variant}"] has no matching playground registry entry (renamed or removed plugin — update the seed)`);
    const namePrefix = playgroundLaunchNamePrefix(playground, repoRoot, playgrounds);
    const reactPlaceholder = JSON.stringify(`@generated:${variant}:react`);
    if (!out.includes(reactPlaceholder)) throw new Error(`🚀️launch/🟦️.ts: seed is missing placeholder ${reactPlaceholder}`);
    const reactName = `🛠️dev${namePrefix}⚛️react`;
    out = out.replace(reactPlaceholder, reindent(JSON.stringify(renderEntry(reactName, launcher, playground, "react", playground.ports.react), null, 2), 4));
    if (launcher.wgpuOrder !== undefined) {
      const wgpuPlaceholder = JSON.stringify(`@generated:${variant}:wgpu`);
      if (!out.includes(wgpuPlaceholder)) throw new Error(`🚀️launch/🟦️.ts: seed is missing placeholder ${wgpuPlaceholder}`);
      const wgpuName = `🛠️dev${namePrefix}🧊️wgpu🌐️wasm`;
      out = out.replace(wgpuPlaceholder, reindent(JSON.stringify(renderEntry(wgpuName, launcher, playground, "wgpu", playground.ports.wgpu), null, 2), 4));
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
  out = refreshDevLaunchNames(out, playgrounds, repoRoot);
  const synthesized: object[] = [];
  for (const [index, playground] of [...playgrounds].sort((left, right) => left.variant.localeCompare(right.variant)).entries()) {
    const curated = devLaunchers[playground.variant];
    const fallback = defaultDevLauncher(playground, curated?.order ?? Math.round((420 + index * 0.01) * 1000) / 1000, repoRoot, playgrounds);
    const launcher = curated ? { ...fallback, ...curated, wgpuOrder: curated.wgpuOrder ?? fallback.wgpuOrder } : fallback;
    const reactName = `🛠️dev${fallback.namePrefix}⚛️react`;
    const wgpuName = `🛠️dev${fallback.namePrefix}🧊️wgpu🌐️wasm`;
    if (!out.includes(JSON.stringify(reactName))) synthesized.push(renderEntry(reactName, launcher, playground, "react", playground.ports.react));
    if (!out.includes(JSON.stringify(wgpuName))) synthesized.push(renderEntry(wgpuName, launcher, playground, "wgpu", playground.ports.wgpu));
  }
  if (synthesized.length > 0) {
    const marker = '\n  ],\n  "compounds":';
    if (!out.includes(marker)) throw new Error("🚀️launch/🟦️.ts: generated skeleton lacks the configurations/compounds boundary");
    out = out.replace(marker, `\n    ,\n${synthesized.map((entry) => reindent(JSON.stringify(entry, null, 2), 4)).join(",\n")}\n  ],\n  "compounds":`);
  }
  out = refreshDevLaunchNames(out, playgrounds, repoRoot);
  try {
    Bun.JSONC.parse(out);
  } catch {
    throw new Error("🚀️launch/🟦️.ts: generated launch output is not valid JSONC");
  }
  return out;
}
//#endregion

function refreshDevLaunchNames(out: string, playgrounds: readonly PlaygroundEntry[], repoRoot: string): string {
  const parsed = Bun.JSONC.parse(out) as { readonly configurations: readonly { readonly name?: string }[] };
  const normalized = normalizeDevLaunchConfigurationNames([...parsed.configurations], playgrounds, repoRoot);
  for (let index = 0; index < parsed.configurations.length; index++) {
    const previous = parsed.configurations[index]?.name;
    const next = (normalized[index] as { readonly name?: string }).name;
    if (!previous || !next || previous === next) continue;
    const needle = `"name": ${JSON.stringify(previous)}`;
    const replacement = `"name": ${JSON.stringify(next)}`;
    const at = out.indexOf(needle);
    if (at === -1) throw new Error(`🚀️launch/🟦️.ts: could not locate launch name ${JSON.stringify(previous)} for emoji refresh`);
    out = out.slice(0, at) + replacement + out.slice(at + needle.length);
  }
  return out;
}
