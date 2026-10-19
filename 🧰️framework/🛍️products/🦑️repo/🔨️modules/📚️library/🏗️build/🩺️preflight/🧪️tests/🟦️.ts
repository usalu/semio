import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {findWorkspaceRoot} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {buildPreflightProblemsV1,readBuildPreflightArgumentsV1,readBuildPreflightPolicyV1} from "../🟦️.ts";

const root=findWorkspaceRoot(import.meta.dir);
test("portable preflight corpus agrees with independent closed-schema admission",()=>{
 const oracle=new Ajv({strict:true}),requirements=oracle.compile(schema.definitions.Platform),facts=oracle.compile(schema.definitions.Facts);
 const admitted=oracle.compile({type:"object",required:["architecture","availableBytes","tools","exchange","caseSensitive"],properties:{architecture:{enum:fixture.requirements.architectures},availableBytes:{type:"integer",minimum:fixture.requirements.minimumAvailableBytes},tools:{type:"array",allOf:fixture.requirements.tools.map(tool=>({contains:{const:tool}}))},exchange:{const:"supported"},caseSensitive:{const:"yes"}}});
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
test("the entry runs preflight before opening any store and takes no scope",()=>{
 const entry=readFileSync(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"),"utf8");
 expect(entry.indexOf("await runBuildPreflightV1")).toBeLessThan(entry.indexOf('const {openRepositoryCachePartitionV1}'));
 expect(entry.includes("preflight --scope")).toBe(false);
});
