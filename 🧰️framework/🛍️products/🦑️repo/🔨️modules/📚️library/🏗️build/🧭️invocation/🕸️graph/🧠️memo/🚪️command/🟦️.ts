import {mkdtempSync,rmSync} from "node:fs";
import {lstat,readFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {GRAPH_MEMO_POLICY_V1,graphMemoEnabledV1,graphMemoKeyV1,openGraphMemoV1,renderGraphMemoTextV1,type GraphMemoEditPortsV1,type GraphMemoV1,type MemoAnswerV1,type MemoRowV1} from "../🟦️.ts";
import {canonicalRootV1} from "../../../../🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../../../../🏡️resident/📜️journal/🟦️.ts";
declare global{interface ImportMeta{readonly semioCapturedImplementationIdentity?:string;readonly semioCapturedLoaderIdentity?:string}}

const UNCHECKED=new Set(["derived","closure","stamp","registry"]);
/** ⚖️ The planner's own fact check (`generationHostV1` `holdsByStat`) for the rows the memo checks itself: environment rows (`field`, `digest`) by the environment rule, every other row by the observation engine with the source analyses registered (an analysis is replayed and compared by its result), nothing taken as stable. A row the planner settles by another computation (a derived fact, a closure entry, an inference stamp, a registry row) does not hold here, so the memo plans.
 * @see ../../../../🕸️graph/🟦️.ts `generationHostV1`
 * @see ../../../../🕸️graph/🕰️generations/🌍️environment/🟨️.mjs `environmentRowHoldsV1` */
export async function memoFactsHoldV1(root:string,rows:readonly MemoRowV1[],signal?:AbortSignal):Promise<boolean>{
 const plain:MemoRowV1[]=[],own:MemoRowV1[]=[];
 for(const row of rows){if(UNCHECKED.has(row.kind))return false;(row.kind==="field"||row.kind==="digest"?own:plain).push(row);}
 if(own.length){
  const {environmentRowHoldsV1}=await import("../../../../🕸️graph/🕰️generations/🌍️environment/🟨️.mjs"),missing=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??"");
  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}catch(error){if(missing(error))return undefined;throw error;}},stat=async(path:string)=>{try{const status=await lstat(join(root,path));return status.isFile()?{size:status.size,mtimeMs:status.mtimeMs}:undefined;}catch(error){if(missing(error))return undefined;throw error;}};
  for(let at=0;at<own.length;at+=64){signal?.throwIfAborted();if(!(await Promise.all(own.slice(at,at+64).map(row=>environmentRowHoldsV1(row as never,{read,stat})))).every(Boolean))return false;}
 }
 if(!plain.length)return true;
 if(plain.some(row=>row.kind==="analysis")){const producer=await import("../../../../🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();}
 const {observationsHold}=await import("../../../../👁️observation/🟨️.mjs");
 return observationsHold(plain as never,64,new Set(),true,signal);
}

/** 🧫️ Runs a child process of the same invocation with extra environment (the planner child's launch with another marker); the memo asks its derivation child through it. */
export type GraphMemoChildV1=(environment:Readonly<Record<string,string>>)=>Promise<void>;

/** ✏️ The edit ports of one repository: rows by their position in a fact index and a frontier, both read from the immutable generation store (a retired document answers `undefined`), the frontier with the planner's own walk predicates; with a `child` runner also the derivation port, which asks the derived questions that read a stale row again in a child process (the planner's deriver, `🧮️derive`) and leaves the executor's heap alone.
 * @see ../🧮️derive/🟦️.ts
 * @see ../../../../🕸️graph/🟦️.ts `buildWalkDescendsV1` `graphMemberPredicateV1` */
export function graphMemoEditPortsV1(root:string,child?:GraphMemoChildV1,warm?:import("../🧮️derive/♨️warm/🟦️.ts").WarmDeriverPortV1):GraphMemoEditPortsV1{
 const store=async()=>{const [{createGenerationStoreV1,createNodeFilePortV1},{repositoryCachePartitionRootV1}]=await Promise.all([import("../../../../🕸️graph/🕰️generations/🟨️.mjs"),import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")]);return createGenerationStoreV1({root:join(repositoryCachePartitionRootV1(root,"graph"),"generations"),fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo"}});};
 return{
  rows:async(index,ids)=>{const engine=await store();try{const view=await engine.readIndexView(index);return view?ids.map(id=>view.rows[id] as MemoRowV1):undefined;}catch{return undefined;}finally{engine.dropIndexes();}},
  frontier:async digest=>{try{const [engine,{buildWalkDescendsV1,graphMemberPredicateV1}]=await Promise.all([store(),import("../../../../🕸️graph/🟦️.ts")]),frontier=await engine.readFrontier(digest);return frontier?{frontier,descend:buildWalkDescendsV1(),member:graphMemberPredicateV1()}:undefined;}catch{return undefined;}},
  ...(child?{derive:async({index,stale,derived})=>{
   const [engine,derivation]=await Promise.all([store(),import("../🧮️derive/🟦️.ts")]);
   try{
    const view=await engine.readIndexView(index);if(!view)return{held:false,reason:"missing"};
    return await derivation.settleGraphMemoDerivationV1({index:view,stale,derived,ask:async suspects=>{const directory=mkdtempSync(join(tmpdir(),"semio-graph-derive-")),file=join(directory,"request.json");try{derivation.writeGraphMemoDeriveFileV1(file,{version:1,index,suspects:[...suspects]});const answered=()=>derivation.readGraphMemoDeriveFileV1(derivation.graphMemoDeriveAnswerFileV1(file),"answer");if(warm&&await warm.ask(file)){const answer=answered();if(answer)return answer;}await child({[GRAPH_MEMO_POLICY_V1.derive]:file});const answer=answered();if(answer)warm?.start(file);return answer;}catch{return undefined;}finally{rmSync(directory,{recursive:true,force:true});}}});
   }catch{return{held:false,reason:"error"};}finally{engine.dropIndexes();}
  }} satisfies Partial<GraphMemoEditPortsV1>:{}),
 };
}

/** 🧠️ Opens the graph memo of one command in this process: off when the switch says so or when this process does not run a captured implementation (the identity of the implementation is part of the key). The journal cursor is taken here, before any planner child starts. A sealed slice whose generation was superseded by another request's refresh is carried to the current generation when it is the same there; a relevant change is settled by re-checking the facts that read it with the planner's own check, and a stale support of derived facts by asking those questions again: first the warm deriver of this implementation, else a `child` process, after which a warm deriver is started for the next edit.
 * @see ../🧳️carry/🟦️.ts
 * @see ../🟦️.ts the policy, the lookup and `settleMemoEditsV1` */
export async function openGraphMemoForCommandV1(root:string,args:readonly string[],capabilities:Readonly<{tool?:string;buildMode?:string}>,signal?:AbortSignal,environment:Readonly<Record<string,string|undefined>>=process.env,child?:GraphMemoChildV1):Promise<GraphMemoV1|undefined>{
 if(!graphMemoEnabledV1(environment))return undefined;
 const implementation=import.meta.semioCapturedImplementationIdentity;
 if(typeof implementation!=="string")return undefined;
 const canonical=canonicalRootV1(root);
 return openGraphMemoV1({root,directory:join(changeJournalDirectoryV1(canonical,environment),GRAPH_MEMO_POLICY_V1.directory),client:connectChangeJournalV1(canonical,environment),key:graphMemoKeyV1({root,args,tool:capabilities.tool,mode:capabilities.buildMode,implementation:implementation+":"+(import.meta.semioCapturedLoaderIdentity??""),environment}),...(signal?{signal}:{}),environment,
  holds:(rows,abort)=>memoFactsHoldV1(root,rows,abort),edit:graphMemoEditPortsV1(root,child,child?await warmPortV1(root,implementation+":"+(import.meta.semioCapturedLoaderIdentity??"")):undefined),
  carry:async(input,abort)=>{const [{carryGraphMemoPinV1},{repositoryCachePartitionRootV1}]=await Promise.all([import("../🧳️carry/🟦️.ts"),import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")]);return carryGraphMemoPinV1({generations:join(repositoryCachePartitionRootV1(root,"graph"),"generations"),...input,...(abort?{signal:abort}:{})});}});
}

/** ♨️ The warm deriver port of a run of this captured implementation (its closure, if the run holds one, is what a started deriver must find on disk). */
async function warmPortV1(root:string,identity:string):Promise<import("../🧮️derive/♨️warm/🟦️.ts").WarmDeriverPortV1>{const{warmDeriverPortV1,capturedWarmClosureV1}=await import("../🧮️derive/♨️warm/🟦️.ts");return warmDeriverPortV1(root,identity,capturedWarmClosureV1());}

/** 🌐️ The language of the command the memo speaks in. */
async function memoLanguageV1(environment:Readonly<Record<string,string|undefined>>):Promise<string>{
 const {resolveLanguageV1}=await import("../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts"),{CATALOG_LANGUAGES_V1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts");
 return resolveLanguageV1(CATALOG_LANGUAGES_V1,[],environment as NodeJS.ProcessEnv).language;
}

/** 🗣️ The line a served graph prints, in the language of the command, a second one when its slice was carried to a newer generation and one when changed paths were settled by re-checking the facts that read them. */
export async function renderGraphMemoServedV1(answer:Extract<MemoAnswerV1,{hit:true}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{
 const language=await memoLanguageV1(environment),lines=[renderGraphMemoTextV1("served",{projects:answer.projects,seconds:Math.round(answer.ageMilliseconds/1000),rows:answer.rows,rechecked:answer.rechecked},language)];
 if(answer.carried)lines.push(renderGraphMemoTextV1("carried",{from:answer.carried.from,to:answer.carried.to,projects:answer.projects},language));
 if(answer.edited)lines.push(renderGraphMemoTextV1("edited",{paths:answer.edited.paths,rows:answer.edited.rows},language));
 if(answer.edited?.questions)lines.push(renderGraphMemoTextV1("derived",{questions:answer.edited.questions},language));
 return lines.join("\n");
}

/** 📣️ The one line a memo that cannot serve prints: the reason and, where one exists, the first relevant change. */
export async function renderGraphMemoRefusedV1(answer:Extract<MemoAnswerV1,{hit:false}>,environment:Readonly<Record<string,string|undefined>>=process.env):Promise<string>{
 return renderGraphMemoTextV1("refused",{reason:answer.detail===undefined?answer.reason:answer.reason+": "+answer.detail},await memoLanguageV1(environment));
}

/** 🧰️ What the planner of one request needs from its caller: the memo of the request, where the child leaves its relevance, how to read the slice, where to record the pin, where to print and mark, and the planner child itself. */
export type GraphMemoPlanInputV1<T extends Readonly<{projects:Readonly<Record<string,unknown>>}>>=Readonly<{
 memo:GraphMemoV1|undefined;stem:string;parse:(text:string)=>T;record:(pin:string|undefined)=>void;notice:(text:string)=>void;mark:(name:string,detail:string)=>void;environment?:Readonly<Record<string,string|undefined>>;
 plan:(sidecar:Readonly<Record<string,string>>|undefined)=>Promise<Readonly<{plan:string;pin:string|undefined}>>;
}>;

/** 🗺️ Serves a request from its memo when the journal proves that nothing the generation read has changed; otherwise runs the planner child (asking it for its relevance when a light record names the generation it will serve, with the request time moved to the moment after the memo cursor so that only a verification that began after the cursor serves the slice it seals) and keeps what it produced. A request without a memo is planned as before.
 * @see ../🟦️.ts the policy and the lookup */
export async function planGraphWithMemoV1<T extends Readonly<{projects:Readonly<Record<string,unknown>>}>>(input:GraphMemoPlanInputV1<T>):Promise<Readonly<{graph:T;served:boolean}>>{
 const {memo,stem}=input,served=await memo?.lookup();
 input.mark("graph.memo",served===undefined?"off":served.hit?served.carried?"hit:carried":served.edited?"hit:edited":"hit":"miss:"+served.reason);
 if(served!==undefined&&!served.hit&&served.reason!=="none")input.notice(await renderGraphMemoRefusedV1(served,input.environment));
 if(served?.hit){const graph=input.parse(served.plan);input.record(served.pin);input.notice(await renderGraphMemoServedV1(served,input.environment));return{graph,served:true};}
 const expect=memo?.cursor===undefined?undefined:memo.expected(),after=memo?.cursorAtMs,produced=await input.plan(expect===undefined||after===undefined?undefined:{[GRAPH_MEMO_POLICY_V1.sidecar]:stem+".json",[GRAPH_MEMO_POLICY_V1.expect]:expect,[GRAPH_MEMO_POLICY_V1.requestedAt]:String(after)}),graph=input.parse(produced.plan);
 input.record(produced.pin);
 if(memo)input.mark("graph.memo-seal",await memo.seal({plan:produced.plan,pin:produced.pin,projects:Object.keys(graph.projects).length,sidecar:stem+".json"}));
 return{graph,served:false};
}
