import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {isReplicaProgressReceiptV1,replicaProgressRestartDueV1,replicaProgressStalledV1} from "../🟦️.ts";

test("owned maintenance needs independent stale progress and attestation with responsive local and store probes",()=>{
 const db=new Database(":memory:"),policy=schema.definitions.Policy.const;
 try{
  for(const fixtureCase of fixture.cases){
   const x={...fixture.base,...fixtureCase.row};
   const sql=db.query("select coalesce(?1-?2>=?8 and ?1-?3>=?8 and ?4 between 0 and ?9 and ?5 between 0 and ?10 and ?6 and ?7,0) as stall");
   const independent=sql.get(x.now,x.began,x.attested,x.lag,x.probeMilliseconds,Number(x.store),Number(x.local),policy.quietMs,policy.maximumLagMs,policy.probeMs) as {stall:number};
   expect(Boolean(independent.stall)).toBe(fixtureCase.expected);
   expect(replicaProgressStalledV1(x)).toBe(Boolean(independent.stall));
  }
 }finally{db.close();}
});

test("progress restart intent is closed and bounds subsequent exact-owned attempts",()=>{
 const db=new Database(":memory:"),policy=schema.definitions.Policy.const;
 const row=fixture.receipt,receipt={...row,at:row.began+policy.quietMs,attested:row.began};
 try{
  expect(isReplicaProgressReceiptV1(receipt)).toBe(true);
  expect(isReplicaProgressReceiptV1({...receipt,attested:row.began+1})).toBe(false);
  expect(isReplicaProgressReceiptV1({...receipt,scope:"transfer"})).toBe(false);
  expect(isReplicaProgressReceiptV1({...receipt,extra:true})).toBe(false);
  expect(replicaProgressRestartDueV1(undefined,receipt.at)).toBe(true);
  expect(replicaProgressRestartDueV1({...receipt,extra:true},receipt.at+policy.restartPauseMs)).toBe(false);
  for(const offset of[-1,0,policy.restartPauseMs-1,policy.restartPauseMs,policy.restartPauseMs+1]){
   const now=receipt.at+offset,independent=db.query("select ?1>=?2+?3 due").get(now,receipt.at,policy.restartPauseMs) as {due:number};
   expect(replicaProgressRestartDueV1(receipt,now)).toBe(Boolean(independent.due));
  }
 }finally{db.close();}
});
