import {test,expect} from "bun:test";
import Ajv from "ajv";
import {createRequire} from "node:module";
import {join} from "node:path";
import {captureOwnedCommandResultV1,readOwnedCommandCaptureV1,writeOwnedCommandOutputV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
const sdk=process.env.SEMIO_NX_TOOLING_ROOT;if(!sdk)throw Error("Actual installed third-party oracle root required");const crossSpawn=createRequire(join(sdk,"package.json"))("cross-spawn") as {sync(command:string,args:readonly string[],options:Readonly<{encoding:"utf8"}>):{status:number|null;signal:string|null;stdout:string;stderr:string}};
const control=(milliseconds=10000)=>{const started=performance.now();return{signal:new AbortController().signal,remainingMilliseconds:()=>milliseconds-(performance.now()-started),publish:async()=>{},yieldContinuation:async()=>{}};};
test("owned separated capture agrees with portable vectors, Ajv and CrossSpawn",async()=>{
 const validate=new Ajv({strict:false}).compile(schema);
 for(const row of fixture.cases){const source=`const out=Buffer.from(${JSON.stringify(row.stdout)});for(const byte of out)process.stdout.write(Buffer.from([byte]));process.stderr.write(${JSON.stringify(row.stderr)});process.exitCode=${row.status};`,result=await captureOwnedCommandResultV1(process.execPath,["-e",source],process.cwd(),{control:control(),output:{maximumBytes:1024,maximumLines:16}}),independent=crossSpawn.sync(process.execPath,["-e",source],{encoding:"utf8"});expect(validate(result),row.name).toBe(true);expect(result,row.name).toEqual({version:1,stdout:row.stdout,stderr:row.stderr,status:row.status,signal:null});expect([result.stdout,result.stderr,result.status]).toEqual([independent.stdout,independent.stderr,independent.status]);}
 for(const row of fixture.refusals)await expect(captureOwnedCommandResultV1(process.execPath,["-e",row.source],process.cwd(),{control:control(),output:{maximumBytes:row.bytes,maximumLines:row.lines}})).rejects.toThrow();
 console.log("[DEBUG] Portable separated capture agrees with actual CrossSpawn bytes/status and Ajv; byte, line and UTF8 refusals held");
});
test("owned capture refuses exhausted Original and preserves cancellation, progress and native signal",async()=>{
 await expect(captureOwnedCommandResultV1(process.execPath,["-e","throw Error('must not run')"],process.cwd(),{control:control(0),output:{maximumBytes:64,maximumLines:4}})).rejects.toThrow();
 const cancel=new AbortController(),original={...control(),signal:cancel.signal};setTimeout(()=>cancel.abort(Error("original cancellation")),40);await expect(captureOwnedCommandResultV1(process.execPath,["-e","setInterval(()=>{},1000)"],process.cwd(),{control:original,output:{maximumBytes:64,maximumLines:4}})).rejects.toThrow();
 if(process.platform!=="win32"){const result=await captureOwnedCommandResultV1(process.execPath,["-e","process.kill(process.pid,'SIGTERM')"],process.cwd(),{control:control(),output:{maximumBytes:64,maximumLines:4}});expect(result.status).toBeNull();expect(result.signal).toBe("SIGTERM");}
 let progress=0;await captureOwnedCommandResultV1(process.execPath,["-e","setTimeout(()=>{},1100)"],process.cwd(),{control:{...control(),publish:async()=>{await Bun.sleep(10);progress++;}},output:{maximumBytes:64,maximumLines:4}});expect(progress).toBeGreaterThan(0);
});

test("separated native result admits complete platform exit integers and retains elapsed Original refusal",async()=>{
 const validate=new Ajv({strict:false}).compile(schema);
 for(const status of fixture.nativeStatuses){const row={version:1 as const,stdout:"",stderr:"",status,signal:null};expect(validate(row)).toBe(true);expect(readOwnedCommandCaptureV1(row)).toEqual(row);}
 for(const status of fixture.invalidNativeStatuses){const row={version:1 as const,stdout:"",stderr:"",status,signal:null};expect(validate(row)).toBe(false);expect(()=>readOwnedCommandCaptureV1(row)).toThrow();}
 const original=control(fixture.delayedOriginal.milliseconds);await Bun.sleep(fixture.delayedOriginal.receiveDelayMilliseconds);await expect(captureOwnedCommandResultV1(process.execPath,["-e","throw Error('must not spawn')"],process.cwd(),{control:original,output:{maximumBytes:64,maximumLines:4}})).rejects.toThrow();
 await expect(captureOwnedCommandResultV1(process.execPath,["-e","setTimeout(()=>{},2000)"],process.cwd(),{control:{...control(),publish:async()=>{throw Error("original publication refused");}},output:{maximumBytes:64,maximumLines:4}})).rejects.toThrow("original publication refused");
});

test("owned captured payload delivery preserves bytes and the same Original cancellation/deadline",async()=>{const {PassThrough}=await import("node:stream");for(const row of fixture.cases){const sink=new PassThrough(),chunks:Buffer[]=[];sink.on("data",chunk=>chunks.push(Buffer.from(chunk)));await writeOwnedCommandOutputV1(sink,row.stdout,control());expect(Buffer.concat(chunks).toString("utf8")).toBe(row.stdout);sink.destroy();}const cancel=new AbortController();setTimeout(()=>cancel.abort(Error("original sink cancellation")),10);await expect(writeOwnedCommandOutputV1({write:()=>false},"pending",{...control(),signal:cancel.signal})).rejects.toThrow("original sink cancellation");await expect(writeOwnedCommandOutputV1({write:()=>false},"pending",control(10))).rejects.toThrow("deadline");});
