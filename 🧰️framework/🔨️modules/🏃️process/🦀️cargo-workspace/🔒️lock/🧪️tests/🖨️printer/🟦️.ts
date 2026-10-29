import{expect,test}from"bun:test";
import{fileURLToPath}from"node:url";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1,scriptInvocationEnvironment}from"../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../🎛️owned-execution/📥️capture/🟦️.ts";
import corpus from"../🧫️fixtures/🧾️diagnostic.json";

declareBuildTestEnvelopeV1({owner:"test:build-workspace-graph-laws",maximumElapsedMilliseconds:60000,capabilities:{}});
test("actual owned uncaught printer retains bounded Cargo resolver diagnostic",async()=>withBuildTestInvocationV1(async original=>{
 const row=corpus.cases.find(row=>row.name==="nested-manifest-cause")!,diagnostic={version:1,status:row.status,stderr:row.stderr,stdout:row.stdout,truncated:false},module=fileURLToPath(new URL("../../🟦️.ts",import.meta.url));
 const child="import{CargoLockRefusedV1}from"+JSON.stringify(module)+";throw new CargoLockRefusedV1(\"en\",null,{en:\"Short resolver refusal\",de:\"Kurze Resolver-Ablehnung\"},"+JSON.stringify(diagnostic)+");";
 const control=createIssuedResourceControlV1(original,"test:lock-uncaught-printer",10000),result=await captureOwnedCommandResultV1(process.execPath,["--eval",child],process.cwd(),{control,env:scriptInvocationEnvironment(original,{...process.env,NO_COLOR:"1",FORCE_COLOR:"0",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}),output:{maximumBytes:32768,maximumLines:256}});
 expect(result.status).toBe(1);expect(result.signal).toBeNull();expect(result.stdout).toBe("");for(const marker of["Short resolver refusal","diagnostic:","Caused by:","package workspace root is missing","original resolver output","/original/漢/Cargo.toml"])expect(result.stderr).toContain(marker);
}),15000);
