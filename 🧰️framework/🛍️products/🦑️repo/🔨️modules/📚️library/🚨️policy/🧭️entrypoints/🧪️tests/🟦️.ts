/** 🧭️ Contract: bundle entry dispatch hands the remaining arguments to the handler named by the first argument (or the default) and prints the usage line and exits 1 for anything else, and only the exact `policy` argument starts a policy run; the corpus is language-agnostic and `🐍️.py` models the handler selection. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {dispatchPolicyArgv,dispatchSubcommand} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
class Exit extends Error{constructor(readonly code:number){super("exit "+code);}}
const guarded=<T,>(action:()=>T):{value?:T;errors:string[];exit:number|null}=>{const errors:string[]=[],error=console.error,exit=process.exit;let code:number|null=null,value:T|undefined;console.error=(...parts:unknown[])=>{errors.push(parts.join(" "));};(process as unknown as {exit:(c?:number)=>never}).exit=(c?:number)=>{throw new Exit(c??0);};try{value=action();}catch(thrown){if(thrown instanceof Exit)code=thrown.code;else throw thrown;}finally{console.error=error;process.exit=exit;}return{value,errors,exit:code};};
for(const[index,row]of corpus.dispatch.entries())test("subcommand dispatch "+index+": "+JSON.stringify(row.segments),()=>{
 const calls:Record<string,string[]|undefined>={};const handlers:Record<string,(rest:string[])=>void>={a:rest=>{calls.a=rest;},run:rest=>{calls.run=rest;}};
 const outcome=guarded(()=>dispatchSubcommand(row.segments,handlers,corpus.usageText,row.default??undefined));
 if(row.usage){expect(outcome.exit).toBe(1);expect(outcome.errors).toEqual(["usage: "+corpus.usageText]);}else{expect(outcome.exit).toBeNull();expect(calls[row.key!]).toEqual(row.calls!);}
});
test("an async handler's promise is returned to the caller",async()=>{const result=dispatchSubcommand(["a"],{a:async()=>{await Promise.resolve();}},"u");expect(result).toBeInstanceOf(Promise);await result;});
for(const row of corpus.defects)test.failing("subcommand dispatch defect "+JSON.stringify(row.segments),()=>{const outcome=guarded(()=>dispatchSubcommand(row.segments,{a:()=>{}},corpus.usageText));expect(outcome.exit).toBe(1);});
for(const row of corpus.policy)test("policy argv "+JSON.stringify(row.segments)+" is not a policy run",async()=>{expect(await dispatchPolicyArgv(row.segments,import.meta.url)).toBe(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
