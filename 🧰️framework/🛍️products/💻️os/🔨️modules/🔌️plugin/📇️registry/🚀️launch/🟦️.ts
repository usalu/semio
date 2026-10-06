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
import { assertLaunchSeedPlacement } from "./🧱️placement/🟦️.ts";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { PlaygroundEntry } from "../🎮️playground/🔎️discovery/🟦️.ts";
import { normalizeDevLaunchConfigurationNames, playgroundLaunchNamePrefix, taxonomyFolderSlug } from "./🏷️name-prefix/🟦️.ts";

const SEED_REL_PATH = ".vscode/🧩️launch.seed.jsonc";
/** 📄️ Repo-relative path of the generated output, shared with `📜️script.ts`'s freshness gate. */
export const LAUNCH_OUTPUT_REL_PATH = ".vscode/launch.json";

//#region 🔖️DevLauncher
/** 🧩️ One `serverReadyAction` shape with a `"{PORT}"` token substituted at render time. */
type ServerReadyTemplate = { readonly pattern: string; readonly uriFormat: string };

/** 👥️ Multi-user expansion template for one playground variant's `@generated:<variant>:users`
 * placeholder: one launcher per registry `userPorts.react[]`/`userPorts.wgpu[]` slot (1-based `{N}`),
 * reusing the variant's own command/serverReadyAction and offsetting its `order`/`wgpuOrder` by
 * `0.01 * N`. `env` values may carry `"{N}"`, `"{PORT}"` and `"{EMAIL}"` tokens and are merged OVER
 * the registry-owned base env; `SEMIO_RENDERER` is set programmatically per renderer. */
type DevLauncherUsersTemplate = {
  readonly namePrefixPattern: string;
  readonly emailPattern: string;
  readonly env: Readonly<Record<string, string>>;
};

/** 🎮️ Hand-curated parts of one playground variant's `3_dev` launch entries that the plugin
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

/** 🌐️ The one `serverReadyAction` shape every playground dev server matches — Vite and the
 * wgpu Trunk server both print `http://<host>:<port>`, with `0.0.0.0` in a devcontainer. */
const DEV_SERVER_READY: ServerReadyTemplate = { pattern: "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):{PORT})", uriFormat: "%s" };

/** 🚀️ The `workspace:dev` invocation for one playground variant. `resolveFrameworkOsPlaygroundPlugin`
 * matches the variant id (or an alias) as the leading segment, so the variant id is always accepted. */
export function playgroundDevCommand(variant: string): string {
  return `bun nx run workspace:dev -- ${variant}`;
}

/** 🔌️ Registry-owned launch env for one variant+renderer. `SEMIO_PLUGIN` carries the **variant**,
 * exactly as `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts` and `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` read it, and
 * `S_OS_PORT` is the only port variable any dev server binds. */
export function playgroundDevEnv(playground: PlaygroundEntry, renderer: "react" | "wgpu", port: number): Record<string, string> {
  return { S_OS_PORT: String(port), SEMIO_PLUGIN: playground.variant, SEMIO_RENDERER: renderer, ...(playground.app ? { SEMIO_APP: playground.app } : {}) };
}
//#endregion

//#region 🔖️SeedSplit
/** ✂️ Projects launch declarations from parsed seed metadata independently of source formatting. */
function readSeed(repoRoot: string, readText?: (path: string) => string): { readonly skeleton: string; readonly devLaunchers: Readonly<Record<string, DevLauncherEntry>>; readonly projectLaunchers?: ProjectLauncherPolicy } {
  const seedPath = join(repoRoot, SEED_REL_PATH);
  const raw = readText ? readText(SEED_REL_PATH) : readFileSync(seedPath, "utf8");
  let document: Record<string, unknown> & { readonly devLaunchers?: Record<string, DevLauncherEntry>; readonly projectLaunchers?: ProjectLauncherPolicy };
  try {
    document = Bun.JSONC.parse(raw) as typeof document;
  } catch {
    throw new Error(`🚀️launch/🟦️.ts: seed file ${seedPath} is not valid JSONC`);
  }
  assertLaunchSeedPlacement(document);
  const { devLaunchers, projectLaunchers, ...launch } = document;
  if (!devLaunchers) throw new Error(`🚀️launch/🟦️.ts: seed file ${seedPath} has no devLaunchers metadata`);
  return { skeleton: JSON.stringify(launch, null, 2), devLaunchers, ...(projectLaunchers ? { projectLaunchers } : {}) };
}
//#endregion

/** ➕️ Appends generated declarations through owned JSONC container offsets. */
function appendLaunchEntries(source: string, name: "configurations" | "inputs", rows: readonly object[]): string {
  if (rows.length === 0) return source;
  const container = launchContainerRanges(source)[name];
  if (!container) throw new Error(`🚀️launch/🟦️.ts: generated skeleton lacks the ${name} container`);
  const at = container.range.end - 1;
  const addition = `${container.elements.length ? "," : ""}\n${rows.map((row) => reindent(JSON.stringify(row, null, 2), 4)).join(",\n")}\n  `;
  return source.slice(0, at) + addition + source.slice(at);
}

//#region 🔖️Render
function renderEnv(template: Readonly<Record<string, string>>, port: number): Record<string, string> {
  const out: Record<string, string> = {};
  for (const [key, value] of Object.entries(template)) out[key] = value === "{PORT}" ? String(port) : value;
  return out;
}

function renderServerReadyAction(template: ServerReadyTemplate, port: number): { action: string; pattern: string; uriFormat: string } {
  return { action: "openExternally", pattern: template.pattern.replaceAll("{PORT}", String(port)), uriFormat: template.uriFormat.replaceAll("{PORT}", String(port)) };
}

/** 🧱️ Builds one `3_dev` launch config object for a variant+renderer, matching the field order
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

/** ↔️ Re-indents a `JSON.stringify(obj, null, 2)` block (0-based) to sit at the seed's 4-space
 * `configurations` array-item depth; only line 1 needs no shift since it replaces an inline placeholder. */
function reindent(jsonText: string, extraSpaces: number): string {
  const pad = " ".repeat(extraSpaces);
  return jsonText
    .split("\n")
    .map((line, i) => (i === 0 ? line : pad + line))
    .join("\n");
}

/** 🔤️ Substitutes the `users` template's `"{N}"` / `"{PORT}"` / `"{EMAIL}"` tokens in `text`. */
function substituteUserTokens(text: string, n: number, port: number, email: string): string {
  return text.replaceAll("{N}", String(n)).replaceAll("{PORT}", String(port)).replaceAll("{EMAIL}", email);
}

/** 👥️ Renders one `users` launcher for user slot `n` (1-based) of one renderer, reusing the
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

/** 👥️ Renders every launcher for one variant's `@generated:<variant>:users` placeholder: one
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

/** 🧩️ Supplies registry-owned dev launcher metadata when a variant has no curated seed row, or
 * when a curated row covers only one of the two browser renderers. */
function defaultDevLauncher(playground: PlaygroundEntry, order: number, repoRoot: string, playgrounds: readonly PlaygroundEntry[]): DevLauncherEntry {
  return { namePrefix: playgroundLaunchNamePrefix(playground, repoRoot, playgrounds), order, wgpuOrder: Math.round((order + 0.001) * 1000) / 1000 };
}
//#endregion

//#region 🔖️ProjectTargets
/** 📋️ One project's declared nx targets (the keys of its `📋️project.json` `targets`). A declared target is an executable
 * command a dev runs, so each one gets a launch row; targets the nx plugins only infer (`describe`, `component-*`,
 * `materialize-*`, inferred test levels, `nx-release-publish`) are pipeline steps reached through declared targets and
 * aggregates, never registered on their own. */
export type DeclaredProjectTargets = { readonly project: string; readonly path: string; readonly targets: readonly string[] };

/** 🧭️ One verb class of the seed's `projectLaunchers.classes`: name emoji, VS Code group, first order, and the target-name
 * tokens that select it (a target name's first token, left to right, that some class lists decides). */
type ProjectLauncherClass = { readonly id: string; readonly emoji: string; readonly group: string; readonly orderBase: number; readonly tokens: readonly string[] };

/** 📐️ Seed-owned rules (`projectLaunchers` in `🧩️launch.seed.jsonc`) that turn declared targets into launch rows: a target
 * name declared by at least `familyMinimumProjects` projects is one family row with a project picker input; every other
 * declared target not already run by a curated seed row gets its own row named `<class emoji><target><project label>`. */
type ProjectLauncherPolicy = {
  readonly familyMinimumProjects: number;
  readonly familyEmoji: string;
  readonly classes: readonly ProjectLauncherClass[];
  readonly fallbackClass: string;
  readonly languageSegments: Readonly<Record<string, string>>;
  readonly transparentSegments: readonly string[];
  readonly skipDirectories: readonly string[];
};

const PROJECT_MANIFEST = "📋️project.json";

/** 🗂️ The read-only tree the project walk reads — structurally the registry's `RegistryCatalogInputView`, so a generator
 * preview sees the projected (post-operation) manifests, and the live filesystem otherwise. */
export type ProjectTargetTreeView = {
  entries(path: string): readonly { readonly name: string; readonly nodeKind: "file" | "directory" | "symlink" }[];
  readText(path: string): string;
};

function filesystemTreeView(repoRoot: string): ProjectTargetTreeView {
  return {
    entries: (path) => readdirSync(join(repoRoot, path), { withFileTypes: true }).map((entry) => ({ name: entry.name, nodeKind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : "file" })),
    readText: (path) => readFileSync(join(repoRoot, path), "utf8"),
  };
}

/** 🔎️ Reads every declared project target (skipping the seed's `skipDirectories`, hidden directories and symlinks), sorted
 * by project name so the rendered rows are byte-stable. */
export function declaredProjectTargets(repoRoot: string, view: ProjectTargetTreeView = filesystemTreeView(repoRoot)): DeclaredProjectTargets[] {
  const policy = readSeed(repoRoot, (path) => view.readText(path)).projectLaunchers;
  if (!policy) throw new Error("🚀️launch/🟦️.ts: seed has no projectLaunchers policy");
  const skip = new Set(policy.skipDirectories);
  const found: DeclaredProjectTargets[] = [];
  const walk = (relative: string): void => {
    const entries = view.entries(relative);
    if (entries.some((entry) => entry.nodeKind === "file" && entry.name === PROJECT_MANIFEST)) {
      const manifest = relative ? `${relative}/${PROJECT_MANIFEST}` : PROJECT_MANIFEST;
      let parsed: { readonly name?: string; readonly targets?: Readonly<Record<string, unknown>> };
      try {
        parsed = JSON.parse(view.readText(manifest)) as typeof parsed;
      } catch {
        throw new Error(`🚀️launch/🟦️.ts: ${manifest} is not valid JSON`);
      }
      if (parsed.name) found.push({ project: parsed.name, path: relative, targets: Object.keys(parsed.targets ?? {}).sort() });
    }
    for (const entry of entries) if (entry.nodeKind === "directory" && !entry.name.startsWith(".") && !skip.has(entry.name)) walk(relative ? `${relative}/${entry.name}` : entry.name);
  };
  walk("");
  return found.sort((left, right) => left.project.localeCompare(right.project));
}

/** 🏷️ The shortest trailing run of a project's path segments (language folders shortened to their emoji, transparent
 * containers dropped) that no other project shares — `🌎️hub🦀️`, `💻️os🦀️`, `🪐️space🟦️`; the root project has none. */
function projectLaunchLabels(projects: readonly DeclaredProjectTargets[], policy: ProjectLauncherPolicy): Map<string, string> {
  const segments = new Map(projects.map((project) => [project.project, project.path === "" ? [] : project.path.split("/").filter((segment) => !policy.transparentSegments.includes(segment)).map((segment) => policy.languageSegments[segment] ?? segment)]));
  const labels = new Map<string, string>();
  for (const [project, own] of segments) {
    let label = own.join("");
    for (let length = 1; length <= own.length; length++) {
      const suffix = own.slice(-length).join("");
      if ([...segments].every(([other, theirs]) => other === project || theirs.slice(-length).join("") !== suffix)) {
        label = suffix;
        break;
      }
    }
    labels.set(project, label);
  }
  return labels;
}

/** 🧭️ The verb class of one target name: its first token (emoji stripped, split on `-`) that a class lists, else the fallback. */
function projectLauncherClass(target: string, policy: ProjectLauncherPolicy): ProjectLauncherClass {
  for (const token of taxonomyFolderSlug(target).split("-")) {
    const found = policy.classes.find((entry) => entry.tokens.includes(token));
    if (found) return found;
  }
  const fallback = policy.classes.find((entry) => entry.id === policy.fallbackClass);
  if (!fallback) throw new Error(`🚀️launch/🟦️.ts: projectLaunchers.fallbackClass ${JSON.stringify(policy.fallbackClass)} names no class`);
  return fallback;
}

/** 🧱️ Renders the family rows (with their picker inputs) and one row per uncovered declared target. A target is covered when a
 * row already in `launchText` runs `nx run <project>:<target>` — the curated seed rows keep their names and places. */
function renderProjectTargetLaunchers(launchText: string, projects: readonly DeclaredProjectTargets[], policy: ProjectLauncherPolicy): { readonly rows: readonly object[]; readonly inputs: readonly object[] } {
  const existing = Bun.JSONC.parse(launchText) as { readonly configurations: readonly unknown[] };
  const covered = new Set<string>();
  const names = new Set<string>();
  for (const row of existing.configurations) {
    if (typeof row !== "object" || row === null) continue;
    const { name, command } = row as { readonly name?: string; readonly command?: string };
    if (name) names.add(name);
    for (const match of String(command ?? "").matchAll(/nx run ([^\s:$]+):(\S+)/gu)) covered.add(`${match[1]}:${match[2]}`);
  }
  const declaring = new Map<string, string[]>();
  for (const project of projects) for (const target of project.targets) declaring.set(target, [...(declaring.get(target) ?? []), project.project]);
  const families = [...declaring].filter(([, owners]) => owners.length >= policy.familyMinimumProjects).sort(([left], [right]) => left.localeCompare(right));
  const familyTargets = new Set(families.map(([target]) => target));
  const labels = projectLaunchLabels(projects, policy);
  const claim = (name: string): string => {
    if (names.has(name)) throw new Error(`🚀️launch/🟦️.ts: generated launch name ${JSON.stringify(name)} collides with an existing row`);
    names.add(name);
    return name;
  };
  const counters = new Map<string, number>();
  const order = (entry: ProjectLauncherClass, offset: number): number => {
    const index = counters.get(entry.id) ?? 0;
    counters.set(entry.id, index + 1);
    return Math.round((entry.orderBase + offset + index / 10000) * 10000) / 10000;
  };
  const inputs: object[] = [];
  const rows: object[] = [];
  for (const [target, owners] of families) {
    const entry = projectLauncherClass(target, policy);
    const id = `projectTarget.${taxonomyFolderSlug(target)}`;
    inputs.push({ id, type: "pickString", description: `Project whose ${target} target runs`, options: [...owners].sort() });
    rows.push({ name: claim(`${entry.emoji}${target}${policy.familyEmoji}`), type: "node-terminal", request: "launch", command: `bun nx run \${input:${id}}:${target}`, cwd: "${workspaceFolder}", presentation: { group: entry.group, order: order(entry, -1) } });
  }
  counters.clear();
  const pending = projects
    .flatMap((project) => project.targets.filter((target) => !familyTargets.has(target) && !covered.has(`${project.project}:${target}`)).map((target) => ({ project: project.project, target, label: labels.get(project.project) ?? "" })))
    .sort((left, right) => left.label.localeCompare(right.label) || left.target.localeCompare(right.target));
  for (const { project, target, label } of pending) {
    const entry = projectLauncherClass(target, policy);
    rows.push({ name: claim(`${entry.emoji}${target}${label}`), type: "node-terminal", request: "launch", command: `bun nx run ${project}:${target}`, cwd: "${workspaceFolder}", presentation: { group: entry.group, order: order(entry, 0) } });
  }
  return { rows, inputs };
}
//#endregion

//#region 🔖️Generate
/** 🔒️ Every variant must own its launch name: the synthesis pass below adds a launcher only
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

/** 🏗️ Renders the full `.vscode/launch.json` text: seed skeleton with every
 * `@generated:<variant>:<renderer>` placeholder substituted by a fresh, registry-ported entry. */
export function generateLaunchJson(repoRoot: string, playgrounds: readonly PlaygroundEntry[], projectTargets: readonly DeclaredProjectTargets[], readText?: (path: string) => string): string {
  const { skeleton, devLaunchers, projectLaunchers } = readSeed(repoRoot, readText);
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
  out = appendLaunchEntries(out, "configurations", synthesized);
  out = refreshDevLaunchNames(out, playgrounds, repoRoot);
  if (projectTargets.length > 0) {
    if (!projectLaunchers) throw new Error("🚀️launch/🟦️.ts: seed has no projectLaunchers policy for the declared project targets");
    const { rows, inputs } = renderProjectTargetLaunchers(out, projectTargets, projectLaunchers);
    out = appendLaunchEntries(out, "inputs", inputs);
    out = appendLaunchEntries(out, "configurations", rows);
  }
  try {
    out = JSON.stringify(Bun.JSONC.parse(out), null, 2) + "\n";
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

//#region 🔖️Reconcile
type LaunchRow = Record<string, unknown>;
type TextRange = { readonly start: number; readonly end: number };
type LaunchContainer = { readonly range: TextRange; readonly elements: readonly TextRange[] };
type TextEdit = TextRange & { readonly text: string };

/** 🧷️ What one reconciliation moved into the seed and why every other unrendered launch row stayed out: `adopted` rows and
 * `adoptedInputs` exist only in the launch file, `edited` seed rows carry other content there (taken only on request, because
 * an unrendered seed edit looks the same), `drifted` generated rows differ and cannot be seeded, `renamed` bodies are rendered
 * under another name, `stale` rows are generator output the registry no longer produces, `misplaced` rows left `inputs`. */
export type LaunchSeedReconciliation = {
  readonly seed: string;
  readonly adopted: readonly string[];
  readonly adoptedInputs: readonly string[];
  readonly edited: readonly string[];
  readonly drifted: readonly string[];
  readonly renamed: readonly string[];
  readonly stale: readonly string[];
  readonly misplaced: number;
};

/** 🧭️ Locates every top-level array of a JSONC document and its elements, so launch rows move between files as bytes. */
export function launchContainerRanges(text: string): Readonly<Record<string, LaunchContainer>> {
  const skip = (at: number): number => {
    for (;;) {
      if (text.startsWith("//", at)) at = text.indexOf("\n", at) === -1 ? text.length : text.indexOf("\n", at);
      else if (text.startsWith("/*", at) && text.indexOf("*/", at + 2) !== -1) at = text.indexOf("*/", at + 2) + 2;
      else if (at < text.length && text.charCodeAt(at) <= 32) at++;
      else return at;
    }
  };
  const value = (at: number, visit?: (key: string | undefined, range: TextRange) => void): number => {
    const head = text[at];
    if (head === '"') {
      let end = at + 1;
      while (end < text.length && text[end] !== '"') end += text[end] === "\\" ? 2 : 1;
      if (end >= text.length) throw new Error(`🚀️launch/🟦️.ts: unterminated launch string at offset ${at}`);
      return end + 1;
    }
    if (head !== "{" && head !== "[") {
      let end = at;
      while (end < text.length && !/[\s,\]}/]/u.test(text[end]!)) end++;
      if (end === at) throw new Error(`🚀️launch/🟦️.ts: unexpected launch text at offset ${at}`);
      return end;
    }
    const close = head === "{" ? "}" : "]";
    let next = skip(at + 1);
    while (text[next] !== close) {
      let key: string | undefined;
      if (head === "{") {
        const keyEnd = value(next);
        key = JSON.parse(text.slice(next, keyEnd)) as string;
        next = skip(keyEnd);
        if (text[next] !== ":") throw new Error(`🚀️launch/🟦️.ts: launch member without a value at offset ${next}`);
        next = skip(next + 1);
      }
      const end = value(next);
      visit?.(key, { start: next, end });
      next = skip(end);
      if (text[next] === ",") next = skip(next + 1);
      else if (text[next] !== close) throw new Error(`🚀️launch/🟦️.ts: launch container is not closed at offset ${next}`);
    }
    return next + 1;
  };
  const containers: Record<string, LaunchContainer> = {};
  value(skip(0), (key, range) => {
    if (key === undefined || text[range.start] !== "[") return;
    const elements: TextRange[] = [];
    value(range.start, (_key, element) => elements.push(element));
    containers[key] = { range, elements };
  });
  return containers;
}

/** ✍️ Applies non-overlapping byte-range edits; everything outside them stays byte-identical. */
function applyTextEdits(text: string, edits: readonly TextEdit[]): string {
  let out = text;
  for (const edit of [...edits].sort((left, right) => right.start - left.start || right.end - left.end)) out = out.slice(0, edit.start) + edit.text + out.slice(edit.end);
  return out;
}

/** 🧬️ Identifies a launcher by what it runs, ignoring its name and menu placement. */
const launchRowBody = (row: LaunchRow): string => JSON.stringify(Object.fromEntries(Object.entries(row).filter(([key]) => key !== "name" && key !== "presentation")));
const isLaunchRow = (value: unknown): value is LaunchRow => typeof value === "object" && value !== null && !Array.isArray(value);
const AUTO_ROW_KEYS = "command,cwd,name,presentation,request,type";

/** 🧹️ Removes configuration rows that sit in the seed's `inputs` container; each must deep-equal a row `configurations` keeps. */
function withoutMisplacedSeedInputs(seed: string): { readonly seed: string; readonly misplaced: number } {
  const document = Bun.JSONC.parse(seed) as { readonly configurations?: readonly unknown[]; readonly inputs?: readonly unknown[] };
  const inputs = document.inputs ?? [];
  const misplaced = inputs.map((row, index) => (isLaunchRow(row) && !Object.hasOwn(row, "id") ? index : -1)).filter((index) => index !== -1);
  if (misplaced.length === 0) return { seed, misplaced: 0 };
  const twins = new Set((document.configurations ?? []).filter(isLaunchRow).map((row) => JSON.stringify(row)));
  const elements = launchContainerRanges(seed).inputs!.elements;
  const edits: TextEdit[] = [];
  let kept = false;
  inputs.forEach((row, index) => {
    if (!misplaced.includes(index)) { kept = true; return; }
    if (!twins.has(JSON.stringify(row))) throw new Error(`🚀️launch/🟦️.ts: seed inputs[${index}] is a configuration row without a twin in configurations — move it by hand`);
    edits.push(kept ? { start: elements[index - 1]!.end, end: elements[index]!.end, text: "" } : { start: elements[index]!.start, end: elements[index + 1]?.start ?? elements[index]!.end, text: "" });
  });
  return { seed: applyTextEdits(seed, edits), misplaced: misplaced.length };
}

/** 🔎️ Classifies every launch row the render does not carry verbatim and plans the byte-range seed edits that keep them. */
function planLaunchSeedReconciliation(seed: string, launch: string, rendered: string, adoptEdits: boolean): Omit<LaunchSeedReconciliation, "seed" | "misplaced"> & { readonly edits: readonly TextEdit[] } {
  const seedDocument = Bun.JSONC.parse(seed) as { readonly configurations: readonly unknown[]; readonly inputs?: readonly unknown[] };
  const seedRanges = launchContainerRanges(seed);
  const launchDocument = Bun.JSONC.parse(launch) as { readonly configurations: readonly unknown[]; readonly inputs?: readonly unknown[] };
  const launchRanges = launchContainerRanges(launch);
  const renderedDocument = Bun.JSONC.parse(rendered) as { readonly configurations: readonly LaunchRow[]; readonly inputs?: readonly LaunchRow[] };
  const renderedRows = new Map(renderedDocument.configurations.map((row) => [row.name, row]));
  const renderedBodies = new Set(renderedDocument.configurations.map(launchRowBody));
  const renderedCommands = renderedDocument.configurations.map((row) => String(row.command ?? ""));
  const renderedTargets = new Set(renderedCommands.flatMap((command) => [...command.matchAll(/nx run ([^\s:$]+):(\S+)/gu)].map((match) => `${match[1]}:${match[2]}`)));
  const renderedVariants = new Set(renderedCommands.flatMap((command) => [...command.matchAll(/workspace:dev -- (\S+)/gu)].map((match) => match[1])));
  const renderedInputs = new Set((renderedDocument.inputs ?? []).map((row) => row.id));
  const owned = new Map<unknown, TextRange>();
  seedDocument.configurations.forEach((row, index) => { if (isLaunchRow(row)) owned.set(row.name, seedRanges.configurations!.elements[index]!); });
  /** 🏭️ Recognizes generator output the registry stopped producing: a family row, a declared-target row another row now runs, a dev row of a removed variant. */
  const isStale = (row: LaunchRow): boolean => {
    const command = String(row.command ?? "");
    const target = Object.keys(row).sort().join(",") === AUTO_ROW_KEYS ? command.match(/^bun nx run (\$\{input:projectTarget\.[^}]+\}|[^\s:$]+):(\S+)$/u) : null;
    if (target) return target[1]!.startsWith("$") || renderedTargets.has(`${target[1]}:${target[2]}`);
    const variant = Object.hasOwn(row, "serverReadyAction") ? command.match(/^bun nx run workspace:dev -- (\S+)$/u) : null;
    return variant !== null && !renderedVariants.has(variant[1]);
  };
  const adopted: string[] = [], edited: string[] = [], drifted: string[] = [], renamed: string[] = [], stale: string[] = [];
  const edits: TextEdit[] = [];
  const insertions = new Map<number, string[]>();
  const seen = new Map<unknown, string>();
  let anchor: TextRange | undefined;
  launchDocument.configurations.forEach((row, index) => {
    if (!isLaunchRow(row) || typeof row.name !== "string") return;
    const text = launch.slice(launchRanges.configurations!.elements[index]!.start, launchRanges.configurations!.elements[index]!.end);
    const twin = renderedRows.get(row.name);
    if (twin) {
      const range = owned.get(row.name);
      if (range) anchor = range;
      if (JSON.stringify(twin) === JSON.stringify(row) || seen.has(row.name)) return;
      seen.set(row.name, JSON.stringify(row));
      if (!range) { if (launchRowBody(twin) !== launchRowBody(row)) drifted.push(row.name); return; }
      edited.push(row.name);
      if (adoptEdits) edits.push({ ...range, text });
      return;
    }
    if (seen.has(row.name)) {
      if (seen.get(row.name) !== JSON.stringify(row)) throw new Error(`🚀️launch/🟦️.ts: launch file carries two different rows named ${JSON.stringify(row.name)}`);
      return;
    }
    seen.set(row.name, JSON.stringify(row));
    if (renderedBodies.has(launchRowBody(row))) { renamed.push(row.name); return; }
    if (isStale(row)) { stale.push(row.name); return; }
    adopted.push(row.name);
    const at = anchor?.end ?? -1;
    insertions.set(at, [...(insertions.get(at) ?? []), text]);
  });
  /** 📍️ Appends rows after one element, or opens a container that holds none yet. */
  const place = (container: LaunchContainer | undefined, name: string, at: number, rows: readonly string[]): TextEdit => {
    if (!container) throw new Error(`🚀️launch/🟦️.ts: seed has no ${name} container for ${rows.length} launch row(s)`);
    if (at !== -1) return { start: at, end: at, text: rows.map((row) => `,\n    ${row}`).join("") };
    const first = container.elements[0];
    if (first) return { start: first.start, end: first.start, text: rows.map((row) => `${row},\n    `).join("") };
    return { start: container.range.start + 1, end: container.range.end - 1, text: `\n    ${rows.join(",\n    ")}\n  ` };
  };
  for (const [at, rows] of insertions) edits.push(place(seedRanges.configurations, "configurations", at, rows));
  const adoptedInputs: string[] = [], inputTexts: string[] = [];
  (launchDocument.inputs ?? []).forEach((row, index) => {
    if (!isLaunchRow(row) || typeof row.id !== "string" || renderedInputs.has(row.id) || adoptedInputs.includes(row.id)) return;
    if (row.id.startsWith("projectTarget.")) { stale.push(row.id); return; }
    adoptedInputs.push(row.id);
    inputTexts.push(launch.slice(launchRanges.inputs!.elements[index]!.start, launchRanges.inputs!.elements[index]!.end));
  });
  if (inputTexts.length > 0) edits.push(place(seedRanges.inputs, "inputs", seedRanges.inputs?.elements.at(-1)?.end ?? -1, inputTexts));
  return { adopted, adoptedInputs, edited, drifted, renamed, stale, edits };
}

/** 🧷️ Keeps hand-made launch rows across a render: rows that exist only in the launch file move into the seed as the bytes
 * they were written in (after the nearest preceding row the seed owns), unrendered inputs follow, and with `adoptEdits` a seed
 * row takes the content its launch twin carries. Nothing else in the seed changes, and the result must be a fixpoint: a second
 * pass over the re-rendered seed finds nothing left to move. `render` is the launch producer for a given seed text. */
export function reconcileLaunchSeed(seed: string, launch: string, render: (seed: string) => string, adoptEdits = false): LaunchSeedReconciliation {
  const repaired = withoutMisplacedSeedInputs(seed);
  const { edits, ...report } = planLaunchSeedReconciliation(repaired.seed, launch, render(repaired.seed), adoptEdits);
  const next = applyTextEdits(repaired.seed, edits);
  if (next !== seed) {
    assertLaunchSeedPlacement(Bun.JSONC.parse(next));
    const settled = planLaunchSeedReconciliation(next, launch, render(next), adoptEdits);
    const open = [...settled.adopted, ...settled.adoptedInputs, ...(adoptEdits ? settled.edited : [])];
    if (settled.edits.length > 0 || open.length > 0) throw new Error(`🚀️launch/🟦️.ts: launch seed reconciliation did not settle: ${open.join(", ")}`);
  }
  return { ...report, seed: next, misplaced: repaired.misplaced };
}
/** 🖥️ Reconciles the repository seed against one launch file through the real producer; the project walk runs once. */
export function reconcileRepositoryLaunchSeed(repoRoot: string, playgrounds: readonly PlaygroundEntry[], launch: string, adoptEdits: boolean): LaunchSeedReconciliation & { readonly path: string; readonly previous: string } {
  const path = join(repoRoot, SEED_REL_PATH), previous = readFileSync(path, "utf8"), files = filesystemTreeView(repoRoot);
  let targets: DeclaredProjectTargets[] | undefined;
  const render = (seed: string): string => {
    const readText = (file: string): string => (file === SEED_REL_PATH ? seed : files.readText(file));
    targets ??= declaredProjectTargets(repoRoot, { entries: files.entries, readText });
    return generateLaunchJson(repoRoot, playgrounds, targets, readText);
  };
  return { ...reconcileLaunchSeed(previous, launch, render, adoptEdits), path, previous };
}
//#endregion
