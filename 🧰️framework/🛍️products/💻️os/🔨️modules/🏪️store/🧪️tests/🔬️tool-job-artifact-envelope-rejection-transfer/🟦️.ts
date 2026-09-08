import { policyMutateRustSourceEvidence, toolJobArtifactEnvelopeRejectionTransferExact, type PolicyRustSourceEvidence } from "../../../../../../../📜️script.ts";

/** 🧪️ Executes tool job artifact envelope rejection transfer policy assertions. */
export function toolJobArtifactEnvelopeRejectionTransferSelfTests(store: PolicyRustSourceEvidence): number {
  const exact = (source: PolicyRustSourceEvidence) => toolJobArtifactEnvelopeRejectionTransferExact(source);
  const mutate = (source: PolicyRustSourceEvidence, from: string, to: string) => policyMutateRustSourceEvidence(source, from, to);
  const mutateLast = (source: PolicyRustSourceEvidence, from: string, to: string) => policyMutateRustSourceEvidence(source, from, to, true);
  if (!exact(store)) throw new Error("[verify interactivity tool-jobs p2a1] valid Store rejection transfer was rejected.");
  const mutations: [string, PolicyRustSourceEvidence][] = [
    ["source record deep drop", mutate(store, "record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>", "record: Option<OwnedSchemaRecordCursor>")],
    ["source lease deep drop", mutate(store, "fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>", "fields: Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>")],
    ["invalid rejection owner loss", mutate(store, "return Err(self);\n        }\n        let mut source = std::mem::ManuallyDrop::new(self);", "panic!(\"invalid rejection\");\n        }\n        let mut source = std::mem::ManuallyDrop::new(self);")],
    ["implicit source transfer", mutate(store, "let mut source = std::mem::ManuallyDrop::new(self);", "let mut source = self;")],
    ["source Drop after transfer", mutate(store, "source.state = ArtifactEnvelopeDecodeState::Transferred;", "source.state = ArtifactEnvelopeDecodeState::Fault(diagnostic);")],
    ["transferred Drop branch removal", mutate(store, "if matches!(self.state, ArtifactEnvelopeDecodeState::Transferred) {", "if false {")],
    ["premature ticket reclamation", mutateLast(store, "if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {", "if !self.field_returned {")],
    ["rejected double return ignored", mutateLast(store, "if !fields.return_now() {", "let _ = fields.return_now();\n            if false {")],
    ["raw rejected record drop", mutate(store, "return match record.close_step(1) {", "drop(self.record.take());\n            return Ok(SnapshotRetirementStep::Complete);\n            match record.close_step(1) {")],
    ["public rejection identity proof", mutate(store, "Some(record_identity));", "None);")],
    ["success ownership law", mutate(store, "artifact_envelope_decode_withholds_success_until_field_and_page_owners_are_terminal_empty", "artifact_envelope_decode_success_smoke")],
    ["cancel ownership law", mutate(store, "cancelled_and_rejected_envelope_decodes_close_one_exact_owner_per_grant", "cancelled_envelope_decode_smoke")],
    ["generation reuse law", mutate(store, "envelope_field_registry_retains_late_return_and_generation_reuse_until_bounded_app_reclaim", "envelope_field_registry_reuse_smoke")],
  ];
  for (const [name, source] of mutations) if (exact(source)) throw new Error(`[verify interactivity tool-jobs p2a1] mutation ${name} was falsely accepted.`);
  return mutations.length;
}
