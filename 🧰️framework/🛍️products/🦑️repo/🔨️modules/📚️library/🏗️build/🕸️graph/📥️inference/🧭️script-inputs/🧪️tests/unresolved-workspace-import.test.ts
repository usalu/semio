/** 🚧️ A workspace import the package metadata does not resolve is reported like any other unresolved import and never aborts the whole graph walk. */
import { expect,test } from "bun:test";
import { mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname,join } from "node:path";
import ts from "typescript";
import * as inference from "../../🟨️.mjs";
import { relativeScriptInputs,workspacePackageModuleV1 } from "../🟨️.mjs";

const files:Record<string,string>={
 "package.json":JSON.stringify({name:"fixture-root",private:true,type:"module"}),
 "pkg/package.json":JSON.stringify({name:"@fixture/pkg",type:"module",exports:{"./present":"./present.ts","./absent":"./absent/index.ts"}}),
 "pkg/present.ts":"export const present=1;\n",
 "entry.ts":"import {present} from \"@fixture/pkg/present\";\nimport \"@fixture/pkg/absent\";\nexport {present};\n",
 "fine.ts":"import {present} from \"@fixture/pkg/present\";\nexport {present};\n",
};
const withWorkspace=async(run:(root:string)=>unknown)=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"unresolved-import-"));
 try{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}mkdirSync(join(root,"node_modules/@fixture"),{recursive:true});symlinkSync(join(root,"pkg"),join(root,"node_modules/@fixture/pkg"));await inference.libraryBootstrap;return await run(root);}
 finally{rmSync(root,{recursive:true,force:true});}
};

test("the resolver marks a declared but absent workspace export as unresolved",()=>withWorkspace(root=>{
 const thrown=(()=>{try{workspacePackageModuleV1(join(root,"entry.ts"),"@fixture/pkg/absent",root,ts);}catch(error){return error as Error&{code?:string;unresolved?:boolean};}})();
 expect(thrown?.message).toContain("Workspace import is not resolved by current package metadata: @fixture/pkg/absent");
 expect(thrown?.code).toBe("MODULE_NOT_FOUND");
 expect(thrown?.unresolved).toBe(true);
 expect(workspacePackageModuleV1(join(root,"entry.ts"),"@fixture/pkg/present",root,ts)).toBe(join(root,"pkg/present.ts"));
}));

test("the command walk treats an absent workspace export like any unresolved import instead of aborting",()=>withWorkspace(async root=>{
 for(const required of [false,true]){
  const walked=await relativeScriptInputs([join(root,"entry.ts")],root,inference.createScriptInputCache({workspaceRoot:root}),required) as string[];
  expect(walked).toContain("{workspaceRoot}/entry.ts");
  expect(walked).toContain("{workspaceRoot}/pkg/present.ts");
  expect(walked.some(path=>path.includes("absent"))).toBe(false);
 }
}),60000);
