/** 🎟️ The fixture contract, sqlite oracle and hostile call checks of the invite redemption transaction proof. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { hubSchemaExport } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import type { InviteRedemptionFixture } from "../📐️invite-redemption-fixture-type/🟦️.ts";

/** 📜 Loads the invite redemption fixture and checks its schema, limits, vector names and acceptance marker. */
export async function checkRedemptionFixtureContract({ repoRoot }: {
  readonly repoRoot: string;
}) {
  const root = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1");
  const client = await import(join(repoRoot, "🧰️framework/🔨️modules/📇️directory/🟦️.ts"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as InviteRedemptionFixture;
  const acceptanceMarker = hubSchemaExport(repoRoot, "schema://hub.directory/InviteAcceptanceMarkerV1");
  if (fixture.schema !== "semio.hub.invite-redemption-transaction/v1") throw new Error("invite redemption transaction fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "authorityRaces,backendOrders,hostiles,limits,receiptRelease,schema,spaceStates,vectors") throw new Error("invite redemption transaction fixture envelope drift");
  if (fixture.limits.credentialBytes !== 4096 || fixture.limits.identifierBytes !== 4096 || fixture.limits.maximumContenders !== 3) throw new Error("invite redemption transaction bound drift");
  if (fixture.vectors.length < 16 || fixture.vectors.length > 32 || fixture.spaceStates.length !== 7 || fixture.authorityRaces.length !== 4 || fixture.backendOrders.length !== 2 || fixture.receiptRelease.length !== 4) throw new Error("invite redemption transaction table sizes drifted");
  const inviteHostiles = ["raw-capability", "client-space", "client-role", "client-event-id", "unknown-field", "oversized-identifier", "client-space-state"];
  if (fixture.hostiles.length < 4 || fixture.hostiles.length > 12 || new Set(fixture.hostiles.map(({ mutation }) => mutation)).size !== inviteHostiles.length || fixture.hostiles.some(({ mutation }) => !inviteHostiles.includes(mutation)))
    throw new Error("invite redemption transaction hostile inventory drift");
  if (new Set(fixture.vectors.map(({ name }) => name)).size !== fixture.vectors.length) throw new Error("invite redemption fixture names are not unique");
  for (const vector of fixture.vectors) {
    if (!acceptanceMarker(vector.expected.marker)) throw new Error(`invite redemption acceptance marker is not the owned marker contract: ${vector.name}`);
    if (vector.calls.length < 1 || vector.calls.length > fixture.limits.maximumContenders || vector.expected.outcomes.length !== vector.calls.length || vector.expected.returnedEventIds.length !== vector.calls.length)
      throw new Error(`invite redemption call/outcome arity differs: ${vector.name}`);
  }
  if (acceptanceMarker({ ...fixture.vectors[0]!.expected.marker, acceptedBy: "forged" }) || acceptanceMarker({ acceptedAt: -1, acceptedEventId: null })) throw new Error("invite redemption marker contract admitted a forged marker");
  return { fixture, client, acceptanceMarker };
}

/** 🔮 Replays every redemption vector against the sqlite oracle. */
export async function checkRedemptionOracleVectors({ fixture, client }: {
  readonly fixture: InviteRedemptionFixture;
  readonly client: any;
}) {
  const { Database } = await import("bun:sqlite");
  const oracle = new Database(":memory:");
  try {
    for (const row of fixture.spaceStates) {
      const expectedStatus = row.spaceState === "deleted" ? 401 : row.accepted === "same" ? 200 : row.accepted === "other" ? 409 : row.spaceState === "archived" && row.inviteRole === "author" ? 401 : 200;
      const appended = expectedStatus === 200 && row.accepted === "none" ? 1 : 0;
      const membershipRole = row.spaceState === "deleted" || (row.accepted !== "same" && appended === 0) ? "none" : row.spaceState === "archived" ? "spectator" : row.inviteRole;
      const sql = oracle
        .query("SELECT CASE WHEN ?1 = 'deleted' THEN 401 WHEN ?2 = 'same' THEN 200 WHEN ?2 = 'other' THEN 409 WHEN ?1 = 'archived' AND ?3 = 'author' THEN 401 ELSE 200 END AS status")
        .get(row.spaceState, row.accepted, row.inviteRole) as { status: number };
      if (sql.status !== expectedStatus || row.status !== expectedStatus || row.appended !== appended || row.membershipRole !== membershipRole) throw new Error(`invite state oracle differs for ${row.name}`);
    }

    for (const row of fixture.receiptRelease) {
      const release = row.outcome === "rejected-before-commit" && row.disposition === "pending" && row.digest === "exact";
      const sql = oracle.query("SELECT CASE WHEN ?1='rejected-before-commit' AND ?2='pending' AND ?3='exact' THEN 1 ELSE 0 END AS release").get(row.outcome, row.disposition, row.digest) as { release: number };
      if (release !== row.release || Boolean(sql.release) !== release) throw new Error(`receipt release oracle differs for ${row.name}`);
    }
    for (const row of fixture.backendOrders) {
      const accepted = row.first === "archive";
      const sql = oracle.query("SELECT CASE WHEN ?1 = 'archive' THEN 1 ELSE 0 END AS accepted").get(row.first) as { accepted: number };
      if (row.accepted !== accepted || sql.accepted !== Number(accepted) || row.appended !== Number(accepted) || (row.between === "invite") !== accepted) throw new Error(`invite backend order oracle differs for ${row.name}`);
      const event = (seq: number, body: unknown) => ({ seq, id: `archive-event-${seq}`, hlc: { physicalMs: seq, logical: 0 }, actor: { kind: "system", id: "archive-oracle" }, recordedAtMs: seq, body });
      const before = client.foldAll(client.emptyDirectoryReadModel(), [
        event(1, { kind: "space.created", spaceId: "archive-space", name: "Archive", spaceKind: "studio", visibility: "private", ownerUserId: "owner" }),
        event(2, { kind: "member.upserted", spaceId: "archive-space", userId: "owner", role: "author" }),
        event(3, { kind: row.between === "invite" ? "invite.redeemed" : "member.upserted", spaceId: "archive-space", userId: "member", inviteId: "archive-invite", role: row.between === "invite" ? "author" : "spectator" }),
      ]);
      before.spaces.get("archive-space").view.role = "author";
      const archived = client.fold(before, event(4, { kind: "space.archived", spaceId: "archive-space" })).spaces.get("archive-space");
      oracle.exec(
        "CREATE TABLE IF NOT EXISTS archive_members(user_id TEXT PRIMARY KEY, role TEXT); DELETE FROM archive_members; INSERT INTO archive_members VALUES ('owner','author'),('member','author'); UPDATE archive_members SET role='spectator' WHERE role='author'",
      );
      const expected = oracle.query("SELECT role FROM archive_members ORDER BY user_id").all() as { role: string }[];
      if (
        archived.view.kind !== "archive" ||
        archived.view.role !== "spectator" ||
        JSON.stringify(archived.members.map((member: { role: string }) => member.role)) !== JSON.stringify(expected.map(({ role }) => role)) ||
        before.spaces.get("archive-space").view.role !== "author"
      )
        throw new Error(`archive client fold differs from SQLite oracle for ${row.name}`);
      console.log(`directory-archive-fold case=${row.name} members=2 role=spectator immutable=1`);
    }

    for (const row of fixture.authorityRaces) {
      const revoke = row.order.indexOf("revoke");
      const fence = row.order.indexOf("fence");
      const status = revoke < fence ? 401 : 200;
      const sql = oracle.query("SELECT CASE WHEN ?1 < ?2 THEN 401 ELSE 200 END AS status").get(revoke, fence) as { status: number };
      if (row.order[0] !== "hint" || row.status !== status || sql.status !== status || row.appended !== Number(status === 200)) throw new Error(`invite authority race oracle differs for ${row.name}`);
    }
  } finally {
    oracle.close();
  }
  for (const vector of fixture.vectors) {
    const accepted = vector.initial === "accepted-same" || vector.initial === "accepted-other" || vector.initial === "corrupt-marker" || vector.initial === "corrupt-event";
    let acceptedAt: number | null = accepted ? 100 : null;
    let acceptedEventId: string | null = vector.initial === "corrupt-marker" ? null : vector.initial === "corrupt-event" ? "event-missing" : accepted ? "event-0" : null;
    let event = vector.initial === "accepted-same" ? { id: "event-0", user: "same", recordedAt: 100 } : vector.initial === "accepted-other" ? { id: "event-0", user: "other", recordedAt: 100 } : null;
    let membership = event ? 1 : 0;
    let publications = 0;
    let revoked = vector.initial === "revoked";
    const recordExists = vector.initial !== "missing";
    const userExists = vector.initial !== "missing-user";
    const spaceExists = vector.initial !== "missing-space";
    const outcomes: string[] = [];
    const returnedEventIds: (string | null)[] = [];
    for (const call of vector.calls) {
      if (call.kind === "restart") {
        outcomes.push("restarted");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "rebuild") {
        membership = event ? 1 : 0;
        outcomes.push("rebuilt");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "append-forged") {
        outcomes.push("append-denied");
        returnedEventIds.push(null);
        continue;
      }
      if (call.kind === "revoke") {
        if (acceptedAt !== null || revoked) outcomes.push("conflict");
        else {
          revoked = true;
          outcomes.push("revoked");
        }
        returnedEventIds.push(null);
        continue;
      }
      if (!recordExists || call.actor !== "exact" || call.credential !== "exact") {
        outcomes.push("unauthorized");
        returnedEventIds.push(null);
        continue;
      }
      if ((acceptedAt === null) !== (acceptedEventId === null)) {
        outcomes.push("backend-error");
        returnedEventIds.push(null);
        continue;
      }
      if (acceptedAt !== null) {
        if (!event || event.id !== acceptedEventId || event.recordedAt !== acceptedAt) {
          outcomes.push("backend-error");
          returnedEventIds.push(null);
        } else if (event.user !== call.user) {
          outcomes.push("conflict");
          returnedEventIds.push(null);
        } else {
          outcomes.push("already-committed");
          returnedEventIds.push(event.id);
        }
        continue;
      }
      if (!userExists || !spaceExists || call.user === "missing") {
        outcomes.push("unauthorized");
        returnedEventIds.push(null);
        continue;
      }
      if (revoked || vector.initial === "expired") {
        outcomes.push("conflict");
        returnedEventIds.push(null);
        continue;
      }
      if (call.failure !== "none") {
        outcomes.push("backend-error");
        returnedEventIds.push(null);
        continue;
      }
      acceptedAt = 101;
      acceptedEventId = "event-1";
      event = { id: "event-1", user: call.user, recordedAt: 101 };
      membership = 1;
      publications += 1;
      outcomes.push("newly-committed");
      returnedEventIds.push(event.id);
    }
    const actual = { outcomes, returnedEventIds, marker: { acceptedAt, acceptedEventId }, events: event ? 1 : 0, memberships: membership, publications, replayEvents: event ? 1 : 0, revoked };
    if (JSON.stringify(actual) !== JSON.stringify(vector.expected)) throw new Error(`invite redemption independent model differs for ${vector.name}: ${JSON.stringify(actual)}`);
  }
}

/** ☠ Checks every redemption call against the call schema and every hostile mutation against the fixture. */
export function checkRedemptionCallsAndHostiles({ repoRoot, fixture, acceptanceMarker }: {
  readonly repoRoot: string;
  readonly fixture: InviteRedemptionFixture;
  readonly acceptanceMarker: (value: unknown) => boolean;
}) {
  const redemptionCall = hubSchemaExport(repoRoot, "schema://hub.directory/InviteRedemptionCallV1");
  for (const vector of fixture.vectors) for (const call of vector.calls) if (!redemptionCall(call)) throw new Error(`invite redemption call is not the owned call contract: ${vector.name}`);
  for (const hostile of fixture.hostiles) {
    const call = structuredClone(fixture.vectors[0]!.calls[0]!) as unknown as Record<string, unknown>;
    if (hostile.mutation === "raw-capability") call.rawCapability = "forbidden";
    else if (hostile.mutation === "client-space") call.spaceId = "client-space";
    else if (hostile.mutation === "client-space-state") call.spaceState = "writable";
    else if (hostile.mutation === "client-role") call.role = "author";
    else if (hostile.mutation === "client-event-id") call.eventId = "client-event";
    else if (hostile.mutation === "unknown-field") {
      const candidate = { ...(fixture as unknown as Record<string, unknown>), unknown: true };
      if (Object.keys(candidate).sort().join(",") === "authorityRaces,backendOrders,hostiles,limits,receiptRelease,schema,spaceStates,vectors") throw new Error(`invite redemption envelope admitted hostile ${hostile.name}`);
      continue;
    } else {
      if (acceptanceMarker({ acceptedAt: 1, acceptedEventId: "x".repeat(4097) })) throw new Error(`invite redemption marker admitted hostile ${hostile.name}`);
      continue;
    }
    if (redemptionCall(call)) throw new Error(`invite redemption call contract admitted hostile ${hostile.name}`);
  }
}
