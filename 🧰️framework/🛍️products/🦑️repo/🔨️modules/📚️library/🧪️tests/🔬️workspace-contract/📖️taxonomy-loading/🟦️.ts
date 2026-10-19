import { requireRecord } from "@semio-tech/schema-validator-ts";
import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, validateTaxonomy, scopedFileKindIdForSourcePath, taxonomyCliAttemptPreparationsProblems, taxonomyCliJsonWritePreparationProblems, taxonomyCliLeaseDirectoryProblems, validateGeneratorContractsAgainstWorkspace } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { artifactFacetPathIsDeclared } from "../../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { fixedDirectoryContractIdsForPath, fixedFilenameContractIdsForPath } from "../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { generatorBuildPreviewCommand } from "../../../🔍️discovery/🛠️generator-commands/🟦️.ts";
import { taxonomyCliBackupPreparationProblems, taxonomyCliBackupWritePreparationProblems, taxonomyCliEditPreparationProblems, taxonomyCliEditWritePreparationProblems, taxonomyCliRestorePreparationProblems } from "../../../🔍️discovery/✍️cli-preparation/🟦️.ts";
import { type Taxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { generatorInputPaths } from "../../../🧹️normalization/🏭️generator-inputs/🟦️.ts";
import { parseGeneratorPreviewManifest } from "../../../🧹️normalization/🧾️plan-parsing/🟦️.ts";
import { loadTaxonomy as loadSourceTaxonomy } from "../../../🧹️normalization/🔣️taxonomy/🟦️.ts";
import { normalizationEngineDeclarations } from "../../../🧹️normalization/🧪️support/🏗️source-services/🟦️.ts";
import { probeClone } from "../🏗️shared-fixtures/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

describe("loadTaxonomy", () => {
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

  test("requires every owned generator to expose one exact read-only preview target", async () => {
    const taxonomy = loadTaxonomy();
    const owned = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => contract.ownership === "owned");
    const external = Object.values(taxonomy.generatorContracts).filter((contract) => contract.ownership === "external");
    const inputRoot = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), input = readFileSync(join(inputRoot, "🔣️.json"), "utf8");
    const vector = JSON.parse(input) as { schemaVersion: number; contract: string; ownedGeneratorIds: string[] };
    expect(vector.ownedGeneratorIds).toEqual([...vector.ownedGeneratorIds].sort());
    const jsonc = await import("jsonc-parser"), inputErrors: import("jsonc-parser").ParseError[] = [];
    expect(jsonc.parse(input, inputErrors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
    expect(inputErrors).toEqual([]);
    const members = (node: import("jsonc-parser").Node | undefined): Map<string, import("jsonc-parser").Node> => {
      if (node?.type !== "object") throw new Error("Expected an exact JSON object");
      const result = new Map<string, import("jsonc-parser").Node>();
      for (const property of node.children ?? []) {
        const [key, value] = property.children ?? [];
        if (property.type !== "property" || key?.type !== "string" || !value || result.has(key.value)) throw new Error("Duplicate or malformed JSON property");
        result.set(key.value, value);
      }
      return result;
    };
    const oracleIds = (source: string): string[] => {
      const errors: import("jsonc-parser").ParseError[] = [], tree = jsonc.parseTree(source, errors, { disallowComments: true, allowTrailingComma: false });
      if (errors.length) throw new Error("Invalid JSON authority");
      return [...members(members(tree).get("generatorContracts"))].flatMap(([id, contract]) => {
        const ownership = members(contract).get("ownership");
        if (ownership?.type !== "string") throw new Error("Missing ownership authority");
        return ownership.value === "owned" ? [id] : [];
      }).sort();
    };
    const expectedIds = vector.ownedGeneratorIds, actualIds = owned.map(([id]) => id).sort();
    expect(actualIds).toEqual(expectedIds);
    expect(oracleIds(readFileSync(join(contractRoot, "../../🔣️taxonomy.json"), "utf8"))).toEqual(expectedIds);
    for (const altered of [actualIds.slice(1), [...actualIds, "unexpected-generator"], ["unexpected-generator", ...actualIds.slice(1)]]) expect(altered.sort()).not.toEqual(expectedIds);
    for (const malformed of ['{"generatorContracts":{},"generatorContracts":{}}', '{"generatorContracts":{"duplicate":{"ownership":"owned"},"duplicate":{"ownership":"owned"}}}', '{"generatorContracts":{"single":{"ownership":"external","ownership":"owned"}}}']) expect(() => oracleIds(malformed)).toThrow("Duplicate or malformed JSON property");
    for (const [, contract] of owned) {
      const project = contract.target!.slice(0, contract.target!.lastIndexOf(":"));
      if (!contract.previewArguments) expect(contract.previewTarget).toBe(`${project}:preview-generated`);
      else expect(contract.previewTarget!.slice(0, contract.previewTarget!.lastIndexOf(":"))).toBe(project);
      expect(generatorBuildPreviewCommand(contract)).toEqual(["bun", "repo", "run", contract.previewTarget!]);
    }
    expect(external.every((contract) => contract.previewTarget === undefined)).toBe(true);
    expect(validateGeneratorContractsAgainstWorkspace(getWorkspaceRoot(), taxonomy)).toEqual([]);
  });

  test("admits only explicit same-project preview invocations", async () => {
    const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const taxonomy = loadTaxonomy(), actor = taxonomy.generatorContracts["actor-typegen"]!;
    for (const row of fixture.previewRoutes) {
      const contract = { ...actor, target: row.target, checkTarget: undefined, previewTarget: row.previewTarget, previewArguments: row.previewArguments };
      const changed = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": contract } };
      expect(validateTaxonomy(changed).length === 0, JSON.stringify(row)).toBe(row.valid);
    }
  }, 30_000);

  test("rejects missing, external, and non-canonical generator preview targets", () => {
    const taxonomy = loadTaxonomy();
    const actor = taxonomy.generatorContracts["actor-typegen"]!;
    expect(actor.ownerPath).toBe("🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust");
    expect(actor.previewLimits?.timeoutMs).toBe(600_000);
    const locks = taxonomy.generatorContracts["external-cargo-locks"]!;
    const missing = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewTarget: undefined } } } as unknown as Taxonomy;
    const external = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "external-cargo-locks": { ...locks, previewTarget: "workspace:preview-generated" } } } as unknown as Taxonomy;
    const nonCanonical = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewTarget: "@semio-tech/framework-actor-rs:preview" } } } as unknown as Taxonomy;
    expect(validateTaxonomy(missing).some((problem) => problem.includes("previewTarget is required"))).toBe(true);
    expect(validateTaxonomy(external).some((problem) => problem.includes("previewTarget is forbidden"))).toBe(true);
    expect(validateTaxonomy(nonCanonical).some((problem) => problem.includes("must be the exact owner preview-generated target"))).toBe(true);
  });

  test("bounds explicit preview resources and invokes only the declared owner command", async () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json"), "utf8"));
    const { generatorPreviewExecution } = await import("../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts"), { generatorPreviewResourceLimits } = await import("../../../🔍️discovery/🛠️generator-commands/🟦️.ts"), ts = await import("typescript");
    const limitSchema = { type: "object", additionalProperties: false, required: ["maxOutputBytes", "timeoutMs"], properties: { maxOutputBytes: { type: "integer", minimum: 1024, maximum: 536870912 }, timeoutMs: { type: "integer", minimum: 1000, maximum: 600000 } } };
    const oracle = new Ajv({ strict: true }).compile(limitSchema);
    const taxonomy = loadTaxonomy(), actor = taxonomy.generatorContracts["actor-typegen"]!;
    for (const row of fixture.previewLimits) {
      expect(oracle(row.limits), JSON.stringify(row)).toBe(row.valid);
      expect(validateTaxonomy({ ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, previewLimits: row.limits } } }).length === 0).toBe(row.valid);
    }
    const source = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization")), syntax = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    const names = ["assertGeneratorPreviewTarget", "invokeGeneratorPreview"];
    const selected = names.map(name => syntax.statements.find(statement => ts.isFunctionDeclaration(statement) && statement.name?.text === name)?.getText(syntax));
    expect(selected.every(Boolean)).toBe(true);
    const contract = { ...actor, ownerPath: "compiler", target: "@neutral/compiler:generate-browser", previewTarget: "@neutral/compiler:preview-browser", previewArguments: ["browser", "preview"], previewLimits: fixture.previewLimits[0].limits };
    for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(selected.join("\n") + "\nreturn invokeGeneratorPreview;"), ts.transpileModule(selected.join("\n") + "\nreturn invokeGeneratorPreview;", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
      let command = "bun ./📜️script.ts browser preview", cwd = "compiler";
      const invocations: unknown[] = [];
      const invoke = new Function("buildTargetRecord", "requireRecord", "generatorPreviewExecution", "generatorPreviewResourceLimits", "receiveGeneratorPreviewProgressV1", "checkCancellation", "absolutePath", "spawnSync", "parseGeneratorPreviewManifest", "generatorPathCompare", "sha256", javascript)(() => ({ executor: "repo:script", options: { cwd, command } }), requireRecord, generatorPreviewExecution, (generatorPreviewResourceLimits), (await import("../../../🏭️generator/👁️preview/📈️progress/🟦️.ts")).receiveGeneratorPreviewProgressV1, () => {}, join, (executable: string, args: string[], options: unknown) => { invocations.push({ executable, args, options }); return { stdout: "{}\n", stderr: "", status: 0, signal: null }; }, () => ({}), (left: string, right: string) => left.localeCompare(right), () => "digest");
      invoke({ repoRoot: "/neutral" }, "neutral", contract, { exclusions: [] });
      expect(invocations).toMatchObject([{ executable: "bun", args: ["./📜️script.ts", "browser", "preview"], options: { cwd: "/neutral/compiler", maxBuffer: 268435456, timeout: 240000 } }]);
      command = fixture.ownerExecutionRoutes.find((row: { id: string }) => row.id === "native-owner").command.replace("preview-generated", "browser preview");
      cwd = ".";
      invoke({ repoRoot: "/neutral" }, "neutral", contract, { exclusions: [] });
      expect(invocations[1]).toMatchObject({ executable: "bun", args: command.split(" ").slice(1), options: { cwd: "/neutral", maxBuffer: 268435456, timeout: 240000 } });
      command = "bun ./📜️script.ts preview-generated";
      expect(() => invoke({ repoRoot: "/neutral" }, "neutral", contract, { exclusions: [] })).toThrow("exact owner JSON preview");
      expect(invocations).toHaveLength(2);
    }
  });

  test("the actual empty General publisher passes the strict normalization receiver", async () => {
    const workspace = getWorkspaceRoot(), discovery = { ...(await import("../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts")), ...(await import("../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts")), ...(await import("../../../🔍️discovery/🛠️generator-commands/🟦️.ts")) }, progress = await import("../../../🏭️generator/👁️preview/📈️progress/🟦️.ts"), ts = await import("typescript"), esbuild = await import("esbuild");
    const source = normalizationEngineDeclarations(join(contractRoot, "../../🧹️normalization")), syntax = ts.createSourceFile("normalization.ts", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS), names = ["assertGeneratorPreviewTarget", "invokeGeneratorPreview"], functions = names.map(name => syntax.statements.find(statement => ts.isFunctionDeclaration(statement) && statement.name?.text === name)?.getText(syntax));
    expect(functions.every(Boolean)).toBe(true);
    const contract = discovery.loadCatalogTaxonomy().generatorContracts["playground-session"]!, target = JSON.parse(readFileSync(join(workspace, contract.ownerPath!, "📋️project.json"), "utf8")).targets[contract.previewTarget!.slice(contract.previewTarget!.lastIndexOf(":") + 1)], output = join(workspace, contract.outputRoots[0].path), before = readFileSync(output), code = functions.join("\n") + "\nreturn invokeGeneratorPreview;";
    expect(contract.previewProgress).toBeUndefined();
    for (const javascript of [ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText, (await esbuild.transform(code, { loader: "ts", target: "es2022" })).code]) {
      const controller = new AbortController();
      const invoke = new Function("buildTargetRecord", "requireRecord", "generatorPreviewExecution", "generatorPreviewResourceLimits", "receiveGeneratorPreviewProgressV1", "checkCancellation", "absolutePath", "spawnSync", "parseGeneratorPreviewManifest", "generatorPathCompare", "sha256", javascript)(() => target, requireRecord, discovery.generatorPreviewExecution, discovery.generatorPreviewResourceLimits, progress.receiveGeneratorPreviewProgressV1, () => { if (controller.signal.aborted) throw new Error("owned test cancellation"); }, join, spawnSync, parseGeneratorPreviewManifest, (left: string, right: string) => Buffer.from(left).compare(Buffer.from(right)), (text: string) => createHash("sha256").update(text).digest("hex"));
      const actual = invoke({ repoRoot: workspace }, "playground-session", contract, { exclusions: [] });
      expect(actual.manifest.contractId).toBe("playground-session");
      expect(actual.manifest.nodes).toHaveLength(1);
      expect(Buffer.from(actual.manifest.nodes[0].bytesBase64, "base64")).toEqual(before);
      expect(readFileSync(output)).toEqual(before);
      controller.abort();expect(() => invoke({ repoRoot: workspace }, "playground-session", contract, { exclusions: [] })).toThrow("owned test cancellation");
      expect(controller.signal.aborted).toBe(true);
    }
  }, 120000);

  test("routes native generator previews through their declared owner", async () => {
    const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const discovery = await import("../../../🔍️discovery/✅️owned-file-approvals/🟦️.ts");
    const { default: parseArgv } = await import("yargs-parser");
    for (const row of fixture.ownerExecutionRoutes) {
      const contract = { ownership: "owned" as const, ownerPath: row.ownerPath ?? "compiler", target: "@neutral/compiler:generate", previewTarget: "@neutral/compiler:preview-generated" };
      const target = { executor: "repo:script", options: { cwd: row.cwd, command: row.command } };
      if (!row.valid) expect(() => discovery.generatorPreviewExecution(contract, target)).toThrow("exact owner JSON preview");
      else {
        const route = discovery.generatorPreviewExecution(contract, target);
        expect([route.command, ...route.args].join(" "), row.id).toBe(row.command);
        expect(route.cwd, row.id).toBe(row.cwd);
        expect(parseArgv([route.command, ...route.args], { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } }), row.id).toEqual(parseArgv(row.command, { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } }));
      }
    }
  });

  test("admits only a closed compiler input manifest authority", () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json"), "utf8"));
    
    expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["contract"]).toEqual("owned-generator-preview-inventory-v1");expect(fixture["compilerManifest"]["version"]).toEqual(1);
    const taxonomy = loadTaxonomy(), actor = taxonomy.generatorContracts["actor-typegen"]!;
    for (const row of fixture.compilerInputAuthorities) {
      const changed = { ...taxonomy, generatorContracts: { ...taxonomy.generatorContracts, "actor-typegen": { ...actor, compilerInputManifest: row.authority } } };
      expect(validateTaxonomy(changed).length === 0, JSON.stringify(row)).toBe(row.valid);
    }
  });

  test("discovers only exact byte-matched compiler manifest inputs", () => {
    const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const sandbox = mkdtempSync(join(tmpdir(), "compiler-input-manifest-")), manifestPath = "📤️output/🧾️manifest.json";
    mkdirSync(join(sandbox, "🌱️source"), { recursive: true });
    mkdirSync(join(sandbox, "📤️output"), { recursive: true });
    writeFileSync(join(sandbox, "🌱️source/🟦️.ts"), "neutral input\n");
    const contract = { ownership: "owned" as const, ownerPath: "owner", target: "owner:generate", previewTarget: "owner:preview-generated", inputPatterns: ["authority.json"], outputRoots: [{ path: "📤️output", inclusion: "ignored" as const }], compilerInputManifest: { kind: "compiler-input-manifest-v1" as const, manifestOutputPath: manifestPath, manifestSchemaPath: "authority.json", staticAuthorityPath: "authority.json", maxFiles: 2 }, reason: "neutral" };
    const taxonomy = loadSourceTaxonomy({ repoRoot: findWorkspaceRoot(contractRoot) });
    writeFileSync(join(sandbox, "authority.json"), "{}\n");
    writeFileSync(join(sandbox, manifestPath), JSON.stringify(fixture.compilerManifest));
    expect(generatorInputPaths({ repoRoot: sandbox }, contract, taxonomy)).toEqual(["authority.json", "🌱️source/🟦️.ts"]);
    for (const manifest of [
      { ...fixture.compilerManifest, inputs: [{ ...fixture.compilerManifest.inputs[0], bytes: 13 }] },
      { ...fixture.compilerManifest, inputs: [{ ...fixture.compilerManifest.inputs[0], path: "../../📦️packages/escape" }] },
      { ...fixture.compilerManifest, inputs: [...fixture.compilerManifest.inputs, fixture.compilerManifest.inputs[0]] },
      { ...fixture.compilerManifest, extra: true },
    ]) {
      writeFileSync(join(sandbox, manifestPath), JSON.stringify(manifest));
      expect(() => generatorInputPaths({ repoRoot: sandbox }, contract, taxonomy)).toThrow();
    }
  });

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
});

describe("validateTaxonomy", () => {
  test("the shipped vocabulary is internally consistent", () => {
    expect(validateTaxonomy()).toEqual([]);
  });

  test("each rule group of the decomposed validator reports its corruption exactly as the pre-split validator did", () => {
    const base = JSON.parse(readFileSync(join(findWorkspaceRoot(contractRoot), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as Record<string, unknown>;
    const rows: readonly Readonly<{ group: string; path: readonly (string | number)[]; value: unknown; expected: readonly string[] }>[] = [
    { group: "auditSchemaVersionAndEvidence", path: ["schemaVersion"], value: 6, expected: ["schemaVersion must be exactly 7, got 6."] },
    { group: "auditLeafAndReferencePolicies", path: ["physicalLeafRendering"], value: {}, expected: ["physicalLeafRendering must be a non-empty object.", "physicalLeafRendering must declare exact forward-only kind-only rendering, longest source extension, primary authoring extension, and canonical-only runtime lookup."] },
    { group: "auditFileKinds", path: ["fileKinds", "json", "emoji"], value: "x", expected: ["fileKinds[\"json\"].emoji must be one canonical NFC emoji sequence.", "transaction-attempt-preparation must retain its validate-all exact child and duplicate-sibling authority.", "transaction-lease-preparation must admit one complete canonical JSON lease before publication.", "semanticDescendantContracts[\"mutation-fixture-bundle-v1\"].pathBudgetReserve.utf16Units must equal derived longest suffix 28.", "semanticDescendantContracts.mutation-fixture-bundle-v1 must encode the exact 12-node data bundle and exclusive diff alternatives."] },
    { group: "auditScopedAndDirectoryKinds", path: ["scopedFileKinds"], value: [], expected: ["scopedFileKinds must be an object.", "scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority.", "scopedFileKinds.transaction-edit-preimage must remain the exact transaction edit displaced-preimage authority.", "scopedFileKinds.transaction-edit-write-candidate must remain the exact unpublished edit-writer authority.", "scopedFileKinds.transaction-backup-write-candidate must remain the exact unpublished backup-writer authority.", "scopedFileKinds.transaction-json-previous must remain the exact displaced canonical JSON authority.", "scopedFileKinds.transaction-backup-candidate must remain the exact transaction restore preimage authority.", "scopedFileKinds.transaction-postimage-candidate must remain the exact transaction restore postimage authority."] },
    { group: "auditTransactionArtifacts", path: ["scopedFileKinds", "transaction-edit-candidate", "pathPattern"], value: "**/x", expected: ["scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority."] },
    { group: "auditMemberAndDescendantContracts", path: ["semanticPathProjectionProfileRenderers"], value: 1, expected: ["semanticPathProjectionProfileRenderers must be a non-empty object.", "semanticPathProjectionContracts[\"artifact-example-model-catalog-v1\"].profileRendererId is missing.", "semanticPathProjectionContracts[\"artifact-example-model-catalog-v1\"] has invalid forward profile authority: Invalid forward profile vector for artifact-example-model-catalog-v1: Unknown or incomplete semantic projection \"artifact-example-model-catalog-v1\".", "semanticPathProjectionContracts[\"artifact-editor-command-bundle-v1\"].profileRendererId is missing.", "semanticPathProjectionContracts[\"artifact-editor-command-bundle-v1\"] has invalid forward profile authority: Invalid forward profile vector for artifact-editor-command-bundle-v1: Unknown or incomplete semantic projection \"artifact-editor-command-bundle-v1\"."] },
    { group: "auditProjectionAndPackageContracts", path: ["semanticPolicyStateCoordinateContracts"], value: 1, expected: ["semanticPolicyStateCoordinateContracts must be a non-empty object."] },
    { group: "auditOwnedFileProjection", path: ["semanticOwnedFileProjectionContracts"], value: {}, expected: ["semanticOwnedFileProjectionContracts must be a non-empty object.", "semanticOwnedFileProjectionContracts must contain the exact artifact-facet, README/LICENSE, ticket-document, active ticket-important, and history ticket-important contracts."] },
    { group: "auditReferenceConsumersAndMutationCatalog", path: ["semanticPathProjectionReferenceConsumerContracts"], value: 1, expected: ["semanticPathProjectionReferenceConsumerContracts must be a non-empty object."] },
    { group: "auditFixedContracts", path: ["fixedFilenameRejectionContracts"], value: 1, expected: ["fixedFilenameRejectionContracts must be a non-empty object."] },
    { group: "auditPackageBoundaryContracts", path: ["packageSourceDispositions"], value: 1, expected: ["packageSourceDispositions must be a non-empty object."] },
    { group: "auditGeneratorContracts", path: ["generatorContracts"], value: {}, expected: ["semanticOwnedFileProjectionContracts[\"readme-license-owner-leaves-v1\"] has an unregistered generator owner.", "generatorContracts must be a non-empty object."] },
    { group: "auditPathPoliciesAndAreas", path: ["unicodeNormalization"], value: {}, expected: ["unicodeNormalization must be NFC/lower/und."] },
    { group: "auditDependencyDirections", path: ["areaLayers"], value: {}, expected: ["areaLayers must be a non-empty object."] },
    { group: "auditEcosystemsAndTargets", path: ["targets", "__probe__"], value: {}, expected: ["targets[\"__probe__\"].lang is missing.", "targets[\"__probe__\"].componentFileKindId is missing.", "targets[\"__probe__\"].entryContractIds must be an array."] },
    { group: "auditTestVocabulary", path: ["testExamplesDirName"], value: "x", expected: ["testExamplesDirName must select the canonical \"📚️examples\" collection.", "Semantic directory \"x\" is not uniquely registered."] },
    { group: "auditManifestAndRootContracts", path: ["rootDataContractIds"], value: ["missing"], expected: ["rootDataContractIds references missing id \"missing\"."] },
    { group: "auditSchemaFormatsAndFacets", path: ["schemaJsonDialect"], value: "x", expected: ["schemaJsonDialect must be the single dialect mutationPayloadSchemaAuthority already declares."] },
    { group: "auditMutationVocabulary", path: ["mutationComponentFileKindId"], value: "x", expected: ["mutationComponentFileKindId references a missing file kind.", "mutationComponentFileKindId must select the direct owner \"rust-source\" kind."] },
    { group: "auditRepresentationAndChildDirectories", path: ["nativeRepresentationDirs"], value: [], expected: ["nativeRepresentationDirs must be a non-empty unique subset of representationDirs."] },
    ];
    expect(new Set(rows.map((row) => row.group)).size).toBe(20);
    for (const row of rows) {
      const clone = structuredClone(base) as Record<string, any>;
      const parent = row.path.slice(0, -1).reduce<any>((value, key) => value[key], clone);
      parent[row.path[row.path.length - 1]!] = row.value;
      expect(validateTaxonomy(clone as unknown as Taxonomy), `${row.group}: ${row.path.join(".")}`).toEqual([...row.expected]);
    }
  }, 120_000);

  test("no rule group, audit context or scope vocabulary module reaches the authority module that injects its ports", () => {
    const library = join(findWorkspaceRoot(contractRoot), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"), discovery = join(library, "🔍️discovery"), authority = join(discovery, "🔐️taxonomy-authority/🟦️.ts");
    const relativeImports = (file: string): string[] => [...readFileSync(file, "utf8").matchAll(/^(?:import|export)\s[^;]*?\sfrom\s"(\.[^"]+)";$/gmu)].map((match) => join(file, "..", match[1]!));
    const owners = relativeImports(authority).filter((file) => /\/(?:[^/]*audit[^/]*)\/🟦️\.ts$/u.test(file));
    expect(owners).toHaveLength(21);
    for (const owner of owners) {
      expect(readFileSync(owner, "utf8").split("\n").length, owner).toBeLessThanOrEqual(600);
      const seen = new Set<string>([owner]), queue = [owner];
      while (queue.length > 0) for (const next of relativeImports(queue.pop()!)) if (!seen.has(next)) { seen.add(next); queue.push(next); }
      expect(seen.has(authority), `${owner} reaches the authority module`).toBe(false);
    }
  });

  test("freezes the four canonical testing collections and the obsolete-category stems", () => {
    const taxonomy = JSON.parse(readFileSync(join(findWorkspaceRoot(contractRoot), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as Taxonomy;
    expect({ tests: taxonomy.testsDirName, fixtures: taxonomy.testFixturesDirName, examples: taxonomy.testExamplesDirName, oracles: taxonomy.testOraclesDirName }).toEqual({ tests: "🧪️tests", fixtures: "🧫️fixtures", examples: "📚️examples", oracles: "🔮️oracles" });
    expect("testContributionDirName" in taxonomy).toBe(false);
    expect("testContributionDirectoryOverrides" in taxonomy).toBe(false);
    const singularOracle = { ...taxonomy, testOraclesDirName: "🔮️oracle" } as Taxonomy;
    expect(validateTaxonomy(singularOracle).some((problem) => problem.includes('canonical "🔮️oracles" collection'))).toBe(true);
    const singularExample = { ...taxonomy, testExamplesDirName: "📚️example" } as Taxonomy;
    expect(validateTaxonomy(singularExample).some((problem) => problem.includes('canonical "📚️examples" collection'))).toBe(true);
    const duplicateStem = { ...taxonomy, testObsoleteCategoryStems: [...taxonomy.testObsoleteCategoryStems, taxonomy.testObsoleteCategoryStems[0]!] } as Taxonomy;
    expect(validateTaxonomy(duplicateStem).some((problem) => problem.includes("testObsoleteCategoryStems") && problem.includes("unique normalized lowercase stems"))).toBe(true);
    const uppercaseStem = { ...taxonomy, testObsoleteTestEmojiCategoryStems: [...taxonomy.testObsoleteTestEmojiCategoryStems, "Helper"] } as Taxonomy;
    expect(validateTaxonomy(uppercaseStem).some((problem) => problem.includes("testObsoleteTestEmojiCategoryStems") && problem.includes("unique normalized lowercase stems"))).toBe(true);
  });

  test("discriminates catalog descendants from exact bundles and diagnoses missing exact alternatives", () => {
    const taxonomy = structuredClone(loadTaxonomy()) as Taxonomy;
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    const malformed = structuredClone(taxonomy) as Taxonomy;
    delete (malformed.semanticDescendantContracts["draw-editor-command-bundle-v1"] as unknown as { exclusiveAlternatives?: unknown }).exclusiveAlternatives;
    expect(validateTaxonomy(malformed).some((problem) => problem.includes("draw-editor-command-bundle-v1") && problem.includes("exclusiveAlternatives must be an array"))).toBe(true);
    const wrongCatalog = structuredClone(taxonomy) as Taxonomy;
    (wrongCatalog.semanticDescendantContracts["cad-model-catalog-bundle-v1"] as unknown as { catalogContractId: string }).catalogContractId = "draw-editor-command-vectors-v1";
    expect(validateTaxonomy(wrongCatalog).some((problem) => problem.includes("cad-model-catalog-bundle-v1") && problem.includes("distributed JSON manifest catalog"))).toBe(true);
  });

  test("declares schema facet kinds partitioning data vs interface formats", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaFacetKinds?.["🧬️data"]?.normativeFormat).toBe("🔣️jsonschema");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.normativeFormat).toBe("📜️wit");
    expect(taxonomy.schemaFacetKinds?.["📜️interface"]?.formats).toEqual(["📜️wit"]);
    expect(taxonomy.schemaFormats?.["📜️wit"]?.fieldCasing).toBe("kebab");
    expect(taxonomy.packagingDirectoryKindIds).toEqual(["targets", "fixtures", "apps"]);
    expect(taxonomy.ecosystems["🦀️rust"]?.packagingDirectoryKindIds).toEqual(["benchmarks"]);
  });

  test("declares canonical semantic collection and module ownership contracts", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.semanticManifestFileKindId).toBe("json");
    expect(taxonomy.semanticExtensionKey).toBe("x-semio");
    expect(taxonomy.semanticConsumerMinimum).toBe(2);
    expect(taxonomy.semanticCollections["🔨️modules"]?.kind).toBe("module");
    expect(taxonomy.semanticCollections["💡️inferences"]?.kind).toBe("inference");
    expect(taxonomy.semanticCollections["🚪️io/📝️text/🧬️mutations"]).toEqual({ kind: "io", direction: "transport" });
    expect(taxonomy.ioSemanticCollectionDirNames).toEqual(["📸️snapshot", "🔺️diff", "💡️inferences", "🧬️mutations"]);
    expect(artifactFacetPathIsDeclared("🚪️io/📝️text/🧬️mutations", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/💾️binary/🧬️mutations", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/📝️text/🧬️mutations", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🚪️io/💾️binary/🧬️mutations", taxonomy)).toBe(false);
    expect(taxonomy.ioSemanticCollectionDirNames).toContain("🧬️mutations");
  });

  test("declares the surface vocabulary ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET froze", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaVersion).toBe(7);
    expect(taxonomy.viewerDirName).toBe("👁️viewer");
    expect(taxonomy.editorDirName).toBe("✏️editor");
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: "👁️viewer", editor: "✏️editor" });
    expect(taxonomy.subsetChildDirs).toContain("👁️viewer");
    expect(taxonomy.subsetChildDirs).toContain("✏️editor");
    expect(taxonomy.subsetRequiredSurfaceDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.contributedSubsetChildDirs).toEqual(["👁️viewer", "✏️editor"]);
    expect(taxonomy.surfaceChildDirs).toContain(taxonomy.modesDirName);
    expect(taxonomy.surfaceComponentLangs).toEqual(["🦀️rust", "🟦️typescript"]);
    expect(taxonomy.windowLeafLangs).toEqual(["🦀️rust", "🟦️typescript"]);
  });

  test("surfaceRoles preserves the AppRole declaration and wire-tag order", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceRoles).toEqual(["viewer", "editor"]);
  });

  test("surface role mappings name their registered v7 directories", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceDirNames).toEqual({ viewer: taxonomy.viewerDirName, editor: taxonomy.editorDirName });
  });

  test("required surface children belong to the structural set and cover every state lane", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.surfaceRequiredChildDirs.every((dir) => taxonomy.surfaceChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.surfaceRequiredChildDirs).toEqual(expect.arrayContaining(["🎚️config", "👥️presence", "🫧️transient"]));
  });

  test("every surface directory belongs to the subset structural set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.subsetSurfaceDirs.every((dir) => taxonomy.subsetChildDirs.includes(dir))).toBe(true);
  });

  test("rejects any area outside the required clean state", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, "🧰️framework": "taxonomy" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('must be "clean"'))).toBe(true);
  });

  test("keeps plugin roots inside the clean area registry", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginAreas.every((area) => taxonomy.areas[area] === "clean")).toBe(true);
    const broken = { ...taxonomy, areas: { ...taxonomy.areas, [taxonomy.pluginAreas[0]!]: "mixed" as never } };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('must be "clean"'))).toBe(true);
  });

  test("reports a completeness dir missing from the structural set", () => {
    const taxonomy = loadTaxonomy();
    const broken = { ...taxonomy, artifactChildDirs: taxonomy.artifactChildDirs.filter((dir) => dir !== "🚪️io") };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🚪️io"))).toBe(true);
  });

  test("keeps derived lifecycle directories out of the subset structural set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.subsetChildDirs.filter((dir) => ["🏗️builder", "🧐️analyzer", "🎹️composer"].includes(dir))).toEqual([]);
  });

  test("registers every mutation facet (behavior and organizational) as a taxonomy leaf parent", () => {
    const taxonomy = loadTaxonomy();
    expect([...taxonomy.mutationBehaviorFacetDirs, ...taxonomy.mutationOrganizationalFacetDirs].every((dir) => taxonomy.taxonomyLeafParentDirs.includes(dir))).toBe(true);
  });

  test("declares direct mutation ownership plus placement-bound behavior and optional organizational subfacets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.mutationComponentFileKindId).toBe("rust-source");
    expect(taxonomy.mutationDescriptorFileKindId).toBe("json");
    expect(taxonomy.mutationBehaviorFacetDirs).toEqual(["🦠️mutation", "🔺️diff", "↩️inverse"]);
    expect(taxonomy.mutationOrganizationalFacetDirs).toEqual(["🧩️plan", "📝️text", "💾️binary", "🧬️schema"]);
    expect(new RegExp(taxonomy.mutationDirectoryPattern, "u").test("➕️add-node")).toBe(true);
    expect(new RegExp(taxonomy.mutationDirectoryPattern, "u").test("➕️node")).toBe(false);
    expect("mutationChildDirs" in taxonomy).toBe(false);
    expect("compositeMutationChildDirs" in taxonomy).toBe(false);
    expect(taxonomy.schemaChildDirs).toContain("🧬️mutations");
  });

  test("keeps required window capabilities non-empty and structural", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.windowRequiredChildDirs.length).toBeGreaterThan(0);
    expect(taxonomy.windowRequiredChildDirs.every((dir) => taxonomy.windowChildDirs.includes(dir))).toBe(true);
  });

  test("declares command facets at plugin and OS ownership boundaries", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.osRequiredChildDirs).toContain("🎮️commands");
    expect(taxonomy.pluginRequiredChildDirs.every((dir) => taxonomy.pluginChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.osRequiredChildDirs.every((dir) => taxonomy.osChildDirs.includes(dir))).toBe(true);
  });

  test("declares every state lane in each state-owning scope", () => {
    const taxonomy = loadTaxonomy();
    const stateLanes = ["🎚️config", "👥️presence", "🫧️transient"];
    expect(stateLanes.every((dir) => taxonomy.modeChildDirs.includes(dir))).toBe(true);
    expect(stateLanes.every((dir) => taxonomy.windowRequiredChildDirs.includes(dir))).toBe(true);
    expect(taxonomy.modeChildDirs).toContain(taxonomy.windowsDirName);
  });

  test("reports an invalid window component language set", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.windowComponentLangs.every((lang) => Boolean(taxonomy.componentFileKinds[lang]))).toBe(true);
    const missingMarkerKind = { ...taxonomy, windowEmptyFacetFileKindId: "unknown" };
    expect(validateTaxonomy(missingMarkerKind).some((problem) => problem.includes("windowEmptyFacetFileKindId references a missing file kind"))).toBe(true);
  });

  test("declares direct plugin-root facets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginChildDirs).toEqual(["🎮️commands", "🔨️modules", "🧬️schema"]);
  });

  test("includes commands at every command-owning scope", () => {
    const taxonomy = loadTaxonomy();
    for (const key of ["surfaceChildDirs", "modeChildDirs", "pluginChildDirs", "osChildDirs"] as const) {
      expect(taxonomy[key]).toContain("🎮️commands");
    }
  });

  test("keeps direct plugin-root facet declarations non-empty", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.pluginChildDirs.every((dir) => dir.length > 0)).toBe(true);
  });

  test("rejects plural example component dirs in taxonomyLeafParentDirs", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      taxonomyLeafParentDirs: [...taxonomy.taxonomyLeafParentDirs, "🎒️packs"],
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("🎒️packs"))).toBe(true);
  });

  test("declares a valid example slug pattern", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.exampleSlugPattern.length).toBeGreaterThan(0);
    expect(() => new RegExp(taxonomy.exampleSlugPattern, "u")).not.toThrow();
  });

  test("declares the schema lifecycle collections", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.schemaChildDirs).toEqual(["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"]);
  });

  test("declares the schema representation kinds", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.representationDirs).toEqual(["📝️text", "💾️binary", "🪶️sqlite"]);
    expect(taxonomy.nativeRepresentationDirs).toEqual(["📝️text", "💾️binary"]);
  });

  test("keeps bare pack out of artifact structural and completeness sets", () => {
    const taxonomy = loadTaxonomy();
    expect(taxonomy.artifactComponentDirs).not.toContain("🎒️pack");
    expect(taxonomy.artifactChildDirs).not.toContain("🎒️pack");
  });

  test("keeps artifact specification mappings on fully qualified v7 facet paths", () => {
    const taxonomy = loadTaxonomy();
    expect("🎒️pack" in taxonomy.artifactSpecFileKinds).toBe(false);
    expect(taxonomy.artifactSpecFileKinds["🚪️io/💾️binary/📸️snapshot"]).toBe("protocol-semio");
  });

  test("reports a schema format referencing an unknown file kind", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      schemaFormats: {
        ...taxonomy.schemaFormats,
        "🦀️rust": { ...taxonomy.schemaFormats["🦀️rust"], fileKindId: "unknown" },
      },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes('schemaFormats["🦀️rust"].fileKindId is missing'))).toBe(true);
  });

  test("reports artifact schema mappings referencing an unknown file kind", () => {
    const taxonomy = loadTaxonomy();
    const broken = {
      ...taxonomy,
      artifactSchemaSpecFileKinds: { ...taxonomy.artifactSchemaSpecFileKinds, "🧬️schema": "unknown" },
    };
    expect(validateTaxonomy(broken).some((problem) => problem.includes("artifactSchemaSpecFileKinds") && problem.includes("unknown"))).toBe(true);
  });
});
