import { readFileSync } from "node:fs";
import { join } from "node:path";
import { sourceDefinitionBodies } from "../../../../../🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";

export type SocketGrantNativeStage = Readonly<{ id: string; args: readonly string[]; source: string | null; declaration: string | null }>;

const hubBinarySource = "🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs";
const hubBinaryLaws = [
  "scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership",
  "scoped_directory_socket_message_matching_is_body_exact_and_removal_private",
  "scoped_directory_socket_admin_removal_uses_the_same_membership_fence",
  "scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event",
  "scoped_directory_socket_removal_and_delivery_have_one_total_membership_order",
  "socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke",
  "socket_directory_revoke_after_admission_suppresses_replay_without_deadlock",
  "socket_directory_visibility_requires_membership_even_for_public_spaces",
  "socket_grant_directory_route_uses_credential_free_hello_and_revokes_live",
  "socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live",
  "socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe",
  "socket_grant_revoke_and_welcome_have_a_bounded_binding_linearization",
  "socket_grant_revoke_before_broadcast_authorization_suppresses_frame",
  "socket_grant_revoke_before_command_admission_has_no_storage_effect",
  "socket_grant_revoke_before_lag_authorization_reads_no_private_control",
] as const;

/** 📋️ Returns the exact current socket-grant native law and build-check sequence. */
export function socketGrantNativeLawPlan(): readonly SocketGrantNativeStage[] {
  const stages: SocketGrantNativeStage[] = hubBinaryLaws.map((declaration) => ({
    id: declaration.replaceAll("_", "-"),
    args: ["test", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub", `tests::${declaration}`, "--", "--exact", "--test-threads=1"],
    source: hubBinarySource,
    declaration,
  }));
  stages.push(
    {
      id: "typed-capabilities",
      args: ["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries", "--", "--test-threads=1"],
      source: "🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs",
      declaration: "typed_capabilities_match_neutral_sha256_vectors_and_fixed_boundaries",
    },
    {
      id: "socket-binding-reads",
      args: ["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "socket_binding_reads_are_exact_id_generation_selector_scope_and_status", "--", "--test-threads=1"],
      source: "🌎️hub/📇️directory/🪶️sqlite/🧪️tests/🔬️unit/🦀️.rs",
      declaration: "socket_binding_reads_are_exact_id_generation_selector_scope_and_status",
    },
    {
      id: "client-frame-socket-hello",
      args: ["test", "--manifest-path", "Cargo.toml", "--all-features", "-p", "semio-framework-replication", "client_frame_socket_hello_v1_round_trips_without_credentials", "--", "--test-threads=1"],
      source: "🧰️framework/🔨️modules/📡️replication/📡️wire/🧪️tests/🔬️unit/🦀️.rs",
      declaration: "client_frame_socket_hello_v1_round_trips_without_credentials",
    },
    { id: "hub-binary-check", args: ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], source: null, declaration: null },
  );
  return stages;
}

/** 🔎 Rejects stale, duplicate, or source-less socket-grant native law selectors. */
export function assertSocketGrantNativeLawSources(repoRoot: string, readSource: (path: string) => string = (path) => readFileSync(path, "utf8")): void {
  const stages = socketGrantNativeLawPlan();
  if (stages.length !== 19 || new Set(stages.map((stage) => stage.id)).size !== stages.length) throw new Error("socket-grant native stage inventory drift");
  for (const stage of stages) {
    if ((stage.source === null) !== (stage.declaration === null)) throw new Error(`socket-grant native stage source binding drift: ${stage.id}`);
    if (!stage.source || !stage.declaration) continue;
    const escaped = stage.declaration.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
    const bodies = sourceDefinitionBodies(readSource(join(repoRoot, stage.source)), new RegExp(`\\b(?:async\\s+)?fn\\s+${escaped}\\s*\\([^)]*\\)\\s*(?:->[^\\{]+)?`, "gu"));
    if (bodies.length !== 1) throw new Error(`socket-grant native law must have one current declaration: ${stage.declaration}`);
  }
}
