import {expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createSidecarCodeMirrorV1,createSidecarCodeRecorderV1,readSidecarCodeRecordV1,sidecarCodeMirrorOfV1,sidecarCodeStoreFileV1} from "../../📇️code-record/🟦️.ts";

const SECOND=1000,HASH_A="a".repeat(64),HASH_B="b".repeat(64);

test("a role whose disk digest moves is stale from the first look on and current again when it restarts onto the new digest",()=>{
 const root=mkdtempSync(join(tmpdir(),"code-record-"));
 try{
  let clock=1_000_000;
  const recorder=createSidecarCodeRecorderV1({root,role:"agent",pid:7,startedMs:clock,now:()=>clock});
  expect(recorder.record(HASH_A,HASH_A)).toMatchObject({current:true,staleSinceMs:null,running:"a".repeat(12)});
  clock+=30*SECOND;
  expect(recorder.record(HASH_A,HASH_B)).toMatchObject({current:false,staleSinceMs:clock,disk:"b".repeat(12)});
  const since=clock;clock+=90*SECOND;
  expect(recorder.record(HASH_A,HASH_B).staleSinceMs,"the clock starts at the first look and is kept").toBe(since);
  expect(readSidecarCodeRecordV1(root,"agent")).toMatchObject({pid:7,current:false,staleSinceMs:since});
  const next=createSidecarCodeRecorderV1({root,role:"agent",pid:8,startedMs:clock,now:()=>clock});
  expect(next.record(HASH_B,HASH_B)).toMatchObject({current:true,pid:8});
  expect(readSidecarCodeRecordV1(root,"agent")).toMatchObject({pid:8,current:true,staleSinceMs:null});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the table of a machine counts a role whose record is older than two minutes as not current, and an empty table as not current",()=>{
 const base={version:1 as const,role:"agent",pid:1,startedMs:0,running:"a",disk:"a",current:true,staleSinceMs:null};
 const table=sidecarCodeMirrorOfV1({origin:"g1-m09",now:1_000_000,rows:{agent:{...base,atMs:990_000},ready:{...base,role:"ready",atMs:700_000},cache:undefined}});
 expect(table.roles.agent).toMatchObject({current:true,ageSeconds:10});
 expect(table.roles.ready).toMatchObject({current:false,ageSeconds:300});
 expect(Object.keys(table.roles)).toEqual(["agent","ready"]);
 expect(table.current).toBe(false);
 expect(sidecarCodeMirrorOfV1({origin:"g1-m09",now:1,rows:{}}).current).toBe(false);
 expect(sidecarCodeMirrorOfV1({origin:"g1-m09",now:1_000_000,rows:{agent:{...base,atMs:995_000}}}).current).toBe(true);
});

test("the mirror writes one file per origin into the store, only when something changed or a minute passed, and says nothing without a store",()=>{
 const root=mkdtempSync(join(tmpdir(),"code-record-")),store=mkdtempSync(join(tmpdir(),"code-store-"));
 try{
  let clock=2_000_000;
  const recorder=createSidecarCodeRecorderV1({root,role:"ready",pid:3,startedMs:clock,now:()=>clock});
  recorder.record(HASH_A,HASH_A);
  const mirror=createSidecarCodeMirrorV1({root,store,origin:"g1-m09",roles:["agent","ready"],now:()=>clock}),file=sidecarCodeStoreFileV1(store,"g1-m09");
  expect(mirror.tick()?.current).toBe(true);
  const first=readFileSync(file,"utf8");
  clock+=10*SECOND;mirror.tick();
  expect(readFileSync(file,"utf8"),"unchanged inside a minute").toBe(first);
  recorder.record(HASH_A,HASH_B);clock+=10*SECOND;
  expect(mirror.tick()?.current).toBe(false);
  expect(JSON.parse(readFileSync(file,"utf8")).roles.ready).toMatchObject({running:"a".repeat(12),disk:"b".repeat(12),current:false});
  expect(createSidecarCodeMirrorV1({root,store:undefined,origin:"g1-m09",roles:["ready"]}).tick()).toBeUndefined();
  expect(existsSync(join(store,"v1","origins","g1-m10"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(store,{recursive:true,force:true});}
});

test("an unchanged role is rewritten every minute, inside the two minutes after which the table calls a record stale",()=>{
 const root=mkdtempSync(join(tmpdir(),"code-record-"));
 try{
  let clock=5_000_000;
  const recorder=createSidecarCodeRecorderV1({root,role:"spool",pid:9,startedMs:clock,now:()=>clock});
  recorder.record(HASH_A,HASH_A);const first=readSidecarCodeRecordV1(root,"spool")!.atMs;
  clock+=30*SECOND;recorder.record(HASH_A,HASH_A);expect(readSidecarCodeRecordV1(root,"spool")!.atMs,"inside a minute nothing is written").toBe(first);
  clock+=31*SECOND;recorder.record(HASH_A,HASH_A);
  const second=readSidecarCodeRecordV1(root,"spool")!.atMs;
  expect(second-first).toBeLessThan(120*SECOND);expect(second).toBeGreaterThan(first);
  expect(sidecarCodeMirrorOfV1({origin:"g1-m09",now:second+100*SECOND,rows:{spool:readSidecarCodeRecordV1(root,"spool")}}).current).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
