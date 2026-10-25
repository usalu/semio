/** 📐️ Resize geometry and same-source finite work agree with independent SQLite. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";
import { KernelOriginalOperationIssuer, KernelOriginalResourcePool, OriginalOperationSource } from "@semio-tech/framework-actor/retained-turn";
import law from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import Ajv from "ajv";
test("surface resize preserves geometry and original source currencies", () => {
 expect(new Ajv({ strict:true }).compile(schema)(law)).toBe(true);
 const db = new Database(":memory:");
 try {
  for (const row of law.rows) {
   const oracle = db.query("SELECT ? / ? AS width, ? / ? AS height").get(row.width, row.scale, row.height, row.scale) as {width:number;height:number};
   expect(oracle.width).toBe(row.logicalWidth); expect(oracle.height).toBe(row.logicalHeight);
  }
  const resources = { grant:law.stock, fuel:law.parentFuel, maximumElapsedUs:1000n };
  const pool = new KernelOriginalResourcePool(resources), issuer = new KernelOriginalOperationIssuer(request => pool.admit(request));
  const source = new OriginalOperationSource(issuer.admit(resources), () => 1n), turn = source.issue();
  source.receive({input:turn.input, spent:{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0}, workUnitsSpent:law.workUnits});
  const oracle = db.query("SELECT ?-? AS remaining").get(law.parentFuel,law.workUnits) as {remaining:number};
  expect(source.fuelRemaining).toBe(oracle.remaining); expect(source.physicalSpent.workUnitsSpent).toBe(law.workUnits); source.close();
 } finally { db.close(); }
 const source = readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 for (const forbidden of ["StepContext::new", "root_cancel_token()", "MountedWorkerJobSession::try_new", "released_items", "StepOutcome::"]) expect(source.includes(forbidden)).toBe(false);
 for (const required of ["pump_one_for_interactive_turn(cx)", "admit_pending", "close_original_retained_step", "fn borrow_outcome", "OriginalTurnFailure"]) expect(source.includes(required)).toBe(true);
});
