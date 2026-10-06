import { test, expect } from "bun:test";
import { readFileSync, writeFileSync, mkdtempSync, mkdirSync, existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import { buildCargoArtifacts, readCargoArtifactBuildPolicyV1 } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
test("native artifact policy is explicit and schema-first", () => {
  const validate = new Ajv({strict:true}).compile(schema);
  expect(validate(fixture.policy)).toBe(true);
  expect(readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(fixture.policy)},fixture.policy.cwd)).toEqual(fixture.policy);
  for(const mutation of [{...fixture.policy,budgetMs:-1},{...fixture.policy,leaseDirectory:"relative"},{...fixture.policy,extra:true}]){expect(validate(mutation)).toBe(false);expect(()=>readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(mutation)},fixture.policy.cwd)).toThrow();}
  expect(()=>readCargoArtifactBuildPolicyV1({},fixture.policy.cwd)).toThrow();
  expect(()=>readCargoArtifactBuildPolicyV1({SEMIO_CARGO_ARTIFACT_POLICY:JSON.stringify(fixture.policy)},"/foreign")).toThrow();
});
for(const vector of fixture.vectors)test(`captured native artifacts: ${vector.id}`, async()=>{
  const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"native-capture-")), source=join(root,"owner"), emitted=join(root,"emitted");mkdirSync(source);mkdirSync(emitted);
  const manifest=join(source,"Cargo.toml");writeFileSync(manifest,'[package]\nname="fixture"\nversion="0.1.0"\n');
  for(const file of fixture.files)writeFileSync(join(emitted,file.name),Buffer.from(file.hex,"hex"),{mode:0o700});
  const primary={reason:"compiler-artifact",package_id:`path+${pathToFileURL(source).href}#fixture@0.1.0`,target:{kind:["bin"],name:"fixture"},filenames:[join(emitted,fixture.files[0].name)],executable:join(emitted,fixture.files[0].name)};
  const compiler=vector.mode==="wait"?'setInterval(()=>{},1000)':vector.mode==="fail"?'process.exit(19)':vector.mode==="empty"?'process.stdout.write("compiler empty\\n")':`process.stdout.write(${JSON.stringify(JSON.stringify(primary)+'\n')})`;
  const policy={version:1 as const,cwd:source,buildDirectory:join(root,"compiler"),leaseDirectory:join(root,"leases"),captureDirectory:join(root,"captures"),budgetMs:5000};
  const control=new AbortController();let cancel:ReturnType<typeof setTimeout>|undefined;if(vector.mode==="wait")cancel=setTimeout(()=>control.abort(),150);
  let failure:unknown;try{await buildCargoArtifacts(manifest,[],policy,{signal:control.signal,commandPort:{command:process.execPath,args:["-e",compiler]}});}catch(error){failure=error;}finally{clearTimeout(cancel);}
  if(vector.accepted){expect(failure).toBeUndefined();const bytes=readFileSync(join(source,"dist/build",fixture.files[0].name));expect(bytes.toString("hex")).toBe(fixture.files[0].hex);expect(createHash("sha256").update(bytes).digest("hex")).toBe(Buffer.from(await crypto.subtle.digest("SHA-256",Buffer.from(fixture.files[0].hex,"hex"))).toString("hex"));}else{expect(failure).toBeInstanceOf(Error);expect(existsSync(join(source,"dist/build",fixture.files[0].name))).toBe(false);}
  expect(readdirSync(policy.captureDirectory)).toEqual([]);
});

test("actual Cargo artifact capture executes the compiler's retained executable and records its sources",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"real-cargo-capture-")),source=join(root,"owner");mkdirSync(source);
 const manifest=join(source,"Cargo.toml"),main=join(source,"main.rs");writeFileSync(manifest,`[workspace]\n[package]\nname=${JSON.stringify(fixture.cargo.name)}\nversion="0.0.0"\nedition="2021"\n[[bin]]\nname=${JSON.stringify(fixture.cargo.name)}\npath="main.rs"\n`);writeFileSync(main,`fn main(){print!(${JSON.stringify(fixture.cargo.stdout)});}`);
 const policy={version:1 as const,cwd:source,buildDirectory:join(root,"compiler"),leaseDirectory:join(root,"leases"),captureDirectory:join(root,"captures"),budgetMs:10000};
 const locked=spawnSync("cargo",["generate-lockfile","--offline","--manifest-path",manifest],{cwd:source,encoding:"utf8",timeout:10000});expect(locked.status).toBe(0);
 await buildCargoArtifacts(manifest,["--bin",fixture.cargo.name],policy,{sourcesRecord:"sources.json"});
 const binary=join(source,"dist/build",fixture.cargo.name+(process.platform==="win32"?".exe":"")),result=spawnSync(binary,[],{encoding:"utf8"});expect(result.status).toBe(0);expect(result.stdout).toBe(fixture.cargo.stdout);
 const record=JSON.parse(readFileSync(join(source,"dist/build/sources.json"),"utf8"));expect(record.sources).toContain(main);
 const independent=spawnSync("cargo",["run","--offline","--manifest-path",manifest,"--quiet","--target-dir",join(root,"oracle")],{cwd:source,encoding:"utf8",env:{...process.env,CARGO_BUILD_BUILD_DIR:join(root,"oracle-build")},timeout:10000});expect(independent.status).toBe(0);expect(independent.stdout).toBe(result.stdout);
 expect(readdirSync(policy.captureDirectory)).toEqual([]);
}, 30_000);

test("original Cargo publication oracle preserves private final outputs and shared intermediates",async()=>{ const {testCargoArtifactPublication}=await import("./📦️publication/🟦️.ts");await testCargoArtifactPublication(process.env.SEMIO_TEST_ARTIFACT_DIR!); });
