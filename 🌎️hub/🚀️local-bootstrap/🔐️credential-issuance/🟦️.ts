import { randomBytes } from "node:crypto";
import { LOCAL_BOOTSTRAP_DEADLINE_MS, writeLocalFrame } from "../📡️framing/🟦️.ts";
import { authenticatedFrame, LOCAL_BOOTSTRAP_SCHEMA, type LocalClientClass, verifyAuthenticatedFrame } from "../🛂authentication/🟦️.ts";
import type { LocalHubRun } from "../🏃️execution/🟦️.ts";

/** 🔐️ Issues one authenticated client-class credential bound to the active local Hub run. */
export async function issueLocalCredential(run: LocalHubRun, profileId: string, clientClass: LocalClientClass, sequence = 2, exchangeId = randomBytes(16).toString("hex")): Promise<Record<string, any>> {
  const now = Date.now();
  const issue = authenticatedFrame(run.channelKey, {
    schema: LOCAL_BOOTSTRAP_SCHEMA,
    kind: "issue",
    runId: run.runId,
    sequence,
    exchangeId,
    issuedAt: now,
    expiresAt: now + LOCAL_BOOTSTRAP_DEADLINE_MS,
    profileId,
    deviceInstanceId: `${clientClass}-launcher`,
    clientClass,
  });
  await writeLocalFrame(run.pipe, issue);
  const envelope = await run.reader.read();
  verifyCredentialEnvelope(run, envelope, profileId, clientClass, exchangeId);
  return envelope;
}

function verifyCredentialEnvelope(run: LocalHubRun, envelope: Record<string, any>, profileId: string, clientClass: LocalClientClass, exchangeId: string): void {
  verifyAuthenticatedFrame(run.channelKey, envelope);
  if (
    envelope.schema !== "semio.hub.local-credential-envelope/v1" ||
    envelope.runId !== run.runId ||
    envelope.exchangeId !== exchangeId ||
    envelope.profileId !== profileId ||
    envelope.clientClass !== clientClass ||
    envelope.sessionKind !== "development-local" ||
    !Number.isSafeInteger(envelope.authorizationGeneration) ||
    envelope.authorizationGeneration < 1
  )
    throw new Error("local credential envelope binding mismatch");
}
