import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {CAS_POLICY_V1} from "../../../../../../🏗️build/🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {declareV1} from "../../../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {publishUnitV1} from "../../../💾️store/🟦️.ts";
import {claimUnitV1,retireLocalStaleUnitsV1,retireStaleUnitsV1,scanUnitDeclarationsV1,unitWorkspaceOfV1} from "../🟦️.ts";

const layout=UNIT_CACHE_POLICY_V1.layout,scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`unit-stale-${label}-`));scratch.push(path);return path;};
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const [path,text] of Object.entries(files)){const full=join(root,path);mkdirSync(dirname(full),{recursive:true});writeFileSync(full,text);}};

type Seed=Readonly<{key:string;createdMs:number;source?:readonly [string,string];externs?:readonly string[];produces?:string;toolchain?:string;variants?:number;declared?:boolean}>;
function writeEntry(root:string,seed:Seed,kind=layout.entries):void{
 const place=join(root,kind,seed.key.slice(0,2),seed.key);mkdirSync(place,{recursive:true});
 for(let variant=0;variant<(seed.variants??1);variant++){
  const created=seed.createdMs+variant,retention=seed.declared===false?undefined:declareV1({kind:"rust-unit",toolchain:sha(seed.toolchain??"t1"),sources:seed.source===undefined?[]:[{path:seed.source[0],sha256:sha(seed.source[1])}],externs:(seed.externs??[]).map(sha)});
  const entry={version:1,key:seed.key,...retention===undefined?{}:{retention},crate:"c",dependencies:[],environment:[],outputs:[{name:"o",sha256:sha(seed.produces??"blob:"+seed.key),bytes:1,executable:false}],stderr:null,stdout:null,createdMs:created};
  writeFileSync(join(place,String(created).padStart(15,"0")+`-v${variant}.json`),JSON.stringify(entry));
 }
}
const keys=(root:string,kind=layout.entries)=>{const base=join(root,kind);try{return readdirSync(base).flatMap(shard=>readdirSync(join(base,shard))).sort();}catch{return [];}};
const k=(name:string)=>sha("key-"+name);
function generationA(root:string,tree:string):void{
 write(tree,{"core/lib.rs":"core v1","mid/lib.rs":"mid v1","other/lib.rs":"other v1"});
 writeEntry(root,{key:k("core"),createdMs:1000,source:["core/lib.rs","core v1"],externs:["blob:serde","blob:old"],produces:"blob:core"});
 writeEntry(root,{key:k("mid"),createdMs:1000,source:["mid/lib.rs","mid v1"],externs:["blob:core"],produces:"blob:mid"});
 writeEntry(root,{key:k("other"),createdMs:1000,source:["other/lib.rs","other v1"],externs:["blob:serde"],produces:"blob:other"});
 writeEntry(root,{key:k("serde"),createdMs:1000,produces:"blob:serde"});writeEntry(root,{key:k("old"),createdMs:1000,produces:"blob:old"});writeEntry(root,{key:k("plain"),createdMs:1000,declared:false});
}

test("generation A built, one crate edited: that unit and its dependents are retired, the others and the entry that declared nothing stay; dry-run and off remove nothing",async()=>{
 const root=directory("units"),tree=directory("tree");generationA(root,tree);
 const same=await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:1_000_000,graceMilliseconds:100});
 expect(same).toMatchObject({declared:5,live:5,removedKeys:0,wouldRemoveKeys:0});expect(keys(root)).toHaveLength(6);
 write(tree,{"core/lib.rs":"core v2"});
 const dry=await retireStaleUnitsV1(root,{workspace:tree,mode:"dry-run",nowMs:1_000_000,graceMilliseconds:100});
 expect(dry).toMatchObject({mode:"dry-run",removedKeys:0,wouldRemoveKeys:3,stale:{edited:1,extern:1},unreached:1});expect(dry.examples[0]).toContain("core");expect(keys(root),"a dry run removes nothing").toHaveLength(6);
 expect((await retireStaleUnitsV1(root,{workspace:tree,mode:"off",nowMs:1_000_000})).scanned,"off judges nothing").toBe(0);expect(keys(root)).toHaveLength(6);
 const applied=await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:1_000_000,graceMilliseconds:100});
 expect(applied.removedKeysList.map(key=>key.split("/")[2]).sort()).toEqual([k("core"),k("mid"),k("old")].sort());
 expect(keys(root).sort()).toEqual([k("other"),k("plain"),k("serde")].sort());
 expect((await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:1_000_000,graceMilliseconds:100})).removedKeys,"idempotent").toBe(0);
 expect(readFileSync(join(root,"retention.jsonl"),"utf8").trim().split("\n").length,"every judging pass is logged").toBe(4);
});

test("a deleted crate is stale, a key stored within the grace stays, an entry whose newest variant declared nothing is not judged, variants are removed together",async()=>{
 const root=directory("grace"),tree=directory("tree-grace");generationA(root,tree);rmSync(join(tree,"core","lib.rs"));
 writeEntry(root,{key:k("fresh"),createdMs:999_950,source:["core/lib.rs","core v1"]});writeEntry(root,{key:k("multi"),createdMs:1000,source:["mid/lib.rs","gone"],variants:3});
 const result=await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:1_000_000,graceMilliseconds:100});
 expect(result.stale).toMatchObject({deleted:1,edited:1});expect(keys(root)).toContain(k("fresh"));expect(keys(root)).toContain(k("plain"));expect(keys(root)).not.toContain(k("multi"));expect(result.removedFiles).toBeGreaterThanOrEqual(3+1);
 expect(await scanUnitDeclarationsV1(root).then(rows=>rows.find(row=>row.key.endsWith(k("fresh")))?.declaration?.sources[0]?.path)).toBe("core/lib.rs");
});

test("objects only retired entries referenced go back to the host store; shared, pinned and live ones stay",async()=>{
 const root=directory("objects"),tree=directory("tree-objects"),cas=directory("cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}});
 const put=async(text:string)=>(await host.put(new TextEncoder().encode(text))).hash;
 const own=await put("own"),shared=await put("shared"),pinned=await put("pinned"),current=await put("current");await host.pin("holder",[pinned],60_000);
 write(tree,{"a/lib.rs":"a v2","b/lib.rs":"b v1"});
 const entry=(key:string,source:readonly [string,string],objects:string[])=>{
  const place=join(root,layout.entries,key.slice(0,2),key);mkdirSync(place,{recursive:true});
  writeFileSync(join(place,"000000000001000-v.json"),JSON.stringify({version:1,key,retention:declareV1({kind:"rust-unit",toolchain:sha("t1"),sources:[{path:source[0],sha256:sha(source[1])}],externs:[]}),crate:"c",dependencies:[],environment:[],outputs:objects.map(object=>({name:"o",sha256:object,bytes:1,executable:false})),stderr:null,stdout:null,createdMs:1000}));
 };
 entry(k("a1"),["a/lib.rs","a v1"],[own,shared,pinned]);entry(k("b1"),["b/lib.rs","b v1"],[shared,current]);
 const result=await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:1_000_000,graceMilliseconds:100,host});
 expect(result.removedKeys).toBe(1);expect(result.reclaimed).toMatchObject({entries:1,pinned:1});
 expect(existsSync(host.objectPath(own))).toBe(false);for(const kept of[shared,pinned,current])expect(existsSync(host.objectPath(kept))).toBe(true);
 expect(keys(root)).toEqual([k("b1")]);
});

test("a build run entry in the runs directory is judged like a unit entry",async()=>{
 const root=directory("runs"),tree=directory("tree-runs");write(tree,{"r/build.rs":"r v2"});
 writeEntry(root,{key:k("r1"),createdMs:1000,source:["r/build.rs","r v1"]},layout.runs);writeEntry(root,{key:k("r2"),createdMs:1000,source:["r/build.rs","r v2"]},layout.runs);
 const result=await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:5000,graceMilliseconds:100});expect(result.removedKeys).toBe(1);expect(keys(root,layout.runs)).toEqual([k("r2")]);
});

test("a crash at any step of a pass leaves entries that read whole and the next pass finishes",async()=>{
 const template=directory("kill"),tree=directory("tree-kill");write(tree,{"s/lib.rs":"v9"});
 for(let crate=0;crate<3;crate++)for(let index=0;index<3;index++)writeEntry(template,{key:k(`${crate}-${index}`),createdMs:1000+index*100,source:["s/lib.rs",index===2&&crate===0?"v9":"v"+index],variants:2});
 const reference=directory("kill-ref");cpSync(template,reference,{recursive:true});await retireStaleUnitsV1(reference,{workspace:tree,mode:"enforce",nowMs:9000,graceMilliseconds:50});const expected=keys(reference);expect(expected.length).toBe(1);
 let points=0;
 for(let kill=1;kill<=60;kill++){
  const root=directory("kill-"+kill);cpSync(template,root,{recursive:true});let count=0,crashed=false;
  try{await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:9000,graceMilliseconds:50,onStep:()=>{if(++count===kill)throw Error("simulated crash");}});}catch(error){crashed=true;expect((error as Error).message).toBe("simulated crash");}
  if(!crashed){points=kill-1;break;}
  for(const row of await scanUnitDeclarationsV1(root))for(const file of row.files)expect(JSON.parse(readFileSync(join(row.directory,file),"utf8")).key,`kill ${kill}`).toBe(row.directory.split("/").pop());
  for(const base of[join(root,layout.entries)])for(const shard of readdirSync(base))for(const key of readdirSync(join(base,shard)))if(readdirSync(join(base,shard,key)).length===0)utimesSync(join(base,shard,key),1,1);
  await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:9000,graceMilliseconds:50});expect(keys(root),`kill ${kill}`).toEqual(expected);
 }
 expect(points,"eight removed keys with two variants: the leading step, 2 files and 1 directory step each").toBe(8*(1+2+1));
});

test("the collector pass runs at most once per interval, in the mode of the policy, and never fails",async()=>{
 const root=directory("local"),tree=directory("tree-local"),cas=directory("local-cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}});
 write(tree,{"l/lib.rs":"l v2"});writeEntry(root,{key:k("l1"),createdMs:1000,source:["l/lib.rs","l v1"]});writeEntry(root,{key:k("l2"),createdMs:1000,source:["l/lib.rs","l v2"]});
 expect(UNIT_CACHE_POLICY_V1.gc.staleness.mode,"shipped in dry-run until the coordinator enables it").toBe("dry-run");
 expect(await retireLocalStaleUnitsV1(root,1_000_000,undefined,{graceMilliseconds:100,workspace:tree,host}),"dry-run by policy removes nothing").toBe(0);expect(keys(root)).toHaveLength(2);
 expect(await retireLocalStaleUnitsV1(root,1_400_000,undefined,{graceMilliseconds:100,workspace:tree,host,mode:"enforce"})).toBe(1);expect(keys(root)).toEqual([k("l2")]);
 writeEntry(root,{key:k("l3"),createdMs:1000,source:["l/lib.rs","l v0"]});
 expect(await retireLocalStaleUnitsV1(root,1_400_100,undefined,{graceMilliseconds:100,workspace:tree,host,mode:"enforce"}),"inside the interval").toBe(0);
 expect(await retireLocalStaleUnitsV1(root,9_000_000,undefined,{graceMilliseconds:100,workspace:tree,host,mode:"enforce"})).toBe(1);expect(keys(root)).toEqual([k("l2")]);
 expect(await retireLocalStaleUnitsV1(join(root,"missing","deeper"),9_000_000,undefined,{workspace:tree,host,mode:"enforce"}),"an unusable root is not an error").toBe(0);
 expect(unitWorkspaceOfV1(root,{SEMIO_UNIT_CACHE_WORKSPACE_ROOT:"/named"})).toBe("/named");
 mkdirSync(join(tree,".git"));expect(unitWorkspaceOfV1(join(tree,".cache","b","u"),{})).toBe(tree);
});

test("a hit on a key refreshes its stored time (claim), so a key in use stays inside the grace",async()=>{
 const{lookupUnitV1}=await import("../../../💾️store/🟦️.ts"),root=directory("hit"),tree=directory("tree-hit");mkdirSync(join(root,layout.temporary),{recursive:true});write(tree,{"h/lib.rs":"h v2"});
 const slot="a".repeat(64);
 const place=join(root,layout.entries,k("old").slice(0,2),k("old"));mkdirSync(place,{recursive:true});
 writeFileSync(join(place,"000000000001000-v.json"),JSON.stringify({version:1,key:k("old"),slot,retention:declareV1({kind:"rust-unit",toolchain:sha("t1"),sources:[{path:"h/lib.rs",sha256:sha("h v1")}],externs:[]}),crate:"c",dependencies:[],environment:[],outputs:[],stderr:null,stdout:null,createdMs:1000}));
 const context={cwd:root,laneRoot:undefined,environment:{},stampDirectory:undefined};
 expect(lookupUnitV1(root,k("old"),context)?.key).toBe(k("old"));
 const rows=await scanUnitDeclarationsV1(root);expect(Date.now()-rows[0]!.storedAt,"the lookup rewrote its stored time").toBeLessThan(10_000);
 expect((await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:Date.now()+1000,graceMilliseconds:60_000})).removedKeys,"stale but in use within the grace").toBe(0);
 expect((await retireStaleUnitsV1(root,{workspace:tree,mode:"enforce",nowMs:Date.now()+120_000,graceMilliseconds:60_000})).removedKeys,"stale once the grace passed").toBe(1);
 expect(claimUnitV1(root,k("absent"),{nowMs:5000}),"a missing key").toBe(false);
});

test("publishing a unit records its declaration: workspace sources with digests, the digests of the files it consumed and the toolchain, never registry files or generated roots",()=>{
 const root=directory("publish"),work=directory("work");
 write(work,{"crate/src/lib.rs":"fn a(){}","crate/src/util.rs":"fn b(){}",".cache/gen/out.rs":"generated"});const lib=join(work,"libc.rlib"),dep=join(work,"c.d");writeFileSync(lib,"compiled");
 writeFileSync(dep,`${lib}: crate/src/lib.rs crate/src/util.rs ${join(work,".cache/gen/out.rs")} ${join(directory("registry"),"serde.rs")}\n`);
 const entry=publishUnitV1(root,{key:"ab"+"0".repeat(62),crate:"c",cwd:work,outDir:work,laneRoot:undefined,outputs:[lib,dep],stdout:"",stderr:"",stampDirectory:undefined,retention:{workspace:work,externs:[sha("blob:dep"),"builtin","unit:x:y"],units:[sha("unit:dep"),"not-a-key","ab"+"0".repeat(62)],toolchain:"rustc 1.0"}});
 expect(entry?.retention).toEqual(declareV1({kind:"rust-unit",toolchain:"rustc 1.0",sources:[{path:"crate/src/lib.rs",sha256:sha("fn a(){}")},{path:"crate/src/util.rs",sha256:sha("fn b(){}")}],externs:[sha("blob:dep")],self:"ab"+"0".repeat(62),units:[sha("unit:dep")]}));
 expect(JSON.parse(readFileSync(join(root,layout.entries,"ab","ab"+"0".repeat(62),readdirSync(join(root,layout.entries,"ab","ab"+"0".repeat(62)))[0]!),"utf8")).retention.kind).toBe("rust-unit");
});
