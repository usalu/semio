import Ajv from "ajv";
import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import type {GraphStateV1} from "../../../🟦️.ts";
import {PROJECT_RECORD_UNSENT_POLICY_V1 as policy,unsentBytesV1,unsentDropV1,unsentListV1,unsentPruneV1,unsentPutV1,unsentReadV1} from "../🟦️.ts";

/** 📮️ Contract: a finished record that was not stored is kept on disk whole or not at all, read back verified, dropped when old or when the bounds are passed (oldest first), and a record above the byte bound is not kept. */
const scratch:string[]=[],ajv=new Ajv({strict:false}),temp=()=>{const path=mkdtempSync(join(tmpdir(),"unsent-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const state=(label:string):GraphStateV1=>({version:1,frame:{kind:"project-inference",tooling:"0".repeat(64),implementation:"0".repeat(64),platform:"neutral",extra:{}},names:"0".repeat(64),content:"0".repeat(64),files:0,contentFiles:0,digest:createHash("sha256").update(label).digest("hex")}) as unknown as GraphStateV1;
const docs=(size:number)=>[{name:"record",bytes:new Uint8Array(size).fill(7)},{name:"closure-a",bytes:new Uint8Array(size).fill(9)}];

test("the policy and a stored description are valid against the schema",()=>{
 expect(ajv.validate({$ref:"#/definitions/Publish",definitions:schema.definitions},schema.definitions.Publish.const)).toBe(true);
 const directory=temp();expect(unsentPutV1(directory,{state:state("a"),owner:"o",documents:docs(10),cause:"x",attempts:1,now:1000})).toBe(true);
 const read=unsentReadV1(directory,state("a").digest)!;expect(ajv.validate({$ref:"#/definitions/UnsentMeta",definitions:schema.definitions},read.meta)).toBe(true);
});

test("a record is read back with its documents verified",()=>{
 const directory=temp(),entry=state("b");
 expect(unsentPutV1(directory,{state:entry,owner:"o",documents:docs(100),cause:"no answer",attempts:4,now:5})).toBe(true);
 const read=unsentReadV1(directory,entry.digest)!;
 expect(read.documents.map(row=>[row.name,row.bytes.length])).toEqual([["record",100],["closure-a",100]]);expect(read.meta.attempts).toBe(4);expect(unsentBytesV1(directory)).toBeGreaterThan(200);
 writeFileSync(join(directory,entry.digest,"1.bin"),new Uint8Array(100).fill(1));
 expect(unsentReadV1(directory,entry.digest)).toBeUndefined();
 unsentDropV1(directory,entry.digest);expect(unsentListV1(directory)).toEqual([]);
});

test("a put again keeps the first time and replaces the documents",()=>{
 const directory=temp(),entry=state("c");
 unsentPutV1(directory,{state:entry,owner:"o",documents:docs(10),cause:"a",attempts:1,now:100});
 unsentPutV1(directory,{state:entry,owner:"o",documents:docs(20),cause:"b",attempts:2,now:900});
 const read=unsentReadV1(directory,entry.digest)!;expect(read.meta.createdMs).toBe(100);expect(read.meta.attempts).toBe(2);expect(read.documents[0]!.bytes.length).toBe(20);expect(unsentListV1(directory).length).toBe(1);
});

test("old records go first, then the oldest ones until the entry bound holds, and a record above the byte bound is not kept",()=>{
 const directory=temp(),small={...policy,maximumEntries:3,maximumBytes:1000,maximumAgeMilliseconds:500};
 for(let index=0;index<3;index++)unsentPutV1(directory,{state:state("p"+index),owner:"o",documents:[{name:"record",bytes:new Uint8Array(10)}],cause:"c",attempts:1,now:1000+index},small);
 unsentPutV1(directory,{state:state("p3"),owner:"o",documents:[{name:"record",bytes:new Uint8Array(10)}],cause:"c",attempts:1,now:1003},small);
 expect(unsentListV1(directory).length).toBe(3);expect(unsentListV1(directory).some(row=>row.digest===state("p0").digest)).toBe(false);
 expect(unsentPruneV1(directory,1003+600,0,small)).toBe(3);expect(unsentListV1(directory)).toEqual([]);
 expect(unsentPutV1(directory,{state:state("big"),owner:"o",documents:[{name:"record",bytes:new Uint8Array(1001)}],cause:"c",attempts:1,now:1},small)).toBe(false);
});

test("the byte bound drops the oldest records to make room",()=>{
 const directory=temp(),small={...policy,maximumEntries:10,maximumBytes:300,maximumAgeMilliseconds:100000};
 for(let index=0;index<3;index++)unsentPutV1(directory,{state:state("q"+index),owner:"o",documents:[{name:"record",bytes:new Uint8Array(100)}],cause:"c",attempts:1,now:10+index},small);
 unsentPutV1(directory,{state:state("q3"),owner:"o",documents:[{name:"record",bytes:new Uint8Array(100)}],cause:"c",attempts:1,now:20},small);
 const kept=unsentListV1(directory).map(row=>row.digest);
 expect(kept.length).toBe(3);expect(kept.includes(state("q0").digest)).toBe(false);expect(kept.includes(state("q3").digest)).toBe(true);
 utimesSync(directory,new Date(),new Date());
});

test("the default spool keeps two waves of finished records whole, not the last 24 of them",()=>{
 const directory=temp();
 for(let index=0;index<96;index++)unsentPutV1(directory,{state:state("w"+index),owner:"o",documents:[{name:"record",bytes:new Uint8Array(10)}],cause:"c",attempts:1,now:1000+index});
 const oracle=spawnSync("python3",["-I","-c","import os,sys;print(len(os.listdir(sys.argv[1])))",directory],{encoding:"utf8"});
 expect(Number(oracle.stdout.trim())).toBe(96);expect(unsentListV1(directory).length).toBe(96);expect(unsentPruneV1(directory,1096)).toBe(0);
});
