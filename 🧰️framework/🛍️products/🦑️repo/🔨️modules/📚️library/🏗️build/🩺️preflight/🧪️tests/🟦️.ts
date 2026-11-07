import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {findWorkspaceRoot} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {buildPreflightProblemsV1,probeBuildPreflightCaseV1,readBuildPreflightArgumentsV1,readBuildPreflightPolicyV1} from "../🟦️.ts";
import {nativeDirectoryProtocolV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";

const root=findWorkspaceRoot(import.meta.dir);
test("only declared journaled Windows refusals yield fold-safe capability",()=>{
 const protocol=nativeDirectoryProtocolV1(),oracle=new Ajv({strict:true}),admitted=oracle.compile({type:"object",allOf:[schema.definitions.CaseProbe,{oneOf:[{properties:{probe:{const:"sensitive"},expected:{const:"yes"}}},{properties:{probe:{const:"insensitive"},expected:{const:"no"}}},{properties:{platform:{const:"win32"},kind:{const:"journaled"},probe:{enum:protocol.foldSafeRefusals},expected:{const:"fold-safe"}}},{properties:{expected:{const:"refused"}},not:{anyOf:[{properties:{probe:{enum:["sensitive","insensitive"]}}},{properties:{platform:{const:"win32"},kind:{const:"journaled"},probe:{enum:protocol.foldSafeRefusals}}}]}}]}]});
 for(const row of fixture.probes){
  expect(admitted(row),row.id).toBe(true);
  const message=row.probe==="near-match"?protocol.refusals.caseAccess+" ":(protocol.refusals as Readonly<Record<string,string>>)[row.probe]??row.probe,error=row.probe==="foreign"?{message:protocol.refusals.caseAccess}:row.probe==="cancelled"?new DOMException("cancelled","AbortError"):new Error(message);
  const probe=()=>{if(row.probe==="sensitive")return true;if(row.probe==="insensitive")return false;throw error;},run=()=>probeBuildPreflightCaseV1(row.platform,row.kind as "exchange"|"journaled",probe);
  if(row.expected==="refused"){let caught:unknown;try{run();}catch(failure){caught=failure;}expect(caught,row.id).toBe(error);}else expect(run(),row.id).toBe(row.expected);
 }
});
test("portable preflight corpus agrees with independent closed-schema admission",()=>{
 const oracle=new Ajv({strict:true}),requirements=oracle.compile(schema.definitions.Platform),facts=oracle.compile(schema.definitions.Facts);
 const admitted=oracle.compile({type:"object",required:["platform","architecture","availableBytes","tools","exchange","caseSensitive"],properties:{architecture:{enum:fixture.requirements.architectures},availableBytes:{type:"integer",minimum:fixture.requirements.minimumAvailableBytes},tools:{type:"array",allOf:fixture.requirements.tools.map(tool=>({contains:{const:tool}}))},exchange:{const:"supported"}},anyOf:[{properties:{caseSensitive:{const:"yes"}}},{properties:{platform:{const:"win32"},caseSensitive:{const:"fold-safe"}}}]});
 expect(requirements(fixture.requirements)).toBe(true);expect(requirements({...fixture.requirements,skip:true})).toBe(false);
 for(const row of fixture.cases){const observation={...fixture.facts,...row.patch};expect(facts(observation),row.id).toBe(true);expect(admitted(observation),row.id).toBe(row.problems.length===0);expect(buildPreflightProblemsV1(fixture.requirements,observation),row.id).toEqual(row.problems);}
 expect(()=>buildPreflightProblemsV1({...fixture.requirements,skip:true},fixture.facts)).toThrow("Closed build preflight requirements");
 for(const args of fixture.accepted)expect(()=>readBuildPreflightArgumentsV1(args)).not.toThrow();for(const args of fixture.rejected)expect(()=>readBuildPreflightArgumentsV1(args)).toThrow();
});
test("authored policy needs no privilege on any platform and drives the neutral hosts",()=>{
 const policy=readBuildPreflightPolicyV1(),text=readFileSync(resolve(import.meta.dir,"../🔣️policy.json"),"utf8");
 expect(Object.keys(policy.platforms).sort()).toEqual(["darwin","linux","win32"]);
 for(const forbidden of["CAP_SYS_ADMIN","capEff","loop","losetup","mkfs","mount","fallocate","sudo"])expect(text.includes(forbidden),forbidden).toBe(false);
 expect(policy.platforms.linux.exchange).toBe(true);expect(policy.platforms.win32.exchange).toBe(true);expect(policy.platforms.darwin.exchange).toBe(false);
 for(const row of fixture.windows)expect(buildPreflightProblemsV1(policy.platforms.win32,row.facts),row.id).toEqual(row.problems);
 for(const row of fixture.neutral)expect(buildPreflightProblemsV1(policy.platforms[row.platform as "darwin"|"win32"],row.facts),row.id).toEqual(row.problems);
});
test("the entry delegates to phased preflight without opening stores or accepting scope bypasses",()=>{
 const entry=readFileSync(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/🩺️preflight/🟦️.ts"),"utf8");
 expect(entry.indexOf("await runBuildPreflightV1")).toBeGreaterThan(-1);expect(entry.includes("openRepositoryCachePartitionV1")||entry.includes("withRepositoryCacheStoreV1")).toBe(false);
 expect(entry.includes("preflight --scope")).toBe(false);
});
