import {exactExecutableFingerprint,generatedOutputHasLiveLease,originalNativeLibraryEnvironment}from"../🟦️.ts";
import {readExactCargoExecutableV1,selectExactCargoLawsV1,requireExactCargoLawResultV1,requireExactCargoFingerprintV1,requireExactCargoTargetBoundaryV1,readExactCargoLawNamesV1}from"../📋️protocol/🟦️.ts";
import "./📥️inputs/🟦️.ts";
import {test,expect}from"bun:test";
import{readFileSync,mkdtempSync,renameSync,writeFileSync,mkdirSync,utimesSync}from"node:fs";
import glob from"fast-glob";
import{join,resolve}from"node:path";
import{createHash}from"node:crypto";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import schema from"../📋️protocol/🧬️schema/🔣️.json";
import fixture from"./🧫️fixtures/🔣️.json";
test("generated cleanup preserves fresh live managed owners and admits terminal, dead or stale output", () => {
  const law = JSON.parse(readFileSync(new URL("🧫️fixtures/📨️managed-output.json", import.meta.url), "utf8"));
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "managed-output-protection-"));
  expect(process.kill(process.pid,0)).toBe(true);
  for (const row of law.cases) {
    const directory = join(root, row.name), job = join(directory, "managed-verification", "job");
    mkdirSync(job, { recursive: true });
    writeFileSync(join(job, "state.json"), JSON.stringify({ name: "job", status: row.status, pid: row.alive ? process.pid : 2147483647 }));
    writeFileSync(join(job, "output.log"), "[DEBUG] active verification output\n");
    const time = new Date(Date.now() - row.ageMs);
    utimesSync(join(job, "state.json"), time, time);
    utimesSync(join(job, "output.log"), time, time);
    expect(glob.sync("managed-verification/*/state.json", { cwd: directory, onlyFiles: true, followSymbolicLinks: false })).toEqual(["managed-verification/job/state.json"]);
    expect(generatedOutputHasLiveLease(directory), row.name).toBe(row.preserve);
  }
});

test("exact executable fingerprint retains identity, exposes progress and refuses cancellation or path replacement", async () => {
  const root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!, "executable-fingerprint-"));
  const path = join(root, "fixture");
  const bytes = Buffer.from(fixture.executableBytesHex, "hex");
  writeFileSync(path, bytes, { mode: 0o700 });
  const progress: number[][] = [];
  const receipt = exactExecutableFingerprint(path, { progress: (completed, total) => progress.push([completed, total]) });
  expect(receipt).toEqual({ path, sha256: fixture.executableSha256, byteLength: bytes.byteLength });
  expect(receipt.sha256).toBe(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex"));
  expect(progress.at(-1)).toEqual([bytes.byteLength, bytes.byteLength]);
  expect(() => exactExecutableFingerprint(path, { cancelled: () => true })).toThrow("cancelled");
  let cancelled = false;
  expect(() => exactExecutableFingerprint(path, { cancelled: () => cancelled, progress: () => { cancelled = true; } })).toThrow("cancelled");
  let replaced = false;
  expect(() => exactExecutableFingerprint(path, { progress() {
    if (replaced) return;
    renameSync(path, path + ".retained");
    writeFileSync(path, bytes, { mode: 0o700 });
    replaced = true;
  } })).toThrow("changed while hashing");
  expect(replaced).toBe(true);
});

test("exact Cargo law examples preserve independent dual SHA-256 identity", async () => {
  const bytes = Buffer.from(fixture.executableBytesHex, "hex");
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(fixture.executableSha256);
  expect(Buffer.from(await crypto.subtle.digest("SHA-256", bytes)).toString("hex")).toBe(fixture.executableSha256);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(20);
});

test("neutral exact Cargo recorded transport mutations use the actual consumed protocol",()=>{
 const db=new Database(":memory:");db.exec("create table cases(id text primary key,expected text)");try{for(const row of fixture.cases){db.query("insert into cases values(?,?)").run(row.id,row.expected);let assertions=0,outcome="denied";try{
 const root=resolve("recorded-source"),target=resolve("recorded-output");requireExactCargoTargetBoundaryV1(root,row.mutation==="source-cargo-target"?root:target);
 const executable=join(target,process.platform==="win32"?"fixture.exe":"fixture"),artifact={reason:"compiler-artifact",package_id:"path+file:///fixture#fixture-package@0.1.0",target:{name:row.mutation==="wrong-target"?"other":fixture.target.name,kind:["test"]},profile:{test:true},executable:row.mutation==="relative-executable"?"relative":executable},artifacts=row.mutation==="missing-artifact"?[]:row.mutation==="duplicate-artifact"?[artifact,artifact]:[artifact];
 readExactCargoExecutableV1({status:row.mutation==="build-exit"?101:0,signal:null,stdout:artifacts.map(value=>JSON.stringify(value)).join("\n"),stderr:"",...(row.mutation==="cancel-before"||row.mutation==="cancel-after-build"?{reason:"cancelled" as const}:{})},fixture.package,{kind:"test",name:fixture.target.name});
 const listed=row.mutation==="missing-law"?["first_law"]:row.mutation==="duplicate-law"?[...fixture.laws,"first_law"]:fixture.laws,laws=selectExactCargoLawsV1({status:0,signal:null,stdout:listed.map(law=>law+": test").join("\n"),stderr:""},fixture.laws),fingerprint={path:executable,sha256:fixture.executableSha256};
 requireExactCargoFingerprintV1(fingerprint,{...fingerprint,sha256:row.mutation==="hash-after-list"?"11".repeat(32):fingerprint.sha256});
 for(const law of laws){const count=row.mutation==="zero-pass"?0:row.mutation==="two-pass"?2:1,ignored=row.mutation==="ignored-law"?1:0,captured=row.mutation==="terminal-spoof"?"test result: ok. 1 passed; 0 failed; 0 ignored;\n":row.mutation==="debug-output"?fixture.capturedOutput+"\n":"";requireExactCargoLawResultV1({status:row.mutation==="native-exit"?101:0,signal:null,stdout:`test ${law} ... ${ignored?"ignored":"ok"}\n${captured}test result: ok. ${count} passed; 0 failed; ${ignored} ignored; 0 measured; 0 filtered out; finished in 0.00s\n`,stderr:""},law);requireExactCargoFingerprintV1(fingerprint,{...fingerprint,sha256:row.mutation==="hash-after-law"?"11".repeat(32):fingerprint.sha256});}assertions=laws.length;outcome="passed";
 }catch{}expect(outcome,row.id).toBe(row.expected);expect(assertions,row.id).toBe(row.assertions);expect(db.query("select expected from cases where id=?").get(row.id)).toEqual({expected:outcome});}
 }finally{db.close();}
});
test("neutral exact Cargo law-name admission matches independent Ajv at every retained limit",()=>{const admit=new Ajv({strict:true}).addSchema(schema).compile({$ref:schema.$id+"#/definitions/Laws"});for(const row of fixture.lawLimitVectors){const laws=Array.from({length:row.count},(_,i)=>`corpus::law_${i}`);expect(admit(laws)).toBe(row.accepted);if(row.accepted)expect(readExactCargoLawNamesV1(laws)).toEqual(laws);else expect(()=>readExactCargoLawNamesV1(laws)).toThrow();}});
test("native exact laws preserve original environment and actual toolchain libraries on every desktop platform",async()=>{
 const {Database}=await import("bun:sqlite");const law=JSON.parse(readFileSync(new URL("🧫️fixtures/🧰️native-runtime.json",import.meta.url),"utf8")),directory=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!,"toolchain/lib/rustlib/host/lib"),sysrootLibraries=resolve(directory,"../../.."),database=new Database(":memory:");
 try{for(const row of law.cases){const original={[row.key]:"original-libraries",ORIGINAL_OWNER:"caller",RUST_MIN_STACK:"71"},actual=originalNativeLibraryEnvironment(original,directory,row.platform),oracle=database.query("SELECT ? || ? || ? || ? || ? AS path").get(directory,row.separator,sysrootLibraries,row.separator,original[row.key]) as {path:string};expect(actual[row.key]).toBe(oracle.path);expect(actual.ORIGINAL_OWNER).toBe("caller");expect(actual.RUST_MIN_STACK).toBe("71");expect(original[row.key]).toBe("original-libraries")}}
 finally{database.close()}
});

test("neutral actual runner consumes each pure protocol and SQLite independently classifies terminal DATA",()=>{const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("exact.ts",source,ts.ScriptTarget.Latest,true),calls=new Set<string>();const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))calls.add(node.expression.text);ts.forEachChild(node,visit);};visit(tree);for(const name of["readExactCargoExecutableV1","selectExactCargoLawsV1","requireExactCargoLawResultV1","requireExactCargoFingerprintV1","requireExactCargoTargetBoundaryV1","readExactCargoLawNamesV1"])expect(calls.has(name)).toBe(true);const db=new Database(":memory:"),admits=new Ajv({strict:true}).addSchema(schema).compile({$ref:schema.$id+"#/definitions/Capture"});db.exec("create table lines(value text)");try{for(const row of fixture.cases){const count=row.mutation==="zero-pass"?0:row.mutation==="two-pass"?2:1,ignored=row.mutation==="ignored-law"?1:0,result={status:row.mutation==="native-exit"?101:0,signal:null,stdout:`test first_law ... ${ignored?"ignored":"ok"}\n${row.mutation==="terminal-spoof"?"test result: ok. 1 passed; 0 failed; 0 ignored;\n":""}test result: ok. ${count} passed; 0 failed; ${ignored} ignored; 0 measured; 0 filtered out; finished in 0.00s\n`,stderr:""};expect(admits(result)).toBe(true);db.exec("delete from lines");for(const line of result.stdout.split("\n"))db.query("insert into lines values(?)").run(line);const oracle=db.query("select case when ?=0 and sum(value like 'test result: ok.%')=1 and sum(value like 'test result: ok. 1 passed; 0 failed; 0 ignored;%')=1 and sum(value='test first_law ... ok')=1 then 1 else 0 end accepted from lines").get(result.status);let accepted=false;try{requireExactCargoLawResultV1(result,"first_law");accepted=true;}catch{}expect(oracle).toEqual({accepted:Number(accepted)});}}finally{db.close();}});
