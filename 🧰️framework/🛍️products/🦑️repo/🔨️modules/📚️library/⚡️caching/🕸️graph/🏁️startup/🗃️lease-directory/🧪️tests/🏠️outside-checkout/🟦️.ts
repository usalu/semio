import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";

/** 🏠️ The Nx tooling of a machine with a declared state directory lives outside the checkout (`<state>/b/t/m/nx/<digest>`): the module that names the graph lease directory and the tooling store root must not look for a workspace above its own file there but take the declared workspace root. */
const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),here=dirname(fileURLToPath(import.meta.url));
const absolute=(text:string):string=>text.replace(/from\s*"((?:\.\.\/)+)([^"]+)"/gu,(_all,ups:string,rest:string)=>{
 let directory=join(here,"..","..");
 for(let up=0;up<ups.length/3;up++)directory=dirname(directory);
 return `from ${JSON.stringify(pathToFileURL(join(directory,rest)).href)}`;
});
const fakeCheckout=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"lease-root-")));writeFileSync(join(root,"📋️project.json"),"{}");writeFileSync(join(root,"package.json"),JSON.stringify({workspaces:["a"]}));mkdirSync(join(root,"a"));return root;};
const load=async(place:string,declared:string)=>{
 mkdirSync(place,{recursive:true});
 const file=join(place,"🟦️.ts");
 writeFileSync(file,absolute(source));
 const previous=process.env.SEMIO_WORKSPACE_ROOT;process.env.SEMIO_WORKSPACE_ROOT=declared;delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;
 try{return await import(pathToFileURL(file).href+"?"+Math.random());}finally{if(previous===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=previous;}
};
test("a module loaded from outside every checkout takes the declared workspace root, a module in a checkout takes its own",async()=>{
 const declared=fakeCheckout(),other=fakeCheckout(),state=realpathSync(mkdtempSync(join(tmpdir(),"lease-state-")));
 try{
  const outside=await load(join(state,"b","t","m","nx","d1"),declared),previous=process.env.SEMIO_WORKSPACE_ROOT;
  process.env.SEMIO_WORKSPACE_ROOT=declared;
  try{
   expect(outside.graphCheckoutRootV1()).toBe(declared);
   expect(outside.graphResourceLeaseDirectoryV1()).toBe(join(declared,".cache","b","l","m","data"));
  }finally{if(previous===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=previous;}
  const inside=await load(join(other,"a","b","c"),declared);
  process.env.SEMIO_WORKSPACE_ROOT=declared;
  try{expect(inside.graphCheckoutRootV1()).toBe(other);expect(inside.graphResourceLeaseDirectoryV1()).toBe(join(other,".cache","b","l","m","data"));}
  finally{if(previous===undefined)delete process.env.SEMIO_WORKSPACE_ROOT;else process.env.SEMIO_WORKSPACE_ROOT=previous;}
 }finally{for(const path of[declared,other,state])rmSync(path,{recursive:true,force:true});}
});
