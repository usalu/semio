/** 🪪️ TypeScript twin of `a_contributed_manifest_is_admitted_or_faulted_per_fixture_row`.
 *
 * Decides the SAME accept/reject question over the SAME language-agnostic fixture, with
 * `JSON.parse` and `node:assert` instead of the first-party `FlowExtensionManifest` decoder — so a
 * decoder that starts admitting (or refusing) a shape it should not is caught by two independent
 * implementations, not one. Also pins the loud-failure surface itself: the Rust registration must
 * ANSWER a typed rejection, never swallow a bad manifest and register zero operators
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-unknown-kind-2026-09-12.md` §2).
 */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const FIXTURE_URL = new URL("../../🧫️fixtures/🔣️manifest-admission.json", import.meta.url);
const REGISTRY_SOURCE_URL = new URL("../../🦀️.rs", import.meta.url);

type Row = { id: string; why: string; accepted: boolean; operators: string[]; schemas: string[]; manifestJson: string };
type Fixture = {
  schema: string;
  pluginId: string;
  faultCode: string;
  requiredManifestTextKeys: string[];
  requiredManifestListKeys: string[];
  requiredManifestRecordKeys: string[];
  requiredContributesListKeys: string[];
  requiredOperatorTextKeys: string[];
  requiredSchemaTextKeys: string[];
  generationBump: { pluginId: string; extensionId: string; manifestJson: string };
  rows: Row[];
};

const isRecord = (value: unknown): value is Record<string, unknown> => typeof value === "object" && value !== null && !Array.isArray(value);

/** 🔍️ Independent admission oracle: a manifest is admissible exactly when every declared field is
 * present with its declared shape, all the way into each contributed operator and schema. */
export function manifestIsAdmissible(fixture: Fixture, manifestJson: string): boolean {
  let manifest: unknown;
  try {
    manifest = JSON.parse(manifestJson);
  } catch {
    return false;
  }
  if (!isRecord(manifest)) return false;
  const holds = (record: Record<string, unknown>, keys: string[], shape: (value: unknown) => boolean) => keys.every((key) => key in record && shape(record[key]));
  if (!holds(manifest, fixture.requiredManifestTextKeys, (value) => typeof value === "string")) return false;
  if (!holds(manifest, fixture.requiredManifestListKeys, Array.isArray)) return false;
  if (!holds(manifest, fixture.requiredManifestRecordKeys, isRecord)) return false;
  const contributes = manifest.contributes as Record<string, unknown>;
  if (!holds(contributes, fixture.requiredContributesListKeys, Array.isArray)) return false;
  const every = (items: unknown, keys: string[]) => (items as unknown[]).every((item) => isRecord(item) && holds(item, keys, (value) => typeof value === "string"));
  return every(contributes.operators, fixture.requiredOperatorTextKeys) && every(contributes.schemas, fixture.requiredSchemaTextKeys);
}

/** 🧪️ Runs the twin. Answers how many assertions it made, the way every flow source twin does. */
export function flowExtensionManifestAdmissionSelfTests(): number {
  const fixture: Fixture = JSON.parse(readFileSync(fileURLToPath(FIXTURE_URL), "utf8"));
  assert.equal(fixture.schema, "flow.extension-manifest-admission");
  assert.ok(fixture.rows.some((row) => row.accepted) && fixture.rows.some((row) => !row.accepted), "the fixture must exercise both decisions");

  let checks = 2;
  for (const row of fixture.rows) {
    assert.equal(manifestIsAdmissible(fixture, row.manifestJson), row.accepted, `row ${row.id} (${row.why})`);
    assert.ok(row.why.length > 0, `row ${row.id} must say what it stands for`);
    if (!row.accepted) assert.deepEqual([row.operators, row.schemas], [[], []], `row ${row.id} is refused, so it contributes nothing`);
    checks += 2;
  }

  // 🧬️ The one row the contribution fold's own metadata probe cannot catch: its id/name/version
  // read perfectly, so only the FULL manifest decode stands between a broken contributor and a
  // registry that silently holds none of its operators.
  const metadataOnly = fixture.rows.find((row) => row.id === "metadata-only");
  assert.ok(metadataOnly && !metadataOnly.accepted, "the fixture owns the metadata-readable row");
  const metadata = JSON.parse(metadataOnly.manifestJson);
  for (const key of ["id", "name", "version"]) assert.equal(typeof metadata[key], "string", `metadata key ${key} must still read`);
  checks += 4;

  // 🪪️ The registration surface itself: Result-returning, typed, bilingual, and no silent return.
  const source = readFileSync(fileURLToPath(REGISTRY_SOURCE_URL), "utf8");
  const loud = (value: string) =>
    value.includes("fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) -> Result<(), FlowExtensionManifestRejection>")
    && value.includes(`pub const CODE: &'static str = "${fixture.faultCode}";`)
    && value.includes("fn build_flow_extension_registry(contributed: &BTreeMap<String, ContributedFlowExtension>) -> Result<neural::Registry, FlowExtensionManifestRejection>")
    && value.includes("register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json)?;")
    && value.includes("Flow-Erweiterungsmanifest")
    && !/^\s*let Ok\(manifest\) = .*else \{ return \}/m.test(value);
  assert.ok(loud(source), "the flow extension registry must fault a manifest it cannot read");
  for (const mutant of [
    source.replace("register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json)?;", "let _ = register_contributed_manifest(&mut registry, &entry.plugin_id, &entry.manifest_json);"),
    source.replace(`pub const CODE: &'static str = "${fixture.faultCode}";`, 'pub const CODE: &\'static str = "flow.whatever";'),
    source.replace("Flow-Erweiterungsmanifest", "flow extension manifest"),
    `${source}\n    let Ok(manifest) = decode(manifest_json) else { return };\n`,
  ]) {
    assert.ok(!loud(mutant), "the source contract accepted a hostile mutant");
    checks += 1;
  }
  assert.equal(fixture.generationBump.pluginId, `flow-extension-${fixture.generationBump.extensionId}`, "the generation-bump contributor names its own extension");
  assert.ok(manifestIsAdmissible(fixture, fixture.generationBump.manifestJson), "the generation-bump manifest must itself be admissible");
  return checks + 3;
}

if (import.meta.main) {
  console.log(`[twin] flow extension manifest admission: ${flowExtensionManifestAdmissionSelfTests()} assertions`);
}
