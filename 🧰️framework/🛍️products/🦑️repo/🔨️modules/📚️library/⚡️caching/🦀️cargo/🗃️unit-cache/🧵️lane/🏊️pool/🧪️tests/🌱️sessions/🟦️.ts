/** 🌱 Contract: the pool bounds idle lanes by their bytes without their Cargo incremental sessions (the collector bounds those by its incremental share), so a lane that keeps an edited crate's incremental state is not evicted for holding it, while the files of idle lanes still obey the quota least recently used first; the collector records the session part beside every lane in the schema's `LaneSize`, and an independent Python walk agrees on bytes and victims.
 * @see ../../🟦️.ts
 * @see ../../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../../../🧬️schema/🔣️.json" with {type:"json"};
import {enterLaneV1,lanePathsV1,listLanesV1} from "../../../🟦️.ts";
import {readPoolV1,trimPoolV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

test("incremental sessions count against the collector's incremental share, not against the pool quota of the lane that keeps them, and the independent walk agrees",async()=>{
 const validate=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/definitions/LaneSize`)!;
 for(const row of fixture.cases){
  const root=mkdtempSync(join(tmpdir(),"ls-"));
  try{
   for(const lane of fixture.lanes){
    const entered=enterLaneV1(root,lane.id,undefined,lane.crates);entered.release();const paths=lanePathsV1(root,lane.id);
    mkdirSync(join(paths.target,"prototype"),{recursive:true});writeFileSync(join(paths.target,"prototype","lib.rlib"),Buffer.alloc(lane.fileBytes));
    if(lane.sessionBytes>0){const session=join(paths.build,"prototype","incremental",lane.crates[0]!.replaceAll("-","_")+"-0abc","s-1-x");mkdirSync(session,{recursive:true});writeFileSync(join(session,"query-cache.bin"),Buffer.alloc(lane.sessionBytes));}
    await new Promise(done=>setTimeout(done,15));
   }
   const measured=listLanesV1(root,Date.now());
   for(const lane of fixture.lanes){const size=JSON.parse(readFileSync(lanePathsV1(root,lane.id).size,"utf8"));expect(validate(size),JSON.stringify(size)).toBe(true);expect(size.incrementalBytes).toBe(lane.sessionBytes);expect(measured.find(item=>item.id===lane.id)!.incrementalBytes).toBe(lane.sessionBytes);}
   const oracle=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),root,fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);
   const expected=JSON.parse(oracle.stdout) as {bytes:Record<string,number>;trims:Record<string,string[]>};
   expect(Object.fromEntries(readPoolV1(root,Date.now()).map(lane=>[lane.id,lane.bytes]))).toEqual(expected.bytes);
   const before=readPoolV1(root,Date.now()).map(lane=>lane.id);trimPoolV1(root,{maximumLanes:16,maximumLiveLanes:32,quotaBytes:row.quotaBytes},Date.now());const removed=before.filter(id=>!readPoolV1(root,Date.now()).some(lane=>lane.id===id));
   expect(removed,row.id).toEqual(row.expect);expect(expected.trims[row.id]).toEqual(removed);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});
