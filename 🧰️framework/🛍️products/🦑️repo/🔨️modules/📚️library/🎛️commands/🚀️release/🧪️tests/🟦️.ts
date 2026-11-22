import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {ComponentsScript} from "../🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../🏗️build/📥️caller/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import law from "./🔣️.json" with {type:"json"};

test("direct component dispatch enters the original runtime provider before its driver",async()=>{
 const root=fileURLToPath(new URL("../../../../../../../../",import.meta.url));
 const capabilities=createNativeBuildCallerCapabilitiesV1(root,["components",...law.request],"components",{...process.env,SEMIO_BUILD_MODE:"prototype"});
 const envelope=createScriptProcessEnvelope({version:1,owner:"components-runtime-law",maximumElapsedMilliseconds:30000},capabilities,Date.now());
 const failure=await withScriptProcessEnvelope(envelope,original=>new ComponentsScript(root,root,original).run(law.request)).catch((error:unknown)=>error);
 expect(failure).toBeInstanceOf(Error);
 expect((failure as Error).message).toContain(law.refusal);
 expect((failure as Error).stack?.replaceAll("\\","/")).toContain(law.runtimeProvider);
});
