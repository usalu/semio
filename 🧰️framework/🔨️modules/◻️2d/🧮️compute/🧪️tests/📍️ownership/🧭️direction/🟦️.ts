/** 🧭️ Proves the actual compute witness executes without higher owners. */
import {expect,test} from "bun:test";

import * as ts from "typescript";
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,existsSync} from "node:fs";
import {resolve,join,dirname} from "node:path";
import {spawnSync} from "node:child_process";
import law from "../../../🧫️fixtures/📍️ownership/🧭️direction/🔣️.json";
const root=resolve(import.meta.dir,"../../../../../../.."),read=(path:string)=>readFileSync(join(root,path),"utf8"),owner="🧰️framework/🔨️modules/◻️2d/🧮️compute";
test("closed neutral compute ownership contains only portable fields",()=>{
 const fixture = JSON.parse(read(owner + "/🧫️fixtures/📍️ownership/🔣️.json"));
 expect(Object.keys(fixture).sort()).toEqual([...law.keys].sort());
 for(const path of [fixture.source,fixture.unit])expect(path.startsWith(law.neutralPrefix+"/")).toBe(true);
});
test("neutral compute witness and task inputs do not depend on higher owners",()=>{
 const path=owner+"/🧪️tests/📍️ownership/🟦️.ts",source=ts.createSourceFile(path,read(path),ts.ScriptTarget.Latest,true);
 const literals:string[]=[];const visit=(node:ts.Node)=>{if(ts.isStringLiteral(node))literals.push(node.text);ts.forEachChild(node,visit);};visit(source);
 for(const value of literals)for(const higher of law.absent)expect(value.includes(higher),value).toBe(false);
 const project=JSON.parse(read(owner+"/📋️project.json"));for(const input of project.targets["test-ownership"].inputs)if(typeof input==="string")for(const higher of law.absent)expect(input.includes(higher),input).toBe(false);
});
test("deleting all higher owners leaves the actual neutral compute witness executable",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Caller-owned output required");mkdirSync(output,{recursive:true});const sandbox=mkdtempSync(join(output,"compute-neutral-deletion-"));
 for(const path of law.copied){const destination=join(sandbox,path);mkdirSync(dirname(destination),{recursive:true});writeFileSync(destination,read(path));}
 for(const path of law.absent)expect(existsSync(join(sandbox,path))).toBe(false);
 const node=spawnSync("node",["--eval","const fs=require('node:fs');const data=JSON.parse(fs.readFileSync(0,'utf8'));process.stdout.write(JSON.stringify(data.absent.map(p=>fs.existsSync(data.root+'/'+p))));"],{encoding:"utf8",timeout:30000,input:JSON.stringify({root:sandbox,absent:law.absent})});expect(node.status,node.stderr).toBe(0);expect(JSON.parse(node.stdout)).toEqual(law.absent.map(()=>false));
 const child=spawnSync(process.execPath,["test",join(sandbox,law.copied[0])],{cwd:sandbox,encoding:"utf8",timeout:30000,env:{...process.env,SEMIO_TEST_ARTIFACT_DIR:join(sandbox,"output")}});
 writeFileSync(join(output,"compute-neutral-deletion-runtime.json"),JSON.stringify({status:child.status,signal:child.signal,stdout:child.stdout,stderr:child.stderr,sandbox,absent:law.absent}));expect(child.status,child.stderr).toBe(0);expect(child.stderr).toContain("3 pass");
});
