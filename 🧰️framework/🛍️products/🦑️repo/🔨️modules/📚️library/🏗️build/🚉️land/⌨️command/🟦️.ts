import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {resolve} from "node:path";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {writeDiagnosticLineV1,writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {landArgumentsV1,landCheckoutV1,landDeleteV1,landSlotStateV1,readLandPolicyV1,type LandPolicyV1} from "../🟦️.ts";
import {landResultProblemsV1,landRunV1,type LandFlowPortsV1,type LandResultV1} from "../🧭️flow/🟦️.ts";
import {landTextV1,refuseLandV1,type LandLanguageV1,type LandTextKeyV1} from "../📖️texts/🟦️.ts";

/** 🎛️ What the command touches besides the checkout: the working directory, the notices and the ports of the run; tests replace them. */
export type LandCommandPortsV1=Readonly<{directory?:string;notice?:(line:string)=>void;run?:LandFlowPortsV1;policy?:LandPolicyV1}>;

const modes=new Set(["checkout","delete","check","apply","rollback","status"]);

/** 🖨️ Renders a result: the verdict of every gate with the lines of a failed one, then the total. */
export function renderLandResultV1(result:LandResultV1,language:LandLanguageV1):string{
 const text=(key:LandTextKeyV1,values:Record<string,string|number>={})=>landTextV1(language,key,values),lines:string[]=[];
 for(const row of result.gates){
  lines.push(row.status==="skipped"?text("gate.skipped",{gate:row.id,reason:row.detail[0]??""}):text(row.status==="passed"?"gate.passed":"gate.failed",{gate:row.id,seconds:row.seconds}));
  if(row.status==="failed")lines.push(...row.detail.map(line=>"  "+line));
 }
 if(result.reason&&result.status==="refused")lines.push(...result.reason.split("\n"));
 const areas=result.areas.join(", ");
 lines.push(result.status==="landed"?text("totals.landed",{count:result.entries.length,areas,rollback:result.rollback}):result.status==="rolled-back"?text("totals.rolled-back",{count:result.entries.length,id:result.id}):result.status==="checked"?text("totals.checked",{count:result.entries.length,areas}):result.status==="failed"?text("totals.failed",{count:result.gates.filter(row=>row.status==="failed").length}):result.status==="queued"?text("totals.queued",{id:result.id,next:result.slot.next}):text("totals.refused",{count:Math.max(1,(result.reason??"").split("\n").filter(Boolean).length)}));
 return lines.join("\n");
}

/** ▶️ Runs `repo land checkout|delete <dir> <path>...`, `repo land [check|apply] <dir>`, `repo land rollback <id>` and `repo land status` with `[--json] [--dry-run] [--outage] [--rebase] [--no-wait] [--lane] [--title] [--wave]`. Exit code: 0 landed, checked or rolled back; 1 a gate failed; 2 refused (stale base, locks, usage, cancelled); 3 queued for the next slot. Ctrl-C and SIGTERM cancel everything before the apply; the apply is never interrupted. */
export async function runLandCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void=writeResultLineV1,ports:LandCommandPortsV1={}):Promise<number>{
 const language=resolveLanguageV1(["en","de"] as const,args,environment).language as LandLanguageV1,notice=ports.notice??writeDiagnosticLineV1,controller=new AbortController(),abort=():void=>controller.abort();
 try{
  const options=landArgumentsV1(args),shown=options.language??language,policy=ports.policy??readLandPolicyV1(),cwd=ports.directory??process.cwd(),text=(key:LandTextKeyV1,values:Record<string,string|number>={})=>landTextV1(shown,key,values);
  const keyword=modes.has(options.words[0]!)?options.words[0]!:"apply",rest=modes.has(options.words[0]!)?options.words.slice(1):options.words,lane=options.lane??environment.SEMIO_LAND_LANE??"agent";
  if(keyword==="checkout"||keyword==="delete"){
   const directory=resolve(cwd,rest[0]??"");
   if(rest.length<2)throw refuseLandV1("usage","usage");
   const lines:string[]=[];
   if(keyword==="checkout"){
    const {rows}=landCheckoutV1(root,directory,rest.slice(1),{lane,title:options.title??""},cwd,policy);
    for(const row of rows)lines.push(text(row.state==="new"?"checkout.new":"checkout.existing",{path:row.path}));
   }else{landDeleteV1(root,directory,rest.slice(1),cwd,policy);for(const path of rest.slice(1))lines.push(text("checkout.deleted",{path}));}
   lines.push(text("checkout.done",{count:rest.length-1,directory}));
   write(options.json?JSON.stringify({version:1,directory,paths:rest.slice(1)},null,2):lines.join("\n"));
   return 0;
  }
  if(keyword==="status"){
   const lockRoot=resolve(root,policy.locks.root),held=existsSync(lockRoot)?readdirSync(lockRoot).filter(name=>name.startsWith(policy.locks.directoryPrefix)).map(name=>({area:name.slice(policy.locks.directoryPrefix.length),owner:(()=>{try{return readFileSync(resolve(lockRoot,name,policy.locks.ownerFile),"utf8").trim();}catch{return "?";}})()})):[],state=landSlotStateV1(new Date(),policy);
   if(options.json)write(JSON.stringify({version:1,slot:state,locks:held},null,2));
   else write([text("status.slot",{state:text(state.open?"state.open":"state.closed"),next:state.next}),...(held.length?held.map(row=>text("status.held",row)):[text("status.free")])].join("\n"));
   return 0;
  }
  const directory=keyword==="rollback"?undefined:resolve(cwd,rest[0]??"");
  if(rest.length<1||(directory!==undefined&&!(existsSync(directory)&&statSync(directory).isDirectory())))throw directory===undefined?refuseLandV1("usage","usage"):refuseLandV1("missing","stage.missing",{directory:rest[0]??""});
  process.once("SIGINT",abort);process.once("SIGTERM",abort);
  let result:LandResultV1;
  try{result=await landRunV1(root,{mode:keyword==="rollback"?"rollback":keyword==="check"?"check":"land",directory,id:keyword==="rollback"?rest[0]:undefined,options,language:shown,environment},{notice,signal:controller.signal,...ports.run},policy);}
  finally{process.off("SIGINT",abort);process.off("SIGTERM",abort);}
  const problems=landResultProblemsV1(result);
  if(problems.length)throw refuseLandV1("failed","result.invalid",{details:problems.slice(0,3).join("; ")});
  write(options.json?JSON.stringify(result,null,2):renderLandResultV1(result,shown));
  return result.status==="queued"?3:result.status==="failed"?1:result.status==="refused"?2:0;
 }catch(error){
  if(!isRepoErrorV1(error))throw error;
  notice(renderRepoErrorV1(error,language));
  return 2;
 }
}
