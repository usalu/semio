import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🧫️fixtures/owned.json" with {type:"json"};
import schema from "../🧬️schema/owned.json" with {type:"json"};
import escrow from "./🧫️fixtures/escrow.json" with {type:"json"};
test("owned byte donor keeps physical custody until funded handover and release",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const bytes=Buffer.from(law.inputHex,"hex"),db=new Database(":memory:");try{
  expect(db.query("SELECT lower(hex(?)) AS bytes").get(bytes)).toEqual({bytes:law.inputHex});
  expect(db.query("SELECT (?=0) AS donorRetained, (?=0) AS capacityRetained").get(law.deniedCopyBytes,law.deniedReleaseBytes)).toEqual({donorRetained:Number(law.expected.donorRetainedOnDenied),capacityRetained:Number(law.expected.inputCapacityRetainedOnDeniedRelease)});
 }finally{db.close();}
});

test("actor input escrow spans each complete row through the published maximum", () => {
 const db = new Database(":memory:");
 try {
  const valid = db.query<{ result: number }, [number, number, number, number, number]>(
   "select (?1 > 0 and ?2 between ?3 and (?3 + ?4 * ?5) and (?2 - ?3) % ?4 = 0) as result",
  );
  for (const row of escrow.cases) {
   expect(Boolean(valid.get(row.pointer, row.length, escrow.headerBytes, escrow.rowBytes, escrow.maximumRows)?.result)).toBe(row.accepted);
  }
 } finally { db.close(); }
});

test("input admission passes the full nineteen scalar budget after its two byte parameters",()=>{const suffix=law.inputAdmission;const type=[1,96,suffix.callScalars,127,127,...Array(suffix.budgetScalars).fill(126),1,126];const body=[0,32,2];for(let i=3;i<suffix.callScalars;i++)body.push(32,i,124);body.push(11);const bytes=new Uint8Array([0,97,115,109,1,0,0,0,1,type.length,...type,3,2,1,0,7,5,1,1,102,0,0,10,body.length+2,1,body.length,...body]);const module=new WebAssembly.Module(bytes),f=new WebAssembly.Instance(module).exports.f as(...args:(number|bigint)[])=>bigint;const db=new Database(":memory:");try{expect(f(12,34,...suffix.scalars.map(BigInt))).toBe(BigInt(suffix.expectedSum));expect(db.query("SELECT sum(value) AS total FROM json_each(?)").get(JSON.stringify(suffix.scalars))).toEqual({total:suffix.expectedSum});expect(suffix.prefixScalars+suffix.budgetScalars).toBe(suffix.callScalars);}finally{db.close();}});
