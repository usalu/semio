import{afterAll,expect,test}from"bun:test";
import{closeSync,fstatSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,readSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync,writeSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,relative}from"node:path";
import{runToolSyncV1}from"../../../../../../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts";
import{parseMemberIndexV1,recordMemberLeafV1}from"../../🟨️.mjs";
import storage from"../../../../../../../../../../🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"with{type:"json"};

const scan=await import(new URL("../../🔎️scan/🟦️.ts",import.meta.url).href) as typeof import("../../🔎️scan/🟦️.ts");
const roots:string[]=[],oracle=join(import.meta.dir,"🐍️.py"),policy=storage.definitions.OptionalQuota.definitions.DefaultPolicy.const,HOUR=3600000;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const control=(cancelled=()=>false)=>({cancelled,progress:()=>{}});
const hex=(seed:number,length:number)=>Array.from({length},(_,i)=>"0123456789abcdef"[(seed*7+i*13+(seed>>3))%16]).join("");

type Spec={group:string;leaves:readonly{name:string;bytes:number}[];ageMs:number};
/** 🧪️ An image root whose groups hold the given optional leaves (indexed by the real member recorder) plus a mandatory receipt each; every leaf of a group carries the group's age. */
function image(specs:readonly Spec[],now:number):string{
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"lru-"));roots.push(root);
 for(const spec of specs){const base=join(root,"inference",...(spec.group==="canonical"?[]:spec.group.split("/")));
  for(const leaf of spec.leaves){const path=join(base,leaf.name);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,Buffer.alloc(leaf.bytes,1));const at=(now-spec.ageMs)/1000;utimesSync(path,at,at);recordMemberLeafV1({openSync,fstatSync,readSync,writeSync,closeSync,lstatSync,readFileSync},root,path);}
  mkdirSync(join(base,"inference"),{recursive:true});writeFileSync(join(base,"inference","emoji-project-json.receipt.json"),"{}");}
 return root;
}
function files(root:string):string[]{const out:string[]=[];const walk=(at:string)=>{for(const entry of readdirSync(join(root,at),{withFileTypes:true})){const name=at?at+"/"+entry.name:entry.name;if(entry.isDirectory())walk(name);else out.push(name);}};walk("");return out.sort();}
const leasePort=(busy:ReadonlySet<string>,taken:string[],released:{count:number})=>({lease:async(kind:"selection"|"dataset"|"canonical",identity:string)=>{const group=kind==="canonical"?"canonical":(kind==="dataset"?"contexts/":"selections/")+identity;taken.push(group);return busy.has(group)?undefined:{release:()=>{released.count++;}};}});
const leaf=(index:number,kind:number)=>kind===0?"np/"+hex(index,24)+".json":kind===1?"np/ob/"+hex(index+101,64)+".br":kind===2?"mr/k"+index+"-"+index+"-"+hex(index+7,16)+".idx":"mr/k"+index+"-"+index+"-"+hex(index+7,16)+".pack";

test("an over-limit optional cache retires least recently written groups down to the target, never busy, in-use or excluded ones, and keeps mandatory files and a compacted index",async()=>{
 const now=Date.now(),specs:Spec[]=[
  {group:"contexts/"+"a".repeat(64),leaves:[{name:leaf(1,0),bytes:4000},{name:leaf(1,1),bytes:3000}],ageMs:30*HOUR},
  {group:"selections/"+"b".repeat(64),leaves:[{name:leaf(2,0),bytes:2000},{name:leaf(2,2),bytes:100},{name:leaf(2,3),bytes:900}],ageMs:20*HOUR},
  {group:"contexts/"+"c".repeat(64),leaves:[{name:leaf(3,0),bytes:5000}],ageMs:10*HOUR},
  {group:"canonical",leaves:[{name:leaf(4,0),bytes:1000}],ageMs:5*HOUR},
  {group:"selections/"+"d".repeat(64),leaves:[{name:leaf(5,0),bytes:6000}],ageMs:60000}],root=image(specs,now),taken:string[]=[],released={count:0};
 const report=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(["contexts/"+"c".repeat(64)]),taken,released),control(),{targetBytes:9000,now});
 expect(report).toMatchObject({usedBytes:22000,maximumBytes:policy.maximumBytes,targetBytes:9000,retiredGroups:3,retiredFiles:6,retiredBytes:11000,remainingBytes:11000,busy:1,protected:1,excluded:0,cancelled:false});
 expect(taken).toEqual(["contexts/"+"a".repeat(64),"selections/"+"b".repeat(64),"contexts/"+"c".repeat(64),"canonical"]);expect(released.count).toBe(3);
 const left=files(root);
 for(const spec of specs){const prefix="inference/"+(spec.group==="canonical"?"":spec.group+"/"),gone=["contexts/"+"a".repeat(64),"selections/"+"b".repeat(64),"canonical"].includes(spec.group);
  for(const row of spec.leaves)expect(left.includes(prefix+row.name)).toBe(!gone);
  expect(left.includes(prefix+"inference/emoji-project-json.receipt.json")).toBe(true);
  const index=parseMemberIndexV1(readFileSync(join(root,prefix,"members.jsonl"),"utf8"),spec.group);expect(index.valid).toBe(true);expect(index.rows.size).toBe(gone?0:spec.leaves.length);}
 const again=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(),[],{count:0}),control(),{targetBytes:9000,now});
 expect(again).toMatchObject({usedBytes:11000,retiredGroups:1,retiredBytes:5000,remainingBytes:6000,protected:0});
 const fresh=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(),[],{count:0}),control(),{targetBytes:0,now});
 expect(fresh).toMatchObject({usedBytes:6000,retiredGroups:0,protected:1,remainingBytes:6000});
});

test("a dry run plans without leases or removals, the own namespace is excluded and a cancelled pass stops before the next group",async()=>{
 const now=Date.now(),specs:Spec[]=[{group:"contexts/"+"e".repeat(64),leaves:[{name:leaf(6,0),bytes:700}],ageMs:9*HOUR},{group:"selections/"+"f".repeat(64),leaves:[{name:leaf(7,0),bytes:800}],ageMs:8*HOUR},{group:"contexts/"+"0".repeat(64),leaves:[{name:leaf(8,0),bytes:900}],ageMs:7*HOUR}],root=image(specs,now),before=files(root),taken:string[]=[];
 const plan=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(),taken,{count:0}),control(),{targetBytes:1000,dryRun:true,exclude:["contexts/"+"e".repeat(64)],now});
 expect(plan).toMatchObject({usedBytes:2400,retiredGroups:2,retiredBytes:1700,remainingBytes:700,excluded:1,retiredFiles:0});expect(taken).toEqual([]);expect(files(root)).toEqual(before);
 let calls=0;const cut=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(),[],{count:0}),control(()=>calls++>0),{targetBytes:0,now});
 expect(cut).toMatchObject({retiredGroups:1,cancelled:true,remainingBytes:1700});
 const within=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(),taken,{count:0}),control(),{now});
 expect(within).toMatchObject({usedBytes:1700,targetBytes:policy.lowWaterBytes,retiredGroups:0,remainingBytes:1700});expect(taken).toEqual([]);
 await expect(scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(new Set(),[],{count:0}),control(),{targetBytes:-1})).rejects.toThrow("exact-optional-graph-target-bytes-required");
});

test("the retired groups and the remaining bytes equal the independent Python model on 120 seeded caches",async()=>{
 const now=Math.floor(Date.now()/1000)*1000,cases:unknown[]=[],actual:unknown[]=[];let seed=17;const random=(n:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%n;};
 for(let index=0;index<120;index++){
  const count=1+random(6),specs:Spec[]=[],busy=new Set<string>(),exclude:string[]=[];
  for(let g=0;g<count;g++){const group=g===0&&random(2)?"canonical":(random(2)?"contexts/":"selections/")+hex(index*16+g,64);if(specs.some(spec=>spec.group===group))continue;specs.push({group,leaves:[...new Map(Array.from({length:1+random(3)},(_,l)=>({name:leaf(index*64+g*8+l,random(4)),bytes:1+random(3000)})).map(row=>[row.name,row])).values()],ageMs:random(3)===0?1000*random(500):HOUR+random(40)*HOUR});if(random(5)===0)busy.add(group);if(random(7)===0)exclude.push(group);}
  const targetBytes=random(6000),root=image(specs,now),before=new Set(specs.flatMap(spec=>spec.leaves.map(row=>spec.group+"|"+row.name)));
  const report=await scan.retireBuildGraphOptionalGroupsLruV1(root,leasePort(busy,[],{count:0}),control(),{targetBytes,exclude,now});
  const left=new Set(files(root)),retired=specs.filter(spec=>spec.leaves.every(row=>!left.has("inference/"+(spec.group==="canonical"?"":spec.group+"/")+row.name))).map(spec=>spec.group).sort();
  expect(before.size).toBe(specs.reduce((sum,spec)=>sum+spec.leaves.length,0));
  cases.push({targetBytes,now,protectedMs:600000,busy:[...busy],exclude,groups:specs.map(spec=>({group:spec.group,bytes:spec.leaves.reduce((sum,row)=>sum+row.bytes,0),newest:now-spec.ageMs}))});
  actual.push({retired,remainingBytes:report.remainingBytes});
  rmSync(root,{recursive:true,force:true});
 }
 const file=join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"lru-cases-"+process.pid+".json");writeFileSync(file,JSON.stringify({cases}));
 try{const run=runToolSyncV1("python",["-I",oracle,file],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);expect(actual).toEqual(JSON.parse(run.stdout));}finally{rmSync(file,{force:true});}
},120000);
