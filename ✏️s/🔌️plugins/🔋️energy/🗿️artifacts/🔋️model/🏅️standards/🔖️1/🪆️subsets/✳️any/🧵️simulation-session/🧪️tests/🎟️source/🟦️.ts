/** 🎟️ Original finite fuel and semantic timestep units agree with independent SQLite arithmetic. */
import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { KernelOriginalOperationIssuer, KernelOriginalResourcePool, OriginalOperationSource } from "@semio-tech/framework/actor/retained-turn";
import law from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
function readOwnedRustTree(file: URL, visited = new Set<string>()): string {
  if (visited.has(file.href)) return "";
  visited.add(file.href);
  const source = readFileSync(file, "utf8");
  const modules = [...source.matchAll(/((?:#\[[^\n]+\]\s*)*)#\[path\s*=\s*"([^"]+)"\]\s*(?:pub\s+)?mod\b/g)].filter(match => !match[1].includes("#[cfg(test)]"));
  return [source, ...modules.map(match => readOwnedRustTree(new URL(match[2], file), visited))].join("\n");
}

export function verifyEnergyOriginalSessionOracle(): void {
  assert.equal(new Ajv({ strict: true }).compile(schema)(law), true);
  const database = new Database(":memory:");
  try {
    for (const row of law.loans) {
      const oracle = database.query("SELECT min(?,?) AS issued, ?-? AS remaining").get(row.parentFuel, row.requestedFuel, row.parentFuel, row.spentFuel) as { issued: number; remaining: number };
      assert.ok(row.spentFuel <= oracle.issued);
      assert.equal(oracle.remaining, row.remainingFuel);
      const grant = { maximumItems: law.stock.items, maximumCopyBytes: law.stock.copyBytes, maximumCapacityBytes: law.stock.capacityBytes, maximumReleaseBytes: law.stock.releaseBytes, maximumDepth: law.stock.depth };
      const resources = { grant, fuel: row.parentFuel, maximumElapsedUs: 1000n };
      const pool = new KernelOriginalResourcePool(resources), issuer = new KernelOriginalOperationIssuer(request => pool.admit(request));
      const source = new OriginalOperationSource(issuer.admit(resources), () => 1n), turn = source.issue();
      source.receive({ input: turn.input, spent: { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: row.spentFuel });
      assert.equal(source.fuelRemaining, oracle.remaining);
      assert.equal(source.physicalSpent.workUnitsSpent, row.spentFuel);
      assert.equal(pool.fuelRemaining, 0);
      source.close();
    }
    for (const row of law.identityActivity) {
      const oracle = database.query("SELECT ?>? AS timestep").get(row.completedAfter, row.completedBefore) as { timestep: number };
      assert.equal(Boolean(oracle.timestep), row.timestep);
    }
  } finally { database.close(); }
  const source = readOwnedRustTree(new URL("../../🦀️.rs", import.meta.url));
  const native = readFileSync(new URL("../🔬️unit/🦀️.rs", import.meta.url), "utf8");
  for (const forbidden of ["StepContext::new", "StepContext::with_payload_authority", "numerical_sequence", "Box::leak", "root_cancel_token()"] ) { assert.equal(source.includes(forbidden), false); assert.equal(native.includes(forbidden), false); }
  assert.ok(source.includes("cx.with_original_work_loan(fuel, deadline"));
  assert.ok(source.includes("cx.with_retained_loan(grant"));
  assert.ok(source.includes("Operation::new(cx.operation()"));
  assert.ok(source.includes("EnergyRunGrant::Activity"));
  assert.ok(source.includes("checkpoint: Option<sim_wire::EnergyWireLease>"));
  assert.ok(source.includes("*checkpoint = Some(lease)"));
  assert.ok(source.includes("match self.next_action(cx)?"));
  assert.ok(native.includes("turn_with_decoder(TEST_GRANT"));
  assert.ok(native.includes("original Energy caller closes once"));
  console.log("[DEBUG] Energy original source3 loan vectors preserve actual finite fuel; independent SQLite separates identity activity from timesteps; same-source child/checkpoint/native custody source laws retained; native execution separate");
}
