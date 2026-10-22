/** 📌️ Contract: a process that receives its parent's authenticated invocation is committed to the implementation it loaded, so the live-source self-checks of the Stores and readers let a short nested script finish instead of dying on a peer's edit; a process that did not receive one stays unpinned. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{SCRIPT_PROCESS_INVOCATION_ENV,createScriptProcessEnvelope,readScriptPolicy,scriptProcessEnvironment}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const routing=fileURLToPath(new URL("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url)),revision=fileURLToPath(new URL("../../🟨️.mjs",import.meta.url));
const vectors=[
 {id:"received",receive:true,insidePinned:true,afterPinned:false},
 {id:"not-received",receive:false,insidePinned:false,afterPinned:false},
];

function runChild(receive:boolean):{insidePinned:boolean;afterPinned:boolean;status:number|null;stderr:string}{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"received-pin-")),script=join(root,"child.ts");
 try{
  writeFileSync(script,`import{receiveScriptProcessInvocation}from${JSON.stringify(routing)};import{implementationPinnedV1}from${JSON.stringify(revision)};
const inside=${receive}?await receiveScriptProcessInvocation(process.env,async()=>implementationPinnedV1()):implementationPinnedV1();
process.stdout.write(JSON.stringify({insidePinned:inside,afterPinned:implementationPinnedV1()}));`);
  const environment=scriptProcessEnvironment(createScriptProcessEnvelope(readScriptPolicy({version:1,owner:"received-pin-law",maximumElapsedMilliseconds:60000}),{},Date.now()),{});
  const result=Bun.spawnSync([process.execPath,script],{cwd:root,env:{...process.env,[SCRIPT_PROCESS_INVOCATION_ENV]:environment[SCRIPT_PROCESS_INVOCATION_ENV]},stdout:"pipe",stderr:"pipe"});
  return{...(result.exitCode===0?JSON.parse(result.stdout.toString()):{insidePinned:false,afterPinned:false}),status:result.exitCode,stderr:result.stderr.toString()};
 }finally{rmSync(root,{recursive:true,force:true});}
}

for(const vector of vectors)test(`a process that ${vector.receive?"receives":"does not receive"} its parent's invocation is ${vector.insidePinned?"pinned":"unpinned"} (${vector.id})`,()=>{
 const observed=runChild(vector.receive);
 expect(observed.stderr).toBe("");
 expect(observed.status).toBe(0);
 expect(observed.insidePinned).toBe(vector.insidePinned);
 expect(observed.afterPinned).toBe(vector.afterPinned);
});
