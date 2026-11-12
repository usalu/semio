/** 🧪️ Tests for the typed disk refusals: the authored corpus in `🔣️.json` is checked here and the byte format by the independent Python oracle. */
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{afterAll,expect,test}from"bun:test";
import corpus from"./🔣️.json"with{type:"json"};
import{reportBoundaryErrorV1}from"../../🗣️language/📣️progress/🟦️.ts";
import{renderRepoErrorV1,repoErrorExitCodeV1}from"../../🗣️language/🚨️error/🟦️.ts";
import{formatDiskBytesV1,refuseDiskFullV1,refuseLowSpaceV1}from"../🟦️.ts";

const directories:string[]=[];
afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});

test("bytes are written in binary units exactly as the independent Python oracle writes them",()=>{
 const run=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(corpus.bytes.map(row=>row.bytes)),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const python=JSON.parse(run.stdout) as string[];
 corpus.bytes.forEach((row,index)=>{expect(formatDiskBytesV1(row.bytes)).toBe(row.text);expect(python[index]).toBe(row.text);});
});

for(const row of corpus.full)test(`full disk: ${row.name}`,()=>{
 const error=Object.assign(new Error(row.error.message),"code"in row.error?{code:row.error.code}:{},"path"in row.error?{path:row.error.path}:{});
 const refusal=refuseDiskFullV1(error,{free:()=>row.availableBytes===null?undefined:{availableBytes:row.availableBytes,totalBytes:row.totalBytes??undefined},mount:()=>row.partition,probes:["/probe"]});
 expect(refusal?.code).toBe("failed");
 expect(renderRepoErrorV1(refusal!,"en")).toBe(row.en);
 expect(renderRepoErrorV1(refusal!,"de")).toBe(row.de);
 expect(repoErrorExitCodeV1(refusal!.code)).toBe(1);
});

for(const row of corpus.low)test(`low space: ${row.name}`,()=>{
 const refusal=refuseLowSpaceV1({partition:row.partition,freeBytes:row.availableBytes,totalBytes:row.totalBytes,limitFraction:row.limitFraction});
 expect(refusal.code).toBe("refused");
 expect(renderRepoErrorV1(refusal,"en")).toBe(row.en);
 expect(renderRepoErrorV1(refusal,"de")).toBe(row.de);
});

test("a failure that is no full disk is not turned into a disk refusal",()=>{
 expect(refuseDiskFullV1(new Error("EACCES: permission denied"))).toBeUndefined();
});

test("the CLI boundary reports a full disk as the typed refusal in both languages, never as an internal error",()=>{
 const root=mkdtempSync(join(tmpdir(),"disk-boundary-"));directories.push(root);
 for(const language of ["en","de"] as const){
  const lines:string[]=[];let status=-1;
  reportBoundaryErrorV1(Object.assign(new Error("ENOSPC: no space left on device, write"),{code:"ENOSPC",path:join(root,"missing","file")}),{environment:{SEMIO_LANGUAGE:language},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:code=>{status=code;}});
  const text=lines.join("\n");
  expect(text).toContain("repo cache cas gc");expect(text).not.toMatch(/Internal error|Interner Fehler/u);
  expect(text).toContain(language==="en"?"The disk is full":"Der Datenträger ist voll");expect(status).toBe(1);
 }
});
