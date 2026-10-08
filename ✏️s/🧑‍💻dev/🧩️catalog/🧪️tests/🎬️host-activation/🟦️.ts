import { requirePlaygroundVariant } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/⭐️default/🟦️.ts";
/** 🎬️ The hub's own two invariants: a host variant's session really is the WHOLE catalog, and the
 * declared `on-artifact-kind:` rows really do name one owner per artifact kind — the data every
 * "open an artifact whose plugin is not loaded yet" path reads before any wasm module exists. */

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { describe, expect, test } from "vitest";
import { artifactKindActivationOwner, ON_ARTIFACT_KIND_ACTIVATION_PREFIX } from "@semio-tech/framework";
import { composeSpecificOsCatalogV1 } from "../../🟦️.ts";
const catalog = composeSpecificOsCatalogV1("http://localhost/", { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }).catalog;
import { hostProjection, installedScope } from "../🧩️scope/🟦️.ts";
import { resolvePluginHostConfig } from "@semio-tech/framework";
import { resolveFrameworkOsCatalogConfigurationV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧩️catalog/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { buildPlaygroundSession } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts";

const root = resolve(import.meta.dirname, "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
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
    const session = buildPlaygroundSession(requirePlaygroundVariant(installedScope.hostVariant), hostProjection);
    expect(session.hostMode).toBe(true);
    expect(session.plugins).toHaveLength(hostProjection.entries.length);
    expect(session.plugins.map((row) => row.pluginId).sort()).toEqual(hostProjection.entries.map((row) => row.pluginId).sort());
    expect(session.plugins.length).toBeGreaterThan(1);
    expect(resolvePluginHostConfig(catalog, installedScope.missingVariant)).toBeUndefined();
    const operation = new AbortController(), composition = composeSpecificOsCatalogV1("http://localhost/", { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => operation.signal.aborted, progress: event => console.log(`[DEBUG] installed host ${event.completed}/${event.total}`) });
    expect(() => resolveFrameworkOsCatalogConfigurationV1(composition.rows, installedScope.missingVariant, { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => operation.signal.aborted, progress: event => console.log(`[DEBUG] missing host ${event.completed}/${event.total}`) })).toThrow();
  });

  test("a non-host variant stays filtered to its own closure", () => {
    const nonHost = projection.playgrounds.find((row) => !projection.entries.some((entry) => entry.pluginId === row.pluginId && entry.host));
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
    for (const [kind, claimants] of owners) expect(artifactKindActivationOwner(catalog, kind), kind).toBe(claimants[0]);
    expect(artifactKindActivationOwner(catalog, "")).toBeUndefined();
    expect(artifactKindActivationOwner(catalog, "no.such.kind")).toBeUndefined();
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

