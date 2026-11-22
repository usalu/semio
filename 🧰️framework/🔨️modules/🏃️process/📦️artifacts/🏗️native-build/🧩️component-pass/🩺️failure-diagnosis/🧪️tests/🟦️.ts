/** 🩺️ Contract: a unit the host killed is named with its signal and treated as a restart, the diagnostics of every failed unit survive a stream that ends in warnings, and a reader that went away never fails the driver. */
import {EventEmitter} from "node:events";
import {constants} from "node:os";
import {expect,test} from "bun:test";
import {STALE_IMPLEMENTATION_CODES_V1} from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {staleImplementationOfV1} from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import {guardStandardStreamsV1} from "../../../../../../⚙️runtime/🏁️main/🟦️.ts";
import {UNIT_KILLED_CODE_V1,UnitKilledErrorV1,cargoSilentExitV1,createErrorCaptureV1,killedUnitsV1,unitExitSignalV1,type UnitEndV1} from "../🟦️.ts";

const unit=(crate:string,exit:number,peak=1024**3):UnitEndV1=>({pass:"prototype-00",crate,package:crate,crateTypes:["lib"],exit,peakRssBytes:peak});

test("an exit above 128 is the signal that ended the unit; ordinary exits and Windows hosts have none",()=>{
 expect(unitExitSignalV1(128+constants.signals.SIGKILL,"darwin")).toEqual({signal:constants.signals.SIGKILL,name:"SIGKILL"});
 expect(unitExitSignalV1(128+constants.signals.SIGSEGV,"linux")?.name).toBe("SIGSEGV");
 for(const exit of [0,1,101,127,128,193])expect(unitExitSignalV1(exit,"linux"),String(exit)).toBeUndefined();
 expect(unitExitSignalV1(137,"win32")).toBeUndefined();
});

test("a killed unit is named with its signal, exit, peak memory and cause; only SIGKILL is transient",()=>{
 const rows=killedUnitsV1([unit("semio_hub_wfc",137,4*1024**3),unit("semio_crashed",128+constants.signals.SIGSEGV),unit("semio_failed",101),unit("semio_ok",0)],"linux");
 expect(rows.map(row=>[row.unit,row.name,row.transient])).toEqual([["semio_hub_wfc@semio_hub_wfc (lib)","SIGKILL",true],["semio_crashed@semio_crashed (lib)","SIGSEGV",false]]);
 expect(rows[0]!.text).toContain("[prototype-00] unit semio_hub_wfc@semio_hub_wfc (lib) ended by SIGKILL (exit 137)");
 expect(rows[0]!.text).toContain("memory pressure");expect(rows[0]!.text).toContain("capacity-breach kill");expect(rows[0]!.text).toContain("4.00 GiB");
 expect(rows[1]!.text).toContain("the compiler crashed (SIGSEGV)");
});

test("a killed unit ends the process with the restart exit through the declared code",()=>{
 const error=new UnitKilledErrorV1(killedUnitsV1([unit("semio_hub_wfc",137)],"linux"));
 expect(error.code).toBe(UNIT_KILLED_CODE_V1);expect(STALE_IMPLEMENTATION_CODES_V1.has(error.code)).toBe(true);
 expect(staleImplementationOfV1(error)?.origin).toBe("error");
 expect(staleImplementationOfV1(new AggregateError([error],"wrapped"))?.message).toContain("SIGKILL");
 expect(staleImplementationOfV1(Error("compile error"))).toBeUndefined();
});

test("the diagnostics of every failed unit survive a stderr that ends in warnings",()=>{
 const capture=createErrorCaptureV1({blockLines:6,totalLines:20,lineChars:200});
 const stderr=["   Compiling semio-plugin-abi v0.1.0","error: could not write output to /x/y.o: No space left on device","","rustc-LLVM ERROR: IO failure on output stream: No space left on device","warning: `semio-plugin-interaction` (lib) generated 13 warnings","error: could not compile `semio-plugin-interaction` (lib); 13 warnings emitted","","Caused by:","  process didn't exit successfully: `rustc` (exit status: 1)","warning: build failed, waiting for other jobs to finish...","   Compiling semio-hub-wfc v0.1.0","warning: unreachable expression","  --> a.rs:1:1"];
 for(const line of stderr)capture.push(line);
 expect(capture.lines()).toEqual(["error: could not write output to /x/y.o: No space left on device","rustc-LLVM ERROR: IO failure on output stream: No space left on device","error: could not compile `semio-plugin-interaction` (lib); 13 warnings emitted","Caused by:","  process didn't exit successfully: `rustc` (exit status: 1)"]);
});

test("a long line is cut to the declared characters",()=>{
 const capture=createErrorCaptureV1({blockLines:3,totalLines:3,lineChars:10});
 capture.push("error: "+"x".repeat(50));
 expect(capture.lines()).toEqual(["error: xxx…"]);
});

test("an error block is bounded per block and the whole capture per run",()=>{
 const capture=createErrorCaptureV1({blockLines:3,totalLines:5,lineChars:100});
 for(const line of ["error[E0425]: cannot find value","  --> a.rs:1:1","  |","  | 1","  | 2","error: second","  detail","error: third","  detail"])capture.push(line);
 expect(capture.lines()).toEqual(["error[E0425]: cannot find value","  --> a.rs:1:1","  |","error: second","  detail"]);
});

test("the standard stream guard drops EPIPE once per stream and raises any other error",()=>{
 const stream=new EventEmitter() as unknown as Pick<NodeJS.WriteStream,"on"|"listeners">&EventEmitter;
 expect(guardStandardStreamsV1([stream])).toBe(true);expect(guardStandardStreamsV1([stream])).toBe(false);expect(stream.listenerCount("error")).toBe(1);
 expect(()=>stream.emit("error",Object.assign(Error("write EPIPE"),{code:"EPIPE"}))).not.toThrow();
 expect(()=>stream.emit("error",Object.assign(Error("gone"),{code:"ERR_STREAM_DESTROYED"}))).not.toThrow();
 expect(()=>stream.emit("error",Object.assign(Error("disk"),{code:"ENOSPC"}))).toThrow("disk");
});

test("a Cargo exit without any diagnostic is named instead of left silent",()=>{
 const silent={diagnosed:false,unitsStarted:1050};
 expect(cargoSilentExitV1(0,silent,"linux")).toBeUndefined();
 expect(cargoSilentExitV1(101,{...silent,diagnosed:true},"linux")).toBeUndefined();
 expect(cargoSilentExitV1(128+constants.signals.SIGTERM,silent,"linux")).toContain("ended by SIGTERM (exit 143)");
 expect(cargoSilentExitV1(1,silent,"linux")).toContain("exited with 1 without any compiler error or stderr output after 1050 unit(s)");
 expect(cargoSilentExitV1(143,silent,"win32")).toContain("exited with 143");
});
