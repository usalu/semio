import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import currencies from "../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧬️schema/🔣️.json";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { recordMountedSnapshotPreparation, sameArtifactPendingSourceVersion } from "../🟦️.ts";

/** 🧾️ Uses SQLite's independent conjunction arithmetic for the original receipt corpus. */
export function mountedSnapshotPreparationOracle(): number {
  const db = new Database(":memory:");
  try {
    const ajv = semioSchemaAjvV1(); ajv.addSchema({ $id: currencies.$id, $defs: currencies.$defs }); const validate = ajv.compile(schema), validateGrant = ajv.compile(currencies.$defs.grant);
    const query = db.query("SELECT (?1 <= ?5 AND ?2 <= ?6 AND ?3 <= ?7 AND ?4 <= ?8 AND (?9 > 0 OR (?1 = 0 AND ?2 = 0 AND ?3 = 0 AND ?4 = 0))) AS admitted");
    for (const row of fixture.cases) {
      const [items, copy, capacity, release, depth] = row.grant;
      const [usedItems, usedCopy, usedCapacity, usedRelease] = row.progress;
      const result = query.get(usedItems!, usedCopy!, usedCapacity!, usedRelease!, items!, copy!, capacity!, release!, depth!) as { admitted: number };
      expect(!result.admitted).toBe(row.fault);
      const grant = { maximumItems: items!, maximumCopyBytes: copy!, maximumCapacityBytes: capacity!, maximumReleaseBytes: release!, maximumDepth: depth! };
      expect(validateGrant(grant)).toBe(true);
      const step = { ready: row.ready, progress: { copiedItems: usedItems!, copiedBytes: usedCopy!, retainedCapacityBytes: usedCapacity!, releasedBytes: usedRelease! } };
      expect(validate(step)).toBe(true);
      let actualReceipt: typeof step.progress | undefined, fuel = 8, fault = false;
      try { expect(recordMountedSnapshotPreparation(step, grant, { retainedGrant: () => grant, fuelRemaining: () => fuel, consumeRetained: progress => { actualReceipt = progress; }, consumeFuel: items => { fuel -= items; } })).toBe(row.ready); } catch { fault = true; }
      expect(fault).toBe(row.fault); expect(actualReceipt).toEqual(step.progress); expect(fuel).toBe(8 - usedItems!);
      console.log(`[DEBUG] mounted-snapshot-preparation ${row.name} ready=${row.ready} fault=${row.fault}`);
    }
    return fixture.cases.length;
  } finally { db.close(); }
}

test("mounted preparation receipt corpus agrees with SQLite", () => expect(mountedSnapshotPreparationOracle()).toBe(9));

test("original mounted pending source compares full document revision and provisional authority",()=>{const db=new Database(":memory:");try{for(const row of fixture.sourceVersions){expect(semioSchemaAjvV1().compile({...schema,type:undefined,properties:undefined,required:undefined,additionalProperties:undefined,$ref:"#/$defs/sourceTransition"})(row)).toBe(true);const a=row.before,b=row.after;const oracle=db.query("SELECT (?1=?5 AND ?2=?6 AND ?3=?7 AND ?4=?8) equal").get(a.baseRevision,a.generation,a.canonicalRevision,a.overlay,b.baseRevision,b.generation,b.canonicalRevision,b.overlay)as{equal:number};expect(!oracle.equal).toBe(row.changed);expect(!sameArtifactPendingSourceVersion(a,b)).toBe(row.changed);console.log(`[DEBUG] original poll source ${row.name} changed=${row.changed}`);}}finally{db.close();}});
