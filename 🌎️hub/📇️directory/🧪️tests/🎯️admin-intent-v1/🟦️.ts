import { readFileSync } from "node:fs";
import Ajv from "ajv";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎯️admin-intent-v1/🔣️.json", import.meta.url), "utf8"));
const module = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(module);
const contract = (exportId: string) => {
  const validate = ajv.getSchema(`${module.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`hub.directory exports no ${exportId}`);
  return validate;
};

if (fixture.schema !== "semio.hub.admin-intent-v1.fixture/v1") throw new Error("admin intent fixture schema drift");
if (Object.keys(fixture).sort().join(",") !== "auditPhases,createSpaceIntent,expectedActor,invalidVectors,limits,outcomes,principal,recordedConnection,redactionProbes,schema") throw new Error("admin intent fixture envelope drift");
if (fixture.limits.textBytes !== 256 || fixture.limits.intentBytes !== 8192 || fixture.limits.pageRows !== 100 || fixture.limits.responseBytes !== 65536) throw new Error("admin intent bound drift");
if (!contract("AdminPrincipalV1")(fixture.principal)) throw new Error("admin intent principal is not the owned principal contract");
if (!contract("AdminCreateSpaceIntentV1")(fixture.createSpaceIntent)) throw new Error("admin intent create-space intent is not the owned intent contract");
if (!contract("AdminActorV1")(fixture.expectedActor)) throw new Error("admin intent actor is not the owned actor contract");
if (!contract("AdminRecordedConnectionV1")(fixture.recordedConnection.stored)) throw new Error("admin intent recorded connection is not the owned connection contract");
for (const name of ["durableRevoke", "ephemeralKick"]) if (!contract("AdminIntentOutcomeV1")(fixture.outcomes[name])) throw new Error(`admin intent outcome is not the owned outcome contract: ${name}`);
if (contract("AdminCreateSpaceIntentV1")({ ...fixture.createSpaceIntent, command: { kind: "announce-document" } })) throw new Error("admin intent contract admitted a generic directory command");
if (contract("AdminPrincipalV1")({ ...fixture.principal, peerClass: "public-rest" })) throw new Error("admin intent contract admitted a non-administrative peer class");
if (fixture.recordedConnection.publicKeys.length !== 7 || new Set(fixture.recordedConnection.publicKeys).size !== 7) throw new Error("admin intent public projection inventory drift");
if (fixture.auditPhases.length !== 2 || new Set(fixture.invalidVectors).size !== fixture.invalidVectors.length || fixture.invalidVectors.length < 20) throw new Error("admin intent audit/invalid inventory drift");
if (fixture.redactionProbes.length < 3 || new Set(fixture.redactionProbes).size !== fixture.redactionProbes.length) throw new Error("admin intent redaction probe inventory drift");

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
console.log("admin-intent-v1 oracle: 5/5; invalid inventory 22/22; hub.directory exports 5/5");
