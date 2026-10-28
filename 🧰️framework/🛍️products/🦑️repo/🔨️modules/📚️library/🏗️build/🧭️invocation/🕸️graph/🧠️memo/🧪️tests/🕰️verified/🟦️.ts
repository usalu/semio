/** 🕰️ Contract: the graph memo seals only a slice the planner verified after the memo's journal cursor. A slice a peer's verified stamp served (`covered`: the stamp is newer than the door's request time, but its verification may predate an edit that lands before the memo cursor) or a bound pin of an outer invocation served (`pinned`) was never checked against the tree of the cursor; sealing it vouches for a change the journal will never report again, and every later identical request is served the stale slice. Oracles: the planner itself (a fresh request of the same closure must get the graph the memo serves), and the pin the planner recorded (its coverage time must not be older than the cursor).
 * @see ../../🚪️command/🟦️.ts `planGraphWithMemoV1`
 * @see ../../📤️relevance/🟦️.ts `writeGraphMemoSliceSidecarV1`
 * @see ../../../../../🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs `coverageOfV1` */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {loadGenerationGraphV1} from "../../../../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs";
import {encodeGenerationPinV1} from "../../../../../🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
import {compareCodePointsV1} from "../../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {applyOps,createMockHost,seed} from "../../../../../🕸️graph/🕰️generations/🧪️tests/🏃️world/🟦️.ts";
import type {ChangeCursorV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import type {ChangeAnswerV1,ChangeJournalClientV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {GRAPH_MEMO_POLICY_V1,openGraphMemoV1} from "../../🟦️.ts";
import {planGraphWithMemoV1} from "../../🚪️command/🟦️.ts";
import {writeGraphMemoSliceSidecarV1} from "../../📤️relevance/🟦️.ts";

const EPOCH="0123456789abcdef",REQUESTED="SEMIO_BUILD_GRAPH_REQUESTED_AT",exclude={rooted:[".gen",".leases"],names:["node_modules"]},insensitive=process.platform==="darwin"||process.platform==="win32";
let sandbox="",counter=0;
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"memo-verified-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
const sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));

/** 📓️ An exact journal whose cursor advances with every call and that reports what the law changed after a cursor: every edit of this law happens before the next cursor is taken, so a faithful journal has nothing to report. */
class ExactJournal implements ChangeJournalClientV1{
 seq=0;
 excluder(){return(relative:string)=>relative.split("/").some(part=>exclude.names.includes(part))||exclude.rooted.some(path=>relative===path||relative.startsWith(path+"/"));}
 async cursor(){return{epoch:EPOCH,seq:++this.seq};}
 async since(from:ChangeCursorV1):Promise<ChangeAnswerV1>{return from.epoch===EPOCH?{exact:true,cursor:{epoch:EPOCH,seq:++this.seq},rows:[]}:{exact:false,reason:"epoch"};}
}

/** 🏗️ One planner child as `loadBuildGraphInChildV1` starts it: the environment of the parent plus what the memo asks for, the planner (`loadGenerationGraphV1` of the closure), the pin it records and, when asked, the sidecar of the slice it served. */
function child(dir:string,journal:ExactJournal,doorAtMs:number){
 return async(sidecar:Readonly<Record<string,string>>|undefined)=>{
  const environment:Record<string,string>={[REQUESTED]:String(doorAtMs),...sidecar},previous=process.env[REQUESTED];
  process.env[REQUESTED]=environment[REQUESTED];
  try{
   const mock=createMockHost(dir,{stamp:true});let pin:string|undefined;
   Object.assign(mock.host,{sealed:(record:{generation:number;manifest:string;owners:string[];covered:ReadonlySet<string>|undefined;atMs:number})=>{pin=encodeGenerationPinV1({owner:1,...record});}});
   const graph=await loadGenerationGraphV1(mock.host,{closure:["a"]});
   const out=environment[GRAPH_MEMO_POLICY_V1.sidecar];
   if(out!==undefined)await writeGraphMemoSliceSidecarV1({root:dir,generations:join(dir,".gen"),exclude,pin,stem:out.replace(/\.json$/u,""),expected:environment[GRAPH_MEMO_POLICY_V1.expect],client:journal,insensitive,verifiedAfter:Number(environment[REQUESTED])} as Parameters<typeof writeGraphMemoSliceSidecarV1>[0]);
   return{plan:JSON.stringify(graph),pin,said:mock.said.map(row=>row.key)};
  }finally{if(previous===undefined)delete process.env[REQUESTED];else process.env[REQUESTED]=previous;}
 };
}

/** 🗺️ One request through the memo exactly as the executor runs it, with the stem of a fresh plan directory. */
async function request(dir:string,journal:ExactJournal,doorAtMs:number){
 const memo=await openGraphMemoV1({root:dir,directory:join(sandbox,"memo"+dir.slice(-2)),client:journal,key:"a".repeat(64),insensitive,holds:rows=>observation.observationsHold(rows as never,64,new Set(),true)});
 const plan=child(dir,journal,doorAtMs),said:string[][]=[],marks:Record<string,string>={};
 const result=await planGraphWithMemoV1({memo,stem:join(sandbox,"stem"+counter++),parse:text=>JSON.parse(text) as {projects:Record<string,unknown>},record:()=>undefined,notice:()=>undefined,mark:(name,detail)=>{marks[name]=detail;},
  plan:async sidecar=>{const produced=await plan(sidecar);said.push(produced.said);return produced;}});
 return{...result,said,marks};
}

/** 🌍️ One owner `a`; the first request of the law starts at `doorAtMs`, a peer builds the generation afterwards and seals a verified stamp newer than that request. */
async function workspace(){
 const dir=join(sandbox,"w"+counter++);
 await seed(dir,{"pkg/a/project.json":JSON.stringify({name:"a",package:"@syn/a",uses:[]}),"pkg/a/src/index.ts":"export const v=1;\n"});
 const doorAtMs=Date.now();await sleep(5);
 await loadGenerationGraphV1(createMockHost(dir,{stamp:true}).host,{closure:["a"]});
 return{dir,doorAtMs};
}

test("a slice a peer's stamp covered is not sealed: an edit after the peer's verification and before the memo cursor is never served from the memo",async()=>{
 const {dir,doorAtMs}=await workspace(),journal=new ExactJournal();
 const first=await request(dir,journal,doorAtMs);
 expect(first.said[0],"precondition: the peer's stamp serves the request without a verification").toContain("covered");
 expect(first.marks["graph.memo-seal"]).toBe("light");
 await applyOps(dir,[{op:"write",path:"pkg/a/src/index.ts",content:"export const v=2;\n"}]);await sleep(5);
 await request(dir,journal,doorAtMs);
 const served=await request(dir,journal,Date.now());
 if(served.served)expect(served.graph,"oracle: the planner serves a fresh request of the same closure this graph").toEqual(JSON.parse(JSON.stringify(await loadGenerationGraphV1(createMockHost(dir,{stamp:true}).host,{closure:["a"]}))));
 expect(served.served,"the memo must not vouch for the edit it never saw").toBe(false);
 expect(served.marks["graph.memo-seal"],"liveness: a request that verified after its cursor seals").toBe("sealed");
 const hit=await request(dir,journal,Date.now());
 expect(hit.served).toBe(true);
 expect(hit.graph,"oracle: the served slice equals the planner's").toEqual(JSON.parse(JSON.stringify(await loadGenerationGraphV1(createMockHost(dir,{stamp:true}).host,{closure:["a"]}))));
},60000);

test("a slice served from a pin older than the memo cursor (a nested command bound to its outer invocation) is unsealable",async()=>{
 const {dir}=await workspace(),manifest=JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8")) as {generation:number;manifest:string},owners=["a"].sort(compareCodePointsV1),cursorAtMs=Date.now(),stem=join(sandbox,"pin"+counter++);
 const pin=encodeGenerationPinV1({owner:1,generation:manifest.generation,manifest:manifest.manifest,owners,covered:new Set(owners),atMs:cursorAtMs-60000});
 await writeGraphMemoSliceSidecarV1({root:dir,generations:join(dir,".gen"),exclude,pin,stem,client:new ExactJournal(),insensitive,verifiedAfter:cursorAtMs} as Parameters<typeof writeGraphMemoSliceSidecarV1>[0]);
 expect((JSON.parse(readFileSync(stem+".json","utf8")) as {unsealable:string|null}).unsealable).toBe("unverified");
 const current=join(sandbox,"pin"+counter++);
 await writeGraphMemoSliceSidecarV1({root:dir,generations:join(dir,".gen"),exclude,pin:encodeGenerationPinV1({owner:1,generation:manifest.generation,manifest:manifest.manifest,owners,covered:new Set(owners),atMs:cursorAtMs}),stem:current,client:new ExactJournal(),insensitive,verifiedAfter:cursorAtMs} as Parameters<typeof writeGraphMemoSliceSidecarV1>[0]);
 expect((JSON.parse(readFileSync(current+".json","utf8")) as {unsealable:string|null}).unsealable,"a slice verified at the cursor stays sealable").toBeNull();
});
