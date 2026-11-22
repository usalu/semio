/** 🪟️ Contract: the free bytes of a build worker's volume come from the one free-space question, so a Windows worker is asked through the volume call and a runtime that answers a row of zeros leaves the slots unknown, not at zero. */
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{afterAll,expect,test}from"bun:test";
import{remoteExecutionDiskSlotsV1,remoteExecutionFreeBytesV1}from"../../🟦️.ts";

const directories:string[]=[];afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const GiB=2**30;

test("a Windows worker is asked for its volume's free bytes, the nearest existing parent answers, and a zero row or a failing call is unknown",()=>{
 const base=mkdtempSync(join(tmpdir(),"agent-disk-"));directories.push(base);
 const asked:string[]=[];
 expect(remoteExecutionFreeBytesV1(join(base,"a","b"),{platform:"win32",windowsAvailableBytes:path=>{asked.push(path);return 40*GiB;}})).toBe(40*GiB);
 expect(asked).toEqual([base]);
 expect(remoteExecutionFreeBytesV1(base,{read:()=>({bsize:4096,blocks:0,bavail:0,files:0,ffree:0})})).toBeUndefined();
 expect(remoteExecutionFreeBytesV1(base,{platform:"win32",windowsAvailableBytes:()=>{throw Error("Win32 error 3");}})).toBeUndefined();
 expect(remoteExecutionFreeBytesV1(base,{read:()=>({bsize:4096,blocks:1000,bavail:0})})).toBe(0);
 const real=remoteExecutionFreeBytesV1(base);expect(real).toBeGreaterThan(0);
});

test("slots follow the measured bytes, and an unknown volume is not a volume with no room",()=>{
 const disk={reserveGigabytes:4,hostReserveGigabytes:0,perActionGigabytes:2};
 expect(remoteExecutionDiskSlotsV1({freeBytes:undefined,concurrency:4,disk}).freeBytes).toBeUndefined();
 expect(remoteExecutionDiskSlotsV1({freeBytes:20*GiB,concurrency:4,disk}).slots).toBeGreaterThan(0);
});
