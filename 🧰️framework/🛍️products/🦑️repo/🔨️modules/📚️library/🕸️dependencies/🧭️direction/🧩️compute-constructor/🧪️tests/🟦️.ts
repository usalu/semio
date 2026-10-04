/** 🧭️ Repository constructor policy retains its original owned path witnesses. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import * as ts from "typescript";
import {readFileSync,mkdirSync,writeFileSync,mkdtempSync,readdirSync} from "node:fs";
import {resolve,join,dirname} from "node:path";
import {spawnSync} from "node:child_process";
import {POLICY_SKIP_DIRS,policyReadFileSafe} from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
const root=resolve(import.meta.dir,"../../../../../../../../.."),read=(path:string)=>readFileSync(join(root,path),"utf8");
test("closed compute constructor corpus agrees with independent schema admission",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(corpus)).toBe(true);expect(validateJsonSchemaSubset(schema,corpus)).toEqual([]);for(const hostile of [{...corpus,extra:true},{...corpus,constructorPolicy:{...corpus.constructorPolicy,cases:corpus.constructorPolicy.cases.map(row=>row.id==="foreign-owner"?{...row,path:corpus.constructorPolicy.owners[0]+"/🦀️.rs",allowed:true}:row)}}]){expect(validate(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}});
test("actual constructor policy names the canonical owner and retained plugin only",()=>{
 const source=ts.createSourceFile("📜️script.ts",read("📜️script.ts"),ts.ScriptTarget.Latest,true);
 const declaration=source.statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(item=>item.name.getText(source)==="POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS"));
 expect(declaration).toBeDefined();
 const compiled=ts.transpileModule(declaration!.getText(source),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 expect(new Function(compiled+";return POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS;")()).toEqual(corpus.constructorPolicy.owners);
 expect(new Set(corpus.constructorPolicy.cases.map(row=>row.id)).size).toBe(11);
});
test("actual constructor-owner predicate matches independent Node path containment",()=>{
 const source=ts.createSourceFile("📜️script.ts",read("📜️script.ts"),ts.ScriptTarget.Latest,true);
 const declaration=source.statements.find(node=>ts.isVariableStatement(node)&&node.declarationList.declarations.some(item=>item.name.getText(source)==="POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS"));
 const predicate=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="policyEngineCacheConstructorOwner");
 expect(predicate).toBeDefined();
 const compiled=ts.transpileModule(declaration!.getText(source)+"\n"+predicate!.getText(source).replace(/^export\s+/,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const actual=new Function(compiled+";return policyEngineCacheConstructorOwner;")() as (path:string)=>boolean;
 const child=spawnSync("node",["--eval",`const fs=require("node:fs"),path=require("node:path").posix;
 const data=JSON.parse(fs.readFileSync(0,"utf8")),actual=new Function(data.compiled+";return policyEngineCacheConstructorOwner;")();
 const rows=data.policy.cases.map(row=>{const normalized=row.path.split(String.fromCharCode(92)).join("/"),segments=normalized.split("/");const reference=!path.isAbsolute(normalized)&&!/^\\w:/.test(normalized)&&segments.every(part=>part!==""&&part!=="."&&part!=="..")&&data.policy.owners.some(owner=>{const relative=path.relative(owner,normalized);return relative===""||(!relative.startsWith("../")&&relative!==".."&&!path.isAbsolute(relative));});return {id:row.id,actual:actual(row.path),reference};});process.stdout.write(JSON.stringify(rows));`],{encoding:"utf8",input:JSON.stringify({compiled,policy:corpus.constructorPolicy})});
 expect(child.status,child.stderr).toBe(0);
 const rows=JSON.parse(child.stdout) as {id:string;actual:boolean;reference:boolean}[];
 expect(rows).toHaveLength(corpus.constructorPolicy.cases.length);
 for(const row of corpus.constructorPolicy.cases){expect(actual(row.path),row.id).toBe(row.allowed);expect(rows.find(item=>item.id===row.id),row.id).toEqual({id:row.id,actual:row.allowed,reference:row.allowed});}
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("caller-owned ticket output required");writeFileSync(join(output,"compute-constructor-policy-node.json"),child.stdout);
 const scanner=source.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="policyDissolvedEngineCacheScopeBreaches");
 expect(scanner?.getText(source)).toContain("policyEngineCacheConstructorOwner(relPath)");
});
test("actual constructor scanner refuses a recreated legacy debt path",()=>{
 const source=ts.createSourceFile("📜️script.ts",read("📜️script.ts"),ts.ScriptTarget.Latest,true);
 const names=new Set(["POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS","POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST","policyEngineCacheConstructorOwner","policyDissolvedEngineCacheScopeBreaches","policyAllRustFiles"]);
 const statements=source.statements.filter(node=>ts.isVariableStatement(node)?node.declarationList.declarations.some(item=>names.has(item.name.getText(source))):ts.isFunctionDeclaration(node)&&!!node.name&&names.has(node.name.text));
 expect(statements).toHaveLength(5);
 const compiled=ts.transpileModule(statements.map(node=>node.getText(source).replace(/^export\s+/,"")).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const scan=new Function("POLICY_SKIP_DIRS","policyReadFileSafe","join","readdirSync",compiled+";return policyDissolvedEngineCacheScopeBreaches;")(POLICY_SKIP_DIRS,policyReadFileSafe,join,readdirSync) as (root:string)=>{scope:string;kind:string}[];
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("caller-owned ticket output required");
 const fixtureRoot=mkdtempSync(join(output,"compute-constructor-policy-"));
 const rows=corpus.constructorPolicy.cases.filter(row=>!row.path.includes("\\")&&!row.path.split("/").includes(".."));
 for(const row of rows){const file=join(fixtureRoot,row.path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,"fn cache() { let cache = EngineCache::new(4); }\n");}
 const breaches=scan(fixtureRoot);
 expect(breaches.map(row=>row.scope).sort()).toEqual(rows.filter(row=>!row.allowed).map(row=>row.path).sort());
 expect(breaches.every(row=>row.kind==="dissolved-kernels/engine-cache-scope")).toBe(true);
});
