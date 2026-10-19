import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtemp,mkdir,readFile,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {bazelNonAsciiV1,bazelPlatformProblemsV1,bazelProgressLineV1,collectBazelPlatformFactsV1,describeBazelReadinessV1,requireBazelPlatformV1,type BazelProgressEventV1} from "../🟦️.ts";

const oracle=new Ajv({strict:true}),admitted=oracle.compile(schema.definitions.PlatformFacts),ascii=oracle.compile({type:"string",pattern:"^[\\x00-\\x7F]*$"});

test("platform problems match the authored corpus and an independent closed-schema oracle",()=>{
 expect(admitted(fixture.facts),JSON.stringify(admitted.errors)).toBe(true);
 for(const row of fixture.cases){
  const facts={...fixture.facts,...row.patch};expect(admitted(facts),row.id+JSON.stringify(admitted.errors)).toBe(true);
  const report=bazelPlatformProblemsV1(facts);
  expect(report.refusals.length,row.id).toBe(row.refusals.length);expect(report.warnings.length,row.id).toBe(row.warnings.length);
  row.refusals.forEach((text,index)=>expect(report.refusals[index],row.id).toContain(text));row.warnings.forEach((text,index)=>expect(report.warnings[index],row.id).toContain(text));
  if(facts.platform==="win32")for(const entry of facts.paths)expect(ascii(entry.path)===!(report.refusals.concat(report.warnings).some(message=>message.includes(`path ${entry.path} (`))),row.id+entry.path).toBe(true);
 }
 for(const patch of fixture.invalid){const facts={...fixture.facts,...patch};expect(admitted(facts),JSON.stringify(patch)).toBe(false);expect(()=>bazelPlatformProblemsV1(facts),JSON.stringify(patch)).toThrow("Closed Bazel platform facts");}
});

test("non-ASCII detection agrees with the ECMAScript pattern oracle including astral characters",()=>{
 for(const row of fixture.nonAscii){expect(ascii(row.text),row.text).toBe(row.ascii);expect(bazelNonAsciiV1(row.text)===undefined,row.text).toBe(row.ascii);}
});

test("download progress emits one bounded line per phase and ten percent step",()=>{
 const lines:string[]=[];let last=0;
 for(const row of fixture.progress.events){const step=bazelProgressLineV1({phase:row.phase,bytes:row.bytes,total:fixture.progress.total} as BazelProgressEventV1,last);last=step.lastPercent;if(step.line)lines.push(step.line);}
 expect(lines).toEqual(fixture.progress.lines);
});

test("readiness reports the pinned binary and the module catalog and agrees with an independent hash",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"semio-bazel-readiness-")),blobs=join(directory,"repositories/content_addressable/sha256"),digest=(value:string)=>new Bun.CryptoHasher("sha256").update(value).digest("hex");
 try{
  await mkdir(join(blobs,digest("a")),{recursive:true});await mkdir(join(blobs,digest("b")),{recursive:true});await mkdir(join(blobs,"not-a-digest"),{recursive:true});
  const input={executable:join(directory,"bazel"),platform:"linux",arch:"x64",sha256:digest("binary"),lock:"{\"lockFileVersion\":1}",repository:join(directory,"repositories")},report=await describeBazelReadinessV1(input);
  const validate=oracle.compile(schema.definitions.Readiness);expect(validate(report.readiness),JSON.stringify(validate.errors)).toBe(true);
  expect(report.readiness.modules).toEqual({lockSha256:digest(input.lock),blobs:2});expect(report.readiness.bazel.version).toBe(schema.definitions.Release.const.version);
  expect(report.line).toContain(`binary=${schema.definitions.Release.const.version} linux/x64`);expect(report.line).toContain("modules=2 blobs");
  await expect(describeBazelReadinessV1({...input,sha256:"short"})).rejects.toThrow("Closed Bazel readiness");
 }finally{await rm(directory,{recursive:true,force:true});}
});

test("the observed host satisfies the closed facts schema and Darwin needs no privilege",async()=>{
 if(!["linux","darwin","win32"].includes(process.platform))return;
 const root=resolve(import.meta.dir,"../../../../../../../../../../.."),facts=await collectBazelPlatformFactsV1(root,{tools:join(root,".cache/b/t"),dependencies:join(root,".cache/b/d"),products:join(root,".cache/b/p")},new AbortController().signal);
 expect(admitted(facts),JSON.stringify(admitted.errors)).toBe(true);expect(facts.paths.map(row=>row.label)).toEqual(["workspace","tools","dependencies","products","temporary"]);
 if(process.platform==="darwin")expect(bazelPlatformProblemsV1(facts).refusals).toEqual([]);
 const controller=new AbortController();controller.abort(Error("cancelled"));
 await expect(collectBazelPlatformFactsV1(root,{tools:root,dependencies:root,products:root},controller.signal)).rejects.toThrow("cancelled");
 await expect(requireBazelPlatformV1(root,controller.signal)).rejects.toThrow("cancelled");
});

test("tools setup bazel checks the platform before attaching stores and prepares the module catalog after the binary",async()=>{
 const entry=await readFile(resolve(import.meta.dir,"../../../../../../🏗️build/📜️script.ts"),"utf8"),branch=entry.slice(entry.indexOf('if(tools!=="nx"){'));
 const order=["requireBazelPlatformV1(root","withBazelStorageV1(root","bootstrap.prepareBazel(root","prepareBazelModulesV1(root","describeBazelReadinessV1({"].map(marker=>branch.indexOf(marker));
 expect(order.every(index=>index>0)).toBe(true);expect([...order].sort((a,b)=>a-b)).toEqual(order);
});

test("Linux readiness demands no privilege, loop device or mount tool",()=>{
 const text=JSON.stringify(schema.definitions.PlatformPolicy.const);
 for(const forbidden of["CAP_SYS_ADMIN","loop","mkfs","losetup","privileged","fallocate"])expect(text.includes(forbidden),forbidden).toBe(false);
 const report=bazelPlatformProblemsV1({...fixture.facts,exchange:"unsupported"});
 for(const message of report.refusals)for(const forbidden of["CAP_SYS_ADMIN","privileged","--privileged","sudo"])expect(message.includes(forbidden),forbidden).toBe(false);
});
