import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {withViteOptimizerDrainV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json" with {type:"json"};

test("Vite optimizer drainage outcome corpus agrees with Ajv and SQLite",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");try{db.exec("create table reason(ordinal integer primary key,kind text)");
 for(const row of corpus.cases){db.exec("delete from reason");db.query("insert into reason values(0,?)").run(row.operation);for(const[i,kind]of row.processing.entries())db.query("insert into reason values(?,?)").run(i+1,kind);const oracle=db.query("select kind from reason where kind<>'value' order by ordinal limit 1").get() as {kind:string}|null;expect(oracle?.kind??"value").toBe(row.expected);
 const error=new Error("actual optimizer failure"),reason=(kind:string)=>kind==="undefined"?undefined:error,events:string[]=[];let failed=false,actual:unknown;
 try{actual=await withViteOptimizerDrainV1(async()=>{events.push("operation");if(row.operation!=="value")throw reason(row.operation);return "value";},()=>row.processing.map(kind=>kind==="value"?Promise.resolve():Promise.reject(reason(kind))),()=>{events.push("release");});}catch(value){failed=true;actual=value;}
 expect(failed).toBe(row.expected!=="value");expect(actual).toBe(row.expected==="value"?"value":reason(row.expected));expect(events).toEqual(["operation","release"]);
 }}finally{db.close();}
});

test("Vite optimizer drainage joins accepted work before capacity release and preserves undefined",async()=>{
 let resolveFirst:()=>void=()=>{},rejectSecond:(reason:unknown)=>void=()=>{},released=0,settled=false;const first=new Promise<void>(resolve=>{resolveFirst=resolve;}),second=new Promise<void>((_,reject)=>{rejectSecond=reject;});
 const task=withViteOptimizerDrainV1(async()=>42,()=>[first,second],()=>{released++;});const observed=task.then(value=>({failed:false,value}),reason=>({failed:true,value:reason}));void observed.then(()=>{settled=true;});
 await Promise.resolve();rejectSecond(undefined);await Promise.resolve();expect(released).toBe(0);expect(settled).toBe(false);resolveFirst();const result=await observed;expect(result.failed).toBe(true);expect(result.value).toBeUndefined();expect(released).toBe(1);
 let closes=0;const primary=new Error("operation primary"),secondary=new Error("snapshot failure");await expect(withViteOptimizerDrainV1(async()=>{throw primary;},()=>{throw secondary;},()=>{closes++;})).rejects.toBe(primary);expect(closes).toBe(1);
 await expect(withViteOptimizerDrainV1(async()=>1,()=>{throw secondary;},()=>{})).rejects.toBe(secondary);
 const release=new Error("release failure");await expect(withViteOptimizerDrainV1(async()=>1,()=>[],()=>{throw release;})).rejects.toBe(release);
});

test("actual Vite admitted operation retains scoped capacity through optimizer drain",()=>{
 const path=resolve(import.meta.dir,"../../../🟦️.ts"),source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.ES2022,true);const calls:ts.CallExpression[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="withViteOptimizerDrainV1")calls.push(node);node.forEachChild(visit);};visit(source);expect(calls).toHaveLength(1);const call=calls[0];if(!call)throw Error("Actual drain call missing");expect(call.arguments[0]?.getText(source)).toContain("phase.signal.throwIfAborted()");expect(call.arguments[0]?.getText(source)).toContain("return operation()");expect(call.arguments[1]?.getText(source)).toContain("environment.depsOptimizer?.metadata.discovered");expect(call.arguments[2]?.getText(source)).toContain("checkResourceLeaseControl(phase)");expect(source.text).toContain("pending.add(task)");expect(source.text).toContain("withCompilerCapacityScopeV1");
});
