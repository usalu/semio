/** 🟠️ The rejects unsettled broad foreign tests of the loadTaxonomy suite. */
import { expect, test } from "bun:test";
import { loadTaxonomy, validateTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type Taxonomy } from "../../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";

/** 🟡 Registers the rejects unsettled broad foreign tests of the loadTaxonomy suite. */
export function registerRejectsUnsettledBroadForeignTests(): void {
  test("rejects unsettled, broad foreign, foreign-owning, and false root generation contracts", () => {
    const taxonomy = loadTaxonomy();
    const locks = taxonomy.generatorContracts["external-cargo-locks"]!;
    const unsettled = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "external-cargo-locks": { ...locks, ownership: "unknown" } } } as unknown as Taxonomy;
    expect(validateTaxonomy(unsettled).some((problem) => problem.includes("zero unknown or unsafe"))).toBe(true);
    const broad = { ...taxonomy, fixedDirectoryContracts: { ...taxonomy.fixedDirectoryContracts, "ralph-prd-identifier": { ...taxonomy.fixedDirectoryContracts["ralph-prd-identifier"]!, pathPattern: ".ralph-tui/**" } } };
    expect(validateTaxonomy(broad).some((problem) => problem.includes("recursive wildcard"))).toBe(true);
    const foreignOutput = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "external-cargo-locks": { ...locks, outputRoots: [{ path: ".ralph-tui/session.json", inclusion: "tracked" as const }] } } };
    expect(validateTaxonomy(foreignOutput).some((problem) => problem.includes("a foreign tool's own state is never a repository output"))).toBe(true);
    const generatedRoot = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "external-cargo-locks": { ...locks, outputRoots: [{ path: "package.json", inclusion: "tracked" as const }] } } };
    expect(validateTaxonomy(generatedRoot).some((problem) => problem.includes("authored fixed contracts"))).toBe(true);
  });
  test("declares direct plugin-root facets without a nested directory taxonomy field", () => {
    const taxonomy = loadTaxonomy();
    expect("pluginDirName" in taxonomy).toBe(false);
    expect(taxonomy.pluginChildDirs).toEqual(["🎮️commands", "🔨️modules", "🧬️schema"]);
    expect(taxonomy.osChildDirs).toEqual(["🎮️commands", "🎚️config"]);
  });
  test("keeps the artifact completeness set and the artifact structural set as two separate lists", () => {
    const taxonomy = loadTaxonomy();
    // 🗿️ Completeness is 🧬️schema + 🚪️io — never ⚙️engine; structural-only extra is 📚️examples.
    expect(taxonomy.artifactChildDirs).toEqual(["🧬️schema", "🚪️io", "📚️examples", "🔨️modules"]);
    expect(taxonomy.artifactChildDirs.filter((dir) => !taxonomy.artifactComponentDirs.includes(dir))).toEqual(["📚️examples", "🔨️modules"]);
  });
  test("derives lifecycle capabilities instead of declaring artifact facet directories", () => {
    const taxonomy = loadTaxonomy();
    const lifecycleDirs = ["🏗️builder", "🧐️analyzer", "🎹️composer"];
    expect(taxonomy.newArtifactComponentDirs).toEqual(["🏅️standards"]);
    expect(taxonomy.newArtifactChildDirs).toEqual(["🏅️standards"]);
    expect(taxonomy.standardComponentDirs).toEqual(["🪆️subsets"]);
    expect(taxonomy.standardChildDirs).toEqual(["🪆️subsets", "🔨️modules"]);
    expect(taxonomy.subsetComponentDirs).toEqual(["🧬️schema", "🚪️io"]);
    expect(taxonomy.subsetChildDirs).toEqual(["🧬️schema", "🚪️io", "📚️examples", "👁️viewer", "✏️editor"]);
    expect(taxonomy.subsetArchetypes).toEqual(["owning", "derived"]);
    expect(taxonomy.ioFidelityClasses).toEqual(["exact", "canonical", "semantic", "lossy"]);
    expect([
      ...taxonomy.newArtifactChildDirs,
      ...taxonomy.standardChildDirs,
      ...taxonomy.subsetChildDirs,
    ].filter((dir) => lifecycleDirs.includes(dir))).toEqual([]);
  });
  test("describes the per-example assets/tests shape instead of plural facet dirs", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.exampleAssetsDirName).toBe("🖼️assets");
    expect(taxonomy.exampleTestsDirName).toBe("🧪️tests");
    expect(taxonomy.exampleSlugPattern).toBe("^.+\\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$");
    expect(taxonomy.exampleAssetKindPrefixes).toEqual({
      "snapshot-text": "🗣️",
      "snapshot-binary": "🎒️",
      "mutations-text": "🔧️",
      "mutations-binary": "📡️",
      "diff-text": "🔺️",
      cmd: "🎮️",
    });
    expect(taxonomy.exampleMediaKindPrefixes).toEqual({
      image: "🖼️",
      mesh: "🧊️",
      document: "📄️",
      video: "🎬️",
    });
    expect(taxonomy.exampleFileKinds).toEqual({
      "🦀️rust": "rust-source",
      "🟦️typescript": "typescript-source",
    });
    expect(taxonomy.exampleTestFileKinds).toEqual({
      "🦀️rust": "rust-source",
      "🟦️typescript": "typescript-source",
    });
    expect(taxonomy.forbiddenExampleSlugs).toEqual(["♻️reuse", "♻️default", "📕️default", "♻️semio"]);
    expect(taxonomy.forbiddenExamplePluralDirs).toEqual(["🎒️packs", "🗣️dsls", "🔧️ops", "📡️sprs"]);
    expect(taxonomy.surfaceChildDirs).toContain("📚️examples");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🎒️packs");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🗣️dsls");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🔧️ops");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("📡️sprs");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🖼️assets");
    expect(taxonomy.taxonomyLeafParentDirs).not.toContain("🧪️tests");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🧬️mutations");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🦠️mutation");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("↩️inverse");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🔺️diff");
    expect(taxonomy.taxonomyLeafParentDirs).toContain("🧬️schema");
    expect("exampleComponentDirs" in taxonomy).toBe(false);
  });
  test("forbids both the plural and singular implementations spellings", () => {
    expect(loadTaxonomy().forbiddenPathSegments).toEqual(["⚡️implementations", "⚡️implementation"]);
  });
  test("describes every declared lang with a manifest/marker/leaf contract", () => {
    const taxonomy = loadTaxonomy();
    for (const lang of taxonomy.langs) {
      const ecosystem = taxonomy.ecosystems[lang];
      expect(ecosystem).toBeDefined();
      expect(taxonomy.fileKinds[ecosystem.componentFileKindId]).toBeDefined();
    }
    expect(taxonomy.ecosystems["🦀️rust"].marker).toEqual({ in: "manifest", format: "toml", table: "package.metadata.semio", roleKey: "role", idKey: "id" });
    expect(taxonomy.ecosystems["🟦️typescript"].marker).toEqual({ in: "manifest", format: "json", table: "semio", roleKey: "role", idKey: "id" });
    expect(taxonomy.ecosystems["🐍️python"].marker?.table).toBe("tool.semio");
    expect(taxonomy.ecosystems["🐹️go"].manifestContractId).toBe("build-project-manifest");
    expect(taxonomy.ecosystems["🐹️go"].moduleRootContractId).toBe("go-module");
    expect(taxonomy.ecosystems["🐹️go"].marker?.table).toBe("metadata.semio");
  });
  test("encodes Shape V2 entry location and both valid rust #[path] base conventions", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.entryLocation).toBe("packages");
    expect(taxonomy.rustEntryPathRules.entryDirFromOwner).toBe("📦️packages/🦀️rust");
    expect(taxonomy.rustEntryPathRules.resolution).toBe("cumulative");
    expect(taxonomy.rustEntryPathRules.conventions.map((convention) => convention.id)).toEqual(["leaf-prefixed", "once-reset"]);
    // 🥞️ A grouping `#[path = "."]` reset is never prefixed under either convention — the bug that only a real compile catches.
    expect(taxonomy.rustEntryPathRules.conventions.every((convention) => convention.groupingReset === ".")).toBe(true);
  });
  test("enforces clean declared and undeclared areas while keeping recovered compose trees opaque", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.areaEnforcement).toEqual({ requiredState: "clean", undeclaredAreas: "enforce", opaquePathExclusionIds: ["compose", "temp-compose", "mit-bestand-recherche"] });
    expect(Object.entries(taxonomy.areas).filter(([, state]) => state !== "clean")).toEqual([[".🧬semio/🦑️repo/🎫️tickets", "exempt"]]);
    expect(taxonomy.migratedMarker).toBe("packages-dir-exists");
  });
}
