import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { artifactFacetPathIsDeclared, mutationDirectLeafInlinedBehaviorFacets } from "../../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { buildSemanticCensus, renderSemanticCensusJson, resolveRustPathAttributes } from "../../../🔍️discovery/🧩️semantic-census/🟦️.ts";
import { inspectRustStructure } from "../../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { inspectRustVirtualSources, renderRustStructuralFactsJson } from "../../../🔍️discovery/🔎️rust-source-identities/🟦️.ts";
import { type Taxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";

const DIRECT_MUTATION_RUST_FIXTURE = `
#![allow(dead_code)]

pub struct Snapshot;
pub struct AddNode {
    pub id: u64,
    pub label: String,
}

#[path = "➕️add-node/🦀️.rs"]
pub mod add_node;

#[cfg(test)]
#[path = "🧪️tests/add-node/🦀️.rs"]
mod tests;

pub enum Mutation {
    AddNode(add_node::AddNode),
    RemoveNode { id: u64 },
    Noop,
}

impl protocol::MutationKind<Snapshot, Mutation> for AddNode {
    const ID: &'static str = "add-node";

    fn diff(&self, _base: &Snapshot) -> u64 {
        self.id
    }

    fn inverse(&self, _base: &Snapshot) -> Vec<Mutation> {
        Vec::new()
    }
}

pub const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
    verb: "add",
    entity: "node",
    kind: "add-node",
    record: "AddNode",
};
pub const WIRE_KIND: &str = "add-node";
const GENERATED: &str = include!(concat!(env!("OUT_DIR"), "/mutations.rs"));
const FAKE_RUST: &str = r#"pub enum Fake { Bad(fake::Bad) }"#;
// pub enum CommentFake { Bad(comment_fake::Bad) }

pub fn dispatch(value: &Mutation) -> u64 {
    match value {
        Mutation::AddNode(payload) => payload.id,
        Mutation::RemoveNode { id } => *id,
        Mutation::Noop => 0,
    }
}
`;

/** 🚨️ Collapsed shape: a direct leaf that re-inlines diff and inverse behavior instead of splitting it into 🔺️diff/↩️inverse. */
const INLINED_MUTATION_DIRECT_LEAF_FIXTURE = `//! ❌️ \`delete-node\` — removes an id-keyed graph node, cascading to every edge incident on it.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
pub struct DeleteNode {
    pub id: String,
}

pub async fn diff(payload: &DeleteNode, base: &Snapshot) -> Outcome {
    todo!()
}

pub async fn inverse(payload: &DeleteNode, base: &Snapshot) -> Vec<Mutation> {
    todo!()
}
`;

/** ✅️ Split shape (\`🌿️vcs/🏷️add-tag\`): the direct leaf keeps only its payload + dispatch, delegating diff/inverse to their own facets. */
const SPLIT_MUTATION_DIRECT_LEAF_FIXTURE = `//! 🏷️ VCS mutation — \`AddTag\`: attaches a set-like tag member to the document.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
pub struct AddTag {
    pub tag: String,
}

impl protocol::MutationKind<VcsSnapshot, VcsDemoMutation> for AddTag {
    fn diff(&self, base: &VcsSnapshot) -> protocol::MutationOutcome<VcsDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
        super::inverse::inverse(self, base)
    }
}
`;

/** 🧫️ Marker-free collapsed shape (real \`🏛️architect/…/🗑️🧱delete-program-element/🦀️.rs\` structure): free \`pub async fn diff\`/\`pub async fn inverse\` with no \`//#region\` at all and no sibling facet dir — the 266-mutation blind spot the marker-only detector missed. */
const MARKER_FREE_INLINED_MUTATION_DIRECT_LEAF_FIXTURE = `//! 🦠️ ProgramSnapshot mutation — \`delete-program-element\` leaf (delete).

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
pub struct DeleteProgramElement {
    pub id: EntityId,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for DeleteProgramElement {
    async fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        diff(self, base)
    }
    async fn inverse(&self, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
        inverse(self, base)
    }
}

pub async fn diff(payload: &DeleteProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    todo!()
}

pub async fn inverse(payload: &super::DeleteProgramElement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    todo!()
}
`;

describe("direct mutation taxonomy", () => {
  test("models direct ownership plus behavior and organizational child facets", () => {
    const taxonomy = loadTaxonomy();
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/➕️add-node", taxonomy)).toBe(true);
    for (const facet of [...taxonomy.mutationBehaviorFacetDirs, ...taxonomy.mutationOrganizationalFacetDirs]) expect(artifactFacetPathIsDeclared(`🧬️schema/🧬️mutations/➕️add-node/${facet}`, taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/🧬️mutations/➕️node", taxonomy)).toBe(false);
  });

  test("flags a direct leaf that inlines diff/inverse behavior (marked or marker-free) and accepts the split add-tag shape", () => {
    expect(mutationDirectLeafInlinedBehaviorFacets(INLINED_MUTATION_DIRECT_LEAF_FIXTURE, new Set())).toEqual(["↩️inverse", "🔺️diff"]);
    expect(mutationDirectLeafInlinedBehaviorFacets(SPLIT_MUTATION_DIRECT_LEAF_FIXTURE, new Set())).toEqual([]);
    // 🏛️architect regression: no `//#region` marker at all — only the structural free-function check catches this.
    expect(mutationDirectLeafInlinedBehaviorFacets(MARKER_FREE_INLINED_MUTATION_DIRECT_LEAF_FIXTURE, new Set())).toEqual(["↩️inverse", "🔺️diff"]);
    // A sibling facet directory already existing suppresses the structural flag for that facet, marker-free source unchanged.
    expect(mutationDirectLeafInlinedBehaviorFacets(MARKER_FREE_INLINED_MUTATION_DIRECT_LEAF_FIXTURE, new Set(["🔺️diff", "↩️inverse"]))).toEqual([]);
  });

  test("extracts stable Rust structural facts without treating comments or strings as syntax", () => {
    const facts = inspectRustStructure(DIRECT_MUTATION_RUST_FIXTURE);
    expect(facts.schemaVersion).toBe(1);
    expect(facts.modules).toEqual([
      { name: "add_node", visibility: "pub", inline: false, pathTarget: "➕️add-node/🦀️.rs", cfgTest: false },
      { name: "tests", visibility: "private", inline: false, pathTarget: "🧪️tests/add-node/🦀️.rs", cfgTest: true },
    ]);
    expect(facts.enums).toEqual([{
      name: "Mutation",
      visibility: "pub",
      variants: [
        { name: "AddNode", fieldStyle: "tuple", fieldTypes: ["add_node::AddNode"], wrappedTupleLeafType: "add_node::AddNode" },
        { name: "RemoveNode", fieldStyle: "struct", fieldTypes: ["u64"], wrappedTupleLeafType: null },
        { name: "Noop", fieldStyle: "unit", fieldTypes: [], wrappedTupleLeafType: null },
      ],
    }]);
    expect(facts.inlinePayloads.find((payload) => payload.name === "AddNode")?.fields).toEqual([
      { name: "id", type: "u64" },
      { name: "label", type: "String" },
    ]);
    expect(facts.impls).toContainEqual({
      traitPath: "protocol::MutationKind<Snapshot,Mutation>",
      selfType: "AddNode",
      methods: ["diff", "inverse"],
      associatedConstants: ["ID"],
    });
    expect(facts.matchArms.map((arm) => arm.variantPath)).toEqual(["Mutation::AddNode", "Mutation::RemoveNode", "Mutation::Noop"]);
    expect(facts.constants.find((constant) => constant.name === "SEMANTICS")?.identityFields).toEqual({ entity: "node", kind: "add-node", record: "AddNode", verb: "add" });
    expect(facts.constants.find((constant) => constant.name === "WIRE_KIND")?.stringValue).toBe("add-node");
    expect(facts.includes).toEqual([{ macro: "include", expression: 'concat!(env!("OUT_DIR"),"/mutations.rs")', usesOutDir: true }]);
    expect(facts.testModules).toEqual([facts.modules[1]]);
    expect(renderRustStructuralFactsJson(facts)).toBe(renderRustStructuralFactsJson(inspectRustStructure(DIRECT_MUTATION_RUST_FIXTURE)));
  });

  test("filters virtual opaque paths before reading and normalizes Windows separators", () => {
    const outside = "🧰️framework/🧬️schema/🧬️mutations/➕️add-node/🦀️.rs";
    const opaque = "compose/🧬️schema/🧬️mutations/➕️add-node/🦀️.rs";
    const recoveredOpaque = "temp\\compose\\🧬️schema\\🧬️mutations\\➕️add-node\\🦀️.rs";
    const reads: string[] = [];
    const inspected = inspectRustVirtualSources([opaque, outside, recoveredOpaque], (path) => {
      reads.push(path);
      if (path !== outside) throw new Error(`opaque virtual source was read: ${path}`);
      return DIRECT_MUTATION_RUST_FIXTURE;
    });
    expect(reads).toEqual([outside]);
    expect(inspected.map((entry) => entry.path)).toEqual([outside]);
    expect(inspected[0]?.facts.enums[0]?.name).toBe("Mutation");
  });

  test("agrees with the nightly Rust parser on every declared structural identity", () => {
    const facts = inspectRustStructure(DIRECT_MUTATION_RUST_FIXTURE);
    const parsed = spawnSync("rustc", ["-Zunpretty=ast-tree", "--crate-type=lib", "-"], { input: DIRECT_MUTATION_RUST_FIXTURE, encoding: "utf8" });
    expect(parsed.status).toBe(0);
    const ast = parsed.stdout;
    for (const identity of [...facts.modules.map((fact) => fact.name), ...facts.enums.map((fact) => fact.name), ...facts.inlinePayloads.map((fact) => fact.name), ...facts.constants.map((fact) => fact.name)]) expect(ast).toContain(identity);
  });
});

function semanticFixture(options: { readonly secondProductionConsumer?: boolean; readonly rootBehavior?: boolean; readonly rustRelativeConsumers?: boolean; readonly glueConsumer?: boolean } = {}): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-semantic-census-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🔨️modules/📏measure/🟦️.ts", "export const measure = (value: number) => value;\n");
  if (options.rustRelativeConsumers || options.glueConsumer) write("🧰️framework/🔨️modules/📏measure/🦀️.rs", "pub fn measure(value: u32) -> u32 { value }\n");
  write("🧰️framework/🔨️modules/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: "📏measure", id: "measure", kind: "module", responsibility: "exact and stable numeric measurement", module: { productionConsumers: options.secondProductionConsumer ? ["height", "width"] : ["width"] } }] } }));
  write("🧰️framework/💡️inferences/📏width/🟦️.ts", 'import { measure } from "../../🔨️modules/📏measure/🟦️";\nexport const width = measure(1);\n');
  if (options.secondProductionConsumer) write("🧰️framework/💡️inferences/↕️height/🟦️.ts", 'import { measure } from "../../🔨️modules/📏measure/🟦️";\nexport const height = measure(1);\n');
  if (options.rustRelativeConsumers) {
    write("🧰️framework/💡️inferences/📏width/🦀️.rs", "use super::super::modules::measure::measure;\npub fn width() -> u32 { measure(1) }\n");
    if (options.secondProductionConsumer) write("🧰️framework/💡️inferences/↕️height/🦀️.rs", "use super::super::modules::measure::measure;\npub fn height() -> u32 { measure(1) }\n");
  }
  if (options.glueConsumer) write("🧰️framework/💡️inferences/📏width/📦️packages/🦀️rust/🦀️.rs", '#[path = "../../../../🔨️modules/📏measure/🦀️.rs"]\npub mod measure;\n');
  write("🧰️framework/💡️inferences/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📏width", id: "width", kind: "inference", responsibility: "derived width", inference: { inputs: ["value"], target: "width" } },
    ...(options.secondProductionConsumer ? [{ directory: "↕️height", id: "height", kind: "inference", responsibility: "derived height", inference: { inputs: ["value"], target: "height" } }] : []),
  ] } }));
  if (options.rootBehavior) write("🧰️framework/💡️inferences/🟦️.ts", "export function analyze() { return 1; }\n");
  return { root, taxonomy };
}

function schemaMountedRustModulesFixture(): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-schema-mounted-rust-modules-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  write("🧰️framework/🔨️modules/📏measure/🦀️.rs", "pub fn measure(value: u32) -> u32 { value }\n");
  write("🧰️framework/🔨️modules/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: "📏measure", id: "measure", kind: "module", responsibility: "exact and stable numeric measurement", module: { productionConsumers: ["height", "width"] } }] } }));
  write("🧰️framework/🧬️schema/💡️inferences/📏width/🦀️.rs", "use super::super::modules::measure::measure;\npub fn width() -> u32 { measure(1) }\n");
  write("🧰️framework/🧬️schema/💡️inferences/↕️height/🦀️.rs", "use super::super::modules::measure::measure;\npub fn height() -> u32 { measure(1) }\n");
  write("🧰️framework/🧬️schema/💡️inferences/🔣️.json", JSON.stringify({ "x-semio": { kind: "collection", members: [
    { directory: "📏width", id: "width", kind: "inference", responsibility: "derived width", inference: { inputs: ["value"], target: "width" } },
    { directory: "↕️height", id: "height", kind: "inference", responsibility: "derived height", inference: { inputs: ["value"], target: "height" } },
  ] } }));
  return { root, taxonomy };
}

function mutationTransportFixture(): { readonly root: string; readonly taxonomy: Taxonomy } {
  const root = mkdtempSync(join(tmpdir(), "semio-mutation-transport-"));
  const taxonomy = loadTaxonomy();
  const write = (path: string, content: string): void => {
    const absolute = join(root, path);
    mkdirSync(join(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  };
  const codec = (representation: string, member: string, id: string, format: string, source: string): void => {
    write(`🧰️framework/🚪️io/${representation}/🧬️mutations/${member}/🟦️.ts`, source);
    write(`🧰️framework/🚪️io/${representation}/🧬️mutations/🔣️.json`, JSON.stringify({ "x-semio": { kind: "collection", members: [{ directory: member, id, kind: "io", responsibility: `frozen ${format} transport`, io: { format, direction: "transport" } }] } }));
  };
  codec("📝️text", "🧾️mutation-text", "framework.io.mutation-text", "Semio mutation text", "export const encodeMutationText = (value: string) => value;\n");
  codec("💾️binary", "🧾️mutation-binary", "framework.io.mutation-binary", "Semio mutation binary", "export const encodeMutationBinary = (value: Uint8Array) => value;\n");
  return { root, taxonomy };
}

describe("semantic collection census", () => {
  test("accepts a genuine two-production-component module at its lowest common owner", () => {
    const fixture = semanticFixture({ secondProductionConsumer: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
      expect(census.records.find((record) => record.id === "measure")?.computedLowestCommonOwner).toBe("🧰️framework");
      expect(renderSemanticCensusJson(census)).toBe(renderSemanticCensusJson(buildSemanticCensus(fixture.root, {}, fixture.taxonomy)));
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves relative Rust and TypeScript imports to terminal production components", () => {
    const fixture = semanticFixture({ secondProductionConsumer: true, rustRelativeConsumers: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      const imports = census.graph.edges.filter((edge) => edge.to === "measure" && edge.mechanism === "static-import");
      expect(imports.map((edge) => edge.from)).toEqual(["height", "height", "width", "width"]);
      expect(imports.map((edge) => edge.target.endsWith("🦀️.rs") || edge.target.endsWith("🟦️.ts"))).toEqual([true, true, true, true]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves logical schema modules to their closest physical module collection", () => {
    const fixture = schemaMountedRustModulesFixture();
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.graph.edges.filter((edge) => edge.to === "measure" && edge.mechanism === "static-import").map((edge) => edge.from)).toEqual(["height", "width"]);
      expect(census.records.find((record) => record.id === "measure")?.productionConsumers).toEqual(["height", "width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("rejects a one-consumer module and authored list-root behavior", () => {
    const fixture = semanticFixture({ rootBehavior: true });
    try {
      const codes = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code);
      expect(codes).toContain("module-consumer-minimum");
      expect(codes).toContain("module-production-consumer-minimum");
      expect(codes).toContain("collection-authored-behavior");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("includes unregistered collection-path findings beneath a semantic owner scope", () => {
    const fixture = semanticFixture({ rootBehavior: true });
    try {
      const manifestPath = join(fixture.root, "🧰️framework/💡️inferences/🔣️.json");
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      manifest["x-semio"].members[0].id = "framework.inference.width";
      writeFileSync(manifestPath, JSON.stringify(manifest));
      const scoped = buildSemanticCensus(fixture.root, { scope: "framework" }, fixture.taxonomy);
      expect(scoped.records.map((record) => record.id)).toEqual(["framework.inference.width"]);
      expect(scoped.problems.some((problem) => problem.code === "collection-authored-behavior" && problem.path.endsWith("💡️inferences/🟦️.ts"))).toBe(true);
      expect(scoped.problems.some((problem) => problem.code === "module-production-consumer-minimum" && problem.path.endsWith("🔨️modules/📏measure"))).toBe(true);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("accepts mutation codecs only as explicit bidirectional I/O transport boundaries", () => {
    const fixture = mutationTransportFixture();
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      expect(census.problems).toEqual([]);
      expect(census.records.map((record) => record.id)).toEqual(["framework.io.mutation-binary", "framework.io.mutation-text"]);
      const manifestPath = join(fixture.root, "🧰️framework/🚪️io/📝️text/🧬️mutations/🔣️.json");
      const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
      manifest["x-semio"].members[0].io.direction = "import";
      writeFileSync(manifestPath, JSON.stringify(manifest));
      expect(buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code)).toContain("io-contract-missing");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("does not count a test call site as a second production component", () => {
    const fixture = semanticFixture();
    try {
      const testPath = join(fixture.root, "🧰️framework/💡️inferences/📏width/🧪️tests/🟦️.ts");
      mkdirSync(join(testPath, ".."), { recursive: true });
      writeFileSync(testPath, 'import { measure } from "../../../🔨️modules/📏measure/🟦️";\nexport const checked = measure(1);\n');
      const module = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).records.find((record) => record.id === "measure");
      expect(module?.productionConsumers).toEqual(["width"]);
      expect(module?.excludedConsumers).toEqual(["width"]);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("does not count a package glue mount as a production consumer", () => {
    const fixture = semanticFixture({ glueConsumer: true });
    try {
      const census = buildSemanticCensus(fixture.root, {}, fixture.taxonomy);
      const glueEdge = census.graph.edges.find((edge) => edge.source.endsWith("🦀️.rs") && edge.to === "measure");
      expect(glueEdge?.production).toBe(false);
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("rejects missing manifest members, stale declarations, and generic stems", () => {
    const fixture = semanticFixture();
    try {
      const collection = join(fixture.root, "🧰️framework/💡️inferences");
      mkdirSync(join(collection, "🧪️orphan"), { recursive: true });
      writeFileSync(join(collection, "🟦️orphan.ts"), "export const orphan = true;\n");
      const manifest = JSON.parse(readFileSync(join(collection, "🔣️.json"), "utf8"));
      manifest["x-semio"].members.push({ directory: "🧹️stale", id: "stale", kind: "inference", responsibility: "stale result", inference: { inputs: ["x"], target: "stale" } });
      manifest["x-semio"].members[0].directory = "🧪️shared";
      writeFileSync(join(collection, "🔣️.json"), JSON.stringify(manifest));
      const codes = buildSemanticCensus(fixture.root, {}, fixture.taxonomy).problems.map((problem) => problem.code);
      expect(codes).toContain("manifest-child-missing");
      expect(codes).toContain("manifest-child-extra");
      expect(codes).toContain("member-generic-stem");
    } finally {
      rmSync(fixture.root, { recursive: true, force: true });
    }
  });

  test("resolves cumulative nested Rust path attributes", () => {
    const root = join(tmpdir(), "semantic-rust-path");
    const source = [
      '#[path = "../.."]',
      "pub mod gltf {",
      '  #[path = "."]',
      "  pub mod schema {",
      '    #[path = "metrics/🦀️.rs"]',
      "    pub mod metric;",
      "  }",
      "}",
    ].join("\n");
    expect(resolveRustPathAttributes(join(root, "🦀️.rs"), source).at(-1)?.target).toBe(resolve(root, "../../metrics/🦀️.rs"));
  });

  test("places inference codecs below I/O rather than the inference result collection", () => {
    const taxonomy = loadTaxonomy();
    expect(artifactFacetPathIsDeclared("🚪️io/📝️text/💡️inferences", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🚪️io/💾️binary/💡️inferences", taxonomy)).toBe(true);
    expect(artifactFacetPathIsDeclared("🧬️schema/💡️inferences/📝️text", taxonomy)).toBe(false);
    expect(artifactFacetPathIsDeclared("🧬️schema/💡️inferences/💾️binary", taxonomy)).toBe(false);
  });
});
