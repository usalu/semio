import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {uptime} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {parseNativeBootEpochV1} from "../../../📋️plan/🟦️.ts";
import {readHostBootEpochMsV1,type NativeBootEpochPortsV1} from "../../🟦️.ts";
import {readWindowsBootEpochNativeV1,windowsBootEpochMsFromTimeOfDayV1} from "../🟦️.ts";
import schema from "../../../🧬️schema/🔣️.json";

const native=schema.definitions.Protocol.const.bootEpoch.win32.native,oracle=join(import.meta.dir,"🐍️oracle","🐍️.py"),python=testPythonV1()??Bun.which("python3")??"python3";
const py=(...args:string[]):any=>JSON.parse(spawnSync(python,["-I",oracle,...args],{encoding:"utf8"}).stdout);
const record=(ticks:bigint,length=48):Buffer=>{const bytes=Buffer.alloc(length);bytes.writeBigInt64LE(ticks,0);bytes.writeBigInt64LE(ticks+10_000_000n,8);return bytes;};
const TICKS=[133_800_000_000_000_000n,134_000_123_456_789n+133_000_000_000_000_000n,133_912_345_678_999_999n,133_912_345_679_000_000n,116_444_736_010_000_000n,150_000_000_000_000_000n];

/** 🪟️ A model of the one `ntdll` call: it validates the information class, the buffer and the pointers, writes the boot record in the layout the oracle computed, and answers the status of the scenario. */
function ntdllModelV1(ticks:bigint,scenario:{status?:(length:number)=>number;throwOnOpen?:boolean}={}){
 const pointers=new Map<number,Buffer>(),calls:number[]=[],state={opened:0,closed:0,pointer:8192};
 const system={
  ptr:(view:ArrayBufferView)=>{state.pointer+=16;pointers.set(state.pointer,Buffer.from(view.buffer,view.byteOffset,view.byteLength));return state.pointer;},
  dlopen:(library:string,symbols:Record<string,unknown>)=>{
   if(scenario.throwOnOpen)throw Error("dlopen failed");expect(library).toBe(native.library);expect(Object.keys(symbols)).toEqual([native.symbol]);expect(symbols[native.symbol]).toEqual({args:["u32","ptr","u32","ptr"],returns:"i32"});state.opened++;
   return{close:()=>{state.closed++;},symbols:{[native.symbol]:(information:number,buffer:number,length:number,returned:number)=>{
    calls.push(length);expect(information).toBe(3);expect(pointers.get(buffer)!.length).toBe(length);expect(pointers.get(returned)!.length).toBe(4);
    const status=scenario.status?.(length)??0;if(status<0)return status;pointers.get(buffer)!.fill(0);record(ticks,Math.min(length,48)).copy(pointers.get(buffer)!);return status;}}};
  },
 };
 return{system,calls,state};
}

test("the record layout and the FILETIME conversion equal the independent oracle (ctypes field list, datetime arithmetic, the PowerShell text of the same instant)",()=>{
 const layout=py("layout");expect(native.lengths[0]).toBe(layout.bytes);expect(native.bootTimeOffset).toBe(layout.bootTime);expect(native.class).toBe(3);expect(native.fileTimeEpochOffsetMs).toBe(11_644_473_600_000);
 const expected=py("convert",...TICKS.map(String)) as {milliseconds:number;text:string}[];
 TICKS.forEach((ticks,index)=>{
  expect(windowsBootEpochMsFromTimeOfDayV1(record(ticks)),String(ticks)).toBe(expected[index]!.milliseconds);
  expect(parseNativeBootEpochV1("win32",expected[index]!.text+"\r\n"),"text path "+expected[index]!.text).toBe(expected[index]!.milliseconds);
 });
 for(const bad of [0n,-5n,1n,10_000_000n])expect(windowsBootEpochMsFromTimeOfDayV1(record(bad)),String(bad)).toBeNull();
 expect(windowsBootEpochMsFromTimeOfDayV1(Buffer.alloc(7))).toBeNull();expect(windowsBootEpochMsFromTimeOfDayV1(record(TICKS[0]!,32))).toBe(expected[0]!.milliseconds);
});

test("the in-process read asks the system once, falls back to the older length, closes the library, and answers null instead of throwing",()=>{
 const good=ntdllModelV1(TICKS[0]!);expect(readWindowsBootEpochNativeV1(good.system as never,"win32")).toBe(py("convert",String(TICKS[0]))[0].milliseconds);expect(good.calls).toEqual([48]);expect([good.state.opened,good.state.closed]).toEqual([1,1]);
 const older=ntdllModelV1(TICKS[0]!,{status:length=>length>32?-1073741820:0});expect(readWindowsBootEpochNativeV1(older.system as never,"win32")).toBe(py("convert",String(TICKS[0]))[0].milliseconds);expect(older.calls).toEqual([48,32]);expect(older.state.closed).toBe(1);
 const refused=ntdllModelV1(TICKS[0]!,{status:()=>-1073741790});expect(readWindowsBootEpochNativeV1(refused.system as never,"win32")).toBeNull();expect(refused.calls).toEqual([48,32]);expect(refused.state.closed).toBe(1);
 expect(readWindowsBootEpochNativeV1(ntdllModelV1(TICKS[0]!,{throwOnOpen:true}).system as never,"win32")).toBeNull();
 const other=ntdllModelV1(TICKS[0]!);expect(readWindowsBootEpochNativeV1(other.system as never,"darwin")).toBeNull();expect(readWindowsBootEpochNativeV1(other.system as never,"linux")).toBeNull();expect(other.state.opened).toBe(0);
 const empty=ntdllModelV1(0n);expect(readWindowsBootEpochNativeV1(empty.system as never,"win32")).toBeNull();
});

test("the clock asks the shell only when the in-process read is unavailable, and every host route keeps its text and parser",async()=>{
 const instant=TICKS[0]!,expected=py("convert",String(instant))[0] as {milliseconds:number;text:string};
 const make=(native:number|null)=>{const log:string[]=[];const ports:NativeBootEpochPortsV1={native:()=>{log.push("native");return native;},command:async(program,args)=>{log.push("command "+program+" "+args.join(" "));return program==="/usr/sbin/sysctl"?"{ sec = 1792504809, usec = 123456 } Tue Oct 20 15:00:09 2026":expected.text;},file:path=>{log.push("file "+path);return "cpu 1 2 3\nbtime 1792504809\n";},powershell:()=>"C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe"};return{ports,log};};
 const direct=make(expected.milliseconds);expect(await readHostBootEpochMsV1("win32",direct.ports)).toBe(expected.milliseconds);expect(direct.log).toEqual(["native"]);
 const shell=make(null);expect(await readHostBootEpochMsV1("win32",shell.ports)).toBe(expected.milliseconds);expect(shell.log).toEqual(["native","command C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe "+schema.definitions.Protocol.const.bootEpoch.win32.arguments.join(" ")]);
 const mac=make(5);expect(await readHostBootEpochMsV1("darwin",mac.ports)).toBe(1792504809123);expect(mac.log.length).toBe(1);expect(mac.log[0]).toContain("/usr/sbin/sysctl");
 const linux=make(5);expect(await readHostBootEpochMsV1("linux",linux.ports)).toBe(1792504809000);expect(linux.log).toEqual(["file /proc/stat"]);
 const broken={...make(null).ports,command:async()=>"garbage"};await expect(readHostBootEpochMsV1("win32",broken)).rejects.toThrow("Exact boot epoch text required");
});

test("on a real Windows host the in-process boot time is the instant PowerShell reports, is plausible, and costs far less than the shell (timings are printed for the record)",()=>{
 if(process.platform!=="win32"){expect(readWindowsBootEpochNativeV1(undefined,process.platform)).toBeNull();return;}
 const started=performance.now(),direct=readWindowsBootEpochNativeV1(),nativeMs=performance.now()-started;
 expect(direct).not.toBeNull();expect(Math.abs(direct!-(Date.now()-uptime()*1000))).toBeLessThan(5000);
 const begun=performance.now(),shell=spawnSync("powershell.exe",schema.definitions.Protocol.const.bootEpoch.win32.arguments,{encoding:"utf8"}),shellMs=performance.now()-begun;
 const viaShell=parseNativeBootEpochV1("win32",shell.stdout);console.log(`[xplat2-g] windows boot clock: in-process ${nativeMs.toFixed(2)} ms, PowerShell ${shellMs.toFixed(0)} ms, difference ${direct!-viaShell!} ms`);
 expect(Math.abs(direct!-viaShell!)).toBeLessThanOrEqual(1);expect(nativeMs).toBeLessThan(shellMs);
},30000);
