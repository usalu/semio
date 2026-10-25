/** 📌️ Contract: Bazel, Bun and the Nx native packages are pinned by digest for darwin, linux and win32 on arm64 and x64, so `tools setup` downloads exactly one verified artifact per host without a manual step. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import fixture from "./🔣️.json";
import bazelSchema from "../../🧬️schema/🔣️.json";
import bunSchema from "../../../🟦️bun/🧬️schema/🔣️.json";
import tools from "../../../package.json";
import {bazelDistribution} from "../../📜️script.ts";
import {nxDependencyInstallFilesV1,nxExternalPinsV1} from "../../../📦️sdk/📥️source/📐️projection/🟦️.ts";

const bazel=bazelSchema.definitions.Release.const,bun=bunSchema.definitions.Release.const,lock=Bun.JSONC.parse(readFileSync(join(import.meta.dir,"../../../bun.lock"),"utf8")) as {packages:Record<string,[string,string,unknown,string]>};
const python=testPythonV1();
const hex=/^[a-f0-9]{64}$/u;

test("Bazel pins one verified binary per OS and CPU pair, selected by the production function",()=>{
 const digests=new Set<string>();
 for(const pair of fixture.pairs){
  const row=bazelDistribution(pair.platform,pair.arch);
  expect(row.file,`${pair.platform}/${pair.arch}`).toBe(pair.bazel);expect(row.sha256).toMatch(hex);expect(row.bytes).toBeGreaterThan(10_000_000);expect(row.file.endsWith(".exe")).toBe(pair.platform==="win32");digests.add(row.sha256);
 }
 expect(digests.size).toBe(fixture.pairs.length);expect(bazel.release).toBe(`https://releases.bazel.build/${bazel.version}/release`);
 expect(readFileSync(join(import.meta.dir,...Array(11).fill(".."),".bazelversion"),"utf8").trim()).toBe(bazel.version);
});

test("Bun pins one archive per OS and CPU pair",()=>{
 for(const pair of fixture.pairs){
  const rows=bun.distributions.filter(row=>row.platform===pair.platform&&row.arch===pair.arch&&!row.musl);
  expect(rows.length,`${pair.platform}/${pair.arch}`).toBe(1);expect(rows[0]!.archive).toBe(pair.bun);expect(rows[0]!.sha256).toMatch(hex);expect(rows[0]!.bytes).toBeGreaterThan(10_000_000);
 }
});

test("the Nx tooling lock holds an integrity digest for the native package of every OS and CPU pair",()=>{
 const version=tools.dependencies.nx;
 for(const pair of fixture.pairs){
  const entry=lock.packages[pair.nx];expect(entry,pair.nx).toBeDefined();expect(entry![0]).toBe(`${pair.nx}@${version}`);expect(entry![3]).toMatch(/^sha512-[A-Za-z0-9+/]{86}==$/u);
 }
 if(python){
  const result=Bun.spawnSync([python,"-I","-c","import json,re,sys\ntext,names=json.loads(sys.stdin.read())\nprint(json.dumps({n:(re.search(r'\"'+re.escape(n)+r'\": \\[\"[^\"]+\", \"\", \\{[^}]*\\}, \"(sha512-[^\"]+)\"\\]',text) or [None,None])[1] for n in names}))"],{stdin:new TextEncoder().encode(JSON.stringify([readFileSync(join(import.meta.dir,"../../../bun.lock"),"utf8"),fixture.pairs.map(pair=>pair.nx)])),stdout:"pipe",stderr:"pipe"});
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  expect(JSON.parse(result.stdout.toString())).toEqual(Object.fromEntries(fixture.pairs.map(pair=>[pair.nx,lock.packages[pair.nx]![3]])));
 }
});

test("workspace links in the Nx tooling recipe are never installer pins, so the captured install equals the lock",()=>{
 expect(nxExternalPinsV1({nx:"23.2.0","@semio-tech/framework-filesystem":"workspace:*"})).toEqual({nx:"23.2.0"});
 const recipe={name:tools.name,private:true as const,packageManager:tools.packageManager,dependencies:tools.dependencies as Record<string,string>,semio:tools.semio,exports:tools.exports,devDependencies:tools.devDependencies},locked=(lock as unknown as {workspaces:Record<string,{dependencies:Record<string,string>}>}).workspaces[""]!.dependencies;
 const installed=JSON.parse(nxDependencyInstallFilesV1(recipe,lock).get("package.json")!.toString()) as {dependencies:Record<string,string>};
 expect(installed.dependencies).toEqual(locked);expect(Object.values(installed.dependencies).every(version=>/^[0-9]+\.[0-9]+\.[0-9]+$/u.test(version))).toBe(true);
 expect(()=>nxDependencyInstallFilesV1({...recipe,dependencies:{nx:"^23.2.0"}},lock)).toThrow("Closed pinned dependency recipe required");
});

test.skipIf(!process.env.SEMIO_TEST_NETWORK)("the pinned Bazel digests equal the checksums published beside the upstream release",async()=>{
 for(const pair of fixture.pairs){
  const response=await fetch(fixture.upstream.bazelChecksum.replace("{version}",bazel.version).replace("{file}",pair.bazel));
  expect(response.ok,pair.bazel).toBe(true);expect((await response.text()).trim().split(/\s+/u)[0],pair.bazel).toBe(bazelDistribution(pair.platform,pair.arch).sha256);
 }
},120000);
