/** 📊️ Contract: the facts name the host volume and every mounted partition or native shard with free and total megabytes in a fixed line format, never throw, and the failure mirrors of the ready stages and of the service carry them.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {VOLUME_FACTS_POLICY_V1,formatVolumeFactsV1,readVolumeFactsV1,volumeFactsTextV1} from "../🟦️.ts";

test("the line format is exact and every row obeys the schema",()=>{
 const rows=[{path:"/w",totalBytes:256*1048576*1024,freeBytes:60*1048576*1024},{path:"/w/.cache/b/n/m/x/native/m",totalBytes:66060288,freeBytes:26501120}];
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);for(const row of rows)expect(ajv.validate("schema://repo.disk-recovery/VolumeFactsV1#/definitions/Row",row)).toBe(true);
 expect(formatVolumeFactsV1(rows).split("\n")).toEqual([VOLUME_FACTS_POLICY_V1.header,"/w: 61440 of 262144","/w/.cache/b/n/m/x/native/m: 25 of 63"]);
});

test("a row with an inode table names free and total inodes after the megabytes, and the host volume of this machine reports them",()=>{
 expect(formatVolumeFactsV1([{path:"/w",totalBytes:256*1048576*1024,freeBytes:60*1048576*1024,totalInodes:30000000,freeInodes:123215}]).split("\n")[1]).toBe("/w: 61440 of 262144, inodes 123215 of 30000000");
 const row=readVolumeFactsV1(process.cwd().split("/.🧬semio")[0]!)[0]!;
 expect(row.totalInodes===undefined||row.freeInodes!<=row.totalInodes).toBe(true);
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.validate("schema://repo.disk-recovery/VolumeFactsV1#/definitions/Row",row)).toBe(true);
});

test("the host volume of a workspace is read, plain folders are no mounts, a missing path never throws",()=>{
 const root=mkdtempSync(join(tmpdir(),"vf-"));
 try{
  expect(readVolumeFactsV1(root).map(row=>row.path)).toEqual([root]);
  mkdirSync(join(root,".cache","b","n","m"),{recursive:true});
  expect(readVolumeFactsV1(root).length).toBe(1);
  const text=volumeFactsTextV1(root);expect(text).toContain(root);expect(text).toContain("Plattenfakten");
  expect(volumeFactsTextV1(join(root,"missing"))).toBe(formatVolumeFactsV1([]));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the live machine facts agree with df for the host volume",()=>{
 const rows=readVolumeFactsV1(process.cwd().split("/.🧬semio")[0]!);
 const df=Bun.spawnSync(["df","-k",rows[0]!.path]).stdout.toString().trim().split("\n").at(-1)!.split(/\s+/);
 expect(Math.abs(rows[0]!.freeBytes/1024-Number(df[3]))/Number(df[3])).toBeLessThan(0.02);
});
