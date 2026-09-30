import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { requestLocalBrokerSession } from "../../🟦️.ts";
import { LOCAL_SESSION_BROKER_FILE, parseLocalSessionBrokerRecordV1, parseLocalSessionRequestV1, parseLocalSessionV1 } from "../../🧬️schema/🟦️.ts";

/** 🧪️ Shared contract cases and live loopback exchanges prove issuer/client ownership. */
export async function proveLocalSessionBrokerContract(artifactDirectory: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const defs = { record: "LocalSessionBrokerRecordV1", request: "LocalSessionRequestV1", session: "LocalSessionV1" } as const;
  const parsers = { record: parseLocalSessionBrokerRecordV1, request: parseLocalSessionRequestV1, session: parseLocalSessionV1 };
  for (const kind of Object.keys(defs) as (keyof typeof defs)[]) {
    const validate = new Ajv({ strict: true }).addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } }).compile({ ...schema, $ref: `#/$defs/${defs[kind]}` });
    for (const row of fixture.cases.filter((row: { kind: string }) => row.kind === kind)) {
      let accepted = true;
      try { parsers[kind](row.value); } catch { accepted = false; }
      assert.equal(accepted, row.accepted, row.name);
      assert.equal(validate(row.value), row.accepted, `${row.name}: Ajv`);
    }
  }
  mkdirSync(artifactDirectory, { recursive: true });
  const dataDir = mkdtempSync(join(artifactDirectory, "session-broker-"));
  const record = fixture.cases.find((row: { name: string }) => row.name === "record").value;
  const session = fixture.cases.find((row: { name: string }) => row.name === "session").value;
  let observedRequests = 0;
  const origin = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: () => Response.json({ runId: record.runId }) });
  const broker = Bun.serve({ hostname: "127.0.0.1", port: 0, async fetch(request) {
    observedRequests++;
    assert.equal(request.headers.get("authorization"), `Bearer ${record.secret}`);
    assert.equal(parseLocalSessionRequestV1(await request.json()), "developer");
    return Response.json(session);
  } });
  try {
    const hubOrigin = `http://127.0.0.1:${origin.port}`;
    writeFileSync(join(dataDir, LOCAL_SESSION_BROKER_FILE), JSON.stringify({ ...record, hubOrigin, port: broker.port }));
    assert.deepEqual(await requestLocalBrokerSession(dataDir, hubOrigin, "developer"), session);
    assert.equal(await requestLocalBrokerSession(dataDir, hubOrigin, "foreign"), null);
    writeFileSync(join(dataDir, LOCAL_SESSION_BROKER_FILE), JSON.stringify({ ...record, hubOrigin, port: broker.port, runId: "3".repeat(32) }));
    assert.equal(await requestLocalBrokerSession(dataDir, hubOrigin, "developer"), null);
    assert.equal(observedRequests, 1);
    console.log(`local-session-broker-contract: vectors=${fixture.cases.length} ajv+typescript=1 live-exchange=1 stale-run=refused foreign-profile=refused`);
  } finally {
    origin.stop(true);
    broker.stop(true);
    rmSync(dataDir, { recursive: true, force: true });
  }
  return fixture.cases.length;
}
