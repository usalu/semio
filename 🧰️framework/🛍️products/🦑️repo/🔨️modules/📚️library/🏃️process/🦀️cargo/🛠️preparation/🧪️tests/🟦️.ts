import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,writeFileSync,readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {setImmediate} from "node:timers/promises";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {runRepositoryExactCargoLaws} from "../../🎯️exact/🟦️.ts";
import {runOwnedCommand,captureOwnedProcess,type OwnedCommandOptions,type OwnedProcessCaptureOptions} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../📦️packages/🟦️typescript/🧬️schema/🟦️.ts";

const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifactRoot)throw Error("Original ticket artifact owner required");
const oracle='const row=JSON.parse(process.argv[1]),cp=require("node:child_process");let prepare=0,execute=0,capture=0;try{if(row.mutation==="missing-policy")throw Error("Complete repository preparation policy required");if(row.mutation==="publisher-refused")throw Error("Original preparation publication refused");prepare++;execute++;const child=cp.spawnSync(process.execPath,["--eval",row.mutation==="line-limit"?"console.log(1);console.log(2);console.log(3)":"process.exitCode=0"],{timeout:7000});if(child.status!==0)throw Error("Independent child failed");if(child.stdout.toString().split("\\n").filter(Boolean).length>2)throw Error("output line limit");capture++;}catch(error){if(!/policy required|publication refused|output line limit/.test(String(error)))throw error;}console.log(JSON.stringify({prepare,execute,capture}));';

test("Repository preparation line authority uses production schema and independent Ajv",()=>{
 const admit=new Ajv({strict:true}).compile(schema);
 expect(admit(corpus.policy)).toBe(true);
 expect(validateJsonSchemaSubset(schema,corpus.policy)).toEqual([]);
 for(const value of [{version:1},{...corpus.policy,maximumOutputLines:0},{...corpus.policy,maximumOutputLines:65537},{...corpus.policy,maximumOutputLines:1.5},{...corpus.policy,maximumOutputBytes:4096}]){expect(admit(value)).toBe(false);expect(validateJsonSchemaSubset(schema,value).length).toBeGreaterThan(0);}
});

for(const row of corpus.cases)test(row.id,async()=>{
 const root=mkdtempSync(join(artifactRoot,"rp12-")),output=join(root,"actual"),started=performance.now(),controller=new AbortController(),timer=globalThis.setTimeout(()=>controller.abort(Error("Original preparation caller timeout")),corpus.callerCeilingMilliseconds),ledger:string[]=[];
 mkdirSync(join(root,"src"));writeFileSync(join(root,"Cargo.toml"),'[package]\nname="preparation-receiver"\nversion="0.1.0"\n[workspace]\n');writeFileSync(join(root,"src/lib.rs"),"pub fn original() {}\n");
 let parentRemaining=5000.5,prepare=0,execute=0,capture=0,budget:number|null=null,signal=false,lines:number|null=null,bytes:number|null=null,executedFractional=false;
 const invocation:ScriptInvocation={policy:{version:1,owner:row.id,maximumElapsedMilliseconds:10000},capabilities:{artifactDirectory:output,buildMode:"prototype"},control:{signal:controller.signal,remainingMilliseconds:()=>Math.min(parentRemaining,10000-(performance.now()-started)),async publish(event){ledger.push("publish:"+event.command);await setImmediate();if(event.command==="repository:exact-cargo-preparation"&&row.mutation==="publisher-refused")throw Error("Original preparation publication refused");},async yieldContinuation(){ledger.push("yield");await setImmediate();}}};
 const port={fingerprint(){throw Error("Refused compiler has no executable");},prepare(_command:string,_args:string[],options:OwnedProcessCaptureOptions){expect(_args.slice(_args.indexOf("--profile"),_args.indexOf("--profile")+2)).toEqual(["--profile","prototype"]);expect(options.invocation).toBe(invocation);expect(ledger.at(-1)).toBe("yield");prepare++;return{command:"node",args:["--eval",row.mutation==="line-limit"?"console.log(1);console.log(2);console.log(3)":"process.exitCode=0"],cwd:root,environment:process.env};},async execute(command:string,args:string[],cwd:string,label:string,childBudget:number,options:OwnedCommandOptions){execute++;executedFractional=childBudget>0&&!Number.isInteger(childBudget);signal=options.signal===controller.signal;lines=options.output?.maximumLines??null;bytes=options.output?.maximumBytes??null;await runOwnedCommand(command,args,cwd,label,childBudget,options);parentRemaining=row.mutation==="submillisecond"?0.75:125.5;},async capture(_command:string,_args:string[],options:OwnedProcessCaptureOptions){capture++;budget=options.budgetMs;expect(options.invocation).toBe(invocation);return await captureOwnedProcess("node",["--eval","process.exitCode=101"],options);}};
 const oracleChild=Bun.spawn(["node","--eval",oracle,JSON.stringify(row)],{stdout:"pipe",stderr:"pipe"});
 let refusal="";
 try{
  try{await (runRepositoryExactCargoLaws as unknown as (options:object,port:object)=>Promise<unknown>)({invocation,policy:corpus.exactPolicy,...(row.mutation==="missing-policy"?{}:{preparation:corpus.policy}),cwd:root,manifestPath:join(root,"Cargo.toml"),artifactDir:output,env:process.env,groups:[{package:"preparation-receiver",target:{kind:"lib"},laws:["original_law"]}]},port);}catch(error){refusal=String(error);}
  const [stdout,stderr,status]=await Promise.all([new Response(oracleChild.stdout).text(),new Response(oracleChild.stderr).text(),oracleChild.exited]);expect(status,stderr).toBe(0);
  const expected={prepare:row.expectedPrepare,execute:row.expectedExecute,capture:row.expectedCapture};expect(JSON.parse(stdout)).toEqual(expected);expect({prepare,execute,capture}).toEqual(expected);
  if(execute){expect(signal).toBe(true);expect(executedFractional).toBe(true);expect(lines).toBe(corpus.policy.maximumOutputLines);expect(bytes).toBe(corpus.exactPolicy.transport.maximumBuildOutputBytes);}
  if(capture){expect(budget).toBeGreaterThan(0);expect(budget).toBeLessThanOrEqual(row.mutation==="submillisecond"?0.75:125.5);expect(Number.isInteger(budget)).toBe(false);}
  if(row.mutation==="missing-policy")expect(refusal).toContain("Complete repository preparation policy required");
  if(row.mutation==="publisher-refused")expect(refusal).toContain("Original preparation publication refused");
  if(row.mutation==="line-limit")expect(refusal).toContain("output line limit");
  console.log("[DEBUG] "+JSON.stringify({id:row.id,prepare,execute,capture,budget,signal,lines,bytes,executedFractional,refusal,ledger}));
 }finally{clearTimeout(timer);if(oracleChild.exitCode===null){oracleChild.kill();await oracleChild.exited;}}
},corpus.callerCeilingMilliseconds);

/** 🎚️ Exact native laws use only the original supplied mode, with independent schema and TOML oracles. */
test("Repository exact native law profile belongs to the original caller",async()=>{
 const workspace=resolve(import.meta.dir,"../../../../../../../../.."),modeSchema=JSON.parse(readFileSync(join(workspace,"🧰️framework/🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json"),"utf8")),profiles=(Bun.TOML.parse(readFileSync(join(workspace,".cargo/config.toml"),"utf8")) as {profile:Record<string,unknown>}).profile,admit=new Ajv({strict:true}).compile(modeSchema);
 for(const row of corpus.profiles){const root=mkdtempSync(join(artifactRoot!,"rp-mode-"));mkdirSync(join(root,"src"));writeFileSync(join(root,"Cargo.toml"),'[package]\nname="preparation-profile"\nversion="0.1.0"\n[workspace]\n');writeFileSync(join(root,"src/lib.rs"),"pub fn original() {}\n");const controller=new AbortController(),invocation:ScriptInvocation={policy:{version:1,owner:"original-profile",maximumElapsedMilliseconds:10000},capabilities:{buildMode:row.caller},control:{signal:controller.signal,remainingMilliseconds:()=>5000,async publish(){},async yieldContinuation(){}}};let selected:string|undefined,refusal="";
  const port={fingerprint(){throw Error("refused compiler has no executable")},prepare(_command:string,args:string[]){selected=args[args.indexOf("--profile")+1];return undefined},async execute(){throw Error("profile law must not create preparation child")},async capture(){return{status:101,signal:null,stdout:"",stderr:"original selected profile"}}};
  try{await runRepositoryExactCargoLaws({invocation,preparation:corpus.policy,policy:corpus.exactPolicy,cwd:root,manifestPath:join(root,"Cargo.toml"),artifactDir:root,env:{...process.env,SEMIO_BUILD_MODE:row.environment},cargoArgs:row.override??[],groups:[{package:"preparation-profile",target:{kind:"lib"},laws:["original_law"]}]},port)}catch(error){refusal=String(error)}
  if(row.refused){expect(selected).toBeUndefined();expect(refusal).toMatch(/Build mode must|profile belongs/)}else{expect(admit(row.caller)).toBe(true);expect(profiles[row.profile!]).toBeDefined();expect(selected).toBe(row.profile);expect(refusal).toContain("build failed")}
 }
 console.log("[DEBUG] exact native originalMode=true explicitPrototypeRelease=true environmentCannotReplace=true overridesDenied=true independentAjvTOML=true");
});
