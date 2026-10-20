import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { toolJobOwnerSourceEvidence, toolJobOwnerSourceText } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";

/** 🧪️ Checks scalar Config publication laws against Immer and exact live source policy witnesses. */
export function toolJobScalarConfigCohortSelfTests(): { routes: number; migrated: number; batchOnly: number; forbidden: number; mutationOracles: number; hostileCases: number } {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🎚️scalar-config-cohort.json"), "utf8"));
  const requireTest = createRequire(import.meta.url);
  const { produceWithPatches, applyPatches, enablePatches } = requireTest("immer");
  enablePatches();
  
  
  const files = new Map<string, string>(fixture.sources.map((file: string) => [file, toolJobOwnerSourceText(WORKSPACE_ROOT, file)]));
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
    for (const lane of ["WindowConfig", "WindowTransient"] as const) {
      const routed = evidence.appOwned.filter((row) => row.ownerFile === owner.file && row.publicationLanes.includes(lane));
      if (!routed.length) continue;
      const name = lane === "WindowConfig" ? "register_window_config_owners" : "register_window_transient_owners";
      const missing = new Map(files);
      missing.set(owner.file, files.get(owner.file)!.replaceAll(name, "missing_window_registry_authority"));
      const rejected = toolJobOwnerSourceEvidence(missing);
      if (routed.some((row) => rejected.appOwned.find((candidate) => candidate.ownerFile === row.ownerFile && candidate.toolId === row.toolId)?.publicationReady)) throw new Error(`window registry authority omission ${owner.id}/${lane}`);
      hostileCases++;
    }

    const source = files.get(owner.file)!;
    const ownerPreparationWitnesses = owner.preparationLane === "WindowConfig" ? ["fn register_window_config_owners(", "fn register_window_transient_owners(", ...owner.registrationWitnesses] : [
      `const ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;`,
      `const ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;`,
      "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()",
      "request.authority.actor().len() > 64", "self.preflight(&request.mutation, request.lane).is_err()",
      `_config_text_bytes(request.base.get()) > ${owner.prefix}_CONFIG_TEXT_MAXIMUM_BYTES`,
      `!grant.permits_one() || grant.maximum_copy_bytes < ${owner.prefix}_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.owners.is_closing()`,
      "if self.checkpoint.cursor != 0", "authority.next_edit(mutation.clone(), vec![inverse])", "match authority.prepare_one_item(edit, std::sync::Arc::new(next))",
      "*self.owners.refused = Some((edit, post))", "self.owners.begin_close()", "self.owners.close_step(grant.retained_grant())", "self.owners.terminal_is_empty()",
    ];
    const ownerPreparationExact = (candidate: string) => ownerPreparationWitnesses.every((witness) => candidate.includes(witness));
    if (!ownerPreparationExact(source)) throw new Error(`scalar Config owner preparation contract ${owner.id}`);
    for (const witness of ownerPreparationWitnesses) {
      if (ownerPreparationExact(source.replaceAll(witness, "REMOVED_BY_HOSTILE_FIXTURE"))) throw new Error(`scalar Config hostile preparation witness ${owner.id}/${witness}`);
      hostileCases++;
    }
    const ownerWitnesses: readonly [RegExp, string][] = [
      [/type Edit = (?:[A-Za-z_][A-Za-z0-9_]*::)*(?:Window)?ConfigApplyEdit</u, "type Edit = REMOVED_BY_HOSTILE_FIXTURE<"],
      [/fn register_window_config_owners\(/u, "fn REMOVED_BY_HOSTILE_FIXTURE("],
      [/registry\s*\.register::</u, "registry.REMOVED_BY_HOSTILE_FIXTURE::<"],
    ];
    const ownerExact = (candidate: string) => ownerWitnesses.every(([pattern]) => pattern.test(candidate));
    if (!ownerExact(source)) throw new Error(`scalar Config owner window config contract ${owner.id}`);
    for (const [pattern, replacement] of ownerWitnesses) {
      if (ownerExact(source.replaceAll(new RegExp(pattern.source, "gu"), replacement))) throw new Error(`scalar Config hostile owner witness ${owner.id}/${pattern.source}`);
      hostileCases++;
    }
  }
  const store = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store");
  const preparation = ["🧬️snapshot-clone/✏️config-apply/🦀️.rs", "📬️paged-one-item/🦀️.rs"].map((file) => readFileSync(join(store, file), "utf8")).join("\n");
  const witnesses = [
    "if !mutation.admits_lane(lane) || !mutation.admissible()",
    "mutation.payload_bytes().checked_mul(4).and_then(|bytes| bytes.checked_add(CONFIG_EDIT_RETAINED_BASELINE_BYTES)).filter(|bytes| *bytes <= ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)",
    "pub const CONFIG_EDIT_RETAINED_BASELINE_BYTES: usize = 131072;",
    "if self.cancelled || self.custody.is_closing() || grant.maximum_items == 0 || grant.maximum_depth < 2",
    "rows.try_reserve_exact(1)",
    'if actual > grant.maximum_capacity_bytes { self.cancelled = true; return Err(refusal("config apply inverse row allocation exceeded its admitted capacity")); }',
    'Ok(inverse) => fields.1.as_mut().expect("validated inverse row capacity").push(inverse)',
    "Err((error, owned)) => { fields.0 = Some(owned); self.cancelled = true; return Err(error); }",
    "fn take_inverse(&mut self) -> Option<Vec<M>> { (self.stage == ApplyStage::Done)",
    "fn terminal_is_empty(&self) -> bool { self.clone.is_none() && self.custody.terminal_is_empty() }",
    '"config apply cursor requires complete admitted closure"',
    "request.operation == request.authority.operation() && request.generation == request.authority.generation() && request.base_revision == request.authority.base_revision() && request.authority.actor().len() <= ARTIFACT_STORE_ONE_ITEM_ID_BYTES",
    "if lane != HistoryLane::Document || !E::recognizes(mutation)",
    "if retained_bytes > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES",
    "if !grant.permits_one() || self.cancelled || self.closing",
    "self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);",
    "authority.begin_one_item_seal(edit, Arc::new(post), mutation_retirement, snapshot_retirement, retained)",
  ];
  const exact = (candidate: string) => witnesses.every((witness) => candidate.includes(witness));
  if (!exact(preparation)) throw new Error("scalar Config shared preparation contract");
  for (const witness of witnesses) {
    if (exact(preparation.replaceAll(witness, "REMOVED_BY_HOSTILE_FIXTURE"))) throw new Error(`scalar Config hostile preparation witness ${witness}`);
    hostileCases++;
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
  return { routes, migrated, batchOnly, forbidden, mutationOracles: fixture.mutations.length, hostileCases };
}
