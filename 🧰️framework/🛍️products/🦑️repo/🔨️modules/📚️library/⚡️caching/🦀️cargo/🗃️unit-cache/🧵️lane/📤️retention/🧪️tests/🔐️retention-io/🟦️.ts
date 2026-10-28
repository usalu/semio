/** 🔐️ Contract: the retention rule publishes the provenance of a lane compile before the lane can be trimmed, without ever replacing a file, without losing a file to an interrupted pass and without a trimmer or a second process being able to interleave with it. Real files, real processes, the real lane pool; the digests are checked by an independent implementation (`hashlib`).
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,truncateSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../../🔏️key/🟦️.ts";
import {acquirePooledLaneV1,readPoolV1,trimPoolV1} from "../../../🏊️pool/🟦️.ts";
import {retainLaneOutputsV1,withLaneRetentionV1} from "../../🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.retention,here=fileURLToPath(new URL(".",import.meta.url)),worker=resolve(here,"📥️worker/🟦️.ts"),base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),python=requireTestToolV1("python");
mkdirSync(base,{recursive:true});
const scratch=()=>mkdtempSync(join(base,"rt-")),R="cargo-unit-provenance-";
const sha=(path:string):string=>{const run=spawnSync(python,["-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",path],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return run.stdout.trim();};
const layout=(root:string)=>({lane:join(root,"lane"),shared:join(root,"shared"),laneLedger:join(root,"lane",rules.ledger),sharedLedger:join(root,"shared",rules.ledger)});
const seed=(directory:string,files:Record<string,string>)=>{mkdirSync(directory,{recursive:true});for(const [name,text] of Object.entries(files))writeFileSync(join(directory,name),text);};
const input=(root:string,lines?:string[])=>({lane:layout(root).lane,shared:layout(root).shared,laneId:"ab12",language:"en" as const,...lines?{report:(line:string)=>{lines.push(line);}}:{}});

test("the retention policy is complete: every field the module reads is authored, so no path is ever built from an undefined name",()=>{
 expect(Object.keys(rules).sort()).toEqual(["chunkBytes","ledger","ledgerPattern","maximumFileBytes","maximumFiles"]);
 for(const [name,value] of Object.entries(rules))expect(value,name).not.toBeUndefined();
 expect(typeof rules.ledger).toBe("string");expect(new RegExp(rules.ledgerPattern,"u").test("cargo-unit-provenance-x.json")).toBe(true);
});

test("a lane that wrote no provenance at all (a test or lint run, an empty lane) retains nothing, creates nothing and does not fail",async()=>{
 const root=scratch(),dir=layout(root);
 try{
  mkdirSync(dir.lane,{recursive:true});
  expect(await retainLaneOutputsV1(input(root))).toEqual({published:0,same:0,conflicts:[],refused:0,bytes:0});
  expect(existsSync(dir.shared)).toBe(false);
  mkdirSync(dir.laneLedger,{recursive:true});
  expect(await retainLaneOutputsV1(input(root))).toMatchObject({published:0,conflicts:[]});
  expect(existsSync(dir.sharedLedger)).toBe(false);
  expect(await retainLaneOutputsV1({...input(root),lane:join(root,"never-created")})).toMatchObject({published:0});
  let released=0;
  expect(await withLaneRetentionV1({lane:{paths:{id:"ab12",build:join(root,"never-created")},release(){released++;}},shared:join(root,"also-never"),language:"en"},async()=>"ok")).toBe("ok");expect(released).toBe(1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("receipts are published byte for byte, removed from the lane, and a second pass does nothing",async()=>{
 const root=scratch(),dir=layout(root);
 try{
  seed(dir.laneLedger,{[R+"a.json"]:'{"a":1}',[R+"b.json"]:'{"b":"é"}'});
  const before=new Map(readdirSync(dir.laneLedger).map(name=>[name,sha(join(dir.laneLedger,name))]));
  const summary=await retainLaneOutputsV1(input(root));
  expect(summary).toMatchObject({published:2,same:0,conflicts:[],refused:0});
  for(const [name,digest] of before)expect(sha(join(dir.sharedLedger,name))).toBe(digest);
  expect(readdirSync(dir.laneLedger)).toEqual([]);
  expect(readdirSync(dir.sharedLedger).filter(name=>name.endsWith(".tmp")||name.startsWith("."))).toEqual([]);
  expect(await retainLaneOutputsV1(input(root))).toMatchObject({published:0,same:0,conflicts:[],refused:0});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an interrupted pass is finished by the next one: a file already published is dropped from the lane, a stale temporary is left alone",async()=>{
 const root=scratch(),dir=layout(root);
 try{
  seed(dir.laneLedger,{[R+"a.json"]:'{"a":1}',[R+"b.json"]:'{"b":2}'});seed(dir.sharedLedger,{[R+"a.json"]:'{"a":1}',[".cargo-unit-provenance-b.json.dead.tmp"]:"partial"});
  const summary=await retainLaneOutputsV1(input(root));
  expect(summary).toMatchObject({published:1,same:1,conflicts:[]});
  expect(readdirSync(dir.laneLedger)).toEqual([]);
  expect(readFileSync(join(dir.sharedLedger,R+"b.json"),"utf8")).toBe('{"b":2}');
  expect(readFileSync(join(dir.sharedLedger,".cargo-unit-provenance-b.json.dead.tmp"),"utf8")).toBe("partial");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a name that exists with other bytes is never replaced, stays in the lane and is reported in both languages",async()=>{
 const root=scratch(),dir=layout(root);
 try{
  seed(dir.laneLedger,{[R+"a.json"]:"lane",[R+"b.json"]:"fresh"});seed(dir.sharedLedger,{[R+"a.json"]:"shared"});
  const lines:string[]=[],summary=await retainLaneOutputsV1(input(root,lines));
  expect(summary).toMatchObject({published:1,conflicts:[R+"a.json"]});
  expect(readFileSync(join(dir.sharedLedger,R+"a.json"),"utf8")).toBe("shared");expect(readFileSync(join(dir.laneLedger,R+"a.json"),"utf8")).toBe("lane");
  expect(readdirSync(dir.laneLedger)).toEqual([R+"a.json"]);
  expect(lines.length).toBe(1);expect(lines[0]).toContain("ab12");expect(lines[0]).toContain(R+"a.json");expect(lines[0]).toContain("provenance");
  const german:string[]=[];await retainLaneOutputsV1({...input(root),language:"de",report:line=>german.push(line)});
  expect(german[0]).toContain("Provenienz");expect(german[0]).toContain("ab12");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("unrelated names, symbolic links, directories and oversize files are never published",async()=>{
 const root=scratch(),dir=layout(root);
 try{
  seed(dir.laneLedger,{"notes.txt":"x",[".hidden.json"]:"x",[R+"ok.json"]:"ok"});
  writeFileSync(join(root,"outside"),"secret");symlinkSync(join(root,"outside"),join(dir.laneLedger,R+"link.json"));mkdirSync(join(dir.laneLedger,R+"dir.json"));
  writeFileSync(join(dir.laneLedger,R+"big.json"),"");truncateSync(join(dir.laneLedger,R+"big.json"),rules.maximumFileBytes+1);
  const summary=await retainLaneOutputsV1(input(root));
  expect(summary).toMatchObject({published:1,refused:1,conflicts:[]});
  expect(readdirSync(dir.sharedLedger)).toEqual([R+"ok.json"]);
  expect(readdirSync(dir.laneLedger).sort()).toEqual([".hidden.json",R+"big.json",R+"dir.json",R+"link.json","notes.txt"].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the lane is released only after its outputs are published, a failing body is still retained, and a failing retention after a good body is the error",async()=>{
 const root=scratch(),dir=layout(root);
 try{
  const events:string[]=[],lane={paths:{id:"ab12",build:dir.lane},release(){events.push("release:"+existsSync(join(dir.sharedLedger,R+"a.json")));}};
  seed(dir.laneLedger,{[R+"a.json"]:"a"});
  expect(await withLaneRetentionV1({lane,shared:dir.shared,language:"en"},async()=>{events.push("body");return 7;})).toBe(7);
  expect(events).toEqual(["body","release:true"]);
  seed(dir.laneLedger,{[R+"b.json"]:"b"});events.length=0;
  await expect(withLaneRetentionV1({lane,shared:dir.shared,language:"en"},async()=>{throw Error("compile failed");})).rejects.toThrow("compile failed");
  expect(readFileSync(join(dir.sharedLedger,R+"b.json"),"utf8")).toBe("b");expect(events.length).toBe(1);
  seed(dir.laneLedger,{[R+"c.json"]:"c"});rmSync(dir.shared,{recursive:true,force:true});writeFileSync(dir.shared,"a file where the shared directory should be");events.length=0;
  await expect(withLaneRetentionV1({lane,shared:dir.shared,language:"en"},async()=>1)).rejects.toThrow();
  expect(events.length).toBe(1);
  await expect(withLaneRetentionV1({lane,shared:dir.shared,language:"en"},async()=>{throw Error("body");})).rejects.toBeInstanceOf(AggregateError);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a pooled lane keeps its provenance through trimming: once released the trimmer may remove the lane and the shared ledger still holds every receipt",async()=>{
 const root=scratch(),dir=layout(root),cache=join(root,"u");
 try{
  const lane=await acquirePooledLaneV1(cache,{crates:["x"],quotaBytes:Number.MAX_SAFE_INTEGER});
  seed(join(lane.paths.build,rules.ledger),{[R+"x.json"]:"kept"});
  await withLaneRetentionV1({lane,shared:dir.shared,language:"en"},async()=>{expect(trimPoolV1(cache,{maximumLanes:0,maximumLiveLanes:32,quotaBytes:0},Date.now()).removed).toBe(0);});
  expect(trimPoolV1(cache,{maximumLanes:0,maximumLiveLanes:32,quotaBytes:0},Date.now()).removed).toBe(1);
  expect(readPoolV1(cache,Date.now())).toEqual([]);
  expect(readFileSync(join(dir.sharedLedger,R+"x.json"),"utf8")).toBe("kept");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("six processes release into one shared ledger beside a trimmer: every receipt arrives once with its bytes, identical names are one file, one racing name has exactly one winner",async()=>{
 const root=scratch(),dir=layout(root),workers=6,count=12;
 try{
  mkdirSync(dir.shared,{recursive:true});
  const run=(args:string[])=>new Promise<{code:number|null;out:string}>(done=>{const child=spawn(process.execPath,[worker,...args],{stdio:["ignore","pipe","inherit"]});let out="";child.stdout.on("data",chunk=>{out+=chunk;});child.once("close",code=>done({code,out}));});
  const results=await Promise.all([...Array.from({length:workers},(_,index)=>run(["lane",root,dir.shared,String(index),String(count)])),run(["trim",root,"80"])]);
  expect(results.every(result=>result.code===0),JSON.stringify(results)).toBe(true);
  const names=readdirSync(dir.sharedLedger).sort(),expected=[...Array.from({length:workers},(_,index)=>Array.from({length:count},(_,item)=>`${R}w${index}-${item}.json`)).flat(),R+"common.json",R+"race.json"].sort();
  expect(names).toEqual(expected);
  for(let index=0;index<workers;index++)for(let item=0;item<count;item++)expect(JSON.parse(readFileSync(join(dir.sharedLedger,`${R}w${index}-${item}.json`),"utf8"))).toMatchObject({worker:index,item});
  expect(JSON.parse(readFileSync(join(dir.sharedLedger,R+"common.json"),"utf8"))).toEqual({common:true});
  expect([0,1,2,3,4,5]).toContain(JSON.parse(readFileSync(join(dir.sharedLedger,R+"race.json"),"utf8")).winner);
  const leftovers=readPoolV1(join(root,"u"),Date.now()).flatMap(lane=>{try{return readdirSync(join(root,"u","l",lane.id,"b",rules.ledger));}catch{return[];}});
  expect(leftovers.filter(name=>name!==R+"race.json")).toEqual([]);
  expect(leftovers.length).toBeLessThanOrEqual(workers-1);
  expect(readdirSync(dir.sharedLedger).filter(name=>name.startsWith(".")||name.endsWith(".tmp"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
