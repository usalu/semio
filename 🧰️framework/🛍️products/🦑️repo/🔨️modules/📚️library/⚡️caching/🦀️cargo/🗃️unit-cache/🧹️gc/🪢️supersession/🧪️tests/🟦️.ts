import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CAS_POLICY_V1} from "../../../../../../🏗️build/🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {slotIdV1} from "../../../../../../🏗️build/🪢️slots/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {claimUnitV1,scanUnitSlotsV1,supersedeUnitsV1} from "../🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout,python=testPythonV1(),oracle=join(import.meta.dir,"../../../../../../🏗️build/🪢️slots/🧪️tests/🐍️oracle/🐍️.py"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`unit-slot-${label}-`));scratch.push(path);return path;};
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
const slot=(name:string)=>slotIdV1("rust-unit",["tc","aarch64-apple-darwin","dev",name,name,"lib/link","",""]);
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

type Seed={key:string;slot:string|null;createdMs:number;objects:string[];variants?:number};
function writeEntry(root:string,seed:Seed,kind=layout.entries):void{
 const directoryPath=join(root,kind,seed.key.slice(0,2),seed.key);mkdirSync(directoryPath,{recursive:true});
 for(let variant=0;variant<(seed.variants??1);variant++){
  const created=seed.createdMs+variant,entry={version:1,key:seed.key,crate:"c",dependencies:[],environment:[],outputs:seed.objects.map(object=>({name:"o",sha256:object,bytes:1,executable:false})),stderr:null,stdout:null,createdMs:created,...(seed.slot===null?{}:{slot:seed.slot})};
  writeFileSync(join(directoryPath,String(created).padStart(15,"0")+`-v${variant}.json`),JSON.stringify(entry));
 }
}
const keys=(root:string,kind=layout.entries)=>{const base=join(root,kind);try{return readdirSync(base).flatMap(shard=>readdirSync(join(base,shard))).sort();}catch{return [];}};

test.skipIf(!python)("supersession removes exactly the keys an independent Python model marks expired, over 200 seeded caches with variants and unslotted keys",async()=>{
 const next=seeded(20261113),requests:unknown[]=[],cases:{root:string;seeds:Seed[];now:number;grace:number}[]=[];
 for(let round=0;round<200;round++){
  const root=directory("rand"),count=Math.floor(next()*25),slots=1+Math.floor(next()*5),seeds:Seed[]=[];
  for(let index=0;index<count;index++){const key=sha(`${round}-${index}`);const seed={key,slot:next()<0.2?null:slot("s"+Math.floor(next()*slots)),createdMs:1_000_000+Math.floor(next()*(round%3===0?3:300))*10,objects:[],variants:1+Math.floor(next()*3)};seeds.push(seed);writeEntry(root,seed);}
  const now=1_000_000+Math.floor(next()*500)*10,grace=round%4===0?0:Math.floor(next()*20)*10;cases.push({root,seeds,now,grace});
  requests.push({op:"classify",rows:seeds.map(seed=>({key:seed.key,slot:seed.slot,storedAt:seed.createdMs+(seed.variants??1)-1})),now,grace});
 }
 const result=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString()) as Record<string,[string,number|null,number|null]>[];let removedTotal=0;
 for(const[index,round]of cases.entries()){
  const expected=Object.entries(model[index]!).filter(([,verdict])=>verdict[0]==="expired").map(([key])=>key).sort();
  const dry=await supersedeUnitsV1(round.root,{nowMs:round.now,graceMilliseconds:round.grace,dryRun:true});
  expect(keys(round.root).length,"a dry run removes nothing").toBe(round.seeds.length);
  const applied=await supersedeUnitsV1(round.root,{nowMs:round.now,graceMilliseconds:round.grace});
  expect(dry.removedKeys,`round ${index}`).toBe(expected.length);
  expect(applied.removedKeysList.map(key=>key.split("/")[2]).sort(),`round ${index}`).toEqual(expected);
  expect(keys(round.root).sort(),`round ${index}`).toEqual(round.seeds.map(seed=>seed.key).filter(key=>!expected.includes(key)).sort());
  removedTotal+=expected.length;
  const again=await supersedeUnitsV1(round.root,{nowMs:round.now,graceMilliseconds:round.grace});expect(again.removedKeys,"idempotent").toBe(0);
 }
 expect(removedTotal).toBeGreaterThan(100);
});

test("objects only superseded entries referenced go back to the host store; shared, pinned and workspace-linked ones stay",async()=>{
 const root=directory("objects"),cas=directory("cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}});
 const put=async(text:string)=>(await host.put(new TextEncoder().encode(text))).hash;
 const own=await put("own"),shared=await put("shared"),pinned=await put("pinned"),current=await put("current"),unslotted=await put("unslotted");
 await host.pin("holder",[pinned],60_000);
 writeEntry(root,{key:sha("a1"),slot:slot("a"),createdMs:1000,objects:[own,shared,pinned]});
 writeEntry(root,{key:sha("a2"),slot:slot("a"),createdMs:2000,objects:[shared,current]});
 writeEntry(root,{key:sha("u1"),slot:null,createdMs:500,objects:[unslotted]});
 const result=await supersedeUnitsV1(root,{nowMs:10_000,graceMilliseconds:100,host});
 expect(result.removedKeys).toBe(1);expect(result.reclaimed).toMatchObject({entries:1,pinned:1});
 expect(existsSync(host.objectPath(own))).toBe(false);for(const kept of[shared,pinned,current,unslotted])expect(existsSync(host.objectPath(kept))).toBe(true);
 expect(keys(root)).toEqual([sha("a2"),sha("u1")].sort());
});

test("a build run entry in the runs directory is superseded like a unit entry",async()=>{
 const root=directory("runs");
 writeEntry(root,{key:sha("r1"),slot:slot("run"),createdMs:1000,objects:[]},layout.runs);writeEntry(root,{key:sha("r2"),slot:slot("run"),createdMs:2000,objects:[]},layout.runs);
 const result=await supersedeUnitsV1(root,{nowMs:5000,graceMilliseconds:100});expect(result.removedKeys).toBe(1);expect(keys(root,layout.runs)).toEqual([sha("r2")]);
});

test("within the grace nothing goes; a claim on the older key after a revert makes it current and the newer one goes",async()=>{
 const root=directory("claim");mkdirSync(join(root,layout.temporary),{recursive:true});
 writeEntry(root,{key:sha("k1"),slot:slot("c"),createdMs:1000,objects:[]});writeEntry(root,{key:sha("k2"),slot:slot("c"),createdMs:2000,objects:[]});
 expect((await supersedeUnitsV1(root,{nowMs:2050,graceMilliseconds:100,dryRun:true})).removedKeys,"inside the grace").toBe(0);
 expect(claimUnitV1(root,sha("k1"),{nowMs:5000,minimumAgeMilliseconds:0}),"rewrites k1 as stored at 5000").toBe(true);
 expect(claimUnitV1(root,sha("k1"),{nowMs:5001,minimumAgeMilliseconds:60_000}),"a hot key is not rewritten again").toBe(false);
 expect(claimUnitV1(root,sha("absent"),{nowMs:5000}),"a missing key").toBe(false);
 const rows=await scanUnitSlotsV1(root);expect(rows.find(row=>row.key.endsWith(sha("k1")))!.storedAt).toBe(5000);
 const result=await supersedeUnitsV1(root,{nowMs:6000,graceMilliseconds:100});expect(result.removedKeysList.map(key=>key.split("/")[2])).toEqual([sha("k2")]);expect(keys(root)).toEqual([sha("k1")]);
 expect(readdirSync(join(root,layout.entries,sha("k1").slice(0,2),sha("k1"))).length,"the claim leaves one file").toBe(1);
});

test("a crash at any step of a pass leaves entries that read whole and the next pass finishes",async()=>{
 const template=directory("kill");
 for(let slotIndex=0;slotIndex<3;slotIndex++)for(let index=0;index<3;index++)writeEntry(template,{key:sha(`${slotIndex}-${index}`),slot:slot("k"+slotIndex),createdMs:1000+index*100,objects:[],variants:2});
 const reference=directory("kill-ref");cpSync(template,reference,{recursive:true});await supersedeUnitsV1(reference,{nowMs:9000,graceMilliseconds:50});const expected=keys(reference);expect(expected.length).toBe(3);
 let points=0;
 for(let kill=1;kill<=60;kill++){
  const root=directory("kill-"+kill);cpSync(template,root,{recursive:true});let count=0,crashed=false;
  try{await supersedeUnitsV1(root,{nowMs:9000,graceMilliseconds:50,onStep:()=>{if(++count===kill)throw Error("simulated crash");}});}catch(error){crashed=true;expect((error as Error).message).toBe("simulated crash");}
  if(!crashed){points=kill-1;break;}
  for(const row of await scanUnitSlotsV1(root))for(const file of row.files)expect(JSON.parse(readFileSync(join(row.directory,file),"utf8")).key,`kill ${kill}`).toBe(row.directory.split("/").pop());
  for(const base of[join(root,layout.entries)])for(const shard of readdirSync(base))for(const key of readdirSync(join(base,shard)))if(readdirSync(join(base,shard,key)).length===0)utimesSync(join(base,shard,key),1,1);
  await supersedeUnitsV1(root,{nowMs:9000,graceMilliseconds:50});expect(keys(root),`kill ${kill}`).toEqual(expected);
 }
 expect(points,"six removed keys with two variants: 2 files + 1 directory step plus the leading step each").toBe(6*(1+2+1));
});

test("the collector pass runs at most once per interval and never fails",async()=>{
 const{supersedeLocalUnitsV1}=await import("../🟦️.ts"),root=directory("local"),cas=directory("local-cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}});
 writeEntry(root,{key:sha("l1"),slot:slot("l"),createdMs:1000,objects:[]});writeEntry(root,{key:sha("l2"),slot:slot("l"),createdMs:2000,objects:[]});
 expect(await supersedeLocalUnitsV1(root,1_000_000,undefined,{graceMilliseconds:100,host})).toBe(1);expect(keys(root)).toEqual([sha("l2")]);
 writeEntry(root,{key:sha("l3"),slot:slot("l"),createdMs:3000,objects:[]});
 expect(await supersedeLocalUnitsV1(root,1_000_100,undefined,{graceMilliseconds:100,host}),"inside the interval").toBe(0);
 expect(await supersedeLocalUnitsV1(root,1_400_000,undefined,{graceMilliseconds:100,host})).toBe(1);expect(keys(root)).toEqual([sha("l3")]);
 expect(await supersedeLocalUnitsV1(join(root,"missing","deeper"),9_000_000,undefined,{host}),"an unusable root is not an error").toBe(0);
});

test("a cache hit on a slotted key claims it: the lookup makes the key current, so a revert keeps the older state and drops the newer one",async()=>{
 const{lookupUnitV1}=await import("../../../💾️store/🟦️.ts"),root=directory("hit");mkdirSync(join(root,layout.temporary),{recursive:true});
 writeEntry(root,{key:sha("old"),slot:slot("hit"),createdMs:1000,objects:[]});writeEntry(root,{key:sha("new"),slot:slot("hit"),createdMs:Date.now()-120_000,objects:[]});
 const context={cwd:root,laneRoot:undefined,environment:{},stampDirectory:undefined};
 expect(lookupUnitV1(root,sha("old"),context)?.key).toBe(sha("old"));
 const claimed=(await scanUnitSlotsV1(root)).find(row=>row.key.endsWith(sha("old")))!;expect(Date.now()-claimed.storedAt).toBeLessThan(10_000);
 const result=await supersedeUnitsV1(root,{nowMs:Date.now()+1000,graceMilliseconds:100});
 expect(result.removedKeysList.map(key=>key.split("/")[2])).toEqual([sha("new")]);expect(keys(root)).toEqual([sha("old")]);
 writeEntry(root,{key:sha("plain"),slot:null,createdMs:1000,objects:[]});
 expect(lookupUnitV1(root,sha("plain"),context)?.key,"an unslotted entry is served and not rewritten").toBe(sha("plain"));
});
