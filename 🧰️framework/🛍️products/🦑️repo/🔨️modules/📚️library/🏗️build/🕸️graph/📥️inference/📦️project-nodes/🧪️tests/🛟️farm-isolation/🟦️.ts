/** 🛟️ Contract: an owner failure inside a farm child (or in the late closure-reference stage) is isolated like one in the closure walk: observed again while the inputs it read moved, answered to the parent as the named bilingual failure when it reproduces on settled inputs, and then withheld by name with the rows it read, so the read seals without that owner. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});

const inference=await import("../../../🟨️.mjs") as any;
await inference.libraryBootstrap;
const observation=await import("../../../../../👁️observation/🟨️.mjs") as any,nodes=await import("../../🟨️.mjs") as any,{PROJECT_BASENAME}=await import("../../../📍️coordinates/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const setup=async()=>{
 const root=await repository.scratch("farm-isolation-");roots.push(root);
 const put=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 put("owner/"+PROJECT_BASENAME,JSON.stringify({name:"@t/owner"}));put("owner/input.txt","bad");
 const scope={workspaceRoot:root,fallback:join(root,"owner",PROJECT_BASENAME)},cachePath=join(root,"np","owner.json");
 const lines:string[]=[],pauses:number[]=[];
 const read=(onRead?:()=>void)=>{
  const text=String(observation.observedFs.readFileSync(join(root,"owner","input.txt"),"utf8"));
  onRead?.();
  if(text.startsWith("bad"))throw Error("owner-refused: input is bad");
  return{text} as any;
 };
 const attempt=(onRead?:()=>void)=>observation.observeInferenceSnapshotV1(()=>observation.observeDetachedV1(async()=>read(onRead))).then((done:{value:unknown})=>done.value);
 const infer=(round:number,onRead?:()=>void)=>observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"owner",owner:"owner"},async()=>{
  const started=Date.now();
  try{return{text:read(onRead).text,withheld:false} as any;}
  catch(raised){
   const error=await nodes.ownerInferenceFailureV1(raised,{workspaceRoot:root,root:"owner",name:"@t/owner",aborted:false,round,started});
   if(error===undefined)throw raised;
   observation.observedFs.readFileSync(error.source);
   return{withheld:true,message:error.message,code:error.code} as any;
  }
 }));
 const child=(body:()=>Promise<unknown>)=>async()=>{
  try{const refs=await nodes.observeRefsWithRetriesV1(body,{owner:"@t/owner",scope,pause:async(milliseconds:number)=>{pauses.push(milliseconds);},say:(line:string)=>lines.push(line)});return{verdict:"holds",headerDigest:"h",refs};}
  catch(error){return{verdict:"holds",headerDigest:"h",refs:nodes.refsFailureV1(error,scope)};}
 };
 const farmOf=(answer:()=>Promise<unknown>)=>({exists:()=>true,observe:async()=>answer()});
 const ask=()=>nodes.askOwnerRefsV1({farm:farmOf,job:{name:"@t/owner",root:"owner"},headerDigest:"h"});
 return{root,put,scope,attempt,infer,child,pauses,lines,farmOf};
};

test("a farm child whose owner throws on an input that moved under it recovers without the parent seeing a failure",async()=>repository.invoke(async()=>{
 const state=await setup();
 let moved=false;
 const answer=state.child(()=>state.attempt(()=>{if(!moved){moved=true;state.put("owner/input.txt","good!");}}).then((value:unknown)=>({project:value})));
 const reply=await state.farmOf(answer).observe() as any;
 expect(reply.refs.project.text).toBe("good!");
 expect(reply.refs.ownerFailed).toBeUndefined();
 expect(state.pauses.length).toBe(1);
 expect(state.lines.length).toBe(1);
 expect(state.lines[0]).toContain("[graph-owner-retry]");
 const line=JSON.parse(state.lines[0]!.replace("[graph-owner-retry] ",""));
 expect(line.text.en).toContain("@t/owner");
 expect(line.text.de).toContain("Eigentümer @t/owner");
 const asked=await nodes.askOwnerRefsV1({farm:{exists:()=>true,observe:async()=>({verdict:"holds",headerDigest:"h",refs:reply.refs})},job:{name:"@t/owner",root:"owner"},headerDigest:"h"});
 expect(asked.refs.project.text).toBe("good!");
}),60_000);

test("a stable failure in a farm child is the named bilingual failure, answered once, and the parent withholds the owner with the rows the child read",async()=>repository.invoke(async()=>{
 const state=await setup();
 await new Promise(done=>setTimeout(done,2300));
 const answer=state.child(()=>state.attempt());
 const reply=await state.farmOf(answer).observe() as any;
 expect(state.pauses.length).toBe(0);
 expect(reply.refs.ownerFailed.stable).toBe(true);
 expect(reply.refs.ownerFailed.text.en).toContain("owner-refused");
 expect(reply.refs.ownerFailed.text.de).toContain("kann nicht abgeleitet werden");
 expect(reply.refs.ownerFailed.observed.some((row:{kind:string;path:string})=>row.kind==="file"&&row.path===join(state.root,"owner","input.txt"))).toBe(true);
 let calls=0;
 const failure=await nodes.askOwnerRefsV1({farm:{exists:()=>true,observe:async()=>{calls++;return reply;}},job:{name:"@t/owner",root:"owner"},headerDigest:"h"}).catch((error:unknown)=>error) as any;
 expect(calls).toBe(1);
 expect(failure.code).toBe("BUILD_OWNER_FAILED");
 expect(failure.owners.map((owner:{name:string})=>owner.name)).toEqual(["@t/owner"]);
 const withheld=nodes.ownerRefsFailureV1(failure,{workspaceRoot:state.root,root:"owner",name:"@t/owner",aborted:false});
 expect(withheld.code).toBe("BUILD_SOURCE_UNPARSEABLE");
 expect(withheld.source).toBe(join(state.root,"owner",PROJECT_BASENAME));
 expect(withheld.message).toContain(" / ");
 expect(withheld.observedRows.length).toBeGreaterThan(0);
 expect(await observation.observationsHold(withheld.observedRows)).toBe(true);
 state.put("owner/input.txt","good!");
 expect(await observation.observationsHold(withheld.observedRows),"editing the input makes the withholding stale").toBe(false);
 expect(nodes.ownerRefsFailureV1(Object.assign(failure,{owners:[{name:"@t/other"}]}),{workspaceRoot:state.root,root:"owner",name:"@t/owner",aborted:false}),"another owner's named failure is not this owner's").toBeUndefined();
}),60_000);

test("rows that keep moving through every round end as the unsettled failure of the owner; foreign and source failures pass through the child untouched",async()=>repository.invoke(async()=>{
 const state=await setup();
 let round=0;
 const failure=await nodes.observeRefsWithRetriesV1(()=>state.attempt(()=>{round++;state.put("owner/input.txt","bad"+"!".repeat(round));}),{owner:"@t/owner",scope:state.scope,pause:async()=>undefined,say:()=>undefined}).catch((error:unknown)=>error) as any;
 expect(failure.code).toBe("BUILD_OWNER_FAILED");
 expect(failure.stable).toBe(false);
 expect(failure.rounds).toBe(4);
 expect(round).toBe(4);
 expect(failure.text.en).toContain("kept changing");
 for(const code of ["BUILD_AUTHORITY_CHANGED","graph.farm-job-failed","BUILD_STORE_CHANGED","EMFILE"]){
  let calls=0;
  const raised=await nodes.observeRefsWithRetriesV1(async()=>{calls++;throw Object.assign(Error("layer"),{code});},{owner:"@t/owner",scope:state.scope,pause:async()=>undefined,say:()=>undefined}).catch((error:unknown)=>error) as any;
  expect(raised.code,code).toBe(code);
  expect(calls,code).toBe(1);
 }
}),60_000);

test("the inference attempt loop observes an owner again after a retry failure and gives the last failure of an exhausted loop its cause",async()=>repository.invoke(async()=>{
 const rounds:number[]=[],lines:string[]=[];
 const value=await nodes.observeOwnerAttemptsV1(async(round:number)=>{rounds.push(round);if(round<3)throw Object.assign(Error("moved"),{code:nodes.OWNER_RETRY_CODE_V1,ownerRetry:{owner:"o",round,milliseconds:5,text:{en:"e",de:"d"}}});return "done";},{pause:async()=>undefined,say:(line:string)=>lines.push(line)});
 expect(value).toBe("done");
 expect(rounds).toEqual([1,2,3]);
 expect(lines.length).toBe(2);
 const plain=await nodes.observeOwnerAttemptsV1(async()=>{throw Error("plain");},{pause:async()=>undefined,say:()=>undefined}).catch((error:unknown)=>error) as Error;
 expect(plain.message).toBe("plain");
}),60_000);

test("the owner inference of a farm child or of the parent observes the owner again when its input moved, withholds it by name when it reproduces, and names the unsettled one",async()=>repository.invoke(async()=>{
 const moved=await setup();
 let once=false;
 const value=await nodes.observeOwnerAttemptsV1((round:number)=>moved.infer(round,()=>{if(!once){once=true;moved.put("owner/input.txt","good!");}}),{pause:async(milliseconds:number)=>{moved.pauses.push(milliseconds);},say:(line:string)=>moved.lines.push(line)});
 expect(value.withheld).toBe(false);
 expect(value.text).toBe("good!");
 expect(moved.pauses.length).toBe(1);
 expect(moved.lines[0]).toContain("[graph-owner-retry]");

 const stable=await setup();
 await new Promise(done=>setTimeout(done,2300));
 const kept=await nodes.observeOwnerAttemptsV1((round:number)=>stable.infer(round),{pause:async(milliseconds:number)=>{stable.pauses.push(milliseconds);},say:()=>undefined});
 expect(kept.withheld).toBe(true);
 expect(kept.code).toBe("BUILD_SOURCE_UNPARSEABLE");
 expect(kept.message).toContain("reproduces on unchanged inputs");
 expect(kept.message).toContain("kann nicht abgeleitet werden");
 expect(stable.pauses.length).toBe(0);
 expect(await observation.observationsHold(observation.projectInferenceObservationsV1(join(stable.root,"np","owner.json"))??[])).toBe(true);

 const racing=await setup();
 let round=0;
 const unsettled=await nodes.observeOwnerAttemptsV1((current:number)=>racing.infer(current,()=>{round++;racing.put("owner/input.txt","bad"+"!".repeat(round));}),{pause:async(milliseconds:number)=>{racing.pauses.push(milliseconds);},say:()=>undefined});
 expect(unsettled.withheld).toBe(true);
 expect(unsettled.message).toContain("kept changing during 4 observations");
 expect(racing.pauses.length).toBe(3);
 expect(round).toBe(4);
}),90_000);
