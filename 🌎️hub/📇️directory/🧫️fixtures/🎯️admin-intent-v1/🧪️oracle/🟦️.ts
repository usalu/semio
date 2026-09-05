import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";

const fixture = JSON.parse(readFileSync(new URL("../🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️.schema.json", import.meta.url), "utf8"));
const validate = new Ajv2020({ strict: true }).compile(schema);
if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));

const actor = `user:${fixture.principal.userId}#admin-session:${fixture.principal.sessionId}`;
if (actor !== fixture.expectedActor.id || fixture.expectedActor.kind !== "user" || fixture.expectedActor.ownerUserId !== fixture.principal.userId) throw new Error("session-derived actor mismatch");
if (fixture.auditPhases[0] !== "accepted" || !["succeeded", "failed", "cancelled"].includes(fixture.auditPhases[1])) throw new Error("append-only audit ordering mismatch");
if (!fixture.outcomes.durableRevoke.durable || fixture.outcomes.ephemeralKick.durable) throw new Error("revoke and kick durability were conflated");
if (fixture.outcomes.durableRevoke.kickSignalled > fixture.outcomes.durableRevoke.kickAttempted) throw new Error("durable revoke kick accounting mismatch");
if (fixture.createSpaceIntent.kind !== "create-space" || "command" in fixture.createSpaceIntent) throw new Error("generic directory command escaped into admin intent");
if (!fixture.invalidVectors.includes("generic-directory-command") || !fixture.invalidVectors.includes("forbidden-admin-announce-document")) throw new Error("closed admin taxonomy negatives missing");

const stored = fixture.recordedConnection.stored;
const projected = {
  syncSessionId: stored.syncSessionId,
  scope: { spaceId: stored.spaceId, documentId: stored.documentId },
  authenticatedUserId: stored.authenticatedUserId,
  email: stored.email,
  role: stored.role,
  connectedAtMs: stored.connectedAtMs,
  source: "recorded-sync-session",
};
if (JSON.stringify(Object.keys(projected)) !== JSON.stringify(fixture.recordedConnection.publicKeys)) throw new Error("recorded connection projection drift");
for (const forbidden of ["actorId", "clientLabel", "surface", "presenceKnown"]) if (forbidden in projected) throw new Error(`legacy connection claim escaped: ${forbidden}`);

const publicEnvelope = JSON.stringify({ actor, projected, auditPhases: fixture.auditPhases, outcomes: fixture.outcomes });
for (const secret of fixture.redactionProbes) if (publicEnvelope.includes(secret)) throw new Error("secret or locator escaped into public admin state");
if (fixture.invalidVectors.length !== 22) throw new Error("admin invalid-vector inventory drift");
console.log("admin-intent-v1 oracle: 5/5; invalid inventory 22/22");
