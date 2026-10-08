import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import TOML from "@iarna/toml";
import {cargoNextestConfiguration} from "../../🟦️.ts";
import {repositoryCargoTestPolicyV1} from "../../../../🟦️.ts";
import fixture from "../../🧫️fixtures/🏎️nextest-configuration/🔣️.json";

test("native test profiles belong to their selected workspace or repository configuration",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");mkdirSync(output,{recursive:true});
 for(const row of fixture.cases){
  const root=mkdtempSync(join(output,"nextest-profile-"));mkdirSync(join(root,row.directory),{recursive:true});
  for(const directory of row.configurations){const path=join(root,directory,".config/nextest.toml");mkdirSync(dirname(path),{recursive:true});writeFileSync(path,Object.entries(fixture.periods).map(([level,period])=>'[profile.'+level+']\nslow-timeout={period="'+period+'",terminate-after=1}\n').join("\n"));}
  if(row.expected===null){expect(()=>cargoNextestConfiguration(root,row.directory)).toThrow("configuration is missing");continue;}
  const selected=cargoNextestConfiguration(root,row.directory);expect(selected).toBe(resolve(root,row.expected));
  const independent=TOML.parse(readFileSync(selected,"utf8")) as any;for(const[level,period]of Object.entries(fixture.periods))expect(independent.profile[level]["slow-timeout"]).toEqual({period,"terminate-after":1});
 }
 const root=mkdtempSync(join(output,"nextest-profile-escaped-"));expect(()=>cargoNextestConfiguration(root,"../outside")).toThrow("escapes repository");
 for(const row of fixture.nativePackages){const repository=process.cwd(),policy=repositoryCargoTestPolicyV1(join(repository,row.manifest),repository,{...process.env,SEMIO_TEST_LEVEL:"quick"});expect(policy.configPath).toBe(join(repository,row.configuration));const independent=TOML.parse(readFileSync(policy.configPath,"utf8")) as any;expect(independent.profile.quick["slow-timeout"].period).toBe(fixture.periods.quick);}
 console.log("[DEBUG] Native Nextest selection agrees with independent TOML for selected workspace and repository scope");
});

test("isolated artifact Nextest executes a real law without unrelated product filters",()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");mkdirSync(output,{recursive:true});
 const repository=process.cwd(),root=mkdtempSync(join(output,"nextest-native-witness-")),directory="product/artifact",native=join(root,directory),manifest=join(native,"Cargo.toml");
 for(const configured of [".","product"]){mkdirSync(join(root,configured,".config"),{recursive:true});writeFileSync(join(root,configured,".config/nextest.toml"),readFileSync(join(repository,configured==="."?".config/nextest.toml":"✏️s/.config/nextest.toml"),"utf8"));}
 mkdirSync(native,{recursive:true});writeFileSync(manifest,'[package]\nname="owned-profile-witness"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="🦀️.rs"\n[workspace]\nmembers=["."]\n');writeFileSync(join(native,"🦀️.rs"),'#[test]\nfn native_profile_witness(){println!("[DEBUG] Isolated native profile law executed");assert_eq!(2+2,4);}\n');
 const selected=cargoNextestConfiguration(root,directory),result=Bun.spawnSync(["cargo","nextest","run","--offline","--manifest-path",manifest,"--config-file",selected,"--profile","quick","--no-tests","fail","--success-output","immediate","--target-dir",join(root,"target")],{cwd:native,stdout:"pipe",stderr:"pipe",env:{...process.env,CARGO_TARGET_DIR:join(root,"target"),CARGO_BUILD_BUILD_DIR:join(root,"target"),NEXTEST_PROFILE:"quick"}});
  const transcript=(result.stdout.toString()+result.stderr.toString()).replace(/\u001b\[[0-9;]*m/g,"");if(result.exitCode!==0)throw Error(transcript);expect(transcript).toContain("[DEBUG] Isolated native profile law executed");expect(transcript).toContain("1 test run");console.log("[DEBUG] Actual Nextest compiled and executed the isolated native profile law");
},120000);
