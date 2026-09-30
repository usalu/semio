import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { LOCAL_SESSION_BROKER_FILE, LOCAL_SESSION_REQUEST_SCHEMA, parseLocalSessionBrokerRecordV1, parseLocalSessionV1, type LocalSessionBrokerRecordV1, type LocalSessionV1 } from "./🧬️schema/🟦️.ts";

/** 🎫️ Asks the live hub's broker (the record in `dataDir` whose `runId` the hub at `hubOrigin` answers `/readyz` with)
 * for a fresh session of `profileId`. `null` when that hub has no live broker — a hub someone else operates. */
export async function requestLocalBrokerSession(dataDir: string, hubOrigin: string, profileId: string, signal: AbortSignal = AbortSignal.timeout(20_000)): Promise<LocalSessionV1 | null> {
  const path = join(dataDir, LOCAL_SESSION_BROKER_FILE);
  if (!existsSync(path)) return null;
  let record: LocalSessionBrokerRecordV1;
  try {
    record = parseLocalSessionBrokerRecordV1(JSON.parse(readFileSync(path, "utf8")));
  } catch {
    return null;
  }
  if (record.hubOrigin !== hubOrigin.replace(/\/+$/u, "") || !record.profiles.includes(profileId)) return null;
  const readiness = (await fetch(`${record.hubOrigin}/readyz`, { signal }).then((response) => response.json()).catch(() => null)) as { readonly runId?: unknown } | null;
  if (readiness?.runId !== record.runId) return null;
  const response = await fetch(`http://127.0.0.1:${record.port}/session`, {
    method: "POST",
    headers: { authorization: `Bearer ${record.secret}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: LOCAL_SESSION_REQUEST_SCHEMA, profileId }),
    signal,
  }).catch(() => null);
  if (response === null || !response.ok) return null;
  return parseLocalSessionV1(await response.json());
}
