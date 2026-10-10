#!/usr/bin/env bun
import { configuredExactCargoLawPolicyV1 } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🪐️ `@semio-tech/space-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import Ajv from "ajv";
import { parse as parseToml } from "@iarna/toml";
import { registerPlaygroundSiteBuildCommands, runRepositoryCargoTests, runRepositoryExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

/** 🧵️ Keeps compiler worker stacks bounded while retaining the native laws' deeper runtime stack. */
function homeExactCargoEnvironment(): { env: NodeJS.ProcessEnv; nativeEnv: NodeJS.ProcessEnv } {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

/** 🧵️ Validates genuine command budgets, route records, and publication records at their shared owner. */
function retainedCommandValidators(repoRoot: string) {
  const document = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(document);
  return {
    budget: ajv.compile({ $ref: `${document.$id}#/$defs/RetainedCommandBudget` }),
    route: ajv.compile({ $ref: `${document.$id}#/$defs/RetainedCommandRoute` }),
    publication: ajv.compile({ $ref: `${document.$id}#/$defs/RetainedCommandPublicationContract` }),
  };
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-hub-space"], this.repoRoot, this.invocation.control, rest);
  }
}

/** 📇️ Proves the Home transient directory projection's wire is document-complete and corruption-explicit, and that the
 * projection is never persisted in the config. */
function homeDirectoryProjectionPersistenceOracle(repoRoot: string): number {
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
  const fixture = JSON.parse(readFileSync(join(base, "🫧️transient/🧫️fixtures/📇️projection-wire-v1/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(base, "🫧️transient/🧬️schema/🔣️.json"), "utf8"));
  const directory = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(directory);
  ajv.addSchema(module);
  const validate = ajv.compile({ $ref: `${module.$id}#/$defs/HomeDirectoryProjectionV1` });
  assert(validate(fixture.wire), JSON.stringify(validate.errors));
  const validateTransient = ajv.compile({ $ref: module.$id });
  assert(validateTransient({ sessionBindingSha256: "", authorizationGeneration: 0, receiptSha256: "", directory: fixture.wire }), JSON.stringify(validateTransient.errors));
  assert.equal(validateTransient({ sessionBindingSha256: "", authorizationGeneration: 0, receiptSha256: "", directory: { ...fixture.wire, cursor: -1 } }), false);
  const exactKeys = (value: unknown, keys: string[]): boolean => Boolean(value) && typeof value === "object" && !Array.isArray(value) && JSON.stringify(Object.keys(value as object).sort()) === JSON.stringify([...keys].sort());
  const decode = (text: string): unknown => {
    const wire = JSON.parse(text);
    assert(exactKeys(wire, ["spaces", "cursor", "users"]));
    assert(Number.isSafeInteger(wire.cursor) && wire.cursor >= 0);
    for (const space of Object.values(wire.spaces as Record<string, unknown>)) {
      assert(exactKeys(space, ["view", "members", "documents", "indexedDocuments"]));
      assert(Array.isArray((space as { documents?: unknown }).documents));
      assert(Array.isArray((space as { indexedDocuments?: unknown }).indexedDocuments));
    }
    return wire;
  };
  assert.deepEqual(decode(JSON.stringify(fixture.wire)), fixture.wire);
  assert.deepEqual(Object.values(fixture.wire.spaces).flatMap((space: any) => space.documents.map((document: any) => document.documentId)), fixture.expectedDocumentIds);
  assert.deepEqual(Object.values(fixture.wire.spaces).flatMap((space: any) => space.indexedDocuments.map((document: any) => document.descriptor.documentId)), fixture.expectedIndexedDocumentIds);
  for (const malformed of fixture.malformed) assert.throws(() => decode(malformed));
  const hostileFixture = structuredClone(fixture);
  delete hostileFixture.wire.spaces["space-α"].documents;
  assert.equal(validate(hostileFixture.wire), false);
  const missingIndexFixture = structuredClone(fixture);
  delete missingIndexFixture.wire.spaces["space-α"].indexedDocuments;
  assert.equal(validate(missingIndexFixture.wire), false);
  const source = readFileSync(join(base, "🫧️transient/🦀️.rs"), "utf8");
  const config = readFileSync(join(base, "🎚️config/🦀️.rs"), "utf8");
  const exactSource = (text: string, configText: string): boolean => text.includes("documents: Vec<store::os_directory::DocumentDescriptor>")
    && text.includes("indexed_documents: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>")
    && text.includes("documents: space.documents.clone()")
    && text.includes("indexed_documents: space.indexed_documents.clone()")
    && text.includes("documents: space.documents,")
    && text.includes("indexed_documents: space.indexed_documents }")
    && text.includes("fn from_wire(wire: HomeTransientWire) -> Result<Self, semio_framework_value::ValueError>")
    && text.includes("if !directory.resume_state_is_valid()")
    && !text.includes("unwrap_or_default")
    && !configText.includes("directory_json")
    && !configText.includes("DirectoryReadModel");
  assert(exactSource(source, config), "Home directory projection wire drops documents, defaults corruption, or is persisted in the config");
  for (const [hostile, hostileConfig] of [
    [source.replace("documents: Vec<store::os_directory::DocumentDescriptor>", "documents_removed: Vec<store::os_directory::DocumentDescriptor>"), config],
    [source.replace("indexed_documents: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>", "indexed_documents_removed: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>"), config],
    [source.replace("documents: space.documents.clone()", "documents: Vec::new()"), config],
    [source.replace("if !directory.resume_state_is_valid()", "if false"), config],
    [`${source}\nlet _ = malformed.unwrap_or_default();`, config],
    [source, `${config}\npub directory_json: String,`],
  ] as const) assert.equal(exactSource(hostile, hostileConfig), false);
  return 13;
}

class HomeDirectoryProjectionPersistenceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("home-directory-projection-persistence-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, ...homeExactCargoEnvironment(), groups: [{
          package: "semio-s-artifact-space-home",
          target: { kind: "lib" },
          cargoArgs: ["--features", "component-app-assembly"],
          laws: ["editor::home::transient::component::tests::the_projection_wire_round_trips_documents_and_rejects_corruption"],
        }], progress(event) { console.log(`home-directory-projection-persistence ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
      console.log(`home-directory-projection-persistence-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`home-directory-projection-persistence-check: checks=${homeDirectoryProjectionPersistenceOracle(this.repoRoot)} clean`);
  }
}

/** 📄️ Proves Home accepts one sealed directory page as one bounded, non-invertible TRANSIENT item through the page route
 * both surfaces share — never a config (history) edit — and answers the typed receipt the host acknowledges it by. */
function homeDirectoryEventPageOwnerOracle(repoRoot: string): number {
  const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json");
  const schemaPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
  const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
  const directoryAjv = new Ajv({ strict: false, allErrors: true, discriminator: true });
  directoryAjv.addSchema(schema);
  const validate = directoryAjv.compile({ $ref: `${schema.$id}#/$defs/DirectoryEventPageV1` });
  assert(validate(fixture.valid), JSON.stringify(validate.errors));
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
  const configSchema = JSON.parse(readFileSync(join(base, "🎚️config/🧬️schema/🔣️.json"), "utf8"));
  const validateConfig = new Ajv({ strict: false, allErrors: true }).compile(configSchema);
  const configVector = { retiredLocalStudioIds: ["studio-a", "studio-b"] };
  assert(validateConfig(configVector), JSON.stringify(validateConfig.errors));
  assert.equal(validateConfig({ ...configVector, directoryJson: JSON.stringify({ spaces: {}, cursor: 5, users: {} }) }), false, "the config schema admits a persisted directory projection");
  for (const [name, retiredLocalStudioIds] of [["duplicate", ["studio-a", "studio-a"]], ["empty-id", [""]], ["non-string", [7]], ["over-ceiling", Array.from({ length: 257 }, (_, index) => `studio-${index}`)]] as const) {
    assert.equal(validateConfig({ retiredLocalStudioIds }), false, `config schema accepted a ${name} tombstone set`);
  }
  const retainedFixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🧫️retained-command-limits/🔣️.json"), "utf8"));
  const retained = retainedCommandValidators(repoRoot);
  assert(retained.budget(retainedFixture.limits), JSON.stringify(retained.budget.errors));
  for (const route of retainedFixture.routes) assert(retained.route(route), JSON.stringify(retained.route.errors));
  const directoryRoute = retainedFixture.routes.find((route: any) => route.id === "applyDirectoryEventPage");
  assert.equal(directoryRoute?.disposition, "Migrated");
  assert.deepEqual(directoryRoute?.lanes, ["Transient"]);
  const transientSchema = JSON.parse(readFileSync(join(base, "🫧️transient/🧬️schema/🔣️.json"), "utf8"));
  const receiptFixture = JSON.parse(readFileSync(join(base, "🎮️commands/📬️apply-directory-event-page/🧬️receipt/🔣️.json"), "utf8"));
  const receiptAjv = new Ajv({ strict: true, allErrors: true });
  receiptAjv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  const validateReceipt = receiptAjv.compile(transientSchema.$defs.HomeDirectoryProjectionReceipt);
  assert(validateReceipt(receiptFixture.valid), JSON.stringify(validateReceipt.errors));
  for (const row of receiptFixture.hostile) {
    const hostile = { ...structuredClone(receiptFixture.valid), ...row.patch };
    assert.equal(validateReceipt(hostile), false, `receipt schema accepted ${row.id}`);
  }
  const leaf = JSON.parse(readFileSync(join(base, "🫧️transient/🧬️schema/🧬️mutations/📬️apply-directory/🔣️.json"), "utf8"));
  assert.equal(leaf.invertibility, "non-invertible", "a derived directory page is never undone");
  const read = (relative: string): string => readFileSync(join(base, relative), "utf8");
  const command = read("🎮️commands/📬️apply-directory-event-page/🦀️.rs");
  const transient = read("🫧️transient/🦀️.rs");
  const page = read("🫧️transient/🧬️schema/🧬️mutations/📬️apply-directory/🦀️.rs");
  const editor = read("🦀️.rs");
  const viewer = readFileSync(join(base, "../👁️viewer/🦀️.rs"), "utf8");
  const crate = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🦀️.rs"), "utf8");
  const exact = (commandSource: string, transientSource: string, pageSource: string, editorSource: string, viewerSource: string): boolean =>
    commandSource.includes("pub fn directory_page_answer")
    && commandSource.includes("DirectoryEventPageV1::parse_canonical_json(page_json)")
    && commandSource.includes("directory.admit_page(&page)?")
    && commandSource.includes("ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral }")
    && commandSource.includes("context.transient.directory()")
    && commandSource.includes("Err(Fault::new(FaultOrigin::App, \"s.home.directory-event-page.requires-retained-job\"")
    && !commandSource.includes("unwrap_or_default")
    && transientSource.includes("pub fn admit_page")
    && transientSource.includes("page.after_seq_exclusive != self.cursor")
    && transientSource.includes("let folded = store::os_directory::fold(model, event);")
    && transientSource.includes("next.cursor = page.through_seq_inclusive;")
    && transientSource.includes("Arc::unwrap_or_clone(row)")
    && transientSource.includes("pub struct DirectoryProjectionReceiptV1")
    && pageSource.includes("fn inverse(&self, _base: &HomeTransient) -> Vec<HomeTransientMutation> {\n        Vec::new()")
    && editorSource.includes("ArtifactToolPublicationContract { tool_id: \"applyDirectoryEventPage\", lanes: &[ArtifactToolPublicationLane::Transient] }")
    && editorSource.includes('.action_interactive_job("applyDirectoryEventPage", InteractiveJobClassification::Migrated)')
    && editorSource.includes("HomeDirectoryPageWork::<EditorApp<Self>>::new(tool_id, home_directory_page_json)")
    && editorSource.includes("bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>()")
    && viewerSource.includes("HomeDirectoryPageWork::<ViewerApp<Self>>::new(tool_id, home_view_page_json)")
    && viewerSource.includes("lanes: &[ArtifactToolPublicationLane::Transient]")
    && crate.includes("pub mod transient {")
    && crate.includes("pub mod apply_directory_event_page;");
  assert(exact(command, transient, page, editor, viewer), "Home directory event-page transient route is incomplete");
  for (const [hostileCommand, hostileTransient, hostilePage, hostileEditor, hostileViewer] of [
    [command.replace("ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral }", "ArtifactCommandWorkStep::Complete(emit)"), transient, page, editor, viewer],
    [`${command}\nlet _ = malformed.unwrap_or_default();`, transient, page, editor, viewer],
    [command, transient.replace("page.after_seq_exclusive != self.cursor", "false"), page, editor, viewer],
    [command, transient.replace("next.cursor = page.through_seq_inclusive;", ""), page, editor, viewer],
    [command, transient, page.replace("Vec::new()", "vec![self.clone().into()]"), editor, viewer],
    [command, transient, page, editor.replace("lanes: &[ArtifactToolPublicationLane::Transient] }", "lanes: &[ArtifactToolPublicationLane::Config] }"), viewer],
    [command, transient, page, editor, viewer.replace("lanes: &[ArtifactToolPublicationLane::Transient]", "lanes: &[ArtifactToolPublicationLane::Config]")],
  ] as const) assert.equal(exact(hostileCommand, hostileTransient, hostilePage, hostileEditor, hostileViewer), false);
  for (const leafName of ["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🔣️.json", "🛰️.proto"]) {
    assert(!readFileSync(join(base, `🎚️config/🧬️schema/${leafName}`), "utf8").includes("directory"), `config schema leaf ${leafName} still persists directory state`);
  }
  return 31;
}

class HomeDirectoryEventPageOwnerCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("home-directory-event-page-owner-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, ...homeExactCargoEnvironment(), groups: [{
          package: "semio-s-artifact-space-home",
          target: { kind: "lib" },
          cargoArgs: ["--features", "component-app-assembly"],
          laws: [
            "editor::home::commands::apply_directory_event_page::tests::sealed_page_replaces_projection_once_and_rejects_races",
            "editor::home::transient::component::tests::a_ten_thousand_space_directory_bootstraps_page_by_page_under_the_one_item_bound",
            "editor::home::transient::component::tests::a_dropped_transient_rebootstraps_from_the_origin_without_history",
            "editor::home::component::tests::a_dispatched_page_publishes_one_transient_item_and_no_history_row",
          ],
        }], progress(event) { console.log(`home-directory-event-page-owner ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
      console.log(`home-directory-event-page-owner-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`home-directory-event-page-owner-check: checks=${homeDirectoryEventPageOwnerOracle(this.repoRoot)} clean`);
  }
}

/** 🪪️ Proves only the current Hub author identity receives Home administration affordances. */
function homeDirectoryIdentityRowsOracle(repoRoot: string): number {
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any");
  const readSources = (paths: string[]): string => paths.map((path) => readFileSync(join(repoRoot, path), "utf8")).join("\n");
  const controller = readFileSync(join(base, "✏️editor/🦀️.rs"), "utf8");
  const editor = readFileSync(join(base, "✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const editorTests = readFileSync(join(base, "✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const viewer = readFileSync(join(base, "👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const viewerTests = readFileSync(join(base, "👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const homeViewerApp = readSources([
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const homeOperations = readFileSync(join(base, "🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const homeBinary = readFileSync(join(base, "🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const spaceOperations = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const spaceEditor = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const spaceViewer = readSources([
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const spaceViewerApp = readSources([
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const spaceMembers = readSources([
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/👥️members/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/👥️members/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const spaceIndexController = readSources([
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const spaceEngine = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs"), "utf8");
  const homeCrateRoot = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home");
  const homeCrate = readFileSync(join(homeCrateRoot, "🦀️.rs"), "utf8");
  const spaceIndexCrateRoot = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space");
  const spaceIndexCrate = readFileSync(join(spaceIndexCrateRoot, "🦀️.rs"), "utf8");
  const spaceShared = readSources([
    "✏️s/🔌️plugins/🪐️space/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/🧪️tests/🔬️surface/🦀️.rs",
  ]);
  const spaceConfig = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const exportMedia = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/📤️export-media/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const setAppRegistrations = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/📇️set-app-registrations/🦀️.rs",
  ]);
  const createStudio = readFileSync(join(base, "✏️editor/🎮️commands/🏗️create-studio/🦀️.rs"), "utf8");
  const cataloguePanel = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🛍️catalogue/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const inspectionPanel = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const parametersPanel = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔢️parameters/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔢️parameters/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const nodeGraphEdit = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const setActiveExample = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const setActivePanelTab = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/⚙️set-active-panel-tab/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/⚙️set-active-panel-tab/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const workflowWindow = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const compiledDagWindow = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const spawnApp = readSources([
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🚀️spawn-app/🦀️.rs",
    "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🚀️spawn-app/🧪️tests/🔬️unit/🦀️.rs",
  ]);
  const ownerScript = readFileSync(import.meta.filename, "utf8");
  const osHost = readSources(["🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs", "🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️workflow-unit/🦀️.rs"]);
  const exact = (editorSource: string, editorTestSource: string, viewerTestSource: string): boolean => editorSource.includes("row.role == Some(crate::DirectorySpaceRole::Author)")
    && editorSource.includes('home_row_action(IconName::Users, labels.action_manage, "manageSpace")?')
    && editorTestSource.includes('assert_eq!(buttons.len(), 5')
    && editorTestSource.includes('text_arg(manage_button, "spaceId")')
    && editorTestSource.includes("spectator_and_unbound_hub_rows_only_carry_open")
    && editorTestSource.includes("role: Some(crate::DirectorySpaceRole::Spectator)")
    && editorTestSource.includes("role: None")
    && viewerTestSource.includes('origin: "hub", data_class: "persistedShared", role: None');
  assert(/use crate::editor::home::commands::\{[^}]*\bmanage_space\b[^}]*\};/.test(controller) && controller.includes('"manageSpace" as "manage-space" => manage_space::ManageSpace'), "Home controller does not import and dispatch the manageSpace command module");
  assert(exact(editor, editorTests, viewerTests), "Home identity rows expose administration without current author authority");
  const catalogGenerationFixture = "🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/🧪️bumps/🦀️.rs";
  const catalogGenerationSource = readFileSync(join(base, catalogGenerationFixture), "utf8");
  assert(existsSync(join(base, catalogGenerationFixture)), "Home catalog-generation fixture is not present at its canonical bounded physical path");
  assert(homeCrate.includes(`🏅️standards/🔖️1/🪆️subsets/✳️any/${catalogGenerationFixture}`), "Home artifact crate misses the canonical bounded fixture path");
  const spaceBase = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any");
  const createArtifactFixture = "🧬️schema/🧬️mutations/🌱create-artifact/🧪️tests/🧪️appends/🦀️.rs";
  assert(existsSync(join(spaceBase, createArtifactFixture)), "Space create-artifact fixture is not present at its canonical bounded physical path");
  assert(spaceIndexCrate.includes(`🏅️standards/🔖️1/🪆️subsets/✳️any/${createArtifactFixture}`), "Space Index artifact crate misses the canonical create-artifact fixture path");
  const missingMounts = [
    ...[...homeCrate.matchAll(/#\[path = "([^"]+)"\]/g)].map((match) => [homeCrateRoot, match[1]] as const),
    ...[...spaceIndexCrate.matchAll(/#\[path = "([^"]+)"\]/g)].map((match) => [spaceIndexCrateRoot, match[1]] as const),
  ].filter(([root, mount]) => !existsSync(join(root, mount)));
  assert.deepEqual(missingMounts, [], `Space artifact crates mount missing physical paths: ${missingMounts.map(([, mount]) => mount).join(", ")}`);
  assert(homeOperations.includes("use protocol::os_spr::protocol_laws::{") && spaceOperations.includes("use protocol::os_spr::protocol_laws::{"), "Home or Space mutation laws import the Pack test context instead of the current SPR test context");
  const operationSources = `${homeOperations}\n${spaceOperations}`;
  const sprLawCalls = operationSources.split("\n").filter((line) => /\bassert_(?:fatal_never_applies|missing_target_is_error|mutation_diff_absorb_law|mutation_inverse_law|outcome_policy_matrix)\(/.test(line));
  assert(sprLawCalls.length === 14 && sprLawCalls.every((call) => call.includes(".await;")), "Home or Space mutation laws do not await the current async SPR test context");
  assert(homeBinary.includes("ArtifactStore::new(envelope).await") && homeBinary.includes(".dispatch(store::ArtifactCommand::Apply") && homeBinary.includes(" }).await.expect"), "Home document codec law does not await the current Store construction and dispatch boundary");
  assert(catalogGenerationSource.includes("dsl::from_dsl_value(pack::json_to_dsl_value(&json))") && !catalogGenerationSource.includes("serde_json::from_str(BEFORE)"), "Home snapshot fixture bypasses the first-party value codec");
  assert(!spaceEngine.includes("Some(&json!(") && !spaceEngine.includes("Some(&pack::json!(") && spaceEngine.match(/pack::json_to_dsl_value\(&pack::json!\(/g)?.length === 3, "Space checkpoint tests do not convert first-party JSON into the current DSL action boundary");
  const pluginImports = (source: string): string[] => source.match(/use semio_framework_plugin::\{([^}]*)\};/)?.[1]!.split(",").map((name) => name.trim()) ?? [];
  assert(["LocalizedLabel", "WindowKindDefinition"].every((name) => pluginImports(editor).includes(name)) && !editor.includes("let UiNode::") && ["IconName", "WindowKindDefinition"].every((name) => pluginImports(spaceEditor).includes(name)) && !spaceEditor.includes("let UiNode::"), "Home or Space row tests do not use the current fixed BuiltNode projection");
  assert(!spaceViewer.includes("let UiNode::") && spaceViewer.includes("BuiltTreeRetirement::new"), "Space viewer rows bypass the current fixed BuiltNode projection and retirement boundary");
  assert(homeViewerApp.includes("create_home_viewer().await") && homeViewerApp.includes("project_and_retire_fixture_tree(tree)"), "Home viewer fixtures do not await and retire the current manifest/render boundaries");
  assert(spaceViewerApp.includes("let def = create_space_index_viewer();") && !spaceViewerApp.includes("create_space_index_viewer().await") && spaceViewerApp.includes("project_and_retire_fixture_tree(tree)"), "Space viewer fixtures do not use the synchronous manifest and current retained render boundary");
  assert(editor.includes("fn render_rows_wrapped(") && editorTests.includes("render_rows_wrapped(rows, &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN, &TreeWindows::unhosted())"), "Home production and injected-row composition do not share the current fallible node builder");
  assert(!editorTests.includes("async fn one_local_row()") && !editorTests.includes("async fn one_hub_row()"), "Pure Home row fixtures are needlessly async");
  assert(controller.includes("pub async fn create_home_app()"), "Home editor does not expose its async app-definition constructor");
  assert(spaceEngine.includes("pub(crate) async fn app_with_registry()") && !spaceEngine.includes("new_app::<SpaceApp>()"), "Space test context bypasses its registered async app constructor");
  assert(spaceEngine.includes("new_registered_app::<SpaceApp, _>(create_space_app()).await"), "Space test context does not await its async manifest through the registered constructor");
  assert(spaceEngine.includes("SpaceApp::initial_snapshot().await.graph.nodes.is_empty()"), "Space snapshot fixture dereferences the current async initial snapshot before awaiting it");
  assert(!spaceEngine.includes("VcsArtifactApp::<SpaceApp>::new(SpaceApp::default())") && !spaceEngine.includes("pack::to_json_string(&SpaceApp::render(") && spaceEngine.match(/plugin_laws::project_and_retire_fixture_tree\(/g)?.length === 2, "Space fixtures bypass the registered app constructor or do not await and retire rendered component trees");
  assert(spaceEngine.includes("space_workflow_context_menu_items(&registry, labels, false, None, &selected_node_ids).await"), "Space context-menu fixture dereferences the current async projection before awaiting it");
  assert(spaceEngine.includes("pack::json_from_dsl_value(&dsl::ToValue::to_value(&base))") && spaceEngine.includes("let post_oracle: serde_json::Value = serde_json::from_str"), "Space config law bypasses the first-party value codec or independent JSON oracle");
  assert(!spaceEngine.includes("let projection = demo_space_projection();") && !spaceEngine.includes("let app = create_space_app();") && !spaceEngine.includes("let studio = create_space_app();"), "Space engine retains an unawaited async fixture");
  assert(!spaceEngine.includes("VcsArtifactApp::new(SpaceApp::default());") && !spaceEngine.includes('context::test_surface_id("draw"),'), "Space engine retains an unawaited app or surface fixture");
  assert(spaceIndexController.includes("pub async fn new_app() -> SpaceIndexApp") && spaceIndexController.includes("new_app_with_registry::<EditorApp<SpaceIndexEditor>>(space_index_manifest_for_tests).await"), "Space index test context does not await its registered async app construction");
  assert(!/ActionRef::new\([^\n]+\)\?/.test(spaceIndexController), "Space index dialog fixtures retain fallibility from the current infallible action reference constructor");
  assert(!spaceIndexController.includes("pack::to_json_string(&<SpaceIndexEditor") && spaceIndexController.includes("project_and_retire_fixture_tree"), "Space index fixtures do not admit and retire the current fallible component tree");
  assert(spaceMembers.includes("wire_and_retire(render(") && !spaceMembers.includes("pack::to_json_string(&node)"), "Space members fixtures bypass the current fallible BuiltNode and retirement boundary");
  assert(spaceShared.includes("Some(document_backbone_ref(backbone_uri).await)"), "Space document synchronization does not await its typed backbone reference");
  assert(spaceConfig.match(/round_trip\(&config, &operation\)\.await/g)?.length === 2 && exportMedia.includes("register_format_descriptors([") && /\.await\s*\.expect\(\"register neutral format descriptor\"\)/.test(exportMedia), "Space config or format-registration fixtures do not await their current async boundaries");
  assert(cataloguePanel.includes(').await.expect("catalogue tree")') && setActivePanelTab.includes(').await.expect("catalogue tree")') && setActiveExample.includes("register_studio_port_for_test(&entry.id, port).await") && cataloguePanel.includes("project_and_retire_fixture_tree") && setActivePanelTab.includes("project_and_retire_fixture_tree"), "Space catalogue or studio-port fixtures bypass current async ownership and component retirement");
  assert(nodeGraphEdit.includes("serde_json::Value::as_object_mut") && nodeGraphEdit.includes("serde_json::Value::Object(position)") && !nodeGraphEdit.includes("fixture.get_mut(\"layout\").and_then(pack::JsonValue::as_object_mut)"), "Space node-graph fixture crosses serde JSON through the first-party Pack value family");
  assert(setActiveExample.includes("OsBackbonePorts::Store(store::BackbonePorts::Memory") && setActiveExample.match(/empty_workflow_snapshot\(\)\.await/g)?.length === 5 && setActivePanelTab.includes("let projection = empty_workflow_snapshot().await;"), "Space fixtures retain a stale backbone enum or unresolved workflow snapshot future");
  assert(setActiveExample.match(/load_document_snapshot\(&emit\)\.await/g)?.length === 2 && workflowWindow.match(/\.render\(S_PLAY_BODY_WORKFLOW,[^;]+\.await\.expect\("render"\)/g)?.length === 2 && compiledDagWindow.includes('.render(S_PLAY_BODY_COMPILED_DAG, None, &ViewModel::default()).await.expect("render")') && workflowWindow.match(/project_and_retire_fixture_tree\(node\)/g)?.length === 2 && compiledDagWindow.includes("project_and_retire_fixture_tree(node)"), "Space window fixtures retain unresolved async renders or unretired component owners");
  const spaceAppFixtures = `${workflowWindow}\n${compiledDagWindow}\n${spawnApp}`;
  assert(!spaceAppFixtures.includes("VcsArtifactApp::new(crate::engine::space::SpaceApp::default())") && !spaceAppFixtures.includes("VcsArtifactApp::<crate::engine::space::SpaceApp>::new") && spaceAppFixtures.match(/crate::engine::space::unit_tests::context::app_with_registry\(\)\.await/g)?.length === 4, "Space window and spawn fixtures bypass the registered app constructor");
  const fixedPanelFixtures = `${inspectionPanel}\n${parametersPanel}`;
  assert(!fixedPanelFixtures.includes("pack::to_json_string(&node)") && fixedPanelFixtures.match(/project_and_retire_fixture_tree\(semio_framework_plugin::ComponentTree \{ root: node \}\)/g)?.length === 3, "Space panel fixtures serialize retained BuiltNode owners instead of projecting and retiring them");
  assert(spaceShared.match(/assert_(?:viewer_never_mutates|editor_and_viewer_share_dialect)::<[^;]+>\(\)\.await;/g)?.length === 3 && homeViewerApp.match(/assert_(?:viewer_never_mutates|editor_and_viewer_share_dialect)::<[^;]+>\(\)\.await;/g)?.length === 1, "Home or Space surface tests leave the async test context future unpolled");
  assert(createStudio.includes("resolve_ready(crate::register_studio_port(&entry.id, port))") && exportMedia.includes("payload.document_json.parse::<Value>()") && setAppRegistrations.includes("resolve_ready(crate::engine::space::engine::apply_app_registrations(&payload.json))"), "Synchronous Space command handlers leave an async registry side effect unpolled");
  for (const law of [
    "editor::home::modes::explore::windows::main::component::tests::a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions",
    "editor::home::modes::explore::windows::main::component::tests::spectator_and_unbound_hub_rows_only_carry_open",
    "viewer::home::modes::view::windows::main::component::tests::a_row_stamps_the_space_row_id",
  ]) assert(ownerScript.includes(law), `Home native gate omitted the current exact selector ${law}`);
  assert(ownerScript.includes('RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432"'), "Home exact builds do not bound compiler worker stacks");
  assert(ownerScript.includes('nativeEnv: { RUST_MIN_STACK: "268435456" }'), "Home exact laws lost their native runtime stack");
  for (const [editorSource, editorTestSource] of [
    [editor.replace("row.role == Some(crate::DirectorySpaceRole::Author)", 'row.origin == "hub"'), editorTests],
    [editor, editorTests.replace('assert_eq!(buttons.len(), 5', 'assert_eq!(buttons.len(), 4')],
    [editor, editorTests.replace("role: Some(crate::DirectorySpaceRole::Spectator)", "role: Some(crate::DirectorySpaceRole::Author)")],
  ]) assert.equal(exact(editorSource, editorTestSource, viewerTests), false);
  for (const law of ["workflow::tests::svg_path_extraction_preserves_transformed_geometry"]) {
    assert(ownerScript.includes(law), `Home native gate omitted ${law}`);
    const name = law.slice(law.lastIndexOf("::") + 2);
    assert(osHost.includes(`fn ${name}()`), `OS host omitted ${law}`);
  }
  assert(ownerScript.includes('cargoArgs: ["--features", "os-host-full"]'), "Home native gate cannot select its feature-owned workflow law");
  return 54;
}

class HomeDirectoryIdentityRowsCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("home-directory-identity-rows-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, ...homeExactCargoEnvironment(), groups: [
          {
            package: "semio-framework-plugin-host",
            target: { kind: "lib" },
            laws: ["component::imports::effect_conversion_tests::request_inference_proposal_preserves_the_closed_kind"],
          },
          {
            package: "semio-framework-os",
            target: { kind: "lib" },
            cargoArgs: ["--features", "os-host-full"],
            laws: [
              "workflow::tests::svg_path_extraction_preserves_transformed_geometry",
            ],
          },
          {
            package: "semio-s-artifact-space-home",
            target: { kind: "lib" },
            cargoArgs: ["--features", "component-app-assembly"],
            laws: [
              "editor::home::modes::explore::windows::main::component::tests::a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions",
              "editor::home::modes::explore::windows::main::component::tests::spectator_and_unbound_hub_rows_only_carry_open",
              "viewer::home::modes::view::windows::main::component::tests::a_row_stamps_the_space_row_id",
            ],
          },
        ], progress(event) { console.log(`home-directory-identity-rows ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
      console.log(`home-directory-identity-rows-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`home-directory-identity-rows-check: checks=${homeDirectoryIdentityRowsOracle(this.repoRoot)} clean`);
  }
}

/** 🪪️ Joins the schema-validated component identity fixture to actual owner source and independent Cargo parsers. */
function spacePluginIdentityOracle(repoRoot: string): number {
  const plugin = join(repoRoot, "🌎️hub/🧩️compositions/🪐️space");
  const fixture = JSON.parse(readFileSync(join(plugin, "🧫️fixtures/🧫️plugin-identity/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(plugin, "🧬️schema/🪪️plugin-identity/🔣️.json"), "utf8"));
  const identityAjv = new Ajv({ strict: true, allErrors: true });
  identityAjv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  identityAjv.addSchema(module);
  assert(identityAjv.compile({ $ref: `${module.$id}#/$defs/SpacePluginIdentity` })(fixture), "plugin-identity fixture violates its owner scope export");

  const cargo = readFileSync(join(plugin, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
  const cargoComponentPackage = cargo.split("[package.metadata.component]")[1]?.split("[")[0]?.match(/package\s*=\s*"([^"]+)"/)?.[1];
  assert.equal(cargoComponentPackage, fixture.packageId, "Cargo component package is not the fixture identity");
  assert.equal(cargo.match(/^name\s*=\s*"([^"]+)"/mu)?.[1], fixture.packageName, "Cargo package name is not the fixture identity");
  assert.equal(cargo.split("[[package.metadata.semio.playground]]")[1]?.match(/variant\s*=\s*"([^"]+)"/)?.[1], fixture.playgroundVariant, "the playground variant row is the OTHER name and must stay declared");

  const pluginRoot = readFileSync(join(plugin, "🦀️.rs"), "utf8");
  assert.equal(pluginRoot.match(/Plugin::<SpaceApps>::builder\("([^"]+)"\)/)?.[1], fixture.pluginId, "plugin() builder id is not the fixture identity");
  assert.equal(pluginRoot.match(/\.package_id\("([^"]+)"\)/)?.[1], fixture.packageId, "plugin().package_id is not the fixture identity");
  assert.equal(fixture.packageId, `semio:${fixture.pluginId}`, "component package identity must be semio:<plugin id>");
  assert.equal(fixture.artifactKindPrefix, `s.${fixture.pluginId}.`, "the canonical s.<plugin>.<kind> owner segment must be the plugin id");

  const declared = Bun.TOML.parse(cargo) as {package:{metadata:{semio:{"deployment-directory":string;"component-kind":string;host:{landing:string;shell:string}}}}};
  assert.deepEqual(declared, parseToml(cargo), "independent TOML oracle differs from first-party runtime parser");
  const metadata = declared.package.metadata.semio;
  assert.equal(metadata["deployment-directory"], fixture.moduleDirectoryName, "authored Cargo directory differs from fixture identity");
  assert.equal(metadata["component-kind"], "plugin", "authored component kind differs from fixture identity");
  assert.deepEqual({ landingAppId: metadata.host.landing, hostAppId: metadata.host.shell }, fixture.host, "authored owner host differs from fixture identity");
  return 11;
}

/** 🧵️ Proves the three `🪐️space` app surfaces declare exactly the interactive-job dispositions their
 * language-neutral fixtures declare, and reports the committed descriptor's drift against them. The
 * descriptor is regenerated only by a full `wasm32-wasip2` build (`describe`), so a stale one is
 * expected — what is NOT tolerated is a descriptor that carries `interactiveJob` and disagrees. */
function interactiveJobCatalogOracle(repoRoot: string): number {
  const plugin = join(repoRoot, "🌎️hub/🧩️compositions/🪐️space");
  const artifactPlugin = join(repoRoot, "✏️s/🔌️plugins/🪐️space");
  const surfaces = [
    { appId: "s.space.studio@1/*#editor", owner: join(plugin, "⚙️engine/🪐️space"), source: join(plugin, "⚙️engine/🪐️space/🦀️.rs"), shape: "status" as const, factory: "SpaceCommandJobFactory" },
    { appId: "s.space.home@1/*#editor", owner: join(artifactPlugin, "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"), scope: join(artifactPlugin, "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any"), export: "HomeRetainedCommandLimits", source: join(artifactPlugin, "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), shape: "disposition" as const, factory: "HomeRetainedCommandJobFactory" },
    { appId: "s.space.space@1/*#editor", owner: join(artifactPlugin, "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"), scope: join(artifactPlugin, "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any"), export: "SpaceIndexRetainedCommandLimits", source: join(artifactPlugin, "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), shape: "status" as const, factory: "SpaceIndexRetainedCommandJobFactory" },
  ];
  const descriptor = JSON.parse(readFileSync(join(plugin, "🔣️.json"), "utf8"));
  let checks = spacePluginIdentityOracle(repoRoot);
  let staleRows = 0;
  for (const surface of surfaces) {
    const fixtureRoot = join(surface.owner, "🧫️fixtures/🧫️retained-command-limits");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const retained = retainedCommandValidators(repoRoot);
    assert(retained.budget(fixture.limits), JSON.stringify(retained.budget.errors));
    for (const route of fixture.routes) assert(retained.route(route), JSON.stringify(retained.route.errors));
    for (const publication of fixture.publicationContracts ?? []) assert(retained.publication(publication), JSON.stringify(retained.publication.errors));
    checks += 1;
    const migrated: string[] = fixture.routes.filter((route: any) => (surface.shape === "status" ? route.status === "Migrated" : route.disposition === "Migrated")).map((route: any) => route.id);
    const lanes = new Map<string, string[]>(
      surface.shape === "status" ? fixture.publicationContracts.map((entry: any) => [entry.toolId, entry.lanes]) : fixture.routes.map((route: any) => [route.id, route.lanes]),
    );
    assert.deepEqual([...migrated].sort(), [...new Set(migrated)].sort(), `${surface.appId} fixture repeats a migrated id`);
    for (const id of migrated) assert((lanes.get(id) ?? []).length > 0, `${surface.appId}:${id} is migrated with no publication lane`);
    checks += 1;
    const source = readFileSync(surface.source, "utf8");
    assert(source.includes(`factory_type: ${surface.factory},`), `${surface.appId} proof catalog declares no owned factory type`);
    assert(source.includes(`controller: "${surface.appId}",`), `${surface.appId} proof catalog controller is not its runtime surface id`);
    for (const id of migrated) {
      assert(source.includes(`"${id}"`), `${surface.appId} source lost the migrated id ${id}`);
      assert.equal(source.includes(`.action_interactive_job("${id}", InteractiveJobClassification::BatchOnlyPendingRewrite)`), false, `${surface.appId}:${id} is still declared batch-only in source`);
    }
    checks += 1;
    const app = descriptor.manifest.apps.find((entry: any) => entry.id === surface.appId);
    assert(app, `descriptor omits ${surface.appId}`);
    const declared = new Map<string, string | undefined>();
    for (const window of app.windowKinds ?? []) for (const action of window.actions ?? []) declared.set(action.id, action.semantics?.execution?.interactiveJob);
    for (const command of app.commands ?? []) declared.set(command.id, command.semantics?.execution?.interactiveJob);
    for (const id of migrated) {
      const published = declared.get(id);
      if (published === undefined) { staleRows += 1; continue; }
      assert.equal(published, "migrated", `descriptor publishes ${surface.appId}:${id} as ${published}, source says migrated`);
    }
    checks += 1;
  }
  console.log(`interactive-job-catalog: descriptor rows without an interactiveJob disposition: ${staleRows} (regenerated by \`describe\` after a wasm build)`);
  return checks;
}

/** 🪪️ Registered gate for {@link spacePluginIdentityOracle}. */
class PluginIdentityCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length > 0) throw new Error("plugin-identity-check accepts no arguments");
    console.log(`plugin-identity-check: checks=${spacePluginIdentityOracle(this.repoRoot)} clean`);
  }
}

/** 🪶️ Reads the actual two-owner composition census and runs its registered owning Native law. */
class SnapshotOwnerCensusScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if(segments.length!==1 || !["source","native"].includes(segments[0]!)) throw Error("snapshot-owner-census requires source or native");
    const fixtureRoot=join(this.repoRoot,"🌎️hub/🧩️compositions/🪐️space/🧫️fixtures/🪶️snapshot-owner-census");
    const fixture=JSON.parse(readFileSync(join(fixtureRoot,"🔣️.json"),"utf8")) as {package:string,owners:{package:string,kind:string,sqlPath:string,tables:string[]}[]};
    const manifest=parseToml(readFileSync(join(this.root,"Cargo.toml"),"utf8")) as {package:{name:string},dependencies:Record<string,unknown>};
    assert.equal(manifest.package.name,fixture.package);
    const {Database}=await import("bun:sqlite");
    for(const owner of fixture.owners){
      assert(Object.hasOwn(manifest.dependencies,owner.package));
      const db=new Database(":memory:");
      try{db.exec(readFileSync(join(this.repoRoot,owner.sqlPath),"utf8"));assert.deepEqual(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all(),owner.tables.map(name=>({name})));assert.deepEqual(db.query("PRAGMA integrity_check").all(),[{integrity_check:"ok"}]);assert.deepEqual(db.query("PRAGMA foreign_key_check").all(),[]);}finally{db.close();}
      console.log("[DEBUG] independent Hub Space census owner "+owner.kind+" package="+owner.package+" authored_SQL_tables="+owner.tables.length);
    }
    if(segments[0]==="native"){
      const receipts=await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd:this.root, env:{...process.env,RUST_MIN_STACK:"268435456"}, groups:[{package:"semio-hub-space",target:{kind:"lib"},laws:["interactive_job_catalog_tests::sqlite_snapshot_composed_owner_census"]}], progress(event){console.log("[DEBUG] Hub Space census Native "+event.stage+" "+(event.law??""));} });
      console.log("[DEBUG] Hub Space census Native receipts "+JSON.stringify(receipts));
    }
  }
}

class InteractiveJobCatalogCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("interactive-job-catalog-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.root, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [{
          package: "semio-hub-space",
          target: { kind: "lib" },
          laws: [
            "interactive_job_catalog_tests::studio_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory",
            "interactive_job_catalog_tests::home_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory",
            "interactive_job_catalog_tests::space_index_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory",
            "interactive_job_catalog_tests::tool_proof_catalogs_match_the_runtime_identity_they_are_joined_against",
            "interactive_job_catalog_tests::manifest_plugin_id_matches_the_cargo_component_package",
            "interactive_job_catalog_tests::plugin_assembly_succeeds_and_registers_all_five_surfaces",
            "interactive_job_catalog_tests::every_app_instance_constructs_against_its_registered_proof_catalog",
            "interactive_job_catalog_tests::sqlite_snapshot_composed_owner_census",
          ],
        }], progress(event) { console.log(`interactive-job-catalog ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
      console.log(`interactive-job-catalog-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`interactive-job-catalog-check: checks=${interactiveJobCatalogOracle(this.repoRoot)} clean`);
  }
}


/** 🗃️ Proves PersistenceDataClass routing (WP-C6). */
function persistenceDataClassOracle(repoRoot: string): number {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧫️fixtures/persistence-data-class-v1/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧬️schema/persistence-data-class/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(schema);
  const validateBinding = ajv.compile({ $ref: `${schema.$id}#/$defs/ClassifiedPersistenceBinding` });
  const validateLane = ajv.compile({ $ref: `${schema.$id}#/$defs/ClassifiedWireLane` });
  for (const c of fixture.cases) {
    assert(validateBinding(c.binding), JSON.stringify(validateBinding.errors));
    if (c.wireLane) assert(validateLane(c.wireLane), JSON.stringify(validateLane.errors));
  }
  const core = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs"), "utf8");
  assert(core.includes('origin: "hub"'));
  assert(core.includes('data_class: "persistedShared"'));
  assert(core.includes('ephemeralLocalOnly'));
  const main = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  assert(main.includes('ephemeralLocalOnly'));
  assert(main.includes("promoteToHubSpace"));
  assert(main.includes("persistLocally"));
  const share = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️share-space/🦀️.rs"), "utf8");
  assert(share.includes("ephemeralShareBlocked"));
  const sync = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"), "utf8");
  assert(sync.includes("enum PersistenceDataClass"));
  assert(sync.includes("wire_lane_data_class"));
  const osTs = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  assert(osTs.includes("export type PersistenceDataClass"));
  assert(osTs.includes("dataClass: \"persistedShared\""));
  assert(fixture.homeUnion.hubSpace.origin === "hub");
  assert(fixture.homeUnion.ephemeralStudio.dataClass === "ephemeralLocalOnly");
  return fixture.cases.length + 6;
}

class PersistenceDataClassCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 0) throw new Error("persistence-data-class-check takes no args");
    console.log(`persistence-data-class-check: checks=${persistenceDataClassOracle(this.repoRoot)} clean`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("home-directory-projection-persistence-check", HomeDirectoryProjectionPersistenceCheckScript).register("home-directory-event-page-owner-check", HomeDirectoryEventPageOwnerCheckScript).register("home-directory-identity-rows-check", HomeDirectoryIdentityRowsCheckScript).register("interactive-job-catalog-check", InteractiveJobCatalogCheckScript).register("snapshot-owner-census", SnapshotOwnerCensusScript).register("plugin-identity-check", PluginIdentityCheckScript).register("persistence-data-class-check", PersistenceDataClassCheckScript);

registerPlaygroundSiteBuildCommands(router);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
