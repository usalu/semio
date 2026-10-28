import {expect,test} from "bun:test";
import {randomUUID} from "node:crypto";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";
import runtimeCorpus from "./🔣️runtime.json";
import runtimeSchema from "./🧬️runtime.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../📥️caller/🟦️.ts";
import {type BazelCommandCapturerV1} from "../../🌿️environment/🟦️.ts";
import {bazelTaskFingerprintHeadV1} from "../../📜️script.ts";
import ambientCorpus from "./🔣️ambient.json";
import ambientSchema from "./🧬️ambient.json";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const graph={projects:{fixture:{root:"fixture",targets:{}}},dependencies:[]} as never,closures={files:()=>[]} as never;
const head=(original:never,owner:string,task:unknown,environment:NodeJS.ProcessEnv,capture:(program:string,args:string[],root:string,options:{env:NodeJS.ProcessEnv})=>Promise<string>,resolutions:ReadonlyMap<string,string>=new Map())=>bazelTaskFingerprintHeadV1(process.cwd(),graph,task as never,new Map(),original,createIssuedResourceControlV1(original,owner,0),true,"implementation",closures,resolutions,{environment,capture:(async(program,args,cwd,options)=>({stdout:await capture(program,[...args],cwd,{env:options.env as NodeJS.ProcessEnv}),stderr:"",status:0,signal:null})) satisfies BazelCommandCapturerV1});

test("Bazel fingerprints declared env and runtime inputs through the actual issued task environment",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const root=process.cwd(),database=new Database(":memory:"),results:{group:string;fingerprint:string;environmentFingerprint:string}[]=[];
 try{for(const[index,row]of corpus.entries()){
  const capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:row.grant.mode,CARGO_NET_OFFLINE:String(row.grant.offline)}),task={id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"repo ./📜️script.ts",env:row.taskEnvironment},inputs:[...Object.keys(row.expected).map(env=>({env})),{runtime:"fixture-runtime"}]}},runtime:NodeJS.ProcessEnv[]=[],runtimeOutputs:string[]=[];
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:fingerprint",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
   const base=Object.fromEntries(Object.entries({...process.env,...row.ambient}).filter(([_key,value])=>value!==null)),run=(selected:unknown,owner:string)=>head(original as never,owner,selected,base,async(_program,_arguments,_root,options)=>{runtime.push(options.env);const program="process.stdout.write(JSON.stringify(Object.fromEntries(JSON.parse(process.argv[1]).map(key=>[key,process.env[key]??null]))))",child=Bun.spawnSync([process.execPath,"-e",program,JSON.stringify(Object.keys(row.expected))],{env:options.env});expect(child.exitCode).toBe(0);const output=child.stdout.toString();runtimeOutputs.push(output);return output;}),fingerprint=await run(task,"bazel:fingerprint");expect(runtime).toHaveLength(1);expect(Object.fromEntries(Object.keys(row.expected).map(key=>[key,runtime[0]![key]??null]))).toEqual(row.expected);database.exec("CREATE TABLE IF NOT EXISTS cases(id INTEGER,key TEXT,value TEXT)");for(const[key,value]of Object.entries(row.expected))database.query("INSERT INTO cases VALUES(?,?,?)").run(index,key,value);const oracle=Object.fromEntries(database.query("SELECT key,value FROM cases WHERE id=? ORDER BY key").all(index).map(value=>[(value as{key:string}).key,(value as{value:string|null}).value]));expect(JSON.parse(runtimeOutputs[0]!)).toEqual(oracle);const environmentTask={...task,definition:{...task.definition,inputs:task.definition.inputs.filter(input=>"env"in input)}},environmentFingerprint=await run(environmentTask,"bazel:environment-fingerprint");for(const previous of results){expect(fingerprint===previous.fingerprint).toBe(row.cacheGroup===previous.group);expect(environmentFingerprint===previous.environmentFingerprint).toBe(row.cacheGroup===previous.group);}results.push({group:row.cacheGroup,fingerprint,environmentFingerprint});
  });
 }}finally{database.close();}
},30000);

test("Bazel runtime input boundaries cannot alias different ordered command outputs",async()=>{
 expect(new Ajv({strict:true}).validate(runtimeSchema,runtimeCorpus)).toBe(true);const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{}),database=new Database(":memory:");
 try{await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:runtime-boundary",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  const fingerprints:string[]=[],task={id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"repo ./📜️script.ts"},inputs:runtimeCorpus.commands.map(runtime=>({runtime}))}};
  for(const outputs of runtimeCorpus.outputs){let position=0;fingerprints.push(await head(original as never,"bazel:runtime-boundary",task,process.env,async()=>outputs[position++]!));expect(position).toBe(runtimeCorpus.commands.length);}
  const oracle=database.query("SELECT json_array(?1,?2)<>json_array(?3,?4) distinctOutput").get(...runtimeCorpus.outputs.flat()) as{distinctOutput:number};expect(Boolean(oracle.distinctOutput)).toBe(runtimeCorpus.distinct);expect(fingerprints[0]!==fingerprints[1]).toBe(runtimeCorpus.distinct);
 });}finally{database.close();}
},30000);


test("Bazel folds the crate resolution identity into the fingerprint, never a lock file, and refuses a lock marker without one",async()=>{
 const root=process.cwd(),key="a".repeat(64),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:"prototype"}),task={id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"repo ./📜️script.ts",env:{}},inputs:[{env:"SEMIO_CARGO_LOCK_"+key}]}};
 const fingerprint=(resolutions:Map<string,string>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:fingerprint",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  return head(original as never,"bazel:fingerprint",task,{...process.env},async()=>"",resolutions);
 });
 const first=await fingerprint(new Map([[key,"1".repeat(64)]])),same=await fingerprint(new Map([[key,"1".repeat(64)]])),moved=await fingerprint(new Map([[key,"2".repeat(64)]]));
 expect(same).toBe(first);expect(moved).not.toBe(first);
 await expect(fingerprint(new Map())).rejects.toThrow("Bazel Cargo resolution identity required");
},30000);

test("Bazel action keys never depend on an ambient host, agent or session variable or on a diagnostic switch, only on the declared inputs",async()=>{
 expect(new Ajv({strict:true}).validate(ambientSchema,ambientCorpus)).toBe(true);const root=process.cwd(),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run","fixture:build"],"bazel",{SEMIO_BUILD_MODE:"prototype"}),task={id:"fixture:build",project:"fixture",target:"build",root:"fixture",args:[],definition:{execution:{kind:"command",command:"repo ./📜️script.ts",env:{}},inputs:[{env:"FIXTURE_DECLARED"},{runtime:"fixture-runtime"}]}},fingerprints:string[]=[];
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:ambient-fingerprint",maximumElapsedMilliseconds:30000},capabilities,Date.now()),async original=>{
  for(const row of ambientCorpus.rows){
   const base=Object.fromEntries(Object.entries({...process.env,...row.ambient}).filter(([_key,value])=>value!==null)),capture=async(_program:string,_arguments:string[],_root:string,options:{env:NodeJS.ProcessEnv})=>"v1.2.3\n"+(options.env.SEMIO_REPO_PROFILE===undefined?"":JSON.stringify({pid:randomUUID()})+"\n")+(Object.keys(options.env).some(name=>name.startsWith("CLAUDE"))?"agent\n":"");
   fingerprints.push(await head(original as never,"bazel:ambient-fingerprint",task,base,capture));
  }
 });
 for(const[index,row]of ambientCorpus.rows.entries())expect(fingerprints[index]===fingerprints[0],row.name).toBe(row.sameAsFirst);
},30000);
