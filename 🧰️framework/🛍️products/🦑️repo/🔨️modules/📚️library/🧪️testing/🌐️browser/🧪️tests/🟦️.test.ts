import{ensureRepositoryBrowserPathV1}from"../🧰️preparation/🟦️.ts";
import{repoCacheDirectory}from"../../../⚡️caching/🟦️.ts";
import {expect,test} from "bun:test";
import {Script} from "node:vm";
import {PLAYWRIGHT_MODULE_SPECIFIER,browserExecutableProbeV1,browserLaunchOptionsV1} from "../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";

test("every browser runner uses the portable software GPU policy",()=>{expect(browserLaunchOptionsV1()).toEqual({headless:true,args:corpus.arguments});});
for(const row of corpus.cases)test(row.name,async()=>{
 const visited:string[]=[],errors:string[]=[],result={exitCode:0};
 await new Script(browserExecutableProbeV1("playwright-registry")).runInNewContext({process:result,console:{error:(error:Error)=>errors.push(error.message)},require:(name:string)=>name==="playwright-registry"?{registry:{registry:{findExecutable:(id:string)=>({executablePathOrDie:()=>{visited.push(id);return row.paths[id as keyof typeof row.paths];},_validateHostRequirements:async()=>{if(!row.hostReady)throw Error("missing host libraries");}})}}}:{constants:{X_OK:1},accessSync:(path:string)=>{if(!row.files.includes(path))throw Error("missing executable");}}});
 expect(result.exitCode===0).toBe(row.ready);expect(visited).toEqual(row.expectedExecutables);expect(errors).toEqual(row.errors);
});

test("browser tool selection resolves the authored installed implementation",async()=>{expect(PLAYWRIGHT_MODULE_SPECIFIER).toBe(corpus.moduleSpecifier);const tool=await import(PLAYWRIGHT_MODULE_SPECIFIER);expect(typeof tool.chromium.executablePath).toBe("function");});

for(const row of corpus.pathCases)test(row.name,()=>{const root=process.cwd(),environment:Record<string,string|undefined>={PLAYWRIGHT_BROWSERS_PATH:row.existing??undefined},before=process.env.PLAYWRIGHT_BROWSERS_PATH;ensureRepositoryBrowserPathV1(root,environment);expect(environment.PLAYWRIGHT_BROWSERS_PATH).toBe(row.existing??repoCacheDirectory(root,"tools","ms-playwright"));expect(process.env.PLAYWRIGHT_BROWSERS_PATH).toBe(before);});
