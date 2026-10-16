/** 🕸️ Plugin graph validation, instance directory and artifact routers.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";

// 🕸️ `@semio-tech/framework`'s `PluginGraph`/`InstanceDirectory`/`ArtifactMutationRouter`/
// `ArtifactInferenceRouter` (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS,
// W2-B) have no in-source-testing harness of their own (`@semio-tech/framework`'s vitest only
// `includeSource`s `🟦️.ts`, not the module file they're defined in) — dynamically importing the
// real workspace package here exercises them under a config that DOES run, without this file taking
// on a static dependency on `@semio-tech/framework`'s runtime exports.
describe("@semio-tech/framework PluginGraph", () => {
  it("validates a graph with every dependency present and exactly pinned", async () => {
    const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
    expect(
      validatePluginDependencyGraph([
        { pluginId: "a", version: "1.2.3" },
        { pluginId: "b", version: "1.0.0", dependencies: [{ pluginId: "a", version: "=1.2.3" }] },
        { pluginId: "c", dependencies: [{ pluginId: "a" }] },
      ]),
    ).toEqual([]);
  });

  it("reports a missing dependency", async () => {
    const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
    expect(validatePluginDependencyGraph([{ pluginId: "b", dependencies: [{ pluginId: "missing" }] }])).toEqual([
      { code: "transaction.dependency-missing", pluginId: "b", dependsOn: "missing" },
    ]);
  });

  it("reports a version mismatch", async () => {
    const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
    expect(
      validatePluginDependencyGraph([
        { pluginId: "a", version: "2.0.0" },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "=1.0.0" }] },
      ]),
    ).toEqual([{ code: "transaction.version-mismatch", pluginId: "b", dependsOn: "a", required: "=1.0.0", actual: "2.0.0" }]);
  });

  it("resolves a diamond load order deterministically, tie-broken lexicographically", async () => {
    const { resolvePluginLoadOrder } = await import("@semio-tech/framework");
    const result = resolvePluginLoadOrder([
      {
        pluginId: "d",
        dependencies: [
          { pluginId: "b" },
          { pluginId: "c" },
        ],
      },
      { pluginId: "c", dependencies: [{ pluginId: "a" }] },
      { pluginId: "b", dependencies: [{ pluginId: "a" }] },
      { pluginId: "a" },
    ]);
    expect(result.errors).toEqual([]);
    expect(result.order).toEqual(["a", "b", "c", "d"]);
  });

  it("names every member of a cycle", async () => {
    const { resolvePluginLoadOrder } = await import("@semio-tech/framework");
    const result = resolvePluginLoadOrder([
      { pluginId: "a", dependencies: [{ pluginId: "b" }] },
      { pluginId: "b", dependencies: [{ pluginId: "a" }] },
    ]);
    expect(result.order).toEqual([]);
    expect(result.errors).toEqual([{ code: "transaction.cycle", members: ["a", "b"] }]);
  });

  it("versionSatisfies admits only the exact pin and refuses every range", async () => {
    const { versionSatisfies } = await import("@semio-tech/framework");
    expect(versionSatisfies("1.2.3", "=1.2.3")).toBe(true);
    expect(versionSatisfies("1.2.4", "=1.2.3")).toBe(false);
    expect(versionSatisfies("1.2.2", "=1.2.3")).toBe(false);
    for (const range of ["*", "^1.2.3", "~1.2.3", ">=1.2.3", "1.2.3"]) expect(versionSatisfies("1.2.3", range)).toBe(false);
  });

  it("orderPluginRegistryEntries drops only the blocked entries, dependency-orders the rest", async () => {
    const { orderPluginRegistryEntries } = await import("@semio-tech/framework");
    const result = orderPluginRegistryEntries([
      { pluginId: "b", moduleUrl: "b.js", dependencies: [{ pluginId: "a" }] },
      { pluginId: "a", moduleUrl: "a.js" },
      { pluginId: "broken", moduleUrl: "broken.js", dependencies: [{ pluginId: "missing" }] },
    ]);
    expect(result.order.map((entry) => entry.pluginId)).toEqual(["a", "b"]);
    expect(result.errors).toEqual([{ code: "transaction.dependency-missing", pluginId: "broken", dependsOn: "missing" }]);
  });

  it("pluginGraphErrorMessage renders a real English and a real German message", async () => {
    const { pluginGraphErrorMessage } = await import("@semio-tech/framework");
    const error = { code: "transaction.dependency-missing" as const, pluginId: "b", dependsOn: "a" };
    expect(pluginGraphErrorMessage(error, "en")).toContain("needs");
    expect(pluginGraphErrorMessage(error, "de")).toContain("benötigt");
  });

  it("PluginGraph.canUnload refuses while a loaded dependent exists, allows once it's gone", async () => {
    const { PluginGraph } = await import("@semio-tech/framework");
    const graph = new PluginGraph([{ pluginId: "a" }, { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] }]);
    expect(graph.canUnload("a", new Set(["a", "b"]))).toBe(false);
    expect(graph.canUnload("a", new Set(["a"]))).toBe(true);
  });
});

describe("@semio-tech/framework InstanceDirectory and ArtifactRouters", () => {
  it("InstanceDirectory registers, resolves, and unregisters", async () => {
    const { InstanceDirectory } = await import("@semio-tech/framework");
    const directory = new InstanceDirectory();
    directory.register("artifact-1", { pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
    expect(directory.resolve("artifact-1")).toEqual({ pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
    directory.unregister("artifact-1");
    expect(directory.resolve("artifact-1")).toBeUndefined();
  });

  it("ArtifactMutationRouter accepts a byte-identical re-registration, rejects a conflicting one", async () => {
    const { ArtifactMutationRouter } = await import("@semio-tech/framework");
    const router = new ArtifactMutationRouter();
    router.registerOwner("s.cad.model", "s.cad#add-wall");
    router.registerOwner("s.cad.model", "s.cad#add-wall");
    expect(router.resolve("s.cad.model", "s.cad#add-wall")).toEqual({ kind: "owner" });
    expect(() =>
      router.registerContributed(
        "s.cad.model",
        "aec-building",
        "cad",
        { mutationId: "s.cad#add-wall", semantics: { verb: "add", entity: "wall", kind: "add-wall", record: "Wall" }, schemaVersion: 1, algorithmVersion: 1 },
        true,
      ),
    ).toThrow(/conflict/);
  });

  it("ArtifactMutationRouter.registerContributed rejects a contributor that doesn't depend on the owner", async () => {
    const { ArtifactMutationRouter } = await import("@semio-tech/framework");
    const router = new ArtifactMutationRouter();
    expect(() =>
      router.registerContributed(
        "s.cad.model",
        "aec-building",
        "cad",
        { mutationId: "s.cad#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
        false,
      ),
    ).toThrow(/not a direct dependency/);
  });

  it("ArtifactInferenceRouter enforces owner === contributor and orders the depends_on DAG", async () => {
    const { ArtifactInferenceRouter } = await import("@semio-tech/framework");
    const router = new ArtifactInferenceRouter();
    router.registerContributed(
      "s.cad.model",
      {
        owner: "aec-building",
        artifactKind: "s.cad.model",
        artifactSchema: "s.cad.model",
        artifactSchemaVersion: 1,
        documentSchema: "s.cad",
        documentSchemaVersion: 1,
        inferenceSchema: "s.aec-building.load-path",
        inferenceSchemaVersion: 1,
        algorithmVersion: 1,
        policyVersion: 1,
        contributor: "aec-building",
        dependsOn: [],
      },
      true,
    );
    expect(router.resolve("s.cad.model", "s.aec-building.load-path")).toEqual({ kind: "contributed", pluginId: "aec-building" });
    expect(router.dependencyOrder()).toEqual(["s.cad.model s.aec-building.load-path"]);
  });

  it("ArtifactInferenceRouter.registerContributed rejects owner !== contributor", async () => {
    const { ArtifactInferenceRouter } = await import("@semio-tech/framework");
    const router = new ArtifactInferenceRouter();
    expect(() =>
      router.registerContributed(
        "s.cad.model",
        {
          owner: "someone-else",
          artifactKind: "s.cad.model",
          artifactSchema: "s.cad.model",
          artifactSchemaVersion: 1,
          documentSchema: "s.cad",
          documentSchemaVersion: 1,
          inferenceSchema: "s.aec-building.load-path",
          inferenceSchemaVersion: 1,
          algorithmVersion: 1,
          policyVersion: 1,
          contributor: "aec-building",
          dependsOn: [],
        },
        true,
      ),
    ).toThrow(/owner\/contributor mismatch/);
  });
});
