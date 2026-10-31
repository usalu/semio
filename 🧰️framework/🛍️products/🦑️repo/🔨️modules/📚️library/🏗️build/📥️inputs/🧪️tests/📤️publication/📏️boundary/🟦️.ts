/** 📏️ Proves exact content-addressed publication at the portable path boundary and cleanup after failed effects. */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{createHash,randomUUID}from"node:crypto";
import{existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{compactSourceInputs}from"../../../🟨️.mjs";
import{withBuildGraphStagingV1}from"../../../💾️storage/🟨️.mjs";
import{concurrentEditV1,retryOnConcurrentEditSyncV1,renameOverBusyFileSyncV1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import fixture from"./🔣️.json";

const artifacts=()=>{const directory=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!directory)throw Error("Original test artifacts required");mkdirSync(directory,{recursive:true});return directory;};
test("real input inventories publish with authentic final hashes at the exact portable boundary",()=>{
 const base=mkdtempSync(join(artifacts(),"p-")),db=new Database(":memory:");
 try{for(const row of fixture.paths){
  const rootLength=row.units-1-"build-input-closures".length-1-69,padding=rootLength-base.length-1;if(padding<1)throw Error("Artifact root leaves no portable boundary fixture room");
  const root=join(base,"x".repeat(padding));mkdirSync(root,{recursive:true});const published:string[]=[];let accepted=false,values:unknown[]=[];
  try{values=withBuildGraphStagingV1({root,published:(path:string)=>published.push(path)},()=>compactSourceInputs(["{workspaceRoot}/owned.txt"],process.cwd()));accepted=true;}catch(error){if(!(error instanceof Error)||!error.message.includes("Input closure staging path exceeds"))throw error;}
  expect(accepted,row.units.toString()).toBe(row.accepted);const oracle=db.query("select ?<=256 accepted").get(row.units)as{accepted:number};expect(accepted).toBe(Boolean(oracle.accepted));
  const directory=join(root,"build-input-closures"),names=existsSync(directory)?readdirSync(directory):[];
  if(accepted){expect(published.length).toBeGreaterThan(0);expect(names.length).toBe(published.length);for(const path of published){expect(path.length).toBe(row.units);expect([...path].length).toBeLessThanOrEqual(256);const bytes=readFileSync(path),digest=createHash("sha256").update(bytes).digest("hex");expect(digest).toBe(new Bun.CryptoHasher("sha256").update(bytes).digest("hex"));expect(path).toBe(join(directory,digest+".json"));expect(JSON.parse(bytes.toString()).files).toContain("owned.txt");expect(values).toContainEqual({env:"SEMIO_SOURCE_CLOSURE_"+digest});}}
  else expect(names).toEqual([]);
 }}finally{db.close();rmSync(base,{recursive:true,force:true});}
});

test("partial writes, rename refusals and cancellation remove only the original temporary sibling",()=>{
 const root=mkdtempSync(join(artifacts(),"f-")),db=new Database(":memory:"),url=new URL("../../../🟨️.mjs",import.meta.url),tree=ts.createSourceFile(url.pathname,readFileSync(url,"utf8"),ts.ScriptTarget.Latest,true),entry=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="publishInputClosureV1");if(!entry)throw Error("Actual publisher required");
 try{for(const fault of fixture.failures){
  const contract=JSON.stringify({version:1,files:["owned.txt"]}),hash=createHash("sha256").update(contract).digest("hex"),path=join(root,hash+".json"),controller=new AbortController(),primary=Error(fault),temporaries:string[]=[];
  const ports={join,dirname,lstatSync,readFileSync,writeFileSync:(temporary:string,bytes:string,options:unknown)=>{temporaries.push(temporary);expect(dirname(temporary)).toBe(dirname(path));expect(temporary.length).toBeLessThanOrEqual(256);writeFileSync(temporary,bytes,options as never);if(fault==="write")throw primary;if(fault==="cancel"){controller.abort(primary);controller.signal.throwIfAborted();}},persistedExists:existsSync,rmSync,randomUUID,renameOverBusyFileSyncV1:(temporary:string,target:string)=>{if(fault==="rename")throw primary;renameOverBusyFileSyncV1(temporary,target);},REVISION_POLICY_V1:{maximumPathUnits:256},VERIFIED_CLOSURES_V1:new Map(),noteBuildGraphStagedLeafV1:()=>{},concurrentEditV1,retryOnConcurrentEditSyncV1};
  const publish=new Function(...Object.keys(ports),entry.getText(tree)+";return publishInputClosureV1;")(...Object.values(ports));expect(()=>publish(path,contract)).toThrow(primary);expect(temporaries.length).toBe(1);expect(temporaries.every(temporary=>!existsSync(temporary))).toBe(true);expect(existsSync(path)).toBe(false);const oracle=db.query("select 0 remaining").get()as{remaining:number};expect(readdirSync(root).length).toBe(oracle.remaining);
 }}finally{db.close();rmSync(root,{recursive:true,force:true});}
});
