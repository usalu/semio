import {test,expect} from "bun:test";
import {mkdtempSync,readdirSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {runCargoTestsV1,runCargoLintV1,type CargoTestPolicyV1} from "../../🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original test artifacts required");

test("neutral missing and fabricated compiler admission cannot create captures or execute a producer",async()=>{
 const root=mkdtempSync(join(artifacts!,"cargo-custody-")),policy:CargoTestPolicyV1={version:1,mode:"prototype",manifestPath:join(root,"Cargo.toml"),targetDirectory:join(root,"target"),buildDirectory:join(root,"build"),leaseDirectory:join(root,"leases"),nextest:false,configPath:join(root,"config.toml"),level:"fundamental",assertionBudgets:{fundamental:1000,quick:1000,long:1000,exhaustive:1000},buildBudgetMs:1000,assertionThreads:1,artifactDirectory:join(root,"captures"),retainArtifacts:false,coverageEnabled:false,coveragePath:null,rustMinStack:"8388608"};
 expect(new Ajv({strict:false}).compile(schema)(policy)).toBe(true);const request={manifestPath:policy.manifestPath,packages:["original"],cwd:root},control={signal:new AbortController().signal,remainingMilliseconds:()=>1000,publish:async()=>{},yieldContinuation:async()=>{}};
 for(const run of [runCargoTestsV1,runCargoLintV1]){await expect(run(request,policy,undefined as any)).rejects.toThrow(fixture.missing);await expect(run(request,policy,{workspaceRoot:root,control,storage:{} as any,sourceBoundary:fixture.sourceBoundary as any})).rejects.toThrow("Live Original resource control issuance required");expect(readdirSync(root)).toEqual([]);}
 let refusal:unknown;try{await receiveScriptProcessInvocation(process.env,async original=>{const issued=scriptCompilerAdmissionControlV1(original,"test:cargo-admission",30000);await runCargoTestsV1(request,policy,{workspaceRoot:root,control:issued,storage:{} as any,sourceBoundary:fixture.sourceBoundary as any});});}catch(error){refusal=error;}expect(String(refusal)).toContain("Original issued native storage reservation");expect(readdirSync(root)).toEqual([]);
});
