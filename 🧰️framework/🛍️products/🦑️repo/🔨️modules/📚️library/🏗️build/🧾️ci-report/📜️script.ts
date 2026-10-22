import {buildBackendAlternationV1,requireBuildBackendIdV1} from "../🔌️backends/🟦️.ts";
import {existsSync,readdirSync,readFileSync,statSync} from "node:fs";
import {join,relative} from "node:path";
import {ciCacheDeltaV1,compareCiPlansV1,judgeCiCacheProofV1,readCiCacheStatusV1,readCiPlanV1,readCiTestResultsIndexV1,renderCiCacheDeltaV1,renderCiParityV1,renderCiPlanV1,renderCiProofV1,renderCiTestsV1,summarizeCiPlanV1,summarizeCiTestsV1,type CiToolV1} from "./🟦️.ts";
import { isMainModuleV1 } from "../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";
import {resolveLanguageV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";

const usage=`Usage: bun script.ts <command> [--lang en|de] ...
  cache-directory --status <file>
  cache-delta  --tool ${buildBackendAlternationV1()} --before <file> --after <file> [--label <text>]
  proof        --tool ${buildBackendAlternationV1()} --status0 <file> --status1 <file> --status2 <file> --log1 <file> --log2 <file>
  plan         --plan <file> --targets <list> [--label <text>]
  parity       --first <file> --second <file> --targets <list>
  tests        --dir <directory> [--require]
Every command reads files that \`bun repo\` wrote, prints Markdown, and exits 1 only when a verdict fails (proof, parity, tests --require).`;

const flags:Readonly<Record<string,readonly string[]>>={
 "cache-directory":["status","lang"],
 "cache-delta":["tool","before","after","label","lang"],
 proof:["tool","status0","status1","status2","log1","log2","lang"],
 plan:["plan","targets","label","lang"],
 parity:["first","second","targets","lang"],
 tests:["dir","require","lang"]
};

function parse(command:string,args:readonly string[]):Map<string,string>{
 const allowed=flags[command]!,values=new Map<string,string>();
 for(let index=0;index<args.length;index++){
  const argument=args[index]!;
  if(!argument.startsWith("--")||!allowed.includes(argument.slice(2)))throw Error(`Unknown option ${argument}`);
  const name=argument.slice(2);
  if(name==="require"){values.set(name,"true");continue;}
  const value=args[++index];
  if(value===undefined||value.startsWith("--"))throw Error(`Option ${argument} needs a value`);
  values.set(name,value);
 }
 return values;
}

const need=(values:Map<string,string>,name:string):string=>{const value=values.get(name);if(value===undefined)throw Error(`Option --${name} required`);return value;};
const tool=(values:Map<string,string>):CiToolV1=>requireBuildBackendIdV1(need(values,"tool"));
const read=(path:string):string=>readFileSync(path,"utf8");

function xmlFiles(directory:string):string[]{
 const found:string[]=[];
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  const path=join(directory,entry.name);
  if(entry.isDirectory())found.push(...xmlFiles(path));
  else if(entry.name.endsWith(".xml"))found.push(path);
 }
 return found.sort();
}

function run(command:string,values:Map<string,string>,environment:Readonly<Record<string,string|undefined>>):{text:string;ok:boolean}{
 const flag=values.get("lang"),language=resolveLanguageV1(CATALOG_LANGUAGES_V1,flag===undefined?[]:["--lang",flag],environment).language;
 if(command==="cache-directory"){
  const status=readCiCacheStatusV1(read(need(values,"status")));
  return {text:`${status.available&&status.directory!==null?status.directory:""}\n`,ok:true};
 }
 if(command==="cache-delta"){
  const before=readCiCacheStatusV1(read(need(values,"before"))),after=readCiCacheStatusV1(read(need(values,"after")));
  tool(values);
  return {text:renderCiCacheDeltaV1(ciCacheDeltaV1(before,after),values.get("label")??after.tool,after.activity!==undefined,language),ok:true};
 }
 if(command==="proof"){
  const statuses=["status0","status1","status2"].map(name=>readCiCacheStatusV1(read(need(values,name)))) as [ReturnType<typeof readCiCacheStatusV1>,ReturnType<typeof readCiCacheStatusV1>,ReturnType<typeof readCiCacheStatusV1>];
  const proof=judgeCiCacheProofV1({tool:tool(values),statuses,logs:[read(need(values,"log1")),read(need(values,"log2"))]});
  return {text:renderCiProofV1(proof,language)+"\n",ok:proof.ok};
 }
 if(command==="plan"){
  const targets=need(values,"targets");
  return {text:renderCiPlanV1(summarizeCiPlanV1(readCiPlanV1(read(need(values,"plan"))),targets),values.get("label")??"plan",language),ok:true};
 }
 if(command==="parity"){
  const parity=compareCiPlansV1(readCiPlanV1(read(need(values,"first"))),readCiPlanV1(read(need(values,"second"))),need(values,"targets"));
  return {text:renderCiParityV1(parity,language)+"\n",ok:parity.equal};
 }
 const directory=need(values,"dir"),indexPath=join(directory,"index.json");
 const files=existsSync(directory)&&statSync(directory).isDirectory()?xmlFiles(directory).map(path=>({file:relative(directory,path).split("\\").join("/"),xml:read(path)})):[];
 const summary=summarizeCiTestsV1(files,existsSync(indexPath)?readCiTestResultsIndexV1(read(indexPath)):undefined);
 return {text:renderCiTestsV1(summary,language),ok:values.has("require")?summary.files>0&&summary.failed.length===0:true};
}

/** 🚦️ Runs one report command; the exit code is 1 only for a failed verdict, 2 for a usage problem. */
export function runCiReportV1(argv:readonly string[],environment:Readonly<Record<string,string|undefined>>,write:(text:string)=>void):number{
 const command=argv[0];
 if(command===undefined||!(command in flags)){write(usage+"\n");return command===undefined||command==="--help"?0:2;}
 try{
  const {text,ok}=run(command,parse(command,argv.slice(1)),environment);
  write(text);
  if(!ok&&environment.GITHUB_ACTIONS==="true")write(`::error::${command} failed\n`);
  return ok?0:1;
 }catch(error){write(`${(error as Error).message}\n`);return 2;}
}

if(isMainModuleV1(import.meta.url))process.exit(runCiReportV1(process.argv.slice(2),process.env,text=>process.stdout.write(text)));
