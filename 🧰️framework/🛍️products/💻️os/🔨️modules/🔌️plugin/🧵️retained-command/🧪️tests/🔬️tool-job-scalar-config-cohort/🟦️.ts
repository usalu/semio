import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT, toolJobOwnerSourceEvidence } from "../../../../../../../../📜️script.ts";

/** 🧪️ Checks scalar Config publication laws against strict Ajv, Immer, and exact live sources. */
export function toolJobScalarConfigCohortSelfTests(): { routes: number; migrated: number; batchOnly: number; forbidden: number; mutationOracles: number; hostileCases: number } {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command");
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/🎚️scalar-config-cohort.json"), "utf8"));
  const requireTest = createRequire(import.meta.url);
  const Ajv = requireTest("ajv");
  const { produceWithPatches, applyPatches, enablePatches } = requireTest("immer");
  enablePatches();
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/ScalarConfigCohortV1" });
  if (!validate(fixture)) throw new Error(`scalar Config fixture schema: ${JSON.stringify(validate.errors)}`);
  const files = new Map<string, string>(fixture.sources.map((file: string) => [file, readFileSync(join(WORKSPACE_ROOT, file), "utf8")]));
  const evidence = toolJobOwnerSourceEvidence(files);
  if (evidence.failures.length || evidence.scanThenMonolith.length) throw new Error(`scalar Config source evidence: ${JSON.stringify(evidence.failures)}`);
  let routes = 0, migrated = 0, batchOnly = 0, forbidden = 0, hostileCases = 0;
  for (const owner of fixture.owners) {
    const actual = evidence.rows.filter((row) => row.file === owner.file);
    if (actual.length !== owner.routes.length || new Set(owner.routes.map((route: { id: string }) => route.id)).size !== actual.length) throw new Error(`scalar Config route bijection ${owner.id}`);
    for (const route of owner.routes) {
      routes++;
      const disposition = evidence.dispositions.find((candidate) => candidate.key === `${owner.file}\0${route.id}`)?.disposition;
      if (!actual.some((row) => row.id === route.id) || disposition !== route.disposition) throw new Error(`scalar Config route disposition ${owner.id}/${route.id}`);
      const owned = evidence.appOwned.find((candidate) => candidate.ownerFile === owner.file && candidate.toolId === route.id);
      if (route.disposition === "Migrated") {
        migrated++;
        if (!owned?.publicationReady || JSON.stringify(owned.publicationLanes) !== JSON.stringify(route.lanes) || route.blocker !== "") throw new Error(`scalar Config exact publication ${owner.id}/${route.id}`);
      } else {
        if (owned || route.lanes.length || !route.blocker) throw new Error(`scalar Config false batch admission ${owner.id}/${route.id}`);
        if (route.disposition === "ForbiddenFromUi") forbidden++; else batchOnly++;
      }
    }
    const source = files.get(owner.file)!;
    const witnesses = [
      `const ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;`,
      `const ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;`,
      "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()",
      "request.authority.actor().len() > 64", "self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()",
      `_config_text_bytes(request.base.get()) > ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES`,
      `grant.maximum_bytes < ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing`,
      "if self.checkpoint.cursor != 0", "inverse: vec![inverse]", "protocol::UndoPolicy::ExactBaseOnly", "authority.prepare_one_item(edit, std::sync::Arc::new(next))?",
      "if !base.return_to_registry()", "self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some()",
    ];
    const exact = (candidate: string) => witnesses.every((witness) => candidate.includes(witness));
    if (!exact(source)) throw new Error(`scalar Config owner preparation contract ${owner.id}`);
    for (const witness of witnesses) {
      if (exact(source.replaceAll(witness, "REMOVED_BY_HOSTILE_FIXTURE"))) throw new Error(`scalar Config hostile preparation witness ${owner.id}/${witness}`);
      hostileCases++;
    }
  }
  for (const law of fixture.mutations) {
    const actual = { ...law.base, ...law.changes };
    const [oracle, patches, inverse] = produceWithPatches(law.base, (draft: Record<string, unknown>) => { for (const [key, value] of Object.entries(law.changes)) draft[key] = value; });
    const reverse = { ...actual };
    for (const key of Object.keys(law.changes)) reverse[key] = law.base[key];
    if (JSON.stringify(actual) !== JSON.stringify(law.expected) || JSON.stringify(oracle) !== JSON.stringify(law.expected)
      || JSON.stringify(reverse) !== JSON.stringify(law.base) || JSON.stringify(applyPatches(oracle, inverse)) !== JSON.stringify(law.base)
      || JSON.stringify(applyPatches(law.base, patches)) !== JSON.stringify(law.expected)) throw new Error(`scalar Config Immer replay oracle ${law.owner}/${law.id}`);
  }
  for (const hostile of [{ ...fixture, extra: true }, { ...fixture, grantBytes: 4_097 }, { ...fixture, textMaximumBytes: 129 }, { ...fixture, owners: [{ ...fixture.owners[0], extra: true }, ...fixture.owners.slice(1)] }]) {
    if (validate(hostile)) throw new Error("scalar Config strict schema accepted a hostile fixture");
    hostileCases++;
  }
  return { routes, migrated, batchOnly, forbidden, mutationOracles: fixture.mutations.length, hostileCases };
}
