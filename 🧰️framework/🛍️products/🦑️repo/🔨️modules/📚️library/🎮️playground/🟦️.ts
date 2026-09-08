import { PLAYGROUND_LOCKED_EXAMPLE_ENV } from "./🔒️preferences/🟦️.ts";
/** @emoji 🎮️ Playground identity for the whole repository: the generated OS playground catalog, the
 * dev/test port table every host binds, and the locked-example Vite define. Split out of
 * `📦️packages/🟦️typescript/🟦️.ts` so a consumer that only needs a port (the styling package's dev
 * servers, and through them `⚙️vite.config.ts`) never drags the repository library's `🔍️discovery`
 * taxonomy walk into its module graph. */
import { ephemeralBox } from "@semio-tech/framework";
import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { getWorkspaceRoot } from "../🗂️workspaces/🟦️.ts";
import { cargoProviderTomlParser, discoverCatalogPackages, loadCatalogTaxonomy } from "../🔍️discovery/🟦️.ts";
import type { PlaygroundBuildTarget as PlaygroundVariant } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts";

export type PlaygroundHostKind = string;

export type PlaygroundSelection = {
  readonly variant: string;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
};

/** 🧭️ Resolves public selections from authored manifests before any generated output exists. */
export function loadFrameworkOsPlaygroundSelections(repoRoot = getWorkspaceRoot(), manifestPaths?: readonly string[]): readonly PlaygroundSelection[] {
  const paths = manifestPaths ?? discoverCatalogPackages(repoRoot, loadCatalogTaxonomy()).filter((entry) => entry.lang === "🦀️rust" && ["plugin", "extension"].includes(entry.role)).map((entry) => entry.manifestPath);
  const selections: PlaygroundSelection[] = [], identities = new Set<string>();
  for (const path of paths) {
    const absolute = resolve(repoRoot, path), local = relative(repoRoot, absolute).replaceAll("\\", "/");
    if (isAbsolute(path) || local !== path || local.startsWith("../") || !local.endsWith("/Cargo.toml")) throw new Error(`Playground manifest is outside its source owner: ${path}`);
    for (let node = absolute; node !== resolve(repoRoot); node = dirname(node)) if (lstatSync(node).isSymbolicLink()) throw new Error(`Playground manifest source is a symlink: ${path}`);
    const metadata = (cargoProviderTomlParser.parse(readFileSync(absolute, "utf8")) as { package?: { metadata?: { component?: { package?: string }; semio?: { role?: string; playground?: PlaygroundSelection[] } } } }).package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.role ?? "")) continue;
    if (!/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/.test(metadata.component.package)) throw new Error(`Invalid playground component identity: ${path}`);
    for (const row of metadata.semio?.playground ?? []) {
      if (typeof row.variant !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant)) throw new Error(`Invalid playground variant: ${path}`);
      const aliases = row.aliases ?? [];
      if (!Array.isArray(aliases) || aliases.some((alias) => typeof alias !== "string" || !alias.trim())) throw new Error(`Invalid playground aliases: ${path}`);
      for (const identity of [row.variant, ...aliases.filter((alias) => alias !== row.variant)]) {
        if (identities.has(identity)) throw new Error(`Duplicate playground selection: ${identity}`);
        identities.add(identity);
      }
      for (const renderer of ["react", "wgpu"] as const) if (!Number.isSafeInteger(row.ports?.[renderer]) || row.ports[renderer] < 1 || row.ports[renderer] > 65535) throw new Error(`Invalid playground ${renderer} port: ${path}`);
      selections.push({ variant: row.variant, aliases, ports: row.ports, pluginId: metadata.component.package.slice(6), cratePath: dirname(local).replaceAll("\\", "/") });
    }
  }
  return selections.sort((a, b) => a.variant.localeCompare(b.variant));
}

/**
 * 📚️ Loads the generated framework OS playground catalog (variant/plugin/aliases/ports rows).
 * Reads the registry owner's `🤖️generated/🎠️playgrounds.json` directly (rather than a static
 * TS import of the gitignored generated module) so this shared kernel never fails to load on a
 * fresh clone before `bun nx run @semio-tech/plugin-registry:generate` has ever run — callers get
 * an empty catalog in that case instead of a hard module-resolution error.
 */
export function loadFrameworkOsPlaygroundCatalog(): readonly PlaygroundVariant[] {
  const catalogPath = join(getWorkspaceRoot(), "./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json");
  if (!existsSync(catalogPath)) return [];
  return JSON.parse(readFileSync(catalogPath, "utf8")) as readonly PlaygroundVariant[];
}

type PlaygroundPortSpec = {
  readonly dev: number;
  readonly test?: number;
  readonly env: string;
};

/** @emoji 🔌️ Builds playground port table from semio.app manifests plus non-app hosts. */
function buildPlaygroundPortsFromManifests(): Record<string, PlaygroundPortSpec> {
  const ports: Record<string, PlaygroundPortSpec> = {
    storybook: { dev: 6010, env: "STORYBOOK_PORT" },
  };
  for (const row of loadFrameworkOsPlaygroundCatalog()) ports[row.variant] = { dev: row.ports.react, test: row.ports.wgpu, env: "S_OS_PORT" };
  const walk = (directory: string): void => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (entry.name === "node_modules" || entry.name === "target" || entry.name === "🎫️tickets" || entry.name.startsWith(".")) continue;
      const path = join(directory, entry.name);
      if (entry.isDirectory()) {
        walk(path);
        continue;
      }
      if (entry.name !== "package.json") continue;
      try {
        const manifest = JSON.parse(readFileSync(path, "utf8")) as { semio?: { app?: { hostKind?: string; port?: { dev?: number; test?: number; env?: string } } } };
        const app = manifest.semio?.app;
        if (app?.hostKind && Number.isSafeInteger(app.port?.dev) && app.port?.env) ports[app.hostKind] = { dev: app.port.dev!, test: app.port.test, env: app.port.env };
      } catch {}
    }
  };
  walk(getWorkspaceRoot());
  return ports;
}

const playgroundPortsCache = ephemeralBox<Record<string, PlaygroundPortSpec> | undefined>("framework.products.repo.modules.library.playground.index.ts.playgroundPortsCache", undefined);

function resolvePlaygroundPorts(): Record<string, PlaygroundPortSpec> {
  playgroundPortsCache.current ??= buildPlaygroundPortsFromManifests();
  return playgroundPortsCache.current;
}

export const PLAYGROUND_PORTS: Record<string, PlaygroundPortSpec> = new Proxy({} as Record<string, PlaygroundPortSpec>, {
  get(_target, prop: string) {
    return resolvePlaygroundPorts()[prop];
  },
  ownKeys() {
    return Reflect.ownKeys(resolvePlaygroundPorts());
  },
  getOwnPropertyDescriptor(_target, prop) {
    const value = resolvePlaygroundPorts()[prop as string];
    if (value === undefined) return undefined;
    return { configurable: true, enumerable: true, value };
  },
});

/** @emoji 🔌️ Local dev port for a playground host. */
export function playgroundDevPort(kind: PlaygroundHostKind): number {
  const spec = resolvePlaygroundPorts()[kind];
  if (!spec) throw new Error(`unknown playground host kind: ${kind}`);
  return spec.dev;
}

/** @emoji 🔌️ String dev port (vite `--port`, nx `env`). */
export function playgroundDevPortString(kind: PlaygroundHostKind): string {
  return String(playgroundDevPort(kind));
}

/** @emoji 🧪️ Vitest/playwright port when set; otherwise `undefined`. */
export function playgroundTestPort(kind: PlaygroundHostKind): number | undefined {
  return resolvePlaygroundPorts()[kind]?.test;
}

/** @emoji 🧪️ String test port for nx `env` / playwright. */
export function playgroundTestPortString(kind: PlaygroundHostKind): string | undefined {
  const port = playgroundTestPort(kind);
  return port === undefined ? undefined : String(port);
}

/** @emoji 🔌️ Process env var holding the dev port override. */
export function playgroundPortEnv(kind: PlaygroundHostKind): string {
  const spec = resolvePlaygroundPorts()[kind];
  if (!spec) throw new Error(`unknown playground host kind: ${kind}`);
  return spec.env;
}

/** @emoji 🚧️ Every assigned playground dev + test port (for strict binding). */
export function allPlaygroundReservedPorts(): ReadonlySet<number> {
  const ports = new Set<number>();
  for (const spec of Object.values(resolvePlaygroundPorts())) {
    ports.add(spec.dev);
    if (spec.test !== undefined) ports.add(spec.test);
  }
  return ports;
}

/** @emoji 🔌️ OS hub service dev port. 8787, not 6070 — 6070 is the `s` react playground's port,
 * see `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` `[[package.metadata.semio.playground]]`. */
export const OS_HUB_PORT = 8787;

/** @emoji 🔌️ Process env var for {@link OS_HUB_PORT}. */
export const OS_HUB_PORT_ENV = "OS_HUB_PORT";

/** @emoji 🔒️ Process env var locking a playground to one example (hides navbar dropdown). */
export { PLAYGROUND_LOCKED_EXAMPLE_ENV } from "./🔒️preferences/🟦️.ts";

/** @emoji 🔒️ Locked example id from process env, if any. */
export function playgroundLockedExampleIdFromEnv(env: NodeJS.ProcessEnv = process.env): string | undefined {
  const raw = env[PLAYGROUND_LOCKED_EXAMPLE_ENV]?.trim();
  return raw || undefined;
}


/** @emoji 🔌️ Vite `define` entries for playground play bundles. */
export function playgroundPlayViteDefine(extra: Record<string, string> = {}): Record<string, string> {
  return {
    "import.meta.env.PLAYGROUND_LOCKED_EXAMPLE_ID": JSON.stringify(playgroundLockedExampleIdFromEnv() ?? ""),
    "import.meta.vitest": "undefined",
    ...extra,
  };
}
