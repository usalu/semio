/** 🪪️ Child fixture of the declared-birth law: declares the envelope `SEMIO_DECLARED_CASE` names and reports what its one callback receives. */
import{test}from"bun:test";
import{existsSync}from"node:fs";
import{declareBuildTestEnvelopeV1,registerBuildTestGrantIssuerV1,withBuildTestInvocationV1}from"../../🟦️.ts";
import{spawnSync}from"node:child_process";
import{scriptInvocationEnvironment}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const kind=process.env.SEMIO_DECLARED_CASE,writer={owner:"test:declared-writer",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true as const,command:"writer"}};
const fake={tool:"fake",arguments:["repo","run","fixture:declared"]},grant={kind:"native-build-caller" as const,arguments:["run","fixture:declared"],scratch:"sandbox" as const};
if(kind?.startsWith("granted")||kind==="unregistered"||kind==="malformed"){
 if(kind!=="unregistered")registerBuildTestGrantIssuerV1("native-build-caller",{prepare:()=>({environment:{SEMIO_DECLARED_FAKE:"prepared"},release(){console.log("[TRACE] Declared release "+JSON.stringify({fake:process.env.SEMIO_DECLARED_FAKE??null}));}}),capabilities:async()=>fake,matches:async(_grant,inherited)=>JSON.stringify(inherited)===JSON.stringify(fake)});
 declareBuildTestEnvelopeV1({owner:"test:declared-granted",maximumElapsedMilliseconds:60000,capabilities:{grant:kind==="malformed"?{...grant,kind:"unknown" as never}:grant}});
}
else if(kind==="artifacts")declareBuildTestEnvelopeV1({owner:"test:declared-artifacts",maximumElapsedMilliseconds:60000,capabilities:{artifacts:true}});
else if(kind!=="undeclared")declareBuildTestEnvelopeV1(writer);
if(kind==="repeated")declareBuildTestEnvelopeV1({owner:writer.owner,maximumElapsedMilliseconds:writer.maximumElapsedMilliseconds,capabilities:{command:"writer",artifacts:true}});
if(kind==="different")declareBuildTestEnvelopeV1({...writer,capabilities:{artifacts:true}});
test("declared birth "+kind,()=>withBuildTestInvocationV1(async original=>{const facts=original.capabilities as{artifacts?:string;command?:string;tool?:string;arguments?:string[]};if(kind?.startsWith("granted"))console.log("[TRACE] Declared grant "+JSON.stringify({tool:facts.tool??null,arguments:facts.arguments??null,fake:process.env.SEMIO_DECLARED_FAKE??null,artifacts:facts.artifacts??null}));if(kind==="granted-nested"){const nested=spawnSync(process.execPath,["test",import.meta.path],{cwd:process.cwd(),env:scriptInvocationEnvironment(original,{...process.env,SEMIO_DECLARED_CASE:"granted"}),encoding:"utf8",timeout:60000});process.stdout.write(nested.stdout);process.stderr.write(nested.stderr);if(nested.status!==0)throw Error("Nested granted birth failed: "+nested.stderr.slice(-2000));}console.log("[TRACE] Declared birth "+JSON.stringify({artifacts:facts.artifacts??null,command:facts.command??null,published:process.env.SEMIO_TEST_ARTIFACT_DIR??null,exists:facts.artifacts!==undefined&&existsSync(facts.artifacts)}));}));
