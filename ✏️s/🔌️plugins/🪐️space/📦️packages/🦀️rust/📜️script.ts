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

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-space"], this.repoRoot);
  }
}

/** 📇️ Proves Home projection persistence is document-complete and corruption-explicit. */
export function homeDirectoryProjectionPersistenceOracle(repoRoot: string): number {
  const base = join(repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/📇️projection-persistence-v1/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧪️fixtures/📇️projection-persistence-v1/🧬️.schema.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert(validate(fixture), JSON.stringify(validate.errors));
  const exactKeys = (value: unknown, keys: string[]): boolean => Boolean(value) && typeof value === "object" && !Array.isArray(value) && JSON.stringify(Object.keys(value as object).sort()) === JSON.stringify([...keys].sort());
  const decode = (text: string): unknown => {
    const wire = JSON.parse(text);
    assert(exactKeys(wire, ["spaces", "cursor", "users"]));
    assert(Number.isSafeInteger(wire.cursor) && wire.cursor >= 0);
    for (const space of Object.values(wire.spaces as Record<string, unknown>)) {
      assert(exactKeys(space, ["view", "members", "documents"]));
      assert(Array.isArray((space as { documents?: unknown }).documents));
    }
    return wire;
  };
  const canonical = JSON.stringify(fixture.wire);
  assert.deepEqual(decode(canonical), fixture.wire);
  assert.deepEqual(Object.values(fixture.wire.spaces).flatMap((space: any) => space.documents.map((document: any) => document.documentId)), fixture.expectedDocumentIds);
  for (const malformed of fixture.malformed) assert.throws(() => decode(malformed));
  const hostileFixture = structuredClone(fixture);
  delete hostileFixture.wire.spaces["space-α"].documents;
  assert.equal(validate(hostileFixture), false);
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const exactSource = (text: string): boolean => text.includes("documents: Vec<store::os_directory::DocumentDescriptor>")
    && text.includes("documents: space.documents.clone()")
    && text.includes("documents: space.documents")
    && text.includes("fn directory_from_json(json: &str) -> Result<store::os_directory::DirectoryReadModel, Fault>")
    && !text.includes("pack::from_json_str(json).unwrap_or_default()")
    && text.includes("pub fn directory(&self) -> Result<store::os_directory::DirectoryReadModel, Fault>");
  assert(exactSource(source), "Home projection persistence still drops documents or defaults corruption");
  for (const hostile of [
    source.replace("documents: Vec<store::os_directory::DocumentDescriptor>", "documents_removed: Vec<store::os_directory::DocumentDescriptor>"),
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
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
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
  const retainedSchema = JSON.parse(readFileSync(join(base, "🧪️fixtures/🧫️retained-command-limits/🧬️.schema.json"), "utf8"));
  const validateRetained = new Ajv2020({ strict: false, allErrors: true }).compile(retainedSchema);
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
        env: { ...process.env, RUST_MIN_STACK: "268435456" },
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

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-space", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript).register("home-directory-projection-persistence-check", HomeDirectoryProjectionPersistenceCheckScript).register("home-directory-event-page-owner-check", HomeDirectoryEventPageOwnerCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
