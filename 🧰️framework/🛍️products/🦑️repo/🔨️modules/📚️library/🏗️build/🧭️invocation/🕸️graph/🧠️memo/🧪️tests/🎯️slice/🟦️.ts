/** 🎯️ Contract: the graph memo of a served slice depends on exactly the facts the planner verified for that slice (the owners its pin covers, the environment owner, the supports of their derived facts). A dependency relink that only an owner OUTSIDE the slice read (its TypeScript analysis recorded the realpath into the package store) leaves the slice served from the memo, while the same relink refuses the slice of that owner, the planner re-infers only that owner and the memo of its next generation holds again. Oracles: the engine's own `observationsHold`, and the rows the compute port emitted per owner (independent of the fact index).
 * @see ../../📤️relevance/🟦️.ts `graphMemoSliceRowIdsV1` */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,unlinkSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,type GenerationRowV1,type GenerationSealedV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {encodeGenerationPinV1} from "../../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {createWorld,seed} from "../../../../../🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts";
import type {ChangeCursorV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {openGraphMemoV1,type MemoRecheckV1,type MemoRowV1} from "../../🟦️.ts";
import {buildGraphMemoRelevanceV1,writeGraphMemoSliceSidecarV1} from "../../📤️relevance/🟦️.ts";

const EPOCH="fedcba9876543210",exclude={rooted:[".gen",".leases"],names:["node_modules"]},insensitive=process.platform==="darwin"||process.platform==="win32";
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-slice-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));

/** 📓️ A journal that has seen nothing since its cursor: node_modules is excluded, so a relink below it is never a journal row. */
class QuietJournal implements ChangeJournalClientV1{
 excluder(){return(relative:string)=>relative.split("/").some(part=>exclude.names.includes(part))||exclude.rooted.some(path=>relative===path||relative.startsWith(path+"/"));}
 async cursor(){return{epoch:EPOCH,seq:1};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{return from.epoch===EPOCH?{exact:true,cursor:{epoch:EPOCH,seq:1},rows:[]}:{exact:false,reason:"epoch"};}
}

const hold=(rows:readonly unknown[])=>observation.observationsHold(rows as never,64,new Set(),true);
const realpathRow=(dir:string):GenerationRowV1=>{const path=join(dir,"node_modules/vitest");return{kind:"typescriptRealpath",path,fact:realpathSync.native(path)};};

/** 🌍️ Two independent owners `a` and `b`; `node_modules` links to a package store and only `b`'s analysis read the realpath of `node_modules/vitest`. */
async function workspace(){
 const dir=join(sandbox,"w"+counter++),emitted=new Map<string,GenerationRowV1[]>();
 await seed(dir,{"pkg/a/project.json":JSON.stringify({name:"a"}),"pkg/a/src/index.ts":"export const a=1;\n","pkg/b/project.json":JSON.stringify({name:"b"}),"pkg/b/src/index.ts":"export const b=2;\n","store/one/vitest/package.json":"{\"version\":\"1\"}\n","store/two/vitest/package.json":"{\"version\":\"2\"}\n"});
 symlinkSync("store/one",join(dir,"node_modules"));
 const world=createWorld(dir,{
  holdsWrap:base=>async(rows,signal)=>{const own=rows.filter(row=>row.kind==="typescriptRealpath");return await hold(own)&&base(rows.filter(row=>row.kind!=="typescriptRealpath"),signal);},
  computeWrap:base=>async(spec,signal)=>{const result=await base(spec,signal);for(const owner of result.owners){if(!owner.rows)continue;if(owner.name==="b")owner.rows=[...owner.rows,realpathRow(dir)];emitted.set(owner.name,owner.rows);}emitted.set(GENERATION_POLICY_V1.envOwner,result.env.rows);return result;},
 });
 return{dir,world,emitted,relink:(to:string)=>{unlinkSync(join(dir,"node_modules"));symlinkSync(to,join(dir,"node_modules"));}};
}

/** 📌️ The pin a planner child records for the slice `names` of a sealed generation. */
const pinOf=(current:GenerationSealedV1,names:ReadonlySet<string>)=>encodeGenerationPinV1({owner:1,generation:current.pointer.generation,manifest:current.pointer.manifest!,owners:Object.keys(current.manifest!.owners).sort(compareCodePointsV1),covered:names,atMs:Date.now()});

/** 🧾️ Writes the sidecar of one served slice and reads back its header and recheck set. */
async function sidecar(dir:string,pin:string|undefined){
 const stem=join(sandbox,"s"+counter++);
 await writeGraphMemoSliceSidecarV1({root:dir,generations:join(dir,".gen"),exclude,pin,stem,client:new QuietJournal(),insensitive});
 const header=JSON.parse(readFileSync(stem+".json","utf8")) as {rows:number;rechecked:number;unsealable:string|null};
 return{stem,header,recheck:header.unsealable===null?JSON.parse(readFileSync(stem+".chk","utf8")) as MemoRecheckV1:undefined};
}

const unique=(rows:Iterable<GenerationRowV1>)=>new Set([...rows].map(row=>canonicalJsonV1([row.kind,row.path,row.fact]))).size;

test("a relink only another owner's analysis read leaves the slice served; the reading owner's slice is refused, re-inferred alone, and holds again",async()=>{
 const {dir,world,emitted,relink}=await workspace();
 const cold=await world.refreshClosure(["a"]);expect(cold.published).toBe(true);
 const current=cold.current,sliceA=pinOf(current,new Set(["a"])),sliceB=pinOf(current,new Set(["b"]));
 const a=await sidecar(dir,sliceA),b=await sidecar(dir,sliceB);
 expect(a.header.unsealable).toBeNull();expect(b.header.unsealable).toBeNull();
 expect(a.header.rows,"oracle: the rows the compute emitted for a and the environment").toBe(unique([...emitted.get("a")!,...emitted.get(GENERATION_POLICY_V1.envOwner)!]));
 expect(b.header.rows,"oracle: the rows the compute emitted for b and the environment").toBe(unique([...emitted.get("b")!,...emitted.get(GENERATION_POLICY_V1.envOwner)!]));
 expect(a.recheck!.rows.some(row=>row.kind==="typescriptRealpath")).toBe(false);
 expect(b.recheck!.rows.some(row=>row.kind==="typescriptRealpath")).toBe(true);
 const index=(await world.store.readIndexView(current.manifest!.factIndex))!,frontier=(await world.store.readFrontier(current.manifest!.frontier))!;
 const whole=await buildGraphMemoRelevanceV1({root:dir,rows:()=>index.rows as unknown as MemoRowV1[],frontier,excluded:new QuietJournal().excluder(),isLink:async()=>false});

 relink("store/two");
 expect(await hold(whole.recheck.rows),"negative control: the whole-generation recheck (the memo before this law) refuses the slice of a").toBe(false);
 expect(await hold(a.recheck!.rows),"the slice of a never read the relinked package").toBe(true);
 expect(await hold(b.recheck!.rows),"the slice of b read it").toBe(false);

 const memo=(key:string)=>openGraphMemoV1({root:dir,directory:join(sandbox,"memo"),client:new QuietJournal(),key:key.repeat(64),insensitive,holds:rows=>hold(rows)});
 relink("store/one");
 for(const [key,pin,side] of [["a",sliceA,a],["b",sliceB,b]] as const){
  expect(await (await memo(key)).seal({plan:"{\"projects\":{}}",pin,projects:1})).toBe("light");
  expect(await (await memo(key)).seal({plan:"{\"projects\":{}}",pin,projects:1,sidecar:side.stem+".json"})).toBe("sealed");
 }
 relink("store/two");
 expect((await (await memo("a")).lookup()).hit,"served from the memo").toBe(true);
 expect(await (await memo("b")).lookup()).toEqual({hit:false,reason:"stale"});

 const keptA=await world.refreshClosure(["a"]);
 expect(keptA.built,"the planner does not re-infer a for a relink a never read").toBe(false);
 const rebuilt=await world.refreshClosure(["b"]);
 expect(rebuilt.built).toBe(true);expect(rebuilt.computed,"only the reading owner is re-inferred").toBe(1);
 const next=await sidecar(dir,pinOf(rebuilt.current,rebuilt.names));
 expect(next.recheck!.rows.find(row=>row.kind==="typescriptRealpath")?.fact,"the right fact is recorded afterwards").toBe(realpathSync.native(join(dir,"node_modules/vitest")));
 expect(await hold(next.recheck!.rows)).toBe(true);
},60000);

test("a sidecar without a pin of the served generation is unsealable",async()=>{
 const {dir,world}=await workspace();
 const cold=await world.refreshClosure(["a"]);
 expect((await sidecar(dir,undefined)).header.unsealable).toBe("pin");
 expect((await sidecar(dir,"{")).header.unsealable).toBe("pin");
 expect((await sidecar(dir,pinOf(cold.current,new Set(["a"])).replace(cold.current.pointer.manifest!,"f".repeat(64)))).header.unsealable).toBe("other-generation");
});
