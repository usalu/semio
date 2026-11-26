import {createInterface} from "node:readline/promises";
import {spawnSync} from "node:child_process";
import {resolveLanguageV1} from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🗣️language/🟦️.ts";
import {writeResultLineV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";
import {collectFleetFactsV1,FLEET_READERS_V1,type FleetReadersV1} from "../📥️readers/🟦️.ts";
import {fleetTextV1,formatFleetReportV1,summarizeFleetV1,validateFleetReportV1,type FleetLanguageV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 📝️ The closed arguments of `repo fleet status`. */
export type FleetArgumentsV1=Readonly<{json:boolean;limit:number}>;
const DEFAULT_LIMIT=20,both=(key:string,values:Readonly<Record<string,string|number>>={}):string=>fleetTextV1("en",key,values)+"\n"+fleetTextV1("de",key,values);

/** 🧷️ Reads the options of `repo fleet status`; the global `--lang <en|de>` / `--lang=<en|de>` is consumed here as it is for the language resolution (`--language` is no flag of `repo`), every unknown or repeated option is refused in both languages. */
export function fleetArgumentsV1(args:readonly string[]):FleetArgumentsV1{
 if(args[0]!=="status")throw Error(both("error.syntax"));
 let json=false,limit=DEFAULT_LIMIT;const seen=new Set<string>(),rest=args.slice(1);
 for(let at=0;at<rest.length;at++)if(rest[at]==="--lang")rest.splice(at,2);else if(rest[at]!.startsWith("--lang="))rest.splice(at--,1);
 for(const argument of rest){
  const name=argument.startsWith("--limit=")?"--limit":argument;
  if(seen.has(name)||name!=="--json"&&name!=="--limit")throw Error(both("error.syntax"));
  seen.add(name);
  if(name==="--json")json=true;
  else{const value=Number(argument.slice(8)),maximum=schema.definitions.Policy.const.runs.maximumRows;if(!Number.isSafeInteger(value)||value<1||value>maximum)throw Error(both("error.limit",{maximum}));limit=value;}
 }
 return Object.freeze({json,limit});
}

/** 🎛️ What the command touches besides the checkout: the readers, the clock and the output; tests replace them. */
export type FleetCommandPortsV1=Readonly<{readers?:FleetReadersV1;now?:()=>number;hostname?:Readonly<{interactive?:boolean;answer?:(question:string)=>Promise<string>;run?:(command:string,args:readonly string[])=>Promise<number>;platform?:string}>;refuse?:(text:string)=>void}>;

/** ▶️ Runs `repo fleet status [--json] [--limit=<n>]`: reads the existing status readers in parallel, aggregates, validates the report against its closed schema and prints it as compact text in the chosen language or as one JSON document. It takes no lease, loads no graph and writes nothing. */
export async function runFleetCliV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv,write:(line:string)=>void=writeResultLineV1,ports:FleetCommandPortsV1={}):Promise<number>{
  if(args[0]==="hostname"){
  const language:FleetLanguageV1=resolveLanguageV1(["en","de"] as const,args,environment).language,rest=args.slice(1).filter((argument,at,all)=>argument!=="--lang"&&!argument.startsWith("--lang=")&&all[at-1]!=="--lang");
  if(rest.length!==1||rest[0]!=="apply")throw Error(both("error.syntax"));
  const {runTopologyCommandAsyncV1}=await import("../../../../🤝️shared-worktree/🗺️topology/⌨️command/🟦️.ts"),{fleetAddressTextV1}=await import("../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts"),own=ports.hostname??{};
  const refusal=fleetAddressTextV1("en","hostname.tty")+"\n"+fleetAddressTextV1("de","hostname.tty");
  const confirm=async(question:string):Promise<boolean>=>{
   if(!(own.interactive??Boolean(process.stdin.isTTY)))throw Error(refusal);
   if(own.answer!==undefined)return /^(y|yes|j|ja)$/iu.test((await own.answer(question)).trim());
   const lines=createInterface({input:process.stdin,output:process.stdout});
   try{return /^(y|yes|j|ja)$/iu.test((await lines.question(question)).trim());}finally{lines.close();}
  };
  const run=own.run??(async(command:string,commandArguments:readonly string[]):Promise<number>=>spawnSync(command,[...commandArguments],{stdio:"inherit"}).status??1);
  const result=await runTopologyCommandAsyncV1(["hostname","--root",root,"--lang",language],{platform:own.platform??process.platform,signal:new AbortController().signal,progress:write,environment,confirm,run}).catch((error:unknown)=>{if(error instanceof Error&&error.message===refusal){(ports.refuse??console.error)(refusal);return {text:"",exitCode:2};}throw error;});
  if(result.text!=="")write(result.text);
  return result.exitCode;
 }
 const parsed=fleetArgumentsV1(args),language:FleetLanguageV1=resolveLanguageV1(["en","de"] as const,args,environment).language,now=ports.now??Date.now;
 const facts=await collectFleetFactsV1({root,environment,language,now},ports.readers??FLEET_READERS_V1),report=summarizeFleetV1(facts,language),problems=validateFleetReportV1(report);
 if(problems.length>0)throw Error(fleetTextV1(language,"error.invalid",{details:problems.slice(0,3).join("; ")}));
 if(parsed.json)write(JSON.stringify(report,null,2));
 else for(const line of formatFleetReportV1(language,report,parsed.limit))write(line);
 return 0;
}
