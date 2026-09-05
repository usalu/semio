/** @emoji 🎮️ Playground identity for the whole repository: the generated OS playground catalog, the
 * dev/test port table every host binds, and the locked-example Vite define. Split out of
 * `📦️packages/🟦️typescript/🟦️.ts` so a consumer that only needs a port (the styling package's dev
 * servers, and through them `⚙️vite.config.ts`) never drags the repository library's `🔍️discovery`
 * taxonomy walk into its module graph. */
import { ephemeralBox } from "@semio-tech/framework";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../🗂️workspaces/🟦️.ts";
import type { PlaygroundBuildTarget as PlaygroundVariant } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts";

export type PlaygroundHostKind = string;

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
export const PLAYGROUND_LOCKED_EXAMPLE_ENV = "PLAYGROUND_LOCKED_EXAMPLE_ID";

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

