/** 🎬️ The hub's own two invariants: a host variant's session really is the WHOLE catalog, and the
 * declared `on-artifact-kind:` rows really do name one owner per artifact kind — the data every
 * "open an artifact whose plugin is not loaded yet" path reads before any wasm module exists. */

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { describe, expect, test } from "vitest";
import { artifactKindActivationOwner, ON_ARTIFACT_KIND_ACTIVATION_PREFIX } from "@semio-tech/framework";
import { PLUGIN_CATALOG } from "../../🟦️.ts";
import { DEFAULT_HOST_VARIANT } from "../../🤖️generated/🎮️playgrounds/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../📖️catalog-view/🟦️.ts";
import { buildPlaygroundSession } from "../../🎮️playground/🧭️session/🟦️.ts";

const root = resolve(import.meta.dirname, "../..");
const workspace = resolve(root, "../../../../../..");
const projection = readGeneratedCatalogProjection(join(root, "🤖️generated"));

/** 🎬️ The kind → owner index the resolvers read, rebuilt here from the raw rows so the test never
 * shares the implementation it is checking. */
function ownersByArtifactKind(): Map<string, string[]> {
  const owners = new Map<string, string[]>();
  for (const entry of projection.entries) {
    for (const event of entry.activationEvents) {
      if (!event.startsWith(ON_ARTIFACT_KIND_ACTIVATION_PREFIX)) continue;
      const kind = event.slice(ON_ARTIFACT_KIND_ACTIVATION_PREFIX.length);
      owners.set(kind, [...(owners.get(kind) ?? []), entry.pluginId]);
    }
  }
  return owners;
}

describe("host variant session", () => {
  test("the host variant's session is the whole registry, not its dependency closure", () => {
    const session = buildPlaygroundSession(DEFAULT_HOST_VARIANT, projection);
    expect(session.hostMode).toBe(true);
    expect(session.plugins).toHaveLength(projection.entries.length);
    expect(session.plugins.map((row) => row.pluginId).sort()).toEqual(projection.entries.map((row) => row.pluginId).sort());
    expect(session.plugins.length).toBeGreaterThan(1);
  });

  test("a non-host variant stays filtered to its own closure", () => {
    const nonHost = projection.playgrounds.find((row) => row.variant !== DEFAULT_HOST_VARIANT && !projection.entries.some((entry) => entry.pluginId === row.pluginId && entry.host));
    expect(nonHost, "the catalog must declare at least one non-host playground").toBeDefined();
    const session = buildPlaygroundSession(nonHost!.variant, projection);
    expect(session.hostMode).toBe(false);
    expect(session.plugins.length).toBeLessThan(projection.entries.length);
  });
});

describe("artifact kind activation owners", () => {
  test("every declared artifact kind is claimed by exactly one catalog row", () => {
    const owners = ownersByArtifactKind();
    expect(owners.size).toBeGreaterThan(0);
    expect([...owners].filter(([, claimants]) => claimants.length > 1)).toEqual([]);
  });

  test("both kind namespaces a descriptor declares resolve to the same owner", () => {
    const owners = ownersByArtifactKind();
    for (const [kind, claimants] of owners) expect(artifactKindActivationOwner(PLUGIN_CATALOG, kind), kind).toBe(claimants[0]);
    expect(artifactKindActivationOwner(PLUGIN_CATALOG, "")).toBeUndefined();
    expect(artifactKindActivationOwner(PLUGIN_CATALOG, "no.such.kind")).toBeUndefined();
  });

  test("every plugin declaring an app surface owns that surface's artifact kind", () => {
    const owners = ownersByArtifactKind();
    const declared = projection.entries.filter((entry) => entry.activationEvents.some((event) => event.startsWith(ON_ARTIFACT_KIND_ACTIVATION_PREFIX)));
    expect(declared.length).toBeGreaterThan(0);
    for (const entry of declared) {
      const kinds = entry.activationEvents.filter((event) => event.startsWith(ON_ARTIFACT_KIND_ACTIVATION_PREFIX)).map((event) => event.slice(ON_ARTIFACT_KIND_ACTIVATION_PREFIX.length));
      for (const kind of kinds) expect(owners.get(kind), `${entry.pluginId} ${kind}`).toEqual([entry.pluginId]);
    }
  });
});

describe("host variant Nx fan-out", () => {
  test("a host crate's web prepare stays boot-scoped while native still fans out every registered component", async () => {
    const { cacheInternals, libraryBootstrap } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
    await libraryBootstrap;
    const base = join(workspace, ".🧬semio/🦑️repo/⚡️cache/🧪️plugin-registry/host-activation");
    mkdirSync(base, { recursive: true });
    const fixture = mkdtempSync(join(base, "run-"));
    try {
      const put = (path: string, text: string): void => { mkdirSync(dirname(join(fixture, path)), { recursive: true }); writeFileSync(join(fixture, path), text); };
      const project = (path: string, name: string): void => put(join(path, "📋️project.json"), JSON.stringify({ name, targets: { wasm: {}, "wasm-release": {} } }));
      project("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript", "renderer");
      const crate = (directory: string, id: string, extra: string): void => {
        project(directory, id);
        put(join(directory, "Cargo.toml"), `[package]\nname = "${id}"\nversion = "0.1.0"\n[package.metadata.component]\npackage = "semio:${id}"\n[package.metadata.semio]\nrole = "plugin"\n${extra}[[package.metadata.semio.playground]]\nvariant = "${id}"\n`);
      };
      crate("hub", "hub", 'host = { landing = "home", shell = "studio" }\n');
      crate("alpha", "alpha", "");
      crate("beta", "beta", "");
      const targets = cacheInternals.playgroundPreparationTargets(["hub/Cargo.toml", "alpha/Cargo.toml", "beta/Cargo.toml"], fixture, "owner");
      for (const profile of ["dev", "release"]) {
        for (const renderer of ["react", "wgpu"] as const) {
          const prepare = targets[`prepare-hub-${renderer}-${profile}`];
          expect(prepare, `prepare-hub-${renderer}-${profile}`).toBeDefined();
          expect(prepare.dependsOn, `prepare-hub-${renderer}-${profile} hub`).toContain(`hub:materialize-${profile}`);
          expect(prepare.dependsOn, `prepare-hub-${renderer}-${profile} alpha`).not.toContain(`alpha:materialize-${profile}`);
          expect(prepare.dependsOn, `prepare-hub-${renderer}-${profile} beta`).not.toContain(`beta:materialize-${profile}`);
        }
        const nativePrepare = targets[`prepare-hub-native-${profile}`];
        expect(nativePrepare, `prepare-hub-native-${profile}`).toBeDefined();
        for (const id of ["hub", "alpha", "beta"]) expect(nativePrepare.dependsOn, `prepare-hub-native-${profile} ${id}`).toContain(`${id}:materialize-${profile}`);
        // 🚫️ A non-host variant must NOT inherit the fan-out — that is the regression this pins.
        expect(targets[`prepare-alpha-react-${profile}`].dependsOn).not.toContain(`beta:materialize-${profile}`);
        for (const renderer of ["react", "wgpu"]) expect(targets[`activate-hub-${renderer}-${profile}`].dependsOn).toEqual([`prepare-hub-${renderer}-${profile}`]);
      }
    } finally {
      rmSync(fixture, { recursive: true, force: true });
    }
  });
});
