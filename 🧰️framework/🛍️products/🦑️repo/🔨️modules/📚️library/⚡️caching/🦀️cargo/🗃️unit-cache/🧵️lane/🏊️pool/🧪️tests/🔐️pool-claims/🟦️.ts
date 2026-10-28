/** 🔐️ Contract: separate OS processes share one pool without ever sharing a lane. The newest free lane of the same crate set is taken again (warm), a different crate set gets a lane of its own, the idle lanes are trimmed to the bound, a trimmer running next to the workers removes no lane a worker holds, and a killed holder frees its lane.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {crateKeyV1,enterLaneV1,laneIdsV1,readLaneV1,recordLaneSizeV1} from "../../../🟦️.ts";
import {acquirePooledLaneV1,readPoolV1,trimPoolV1,type PoolReportV1} from "../../🟦️.ts";

const here=fileURLToPath(new URL(".",import.meta.url)),worker=resolve(here,"📥️worker/🟦️.ts"),base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});
const scratch=()=>mkdtempSync(join(base,"lp-")),big=Number.MAX_SAFE_INTEGER;
const run=(args:string[])=>new Promise<number|null>(done=>{const child=spawn(process.execPath,[worker,...args],{stdio:["ignore","inherit","inherit"]});child.once("close",code=>done(code));});

test("the lane of the same crate set is taken again, another crate set gets a lane of its own and the metadata names both",async()=>{
 const root=scratch();
 try{
  const first=await acquirePooledLaneV1(root,{crates:["a","b"],quotaBytes:big,owner:{session:"0123456789ab.p",agent:"alpha"}});
  expect(first.choice).toBe("new");const id=first.paths.id;first.release();
  const again=await acquirePooledLaneV1(root,{crates:["b","a","a"],quotaBytes:big});
  expect(again.choice).toBe("warm");expect(again.paths.id).toBe(id);
  const other=await acquirePooledLaneV1(root,{crates:["c"],quotaBytes:big});
  expect(other.choice).toBe("new");expect(other.paths.id).not.toBe(id);
  expect(readLaneV1(root,id)).toMatchObject({id,crateKey:crateKeyV1(["a","b"]),crates:["a","b"],session:"0123456789ab.p",agent:"alpha"});
  again.release();other.release();
  const warmC=await acquirePooledLaneV1(root,{crates:["c"],quotaBytes:big}),warmAB=await acquirePooledLaneV1(root,{crates:["a","b"],quotaBytes:big});
  expect([warmC.choice,warmAB.choice]).toEqual(["warm","warm"]);expect(warmC.paths.id).toBe(other.paths.id);expect(warmAB.paths.id).toBe(id);
  warmC.release();warmAB.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the reserved default lane is no part of the pool and is never handed out or trimmed",async()=>{
 const root=scratch();
 try{
  const reserved=UNIT_CACHE_POLICY_V1.pool.reserved[0]!,entered=enterLaneV1(root,reserved,undefined,["x"]);entered.release();recordLaneSizeV1(root,reserved,10**12,0,Date.now());
  expect(readPoolV1(root,Date.now())).toEqual([]);
  const lane=await acquirePooledLaneV1(root,{crates:["x"],quotaBytes:1});
  expect(lane.paths.id).not.toBe(reserved);lane.release();
  expect(laneIdsV1(root)).toContain(reserved);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a busy pool grows beyond its bound up to the live maximum and trims back when its lanes are idle",async()=>{
 const root=scratch(),messages:string[]=[],report:PoolReportV1=(kind,parameters)=>{messages.push(kind+":"+JSON.stringify(parameters));};
 try{
  const bound=UNIT_CACHE_POLICY_V1.pool.maximumLanes,held=[];
  for(let index=0;index<bound+2;index++)held.push(await acquirePooledLaneV1(root,{crates:["k"+index],quotaBytes:big},{report}));
  expect(new Set(held.map(row=>row.paths.id)).size).toBe(bound+2);
  expect(messages.filter(row=>row.startsWith("overflow:")).length).toBe(2);
  for(const row of held)row.release();
  expect(laneIdsV1(root).length).toBe(bound+2);
  const next=await acquirePooledLaneV1(root,{crates:["fresh"],quotaBytes:big},{report});
  expect(laneIdsV1(root).length).toBeLessThanOrEqual(bound);
  expect(messages.some(row=>row.startsWith("trimmed:"))).toBe(true);
  expect(existsSync(next.paths.target)).toBe(true);next.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the recorded bytes bound the idle lanes, least recently used first, and a held lane is never removed",async()=>{
 const root=scratch();
 try{
  const ids=["q1","q2","q3"];
  for(const [index,id] of ids.entries()){const entered=enterLaneV1(root,id,undefined,["x"+index]);entered.release();recordLaneSizeV1(root,id,400,0,Date.now());await new Promise(done=>setTimeout(done,15));}
  const held=enterLaneV1(root,"q1",undefined,["x0"]);
  const result=trimPoolV1(root,{maximumLanes:16,maximumLiveLanes:32,quotaBytes:900},Date.now());
  expect(result.removed).toBe(1);expect(laneIdsV1(root)).toEqual(["q1","q3"]);
  held.release();
  expect(readPoolV1(root,Date.now()).map(row=>[row.id,row.claimed,row.bytes])).toEqual([["q1",false,400],["q3",false,400]]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("six processes and a trimmer share a three-set pool: nobody is ever alone-less in a lane and no held lane vanishes",async()=>{
 const root=scratch();
 try{
  const sets=[["a"],["b","c"],["d"]],codes=await Promise.all([...sets,...sets].map(crates=>run(["work",root,"12",String(big),...crates])).concat([run(["trim",root,"60","2","1"])]));
  expect(codes.every(code=>code===0),JSON.stringify(codes)).toBe(true);
  const rows=readFileSync(join(root,"events.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line) as {kind:string;lane:string;choice:string;alone:boolean;present:boolean});
  expect(rows.length).toBe(72);
  expect(rows.filter(row=>!row.alone).length).toBe(0);
  expect(rows.filter(row=>!row.present).length).toBe(0);
  expect(rows.filter(row=>row.choice==="warm").length).toBeGreaterThan(0);
  expect(new Set(rows.map(row=>row.lane)).size).toBeGreaterThan(1);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("a killed holder frees its lane for the next process",async()=>{
 const root=scratch();
 try{
  const child=spawn(process.execPath,[worker,"die",root,"z"],{stdio:["ignore","pipe","inherit"]});
  await new Promise<void>(done=>child.stdout.once("data",()=>done()));
  const [held]=readPoolV1(join(root,"u"),Date.now());expect(held!.claimed).toBe(true);
  const busy=await acquirePooledLaneV1(join(root,"u"),{crates:["z"],quotaBytes:big});
  expect(busy.paths.id).not.toBe(held!.id);busy.release();
  child.kill("SIGKILL");await new Promise(done=>child.once("close",done));await new Promise(done=>setTimeout(done,1300));
  expect(readPoolV1(join(root,"u"),Date.now()).find(row=>row.id===held!.id)!.claimed).toBe(false);
  const first=await acquirePooledLaneV1(join(root,"u"),{crates:["z"],quotaBytes:big}),second=await acquirePooledLaneV1(join(root,"u"),{crates:["z"],quotaBytes:big});
  expect(new Set([first.paths.id,second.paths.id])).toEqual(new Set([held!.id,busy.paths.id]));
  expect([first.choice,second.choice]).toEqual(["warm","warm"]);first.release();second.release();
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);
