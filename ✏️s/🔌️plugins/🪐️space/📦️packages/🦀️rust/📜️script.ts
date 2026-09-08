#!/usr/bin/env bun
/** 🪐️ `@semio-tech/space-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

/** 🧵️ Keeps compiler worker stacks bounded while retaining the native laws' deeper runtime stack. */
function homeExactCargoEnvironment(): { env: NodeJS.ProcessEnv; nativeEnv: NodeJS.ProcessEnv } {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

/** 🧫️ Compiles one scope-owned retained-command export against the shared `framework.ui` shape. */
function compileRetainedCommandLimits(repoRoot: string, scopeRoot: string, exportId: string) {
  const ui = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(scopeRoot, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(ui);
  ajv.addSchema(module);
  return ajv.compile({ $ref: `${module.$id}#/$defs/${exportId}` });
}

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-space"], this.repoRoot);
  }
}

/** 📇️ Proves Home projection persistence is document-complete and corruption-explicit. */
export function homeDirectoryProjectionPersistenceOracle(repoRoot: string): number {
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/📇️projection-persistence-v1/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema(module);
  const validate = ajv.compile({ $ref: `${module.$id}#/$defs/HomeProjectionPersistence` });
  assert(validate(fixture), JSON.stringify(validate.errors));
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
  const canonical = JSON.stringify(fixture.wire);
  assert.deepEqual(decode(canonical), fixture.wire);
  assert.deepEqual(Object.values(fixture.wire.spaces).flatMap((space: any) => space.documents.map((document: any) => document.documentId)), fixture.expectedDocumentIds);
  assert.deepEqual(Object.values(fixture.wire.spaces).flatMap((space: any) => space.indexedDocuments.map((document: any) => document.descriptor.documentId)), fixture.expectedIndexedDocumentIds);
  for (const malformed of fixture.malformed) assert.throws(() => decode(malformed));
  const hostileFixture = structuredClone(fixture);
  delete hostileFixture.wire.spaces["space-α"].documents;
  assert.equal(validate(hostileFixture), false);
  const missingIndexFixture = structuredClone(fixture);
  delete missingIndexFixture.wire.spaces["space-α"].indexedDocuments;
  assert.equal(validate(missingIndexFixture), false);
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const exactSource = (text: string): boolean => text.includes("documents: Vec<store::os_directory::DocumentDescriptor>")
    && text.includes("indexed_documents: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>")
    && text.includes("documents: space.documents.clone()")
    && text.includes("indexed_documents: space.indexed_documents.clone()")
    && text.includes("documents: space.documents")
    && text.includes("indexed_documents: space.indexed_documents")
    && text.includes("fn directory_from_json(json: &str) -> Result<store::os_directory::DirectoryReadModel, Fault>")
    && !text.includes("pack::from_json_str(json).unwrap_or_default()")
    && text.includes("pub fn directory(&self) -> Result<store::os_directory::DirectoryReadModel, Fault>");
  assert(exactSource(source), "Home projection persistence still drops documents or defaults corruption");
  for (const hostile of [
    source.replace("documents: Vec<store::os_directory::DocumentDescriptor>", "documents_removed: Vec<store::os_directory::DocumentDescriptor>"),
    source.replace("indexed_documents: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>", "indexed_documents_removed: Vec<store::os_directory::DirectoryIndexedDocumentViewV1>"),
    source.replace("documents: space.documents.clone()", "documents: Vec::new()"),
    source.replace("documents: space.documents", "documents: Vec::new()"),
    source.replace("-> Result<store::os_directory::DirectoryReadModel, Fault>", "-> store::os_directory::DirectoryReadModel"),
  ]) assert.equal(exactSource(hostile), false);
  return 11;
}

class HomeDirectoryProjectionPersistenceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("home-directory-projection-persistence-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        ...homeExactCargoEnvironment(),
        groups: [{
          package: "semio-s-plugin-space",
          target: { kind: "lib" },
          laws: ["editor::home::config::tests::directory_projection_round_trip_preserves_documents_and_rejects_corruption"],
        }],
        progress(event) { console.log(`home-directory-projection-persistence ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`home-directory-projection-persistence-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`home-directory-projection-persistence-check: checks=${homeDirectoryProjectionPersistenceOracle(this.repoRoot)} clean`);
  }
}

/** 📄️ Proves Home accepts one sealed directory page through one retained config replacement. */
export function homeDirectoryEventPageOwnerOracle(repoRoot: string): number {
  const fixturePath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json");
  const schemaPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json");
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
  const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
  const validate = new Ajv2020({ strict: false, allErrors: true, discriminator: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  assert(validate(fixture.valid), JSON.stringify(validate.errors));
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
  const configSchema = JSON.parse(readFileSync(join(base, "🎚️config/🧬️schema/🔣️.json"), "utf8"));
  const validateConfig = new Ajv({ strict: false, allErrors: true }).compile(configSchema);
  const configVector = {
    activePanelTab: "", locale: "de-DE", directoryJson: JSON.stringify({ spaces: {}, cursor: 5, users: {} }),
    directorySessionBindingSha256: "a".repeat(64), directoryAuthorizationGeneration: 7,
    directoryReceiptSha256: fixture.expectedReceiptSha256, clientId: "u-1", clientName: "Ada",
  };
  assert(validateConfig(configVector), JSON.stringify(validateConfig.errors));
  for (const field of ["directoryJson", "directorySessionBindingSha256", "directoryAuthorizationGeneration", "directoryReceiptSha256"]) {
    const hostile = structuredClone(configVector) as Record<string, unknown>;
    delete hostile[field];
    assert.equal(validateConfig(hostile), false, `config schema accepted missing ${field}`);
  }
  const retainedFixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/🧫️retained-command-limits/🔣️.json"), "utf8"));
  const validateRetained = compileRetainedCommandLimits(repoRoot, join(base, ".."), "HomeRetainedCommandLimits");
  assert(validateRetained(retainedFixture), JSON.stringify(validateRetained.errors));
  assert.equal(retainedFixture.routes.find((route: any) => route.id === "applyDirectoryEventPage")?.lanes?.[0], "Config");
  const commandPath = join(base, "🎮️commands/📬️apply-directory-event-page/🦀️.rs");
  const command = existsSync(commandPath) ? readFileSync(commandPath, "utf8") : "";
  const receiptRoot = join(base, "🎮️commands/📬️apply-directory-event-page/🧬️receipt");
  const receiptFixture = JSON.parse(readFileSync(join(receiptRoot, "🔣️.json"), "utf8"));
  const receiptSchema = JSON.parse(readFileSync(join(receiptRoot, "🧬️.schema.json"), "utf8"));
  const validateReceipt = new Ajv2020({ strict: true, allErrors: true }).compile(receiptSchema);
  assert(validateReceipt(receiptFixture.valid), JSON.stringify(validateReceipt.errors));
  for (const row of receiptFixture.hostile) {
    const hostile = { ...structuredClone(receiptFixture.valid), ...row.patch };
    assert.equal(validateReceipt(hostile), false, `receipt schema accepted ${row.id}`);
  }
  const config = readFileSync(join(base, "🎚️config/🦀️.rs"), "utf8");
  const editor = readFileSync(join(base, "🦀️.rs"), "utf8");
  const crate = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/🦀️.rs"), "utf8");
  const exact = (commandSource: string, configSource: string, editorSource: string, crateSource: string): boolean =>
    commandSource.includes("DirectoryEventPageV1::parse_canonical_json")
    && commandSource.includes("apply_directory_event_page")
    && commandSource.includes("DirectoryProjectionReceiptV1::SCHEMA")
    && commandSource.includes("events: vec![event]")
    && !commandSource.includes("unwrap_or_default")
    && configSource.includes("ReplaceDirectoryProjection")
    && configSource.includes("page.after_seq_exclusive != current.cursor")
    && configSource.includes("directory.cursor = page.through_seq_inclusive")
    && configSource.includes("directory_session_binding_sha256")
    && configSource.includes("directory_authorization_generation")
    && configSource.includes("directory_receipt_sha256")
    && configSource.includes("pub struct DirectoryProjectionReceiptV1")
    && configSource.includes("pub fn directory_projection_receipt")
    && editorSource.includes('"applyDirectoryEventPage"')
    && editorSource.includes("ArtifactToolPublicationLane::Config")
    && !editorSource.includes('str_field("pageJson").or_else(|| str_field("page_json"))')
    && crateSource.includes("pub mod apply_directory_event_page;");
  assert(exact(command, config, editor, crate), "Home directory event-page retained owner is incomplete");
  for (const hostile of [
    command.replace("DirectoryEventPageV1::parse_canonical_json", "pack::from_json_str"),
    command.replace("apply_directory_event_page", "fold_directory_events"),
    `${command}\nlet _ = malformed.unwrap_or_default();`,
  ]) assert.equal(exact(hostile, config, editor, crate), false);
  assert.equal(exact(command, config.replace("directory.cursor = page.through_seq_inclusive", ""), editor, crate), false);
  for (const leaf of ["🦀️.rs", "🟦️.ts", "🔗️.graphql", "🔣️.json", "🛰️.proto"]) {
    assert(readFileSync(join(base, `🎚️config/🧬️schema/${leaf}`), "utf8").includes("directory"), `config schema leaf ${leaf} lacks directory state`);
  }
  return 27;
}

class HomeDirectoryEventPageOwnerCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("home-directory-event-page-owner-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        ...homeExactCargoEnvironment(),
        groups: [{
          package: "semio-s-plugin-space",
          target: { kind: "lib" },
          laws: ["editor::home::commands::apply_directory_event_page::tests::sealed_page_replaces_projection_once_and_rejects_races"],
        }],
        progress(event) { console.log(`home-directory-event-page-owner ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`home-directory-event-page-owner-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`home-directory-event-page-owner-check: checks=${homeDirectoryEventPageOwnerOracle(this.repoRoot)} clean`);
  }
}

/** 🪪️ Proves only the current Hub author identity receives Home administration affordances. */
export function homeDirectoryIdentityRowsOracle(repoRoot: string): number {
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any");
  const controller = readFileSync(join(base, "✏️editor/🦀️.rs"), "utf8");
  const editor = readFileSync(join(base, "✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const viewer = readFileSync(join(base, "👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const homeViewerApp = readFileSync(join(base, "👁️viewer/🦀️.rs"), "utf8");
  const homeOperations = readFileSync(join(base, "🧬️schema/⚙️operations/🦀️.rs"), "utf8");
  const homeBinary = readFileSync(join(base, "🧬️schema/🧬️mutations/💾️binary/🦀️.rs"), "utf8");
  const spaceOperations = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"), "utf8");
  const spaceEditor = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const spaceViewer = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"), "utf8");
  const spaceViewerApp = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"), "utf8");
  const spaceMembers = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/👥️members/🦀️.rs"), "utf8");
  const spaceIndexController = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), "utf8");
  const spaceEngine = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs"), "utf8");
  const spaceCrateRoot = join(repoRoot, "✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust");
  const spaceCrate = readFileSync(join(spaceCrateRoot, "🦀️.rs"), "utf8");
  const spaceShared = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/🦀️.rs"), "utf8");
  const spaceConfig = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/🦀️.rs"), "utf8");
  const exportMedia = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs"), "utf8");
  const setAppRegistrations = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/📇️set-app-registrations/🦀️.rs"), "utf8");
  const createStudio = readFileSync(join(base, "✏️editor/🎮️commands/🏗️create-studio/🦀️.rs"), "utf8");
  const cataloguePanel = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🛍️catalogue/🦀️.rs"), "utf8");
  const inspectionPanel = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🦀️.rs"), "utf8");
  const parametersPanel = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/📌️panels/🔢️parameters/🦀️.rs"), "utf8");
  const nodeGraphEdit = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🦀️.rs"), "utf8");
  const setActiveExample = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🎬️set-active-example/🦀️.rs"), "utf8");
  const setActivePanelTab = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/⚙️set-active-panel-tab/🦀️.rs"), "utf8");
  const workflowWindow = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs"), "utf8");
  const compiledDagWindow = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🦀️.rs"), "utf8");
  const spawnApp = readFileSync(join(repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🚀️spawn-app/🦀️.rs"), "utf8");
  const ownerScript = readFileSync(import.meta.filename, "utf8");
  const osHost = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs"), "utf8");
  const exact = (editorSource: string, viewerSource: string): boolean => editorSource.includes("row.role == Some(crate::DirectorySpaceRole::Author)")
    && editorSource.includes('home_row_action(IconName::Users, labels.action_manage, "manageSpace", &row.id)')
    && editorSource.includes('assert_eq!(buttons.len(), 5')
    && editorSource.includes('text_arg(manage_button, "spaceId")')
    && editorSource.includes("spectator_and_unbound_hub_rows_only_carry_open")
    && editorSource.includes("role: Some(crate::DirectorySpaceRole::Spectator)")
    && editorSource.includes("role: None")
    && viewerSource.includes('origin: "hub", role: None');
  assert(controller.includes("fold_directory_events, manage_space, presence_heartbeat"), "Home controller does not import the manageSpace command module");
  assert(exact(editor, viewer), "Home identity rows expose administration without current author authority");
  const catalogGenerationFixture = "🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/📇️bumps-the-36f82f/🦀️.rs";
  const catalogGenerationSource = readFileSync(join(base, catalogGenerationFixture), "utf8");
  assert(existsSync(join(base, catalogGenerationFixture)), "Home catalog-generation fixture is not present at its canonical bounded physical path");
  assert(spaceCrate.includes(`../../🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/${catalogGenerationFixture}`), "Space crate mounts a stale logical path instead of the canonical bounded fixture path");
  const spaceBase = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any");
  const createArtifactFixture = "🧬️schema/🧬️mutations/🌱create-artifact/🧪️tests/🗿️appends-artifact-3-4665d4/🦀️.rs";
  assert(existsSync(join(spaceBase, createArtifactFixture)), "Space create-artifact fixture is not present at its canonical bounded physical path");
  assert(spaceCrate.includes(`../../🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/${createArtifactFixture}`), "Space crate mounts a stale logical create-artifact fixture path");
  const missingMounts = [...spaceCrate.matchAll(/#\[path = "([^"]+)"\]/g)]
    .map((match) => match[1])
    .filter((mount) => !existsSync(join(spaceCrateRoot, mount)));
  assert.deepEqual(missingMounts, [], `Space crate mounts missing physical paths: ${missingMounts.join(", ")}`);
  assert(homeOperations.includes("use protocol::os_spr::testkit::{") && spaceOperations.includes("use protocol::os_spr::testkit::{"), "Home or Space mutation laws import the Pack testkit instead of the current SPR testkit");
  const operationSources = `${homeOperations}\n${spaceOperations}`;
  assert(!operationSources.includes("protocol::testkit::assert_"), "Home or Space mutation laws retain the removed Pack testkit path");
  const sprLawCalls = operationSources.split("\n").filter((line) => /\bassert_(?:fatal_never_applies|missing_target_is_error|mutation_diff_absorb_law|mutation_inverse_law|outcome_policy_matrix)\(/.test(line));
  assert(sprLawCalls.length === 14 && sprLawCalls.every((call) => call.includes(".await;")), "Home or Space mutation laws do not await the current async SPR testkit");
  assert(homeBinary.includes("ArtifactStore::new(envelope).await") && homeBinary.includes(".dispatch(store::ArtifactCommand::Apply") && homeBinary.includes(" }).await.expect"), "Home document codec law does not await the current Store construction and dispatch boundary");
  assert(catalogGenerationSource.includes("dsl::from_dsl_value(pack::json_to_dsl_value(&json))") && !catalogGenerationSource.includes("serde_json::from_str(BEFORE)"), "Home snapshot fixture bypasses the first-party value codec");
  assert(!spaceEngine.includes("Some(&json!(") && !spaceEngine.includes("Some(&pack::json!(") && spaceEngine.match(/pack::json_to_dsl_value\(&pack::json!\(/g)?.length === 3, "Space checkpoint tests do not convert first-party JSON into the current DSL action boundary");
  assert(editor.includes("LocalizedLabel, WindowKindDefinition") && !editor.includes("let UiNode::") && spaceEditor.includes("IconName, WindowKindDefinition") && !spaceEditor.includes("let UiNode::"), "Home or Space row tests do not use the current fixed BuiltNode projection");
  assert(!spaceViewer.includes("let UiNode::") && spaceViewer.includes("BuiltTreeRetirement::new"), "Space viewer rows bypass the current fixed BuiltNode projection and retirement boundary");
  assert(homeViewerApp.includes("create_home_viewer().await") && homeViewerApp.includes("project_and_retire_fixture_tree(tree)"), "Home viewer fixtures do not await and retire the current manifest/render boundaries");
  assert(spaceViewerApp.includes("let def = create_space_index_viewer();") && !spaceViewerApp.includes("create_space_index_viewer().await") && spaceViewerApp.includes("project_and_retire_fixture_tree(tree)"), "Space viewer fixtures do not use the synchronous manifest and current retained render boundary");
  assert(editor.includes("fn render_rows_wrapped(") && editor.includes("render_rows_wrapped(rows, &HomeTableLabels::NATIVE_EN, &SHomeLabels::NATIVE_EN)"), "Home production and injected-row composition do not share the current fallible node builder");
  assert(!editor.includes("async fn one_local_row()") && !editor.includes("async fn one_hub_row()"), "Pure Home row fixtures are needlessly async");
  assert(controller.includes("semio_framework_plugin::testkit::new_app::<EditorApp<HomeApp>>().await"), "Home testkit does not await async app construction");
  assert(spaceEngine.includes("semio_framework_plugin::testkit::new_app::<SpaceApp>().await"), "Space testkit does not await bare async app construction");
  assert(spaceEngine.includes("new_registered_app::<SpaceApp, _>(create_space_app()).await"), "Space testkit does not await its async manifest through the registered constructor");
  assert(spaceEngine.includes("SpaceApp::initial_snapshot().await.graph.nodes.is_empty()"), "Space snapshot fixture dereferences the current async initial snapshot before awaiting it");
  assert(spaceEngine.match(/VcsArtifactApp::<SpaceApp>::new\(SpaceApp::default\(\)\)\.await/g)?.length === 2 && !spaceEngine.includes("pack::to_json_string(&SpaceApp::render(") && spaceEngine.match(/plugin_testkit::project_and_retire_fixture_tree\(/g)?.length === 4, "Space fixtures do not select the current member type or await and retire rendered component trees");
  assert(spaceEngine.includes("space_workflow_context_menu_items(&registry, labels, false, None, &selected_node_ids).await"), "Space context-menu fixture dereferences the current async projection before awaiting it");
  assert(spaceEngine.includes("pack::json_from_dsl_value(&dsl::ToValue::to_value(&base))") && spaceEngine.includes("let post_oracle: serde_json::Value = serde_json::from_str"), "Space config law bypasses the first-party value codec or independent JSON oracle");
  assert(!spaceEngine.includes("let projection = demo_space_projection();") && !spaceEngine.includes("let app = create_space_app();") && !spaceEngine.includes("let studio = create_space_app();"), "Space engine retains an unawaited async fixture");
  assert(!spaceEngine.includes("VcsArtifactApp::new(SpaceApp::default());") && !spaceEngine.includes('testkit::test_surface_id("draw"),'), "Space engine retains an unawaited app or surface fixture");
  assert(spaceIndexController.includes("pub async fn new_app() -> SpaceIndexApp") && spaceIndexController.includes("framework_new_app::<EditorApp<SpaceIndexEditor>>().await"), "Space index testkit does not await async app construction");
  assert(!/ActionRef::new\([^\n]+\)\?/.test(spaceIndexController), "Space index dialog fixtures retain fallibility from the current infallible action reference constructor");
  assert(!spaceIndexController.includes("pack::to_json_string(&<SpaceIndexEditor") && spaceIndexController.includes("project_and_retire_fixture_tree"), "Space index fixtures do not admit and retire the current fallible component tree");
  assert(spaceMembers.includes("wire_and_retire(render(") && !spaceMembers.includes("pack::to_json_string(&node)"), "Space members fixtures bypass the current fallible BuiltNode and retirement boundary");
  assert(spaceShared.includes("Some(document_backbone_ref(backbone_uri).await)"), "Space document synchronization does not await its typed backbone reference");
  assert(spaceConfig.match(/round_trip\(&config, &operation\)\.await/g)?.length === 2 && exportMedia.includes("register_format_descriptors([") && exportMedia.includes(".await\n        .expect(\"register neutral format descriptor\")"), "Space config or format-registration fixtures do not await their current async boundaries");
  assert(cataloguePanel.includes(').await.expect("catalogue tree")') && setActivePanelTab.includes(').await.expect("catalogue tree")') && setActiveExample.includes("register_studio_port_for_test(&entry.id, port).await") && cataloguePanel.includes("project_and_retire_fixture_tree") && setActivePanelTab.includes("project_and_retire_fixture_tree"), "Space catalogue or studio-port fixtures bypass current async ownership and component retirement");
  assert(nodeGraphEdit.includes("serde_json::Value::as_object_mut") && nodeGraphEdit.includes("serde_json::Value::Object(position)") && !nodeGraphEdit.includes("fixture.get_mut(\"layout\").and_then(pack::JsonValue::as_object_mut)"), "Space node-graph fixture crosses serde JSON through the first-party Pack value family");
  assert(setActiveExample.includes("OsBackbonePorts::Store(store::BackbonePorts::Memory") && setActiveExample.match(/empty_workflow_snapshot\(\)\.await/g)?.length === 5 && setActivePanelTab.includes("let projection = empty_workflow_snapshot().await;"), "Space fixtures retain a stale backbone enum or unresolved workflow snapshot future");
  assert(setActiveExample.match(/load_document_snapshot\(&emit\)\.await/g)?.length === 2 && workflowWindow.match(/\.render\(S_PLAY_BODY_WORKFLOW,[^;]+\.await\.expect\("render"\)/g)?.length === 2 && compiledDagWindow.includes('.render(S_PLAY_BODY_COMPILED_DAG, None, &ViewModel::default()).await.expect("render")') && workflowWindow.match(/project_and_retire_fixture_tree\(node\)/g)?.length === 2 && compiledDagWindow.includes("project_and_retire_fixture_tree(node)"), "Space window fixtures retain unresolved async renders or unretired component owners");
  const spaceAppFixtures = `${workflowWindow}\n${compiledDagWindow}\n${spawnApp}`;
  assert(!spaceAppFixtures.includes("VcsArtifactApp::new(crate::engine::space::SpaceApp::default())") && spaceAppFixtures.match(/VcsArtifactApp::<crate::engine::space::SpaceApp>::new/g)?.length === 4, "Space fixtures leave the current NoMembers app owner ambiguous");
  const fixedPanelFixtures = `${inspectionPanel}\n${parametersPanel}`;
  assert(!fixedPanelFixtures.includes("pack::to_json_string(&node)") && fixedPanelFixtures.match(/project_and_retire_fixture_tree\(semio_framework_plugin::ComponentTree \{ root: node \}\)/g)?.length === 3, "Space panel fixtures serialize retained BuiltNode owners instead of projecting and retiring them");
  assert(spaceShared.match(/assert_(?:viewer_never_mutates|editor_and_viewer_share_dialect)::<[^;]+>\(\)\.await;/g)?.length === 4 && homeViewerApp.match(/assert_(?:viewer_never_mutates|editor_and_viewer_share_dialect)::<[^;]+>\(\)\.await;/g)?.length === 2, "Home or Space surface tests leave the async testkit future unpolled");
  assert(createStudio.includes("resolve_ready(crate::register_studio_port(&entry.id, port))") && exportMedia.includes("resolve_ready(crate::ensure_space_fixtures_registered())") && setAppRegistrations.includes("resolve_ready(crate::engine::space::engine::apply_app_registrations(&payload.json))"), "Synchronous Space command handlers leave an async registry side effect unpolled");
  for (const law of [
    "editor::home::modes::explore::windows::main::component::tests::a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions",
    "editor::home::modes::explore::windows::main::component::tests::spectator_and_unbound_hub_rows_only_carry_open",
    "viewer::home::modes::view::windows::main::component::tests::a_row_stamps_the_space_row_id",
  ]) assert(ownerScript.includes(law), `Home native gate omitted the current exact selector ${law}`);
  assert(ownerScript.includes('RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432"'), "Home exact builds do not bound compiler worker stacks");
  assert(ownerScript.includes('nativeEnv: { RUST_MIN_STACK: "268435456" }'), "Home exact laws lost their native runtime stack");
  for (const hostile of [
    editor.replace("row.role == Some(crate::DirectorySpaceRole::Author)", 'row.origin == "hub"'),
    editor.replace('assert_eq!(buttons.len(), 5', 'assert_eq!(buttons.len(), 4'),
    editor.replace("role: Some(crate::DirectorySpaceRole::Spectator)", "role: Some(crate::DirectorySpaceRole::Author)"),
  ]) assert.equal(exact(hostile, viewer), false);
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
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        ...homeExactCargoEnvironment(),
        groups: [
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
            package: "semio-s-plugin-space",
            target: { kind: "lib" },
            laws: [
              "editor::home::modes::explore::windows::main::component::tests::a_hub_row_stamps_the_space_row_id_and_carries_dispatchable_row_actions",
              "editor::home::modes::explore::windows::main::component::tests::spectator_and_unbound_hub_rows_only_carry_open",
              "viewer::home::modes::view::windows::main::component::tests::a_row_stamps_the_space_row_id",
            ],
          },
        ],
        progress(event) { console.log(`home-directory-identity-rows ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`home-directory-identity-rows-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`home-directory-identity-rows-check: checks=${homeDirectoryIdentityRowsOracle(this.repoRoot)} clean`);
  }
}

/** 🪪️ Proves every authority that names the OS host plugin names the SAME identity — the
 * language-agnostic tuple in `🧪️fixtures/🧫️plugin-identity/🔣️.json`, validated against its own schema by
 * a third-party oracle (ajv 2020), then joined to: the Cargo `[package.metadata.component] package`, the
 * plugin root's `builder(…)`/`package_id(…)` literals, the hand-authored deployment catalog row (public
 * id + physical module directory), and the generated registry row (`pluginId`/`packageId`/`packageName`/
 * `host`). The playground VARIANT (`s`) is a different name and is pinned separately against the Cargo
 * `[[package.metadata.semio.playground]]` row and the generated `DEFAULT_HOST_VARIANT`, so the two can
 * never be conflated again — the 2026-09-05 regression was `builder("space")` landing in Rust alone while
 * every other authority still said `s`, which the wasm assembly gate only reported 90 minutes later. */
export function spacePluginIdentityOracle(repoRoot: string): number {
  const plugin = join(repoRoot, "✏️s/🔌️plugins/🪐️space");
  const registryRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
  const fixture = JSON.parse(readFileSync(join(plugin, "🧪️fixtures/🧫️plugin-identity/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(plugin, "🧬️schema/🔣️.json"), "utf8"));
  const identityAjv = new Ajv({ strict: true, allErrors: true });
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

  const deployment = JSON.parse(readFileSync(join(registryRoot, "📦️deployment/🗺️catalog.json"), "utf8"));
  const row = deployment.modules.find((entry: { pluginId: string }) => entry.pluginId === fixture.pluginId);
  assert(row, `deployment catalog has no row for ${fixture.pluginId}`);
  assert.equal(row.directoryName, fixture.moduleDirectoryName, "deployment catalog directory is not the fixture module directory");

  const registry = JSON.parse(readFileSync(join(registryRoot, "🤖️generated/🔌️plugins.json"), "utf8"));
  const entry = registry.find((candidate: { pluginId: string }) => candidate.pluginId === fixture.pluginId);
  assert(entry, `generated registry has no row for ${fixture.pluginId}`);
  assert.equal(entry.packageId, fixture.packageId, "generated registry packageId is not the fixture identity");
  assert.equal(entry.packageName, fixture.packageName, "generated registry packageName is not the fixture identity");
  assert.deepEqual(entry.host, fixture.host, "generated registry host config is not the fixture host config");
  assert.deepEqual(
    deployment.modules.map((module: { pluginId: string }) => module.pluginId),
    registry.map((candidate: { pluginId: string }) => candidate.pluginId),
    "deployment catalog and generated registry disagree on the public identity roster",
  );

  const playgrounds = readFileSync(join(registryRoot, "🤖️generated/🎮️playgrounds.ts"), "utf8");
  assert(playgrounds.includes(`export const DEFAULT_HOST_VARIANT = ${JSON.stringify(fixture.playgroundVariant)}`), "DEFAULT_HOST_VARIANT is not the declared playground variant");
  assert(playgrounds.includes(`{ variant: ${JSON.stringify(fixture.playgroundVariant)}, pluginId: ${JSON.stringify(fixture.pluginId)},`), "the playground variant row does not resolve to the plugin identity");

  const hosts = readFileSync(join(registryRoot, "🤖️generated/🖥️hosts.rs"), "utf8");
  assert(hosts.includes(`PluginHostConfig { plugin_id: ${JSON.stringify(fixture.pluginId)}, landing_app_id: ${JSON.stringify(fixture.host.landingAppId)}, host_app_id: ${JSON.stringify(fixture.host.hostAppId)} }`), "the generated Rust host table is not the fixture host config");
  return 11;
}

/** 🧵️ Proves the three `🪐️space` app surfaces declare exactly the interactive-job dispositions their
 * language-neutral fixtures declare, and reports the committed descriptor's drift against them. The
 * descriptor is regenerated only by a full `wasm32-wasip2` build (`describe`), so a stale one is
 * expected — what is NOT tolerated is a descriptor that carries `interactiveJob` and disagrees. */
export function interactiveJobCatalogOracle(repoRoot: string): number {
  const plugin = join(repoRoot, "✏️s/🔌️plugins/🪐️space");
  const surfaces = [
    { appId: "s.space.studio@1/*#editor", owner: join(plugin, "⚙️engine/🪐️space"), scope: plugin, export: "SpacePlayRetainedCommandLimits", source: join(plugin, "⚙️engine/🪐️space/🦀️.rs"), shape: "status" as const, factory: "SpaceCommandJobFactory" },
    { appId: "s.space.home@1/*#editor", owner: join(plugin, "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"), scope: join(plugin, "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any"), export: "HomeRetainedCommandLimits", source: join(plugin, "🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), shape: "disposition" as const, factory: "HomeRetainedCommandJobFactory" },
    { appId: "s.space.space@1/*#editor", owner: join(plugin, "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"), scope: join(plugin, "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any"), export: "SpaceIndexRetainedCommandLimits", source: join(plugin, "🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"), shape: "status" as const, factory: "SpaceIndexRetainedCommandJobFactory" },
  ];
  const descriptor = JSON.parse(readFileSync(join(plugin, "🔣️.json"), "utf8"));
  let checks = spacePluginIdentityOracle(repoRoot);
  let staleRows = 0;
  for (const surface of surfaces) {
    const fixtureRoot = join(surface.owner, "🧪️fixtures/🧫️retained-command-limits");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const validate = compileRetainedCommandLimits(repoRoot, surface.scope, surface.export);
    assert(validate(fixture), `${surface.appId} fixture violates ${surface.export}: ${JSON.stringify(validate.errors)}`);
    checks += 1;
    const migrated: string[] = fixture.routes.filter((route: any) => (surface.shape === "status" ? route.status === "migrated" : route.disposition === "Migrated")).map((route: any) => route.id);
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

class InteractiveJobCatalogCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("interactive-job-catalog-check accepts only --native");
    if (segments[0] === "--native") {
      const receipts = await runExactCargoLaws({
        cwd: this.root,
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
        groups: [{
          package: "semio-s-plugin-space",
          target: { kind: "lib" },
          laws: [
            "interactive_job_catalog_tests::studio_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory",
            "interactive_job_catalog_tests::home_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory",
            "interactive_job_catalog_tests::space_index_declares_every_fixture_migrated_id_and_backs_it_with_the_owned_factory",
            "interactive_job_catalog_tests::tool_proof_catalogs_match_the_runtime_identity_they_are_joined_against",
            "interactive_job_catalog_tests::manifest_plugin_id_matches_the_cargo_component_package",
            "interactive_job_catalog_tests::plugin_assembly_succeeds_and_registers_all_five_surfaces",
            "interactive_job_catalog_tests::every_app_instance_constructs_against_its_registered_proof_catalog",
          ],
        }],
        progress(event) { console.log(`interactive-job-catalog ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
      });
      console.log(`interactive-job-catalog-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`interactive-job-catalog-check: checks=${interactiveJobCatalogOracle(this.repoRoot)} clean`);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-space", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript).register("home-directory-projection-persistence-check", HomeDirectoryProjectionPersistenceCheckScript).register("home-directory-event-page-owner-check", HomeDirectoryEventPageOwnerCheckScript).register("home-directory-identity-rows-check", HomeDirectoryIdentityRowsCheckScript).register("interactive-job-catalog-check", InteractiveJobCatalogCheckScript).register("plugin-identity-check", PluginIdentityCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
