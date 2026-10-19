import assert from "node:assert/strict";
import {createRequire} from "node:module";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import policy from "../../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../../../📦️publication/🧬️schema/🔣️.json" with {type:"json"};

/** 🧊️ Loads and runs the actual synchronous dependency bootstrap before any application owner enters the closure. */
export function testFlowBootstrapSources():void {
 const path=fileURLToPath(import.meta.url).replaceAll("\\","/"),workspace=path.slice(0,path.indexOf("/🧰️framework/")),require=createRequire(import.meta.url),owner=fixture.owners.find(row=>row.id===fixture.bootstrap.owner)!;
 const registry=JSON.parse(readFileSync(join(workspace,fixture.bootstrap.registry),"utf8")),entry=registry.sources.find((row:{id:string})=>row.id===fixture.bootstrap.id);
 assert.equal(entry.module,owner.path);
 const compiler=require("typescript"),source=compiler.createSourceFile(owner.path,readFileSync(join(workspace,owner.path),"utf8"),compiler.ScriptTarget.Latest,true);
 for(const statement of source.statements)if(compiler.isImportDeclaration(statement)){const name=statement.moduleSpecifier.text;assert.ok(name.startsWith("node:")||name.startsWith(".")&&name.endsWith(".json"),"Bootstrap imports only system modules and authored JSON: "+name);}
 const bootstrap=require(join(workspace,entry.module));
 assert.equal(typeof bootstrap[entry.export],"function");
 const manifestValue=bootstrap.flowBrowserPackageManifestV1(fixture.bootstrap.version);for(const value of [manifestValue,manifestValue.files,manifestValue.exports,manifestValue.exports["."]])assert.ok(Object.isFrozen(value));
 for(const row of fixture.owners.filter(row=>["flow-bootstrap","browser-publication"].includes(row.id))){const source=compiler.createSourceFile(row.path,readFileSync(join(workspace,row.path),"utf8"),compiler.ScriptTarget.Latest,true),names=source.statements.filter((node:import("typescript").Statement)=>compiler.isFunctionDeclaration(node)&&(node as import("typescript").FunctionDeclaration).modifiers?.some((modifier:import("typescript").ModifierLike)=>modifier.kind===compiler.SyntaxKind.ExportKeyword)).map((node:import("typescript").FunctionDeclaration)=>node.name!.text);assert.deepEqual(names.sort(),[...row.exports].sort(),row.id+" exact function exports");}
 assert.equal(schema.definitions.ManifestIntent.properties.snippets.maxItems+fixture.package.files.length+1,policy.definitions.Protocol.const.maximumFiles);
 const validate=new Ajv({strict:true}).compile(schema.definitions.ManifestIntent);
 for(const row of fixture.bootstrap.cases){const request={version:row.version,snippets:row.snippets};assert.equal(validate(request),row.valid,JSON.stringify(request));if(row.valid)assert.deepEqual(bootstrap.flowBrowserPackageManifestV1(row.version,row.snippets).files,[...fixture.package.files,...row.snippets]);else assert.throws(()=>bootstrap.flowBrowserPackageManifestV1(row.version,row.snippets));}
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??resolve(process.env.SEMIO_TICKET_DIR??workspace,"🗑️generated","flow-bootstrap");mkdirSync(parent,{recursive:true});const root=mkdtempSync(join(parent,"cold-")),manifest=join(root,fixture.package.manifestPath);
 try{bootstrap[entry.export](root);const value=JSON.parse(readFileSync(manifest,"utf8"));assert.deepEqual(value,bootstrap.flowBrowserPackageManifestV1(fixture.bootstrap.version));assert.equal(value.name,fixture.package.name);assert.deepEqual(value.exports,fixture.package.exports);writeFileSync(manifest,"{\"retained\":true}\n");bootstrap[entry.export](root);assert.equal(readFileSync(manifest,"utf8"),"{\"retained\":true}\n");}finally{rmSync(root,{recursive:true,force:true});}
 console.log("Flow source bootstrap: actual synchronous registry load, cold manifest materialization and Ajv fixture parity PASS");
}
