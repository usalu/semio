import assert from "node:assert/strict";
import { mkdirSync, readFileSync, mkdtempSync, rmSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { parseDevLocalHubProviderV1 } from "../../🧬️schema/🟦️.ts";
import { devHubWorldV1, ownDevHubV1 } from "../../🏃️execution/🟦️.ts";

/** 🧪️ A neutral owner declaration drives a real process, with JSON Schema as the independent oracle. */
export async function proveDevLocalHubProviderContract(repoRoot: string, artifactDirectory: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const brokerSchema = JSON.parse(readFileSync(new URL("../../../../📇️directory/🎫️local-session/🗄️broker/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true }).addKeyword("x-semio-formats").addSchema(brokerSchema);
  const validate = ajv.compile(schema);
  const processOracle = ajv.compile({ const: fixture.processCases.map((row: { name: string; outcome: string; expectedAlive: boolean }) => ({ name: row.name, outcome: row.outcome, expectedAlive: row.expectedAlive })) });
  for (const row of fixture.cases) {
    let accepted = true;
    try { parseDevLocalHubProviderV1(row.value); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, row.name);
    assert.equal(validate(row.value), row.accepted, `${row.name}: Ajv`);
  }
  mkdirSync(artifactDirectory, { recursive: true });
  const dataDir = mkdtempSync(join(artifactDirectory, "owner-provider-"));
  try {
    const marker = "neutral-owner-contract";
    const provider = parseDevLocalHubProviderV1({ owner: { program: "bun", args: ["-e", `console.log(JSON.stringify({ marker: ${JSON.stringify(marker)}, args: process.argv.slice(1) }));setInterval(()=>{},1000)`] }, defaultProfileId: "human" });
    const world = devHubWorldV1(repoRoot, "en", provider);
    const owner = await world.spawnOwner("http://127.0.0.1:8787", dataDir);
    assert.notEqual(owner, null);
    const deadline = Date.now() + 10_000;
    while (readFileSync(join(dataDir, "local-hub.log"), "utf8").trim().length === 0 && Date.now() < deadline) await new Promise((done) => setTimeout(done, 20));
    const output = JSON.parse(readFileSync(join(dataDir, "local-hub.log"), "utf8").trim());
    assert.equal(output.marker, marker);
    assert.deepEqual(output.args.slice(-2), ["http://127.0.0.1:8787", dataDir]);
    const outcomes = [{ name: "declared-owner", outcome: owner === null ? "refused" : "spawned", expectedAlive: owner !== null && world.alive(owner) }];
    await world.stopOwner(owner!);
    assert.equal(world.alive(owner!), false, "owned process cleanup completed");
    const missing = devHubWorldV1(repoRoot, "en", { ...provider, owner: { program: join(dataDir, "absent-executable"), args: ["owner"] } });
    const beforeMissing = Date.now();
    const failed = await missing.spawnOwner("http://127.0.0.1:8787", dataDir);
    assert.equal(failed, null, "missing executable is a controlled refusal");
    assert.ok(Date.now() - beforeMissing < 5_000, "missing executable refusal is bounded");
    outcomes.push({ name: "missing-executable", outcome: failed === null ? "refused" : "spawned", expectedAlive: failed !== null && missing.alive(failed) });
    const cancellation = new AbortController();
    let pendingOwner: number | null = null;
    const cancelWorld = { ...world, ready: async () => false, portInUse: () => false, sleep: async () => { cancellation.abort(); }, spawnOwner: async (url: string, directory: string, signal?: AbortSignal) => { pendingOwner = await world.spawnOwner(url, directory, signal); return pendingOwner; } };
    const beforeCancel = Date.now();
    const up = await ownDevHubV1("http://127.0.0.1:8787", dataDir, join(dataDir, "leases"), cancelWorld, cancellation.signal);
    assert.equal(up, false);
    assert.notEqual(pendingOwner, null, "cancellation exercised an actual child");
    assert.ok(Date.now() - beforeCancel < 5_000, "cancellation cleanup is bounded");
    outcomes.push({ name: "cancelled-startup", outcome: up ? "spawned" : "refused", expectedAlive: pendingOwner !== null && world.alive(pendingOwner) });
    assert.ok(processOracle(outcomes), "actual processes agree with neutral lifecycle vectors");
    const alreadyCancelled = new AbortController(); alreadyCancelled.abort();
    assert.equal(await world.spawnOwner("http://127.0.0.1:8787", dataDir, alreadyCancelled.signal), null);
    console.log(`dev-local-hub-provider-contract: vectors=${fixture.cases.length} ajv+typescript=1 owner-process=observed missing-executable=controlled cancelled-owner=closed`);
  } finally { rmSync(dataDir, { recursive: true, force: true }); }
  return fixture.cases.length;
}
