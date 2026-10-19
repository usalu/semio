import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {findWorkspaceRoot} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {buildPreflightProblemsV1,readBuildPreflightScopeV1,readBuildPreflightRequirementsV1} from "../🟦️.ts";

const root=findWorkspaceRoot(import.meta.dir);
test("portable preflight corpus agrees with independent closed-schema admission",()=>{
 const oracle=new Ajv({strict:true}),requirements=oracle.compile(schema.definitions.Requirements),facts=oracle.compile(schema.definitions.Facts);
 const admitted=oracle.compile({type:"object",required:["platform","architecture","distribution","distributionVersion","capEff","availableBytes","loopControl","freeLoops","tools"],properties:{platform:{const:fixture.requirements.platform},architecture:{const:fixture.requirements.architecture},distribution:{const:fixture.requirements.distribution},distributionVersion:{const:fixture.requirements.distributionVersion},capEff:{type:"string",pattern:"^[0-9a-fA-F]*[2367abefABEF][0-9a-fA-F]{5}$"},availableBytes:{type:"integer",minimum:fixture.requirements.minimumAvailableBytes},loopControl:{const:true},freeLoops:{type:"integer",minimum:fixture.requirements.minimumFreeLoops},tools:{type:"array",allOf:fixture.requirements.tools.map(tool=>({contains:{const:tool}}))}}});
 expect(requirements(fixture.requirements)).toBe(true);expect(requirements({...fixture.requirements,skip:true})).toBe(false);
 for(const row of fixture.cases){const observation={...fixture.facts,...row.patch};expect(facts(observation),row.id).toBe(true);expect(admitted(observation),row.id).toBe(row.problems.length===0);expect(buildPreflightProblemsV1(fixture.requirements,observation),row.id).toEqual(row.problems);}
 expect(()=>buildPreflightProblemsV1({...fixture.requirements,skip:true},fixture.facts)).toThrow("Closed build preflight requirements");
 for(const args of fixture.accepted)expect(readBuildPreflightScopeV1(args)).toBe("@teaching/architecture-course");for(const args of fixture.rejected)expect(()=>readBuildPreflightScopeV1(args)).toThrow();
});
test("authored owner preflight metadata is the portable requirement without opening any store",()=>{
 const requirements=readBuildPreflightRequirementsV1(root,"@teaching/architecture-course");expect(requirements).toEqual(fixture.requirements);
 const entry=readFileSync(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"),"utf8");
 expect(entry.indexOf("await runBuildPreflightV1")).toBeLessThan(entry.indexOf('const {openRepositoryCachePartitionV1}'));
 expect(()=>readBuildPreflightRequirementsV1(root,"@foreign/owner")).toThrow("Unknown build preflight scope");
});
