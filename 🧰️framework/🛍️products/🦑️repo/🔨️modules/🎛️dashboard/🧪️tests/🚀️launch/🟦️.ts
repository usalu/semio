/** 🚀️ Neutral dashboard launch ownership, with independent Ajv and RFC6902 oracles. */
import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import { resolve } from "node:path";
import { readFileSync } from "node:fs";
const owner=()=>import("../../🌳️command-tree/🚀️launch/🟦️.ts");
const dashboard=resolve(import.meta.dir,"../..");
const read=(p:string)=>JSON.parse(readFileSync(resolve(dashboard,p),"utf8"));
const fixture=read("🧪️tests/🧫️fixtures/🚀️launch/🔣️.json");
const registry=read("🧬️schema/🎮️registry/🔣️.json");
const ajv=new Ajv2020({strict:false});ajv.addSchema(registry);
const admits=ajv.compile({$ref:registry.$id+"#/$defs/TicketCommands"});
const output=ajv.compile(read("🧬️schema/🚀️launch/🔣️.json"));
test("dashboard launch preserves unrelated entries and projects exact owned commands in EN and DE",async()=>{
 expect(admits({tools:fixture.tools})).toBe(true);
 const {projectDashboardLaunch,ticketLaunchCommands}=await owner();
 const commands=ticketLaunchCommands(fixture.ticket,{tools:fixture.tools});
 const current={version:"0.2.0",configurations:[fixture.unrelated],inputs:[]};
 const actual=projectDashboardLaunch(current,commands,fixture.ticket);
 expect(output(actual)).toBe(true);
 for(const row of fixture.runtimeInvalid){const changed=structuredClone(actual);changed.configurations[1]!.env={...changed.configurations[1]!.env,[row.key]:row.value};expect(output(changed)).toBe(false);}
 for(const key of fixture.runtimeMissing){const changed=structuredClone(actual);delete changed.configurations[1]!.env[key];expect(output(changed)).toBe(false);}
 expect(actual.configurations[0]).toEqual(fixture.unrelated);
 expect(actual.configurations.slice(1).map(x=>x.name)).toEqual([
  "Test · ticket:26/09/30/HISTORY/test-choice [EN]","Test · ticket:26/09/30/HISTORY/test-text [EN]",
  "Prüfen · ticket:26/09/30/HISTORY/test-choice [DE]","Prüfen · ticket:26/09/30/HISTORY/test-text [DE]"]);
 for(const row of actual.configurations.slice(1))expect(row.command).toBe("bun repo run @semio-tech/repo-dashboard-rs:launch -- run "+row.name.split(" · ")[1]!.split(" [")[0]);
 const oracle=applyPatch(structuredClone(current),[...fixture.expectedConfigurations.map((value:any)=>({op:"add" as const,path:"/configurations/-",value})),{op:"replace",path:"/inputs",value:fixture.expectedInputs}],true).newDocument;
 expect(actual).toEqual(oracle);
 expect(projectDashboardLaunch(actual,commands,fixture.ticket)).toEqual(actual);
 const peerAfter={...actual,configurations:[...actual.configurations,{...fixture.unrelated,name:"Peer Appended Later"}]};
 expect(projectDashboardLaunch(peerAfter,commands,fixture.ticket)).toEqual(peerAfter);
 console.log(`[DEBUG] dashboard launch commands=${commands.length} locales=en,de peerPreserved=true idempotent=true`);
});
test("parameter values remain raw arguments and never enter shell command text",async()=>{
 const {projectDashboardLaunch,ticketLaunchCommands,launchParameterEnvironment,launchParameterArguments}=await owner();
 const tool=fixture.tools[0],name=launchParameterEnvironment("value");
 for(const value of fixture.values){expect(launchParameterArguments(tool,{[name]:value})).toEqual(["--param","value="+value]);}
 const commands=ticketLaunchCommands(fixture.ticket,{tools:[tool]});
 const result=projectDashboardLaunch({version:"0.2.0",configurations:[],inputs:[]},commands,fixture.ticket);
 expect(result.configurations[0].env[name]).toContain("${input:");
 expect(result.inputs![0].type).toBe("promptString");
 for(const id of fixture.unsafeIds)expect(()=>projectDashboardLaunch({version:"0.2.0",configurations:[],inputs:[]},[{...commands[0],id}],fixture.ticket)).toThrow();
 console.log(`[DEBUG] dashboard launch raw parameter vectors=${fixture.values.length} unsafeIds=${fixture.unsafeIds.length} shellInterpolation=false`);
});

test("current ticket declarations and generated launch routes are admitted by their actual owner schemas",async()=>{
 const {ticketLaunchCommands,projectDashboardLaunch}=await owner();
 const root=resolve(dashboard,"../../../../..");
 const ticket="26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING";
 const declaration=JSON.parse(readFileSync(resolve(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🎮️commands.json"),"utf8"));
 expect(admits(declaration),JSON.stringify(admits.errors)).toBe(true);
 const ts=await import("typescript"),published=ts.parseConfigFileTextToJson("launch.json",readFileSync(resolve(root,".vscode/launch.json"),"utf8")).config,seed=ts.parseConfigFileTextToJson("seed.jsonc",readFileSync(resolve(root,".vscode/🧩️launch.seed.jsonc"),"utf8")).config;
 expect(output(published),JSON.stringify(output.errors)).toBe(true);
 const runtime=ajv.compile(read("🧬️schema/🚀️launch/🔣️.json").$defs.RuntimeEnvironment);for(const document of[published,seed]){const rows=document.configurations.filter((value:any)=>value?.command==="bun repo run repo:test-toolkit-outputs --exclude-dependencies --skip-cache");expect(rows).toHaveLength(2);for(const row of rows)expect(runtime({BUN_RUNTIME_TRANSPILER_CACHE_PATH:row.env.BUN_RUNTIME_TRANSPILER_CACHE_PATH,NODE_DISABLE_COMPILE_CACHE:row.env.NODE_DISABLE_COMPILE_CACHE})).toBe(true);}
 expect(output(published),JSON.stringify(output.errors)).toBe(true);
 const commands=ticketLaunchCommands(ticket,declaration);
 expect(projectDashboardLaunch(published,commands,ticket)).toEqual(published);
 const rows=published.configurations.filter((row:any)=>row.presentation?.group?.startsWith("semio.dashboard."+ticket+"."));
 expect(rows.length).toBe(commands.length*2);
 for(const command of commands)expect(rows.filter((row:any)=>row.command==="bun repo run @semio-tech/repo-dashboard-rs:launch -- run "+command.id).length).toBe(2);
 const project=JSON.parse(readFileSync(resolve(dashboard,"📦️packages/🦀️rust/📋️project.json"),"utf8"));
 expect(project.targets.launch.options.command).toBe("bun ./📜️script.ts launch");
 console.log(`[DEBUG] dashboard current declaration schema=true tools=${commands.length} launchConfigurations=${rows.length} BuildRouting=true`);
});

/** 🧾️ Unrelated serialized data is preserved without interpreting backend-looking words. */
test("launch publication preserves unrelated serialized values without backend compatibility rewrites",async()=>{
 const api=await owner(),actual=api.projectDashboardLaunch(structuredClone(fixture.unrelatedSerialized),[],fixture.ticket),expected=applyPatch(structuredClone(fixture.unrelatedSerialized),[],true).newDocument;
 expect(actual).toEqual(expected);expect(output(actual),JSON.stringify(output.errors)).toBe(true);expect(actual.configurations[0].env.SEMIO_PREVIEW_JSON).toBe(fixture.unrelatedSerialized.configurations[0].env.SEMIO_PREVIEW_JSON);expect(actual.inputs![0].options).toEqual(["nx","bazel"]);
 const ts=await import("typescript"),source=readFileSync(resolve(dashboard,"🌳️command-tree/🚀️launch/🟦️.ts"),"utf8"),file=ts.createSourceFile("launch.ts",source,ts.ScriptTarget.Latest,true),exports=file.statements.filter(ts.isFunctionDeclaration).filter(node=>node.modifiers?.some(token=>token.kind===ts.SyntaxKind.ExportKeyword)).map(node=>node.name?.text);expect(exports).not.toContain("normalizeDashboardLaunchBuildV1");console.log("[DEBUG] Actual dashboard projection agrees with independent RFC6902 identity and leaves literal serialized backend labels/choices unchanged; AST has no compatibility normalizer");
});
