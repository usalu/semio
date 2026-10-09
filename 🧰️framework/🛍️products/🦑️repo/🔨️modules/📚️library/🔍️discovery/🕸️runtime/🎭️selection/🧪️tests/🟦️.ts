import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve, join } from "node:path";
import ts from "typescript";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
const cases=()=>fixture.cases.map(row=>{const value=structuredClone(fixture.evidence);for(const change of row.changes){let parent:Record<string|number,unknown>=value;for(const part of change.path.slice(0,-1))parent=parent[part] as Record<string|number,unknown>;parent[change.path.at(-1)!]=change.value;}return{...row,value};});
const authority=()=>{const controller=new AbortController(),started=performance.now(),progress:unknown[]=[];return{controller,progress,control:{maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>controller.signal.aborted,remainingMs:()=>60000-(performance.now()-started),onProgress:(event:unknown)=>{progress.push(event);}}};};
test("complete neutral selected actor evidence agrees with independent schema and JSON parsing",async()=>{
 const oracle=new Ajv({strict:false}).compile(schema),jsonc=await import("jsonc-parser");
 for(const row of cases()){expect(oracle(row.value),row.name+JSON.stringify(oracle.errors)).toBe(row.accepted);expect(validateJsonSchemaSubset(schema,row.value).length===0,row.name).toBe(row.accepted);if(row.accepted)expect(jsonc.parse(JSON.stringify(row.value))).toEqual(row.value);}
 console.log("[DEBUG] complete portable actor evidence vectors matched first-party schema validation and independent Ajv/JSON parser output");
});
test("required neutral publication port admits typed evidence and keeps acquisition explicit",async()=>{
 const selected=await import("../🧬️schema/🟦️.ts"),owned=authority();
 for(const row of cases()){if(row.accepted)expect(await selected.parseSelectedRuntimeActorEvidenceV1(row.value,owned.control)).toEqual(row.value);else await expect(selected.parseSelectedRuntimeActorEvidenceV1(row.value,owned.control)).rejects.toThrow();}
 const evidence=await selected.parseSelectedRuntimeActorEvidenceV1(fixture.evidence,owned.control),events:string[]=[],port={diagnosticOwner:"neutral/actor-publication",observeCurrent:async(control:typeof owned.control)=>{expect(control).toBe(owned.control);events.push("observe");return{...evidence,recheck:async(control:typeof owned.control)=>{expect(control).toBe(owned.control);events.push("recheck");}};},acquireCurrent:async(control:typeof owned.control)=>{expect(control).toBe(owned.control);events.push("acquire");}};
 const current=await selected.observeSelectedRuntimeActorsV1(port,owned.control);expect(events).toEqual(["observe"]);await current.recheck(owned.control);expect(events).toEqual(["observe","recheck"]);await selected.acquireSelectedRuntimeActorsV1(port,owned.control);expect(events).toEqual(["observe","recheck","acquire"]);expect(owned.progress.length).toBeGreaterThan(0);
 await expect(selected.observeSelectedRuntimeActorsV1(undefined,owned.control)).rejects.toThrow();
 owned.controller.abort();await expect(selected.observeSelectedRuntimeActorsV1(port,owned.control)).rejects.toThrow();expect(events).toEqual(["observe","recheck","acquire"]);
});
test("actual General verifier has a required owned port and no Specific actor discovery",()=>{
 const owner=resolve(import.meta.dir,"../.."),source=readFileSync(join(owner,"🔎️verification/🟦️.ts"),"utf8"),syntax=ts.createSourceFile("verification.ts",source,ts.ScriptTarget.Latest,true),definition=syntax.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="verifyRuntimeFixtureGraphV1");expect(definition&&ts.isFunctionDeclaration(definition)&&definition.parameters[2]?.type?.getText(syntax)).toBe("SelectedRuntimeActorPublicationPortV1");
 for(const name of["runtimeSelectedActorsV1","loadCurrentDevelopmentActorsV1"])expect(syntax.statements.some(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)).toBe(false);
 const config=readFileSync(join(owner,"🔣️.json"),"utf8");for(const key of["actorProvenanceOwner","actorCatalogOwner","actorDevelopmentOwner","actorProject"])expect(config).not.toContain(key);
 expect(source).not.toContain("config.actorProvenanceOwner");expect(source).not.toContain("config.actorDevelopmentOwner");
});

test("selected actor evidence obeys portable finite observation authority",async()=>{
 const selected=await import("../🧬️schema/🟦️.ts");
 for(const row of fixture.controls)await expect(selected.parseSelectedRuntimeActorEvidenceV1(fixture.evidence,{...row,cancelled:()=>row.cancelled,remainingMs:()=>row.remainingMs,onProgress:()=>{}})).rejects.toThrow();
 const owned=authority(),cancelled={...owned.control,onProgress:()=>owned.controller.abort()};await expect(selected.parseSelectedRuntimeActorEvidenceV1(fixture.evidence,cancelled)).rejects.toThrow();
 const unsafe={...fixture.evidence};Object.defineProperty(unsafe,"dataRoot",{get:()=>{throw Error("getter invoked");},enumerable:true});await expect(selected.parseSelectedRuntimeActorEvidenceV1(unsafe,authority().control)).rejects.toThrow(/accessor/u);
});

test("selected actor public contract has actual strict first-party types and a canonical schema owner",async()=>{
 const selection=resolve(import.meta.dir,".."),root=process.cwd(),port=join(selection,"🧬️schema/🟦️.ts"),cargo=resolve(selection,"../🧬️schema/🟦️.ts"),options={target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,strict:true,noEmit:true,skipLibCheck:true,resolveJsonModule:true,allowImportingTsExtensions:true,esModuleInterop:true,types:["bun"]},program=(()=>{console.log("[DEBUG] selected strict type program begin");const value=ts.createProgram([port,cargo],options);console.log("[DEBUG] selected strict type program acquired");return value;})(),diagnostics=ts.getPreEmitDiagnostics(program).filter(row=>row.file&&(row.file.fileName===port||row.file.fileName===cargo));expect(diagnostics.map(row=>ts.flattenDiagnosticMessageText(row.messageText,"\n"))).toEqual([]);
 console.log("[DEBUG] selected strict diagnostics completed");const {inventorySchemaScopes}=await import("../../../🟦️.ts");console.log("[DEBUG] selected inventory begin");const inventory=inventorySchemaScopes(root),scope=inventory.catalog.scopes["repo.discovery.runtime.selection"];expect(scope?.path).toBe("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🎭️selection/🧬️schema");expect(scope?.exports.SelectedRuntimeActorEvidenceV1).toBeDefined();expect(Object.keys(scope!.exports).sort()).toEqual(Object.keys(schema.$defs).sort());expect(inventory.diagnostics.filter(row=>row.path.startsWith(scope?.path??"missing")).length).toBe(0);
 console.log("[DEBUG] selected actor public contract strict TypeScript and actual canonical schema inventory joined");
},120000);

test("actual graph package coverage is explicit caller authority with portable roots",async()=>{
 const selected=await import("../🧬️schema/🟦️.ts"),oracle=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/SelectedRuntimeGraphPackageRootsV1"});for(const row of fixture.graphPackageRoots){expect(oracle(row.value),row.name).toBe(row.accepted);if(row.accepted)expect(selected.parseSelectedRuntimeGraphPackageRootsV1(row.value)).toEqual(row.value);else expect(()=>selected.parseSelectedRuntimeGraphPackageRootsV1(row.value)).toThrow();}
 const owner=resolve(import.meta.dir,"../.."),source=readFileSync(join(owner,"🔎️verification/🟦️.ts"),"utf8"),syntax=ts.createSourceFile("verification.ts",source,ts.ScriptTarget.Latest,true),definition=syntax.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="verifyRuntimeFixtureGraphV1");expect(definition&&ts.isFunctionDeclaration(definition)&&definition.parameters[3]?.type?.getText(syntax)).toBe("SelectedRuntimeGraphPackageRootsV1");expect(source).not.toContain('"🌎️hub"');expect(source).not.toContain('"✏️s"');const root=ts.createSourceFile("root.ts",readFileSync(join(process.cwd(),"📜️script.ts"),"utf8"),ts.ScriptTarget.Latest,true);expect(root.parseDiagnostics).toEqual([]);
});
