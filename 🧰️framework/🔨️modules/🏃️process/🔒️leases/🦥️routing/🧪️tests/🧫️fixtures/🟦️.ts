import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,it} from "vitest";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withResourceLeases} from "../../../../📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

it("takes a shared lease with the issued control of its own envelope",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"routing-bridge-")),directory=join(root,"leases");
 try{
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"law:routing-bridge",maximumElapsedMilliseconds:5000},{},Date.now()),async invocation=>{
   const control=createIssuedResourceControlV1(invocation,"law:routing-bridge",0);
   let ran=false;
   await withResourceLeases({directory,signal:control.signal,control,resources:[{resource:"artifact:"+join(root,"x"),mode:"shared"}]},async()=>{ran=true;});
   expect(ran).toBe(true);
  });
 }finally{rmSync(root,{recursive:true,force:true});}
});
