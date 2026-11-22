/** 🔁️ Package metadata is read afresh on every resolution: a long-lived process sees an edited `exports` map and a target file written later, and a dangling entry names its missing target. */
import { expect,test } from "bun:test";
import { mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname,join } from "node:path";
import ts from "typescript";
import * as inference from "../../🟨️.mjs";
import { workspacePackageModuleV1 } from "../🟨️.mjs";

const write=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const manifest=(exports:Record<string,string>)=>JSON.stringify({name:"@fixture/pkg",type:"module",exports});
const attempt=(root:string,name:string)=>{try{return workspacePackageModuleV1(join(root,"entry.ts"),"@fixture/pkg/"+name,root,ts);}catch(error){return error as Error&{target?:string;unresolved?:boolean};}};

test("resolve, edit exports, resolve again in the same process sees the new export; a target written later resolves",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"package-metadata-"));
 try{
  write(root,"package.json",JSON.stringify({name:"fixture-root",private:true,type:"module"}));
  write(root,"entry.ts","export {};\n");write(root,"pkg/one.ts","export const one=1;\n");write(root,"pkg/two.ts","export const two=2;\n");
  write(root,"pkg/package.json",manifest({"./one":"./one.ts"}));
  mkdirSync(join(root,"node_modules/@fixture"),{recursive:true});symlinkSync(join(root,"pkg"),join(root,"node_modules/@fixture/pkg"));
  await inference.libraryBootstrap;
  expect(attempt(root,"one")).toBe(join(root,"pkg/one.ts"));
  const before=attempt(root,"two") as Error&{unresolved?:boolean};
  expect(before.unresolved).toBe(true);expect(before.message).toContain("export-not-declared");
  write(root,"pkg/package.json",manifest({"./one":"./one.ts","./two":"./two.ts"}));
  expect(attempt(root,"two")).toBe(join(root,"pkg/two.ts"));
  write(root,"pkg/package.json",manifest({"./one":"./one.ts","./two":"./two.ts","./three":"./wrapper/three.ts"}));
  const dangling=attempt(root,"three") as Error&{target?:string;unresolved?:boolean};
  expect(dangling.unresolved).toBe(true);expect(dangling.target).toBe(join(root,"pkg/wrapper/three.ts"));expect(dangling.message).toContain("export-target-missing: "+join(root,"pkg/wrapper/three.ts"));
  write(root,"pkg/wrapper/three.ts","export const three=3;\n");
  expect(attempt(root,"three")).toBe(join(root,"pkg/wrapper/three.ts"));
  write(root,"pkg/package.json",manifest({"./one":"./one.ts"}));
  expect((attempt(root,"two") as Error).message).toContain("export-not-declared");
 }finally{rmSync(root,{recursive:true,force:true});}
});
