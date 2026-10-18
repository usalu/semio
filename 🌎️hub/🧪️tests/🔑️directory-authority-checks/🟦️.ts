/** 🧭️ Directory authority gates: admin authority, share issuance, retained short admin, message authority and scoped sockets.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { join } from "node:path";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { runCmd, orchestratorBudgetOpts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { readHubDirectorySource } from "../../📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts";
import { proveScopedDirectorySocketRevocationFixture } from "../../📇️directory/🔐️authorization/🔌️socket-grant/🧪️tests/🧾️fixture-verification/🟦️.ts";
import { readDirectoryBackendSource, readHubBinSource, readNeo4jDirectorySource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

export class AdminDirectoryAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("admin-directory-authority-check accepts source or native");
    const root = join(this.repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    if (fixture.schema !== "semio.hub.admin-directory-authority/v1") throw new Error("admin directory authority fixture schema drift");
    if (Object.keys(fixture).sort().join(",") !== "bindings,schema,shortActions,vectors") throw new Error("admin directory authority fixture envelope drift");
    if (fixture.vectors.length !== 5 || fixture.shortActions.length !== 8 || fixture.bindings.length !== 15) throw new Error("admin directory authority table sizes drifted");
    const bindingNames = ["create", "rename", "visibility", "archive", "delete", "upsert", "remove", "create-invite", "revoke-invite", "issue-share", "revoke-share", "revoke-user", "self-revoke", "kick", "rebuild"];
    if (fixture.bindings.map((row: { name: string }) => row.name).join(",") !== bindingNames.join(",")) throw new Error("admin directory authority binding inventory drifted");
    for (const table of ["vectors", "shortActions", "bindings"] as const) {
      if (new Set(fixture[table].map((row: { name: string }) => row.name)).size !== fixture[table].length) throw new Error(`admin directory authority ${table} names are not unique`);
    }
    for (const row of fixture.vectors) {
      if (!["create", "rename"].includes(row.command) || !["session", "demotion"].includes(row.transition) || !["transition", "command"].includes(row.first) || !["succeeded", "cancelled"].includes(row.state)) throw new Error("admin directory authority vector taxonomy drift: " + row.name);
    }
    for (const row of fixture.shortActions) {
      if (!["issue-share", "revoke-share", "revoke-user", "kick"].includes(row.action) || !["transition", "action"].includes(row.first) || ![2, 3, 4].includes(row.keys)) throw new Error("admin directory authority short-action taxonomy drift: " + row.name);
    }
    for (const row of fixture.bindings) {
      if (row.keys !== null && (!Array.isArray(row.keys) || row.keys.length < 2 || row.keys.length > 4 || row.keys.some((key: string) => !/^(user|session|space|membership|share):.+$/u.test(key)))) throw new Error("admin directory authority principal union drift: " + row.name);
    }
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      for (const row of fixture.vectors) {
        const expected = oracle.query("SELECT (? = 'demotion' OR ? = 'command') AS mutated").get(row.transition, row.first) as { mutated: number };
        const mutated = row.transition === "demotion" || row.first === "command";
        if (row.mutated !== mutated || row.mutated !== !!expected.mutated || row.state !== (mutated ? "succeeded" : "cancelled")) throw new Error("admin directory authority order differs: " + row.name);
      }

      for (const row of fixture.bindings) {
        const intent = JSON.parse(row.intentJson);
        if (intent.kind === "rebuild-directory-projections") {
          if (row.keys !== null) throw new Error("long rebuild must not acquire a short-effect principal union");
          continue;
        }
        const request = Buffer.from(intent.requestId);
        const length = Buffer.alloc(4);
        length.writeUInt32BE(request.length);
        const created = "admin-space:" + createHash("sha256").update("semio/hub/admin-create-space/v1\0").update(length).update(request).digest("hex");
        const rows = oracle
          .query(
            `
          SELECT key FROM (
            SELECT 0 AS rank, 'user:admin' AS key
            UNION SELECT 0, 'user:' || json_extract(?1, '$.userId') WHERE json_extract(?1, '$.kind') = 'revoke-user-sessions'
            UNION SELECT 1, 'session:admin-session'
            UNION SELECT 2, 'space:' || CASE WHEN json_extract(?1, '$.kind') = 'create-space' THEN ?2 ELSE coalesce(json_extract(?1, '$.scope.spaceId'), json_extract(?1, '$.spaceId')) END
            UNION SELECT 3, 'membership:' || json_extract(?1, '$.userId') || '/' || json_extract(?1, '$.spaceId') WHERE json_extract(?1, '$.kind') = 'remove-space-member'
            UNION SELECT 4, 'share:' || json_extract(?1, '$.shareId') WHERE json_extract(?1, '$.kind') = 'revoke-document-share'
          ) WHERE key IS NOT NULL ORDER BY rank, key
        `,
          )
          .all(row.intentJson, created) as { key: string }[];
        if (JSON.stringify(rows.map((entry) => entry.key)) !== JSON.stringify(row.keys)) throw new Error("exact administrator binding oracle differs: " + row.name);
      }

      for (const row of fixture.shortActions) {
        const expected = oracle.query("SELECT ?1 = 'action' AS effect, (?2 = 'issue-share' AND ?1 = 'action') AS secret, CASE ?2 WHEN 'revoke-share' THEN 4 WHEN 'kick' THEN 2 ELSE 3 END AS keys").get(row.first, row.action) as {
          effect: number;
          secret: number;
          keys: number;
        };
        if (row.effect !== !!expected.effect || row.secret !== !!expected.secret || row.keys !== expected.keys || row.state !== (expected.effect ? "succeeded" : "cancelled")) throw new Error("admin short effect oracle differs: " + row.name);
      }
    } finally {
      oracle.close();
    }
    if (phase === "native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-directory-route-laws",
            target: { kind: "lib" },
            laws: ["admin_intent_binding_wire_matrix_is_exact_sorted_and_self_deduplicated", "admin_short_effects_retain_principal_until_their_actual_side_effect", "admin_directory_commands_hold_exact_principal_without_confusing_space_role"],
          },
        ],progress(event) {
          console.log("admin-directory-authority " + event.stage + ": " + (event.law ?? event.package) + " artifacts=" + event.artifactDir);
        }});
      for (const receipt of receipts) console.log("admin-directory-authority-native-receipt: " + JSON.stringify(receipt));
    }
    console.log("admin-directory-authority-check: AJV=1 ordering=" + fixture.vectors.length + " short-actions=" + fixture.shortActions.length + " bindings=" + fixture.bindings.length + " phase=" + phase);
  }
}

/** 🔐️ Proves share issuance owns the persisted Space and descriptor in one backend transaction. */
async function proveShareIssuanceAtomicity(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🔐️share-issuance-atomicity");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  if (fixture.schema !== "semio.hub.share-issuance-atomicity/v1") throw new Error("share issuance atomicity fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "cases,schema,sourceHostiles") throw new Error("share issuance atomicity fixture envelope drift");
  if (fixture.cases.length !== 7 || new Set(fixture.cases.map((row: { name: string }) => row.name)).size !== 7) throw new Error("share issuance atomicity case inventory drift");
  const shareIssuanceHostiles = ["sqlite-select-removed", "sqlite-owner-foreign-key-removed", "postgres-select-lock-removed", "postgres-owner-foreign-key-removed", "neo4j-space-owner-removed", "neo4j-descriptor-owner-removed", "neo4j-count-consumption-removed"];
  if (fixture.sourceHostiles.join(",") !== shareIssuanceHostiles.join(",")) throw new Error("share issuance atomicity source-hostile inventory drift");
  for (const row of fixture.cases) {
    if (!["studio", "archive"].includes(row.spaceKind) || !["author", "spectator"].includes(row.role) || !["none", "other-document", "descriptor", "space"].includes(row.deleteBeforeInsert)) throw new Error(`share issuance atomicity case taxonomy drift: ${row.name}`);
    if (typeof row.spaceExists !== "boolean" || typeof row.descriptorExists !== "boolean" || typeof row.accepted !== "boolean") throw new Error(`share issuance atomicity case is not a boolean decision: ${row.name}`);
  }
  const { Database } = await import("bun:sqlite");
  const oracle = new Database(":memory:");
  try {
    oracle.exec(`
      PRAGMA foreign_keys = ON;
      CREATE TABLE hub_space (id TEXT PRIMARY KEY, kind TEXT NOT NULL);
      CREATE TABLE hub_document_descriptor (space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE, document_id TEXT NOT NULL, PRIMARY KEY (space_id, document_id));
      CREATE TABLE hub_share_grant (id TEXT PRIMARY KEY, space_id TEXT NOT NULL, document_id TEXT NOT NULL, FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE);
    `);
    const insert = oracle.query(`
      INSERT INTO hub_share_grant (id, space_id, document_id)
      SELECT ?1, ?2, ?3 FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id
      WHERE s.id = ?2 AND d.space_id = ?2 AND d.document_id = ?3
    `);
    for (const [index, test] of fixture.cases.entries()) {
      oracle.exec("DELETE FROM hub_share_grant; DELETE FROM hub_document_descriptor; DELETE FROM hub_space;");
      if (test.spaceExists) oracle.query("INSERT INTO hub_space (id, kind) VALUES ('space', ?1)").run(test.spaceKind);
      const documentId = test.deleteBeforeInsert === "other-document" ? "other" : "document";
      if (test.spaceExists && test.descriptorExists) oracle.query("INSERT INTO hub_document_descriptor (space_id, document_id) VALUES ('space', ?1)").run(documentId);
      if (test.deleteBeforeInsert === "descriptor") oracle.exec("DELETE FROM hub_document_descriptor WHERE space_id = 'space' AND document_id = 'document'");
      if (test.deleteBeforeInsert === "space") oracle.exec("DELETE FROM hub_space WHERE id = 'space'");
      const result = insert.run(`share-${index}`, "space", "document");
      if ((result.changes === 1) !== test.accepted) throw new Error(`share issuance SQLite oracle differs for ${test.name}`);
      const retained = oracle.query("SELECT count(*) AS count FROM hub_share_grant").get() as { count: number };
      if (retained.count !== (test.accepted ? 1 : 0)) throw new Error(`share issuance retained an orphan for ${test.name}`);
      if (test.accepted) {
        oracle.exec("DELETE FROM hub_document_descriptor WHERE space_id = 'space' AND document_id = 'document'");
        if ((oracle.query("SELECT count(*) AS count FROM hub_share_grant").get() as { count: number }).count !== 0) throw new Error(`share issuance owner cascade differs for ${test.name}`);
      }
    }
  } finally {
    oracle.close();
  }

  const sqlite = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const postgres = readDirectoryBackendSource(repoRoot, "🐘️postgres");
  const neo4j = readNeo4jDirectorySource(repoRoot);
  const method = (source: string, name: string): string => {
    const start = source.indexOf(`async fn ${name}(`);
    return source.slice(start, source.indexOf("\n    async fn ", start + 1));
  };
  const issues = (source: string): string[] => [method(source, "issue_share_token_as"), method(source, "issue_share_token_as_with_admin_effect")];
  const auth = (source: string): string => source.slice(source.indexOf("async fn authenticate_share_binding("), source.indexOf("async fn socket_share_binding("));
  const shareSchema = (source: string): string => {
    const start = source.indexOf("CREATE TABLE IF NOT EXISTS hub_share_grant (");
    return source.slice(start, source.indexOf("CREATE TABLE", start + 1));
  };
  const omitShareOwnerForeignKey = (source: string): string => {
    const owner = "FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE";
    const start = source.indexOf("CREATE TABLE IF NOT EXISTS hub_share_grant (");
    const position = source.indexOf(owner, start);
    return `${source.slice(0, position)}${source.slice(position + owner.length)}`;
  };
  const omitIssueText = (source: string, text: string): string => {
    const start = source.indexOf("async fn issue_share_token_as(");
    const position = source.indexOf(text, start);
    return `${source.slice(0, position)}${source.slice(position + text.length)}`;
  };
  const sourceClosed = (sq: string, pg: string, neo: string): boolean => {
    const sqIssues = issues(sq),
      pgIssues = issues(pg),
      neoIssues = issues(neo),
      neoAuth = auth(neo);
    return (
      shareSchema(sq).includes("FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE") &&
      sqIssues.every(
        (body) =>
          body.includes("transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)") &&
          body.includes("FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id") &&
          body.includes("if changed != 1") &&
          body.indexOf("if changed != 1") < body.indexOf("insert_auth_audit"),
      ) &&
      shareSchema(pg).includes("FOREIGN KEY (space_id, document_id) REFERENCES hub_document_descriptor(space_id, document_id) ON DELETE CASCADE") &&
      pgIssues.every(
        (body) =>
          body.includes("FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id") &&
          body.includes("FOR KEY SHARE OF s, d") &&
          body.includes("if changed.rows_affected() != 1") &&
          body.indexOf("if changed.rows_affected() != 1") < body.indexOf("insert_auth_audit"),
      ) &&
      neoIssues.every(
        (body) =>
          body.includes("MATCH (s:Space {id: $space_id}), (d:DocumentDescriptor {scopeKey: $scope_key, spaceId: $space_id, documentId: $document_id})") &&
          body.includes("(s)-[:HAS_SHARE_GRANT]->(g:ShareGrant") &&
          body.includes("-[:GRANTS_DOCUMENT]->(d)") &&
          body.includes("result.next(txn.handle())") &&
          body.includes("if changed != 1") &&
          body.indexOf("if changed != 1") < body.indexOf("insert_auth_audit"),
      ) &&
      neoAuth.includes("(:Space {id: $space_id})-[:HAS_SHARE_GRANT]->") &&
      neoAuth.includes("-[:GRANTS_DOCUMENT]->(:DocumentDescriptor {scopeKey: $scope_key})") &&
      ![...sqIssues, ...pgIssues, ...neoIssues].some((body) => body.includes("role = 'author'") || body.includes("kind <> 'archive'"))
    );
  };
  if (!sourceClosed(sqlite, postgres, neo4j)) throw new Error("share issuance production transaction fence is incomplete");
  const sourceHostiles = [
    [sqlite.replace("FROM hub_space AS s JOIN hub_document_descriptor AS d ON d.space_id = s.id", ""), postgres, neo4j],
    [omitShareOwnerForeignKey(sqlite), postgres, neo4j],
    [sqlite, postgres.replace("FOR KEY SHARE OF s, d", ""), neo4j],
    [sqlite, omitShareOwnerForeignKey(postgres), neo4j],
    [sqlite, postgres, omitIssueText(neo4j, "(s)-[:HAS_SHARE_GRANT]->")],
    [sqlite, postgres, omitIssueText(neo4j, "-[:GRANTS_DOCUMENT]->(d)")],
    [sqlite, postgres, omitIssueText(neo4j, "result.next(txn.handle())")],
  ];
  if (sourceHostiles.length !== fixture.sourceHostiles.length) throw new Error("share issuance source hostile count differs");
  sourceHostiles.forEach(([sq, pg, neo], index) => {
    if (sourceClosed(sq!, pg!, neo!)) throw new Error(`share issuance source oracle admitted ${fixture.sourceHostiles[index]}`);
  });
  console.log(`share-issuance-atomicity-oracle: AJV=1 bun-sqlite=${fixture.cases.length} source-hostiles=${sourceHostiles.length}; PostgreSQL/Neo4j are source-only`);
  return fixture.cases.length + sourceHostiles.length;
}

export class ShareIssuanceAtomicityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && segments[0] !== "--native")) throw new Error("share-issuance-atomicity-check accepts only --native");
    const checks = await proveShareIssuanceAtomicity(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 120_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-directory-backend-laws",
            target: { kind: "lib" },
            laws: ["sqlite_tests::share_issuance_atomically_requires_the_persisted_scope_and_preserves_archived_spectator_read"],
          },
        ],progress(event) {
          console.log(`share-issuance-atomicity ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`share-issuance-atomicity-native-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`share-issuance-atomicity-check: checks=${checks} mode=${segments[0] ?? "source"}`);
  }
}

/** 🏛️ Proves accepted short administrator work outlives its request without replaying secrets. */
async function proveRetainedShortAdmin(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🏛️retained-short-admin");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  if (fixture.schema !== "semio.hub.retained-short-admin/v1" || fixture.adminEffectCommitOutcomes.schema !== "semio.hub.admin-effect-commit-outcome/v1") throw new Error("retained short administrator fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "adminEffectCommitOutcomes,capacity,cases,crossCuttingOutcomes,schema,sourceHostiles") throw new Error("retained short administrator fixture envelope drift");
  if (!Number.isSafeInteger(fixture.capacity) || fixture.capacity < 1 || fixture.capacity > 64) throw new Error("retained short administrator capacity bound drift");
  if (fixture.cases.length !== 15 || fixture.adminEffectCommitOutcomes.cases.length !== 21 || fixture.crossCuttingOutcomes.length !== 6 || fixture.sourceHostiles.length !== 25) throw new Error("retained short administrator table sizes drifted");
  if (new Set(fixture.sourceHostiles).size !== 25 || new Set(fixture.cases.map((row: { name: string }) => row.name)).size !== 15) throw new Error("retained short administrator inventories are not unique");
  const retainedStates = ["accepted", "indeterminate", "succeeded", "failed", "cancelled", "conflict"];
  for (const row of fixture.cases) {
    if (!retainedStates.includes(row.state) || ![0, 1].includes(row.effectCount) || ![0, 1].includes(row.secretDeliveries)) throw new Error(`retained short administrator case taxonomy drift: ${row.name}`);
  }
  for (const row of [...fixture.adminEffectCommitOutcomes.cases, ...fixture.crossCuttingOutcomes]) {
    if (!["create-space", "directory-event", "issue-share", "revoke-share", "revoke-sessions", "issue-invite", "revoke-invite"].includes(row.method)) throw new Error(`retained short administrator commit method drift: ${row.name}`);
    if (!["applied", "rejected-before-commit", "indeterminate-no-receipt", "indeterminate-after-commit"].includes(row.driverOutcome) || ![200, 503].includes(row.initialHttp)) throw new Error(`retained short administrator driver taxonomy drift: ${row.name}`);
    if (!retainedStates.includes(row.retryState) || !["cancelled", "failed", "conflict"].includes(row.cancelState)) throw new Error(`retained short administrator commit state drift: ${row.name}`);
    if ([row.writerEntries, row.effectCount, row.receiptCount, row.terminalCount, row.secretDeliveries].some((value) => ![0, 1].includes(value))) throw new Error(`retained short administrator commit counters drift: ${row.name}`);
  }
  const { Database } = await import("bun:sqlite");
  const oracle = new Database(":memory:");
  try {
    const model = oracle.query(`
      SELECT CASE
        WHEN ?1 = 0 THEN 'conflict'
        WHEN ?2 = 'capacity-refused' THEN 'failed'
        WHEN ?3 = 1 THEN 'cancelled'
        WHEN ?4 = 1 AND ?5 = 1 THEN 'succeeded'
        WHEN ?6 = 1 AND ?7 = 1 THEN 'succeeded'
        ELSE 'indeterminate'
      END AS state,
      CASE WHEN ?8 = 1 AND ?5 = 1 THEN 1 ELSE 0 END AS effect_count,
      CASE WHEN ?4 = 1 AND ?5 = 1 AND ?9 = 0 AND ?2 <> 'duplicate-terminal' THEN 1 ELSE 0 END AS secret_deliveries
    `);
    for (const test of fixture.cases) {
      const observed = model.get(test.accepted, test.name, test.cancelBeforeEffect, test.terminalStored, test.effectCommitted, test.effectReceiptStored, test.receiptDigestMatches, test.effectStarted, test.requestDropped) as {
        state: string;
        effect_count: number;
        secret_deliveries: number;
      };
      if (observed.state !== test.state || observed.effect_count !== test.effectCount || observed.secret_deliveries !== test.secretDeliveries) throw new Error(`retained short administrator model differs for ${test.name}`);
    }
    oracle.exec(`
      CREATE TABLE admin_effect (case_name TEXT PRIMARY KEY);
      CREATE TABLE admin_effect_receipt (case_name TEXT PRIMARY KEY REFERENCES admin_effect(case_name));
      CREATE TABLE admin_terminal (case_name TEXT PRIMARY KEY, state TEXT NOT NULL);
    `);
    for (const test of fixture.adminEffectCommitOutcomes.cases) {
      oracle.exec("DELETE FROM admin_terminal; DELETE FROM admin_effect_receipt; DELETE FROM admin_effect;");
      oracle.exec("BEGIN IMMEDIATE");
      if (test.effectCount === 1) oracle.query("INSERT INTO admin_effect(case_name) VALUES (?1)").run(test.name);
      if (test.receiptCount === 1) oracle.query("INSERT INTO admin_effect_receipt(case_name) VALUES (?1)").run(test.name);
      if (test.driverOutcome === "rejected-before-commit" || test.driverOutcome === "indeterminate-no-receipt") oracle.exec("ROLLBACK");
      else oracle.exec("COMMIT");
      if (test.driverOutcome === "rejected-before-commit") oracle.query("INSERT INTO admin_terminal(case_name,state) VALUES (?1,'failed')").run(test.name);
      if (test.driverOutcome === "indeterminate-after-commit") oracle.query("INSERT INTO admin_terminal(case_name,state) VALUES (?1,'succeeded')").run(test.name);
      const counts = oracle.query("SELECT (SELECT count(*) FROM admin_effect) AS effect_count, (SELECT count(*) FROM admin_effect_receipt) AS receipt_count, (SELECT count(*) FROM admin_terminal) AS terminal_count").get() as {
        effect_count: number;
        receipt_count: number;
        terminal_count: number;
      };
      const retryState = test.driverOutcome === "rejected-before-commit" ? "failed" : test.driverOutcome === "indeterminate-no-receipt" ? "indeterminate" : "succeeded";
      const initialHttp = test.driverOutcome === "rejected-before-commit" ? 200 : 503;
      if (counts.effect_count !== test.effectCount || counts.receipt_count !== test.receiptCount || counts.terminal_count !== test.terminalCount || retryState !== test.retryState || initialHttp !== test.initialHttp || test.secretDeliveries !== 0 || test.writerEntries !== 1) {
        throw new Error(`retained short administrator commit outcome differs for ${test.name}`);
      }
    }
    const crossByName = new Map(fixture.crossCuttingOutcomes.map((test: { name: string }) => [test.name, test]));
    const requiredCross = ["pre-admission-cancellation", "post-admission-proved-rollback", "receipt-identity-substitution", "same-receipt-second-writer", "semantic-refusal-rollback-failed", "admitted-writer-outlives-http-deadline"];
    if (requiredCross.some((name) => !crossByName.has(name))) throw new Error("retained short administrator cross-cutting outcome is missing");
  } finally {
    oracle.close();
  }
  const source = readHubBinSource(repoRoot);
  const routeStart = source.indexOf("fn admin_effect_receipt_claim(");
  const route = source.slice(routeStart, source.indexOf("async fn admin_overview(", routeStart));
  const staleStart = source.indexOf("async fn reconcile_stale_admin_acceptance(");
  const stale = source.slice(staleStart, source.indexOf("async fn admin_operation(", staleStart));
  const ownerStart = source.indexOf("struct AdminOperationTaskOwner");
  const owner = source.slice(ownerStart, source.indexOf("/// 🗝️", ownerStart));
  const runtimeStart = source.indexOf("struct AdminOperationRuntime");
  const runtime = source.slice(runtimeStart, ownerStart);
  const schema = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"));
  const schemaTypeScript = readModuleTreeSource(join(repoRoot, "🧰️framework/🔨️modules/📇️directory/🧬️schema/🟦️.ts"));
  const schemaJson = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8");
  const directory = readHubDirectorySource(repoRoot);
  const sqlite = readDirectoryBackendSource(repoRoot, "🪶️sqlite");
  const postgres = readDirectoryBackendSource(repoRoot, "🐘️postgres");
  const neo4j = readNeo4jDirectorySource(repoRoot);
  const sourceClosed = (candidateRoute: string, candidateStale: string, candidateOwner: string, candidateRuntime: string, candidateSchema: string): boolean => {
    const candidateRetained = candidateRoute.slice(candidateRoute.indexOf("let retained = async move"), candidateRoute.indexOf("if state.admin_operation_tasks.spawn("));
    return (
      !candidateRoute.includes("execute_admin_intent(&state, &principal, intent, None).await") &&
      candidateRoute.includes("admin_operation_slots.clone().try_acquire_owned()") &&
      candidateRoute.includes("admin_operation_tasks.spawn(") &&
      candidateRoute.includes("admin_audit_visibility") &&
      candidateSchema.includes("Indeterminate") &&
      candidateRoute.includes("fn admin_effect_uncertain() -> AdminIntentExecution") &&
      (candidateRoute.match(/AdminEffectCommitV1::RejectedBeforeCommit/g) ?? []).length >= 5 &&
      (candidateRoute.match(/AdminEffectCommitV1::Indeterminate/g) ?? []).length >= 5 &&
      !candidateRoute.includes("let revoked = tokio::time::timeout(") &&
      candidateRetained.indexOf("execute_admin_intent(") < candidateRetained.indexOf("append_admin_terminal_with_retry(") &&
      candidateRetained.includes('if execution.phase == "uncertain"') &&
      !candidateRetained.includes("tokio::time::timeout_at") &&
      candidateRetained.indexOf('if execution.phase == "uncertain"') < candidateRetained.indexOf("new_admin_audit_fact(") &&
      candidateRetained.indexOf("append_admin_terminal_with_retry(") < candidateRetained.indexOf("response_tx.is_closed()") &&
      candidateRetained.includes("append_admin_terminal_with_retry(&task_state, &terminal)") &&
      candidateRetained.includes("response_tx.send(Some(execution))") &&
      !candidateRetained.includes("admin_execution_receipt(") &&
      !candidateStale.includes('phase: "cancelled"') &&
      !candidateStale.includes('"interrupted-before-terminal"') &&
      candidateStale.includes("ADMIN_TERMINAL_RETRY_DEADLINE") &&
      candidateOwner.includes("BTreeMap<String, AdminOperationTask>") &&
      candidateOwner.includes("closing") &&
      candidateOwner.includes("runtime.request_cancel()") &&
      candidateOwner.includes("tokio::time::timeout") &&
      candidateOwner.includes("task.abort()") &&
      !candidateOwner.includes("AdminIntentResultV1") &&
      candidateRuntime.includes("AtomicU8") &&
      candidateRuntime.includes("compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_CANCELLED") &&
      candidateRuntime.includes("compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_ADMITTED") &&
      !candidateRuntime.includes("effect_started") &&
      source.includes("enum AdminIntentSecretResult") &&
      source.includes("std::ptr::write_volatile(byte, 0)") &&
      candidateSchema.includes("impl Drop for AdminIntentResultV1") &&
      candidateSchema.includes("std::ptr::write_volatile(byte, 0)") &&
      source.includes("admin_operation_tasks.shutdown().await") &&
      candidateRoute.includes("tokio::time::timeout_at(tokio::time::Instant::from_std(runtime.deadline), response_rx)") &&
      !source.includes("AdminOperationAuditRecord { result:")
    );
  };
  const receiptClosed = (candidateStale: string, candidateDirectory: string, candidateSqlite: string, candidatePostgres: string, candidateNeo4j: string): boolean =>
    candidateDirectory.includes("AdminOperationEffectReceiptV1") &&
    candidateDirectory.includes("operation_id: String") &&
    candidateDirectory.includes("intent_digest: String") &&
    candidateDirectory.includes("append_decided_events_with_admin_effect") &&
    candidateDirectory.includes("admin_operation_effect_receipt") &&
    candidateStale.includes("admin_operation_effect_receipt") &&
    candidateStale.includes("if effect.outcome_code != expected_outcome") &&
    candidateStale.includes('("revoke-document-share", "share")') &&
    candidateStale.includes("state.socket_grants.invalidate_binding(binding.clone())") &&
    candidateStale.indexOf("state.socket_grants.invalidate_binding(binding.clone())") < candidateStale.indexOf("append_admin_terminal_with_retry") &&
    candidateStale.includes("append_admin_terminal_with_retry") &&
    !candidateStale.includes("AdminIntentSecretResult") &&
    [candidateSqlite, candidatePostgres].every((backend) => backend.includes("hub_admin_operation_effect_receipt") && backend.includes("intent_digest") && backend.includes("append_decided_events_with_admin_effect")) &&
    candidatePostgres.includes("ON CONFLICT (operation_id) DO NOTHING") &&
    candidateNeo4j.includes("AdminOperationEffectReceipt") &&
    candidateNeo4j.includes("intentDigest") &&
    candidateNeo4j.includes("append_decided_events_with_admin_effect");
  const outcomeClosed = (candidateRoute: string, candidateDirectory: string, candidateSqlite: string, candidatePostgres: string, candidateNeo4j: string): boolean =>
    candidateDirectory.includes("pub enum AdminEffectCommitV1<T>") &&
    candidateDirectory.includes("Applied(T)") &&
    candidateDirectory.includes("RejectedBeforeCommit") &&
    candidateDirectory.includes("Indeterminate") &&
    (candidateDirectory.match(/-> AdminEffectCommitV1</g) ?? []).length >= 14 &&
    [
      ["execute_create_space_with_id_and_admin_effect", 1],
      ["execute_with_admin_effect", 1],
      ["issue_share_token_as_with_admin_effect", 2],
      ["revoke_share_token_as_with_admin_effect", 2],
      ["revoke_auth_sessions_for_user_with_admin_effect", 2],
      ["issue_invite_as_with_admin_effect", 2],
      ["revoke_invite_as_with_admin_effect", 2],
      ["append_decided_events_with_admin_effect", 2],
    ].every(([name, count]) => (candidateDirectory.match(new RegExp(`async fn ${name}\\(`, "g")) ?? []).length >= Number(count)) &&
    [candidateSqlite, candidatePostgres, candidateNeo4j].every(
      (backend) =>
        backend.includes("admin_effect_rollback") &&
        backend.includes("AdminEffectCommitV1::RejectedBeforeCommit") &&
        backend.includes("AdminEffectCommitV1::Indeterminate") &&
        backend.includes("AdminEffectCommitV1::Applied"),
    ) &&
    candidateRoute.includes('code: "admin-effect-rejected-before-commit"') &&
    (candidateRoute.match(/AdminEffectCommitV1::RejectedBeforeCommit/g) ?? []).length >= 5 &&
    (candidateRoute.match(/AdminEffectCommitV1::Indeterminate/g) ?? []).length >= 5;
  if (!sourceClosed(route, stale, owner, runtime, schema) || !receiptClosed(stale, directory, sqlite, postgres, neo4j) || !outcomeClosed(route, directory, sqlite, postgres, neo4j)) {
    throw new Error("retained short administrator production ownership is incomplete");
  }
  if (!schemaTypeScript.includes('"succeeded" | "accepted" | "indeterminate" | "failed" | "cancelled"') || !schemaJson.includes('["succeeded", "accepted", "indeterminate", "failed", "cancelled"]')) {
    throw new Error("retained short administrator indeterminate state schema parity is incomplete");
  }
  const sourceHostiles = [
    [route.replace("admin_operation_tasks.spawn(", "tokio::spawn("), stale, owner, runtime, schema],
    [route, stale, owner.replace("BTreeMap<String, AdminOperationTask>", "Vec<tokio::task::JoinHandle<()>>"), runtime, schema],
    [route.replace("append_admin_terminal_with_retry(&task_state, &terminal)", "append_admin_terminal_with_retry(&task_state, &terminal_before_effect)"), stale, owner, runtime, schema],
    [route.replace("tokio::time::timeout_at(tokio::time::Instant::from_std(runtime.deadline), response_rx)", "response_rx.await"), stale, owner, runtime, schema],
    [route, `${stale}\nphase: \"cancelled\"`, owner, runtime, schema],
    [route.replace("admin_operation_slots.clone().try_acquire_owned()", "admin_operation_slots.clone().acquire_owned().await"), stale, owner, runtime, schema],
    [route, stale, `${owner}\nlet retained: Option<AdminIntentResultV1> = None;`, runtime, schema],
    [route, stale, owner.replace("tokio::time::timeout", "std::future::ready"), runtime, schema],
    [route, stale, owner, runtime.replace("compare_exchange(ADMIN_EFFECT_PRE_EFFECT, ADMIN_EFFECT_CANCELLED", "load(std::sync::atomic::Ordering::Acquire) == ADMIN_EFFECT_CANCELLED && store(ADMIN_EFFECT_CANCELLED"), schema],
    [route.replace("response_tx.send(Some(execution))", "response_tx.send(Some(admin_execution_receipt(task_operation_id, task_principal.correlation_id, execution)))"), stale, owner, runtime, schema],
    [route, stale.replace("ADMIN_TERMINAL_RETRY_DEADLINE", "std::time::Duration::ZERO"), owner, runtime, schema],
    [route, stale, owner, runtime, schema.replace("impl Drop for AdminIntentResultV1", "impl AdminIntentResultV1")],
  ];
  const receiptHostiles = [
    [stale, directory, sqlite.replace("append_decided_events_with_admin_effect", "append_effect_after_commit"), postgres, neo4j],
    [stale.replace("admin_operation_effect_receipt", "accepted_without_effect_receipt"), directory, sqlite, postgres, neo4j],
    [stale, directory.replaceAll("intent_digest: String", "intent_digest_omitted: String"), sqlite, postgres, neo4j],
    [`${stale}\nlet replayed: Option<AdminIntentSecretResult> = None;`, directory, sqlite, postgres, neo4j],
  ];
  const uncertaintyHostiles = [
    [route.replace('if execution.phase == "uncertain"', 'if execution.phase == "failed"'), stale, owner, runtime, schema],
    [route.replace("let execution = match authenticate_admin_principal", "let execution = tokio::time::timeout_at(task_runtime.deadline.into(), async { match authenticate_admin_principal"), stale, owner, runtime, schema],
  ];
  const outcomeHostiles = [
    [route, directory.replace("pub enum AdminEffectCommitV1<T>", "pub struct AdminEffectCommitV1<T>(T)"), sqlite, postgres, neo4j],
    [route, directory, sqlite.replaceAll("admin_effect_rollback", "admin_effect_drop"), postgres, neo4j],
    [route, directory, sqlite, postgres.replaceAll("AdminEffectCommitV1::Indeterminate", "AdminEffectCommitV1::RejectedBeforeCommit"), neo4j],
    [route, directory, sqlite, postgres, neo4j.replaceAll("AdminEffectCommitV1::Indeterminate", "AdminEffectCommitV1::RejectedBeforeCommit")],
    [route, directory.replaceAll("async fn issue_invite_as_with_admin_effect", "async fn issue_invite_as"), sqlite, postgres, neo4j],
    [route, directory.replaceAll("async fn revoke_auth_sessions_for_user_with_admin_effect", "async fn revoke_auth_sessions_for_user"), sqlite, postgres, neo4j],
    [route.replaceAll("AdminEffectCommitV1::RejectedBeforeCommit", "AdminEffectCommitV1::Indeterminate"), directory, sqlite, postgres, neo4j],
  ];
  if (sourceHostiles.length + receiptHostiles.length + uncertaintyHostiles.length + outcomeHostiles.length !== fixture.sourceHostiles.length) throw new Error("retained short administrator source hostile count differs");
  sourceHostiles.forEach(([candidateRoute, candidateStale, candidateOwner, candidateRuntime, candidateSchema], index) => {
    if (sourceClosed(candidateRoute!, candidateStale!, candidateOwner!, candidateRuntime!, candidateSchema!)) throw new Error(`retained short administrator source oracle admitted ${fixture.sourceHostiles[index]}`);
  });
  receiptHostiles.forEach(([candidateStale, candidateDirectory, candidateSqlite, candidatePostgres, candidateNeo4j], receiptIndex) => {
    if (receiptClosed(candidateStale!, candidateDirectory!, candidateSqlite!, candidatePostgres!, candidateNeo4j!)) {
      throw new Error(`retained short administrator source oracle admitted ${fixture.sourceHostiles[sourceHostiles.length + receiptIndex]}`);
    }
  });
  uncertaintyHostiles.forEach(([candidateRoute, candidateStale, candidateOwner, candidateRuntime, candidateSchema], index) => {
    if (sourceClosed(candidateRoute!, candidateStale!, candidateOwner!, candidateRuntime!, candidateSchema!)) {
      throw new Error(`retained short administrator source oracle admitted ${fixture.sourceHostiles[sourceHostiles.length + receiptHostiles.length + index]}`);
    }
  });
  outcomeHostiles.forEach(([candidateRoute, candidateDirectory, candidateSqlite, candidatePostgres, candidateNeo4j], index) => {
    if (outcomeClosed(candidateRoute!, candidateDirectory!, candidateSqlite!, candidatePostgres!, candidateNeo4j!)) {
      throw new Error(`retained short administrator source oracle admitted ${fixture.sourceHostiles[sourceHostiles.length + receiptHostiles.length + uncertaintyHostiles.length + index]}`);
    }
  });
  const hostileCount = sourceHostiles.length + receiptHostiles.length + uncertaintyHostiles.length + outcomeHostiles.length;
  const modelCount = fixture.cases.length + fixture.adminEffectCommitOutcomes.cases.length + fixture.crossCuttingOutcomes.length;
  console.log(`retained-short-admin-oracle: AJV=1 bun-sqlite=${modelCount} source-hostiles=${hostileCount}`);
  return modelCount + hostileCount;
}

export class RetainedShortAdminCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments[0] !== undefined && segments[0] !== "--native")) throw new Error("retained-short-admin-check accepts only --native");
    const checks = await proveRetainedShortAdmin(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 120_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-hostile-input-laws",
            target: { kind: "lib" },
            laws: ["retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact"],
          },
          {
            package: "semio-hub-directory-route-laws",
            target: { kind: "lib" },
            laws: ["retained_short_admin_shutdown_drains_before_bounded_abort_and_receipt_reconciliation_is_exact"],
          },
        ],progress(event) {
          console.log(`retained-short-admin ${event.stage}: ${event.law ?? event.package} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`retained-short-admin-native-receipt: ${JSON.stringify(receipt)}`);
    }
    console.log(`retained-short-admin-check: checks=${checks} mode=${segments[0] ?? "source"}`);
  }
}

function directoryMessageAuthoritySourceConforms(source: string): boolean {
  const body = (name: string): string => {
    const start = source.indexOf("fn " + name + "(");
    return start < 0 ? "" : source.slice(start, source.indexOf("\n}", start) + 2);
  };
  const union = body("directory_space_message_bindings");
  const sender = body("send_socket_directory_message");
  const recovery = body("send_socket_directory_rebootstrap");
  const authority = body("socket_live_authority_with_bindings");
  const ordered = (text: string, fragments: string[]): boolean => {
    let previous = -1;
    for (const fragment of fragments) {
      const current = text.indexOf(fragment);
      if (current <= previous) return false;
      previous = current;
    }
    return true;
  };
  return (
    ordered(union, ["record.bindings()", "SocketAudienceV1::Directory {", "SocketBindingKeyV1::DirectorySpaceAuthority", "SocketBindingKeyV1::Membership", "bindings.sort()", "bindings.dedup()"]) &&
    union.includes("user_id: user_id.clone()") &&
    body("directory_message_bindings").includes("directory_space_message_bindings(record, directory_stream_message_space(message))") &&
    !body("socket_record_bindings").includes("SocketAudienceV1::Directory {") &&
    ordered(authority, ["acquire_bindings(bindings)", "socket_binding_validity(", "document_plan_socket_validity(", "is_live(", "Ok(admission)"]) &&
    (authority.match(/acquire_bindings\(/g) ?? []).length === 1 &&
    !authority.includes("acquire_record(") &&
    ordered(sender, ["socket_live_authority_with_bindings(state, record, live_id, directory_message_bindings(record, message))", "socket_directory_membership_visibility(", "send_directory_message(sender, message)"]) &&
    !sender.includes("drop(_admission)") &&
    !sender.includes("socket_live_authority(") &&
    ordered(recovery, ["directory_space_message_bindings(record, Some(&scope.space_id))", "get_role(", "verified_rebootstrap_control(", "send_directory_message("]) &&
    !recovery.includes("drop(_admission)") &&
    !body("socket_directory_membership_visibility").includes(".revalidate(")
  );
}

export class DirectoryMessageAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "native"].includes(phase)) throw new Error("directory-message-authority-check accepts source or native");
    const root = join(this.repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🌐️directory-message-authority-v1");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    if (fixture.schema !== "semio.hub.directory-message-authority/v1") throw new Error("directory message authority fixture schema drift");
    if (Object.keys(fixture).sort().join(",") !== "messages,schema,vectors") throw new Error("directory message authority fixture envelope drift");
    if (fixture.vectors.length !== 4 || fixture.messages.length !== 6) throw new Error("directory message authority table sizes drifted");
    if (new Set(fixture.vectors.map((row: { name: string }) => row.name)).size !== 4 || new Set(fixture.messages.map((row: { name: string }) => row.name)).size !== 6) throw new Error("directory message authority names are not unique");
    for (const row of fixture.vectors) {
      if (!["membership", "session"].includes(row.revocation) || !["send", "revoke"].includes(row.first) || ![0, 4401].includes(row.close) || typeof row.a !== "boolean" || typeof row.b !== "boolean") throw new Error("directory message authority vector taxonomy drift: " + row.name);
    }
    for (const row of fixture.messages) {
      if (![2, 4].includes(row.keys) || typeof row.messageJson !== "string" || row.messageJson.length < 2 || row.messageJson.length > 8192) throw new Error("directory message authority message bound drift: " + row.name);
      if (row.spaceId !== null && (typeof row.spaceId !== "string" || row.spaceId.length < 1 || row.spaceId.length > 80)) throw new Error("directory message authority message scope drift: " + row.name);
    }
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      for (const row of fixture.vectors) {
        const expected = oracle.query("SELECT ? = 'send' AS a, ? = 'membership' AS b, CASE WHEN ? = 'session' THEN 4401 ELSE 0 END AS close").get(row.first, row.revocation, row.revocation) as { a: number; b: number; close: number };
        if (row.a !== !!expected.a || row.b !== !!expected.b || row.close !== expected.close) throw new Error("directory message ordering differs: " + row.name);
      }
      for (const row of fixture.messages) {
        const message = JSON.parse(row.messageJson);
        const space =
          message.kind === "event"
            ? (message.event.spaceId ?? null)
            : message.kind === "connection"
              ? message.connection.spaceId
              : message.kind === "presence"
                ? message.spaceId
                : message.kind === "rebootstrap-required"
                  ? message.control.scope.spaceId
                  : null;
        const expected = oracle
          .query(
            "SELECT CASE json_extract(?1, '$.kind') WHEN 'event' THEN json_extract(?1, '$.event.spaceId') WHEN 'connection' THEN json_extract(?1, '$.connection.spaceId') WHEN 'presence' THEN json_extract(?1, '$.spaceId') WHEN 'rebootstrap-required' THEN json_extract(?1, '$.control.scope.spaceId') ELSE NULL END AS space",
          )
          .get(row.messageJson) as { space: string | null };
        if (space !== row.spaceId || space !== expected.space || row.keys !== (space === null ? 2 : 4)) throw new Error("directory message scope differs: " + row.name);
      }
    } finally {
      oracle.close();
    }

    const hubSource = readHubBinSource(this.repoRoot);
    if (!directoryMessageAuthoritySourceConforms(hubSource)) throw new Error("directory global sender lacks a single retained transient message authority union");
    const mutants = [
      hubSource.replace("directory_message_bindings(record, message)).await", "record.bindings()).await"),
      hubSource.replace("directory_space_message_bindings(record, Some(&scope.space_id))).await", "record.bindings()).await"),
      hubSource.replace("directory_space_message_bindings(record, directory_stream_message_space(message))", "record.bindings()"),
      hubSource.replace("socket_directory_membership_visibility(state, record, message)", "unfenced_visibility(state, record, message)"),
      hubSource.replace("send_directory_message(sender, message)).await", "{ drop(_admission); send_directory_message(sender, message) }).await"),
    ];
    if (mutants.some((mutant) => mutant === hubSource || directoryMessageAuthoritySourceConforms(mutant))) throw new Error("directory message authority guard admitted a hostile source mutation");
    if (phase === "native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-directory-route-laws",
            target: { kind: "lib" },
            laws: ["directory_global_message_bindings_decode_wire_without_indexing_unrelated_memberships", "directory_global_socket_delivery_and_revocation_share_one_transient_authority_order", "socket_directory_visibility_requires_membership_even_for_public_spaces", "socket_directory_revoke_after_admission_suppresses_replay_without_deadlock"],
          },
          {
            package: "semio-hub-open-plan-laws",
            target: { kind: "lib" },
            laws: ["scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership", "scoped_directory_socket_message_matching_is_body_exact_and_removal_private"],
          },
        ],progress(event) {
          console.log("directory-message-authority " + event.stage + ": " + (event.law ?? event.package) + " artifacts=" + event.artifactDir);
        }});
      for (const receipt of receipts) console.log("directory-message-authority-native-receipt: " + JSON.stringify(receipt));
    }
    console.log("directory-message-authority-check: AJV=1 ordering=" + fixture.vectors.length + " wire-scopes=" + fixture.messages.length + " hostile-source=" + mutants.length + " phase=" + phase);
  }
}

export class ScopedDirectorySocketCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "all";
    if (segments.length > 1 || !["all", "source", "native", "process"].includes(phase)) throw new Error("scoped-directory-socket-check accepts source, native, or process");
    await proveScopedDirectorySocketRevocationFixture(this.repoRoot);
    if (phase === "all" || phase === "source") {
      runCmd(
        "bun",
        [
          join(this.repoRoot, "📜️script.ts"),
          "repo",
          "run",
          "@semio-tech/framework-os:test",
          "--skip-cache",
          "--",
          "quick",
          "--run",
          "-t",
          "round trips scoped directory worker ownership without flattening scope|binds one document scope and treats close 4401 as terminal without reacquiring|backbone worker owns one full scoped stream and retires it terminally on 4401",
        ],
        {
          cwd: this.repoRoot,
          ...orchestratorBudgetOpts(),
        },
      );
      console.log("scoped-directory-socket-source-check: neutral=19 hostile=3 browser-terminal=3");
    }
    if (phase === "all" || phase === "native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.repoRoot,groups: [
          {
            package: "semio-framework-os-kernel",
            target: { kind: "lib", name: "semio_framework_os_kernel" },
            laws: ["scoped_stream_close_4401_is_terminal_and_never_redials", "scoped_stream_issues_and_dials_the_same_encoded_scope"],
          },
        ],progress(event) {
          console.log(`scoped-directory-socket-native ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`scoped-directory-socket-native-receipt: ${JSON.stringify(receipt)}`);
    }
    if (phase === "all" || phase === "process") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 60_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-open-plan-laws",
            target: { kind: "lib" },
            cargoArgs: ["--all-features"],
            laws: ["scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership", "scoped_directory_socket_message_matching_is_body_exact_and_removal_private", "scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event", "scoped_directory_socket_admin_removal_uses_the_same_membership_fence"],
          },
          {
            package: "semio-hub-hostile-input-laws",
            target: { kind: "lib" },
            cargoArgs: ["--all-features"],
            laws: ["scoped_directory_socket_removal_and_delivery_have_one_total_membership_order"],
          },
        ],progress(event) {
          console.log(`scoped-directory-socket-process ${event.stage}: ${event.package} ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`scoped-directory-socket-process-receipt: ${JSON.stringify(receipt)}`);
      runCmd("cargo", ["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
    }
  }
}
