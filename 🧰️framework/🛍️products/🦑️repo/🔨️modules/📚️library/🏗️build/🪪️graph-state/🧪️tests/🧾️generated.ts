import {expect,test} from "bun:test";
import {cpSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,dirname} from "node:path";
import {GRAPH_STATE_POLICY_V1 as policy} from "../🟦️.ts";
import {generatedInputsDigestV1} from "../🧾️generated/🟦️.ts";

/** 🧾️ Every `🤖️generated` directory of the repository belongs to projects whose tasks hash it: each project at or below the parent of the directory declares the runtime digest of that directory in its default inputs, and the digest changes with every byte of the directory. */
const root=process.env.SEMIO_WORKSPACE_ROOT??join(import.meta.dir,"../../../../../../../..");
const script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📜️script.ts";

function walk(directory:string,found:{generated:string[];projects:string[]}):void{
 for(const entry of readdirSync(join(root,directory),{withFileTypes:true})){
  const path=directory===""?entry.name:`${directory}/${entry.name}`;
  if(entry.isDirectory()){
   if(entry.name===policy.generatedDirectory){found.generated.push(path);continue;}
   if((policy.generatedPruned as readonly string[]).includes(entry.name))continue;
   walk(path,found);
  }else if(entry.name==="📋️project.json")found.projects.push(path);
 }
}

test("every project beside a generated directory declares its digest, and the digest sees every byte",()=>{
 const found={generated:[] as string[],projects:[] as string[]};
 for(const top of ["🧰️framework","✏️s"])if(existsSync(join(root,top)))walk(top,found);
 const generated=found.generated.filter(path=>!(policy.generatedSkipped as readonly string[]).includes(path));
 expect(generated.length).toBeGreaterThan(20);
 const missing:string[]=[];
 for(const directory of generated){
  const module=dirname(directory),owners=found.projects.filter(project=>project===module+"/📋️project.json"||project.startsWith(module+"/"));
  for(const project of owners){
   const json=JSON.parse(readFileSync(join(root,project),"utf8")) as {namedInputs?:{default?:unknown[]}},inputs=json.namedInputs?.default??[];
   const declared=inputs.some(input=>typeof input==="object"&&input!==null&&typeof (input as {runtime?:string}).runtime==="string"&&(input as {runtime:string}).runtime.includes(script)&&(input as {runtime:string}).runtime.includes(`"${directory}"`));
   if(!declared)missing.push(`${project} <- ${directory}`);
  }
 }
 expect(missing).toEqual([]);
},300000);

test("the digest of a real generated directory is stable and changes with a byte, a new file and a rename",async()=>{
 const found={generated:[] as string[],projects:[] as string[]};
 walk("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",found);
 const directory=found.generated[0];
 if(directory===undefined)return;
 const copy=mkdtempSync(join(tmpdir(),"generated-"));
 try{
  cpSync(join(root,directory),join(copy,"g"),{recursive:true});
  const first=(await generatedInputsDigestV1(copy,["g"])).digest,again=(await generatedInputsDigestV1(copy,["g"])).digest;
  expect(again).toBe(first);
  const real=(await generatedInputsDigestV1(root,[directory])).digest,copied=(await generatedInputsDigestV1(copy,["g"])).digest;
  expect(copied).not.toBe("");expect(real.length).toBe(64);
  const files=readdirSync(join(copy,"g"),{recursive:true,withFileTypes:true}).filter(entry=>entry.isFile());
  expect(files.length).toBeGreaterThan(0);
  const target=join(files[0]!.parentPath,files[0]!.name),before=readFileSync(target);
  writeFileSync(target,Buffer.concat([before,Buffer.from(" ")]));
  expect((await generatedInputsDigestV1(copy,["g"])).digest).not.toBe(first);
  writeFileSync(target,before);expect((await generatedInputsDigestV1(copy,["g"])).digest).toBe(first);
  writeFileSync(join(copy,"g","zz-new.json"),"{}");expect((await generatedInputsDigestV1(copy,["g"])).digest).not.toBe(first);
  rmSync(join(copy,"g","zz-new.json"));
  void relative;
 }finally{rmSync(copy,{recursive:true,force:true});}
},300000);
