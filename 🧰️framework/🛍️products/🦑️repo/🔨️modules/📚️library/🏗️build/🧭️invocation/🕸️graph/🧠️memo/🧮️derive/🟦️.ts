import {readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {GRAPH_MEMO_POLICY_V1,type MemoDerivationV1,type MemoRowV1} from "../🟦️.ts";
import {createGenerationStoreV1,createNodeFilePortV1,selectFactClassesV1,type FactIndexV1} from "../../../../🕸️graph/🕰️generations/🟨️.mjs";
import {derivedOwnerNameV1,settleSuspectsV1,splitStaleConsumersV1} from "../../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs";
import {refuseBuildV1} from "../../../../📖️texts/🟦️.ts";

/** 🧾️ The closed answer of a derivation child (`DeriveAnswer` of the memo schema). */
export type MemoDeriveAnswerV1=Readonly<{version:1;held:readonly string[];changed:readonly string[];failed:number;uncovered:readonly string[];missing:boolean}>;
/** ❔️ One derived question asked again: its answer and the rows the new computation read (the planner's `derive` port). */
export type MemoDeriverV1=(row:MemoRowV1,signal?:AbortSignal)=>Promise<Readonly<{fact:unknown;rows:readonly MemoRowV1[]}>>;

/** 👥️ Who reads the rows an edit left stale: the owners that hold a stale row themselves (`direct`: a project or the environment, which the planner would infer again), the pseudo owners whose derived question it supports (`suspects`), plus the pseudo owner of every derived fact the journal names; a derived fact no pseudo owner supports is `unsupported`. The planner's own split (`splitStaleConsumersV1`) over every class of the index, so an owner outside the slice only makes it more cautious.
 * @see ../../../../🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs `splitStaleConsumersV1` */
export function graphMemoSuspectsV1(index:FactIndexV1,stale:readonly number[],derived:readonly MemoRowV1[]):Readonly<{direct:readonly string[];suspects:ReadonlySet<string>;unsupported:readonly string[]}>{
 const split=splitStaleConsumersV1(index,index.classes.map((_,at)=>at),new Set(stale)),suspects=new Set(split.suspects),owners=new Set(index.owners),unsupported:string[]=[];
 for(const row of derived){const name=derivedOwnerNameV1(row);if(owners.has(name))suspects.add(name);else unsupported.push(name);}
 return{direct:[...split.direct],suspects,unsupported};
}

/** 🔁️ Asks the suspect questions again with the planner's cutoff (`settleSuspectsV1`): a held answer is kept only when its new support reads no entry (kind and path) the old support did not read, because the sealed relevance watches only the old support; such a question is `uncovered`. */
export async function deriveGraphMemoSuspectsV1(input:Readonly<{index:FactIndexV1;suspects:ReadonlySet<string>;derive:MemoDeriverV1;signal?:AbortSignal}>):Promise<MemoDeriveAnswerV1>{
 const settled=await settleSuspectsV1({index:input.index,suspects:input.suspects,derive:input.derive,...(input.signal?{signal:input.signal}:{})}),identity=(row:MemoRowV1)=>row.kind+"\u0000"+row.path,held:string[]=[],uncovered:string[]=[];
 for(const [name,rows] of settled.held){const old=new Set(selectFactClassesV1(input.index,new Set([name])).ids.map(id=>identity(input.index.rows[id] as MemoRowV1)));(rows.slice(1).every(row=>old.has(identity(row as MemoRowV1)))?held:uncovered).push(name);}
 return{version:1,held,changed:[...settled.changed.keys()],failed:settled.failed,uncovered,missing:false};
}

/** ⚖️ The verdict of one derivation: every suspect question held (asked again, the recorded answer, covered), else the reason. */
export function graphMemoDerivationHeldV1(suspects:ReadonlySet<string>,answer:MemoDeriveAnswerV1|undefined):MemoDerivationV1{
 if(!answer||answer.missing)return{held:false,reason:"missing"};
 if(answer.changed.length||answer.failed)return{held:false,reason:"changed"};
 if(answer.uncovered.length)return{held:false,reason:"uncovered"};
 const held=new Set(answer.held);
 return [...suspects].every(name=>held.has(name))?{held:true,questions:suspects.size}:{held:false,reason:"unanswered"};
}

/** 🧮️ The memo's side of a derivation: stale rows a project or the environment holds itself, an unsupported derived fact, no question at all or more than `maximumDerivedQuestions` questions refuse at once; otherwise `ask` (the child process, or a deriver in a law) answers the suspects and every answer must hold. */
export async function settleGraphMemoDerivationV1(input:Readonly<{index:FactIndexV1;stale:readonly number[];derived:readonly MemoRowV1[];ask:(suspects:ReadonlySet<string>)=>Promise<MemoDeriveAnswerV1|undefined>}>):Promise<MemoDerivationV1>{
 const found=graphMemoSuspectsV1(input.index,input.stale,input.derived);
 if(found.direct.length)return{held:false,reason:"direct:"+found.direct[0]};
 if(found.unsupported.length)return{held:false,reason:"unsupported"};
 if(!found.suspects.size||found.suspects.size>GRAPH_MEMO_POLICY_V1.maximumDerivedQuestions)return{held:false,reason:"questions"};
 return graphMemoDerivationHeldV1(found.suspects,await input.ask(found.suspects));
}

const isSha=(value:unknown):value is string=>typeof value==="string"&&/^[0-9a-f]{64}$/u.test(value);
const names=(value:unknown):value is string[]=>Array.isArray(value)&&value.length<=GRAPH_MEMO_POLICY_V1.deriveMaximumNames&&value.every(name=>typeof name==="string"&&name.length>0&&name.length<=GRAPH_MEMO_POLICY_V1.deriveNameCharacters);
/** 📮️ Reads a derivation request or answer file back; another shape is `undefined`. */
export function readGraphMemoDeriveFileV1<K extends "request"|"answer">(file:string,kind:K):(K extends "request"?Readonly<{version:1;index:string;suspects:readonly string[]}>:MemoDeriveAnswerV1)|undefined{
 try{
  const value=JSON.parse(readFileSync(file,"utf8")) as Record<string,unknown>;
  if(value.version!==1)return undefined;
  const valid=kind==="request"?Object.keys(value).length===3&&isSha(value.index)&&names(value.suspects):Object.keys(value).length===6&&names(value.held)&&names(value.changed)&&names(value.uncovered)&&Number.isSafeInteger(value.failed)&&(value.failed as number)>=0&&typeof value.missing==="boolean";
  return valid?value as never:undefined;
 }catch{return undefined;}
}
/** ✉️ Writes a derivation request or answer atomically. */
export function writeGraphMemoDeriveFileV1(file:string,value:unknown):void{
 const temporary=file+"."+process.pid+".tmp";
 try{writeFileSync(temporary,JSON.stringify(value),{mode:0o600});renameSync(temporary,file);}catch(error){rmSync(temporary,{force:true});throw error;}
}
/** 📬️ Where a derivation child writes the answer to a request file. */
export const graphMemoDeriveAnswerFileV1=(request:string):string=>request.replace(/\.json$/u,"")+".answer.json";

/** 🏭️ The planner's own deriver of a repository, opened exactly as the generation host opens it (`deriveAgain`): the inference implementation booted, its source analyses registered, the owner registry of the workspace.
 * @see ../../../../🕸️graph/🟦️.ts `generationHostV1` */
export async function openPlannerDeriverV1(root:string):Promise<MemoDeriverV1>{
 const producer=await import("../../../../🕸️graph/📥️inference/🟨️.mjs"),{readBuildOwnerRegistryV1}=await import("../../../../🕸️graph/🚪️admission/🟦️.ts");
 await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
 return await producer.openGenerationDeriverV1({workspaceRoot:root,registry:readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)))}) as MemoDeriverV1;
}

/** 🧒️ The whole work of a derivation child (its parent set `Policy.derive` to the request file): reads the fact index the request names from the generation store, asks the suspect questions again with the planner's deriver and writes the answer next to the request; a retired index answers `missing`. `open` replaces the deriver in a law. */
export async function answerGraphMemoDerivationV1(root:string,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal,open:(root:string)=>Promise<MemoDeriverV1>=openPlannerDeriverV1,generations?:string):Promise<void>{
 const file=environment[GRAPH_MEMO_POLICY_V1.derive];
 if(!file)throw refuseBuildV1("usage","memo.derive-variable-required",{variable:GRAPH_MEMO_POLICY_V1.derive});
 const request=readGraphMemoDeriveFileV1(file,"request");
 if(!request)throw refuseBuildV1("invalid","memo.derive-request-invalid",{file});
 const directory=generations??join((await import("../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")).repositoryCachePartitionRootV1(root,"graph"),"generations");
 const store=createGenerationStoreV1({root:directory,fs:createNodeFilePortV1(),holds:async()=>true,resources:{facts:"graph-memo-derive"}}),index=await store.readIndexView(request.index);
 const answer:MemoDeriveAnswerV1=index?await deriveGraphMemoSuspectsV1({index,suspects:new Set(request.suspects),derive:await open(root),...(signal?{signal}:{})}):{version:1,held:[],changed:[],failed:0,uncovered:[],missing:true};
 writeGraphMemoDeriveFileV1(graphMemoDeriveAnswerFileV1(file),answer);
}
