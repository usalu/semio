import {test,expect} from "bun:test";
import * as fs from "node:fs";
import * as paths from "node:path";
import {createHash} from "node:crypto";
import ts from "typescript";
import {sha256} from "@noble/hashes/sha2.js";
import corpus from "./🔣️.json";

test("nested semantic Go members preserve their original compiler package",()=>{
 const source=fs.readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("go-plan.ts",source,ts.ScriptTarget.Latest,true),names=["goPackageClause","packageModuleOwner","canonicalGoModulePlan"],declarations=names.map(name=>{const node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!node)throw Error("Actual Go planner required");return node.getText(tree);}),code=ts.transpileModule(declarations.join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,bindings={existsSync:fs.existsSync,lstatSync:fs.lstatSync,readdirSync:fs.readdirSync,readFileSync:fs.readFileSync,realpathSync:fs.realpathSync,basename:paths.basename,dirname:paths.dirname,join:paths.join,relative:paths.relative,sep:paths.sep,createHash,GO_HOST_ADAPTER_PACKAGE:"adapter"},plan=new Function(...Object.keys(bindings),code+";return canonicalGoModulePlan")(...Object.values(bindings)),artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Issued test artifacts required");
 for(const row of corpus.cases){const directory=fs.mkdtempSync(paths.join(artifact,"go-members-"));try{
  fs.writeFileSync(paths.join(directory,"go.mod"),"module sample\n");
  for(const [name,content]of Object.entries(row.files)){const file=paths.join(directory,name);fs.mkdirSync(paths.dirname(file),{recursive:true});fs.writeFileSync(file,content);}
  const layout={testsDirectory:"tests",implementationFilename:"🐹️.go",packagesDirectory:"packages",languageDirectory:"go",opaqueDirectoryNames:["fixtures","oracles","packages"],sourceDirectoryNames:["member","native"]},actual=plan(directory,layout),expected=Object.fromEntries(Object.entries(row.owners).map(([name,owner])=>{const digest=Buffer.from(sha256(new TextEncoder().encode(name))).toString("hex").slice(0,16),test=name.includes("/tests/");return[paths.join(directory,owner,"zz_semio_"+digest+(test?"_test":"")+".go"),paths.join(directory,name)];}));
  expect(actual.replacements,row.id).toEqual(expected);expect(actual.packages,row.id).toEqual("packages"in row?row.packages:["."]);
 }finally{fs.rmSync(directory,{recursive:true,force:true});}}
});
