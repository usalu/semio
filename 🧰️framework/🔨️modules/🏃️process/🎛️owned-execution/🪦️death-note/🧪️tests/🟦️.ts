/** 🪦️ Contract: a child that dies from a signal nobody here sent leaves its reason behind, in both languages: the signal, its last stderr lines (or the plain statement that it wrote none) and, on macOS, the crash report the operating system wrote for exactly that pid.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {deathCauseV1,findCrashReportV1,ownedDeathNoteV1,windowsDeathStatusV1} from "../🟦️.ts";
import {runOwnedCommand} from "../../🟦️.ts";

const report=(pid:number,type="EXC_BREAKPOINT")=>`{"app_name":"bun"}\n{"pid" : ${pid},"exception" : {"type":"${type}","signal":"SIGKILL"},"termination":{"namespace":"FOUNDATION","code":1}}`;

test("the note names label, pid, signal and the last stderr lines in English then German",()=>{
 const note=ownedDeathNoteV1({label:"repository:command",pid:7,signal:"SIGKILL",status:null,stderrTail:"a\nb\nc\nd",lines:2});
 const [en,de]=note.split("\n");
 expect(en).toContain("repository:command");expect(en).toContain("7");expect(en).toContain("SIGKILL");expect(en).toContain("c | d");expect(en).not.toContain("a | b");
 expect(de).toContain("repository:command");expect(de).toContain("SIGKILL");expect(de).toContain("c | d");
});

test("a child without stderr output says so instead of staying silent",()=>{
 const [en,de]=ownedDeathNoteV1({label:"x",pid:1,signal:"SIGKILL",status:null,stderrTail:"  \n"}).split("\n");
 expect(en).toContain("no output");expect(de).toContain("keine Ausgabe");
});

test("the crash report of exactly that pid is found, others are not, and the note names it",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"death-note-"));
 try{
  writeFileSync(join(directory,"bun-a.ips"),report(11));writeFileSync(join(directory,"bun-b.ips"),report(22));writeFileSync(join(directory,"notes.txt"),"22");
  utimesSync(join(directory,"bun-a.ips"),new Date(Date.now()-3600_000),new Date(Date.now()-3600_000));
  expect(await findCrashReportV1(11,{directory,waitMilliseconds:0})).toBeUndefined();
  const found=await findCrashReportV1(22,{directory,waitMilliseconds:0});
  expect(found?.path).toBe(join(directory,"bun-b.ips"));expect(found?.exception).toContain("EXC_BREAKPOINT");
  const [en,de]=ownedDeathNoteV1({label:"x",pid:22,signal:"SIGKILL",status:null,stderrTail:"",report:found}).split("\n");
  expect(en).toContain("bun-b.ips");expect(de).toContain("bun-b.ips");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a report that appears a moment after the death is waited for, a missing directory is no report",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"death-note-"));
 try{
  setTimeout(()=>writeFileSync(join(directory,"late.ips"),report(5)),60);
  expect((await findCrashReportV1(5,{directory,waitMilliseconds:2000,pollMilliseconds:20}))?.path).toBe(join(directory,"late.ips"));
  expect(await findCrashReportV1(5,{directory:join(directory,"missing"),waitMilliseconds:0})).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a command whose child kills itself with SIGKILL prints the note with its last stderr line before the failure is thrown",async()=>{
 const written:string[]=[],original=process.stderr.write.bind(process.stderr);
 (process.stderr as unknown as {write:(text:unknown,...rest:unknown[])=>boolean}).write=(text:unknown,...rest:unknown[])=>{if(text==="")return (original as (...all:unknown[])=>boolean)(text,...rest);written.push(String(text));return true;};
 let failure:unknown;
 try{await runOwnedCommand("/bin/sh",["-c","echo last words >&2; kill -9 $$"],process.cwd(),"death-probe",20000);}catch(error){failure=error;}finally{(process.stderr as unknown as {write:unknown}).write=original;}
 expect(String(failure)).toContain("death-probe was killed by SIGKILL");
 const note=written.join("").split("\n").filter(line=>line.includes("died from")||line.includes("starb an"));
 expect(note.length).toBe(2);expect(note[0]).toContain("last words");expect(note[1]).toContain("last words");
},60000);

test("Windows has no signals: a fatal NTSTATUS exit code is a death, named, signed or unsigned; a plain failure is not",()=>{
 expect(windowsDeathStatusV1(-1073741819)).toBe("STATUS_ACCESS_VIOLATION");
 expect(windowsDeathStatusV1(3221225477)).toBe("STATUS_ACCESS_VIOLATION");
 expect(windowsDeathStatusV1(0xC0000999)).toBe("NTSTATUS 0xC0000999");
 expect(windowsDeathStatusV1(1)).toBeUndefined();expect(windowsDeathStatusV1(137)).toBeUndefined();expect(windowsDeathStatusV1(null)).toBeUndefined();
 expect(deathCauseV1("SIGKILL",null,"linux")).toBe("SIGKILL");
 expect(deathCauseV1(null,3221225477,"win32")).toBe("STATUS_ACCESS_VIOLATION");
 expect(deathCauseV1(null,3221225477,"linux")).toBeUndefined();
 const oracle=Bun.spawnSync(["python3","-I","-c","import sys;v=int(sys.argv[1]);s=v-2**32 if v>2**31-1 else v;print(-1073741824<=s<=-805306369)","3221225477"]);
 expect(oracle.stdout.toString().trim()).toBe("True");
});

test("a Windows minidump of exactly that pid is the crash report there",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"death-note-"));
 try{
  writeFileSync(join(directory,"bun.exe.4242.dmp"),"MDMP");writeFileSync(join(directory,"bun.exe.4243.dmp"),"MDMP");writeFileSync(join(directory,"bun.exe.4242.txt"),"x");
  expect((await findCrashReportV1(4242,{directory,waitMilliseconds:0,platform:"win32"}))?.path).toBe(join(directory,"bun.exe.4242.dmp"));
  expect(await findCrashReportV1(4244,{directory,waitMilliseconds:0,platform:"win32"})).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
