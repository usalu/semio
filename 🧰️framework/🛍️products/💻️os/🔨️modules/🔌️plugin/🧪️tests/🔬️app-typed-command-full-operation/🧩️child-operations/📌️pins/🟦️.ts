/** 📌️ Child-group laws pins: the recorded pins validate against their schema and equal a fresh generation, and every laws crate keeps its size boundary. */
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import Ajv from"ajv";
import{generate}from"./📜️";
import schema from"./🧬️.json";
import pins from"./📋️.json";
const obligations=["child_complete_group_source_candidate_reader_retains_actual_partial8194_pages","child_complete_group_source_candidate_matches_independent_wire","child_complete_original_semantic_metadata8194_grant4096","child_complete_group_source_candidate_paged_semantic8194_metadata_and_labels","child_complete_group_source_candidate_direct_producer_and_parent","child_complete_group_source_candidate_repeated_prefix_preserves_accepted_owners","child_complete_group_source_candidate_refuses_partial_last_locale_before_commit","child_complete_actual_authored_label_producer_preserves_all_axes_and_retained_source","child_complete_actual_authored_label_receipt_joins_original8194_source_and_prefix","child_complete_group_source_candidate_owned_publication_preserves_empty_original_and_parent","child_complete_group_source_candidate_accepted_ledger_retains_original_registry_sources","child_complete_actual_authored_operation_producer_joins_original_projection_and_retained_receipt","child_complete_actual_authored_typed_source_event_observer_matches_system_backing","child_complete_actual_authored_typed_source_physical8194_grant4096","child_complete_actual_authored_paged_source8194_frame8202_physical4096","child_complete_candidate_retirement_keeps_currencies_independent"];
const root=resolve(import.meta.dir,"../../../../../../../../..");
test("recorded child-group pins equal a fresh generation and fit the source boundary",async()=>{
 expect(new Ajv({strict:true}).validate(schema,pins)).toBe(true);
 expect(await generate()).toEqual(pins);
 expect(pins.crates.flatMap(crate=>crate.laws.map(law=>law.name)).sort()).toEqual([...obligations].sort());
 expect(pins.activeInstances).toBe(pins.uniqueObligations);
 expect(pins.remaining.some(file=>file.path===pins.host)).toBe(true);
 for(const crate of pins.crates){const texts=crate.closure.map(file=>readFileSync(resolve(root,file.path),"utf8"));expect(texts.length).toBeLessThanOrEqual(32);expect(texts.reduce((n,s)=>n+Buffer.byteLength(s),0)).toBeLessThanOrEqual(262144);expect(texts.reduce((n,s)=>n+s.split("\n").filter(l=>l.trim()).length,0)).toBeLessThanOrEqual(4000);}
});
