/** 🔐️ Contract: a lock `mkdir` failure is contention (wait and retry) for `EEXIST` on every platform and for a Windows delete-pending `EPERM`/`EACCES`; everything else is a real failure. A SQLite oracle decides each case independently.
 * @see ../../💾️store/🟦️.ts */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {schedulerLockContendedV1} from "../../💾️store/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

test("lock mkdir failures are contention exactly for EEXIST and Windows delete-pending codes",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  const oracle=db.query<{contended:number},[string,string]>("select ?1='EEXIST' or (?2='win32' and ?1 in ('EPERM','EACCES')) contended");
  for(const row of fixture.cases){
   const expected=oracle.get(row.code,row.platform)!.contended===1;
   expect(expected,`${row.platform}/${row.code}`).toBe(row.contended);
   expect(schedulerLockContendedV1(row.code,row.platform as NodeJS.Platform),`${row.platform}/${row.code}`).toBe(expected);
  }
 }finally{db.close();}
});
