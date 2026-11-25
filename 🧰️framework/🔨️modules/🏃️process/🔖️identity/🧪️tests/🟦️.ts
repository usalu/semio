import Ajv from "ajv";
import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {PROCESS_IDENTITY_POLICY_V1 as P,ProcessIdentityRecordErrorV1,requireProcessIdentityV1,ownProcessIdentityV1,processHolderAliveV1,parsePosixProcessStartV1,pinSpawnedProcessV1,processIdentityDeadV1,processIdentityOfV1,processIdentityVerdictV1,readProcessStartV1,type ProcessStartReaderV1} from "../🟦️.ts";
import {readyIdentityV1,readyPinStageRootV1,readyStageProcessVerdictV1} from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🟦️.ts";

/** 🔖️ Contract: a process is named by pid and creation time; a pid that exists with another creation time is another process and its holder is dead (Windows hands a freed pid to the next process within seconds); an unreadable table never frees a holder; a stage process created late by a loaded machine is not mistaken for a reused pid.
 * @see ../🧬️schema/🔣️.json */
const ajv=new Ajv({strict:false}),T=Date.UTC(2026,10,25,17,25,10);
const reader=(value:ReturnType<ProcessStartReaderV1>):ProcessStartReaderV1=>()=>value;

test("the schema validates identities and the policy",()=>{
 const validate=ajv.compile({...schema.definitions.ProcessIdentity,definitions:schema.definitions});
 expect(validate({pid:10,startMs:T})).toBe(true);
 expect(validate({pid:0,startMs:T})).toBe(false);
 expect(validate({pid:10,startMs:0})).toBe(false);
 expect(validate({pid:10,startMs:T,extra:1})).toBe(false);
 expect(P.toleranceMilliseconds).toBeLessThan(P.spawnGraceMilliseconds);
});

test("pid reuse: the same pid with another creation time is dead, the same creation time lives, gone is dead, unreadable never frees",()=>{
 const recorded={pid:24028,startMs:T};
 expect(processIdentityDeadV1(recorded,reader(T))).toBe(false);
 expect(processIdentityDeadV1(recorded,reader(T+P.toleranceMilliseconds))).toBe(false);
 expect(processIdentityDeadV1(recorded,reader(T+17_000))).toBe(true);
 expect(processIdentityDeadV1(recorded,reader("gone"))).toBe(true);
 expect(processIdentityDeadV1(recorded,reader("unknown"))).toBe(false);
 expect(processIdentityVerdictV1(recorded,T+60_000)).toBe("reused");
 expect(()=>processIdentityVerdictV1({pid:1,startMs:-1},T)).toThrow(ProcessIdentityRecordErrorV1);
 expect(processIdentityVerdictV1({pid:1} as never,T)).toBe("same");
 expect(processIdentityVerdictV1({pid:1} as never,"gone")).toBe("gone");
 expect(requireProcessIdentityV1({pid:7},"x")).toEqual({pid:7,startMs:0});
 expect(()=>requireProcessIdentityV1({pid:7,startMs:"1"},"x")).toThrow(ProcessIdentityRecordErrorV1);
 expect(()=>requireProcessIdentityV1({startMs:T},"x")).toThrow(ProcessIdentityRecordErrorV1);
 expect(()=>requireProcessIdentityV1({pid:1.5,startMs:T},"x")).toThrow();
 expect(requireProcessIdentityV1({pid:7,startMs:T},"x")).toEqual({pid:7,startMs:T});
});

test("the Windows slow spawn of the m05 judgement: a stage process created 17 s after the clock read is pinned, not judged a reused pid, and a later process on the pid is",()=>{
 const flight={pid:24028,startedMs:T},slow={pid:24028,ppid:1,startMs:T+16_550},later={pid:24028,ppid:1,startMs:T+3_600_000};
 expect(readyIdentityV1({pid:24028,startMs:T},[slow])).toBe("reused");
 const pinned=readyPinStageRootV1(flight,[slow]);
 expect(pinned).toEqual({pid:24028,startMs:T+16_550});
 expect(readyIdentityV1(pinned,[slow])).toBe("same");
 expect(readyIdentityV1(pinned,[later])).toBe("reused");
 expect(readyIdentityV1(pinned,[])).toBe("gone");
 expect(readyPinStageRootV1(flight,[later])).toEqual({pid:24028,startMs:T});
 expect(readyPinStageRootV1(flight,[],pinned)).toEqual(pinned);
 expect(readyPinStageRootV1(flight,[],{pid:24028,startMs:T-3_600_000})).toEqual({pid:24028,startMs:T});
 expect(pinSpawnedProcessV1({pid:1,spawnedMs:T},T-P.toleranceMilliseconds-1)).toBeUndefined();
});

test("adoption: the process now at the pid of a stage in flight is judged by the pinned start or the spawn window, and an unreadable start never ends a stage",()=>{
 const flight={pid:24028,startedMs:T},pinned={pid:24028,startMs:T+16_550};
 expect(readyStageProcessVerdictV1(flight,pinned,T+16_550)).toBe("same");
 expect(readyStageProcessVerdictV1(flight,pinned,T+7_200_000)).toBe("reused");
 expect(readyStageProcessVerdictV1(flight,undefined,T+16_550)).toBe("same");
 expect(readyStageProcessVerdictV1(flight,undefined,T+7_200_000)).toBe("reused");
 expect(readyStageProcessVerdictV1(flight,undefined,null)).toBe("same");
});

test("the POSIX start text parses to the calendar instant an independent Python computes",()=>{
 const text="Wed Nov 25 17:25:10 2026",expected=Number(spawnSync("python3",["-I","-c","import calendar,time;print(calendar.timegm(time.strptime('Wed Nov 25 17:25:10 2026','%a %b %d %H:%M:%S %Y'))*1000)"],{encoding:"utf8"}).stdout.trim());
 expect(parsePosixProcessStartV1(text)).toBe(expected);
 expect(parsePosixProcessStartV1(" Sat Jan  3 00:00:01 2026 ")).toBe(Date.UTC(2026,0,3,0,0,1));
 expect(parsePosixProcessStartV1("garbage")).toBeNull();
});

test.skipIf(process.platform==="win32")("a real child: its creation time equals what an independent `ps -o etime` says, and after it ends the pid is gone",async()=>{
 const child=spawn(process.execPath,["-e","setTimeout(()=>{},30000)"],{stdio:"ignore"});
 try{
  await new Promise(done=>setTimeout(done,300));
  const identity=processIdentityOfV1(child.pid!);
  expect(identity).toBeDefined();
  const etime=spawnSync("ps",["-o","etime=","-p",String(child.pid)],{encoding:"utf8"}).stdout.trim(),found=/^(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)$/u.exec(etime)!;
  const seconds=((Number(found[1]??0)*24+Number(found[2]??0))*60+Number(found[3]))*60+Number(found[4]);
  expect(Math.abs(Date.now()-seconds*1000-identity!.startMs)).toBeLessThan(3000);
  expect(processIdentityDeadV1(identity!)).toBe(false);
  expect(processIdentityDeadV1({pid:identity!.pid,startMs:identity!.startMs-3_600_000})).toBe(true);
 }finally{child.kill("SIGKILL");}
 await new Promise(done=>child.once("exit",done));
 expect(readProcessStartV1(child.pid!)).toBe("gone");
});

test("a holder record is judged by pid and creation time: own identity lives, a wrong start is a reused pid, a record from before startMs was recorded is judged by its pid alone, a malformed start is a hard error",()=>{
 const me=ownProcessIdentityV1();
 expect(me.pid).toBe(process.pid);
 expect(processHolderAliveV1(me)).toBe(true);
 expect(processHolderAliveV1({pid:me.pid,startMs:me.startMs-3_600_000})).toBe(false);
 expect(processHolderAliveV1({pid:me.pid,startMs:me.startMs},()=>"unknown")).toBe(true);
 expect(processHolderAliveV1({pid:2_000_000_000,startMs:T})).toBe(false);
 expect(processHolderAliveV1({pid:me.pid} as never)).toBe(true);
 expect(processHolderAliveV1({pid:2_000_000_000} as never)).toBe(false);
 expect(()=>processHolderAliveV1({pid:me.pid,startMs:"x"} as never)).toThrow(ProcessIdentityRecordErrorV1);
});
