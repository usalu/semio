/** 🔶️ The parses taxonomy json expected tests of the loadTaxonomy suite. */
import { expect, test } from "bun:test";
import { loadTaxonomy, validateTaxonomy, scopedFileKindIdForSourcePath, taxonomyCliAttemptPreparationsProblems, taxonomyCliJsonWritePreparationProblems, taxonomyCliLeaseDirectoryProblems } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath } from "../../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { taxonomyCliBackupPreparationProblems, taxonomyCliBackupWritePreparationProblems, taxonomyCliEditPreparationProblems, taxonomyCliEditWritePreparationProblems, taxonomyCliRestorePreparationProblems } from "../../../../🔍️discovery/✍️cli-preparation/🟦️.ts";
import { probeClone } from "../../🏗️shared-fixtures/🟦️.ts";

/** 🔷 Registers the parses taxonomy json expected tests of the loadTaxonomy suite. */
export function registerParsesTaxonomyJsonExpectedTests(): void {
  test("parses 🔣️taxonomy.json into the expected shape", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.artifactComponentDirs).toEqual(["🧬️schema", "🚪️io"]);
    expect(taxonomy.mutationComponentFileKindId).toBe("rust-source");
    expect(taxonomy.mutationBehaviorFacetDirs).toEqual(["🦠️mutation", "🔺️diff", "↩️inverse"]);
    expect(taxonomy.mutationDirectLeafForbiddenRegionMarkers).toEqual({ "🔺️diff": "🔖️Diff", "↩️inverse": "🔖️Inverse" });
    expect(taxonomy.mutationOrganizationalFacetDirs).toEqual(["🧩️plan", "📝️text", "💾️binary", "🧬️schema"]);
    expect(taxonomy.schemaChildDirs).toEqual(["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"]);
    expect(taxonomy.representationDirs).toEqual(["📝️text", "💾️binary", "🪶️sqlite"]);
    expect(taxonomy.nativeRepresentationDirs).toEqual(["📝️text", "💾️binary"]);
    expect(taxonomy.schemaFormats).toEqual({
      "🦀️rust": { fileKindId: "rust-source", fieldCasing: "snake" },
      "🟦️typescript": { fileKindId: "typescript-source", fieldCasing: "camel" },
      "🔗️graphql": { fileKindId: "graphql", fieldCasing: "camel" },
      "🔣️jsonschema": { fileKindId: "json", fieldCasing: "camel" },
      "🛰️protobuf": { fileKindId: "protobuf", fieldCasing: "snake" },
      "📜️wit": { fileKindId: "wit", fieldCasing: "kebab" },
    });
    expect(taxonomy.artifactSchemaSpecFileKinds["🧬️schema"]).toBe("json");
    expect(taxonomy.artifactSchemaSpecFileKinds["🧬️schema/📸️snapshot"]).toBe("json");
    expect(taxonomy.artifactSchemaSpecFileKinds["🧬️schema/🔺️diff"]).toBe("json");
    expect(taxonomy.artifactSpecFileKinds["🚪️io/💾️binary/📸️snapshot"]).toBe("protocol-semio");
    expect("🎒️pack" in taxonomy.artifactSpecFileKinds).toBe(false);
    expect(taxonomy.windowChildDirs).toEqual(["🍱️panes", "🪀️widgets", "🪛️utilities", "🎬️actions", "☑️options", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.windowRequiredChildDirs).toEqual(["🎬️actions", "🪛️utilities", "☑️options", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.modeChildDirs).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.modeRequiredChildDirs).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
    expect(taxonomy.pluginRequiredChildDirs).toEqual(["🎮️commands"]);
    expect(taxonomy.osRequiredChildDirs).toEqual(["🎮️commands"]);
    expect(taxonomy.surfaceRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.transientChildDirs).toEqual(["🧬️schema"]);
    expect(taxonomy.surfaceRequiredChildDirs).toContain("🫧️transient");
    expect(taxonomy.surfaceChildDirs).toContain("🫧️transient");
    expect(taxonomy.surfaceSchemaSpecFileKinds["🫧️transient/🧬️schema"]).toBe("json");
    expect(taxonomy.windowComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowEmptyFacetFileKindId).toBe("markdown");
    expect(taxonomy.componentFileKinds["🦀️rust"]).toBe("rust-source");
    expect(taxonomy.componentFileKinds["🟦️typescript"]).toBe("typescript-source");
    expect(taxonomy.artifactSpecFileKinds["🚪️io/📝️text/📸️snapshot"]).toBe("grammar-semio");
    expect(taxonomy.artifactSpecFileKinds["🚪️io/💾️binary/📸️snapshot"]).toBe("protocol-semio");
    expect(taxonomy.libWiringLineBudget).toBe(150);
    expect(taxonomy.packagesDirName).toBe("📦️packages");
    expect(Object.keys(taxonomy.areas).length).toBeGreaterThan(0);
  });
  test("closes generator ownership and keeps foreign metadata directories out of generator outputs", () => {
    const taxonomy = loadTaxonomy();
    const foreignRoots = Object.values(taxonomy.fixedDirectoryContracts).filter((contract) => contract.scope.kind === "repository-root" && contract.pathPattern.startsWith(".") && !contract.pathPattern.includes("/")).map((contract) => contract.pathPattern);
    expect(Object.values(taxonomy.generatorContracts).every((contract) => contract.ownership === "owned" || contract.ownership === "external")).toBe(true);
    expect(taxonomy.generatorContracts["ownerless-ui-icons"]).toBeUndefined();
    expect(taxonomy.generatorContracts["root-layering-declarations"]).toBeUndefined();
    expect(taxonomy.generatorContracts["setup-wizard-config"]).toBeUndefined();
    expect(foreignRoots).toContain(".ralph-tui");
    for (const contract of Object.values(taxonomy.generatorContracts)) for (const output of contract.outputRoots) for (const root of foreignRoots) expect(output.path === root || output.path.startsWith(`${root}/`)).toBe(false);
    expect(fixedFilenameContractIdsForPath(".ralph-tui/session.json", taxonomy)[0]).toBe("ralph-session");
    expect(fixedDirectoryContractIdsForPath(".ralph-tui", taxonomy)[0]).toBe("ralph-metadata");
    expect(fixedDirectoryContractIdsForPath(".ralph-tui/prd", taxonomy)[0]).toBe("ralph-prd-root");
    expect(fixedDirectoryContractIdsForPath(".ralph-tui/prd/dynamic-prd-id", taxonomy)[0]).toBe("ralph-prd-identifier");
  });
  test("scopes Cargo target triples and adjacent Nx manifests through exact owner identities", () => {
    const taxonomy = loadTaxonomy();
    const ticket = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/TICKET";
    for (const triple of ["wasm32-unknown-unknown", "wasm32-wasip2"] as const) {
      const directoryId = `cargo-target-triple-${triple}`;
      const cacheId = `cargo-cache-tag-${triple}`;
      const target = `${ticket}/🧪️target-probe/${triple}`;
      expect(fixedDirectoryContractIdsForPath(target, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([directoryId]);
      expect(fixedFilenameContractIdsForPath(`${target}/CACHEDIR.TAG`, taxonomy, { parentFixedDirectoryContractIds: [directoryId] })).toEqual([cacheId]);
      expect(fixedDirectoryContractIdsForPath(`tmp/🧪️target-probe/${triple}`, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([]);
    }
    expect(fixedDirectoryContractIdsForPath(`${ticket}/🧪️target-probe/wasm32-wasi`, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual([]);
    const nxRoot = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript";
    expect(fixedFilenameContractIdsForPath(`${nxRoot}/package.json`, taxonomy, { siblingFixedFilenameContractIds: ["build-project-manifest"] })).toEqual(["build-owned-node-package-manifest"]);
    expect(fixedFilenameContractIdsForPath(`${nxRoot}/tsconfig.json`, taxonomy, { siblingFixedFilenameContractIds: ["build-project-manifest"] })).toEqual(["build-owned-typescript-config"]);
    expect(fixedFilenameContractIdsForPath(`${nxRoot}/package.json`, taxonomy)).toEqual([]);
    expect(fixedFilenameContractIdsForPath(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️15/BOARD-REACT-RECONCILER/_tmp/package.json", taxonomy)).toEqual([]);
    const broadCargo = probeClone(taxonomy);
    broadCargo.fixedDirectoryContracts["cargo-target-triple-wasm32-wasip2"]!.pathPattern = "**/wasm32-wasip2";
    expect(validateTaxonomy(broadCargo).some((problem) => problem.includes("cargo-target-triple-wasm32-wasip2") && problem.includes("exact governed ticket"))).toBe(true);
    const unscopedNx = probeClone(taxonomy);
    unscopedNx.fixedFilenameContracts["build-owned-node-package-manifest"]!.scope = { kind: "path-pattern" };
    expect(validateTaxonomy(unscopedNx).some((problem) => problem.includes("build-owned-node-package-manifest") && problem.includes("adjacent exact build project manifest"))).toBe(true);
  });
  test("validates the exact transaction edit preparation directory and sole rendered candidate", () => {
    const taxonomy = loadTaxonomy();
    const directory = "🚧️edit-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    for (const leafNames of [[], ["0123456789abcdef01234567.edit"], ["0123456789abcdef01234567.edit", "0123456789abcdef01234567.pre"], ["0123456789abcdef01234567.pre"]]) expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames, writePreparations: [] }, taxonomy)).toEqual([]);
    expect(scopedFileKindIdForSourcePath(`🚧️stage/${directory}/0123456789abcdef01234567.edit`, taxonomy, { parentDirectoryKindId: "transaction-edit-preparation" })).toBe("transaction-edit-candidate");
    expect(scopedFileKindIdForSourcePath(`🚧️stage/${directory}/0123456789abcdef01234567.pre`, taxonomy, { parentDirectoryKindId: "transaction-edit-preparation" })).toBe("transaction-edit-preimage");
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-backup", directoryName: directory, leafNames: ["0123456789abcdef01234567.edit"], writePreparations: [] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames: ["ffffffffffffffffffffffff.edit"], writePreparations: [] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames: ["0123456789abcdef01234567.pre", "ffffffffffffffffffffffff.pre"], writePreparations: [] }, taxonomy)).not.toEqual([]);
    const write = "🚧️write-42-123e4567-e89b-42d3-a456-426614174000";
    expect(taxonomyCliEditWritePreparationProblems({ parentKindId: "transaction-edit-preparation", directoryName: write, leafNames: ["🚧️.edit"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliEditPreparationProblems({ parentKindId: "transaction-stage", directoryName: directory, leafNames: [], writePreparations: [{ directoryName: write, leafNames: ["🚧️.edit"] }] }, taxonomy)).toEqual([]);
    const backup = "🚧️backup-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    expect(taxonomyCliBackupWritePreparationProblems({ parentKindId: "transaction-backup-preparation", directoryName: write, leafNames: ["🚧️.backup"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliBackupPreparationProblems({ parentKindId: "transaction-backup", directoryName: backup, leafNames: ["0123456789abcdef01234567.backup"], writePreparations: [{ directoryName: write, leafNames: [] }] }, taxonomy)).toEqual([]);
  });
  test("validates every attempt preparation sibling before recovery", () => {
    const taxonomy = loadTaxonomy();
    const first = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000001-42-123e4567-e89b-42d3-a456-426614174000", children: [{ name: "🚧️stage", nodeKind: "directory" }, { name: "💾️backup", nodeKind: "directory" }, { name: "🔒️lease", nodeKind: "directory" }, { name: "🔣️.json", nodeKind: "file" }] } as const;
    const second = { parentKindId: "transaction-attempts", directoryName: "🚧️prepare-000002-43-223e4567-e89b-42d3-a456-426614174000", children: [] } as const;
    expect(taxonomyCliAttemptPreparationsProblems([first, second], taxonomy)).toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([first, first], taxonomy)).not.toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([{ ...second, directoryName: "🚧️prepare-malformed" }], taxonomy)).not.toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([{ ...second, children: [{ name: "foreign", nodeKind: "directory" }] }], taxonomy)).not.toEqual([]);
    expect(taxonomyCliAttemptPreparationsProblems([{ ...second, children: [{ name: "🚧️write-43-223e4567-e89b-42d3-a456-426614174000", nodeKind: "directory" }] }], taxonomy)).not.toEqual([]);
  });
  test("validates restore exchange states and exact JSON write preparations", () => {
    const taxonomy = loadTaxonomy();
    const restore = "🚧️restore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000";
    for (const leaves of [[], ["0123456789abcdef01234567.backup"], ["0123456789abcdef01234567.backup", "0123456789abcdef01234567.post"], ["0123456789abcdef01234567.post"]]) {
      expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-backup", directoryName: restore, leafNames: leaves }, taxonomy)).toEqual([]);
    }
    expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-backup", directoryName: restore, leafNames: ["ffffffffffffffffffffffff.backup"] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliRestorePreparationProblems({ parentKindId: "transaction-stage", directoryName: restore, leafNames: [] }, taxonomy)).not.toEqual([]);
    const write = "🚧️write-42-123e4567-e89b-42d3-a456-426614174000";
    for (const leafNames of [[], ["🔣️.json"], ["🔣️.json", "⏮️.json"], ["⏮️.json"]]) expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-journal-write", directoryName: write, leafNames }, taxonomy)).toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-lease-preparation", directoryName: write, leafNames: ["🔣️.json"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-lease", directoryName: write, leafNames: ["🔣️.json"] }, taxonomy)).toEqual([]);
    expect(taxonomyCliLeaseDirectoryProblems({ parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-preparing", leafNames: ["🔣️.json"], writePreparations: [] }, taxonomy)).toEqual([]);
    expect(taxonomyCliLeaseDirectoryProblems({ parentKindId: "transaction-backup", directoryName: "🚧️lease-42-123e4567-e89b-42d3-a456-426614174000-stale", leafNames: [], writePreparations: [{ directoryName: write, leafNames: ["🔣️.json", "⏮️.json"] }] }, taxonomy)).toEqual([]);
    expect(taxonomyCliLeaseDirectoryProblems({ parentKindId: "transaction-attempt", directoryName: "🔒️lease", leafNames: [], writePreparations: [] }, taxonomy)).not.toEqual([]);
    expect(scopedFileKindIdForSourcePath(`🚧️journal/${write}/⏮️.json`, taxonomy, { parentDirectoryKindId: "transaction-json-write-preparation" })).toBe("transaction-json-previous");
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-stage", directoryName: write, leafNames: ["🔣️.json"] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-journal-write", directoryName: write, leafNames: ["🔣️.json", "partial.json"] }, taxonomy)).not.toEqual([]);
    expect(taxonomyCliJsonWritePreparationProblems({ parentKindId: "transaction-journal-write", directoryName: write, leafNames: ["⏮️.json", "⏮️.json"] }, taxonomy)).not.toEqual([]);
  });
}
