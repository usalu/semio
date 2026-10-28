import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import {createNativeWindowsHostPortV1,nativeWindowsExtendedPathV1,nativeWindowsUtf16V1,readNativeWindowsLongPathsEnabledV1,requireNativeWindowsCaseStatusV1,type NativeWindowsBindingV1} from "../🟦️.ts";
import {nativeDirectoryProtocolV1} from "../../📋️contract/🟦️.ts";

const protocol=nativeDirectoryProtocolV1(),windows=protocol.exchange.windows,python=testPythonV1();
const py=(code:string,...args:string[])=>{if(!python)return undefined;const run=Bun.spawnSync([python,"-I","-c",code,...args],{stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(run.stderr.toString());return run.stdout.toString().trim();};
const recorder=(script:{handle?:bigint;moveOk?:boolean;flushOk?:boolean;closeOk?:boolean;freeOk?:boolean;free?:number;status?:number;error?:number}={})=>{
 const calls:unknown[][]=[];
 const binding:NativeWindowsBindingV1={
  moveFileEx:(from,to,flags)=>{calls.push(["MoveFileExW",from.toString("hex"),to.toString("hex"),flags]);return script.moveOk===false?0:1;},
  createFile:(path,access,share,disposition,flags)=>{calls.push(["CreateFileW",path.toString("hex"),access,share,disposition,flags]);return script.handle??5n;},
  flushFileBuffers:handle=>{calls.push(["FlushFileBuffers",handle]);return script.flushOk===false?0:1;},
  closeHandle:handle=>{calls.push(["CloseHandle",handle]);return script.closeOk===false?0:1;},
  setCaseInformation:(handle,status,information,bytes,informationClass)=>{calls.push(["NtSetInformationFile",handle,status.length,information.toString("hex"),bytes,informationClass]);return script.status??0;},
  getDiskFreeSpaceEx:(path,available)=>{calls.push(["GetDiskFreeSpaceExW",path.toString("hex")]);available.writeBigUInt64LE(BigInt(script.free??0));return script.freeOk===false?0:1;},
  lastError:()=>script.error??0,
  close:()=>{}
 };
 return{binding,calls};
};

test("UTF-16 encoding of wide strings agrees with Python's utf-16-le",()=>{
 for(const text of["C:\\a","\\\\?\\C:\\ünï\\コード","SYSTEM\\CurrentControlSet\\Control\\FileSystem","LongPathsEnabled","😀"]){
  const expected=py("import sys;sys.stdout.write((sys.argv[1]+chr(0)).encode('utf-16-le').hex())",text);
  expect(nativeWindowsUtf16V1(text).toString("hex"),text).toBe(expected??Buffer.from(text+"\0","utf16le").toString("hex"));
 }
});

test("extended paths agree with the declared vectors and Python's ntpath canonical form",()=>{
 for(const row of fixture.windowsPaths){
  expect(nativeWindowsExtendedPathV1(row.path),row.id).toBe(row.extended);
  const canonical=py("import ntpath,sys;sys.stdout.write('1' if ntpath.normpath(sys.argv[1])==sys.argv[1] and ntpath.isabs(sys.argv[1]) else '0')",row.path.replace(/^\\\\\?\\/u,"").replace(/^UNC\\/u,"\\\\"));
  if(canonical!==undefined&&row.id!=="drive-root")expect(canonical,row.id).toBe("1");
 }
 for(const path of fixture.windowsRefusedPaths)expect(()=>nativeWindowsExtendedPathV1(path),JSON.stringify(path)).toThrow();
 expect(()=>nativeWindowsExtendedPathV1("C:\\"+"a".repeat(windows.maximumExtendedUnits))).toThrow("extended-length limit");
});

test("moveFile passes the extended UTF-16 paths and the write-through flag and reports the last error",()=>{
 const{binding,calls}=recorder(),host=createNativeWindowsHostPortV1(binding),from="C:\\cache\\b\\o\\m",to="C:\\cache\\b\\o\\m.old";
 expect(host.moveFile(from,to)).toEqual({ok:true,error:0});
 expect(calls).toEqual([["MoveFileExW",Buffer.from("\\\\?\\"+from+"\0","utf16le").toString("hex"),Buffer.from("\\\\?\\"+to+"\0","utf16le").toString("hex"),8]]);
 const hex=py("import sys;print(('\\\\\\\\?\\\\'+sys.argv[1]+chr(0)).encode('utf-16-le').hex())",from);if(hex!==undefined)expect(calls[0]![1]).toBe(hex);
 const refused=createNativeWindowsHostPortV1(recorder({moveOk:false,error:32}).binding);expect(refused.moveFile(from,to)).toEqual({ok:false,error:32});
 expect(()=>host.moveFile("relative\\a",to)).toThrow("Canonical absolute Windows path");
});

test("flushDirectory opens with backup semantics, flushes, closes, and reports each failure separately",()=>{
 const directory="C:\\cache\\b\\o",access=windows.createFile.genericRead+windows.createFile.genericWrite,flags=windows.createFile.backupSemantics+windows.createFile.openReparsePoint;
 const ok=recorder(),host=createNativeWindowsHostPortV1(ok.binding);expect(host.flushDirectory(directory)).toEqual({ok:true,error:0});
 expect(ok.calls).toEqual([["CreateFileW",Buffer.from("\\\\?\\"+directory+"\0","utf16le").toString("hex"),access,7,3,flags],["FlushFileBuffers",5n],["CloseHandle",5n]]);
 expect(access).toBe(0xC0000000);expect(flags).toBe(0x02200000);
 const opened=recorder({handle:BigInt(windows.invalidHandle),error:5});expect(createNativeWindowsHostPortV1(opened.binding).flushDirectory(directory)).toEqual({ok:false,error:5});expect(opened.calls.map(call=>call[0])).toEqual(["CreateFileW"]);
 const zero=recorder({handle:0n,error:2});expect(createNativeWindowsHostPortV1(zero.binding).flushDirectory(directory)).toEqual({ok:false,error:2});
 const flush=recorder({flushOk:false,error:1});expect(createNativeWindowsHostPortV1(flush.binding).flushDirectory(directory)).toEqual({ok:false,error:1});expect(flush.calls.map(call=>call[0])).toEqual(["CreateFileW","FlushFileBuffers","CloseHandle"]);
 const close=recorder({closeOk:false,error:6});expect(createNativeWindowsHostPortV1(close.binding).flushDirectory(directory)).toEqual({ok:false,error:6});
});

test("setCaseSensitive writes the 4-byte FILE_CASE_SENSITIVE_INFORMATION flag with class 71 on a write-attributes handle",()=>{
 const{binding,calls}=recorder({status:0}),result=createNativeWindowsHostPortV1(binding).setCaseSensitive("C:\\cache\\b\\o\\v");
 expect(result).toEqual({opened:true,error:0,status:0});
 expect(calls).toEqual([["CreateFileW",Buffer.from("\\\\?\\C:\\cache\\b\\o\\v\0","utf16le").toString("hex"),0x100,7,3,0x02000000],["NtSetInformationFile",5n,16,"01000000",4,71],["CloseHandle",5n]]);
 const hex=py("import struct,sys;sys.stdout.write(struct.pack('<I',1).hex())");if(hex!==undefined)expect(calls[1]![3]).toBe(hex);
 const denied=recorder({handle:BigInt(windows.invalidHandle),error:5});expect(createNativeWindowsHostPortV1(denied.binding).setCaseSensitive("C:\\x")).toEqual({opened:false,error:5,status:0});
 const negative=recorder({status:-1073741637});expect(createNativeWindowsHostPortV1(negative.binding).setCaseSensitive("C:\\x").status).toBe(3221225659);
});

test("availableBytes reads the caller's free bytes through GetDiskFreeSpaceExW and openExclusive probes with share mode zero",()=>{
 const wide=(text:string)=>Buffer.from(text+"\0","utf16le").toString("hex"),free=recorder({free:123456789}),host=createNativeWindowsHostPortV1(free.binding);
 expect(host.availableBytes("C:\\cache")).toEqual({ok:true,error:0,bytes:123456789});expect(free.calls).toEqual([["GetDiskFreeSpaceExW",wide("\\\\?\\C:\\cache")]]);
 expect(createNativeWindowsHostPortV1(recorder({freeOk:false,error:3}).binding).availableBytes("C:\\cache")).toEqual({ok:false,error:3,bytes:0});
 const probe=recorder(),open=createNativeWindowsHostPortV1(probe.binding).openExclusive("C:\\cache\\o\\m");
 expect(open).toEqual({ok:true,error:0});expect(probe.calls).toEqual([["CreateFileW",wide("\\\\?\\C:\\cache\\o\\m"),windows.createFile.fileReadAttributes,0,3,windows.createFile.backupSemantics],["CloseHandle",5n]]);
 expect(windows.createFile.fileReadAttributes).toBe(0x80);
 const busy=recorder({handle:BigInt(windows.invalidHandle),error:32});expect(createNativeWindowsHostPortV1(busy.binding).openExclusive("C:\\cache")).toEqual({ok:false,error:32});
});

test("every NTSTATUS of the case-sensitivity call maps to its declared refusal",()=>{
 for(const row of fixture.caseStatuses){
  const run=()=>requireNativeWindowsCaseStatusV1({opened:true,error:0,status:row.status});
  if(row.refusal===null)run();else if(row.refusal==="generic")expect(run,row.id).toThrow("NTSTATUS 0x"+row.status.toString(16));else expect(run,row.id).toThrow((protocol.refusals as Record<string,string>)[row.refusal]!);
 }
 expect(()=>requireNativeWindowsCaseStatusV1({opened:false,error:5,status:0})).toThrow(protocol.refusals.caseAccess);
 expect(()=>requireNativeWindowsCaseStatusV1({opened:false,error:3,status:0})).toThrow("Win32 error 3");
 expect(protocol.refusals.caseFeature).toContain("Microsoft-Windows-Subsystem-Linux");
});

test("LongPathsEnabled is read with the declared hive, key, value and flags and an absent value means disabled",()=>{
 for(const row of fixture.registry){
  const seen:unknown[][]=[],call=(hive:bigint,key:Buffer,value:Buffer,flags:number,data:Buffer,size:Buffer)=>{seen.push([hive,key.toString("hex"),value.toString("hex"),flags,size.readUInt32LE(0)]);data.writeUInt32LE(row.data,0);size.writeUInt32LE(row.size,0);return row.status;};
  if(row.result==="refuse")expect(()=>readNativeWindowsLongPathsEnabledV1(call),row.id).toThrow("registry read failed");else expect(readNativeWindowsLongPathsEnabledV1(call),row.id).toBe(row.result as boolean);
  expect(seen,row.id).toEqual([[BigInt(windows.longPaths.hive),Buffer.from(windows.longPaths.key+"\0","utf16le").toString("hex"),Buffer.from(windows.longPaths.value+"\0","utf16le").toString("hex"),16,4]]);
 }
 expect(windows.longPaths.hive>>>0).toBe(0x80000002);
});

test.skipIf(process.platform==="win32")("the real FFI binding is refused off Windows with a precise message",()=>{
 expect(()=>readNativeWindowsLongPathsEnabledV1()).toThrow("Actual Windows Bun registry receiver required");
});

test("the Windows census substitute refuses a busy directory with the declared message and passes a free one",async()=>{
 const{requireNativeDirectoryUnusedV1}=await import("../../🟦️.ts"),seen:string[]=[],host={moveFile:()=>({ok:true,error:0}),flushDirectory:()=>({ok:true,error:0}),setCaseSensitive:()=>({opened:true,error:0,status:0}),availableBytes:()=>({ok:true,error:0,bytes:0}),openExclusive:(path:string)=>{seen.push(path);return path.endsWith("busy")?{ok:false,error:32}:path.endsWith("odd")?{ok:false,error:5}:{ok:true,error:0};}};
 requireNativeDirectoryUnusedV1(["C:\\a","C:\\b"],host);expect(seen).toEqual(["C:\\a","C:\\b"]);
 expect(()=>requireNativeDirectoryUnusedV1(["C:\\a","C:\\busy","C:\\never"],host)).toThrow(protocol.refusals.windowsInUse);expect(seen).not.toContain("C:\\never");
 expect(()=>requireNativeDirectoryUnusedV1(["C:\\odd"],host)).toThrow("Win32 error 5");
});
