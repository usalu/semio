import { expect, test } from "bun:test";
import { resolve } from "node:path";
import Ajv from "ajv";

type Group = { status: "Migrated" | "BatchOnlyPendingRewrite"; lanes: string[]; routes: string[]; blocker?: string };
type Fixture = { routeCount: number; retainedRoutes: string[]; frameworkOwnedRoutes: string[]; groups: Group[]; globals: unknown[]; scanThenMonolithRoutes: string[]; laws: Record<string, boolean> };

const root = resolve(import.meta.dir, "..");
const sourcePath = resolve(root, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
const retainedPath = resolve(root, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs");
const schemaPath = resolve(import.meta.dir, "🔣️.schema.json");
const fixturePath = resolve(import.meta.dir, "🔣️.json");

const exact = (left: string[], right: string[]) => new Set(left).size === left.length && new Set(right).size === right.length && JSON.stringify([...left].sort()) === JSON.stringify([...right].sort());

test("third-party Ajv accepts the strict Note cohort fixture", async () => {
  const schema = await Bun.file(schemaPath).json();
  const fixture = await Bun.file(fixturePath).json() as Fixture;
  const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

test("Note source and fixture have one exact hostile census", async () => {
  const fixture = await Bun.file(fixturePath).json() as Fixture;
  const source = await Bun.file(sourcePath).text();
  const retainedSource = await Bun.file(retainedPath).text();
  const commands = [...source.matchAll(/^\s*"([^"]+)" as "[^"]+" =>/gm)].map((match) => match[1]!);
  const manifests = new Map([...source.matchAll(/\.action_interactive_job\("([^"]+)",\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => [match[1]!, match[2]!]));
  const classified = fixture.groups.flatMap((group) => group.routes);
  const retained = fixture.groups.filter((group) => group.status === "Migrated").flatMap((group) => group.routes);
  expect(exact(commands, classified)).toBe(true);
  expect(commands.length).toBe(fixture.routeCount);
  expect(exact([...manifests.keys()], classified)).toBe(true);
  expect(exact(fixture.retainedRoutes, retained)).toBe(true);
  expect(fixture.groups.every((group) => group.routes.every((route) => manifests.get(route) === group.status))).toBe(true);
  expect(fixture.groups.every((group) => !group.lanes.includes("HostOnly") || group.lanes.length === 1)).toBe(true);
  expect(fixture.frameworkOwnedRoutes).toEqual([]);
  expect(fixture.globals).toEqual([]);
  expect(fixture.scanThenMonolithRoutes).toEqual([]);
  expect(Object.values(fixture.laws).every(Boolean)).toBe(true);
  const retainedIdsSource = retainedSource.slice(retainedSource.indexOf("pub const NOTE_RETAINED_TOOL_IDS"), retainedSource.indexOf("pub const NOTE_AUDITED_PUBLICATION_CONTRACTS"));
  const registered = [...retainedIdsSource.matchAll(/^\s*"([^"]+)",$/gm)].map((match) => match[1]!);
  const retainedContractsSource = retainedSource.slice(retainedSource.indexOf("pub const NOTE_RETAINED_PUBLICATION_CONTRACTS"), retainedSource.indexOf("fn note_contract"));
  const publication = [...retainedContractsSource.matchAll(/tool_id: "([^"]+)"/g)].map((match) => match[1]!);
  expect(exact(registered, fixture.retainedRoutes)).toBe(true);
  expect(exact(publication, fixture.retainedRoutes)).toBe(true);
  const proofSource = source.slice(source.indexOf("semio_framework_plugin::bounded_first_step_tool_proofs!"), source.indexOf("fn register_tool_job_factories"));
  const proofs = [...proofSource.matchAll(/^\s*"([^"]+)"\s*=>/gm)].map((match) => match[1]!);
  expect(exact(proofs, fixture.retainedRoutes)).toBe(true);
  expect(proofSource).toContain('factory: "BoundedFirstStepCommandJobFactory"');
});

test("hostile source rejects global ids, copied digests, and scan-then-monolith shells", async () => {
  const source = await Bun.file(sourcePath).text();
  const retainedSource = await Bun.file(retainedPath).text();
  const schemaSource = await Bun.file(resolve(root, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs")).text();
  expect(source).not.toContain("BoundedArtifactCommandWork");
  expect(retainedSource).not.toContain("BoundedArtifactCommandWork");
  expect(`${source}\n${retainedSource}`).not.toContain("semio_framework_hash");
  expect(retainedSource).not.toMatch(/fn\s+note_store_edit_digest/);
  expect(retainedSource).not.toContain("ArtifactStoreOneItemPrepared {");
  expect(retainedSource).not.toContain("fn prepare_note_artifact");
  expect(retainedSource).toContain("authority.prepare_one_item(edit");
  expect(retainedSource).toContain("NOTE_MATERIALIZATION_STRING_CHUNK_BYTES: usize = 1_024");
  expect(retainedSource).toContain("local_owner::<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot>()");
  expect(retainedSource).toContain("child.with_local_owner(owner)");
  expect(retainedSource).toContain("text_child_materialization_preserves_present_typed_owner");
  expect(retainedSource).toContain("text_child_materialization_preserves_absent_owner");
  expect(retainedSource).toContain("text_child_materialization_cancellation_retires_partial_metadata");
  expect(retainedSource).toContain("struct NoteSnapshotMaterializationCursor");
  expect(retainedSource).toContain("struct NoteBlockMaterializationCursor");
  expect(retainedSource).toContain("struct NoteArtifactLinkMaterializationCursor");
  expect(retainedSource).toContain(".range::<str, _>((std::ops::Bound::Excluded(last_key.as_str()), std::ops::Bound::Unbounded))");
  expect(retainedSource).toContain("snapshot_materialization_copies_every_nested_owner_and_preserves_typed_text_arc");
  expect(retainedSource).toContain("snapshot_materialization_preserves_absent_typed_owner");
  expect(retainedSource).toContain("snapshot_materialization_cancellation_during_nested_metadata_reaches_terminal_emptiness");
  expect(retainedSource).toContain("struct NoteRootScalarPreparation");
  expect(retainedSource).toContain("NoteMutation::ChangeGridVisible");
  expect(retainedSource).toContain("NoteMutation::ChangeGridSpacing");
  expect(retainedSource).toContain("Note one-item Artifact preparation admits only exact retained root-scalar mutations on the document lane");
  expect(retainedSource).toContain("if self.cursor + 1 < self.units.len()");
  expect(retainedSource).toContain("snapshot_materialization_rejects_stale_operation_authority_and_retires");
  expect(retainedSource).toContain("root_scalar_preflight_admits_only_exact_valid_document_mutations");
  expect(schemaSource).not.toContain("static NEXT");
  expect(schemaSource).not.toContain("AtomicU64");
  expect(schemaSource).toContain("pub struct NoteIdOwner");
});
