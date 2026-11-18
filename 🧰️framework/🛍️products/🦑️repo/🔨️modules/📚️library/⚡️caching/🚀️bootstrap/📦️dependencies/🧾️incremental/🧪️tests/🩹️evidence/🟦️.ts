import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {retireJavascriptDependencyGenerationFilesV1} from "../../../🧾️generation/💾️storage/🟦️.ts";
import * as core from "../../🟦️.ts";
import * as glue from "../../💾️storage/🟦️.ts";

const protocol=core.incrementalProtocolV1,idle={check:()=>{},tick:async()=>{}},hex=(seed:string)=>seed.repeat(64).slice(0,64);
const kindOf=(facts:{isSymbolicLink():boolean;isDirectory():boolean;isFile():boolean}):string=>facts.isSymbolicLink()?"link":facts.isDirectory()?"directory":facts.isFile()?"file":"foreign";
const scratch=(prefix:string):string=>realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),prefix)));
const tree=(root:string):void=>{for(let a=0;a<4;a++)for(let b=0;b<3;b++){const dir=join(root,"a"+a,"b"+b);mkdirSync(dir,{recursive:true});writeFileSync(join(dir,"f"),"x");}for(const file of ["a0","a1","a2","a3"])utimesSync(join(root,file),new Date(Date.now()-3600000),new Date(Date.now()-3600000));};
const scanOf=(root:string)=>{
 const sink=glue.installedLedgerSinkV1(Date.now()),hash=createHash("sha256");let widest=0;
 const walk=(path:string,portable:string):void=>{
  const facts=lstatSync(path,{bigint:true}),names=readdirSync(path).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right))),kids=names.map(name=>({name,kind:kindOf(lstatSync(join(path,name)))}));
  widest=Math.max(widest,path.length);hash.update(JSON.stringify([portable,"directory"])+"\n");
  for(const kid of kids){const child=join(path,kid.name),at=portable==="."?kid.name:portable+"/"+kid.name;if(kid.kind==="directory")walk(child,at);else{widest=Math.max(widest,child.length);hash.update(JSON.stringify([at,kid.kind])+"\n");}}
  sink.directory(portable,facts,kids);
 };
 walk(root,".");
 const counts=core.installedCountsV1(sink.rows);return{rows:sink.rows,scan:{...counts,maximumPathUnits:widest,sha256:hash.digest("hex")}};
};
const seal=async(store:string,generation:string,root:string)=>{const{rows,scan}=scanOf(root);await glue.sealJavascriptDependencyInstalledReceiptV1(store,generation,scan,rows,()=>{},async()=>{});return core.encodeInstalledLedgerV1(rows).evidence.name;};
const world=()=>{const store=scratch("evidence-store-"),root=scratch("evidence-root-");tree(root);return{store,root,generation:hex("c"),clean:()=>{rmSync(store,{recursive:true,force:true});rmSync(root,{recursive:true,force:true});}};};

test("evidence law: the generation sweep keeps the installed-paths and ownership evidence it does not own and still removes its own orphans",async()=>{
 const w=world();
 try{
  const evidence=await seal(w.store,w.generation,w.root),directory=join(w.store,protocol.directory),ownership=hex("d")+protocol.ownership.evidenceSuffix,orphan=hex("e")+".frontier.ndjson",unknown=hex("f")+".mystery.ndjson";
  for(const name of [ownership,orphan,unknown])writeFileSync(join(directory,name),"x\n");
  expect(await glue.verifyJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,idle)).toBeDefined();
  retireJavascriptDependencyGenerationFilesV1(w.store,"scope");
  expect(existsSync(join(directory,evidence))).toBe(true);
  expect(existsSync(join(directory,ownership))).toBe(true);
  expect(existsSync(join(directory,orphan))).toBe(false);
  expect(await glue.verifyJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,idle)).toBeDefined();
 }finally{w.clean();}
},60000);

test("evidence law: a receipt whose evidence is gone is repaired once from the store, leaving one receipt and one evidence file, and a receipt with evidence is never repaired",async()=>{
 const w=world();
 try{
  const evidence=await seal(w.store,w.generation,w.root),directory=join(w.store,protocol.directory);
  rmSync(join(directory,evidence));
  expect(await glue.verifyJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,idle)).toBeUndefined();
  const sha=glue.newestJavascriptDependencyInstalledReceiptV1(w.store,w.generation);
  expect(sha).toBeDefined();
  const repaired=await glue.repairJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,sha!,idle,()=>{},async()=>{});
  expect(repaired).toBeDefined();
  expect(readdirSync(directory).filter(name=>name.endsWith(protocol.installed.receiptSuffix)).length).toBe(1);
  expect(readdirSync(directory).filter(name=>name.endsWith(protocol.installed.evidenceSuffix)).length).toBe(1);
  const known=await glue.verifyJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,idle);
  expect(known?.sha256).toBe(repaired);
  expect(known?.shape).toEqual(core.parseInstalledReceiptV1(readFileSync(join(directory,readdirSync(directory).find(name=>name.endsWith(protocol.installed.receiptSuffix))!))).shape);
  expect(await glue.repairJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,repaired!,idle,()=>{},async()=>{})).toBeUndefined();
 }finally{w.clean();}
},60000);

test("evidence law: the repair never writes evidence for a generation that changed, the receipt stays unproven",async()=>{
 const cases:Record<string,(root:string)=>void>={
  extraFile:root=>writeFileSync(join(root,"a1","b1","extra"),"y"),
  missingDirectory:root=>rmSync(join(root,"a2","b2"),{recursive:true}),
  extraDirectory:root=>mkdirSync(join(root,"a3","b9")),
 };
 for(const[name,change]of Object.entries(cases)){
  const w=world();
  try{
   const evidence=await seal(w.store,w.generation,w.root),directory=join(w.store,protocol.directory),sha=glue.newestJavascriptDependencyInstalledReceiptV1(w.store,w.generation)!;
   rmSync(join(directory,evidence));change(w.root);
   expect({name,repaired:await glue.repairJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,sha,idle,()=>{},async()=>{})}).toEqual({name,repaired:undefined});
   expect({name,present:existsSync(join(directory,evidence))}).toEqual({name,present:false});
   expect(await glue.verifyJavascriptDependencyInstalledReceiptV1(w.store,w.root,w.generation,idle)).toBeUndefined();
  }finally{w.clean();}
 }
},60000);
