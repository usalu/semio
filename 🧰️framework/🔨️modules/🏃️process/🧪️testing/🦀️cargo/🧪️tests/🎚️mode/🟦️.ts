import {test,expect} from "bun:test";
import Ajv from "ajv";
import {parseArgs} from "node:util";
import {spawnSync} from "node:child_process";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {cargoTestPlanV1,cargoTestBuildLeaseIdentityV1,cargoLintPlanV1,readCargoTestPolicyV1} from "../../🟦️.ts";

test("original Cargo policy preserves compiler mode independently of Nextest assertion profile",()=>{
 const validate=new Ajv({strict:true}).compile(schema),parse=(args:string[])=>parseArgs({args,options:{profile:{type:"string"},"cargo-profile":{type:"string"}},strict:false}).values;
 for(const mode of fixture.modes)for(const nextest of [false,true])for(const coverageEnabled of [false,true]){
  const policy={...fixture.policies[0],mode,nextest,coverageEnabled,coveragePath:coverageEnabled?"/owner/coverage.lcov":null};expect(validate(policy)).toBe(true);
  const request={manifestPath:policy.manifestPath,packages:["owner-package"],cwd:"/owner",extraArgs:["--lib"]};const plan=cargoTestPlanV1(request,policy as never,"/owner/results/metadata.json"),compiled=parse(plan[0]!.args),identity=cargoTestBuildLeaseIdentityV1(request,policy as never);
  expect(nextest?compiled["cargo-profile"]:compiled.profile).toBe(mode);expect(identity.profile).toBe(mode);expect(parse(cargoLintPlanV1(request,policy as never).args).profile).toBe(mode);if(nextest)expect(compiled.profile).toBe(policy.level);
 }
 for(const mode of fixture.refusedModes){const policy={...fixture.policies[0],mode};expect(validate(policy)).toBe(false);expect(()=>readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify(policy)})).toThrow()}
 for(const args of [["--release"],["-r"],["-Pquick"],["--profile","dev"],["--cargo-profile=release"]])expect(()=>cargoTestPlanV1({manifestPath:fixture.policies[0].manifestPath,packages:[],cwd:"/owner",extraArgs:args},fixture.policies[0] as never,"/owner/metadata.json")).toThrow("profile");
 for(const [args,flag]of [[ ["test","--help"],"--profile"],[["check","--help"],"--profile"],[["nextest","list","--help"],"--cargo-profile"],[["llvm-cov","test","--help"],"--profile"],[["nextest","archive","--help"],"--cargo-profile"],[["llvm-cov","report","--help"],"--profile"]]as const){const help=spawnSync("cargo",[...args],{encoding:"utf8",timeout:5000});expect(help.status).toBe(0);expect(help.stdout+help.stderr).toContain(flag)}
});
