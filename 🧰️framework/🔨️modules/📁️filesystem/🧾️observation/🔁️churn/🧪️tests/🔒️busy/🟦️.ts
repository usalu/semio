import {expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {churnProtocolV1,filesystemBusyErrorV1,renameOverBusyFileSyncV1,settleBusyFilesystemSyncV1} from "../../🟦️.ts";

const error=(code:string)=>Object.assign(new Error(code),{code});
const plan=churnProtocolV1().profiles.read;

test("the sharing violations are busy on a Windows host only, vanished and unknown codes never", () => {
 for(const code of["EBUSY","EPERM","EACCES"])expect(filesystemBusyErrorV1(error(code),"win32"),code).toBe(true);
 expect(filesystemBusyErrorV1(error("EPERM"),"linux")).toBe(false);expect(filesystemBusyErrorV1(error("EPERM"),"darwin")).toBe(false);
 for(const code of["ENOENT","ENOSPC","EIO"])expect(filesystemBusyErrorV1(error(code),"win32"),code).toBe(false);
 expect(filesystemBusyErrorV1(null,"win32")).toBe(false);expect(filesystemBusyErrorV1({},"win32")).toBe(false);
});

test("a busy file is retried on the declared pauses of the read layer and then settles", () => {
 const pauses:number[]=[];let calls=0;
 const result=settleBusyFilesystemSyncV1("C:\\cache\\unit.lock",()=>{if(++calls<3)throw error("EPERM");return "released";},{platform:"win32",sleepSync:ms=>{pauses.push(ms);}});
 expect(result).toEqual({settled:true,value:"released"});expect(calls).toBe(3);expect(pauses).toEqual(plan.backoffMs.slice(0,2));
});

test("a file that stays busy is returned as a failure after the declared attempts, reported once in every language, never thrown", () => {
 const pauses:number[]=[],reports:string[]=[];let calls=0;
 const result=settleBusyFilesystemSyncV1("C:\\cache\\unit.lock",()=>{calls++;throw error("EBUSY");},{platform:"win32",sleepSync:ms=>{pauses.push(ms);},report:text=>{reports.push(text);},environment:{}});
 expect(result.settled).toBe(false);expect((result as {error:{code:string}}).error.code).toBe("EBUSY");
 expect(calls).toBe(plan.maximumAttempts);expect(pauses).toEqual([...plan.backoffMs]);
 expect(reports.length).toBe(1);expect(reports[0]).toContain("unit.lock");expect(reports[0]!.split("\n").length,"one line per declared language").toBe(churnProtocolV1().languages.length);
});

test("an error that is not busy is not retried and the report stays quiet", () => {
 let calls=0;const reports:string[]=[];
 const result=settleBusyFilesystemSyncV1("/cache/unit.lock",()=>{calls++;throw error("ENOSPC");},{platform:"win32",sleepSync:()=>{throw Error("must not sleep");},report:text=>{reports.push(text);}});
 expect(result.settled).toBe(false);expect(calls).toBe(1);expect(reports).toEqual([]);
});

test.skipIf(process.platform==="win32")("a real permission refusal of this host (a read-only directory) is classified by the platform: busy under win32 rules, final under its own", () => {
 const directory=mkdtempSync(join(tmpdir(),"busy-")),file=join(directory,"claim.lock");writeFileSync(file,"x");chmodSync(directory,0o500);
 try{
  let own=0,pauses=0;
  const native=settleBusyFilesystemSyncV1(file,()=>{own++;unlinkSync(file);},{sleepSync:()=>{pauses++;}});
  if(native.settled)return;
  expect((native as {error:{code:string}}).error.code).toMatch(/^(EACCES|EPERM)$/u);expect(own).toBe(1);expect(pauses).toBe(0);
  let windows=0;const retried=settleBusyFilesystemSyncV1(file,()=>{windows++;unlinkSync(file);},{platform:"win32",sleepSync:()=>{}});
  expect(retried.settled).toBe(false);expect(windows).toBe(plan.maximumAttempts);
 }finally{chmodSync(directory,0o700);rmSync(directory,{recursive:true,force:true});}
});

test("a rename over a target that a reader holds open is retried on the declared pauses and then lands", () => {
 const pauses:number[]=[],moves:string[]=[];let calls=0;
 renameOverBusyFileSyncV1("C:\\graph\\receipt.tmp","C:\\graph\\receipt.json",{platform:"win32",sleepSync:ms=>{pauses.push(ms);},rename:(from,to)=>{moves.push(`${from}>${to}`);if(++calls<3)throw error("EPERM");}});
 expect(calls).toBe(3);expect(pauses).toEqual(plan.backoffMs.slice(0,2));expect(new Set(moves)).toEqual(new Set(["C:\\graph\\receipt.tmp>C:\\graph\\receipt.json"]));
});

test("a rename whose target stays busy throws the last sharing violation after the declared attempts and reports once; other failures are thrown at once and never retried", () => {
 const reports:string[]=[];let calls=0;
 expect(()=>renameOverBusyFileSyncV1("a.tmp","a",{platform:"win32",sleepSync:()=>{},report:text=>{reports.push(text);},environment:{},rename:()=>{calls++;throw error("EBUSY");}})).toThrow("EBUSY");
 expect(calls).toBe(plan.maximumAttempts);expect(reports.length).toBe(1);
 let once=0;
 expect(()=>renameOverBusyFileSyncV1("a.tmp","a",{platform:"win32",sleepSync:()=>{throw Error("must not sleep");},rename:()=>{once++;throw error("ENOENT");}})).toThrow("ENOENT");
 expect(once).toBe(1);
 let posix=0;
 expect(()=>renameOverBusyFileSyncV1("a.tmp","a",{platform:"linux",sleepSync:()=>{throw Error("must not sleep");},rename:()=>{posix++;throw error("EPERM");}})).toThrow("EPERM");
 expect(posix).toBe(1);
});

test("the real rename replaces an existing target atomically and leaves no staging file", () => {
 const directory=mkdtempSync(join(tmpdir(),"rename-busy-")),target=join(directory,"receipt.json"),staged=join(directory,"receipt.json.tmp");
 try{
  writeFileSync(target,"old");writeFileSync(staged,"new");
  renameOverBusyFileSyncV1(staged,target);
  expect(readFileSync(target,"utf8")).toBe("new");expect(existsSync(staged)).toBe(false);
  expect(()=>renameOverBusyFileSyncV1(join(directory,"absent.tmp"),target)).toThrow(/ENOENT/u);expect(readFileSync(target,"utf8")).toBe("new");
 }finally{rmSync(directory,{recursive:true,force:true});}
});
