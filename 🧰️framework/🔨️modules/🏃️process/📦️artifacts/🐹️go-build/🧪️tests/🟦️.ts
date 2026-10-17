import {phaseCompilerAdmissionControlV1} from '../../../🏗️build/🎟️capacity/🟦️.ts';
import { expect,test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdtempSync,writeFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { buildGoArtifactsV1,goBuildArgumentsV1 } from "../🟦️.ts";
test("Go optimizer policy and schema are explicit and match the independent validator",()=>{
 const validate=new Ajv({strict:true}).compile(schema),base={version:1,cwd:"/owner",captureDirectory:"/captures",leaseDirectory:"/leases",budgetMs:0};
 for(const row of fixture.modes){expect(validate({...base,mode:row.mode})).toBe(true);expect(goBuildArgumentsV1(row.mode as "prototype"|"release",["."])).toEqual([...row.flags,"."]);}
 for(const mode of ["dev","ship",undefined])expect(validate({...base,...(mode?{mode}:{})})).toBe(false);
 expect(()=>goBuildArgumentsV1("prototype",["-ldflags=-s -w","."])).toThrow();
});
test("Go compiles both modes, publishes immutable outputs and matches the standard compiler runtime",async()=>{
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"g-"));writeFileSync(join(directory,"go.mod"),'module mode-fixture\n\ngo 1.25\n');writeFileSync(join(directory,"main.go"),fixture.source);
 const environment={...process.env,GOWORK:"off",GOPROXY:"off",GOTOOLCHAIN:"local",GOCACHE:join(directory,"cache")};
 for(const row of fixture.modes){const binary=join(directory,"dist/build",row.mode,`fixture${process.platform==="win32"?".exe":""}`),policy={version:1 as const,mode:row.mode as "prototype"|"release",cwd:directory,captureDirectory:join(directory,"c"),leaseDirectory:process.env.SEMIO_TEST_ARTIFACT_DIR!,budgetMs:120000};await buildGoArtifactsV1(["-o",binary,"."],policy,{environment,control:fixtureAdmissionV1()});const actual=spawnSync(binary,[],{encoding:"utf8"});expect(actual.status).toBe(0);expect(actual.stdout).toBe(fixture.stdout);
 const metadata=spawnSync("go",["version","-m",binary],{encoding:"utf8"});expect(metadata.status).toBe(0);expect(metadata.stdout).toContain(row.mode==="prototype"?"-gcflags":"-trimpath");
 const oracle=spawnSync("go",["run",...row.flags,"."],{cwd:directory,env:environment,encoding:"utf8",timeout:120000});expect(oracle.status).toBe(0);expect(oracle.stdout).toBe(actual.stdout);}
},300000);

/** 🎛️ Owns this fixture compiler phase with its actual signal and fixed test deadline. */
function fixtureAdmissionV1(signal:AbortSignal=new AbortController().signal){return phaseCompilerAdmissionControlV1(signal,120000,()=>undefined,()=>new Promise(done=>setImmediate(done)));}
