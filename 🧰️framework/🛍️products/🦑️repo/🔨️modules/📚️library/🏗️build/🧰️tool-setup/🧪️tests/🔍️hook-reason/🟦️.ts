/** 🔍️ Law: a verify hook that fails says why. Its reason travels to the report and to the refusal of the setup in both languages (before: the hook error was dropped and a provisioned tool reported only `-`), and every tool is named before it is verified so a silent probe is never anonymous. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import Ajv from "ajv";
import setupSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {systemToolContextV1} from "../../⌨️command/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import {setupToolsV1,toolRowV1,toolSetupMessageV1,verifyToolV1,type ToolContextV1} from "../../🟦️.ts";

const signal=new AbortController().signal,row=toolRowV1("node-native-core");
const machine=(hook:ToolContextV1["hook"],environment:Record<string,string|undefined>={})=>({root:"/x",toolsRoot:"/x/t",platform:"linux",environment:{SEMIO_LANGUAGE:"en",...environment},home:"/h",read:()=>"{\"definitions\":{\"Protocol\":{\"const\":{\"version\":7}}}}",exists:()=>false,list:()=>[],resolvePackage:()=>"",capture:async()=>"",provisioned:()=>undefined,hook}) as unknown as ToolContextV1;
const failing=async()=>({found:null,location:null,reason:"Process exited with status 1: core file missing"});

test("the reason of a failed hook is the message of a missing tool in the report",async()=>{
 expect(await verifyToolV1(row,machine(failing),signal)).toMatchObject({status:"missing",found:null,message:"Process exited with status 1: core file missing"});
 expect(await verifyToolV1(row,machine(async()=>({found:null,location:null})),signal)).not.toHaveProperty("message");
});

test("a provisioned tool that still fails its verification names the reason in the refusal, in English and in German",async()=>{
 await expect(setupToolsV1([row],machine(failing),async()=>undefined,signal,()=>undefined)).rejects.toThrow(/reports -, not the pin 7\. Its verification said: Process exited with status 1: core file missing/u);
 await expect(setupToolsV1([row],machine(failing,{SEMIO_LANGUAGE:"de"}),async()=>undefined,signal,()=>undefined)).rejects.toThrow(/Seine Prüfung meldete: Process exited with status 1: core file missing/u);
});

test("every tool is named before its verification starts",async()=>{
 const lines:string[]=[];
 await setupToolsV1([row],machine(async(_row,pin)=>({found:pin,location:"s"})),async()=>undefined,signal,line=>lines.push(line));
 expect(lines).toEqual(["Verifying node-native-core against its pin.","node-native-core 7 already matches its pin."]);
 const german:string[]=[];
 await setupToolsV1([row],machine(async(_row,pin)=>({found:pin,location:"s"}),{SEMIO_LANGUAGE:"de"}),async()=>undefined,signal,line=>german.push(line));
 expect(german[0]).toBe("Prüfe node-native-core gegen seinen Pin.");
});

test("the setup says how many tools it starts with before it opens anything, in both languages",()=>{
 const source=readFileSync(join(import.meta.dir,"../../⌨️command/🟦️.ts"),"utf8");
 expect(source).toContain('console.log(toolSetupMessageV1("starting", {count: rows.length}, context.environment));');
 expect(source.indexOf('toolSetupMessageV1("starting"')).toBeLessThan(source.indexOf("const inStore"));
 expect(toolSetupMessageV1("starting",{count:3},{SEMIO_LANGUAGE:"en"})).toBe("Setting up 3 pinned tool(s).");
 expect(toolSetupMessageV1("starting",{count:3},{SEMIO_LANGUAGE:"de"})).toBe("Richte 3 gepinnte(s) Werkzeug(e) ein.");
});

test("a real verification child can report absence, followed by a successful probe under the same owner",async()=>{
 const root=import.meta.dir,script="🟨️.cjs";
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:tool-hook",maximumElapsedMilliseconds:10000},{},Date.now()),async original=>{
  const control=createIssuedResourceControlV1(original,"test:tool-hook",0),context=systemToolContextV1(root,control);
  for(const [mode,expected] of Object.entries(fixture)){
   const independent=spawnSync(process.execPath,[join(root,script),mode],{encoding:"utf8"});
   expect(independent.status).toBe(expected.code);
   expect(independent.stdout.trim()).toBe(expected.output);
   const candidate={...row,setup:{kind:"script" as const,script,prepare:[mode],verify:[mode]}};
   const result=await context.hook(candidate,"1",control.signal);
   expect(result.found,JSON.stringify(result)).toBe(expected.status==="ok"?"1":null);
   if(expected.status==="missing")expect(result.reason).toContain(expected.output);
  }
 });
});

const refusals = await import("../../🛑️refusal/🟦️.ts");
const settlement = await import("./📋️settlement.json", {with:{type:"json"}});
const validate = new Ajv({strict:true}).compile(setupSchema.definitions.SetupRefusal);
for (const entry of settlement.default.cases) test(`setup records the typed verification primary, ${entry.name}`, () => {
 const rows = entry.reports as readonly import("../../🟦️.ts").ToolReportV1[];
 const lines = refusals.toolSetupRefusalLinesV1(rows);
 const decoded = lines.map(refusals.readToolSetupRefusalLineV1);
 expect(decoded).toEqual(entry.expected);
 expect(decoded.every(row => validate(row))).toBe(true);
 expect(lines.every(line => !/[\r\n]/u.test(line))).toBe(true);
});

test("a bootstrap failure records its primary before any backend can run", async()=>{
 const bootstrap={...row,setup:{kind:"bootstrap" as const}};
 const lines:string[]=[];
 const reports=await setupToolsV1([bootstrap],machine(failing),async()=>{throw Error("unexpected provision");},signal,line=>lines.push(line));
 expect(lines.filter(line=>refusals.readToolSetupRefusalLineV1(line)!==undefined)).toEqual(refusals.toolSetupRefusalLinesV1(reports));
 expect(lines.at(-1)).toContain("core file missing");
});
