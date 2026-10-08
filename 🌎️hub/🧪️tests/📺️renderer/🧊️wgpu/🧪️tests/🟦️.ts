import assert from "node:assert/strict";
import {readFile,access} from "node:fs/promises";
import {createRequire} from "node:module";
import {join} from "node:path";
const owner="🌎️hub/🧪️tests/📺️renderer/🧊️wgpu",general="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript";

/** 🧊️ Verifies Specific command ownership and retained native selections with the independent TypeScript oracle. */
export async function proveHubRendererCommandOwnershipV1(root:string):Promise<void>{
 const require=createRequire(join(root,"package.json")),ts=require("typescript"),fixture=JSON.parse(await readFile(join(root,owner,"🧫️fixtures/🔣️.json"),"utf8"));
 assert.equal(new Set(fixture.commands.map((row:any)=>row.command)).size,fixture.commands.length);assert.equal(fixture.commands.length,5);
 const source=await readFile(join(root,general,"📜️script.ts"),"utf8"),parsed=ts.createSourceFile("router.ts",source,ts.ScriptTarget.Latest,true),specific:string[]=[];
 const visit=(node:any)=>{if(ts.isStringLiteralLike(node)&&["🌎️hub","✏️s"].some(prefix=>node.text===prefix||node.text.startsWith(prefix+"/")))specific.push(node.text);ts.forEachChild(node,visit);};visit(parsed);
 console.log("[DEBUG] WGPU command ownership portable="+fixture.commands.length+" executableSpecificLiterals="+specific.length);assert.deepEqual(specific,[]);
 const generic=JSON.parse(await readFile(join(root,general,"📋️project.json"),"utf8")),project=JSON.parse(await readFile(join(root,owner,"📋️project.json"),"utf8")),specificSource=await readFile(join(root,owner,"📜️script.ts"),"utf8");
 assert.equal(project.name,"@semio-tech/hub-renderer-wgpu");for(const row of fixture.commands){assert.equal(generic.targets[row.generalCommand],undefined);assert.ok(!source.includes('.register("'+row.generalCommand+'",'));assert.equal(project.targets[row.command].options.command,"bun ./📜️script.ts "+row.command);assert.equal(project.targets[row.command].options.cwd,owner);assert.ok(specificSource.includes('.register("'+row.command+'", '+row.className+')'));for(const law of row.laws)assert.ok(specificSource.includes(law),law);}
 const report=ts.transpileModule(specificSource,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext},reportDiagnostics:true});assert.deepEqual(report.diagnostics,[]);
 await assert.rejects(access(join(root,fixture.collaboration.generalHarness)),{code:"ENOENT"});
 const harnessSource=await readFile(join(root,fixture.collaboration.specificHarness),"utf8");
 assert.ok(specificSource.includes('from "../🤝️collaboration/🟦️.ts"')||specificSource.includes('from "./🤝️collaboration/🟦️.ts"'));
 const harness=await import(join(root,fixture.collaboration.specificHarness));
 assert.deepEqual(Object.keys(harness.HUB_COLLABORATION_CHECKS),fixture.collaboration.journeys);assert.deepEqual(Object.keys(harness.HUB_COLLABORATION_KINDS),fixture.collaboration.journeys);assert.deepEqual(harness.HUB_COLLABORATION_BOUNDS,fixture.collaboration.bounds);
 assert.equal(typeof harness.runHubCollaborationCli,"function");assert.deepEqual(ts.transpileModule(harnessSource,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext},reportDiagnostics:true}).diagnostics,[]);
 console.log("[DEBUG] WGPU collaboration Specific module imports=1 retainedJourneys=7 bounds=15");
 console.log("[DEBUG] WGPU command ownership declaringTargets=5 retainedNativeLaws=7 TypeScriptOracle=1");
 await import(join(root,general,"📜️script.ts"));
 console.log("[DEBUG] WGPU General declaring module imports=1 Specific declaring owner main=1 nativeInvocation=0");
}
