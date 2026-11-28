import {join} from "node:path";
import {validateTestFileDocumentV1,type TestFileResultV1} from "../🟦️.ts";
import type {TestFileLanguageV1} from "../📖️texts/🟦️.ts";

/** 🏁️ How the remote attempt for one test file ended: it ran on another machine (`ran`: the machine's origin, the seconds, the structured result with every row of the file and the output of its run that was not already shown live, which this machine prints as a local run would have), it stays here (`local`: the reason, whether a remote run was attempted at all) or the run was cancelled. A test file that failed on the other machine is a `ran` result, never a reason to run it again. */
export type TestFileRemoteOutcomeV1=Readonly<{kind:"ran";origin:string;seconds:number;result:TestFileResultV1;output:string}>|Readonly<{kind:"local";reason:string;detail:string;attempted:boolean}>|Readonly<{kind:"cancelled"}>;

/** 📥️ What one remote attempt is given: the language of its notices, where progress lines go (`notice`), where the output of the remote run goes while it runs (`output`, every line already prefixed with `[<tag><origin>]`) and the tag naming this file among many (`2/3 `, empty for a single file). */
export type TestFileRemoteInputV1=Readonly<{language:TestFileLanguageV1;notice:(line:string)=>void;output?:(text:string)=>void;tag?:string}>;

/** 🌐️ The remote port of a run: asks whether one file leaves this machine, hands it to an idle Ready machine of the same platform and revision, and answers; progress lines go through `notice`, the live output through `output`. `parallel` is how many files the run may have out at once; `measure` records how long a file took here, so the next verdict knows the local and the remote duration. */
export type TestFileRemoteV1=((file:string,budgetMs:number,input:TestFileRemoteInputV1)=>Promise<TestFileRemoteOutcomeV1>)&Readonly<{parallel?:number;measure?:(file:string,seconds:number)=>Promise<void>}>;

/** 🔌️ The seams of the remote port; tests replace them. Replacing `host` replaces the CPU load signal too (it is read only when neither is given). */
export type TestFileRemotePortsV1=Readonly<{memoRoot?:string;host?:()=>Promise<"admit"|"wait">;load?:(file:string)=>Promise<"offload"|"keep">;idle?:()=>Promise<number|null>;offload?:typeof import("../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🔌️client/🟦️.ts")["offloadCheckoutTestV1"]}>;

/** 🧠️ Remembers the answer of an asynchronous question for a short time, so a run of many files asks the host once per window. */
function rememberV1<T>(question:()=>Promise<T>,milliseconds:number):()=>Promise<T>{
 let at=0,answer:T|undefined;
 return async()=>{if(answer!==undefined&&Date.now()-at<milliseconds)return answer;answer=await question();at=Date.now();return answer;};
}

/** 🏷️ Prefixes every complete line of a live output with `[<tag><origin>] `, holding back the partial line until it is complete or the run ends. */
function prefixerV1(tag:string):Readonly<{push:(origin:string,text:string)=>string;end:(origin:string)=>string}>{
 let partial="";
 const label=(origin:string,line:string)=>"["+tag+origin+"] "+line+"\n";
 return{
  push(origin,text){const all=partial+text,cut=all.lastIndexOf("\n");if(cut<0){partial=all;return"";}partial=all.slice(cut+1);return all.slice(0,cut).split("\n").map(line=>label(origin,line)).join("");},
  end(origin){const rest=partial;partial="";return rest===""?"":label(origin,rest);},
 };
}

/** 🌐️ Builds the remote port of `repo test-file` for one checkout, or `undefined` when the mode (`SEMIO_TEST_FILE_OFFLOAD=never`) keeps every file here. The environment is read once now, before the run scrubs the fleet variables from its own. A file leaves when the shared offload gate says the host is under pressure (`auto`: the host-capacity verdict of the compile units, or the CPU load per core with the measured local and remote durations of the file) or always (`always`), and a Ready machine of this platform exists; the other machine runs `repo test-file` for the same file in its own replica at the same revision (its replica cut covers this one's, its copy of the file has this one's digest) and answers with the structured result and the exit status, its output streamed meanwhile. The files of one run that are out at once never share a machine (`claim`). Every fault returns the file to this machine. The heavy parts load only when a file is offered. */
export async function createTestFileRemoteV1(root:string,environment:Readonly<Record<string,string|undefined>>,ports:TestFileRemotePortsV1={}):Promise<TestFileRemoteV1|undefined>{
 const {CHECKOUT_TEST_POLICY_V1,checkoutTestBothV1,checkoutTestModeV1,checkoutTestTextV1}=await import("../../🌐️remote-execution/🚀️offload/🔬️checkout-test/📐️policy/🟦️.ts"),mode=checkoutTestModeV1(environment);
 if(mode==="never")return undefined;
 const snapshot:Readonly<Record<string,string|undefined>>={...environment,SEMIO_WORKSPACE_ROOT:environment.SEMIO_WORKSPACE_ROOT??root},memoRoot=ports.memoRoot??join(root,".cache","f","offload-test"),claim=new Set<string>();
 const remembered=rememberV1<"admit"|"wait">((ports.host??(async()=>(await (await import("../../🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts")).localUnitAdmissionV1()).verdict)),10000);
 const remote=async(file:string,budgetMs:number,input:TestFileRemoteInputV1):Promise<TestFileRemoteOutcomeV1>=>{
  const gates=await import("../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🚦️gate/🟦️.ts");
  const load=ports.load!==undefined?()=>ports.load!(file):ports.host!==undefined?undefined:async()=>gates.checkoutTestLoadVerdictV1({loadPerCore:gates.checkoutTestLoadPerCoreV1(),measure:gates.readCheckoutTestMeasuresV1(memoRoot).files[file]});
  const gate=await gates.checkoutTestGateV1({mode,host:remembered,...load===undefined?{}:{load},idle:ports.idle??(async()=>{const replica=(await import("../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🧭️replica/🟦️.ts")).fleetCheckoutReplicaV1(root);return replica===undefined?null:(await replica.peers?.())?.length??null;})});
  if(!gate.offload)return{kind:"local",reason:gate.reason,detail:"",attempted:false};
  const offload=ports.offload??(await import("../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🔌️client/🟦️.ts")).offloadCheckoutTestV1,controller=new AbortController(),abort=():void=>{controller.abort();},prefixer=prefixerV1(input.tag??"");
  process.once("SIGINT",abort);process.once("SIGTERM",abort);
  let outcome;
  try{
   outcome=await offload({root,memoRoot,environment:snapshot,file,budgetSeconds:Math.max(1,Math.ceil(budgetMs/1000)),language:input.language,stretchBasisPoints:(await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts")).launchStretchBasisPointsV1(),signal:controller.signal,claim,...input.output===undefined?{}:{onOutput:(text:string,origin:string)=>{const lines=prefixer.push(origin,text);if(lines!=="")input.output!(lines);}},onStage:stage=>{if(stage==="QUEUED"||stage==="EXECUTING")input.notice(checkoutTestTextV1(input.language,stage==="QUEUED"?"stage.queued":"stage.executing",{file}));}});
  }finally{process.off("SIGINT",abort);process.off("SIGTERM",abort);}
  if(outcome.kind==="local")return outcome.reason==="cancelled"?{kind:"cancelled"}:{kind:"local",reason:outcome.reason,detail:outcome.detail,attempted:true};
  const problems=validateTestFileDocumentV1("Result",outcome.result);
  if(problems.length>0)return{kind:"local",reason:"remote-failed-local-redo",detail:problems[0]!.slice(0,200),attempted:true};
  const result=outcome.result as TestFileResultV1;
  if(!result.files.some(row=>row.file===file))return{kind:"local",reason:"remote-failed-local-redo",detail:checkoutTestBothV1("detail.no-row"),attempted:true};
  const refused=result.files.find(row=>row.file===file&&row.status==="failed"&&/deadline refused|deadline exhausted/iu.test((row.error??"")+(row.tail??"")));
  if(refused!==undefined)return{kind:"local",reason:"remote-failed-local-redo",detail:(refused.error??"deadline").slice(0,200),attempted:true};
  gates.recordCheckoutTestMeasureV1(memoRoot,file,"remote",outcome.seconds);
  return{kind:"ran",origin:outcome.origin,seconds:outcome.seconds,result,output:outcome.streamed===true?prefixer.push(outcome.origin,outcome.stderr)+prefixer.end(outcome.origin):outcome.stderr};
 };
 return Object.assign(remote,{parallel:CHECKOUT_TEST_POLICY_V1.limits.maximumParallel,measure:async(file:string,seconds:number):Promise<void>=>{(await import("../../🌐️remote-execution/🚀️offload/🔬️checkout-test/🚦️gate/🟦️.ts")).recordCheckoutTestMeasureV1(memoRoot,file,"local",seconds);}});
}
