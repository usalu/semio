/** 🔒️ The production source closure checks of the invite redemption transaction proof. */
import { readHubDirectorySource } from "../../../📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts";
import { readDirectoryBackendSource, readHubBinSource, readNeo4jDirectorySource } from "../../../🧪️testing/📖️source-readers/🟦️.ts";

/** 🧱 Reads the directory backend sources and checks the redemption transaction closure with its hostile mutations. */
export function checkRedemptionSourceClosure({ repoRoot }: {
  readonly repoRoot: string;
}) {
  const directory = readHubDirectorySource(repoRoot);
  const sqlite = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const postgres = readDirectoryBackendSource(repoRoot, "🐘️postgres");
  const neo4j = readNeo4jDirectorySource(repoRoot);
  const body = (source: string, signature: string): string => {
    const offset = source.indexOf(signature);
    const start = source.indexOf("{", offset);
    let depth = 0;
    for (let index = start; offset >= 0 && index < source.length; index += 1) {
      if (source[index] === "{") depth += 1;
      else if (source[index] === "}" && --depth === 0) return source.slice(start + 1, index);
    }
    return "";
  };
  const ordered = (source: string, values: readonly string[]): boolean => values.every((value, index) => source.indexOf(value) >= 0 && (index === 0 || source.indexOf(values[index - 1]!) < source.indexOf(value)));
  const sourceClosed = (shared: string, sq: string, pg: string, neo: string): boolean => {
    const service = body(shared, "pub async fn redeem_invite(");
    const sqliteClaim = body(sq, "async fn redeem_invite_atomic(");
    const postgresClaim = body(pg, "async fn redeem_invite_atomic(");
    const neoClaim = body(neo, "async fn redeem_invite_atomic(");
    return (
      shared.includes("pub accepted_event_id: Option<String>") &&
      shared.includes("InviteRedemptionCommit::AlreadyCommitted") &&
      ordered(service, ["let mut clock = self.write.lock().await", "let hlc = clock.tick()", "self.dir.redeem_invite_atomic", "if let InviteRedemptionCommit::NewlyCommitted { event } = &committed", "publish_persisted_locked", "Ok(committed)"]) &&
      !service.includes("InviteRedemptionCommit::AlreadyCommitted") &&
      sq.includes("CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))") &&
      sqliteClaim.includes("TransactionBehavior::Immediate") &&
      ordered(sqliteClaim.slice(sqliteClaim.indexOf("SET accepted_at = ?2")), ["SET accepted_at = ?2, accepted_event_id = ?3", "persist_event_with_identity", "self.project(&tx", "tx.commit()"]) &&
      pg.includes("CHECK ((accepted_at IS NULL) = (accepted_event_id IS NULL))") &&
      postgresClaim.includes("WHERE singleton FOR UPDATE") &&
      postgresClaim.includes("WHERE selector = $1 FOR UPDATE") &&
      ordered(postgresClaim.slice(postgresClaim.indexOf("SET accepted_at = $2")), ["SET accepted_at = $2, accepted_event_id = $3", "INSERT INTO hub_directory_event", "self.project(&mut tx", "tx.commit()"]) &&
      pg.includes("CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP") &&
      pg.includes("accepted_at, accepted_event_id FROM hub_rebuild_space_invite") &&
      neoClaim.includes("SET c.claimNonce") &&
      neoClaim.includes("SET i.claimNonce") &&
      ordered(neoClaim.slice(neoClaim.indexOf("SET i.acceptedAt = $accepted_at")), ["SET i.acceptedAt = $accepted_at, i.acceptedEventId = $event_id", "CREATE (e:DirectoryEvent", "self.project(&mut txn", "txn.commit()"]) &&
      [sq, pg, neo].every((source) => source.includes("DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. }")) &&
      [sq, pg, neo].every((source) => source.includes("accepted_at") && source.includes("accepted_event_id"))
    );
  };
  if (!sourceClosed(directory, sqlite, postgres, neo4j)) throw new Error("invite redemption production transaction fence is incomplete");
  const sourceHostiles = [
    [directory.replace("pub accepted_event_id: Option<String>", ""), sqlite, postgres, neo4j],
    [directory.replace("if let InviteRedemptionCommit::NewlyCommitted { event } = &committed", "if let InviteRedemptionCommit::AlreadyCommitted { event } = &committed"), sqlite, postgres, neo4j],
    [
      directory,
      sqlite.replace(
        "transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;\n        let accepted_at_ms",
        "transaction_with_behavior(rusqlite::TransactionBehavior::Deferred).map_err(backend)?;\n        let accepted_at_ms",
      ),
      postgres,
      neo4j,
    ],
    [directory, sqlite, postgres.replaceAll("FOR UPDATE", ""), neo4j],
    [directory, sqlite, postgres.replace("CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP", ""), neo4j],
    [directory, sqlite, postgres, neo4j.replaceAll("claimNonce", "claimRead")],
  ];
  sourceHostiles.forEach((hostile, index) => {
    if (sourceClosed(hostile[0]!, hostile[1]!, hostile[2]!, hostile[3]!)) throw new Error(`invite redemption source oracle admitted removed transaction fence ${index}`);
  });
  return { body, ordered, directory, sqlite, postgres, neo4j, sourceHostiles };
}

/** 🏛 Checks the invite state and ingress authority closure with its hostile mutations. */
export function checkStateAuthorityClosure({ repoRoot, body, ordered, directory, sqlite, postgres, neo4j }: {
  readonly repoRoot: string;
  readonly body: (source: string, signature: string) => string;
  readonly ordered: (source: string, values: readonly string[]) => boolean;
  readonly directory: string;
  readonly sqlite: string;
  readonly postgres: string;
  readonly neo4j: string;
}) {
  const hub = readHubBinSource(repoRoot);
  const stateAuthorityClosed = (shared: string, sq: string, pg: string, neo: string, server: string): boolean => {
    const preflight = body(shared, "pub(crate) fn invite_redemption_preflight(");
    const hint = body(shared, "pub(crate) fn verify_invite_redemption_scope_hint(");
    const route = body(server, "async fn post_redeem_invite(");
    return (
      ordered(preflight, ["space_state == InviteRedemptionSpaceStateV1::Missing", "record.accepted_at.is_some()", "space_state == InviteRedemptionSpaceStateV1::Archived && record.role == SpaceRole::Author"]) &&
      hint.includes("actor_user_id(actor).ok() != Some(user_id)") &&
      hint.includes("!constant_time_digest_eq(&record.secret_digest, &capability.secret_digest())") &&
      body(sq, "async fn redeem_invite_atomic(").includes("SELECT kind FROM hub_space WHERE id = ?1") &&
      body(pg, "async fn redeem_invite_atomic(").includes("SELECT kind FROM hub_space WHERE id = $1") &&
      body(neo, "async fn redeem_invite_atomic(").includes("RETURN s.kind AS kind") &&
      [sq, pg, neo].every((source) => body(source, "async fn redeem_invite_atomic(").includes("InviteRedemptionSpaceStateV1::from_kind")) &&
      [sq, pg, neo].every((source) => body(source, "async fn invite_redemption_scope_hint(").includes("verify_invite_redemption_scope_hint(record.as_ref(), capability, actor, user_id)")) &&
      ordered(route, [
        "state.directory.invite_redemption_scope_hint",
        "false).await",
        "state.socket_binding_gates.acquire_bindings",
        "revalidate_directory_caller",
        "state.directory_service.redeem_invite",
        "InviteRedemptionCommit::NewlyCommitted",
        "invalidate_directory_event_authority",
        "Ok(DirectoryJson",
      ]) &&
      ["SocketBindingKeyV1::User(user.user_id.clone())", "SocketBindingKeyV1::Session(user.session_id.clone())", "SocketBindingKeyV1::DirectorySpaceAuthority", "SocketBindingKeyV1::Membership"].every((key) => route.includes(key)) &&
      route.split("revalidate_directory_caller(&state, &user)").length === 3 &&
      !route.includes("drop(_authority)") &&
      !route.includes("get_role")
    );
  };
  if (!stateAuthorityClosed(directory, sqlite, postgres, neo4j, hub)) throw new Error("invite state or ingress authority topology is incomplete");
  const route = body(hub, "async fn post_redeem_invite(");
  const stateHostiles = [
    [directory.replace("space_state == InviteRedemptionSpaceStateV1::Archived && record.role == SpaceRole::Author", "false"), sqlite, postgres, neo4j, hub],
    [directory, sqlite.replaceAll("SELECT kind FROM hub_space WHERE id = ?1", "SELECT id FROM hub_space WHERE id = ?1"), postgres, neo4j, hub],
    [directory, sqlite, postgres.replaceAll("SELECT kind FROM hub_space WHERE id = $1", "SELECT id FROM hub_space WHERE id = $1"), neo4j, hub],
    [directory, sqlite, postgres, neo4j.replaceAll("RETURN s.kind AS kind", "RETURN s.id AS kind"), hub],
    [directory.replace("!constant_time_digest_eq(&record.secret_digest, &capability.secret_digest())", "false"), sqlite, postgres, neo4j, hub],
    [directory, sqlite, postgres, neo4j, hub.replace(route, route.replace("SocketBindingKeyV1::User(user.user_id.clone())", "SocketBindingKeyV1::User(String::new())"))],
  ];
  for (const [index, candidate] of stateHostiles.entries()) {
    if (stateAuthorityClosed(candidate[0]!, candidate[1]!, candidate[2]!, candidate[3]!, candidate[4]!)) throw new Error(`invite state authority oracle admitted removed fence ${index}`);
  }
  return { stateHostiles };
}

/** 🗄 Checks the archive transaction and rollback closure with its hostile mutations. */
export function checkArchiveClosure({ body, ordered, directory, sqlite, postgres, neo4j }: {
  readonly body: (source: string, signature: string) => string;
  readonly ordered: (source: string, values: readonly string[]) => boolean;
  readonly directory: string;
  readonly sqlite: string;
  readonly postgres: string;
  readonly neo4j: string;
}) {
  const archiveClosed = (shared: string, sq: string, pg: string, neo: string): boolean => {
    const execute = body(shared, "pub async fn execute_idempotent(");
    const appendTail = execute.slice(execute.indexOf("self.dir.append_decided_events"));
    const invariant = body(shared, "pub(crate) fn directory_projection_rejection_v1(");
    const decider = shared.slice(shared.indexOf("DirectoryCommand::ArchiveSpace { space_id } =>"), shared.indexOf("DirectoryCommand::DeleteSpace { space_id } =>"));
    return (
      !decider.includes("list_members") &&
      decider.includes("Ok(single(") &&
      invariant.includes("space_kind.is_none()") &&
      invariant.includes('Some("archive")') &&
      invariant.includes("DirectoryEventBody::MemberUpserted") &&
      invariant.includes("DirectoryEventBody::InviteRedeemed") &&
      ordered(appendTail, ["self.dir.append_decided_events", "await?", "DirectoryAppendOutcomeV1::RejectedBeforeCommit", "release_directory_command_receipt(&claim.actor_user_id, &claim.request_id, &claim.command_sha256)"]) &&
      [sq, pg, neo].every((source) => {
        const append = body(source, "async fn append_decided_events(");
        const release = body(source, "async fn release_directory_command_receipt(");
        const project = body(source, source === sq ? "fn project(" : "async fn project(");
        return (
          ordered(append, ["projection_rejection", ".rollback()", "RejectedBeforeCommit", ".commit()"]) &&
          project.includes("projection_rejection") &&
          project.includes("'spectator'") &&
          project.includes("'author'") &&
          release.includes("command_sha256") &&
          release.includes("pending") &&
          release.includes("changed != 1")
        );
      })
    );
  };
  if (!archiveClosed(directory, sqlite, postgres, neo4j)) throw new Error("archive transaction or rollback-confirmed receipt authority is incomplete");
  const archiveHostiles = [
    [directory.replace('space_kind == Some("archive")', "false"), sqlite, postgres, neo4j],
    [directory, sqlite.replaceAll("DirectoryAppendOutcomeV1::RejectedBeforeCommit", "DirectoryAppendOutcomeV1::Appended"), postgres, neo4j],
    [directory, sqlite, postgres.replaceAll(".rollback().await", ".commit().await"), neo4j],
    [directory, sqlite, postgres, neo4j.replaceAll("changed != 1", "changed < 0")],
    [directory.replace("self.dir.append_decided_events(&decision.events).await?", "self.dir.append_decided_events(&decision.events).await.unwrap_or_default()"), sqlite, postgres, neo4j],
  ];
  for (const [index, candidate] of archiveHostiles.entries()) if (archiveClosed(candidate[0]!, candidate[1]!, candidate[2]!, candidate[3]!)) throw new Error(`archive authority oracle admitted removed fence ${index}`);
  return { archiveHostiles };
}
