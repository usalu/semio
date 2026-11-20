import {constants} from "node:os";
import {churnProtocolV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** 🏷️ The error code of a build unit that the host killed under it; the churn protocol lists it with the codes that end the process with the restart exit code (75), so the outermost command runs the pass again. */
export const UNIT_KILLED_CODE_V1="SEMIO_UNIT_KILLED";

/** 🪓️ One compile unit whose process ended by signal: the wrapper reports `128 + signal` as its exit, which is all Cargo gets to see, so Cargo prints no compiler diagnostic for it. `transient` marks the kill a fresh run cures (`SIGKILL`: the host killed it, the usual cause is memory pressure); any other signal is a crash of the compiler itself and stays a hard failure. */
export type UnitKillV1=Readonly<{unit:string;pass:string;exit:number;signal:number;name:string;peakRssBytes:number;transient:boolean;text:string}>;

/** 📋️ What the driver knows about one finished unit: the broker's record. */
export type UnitEndV1=Readonly<{pass:string;crate:string;package:string;crateTypes:readonly string[];exit:number;peakRssBytes:number}>;

const signalNames:ReadonlyMap<number,string>=new Map(Object.entries(constants.signals).map(([name,number])=>[number,name]));
const largestSignalV1=64,sigkillV1=constants.signals.SIGKILL;

/** 🔎️ The signal behind a unit's exit, or `undefined` when it exited on its own: shells and the unit wrapper report `128 + signal`; a Windows host has no signals, so its exit codes are never read that way. */
export function unitExitSignalV1(exit:number,platform:string=process.platform):Readonly<{signal:number;name:string}>|undefined{
 if(platform==="win32"||!Number.isInteger(exit)||exit<=128||exit>128+largestSignalV1)return;
 const signal=exit-128;return Object.freeze({signal,name:signalNames.get(signal)??`signal ${signal}`});
}

/** 🩺️ The units that were killed by a signal, each with the line that names it, the signal, its peak resident memory and the cause when it is known. A `SIGKILL` arrives from outside the compiler: the Linux out-of-memory killer, the macOS jetsam daemon or a capacity guard cancelling a unit at its limit all end a unit this way, and none of them leaves a message, so the text names those candidates together with the peak the unit reached before it was gone. */
export function killedUnitsV1(units:readonly UnitEndV1[],platform:string=process.platform):readonly UnitKillV1[]{
 const rows:UnitKillV1[]=[];
 for(const unit of units){
  const ended=unitExitSignalV1(unit.exit,platform);if(!ended)continue;
  const transient=ended.signal===sigkillV1,name=`${unit.package}@${unit.crate} (${unit.crateTypes.join(",")})`,peak=`${(unit.peakRssBytes/1024**3).toFixed(2)} GiB`;
  const cause=transient?`killed from outside the compiler, which is memory pressure (out-of-memory kill, jetsam) or a capacity-breach kill; the unit had reached ${peak} resident`:`the compiler crashed (${ended.name}); the unit had reached ${peak} resident`;
  rows.push(Object.freeze({unit:name,pass:unit.pass,exit:unit.exit,signal:ended.signal,name:ended.name,peakRssBytes:unit.peakRssBytes,transient,text:`[${unit.pass}] unit ${name} ended by ${ended.name} (exit ${unit.exit}): ${cause}`}));
 }
 return Object.freeze(rows);
}

/** 🔁️ A run lost units to the host, not to their sources: it carries the churn protocol's own code for such a failure, so the outermost command starts the pass again with the same exit code as for a peer's edit (75) and says why. */
export class UnitKilledErrorV1 extends Error{
 readonly code:string=UNIT_KILLED_CODE_V1;readonly units:readonly UnitKillV1[];
 constructor(units:readonly UnitKillV1[]){
  super(`${units.length} compile unit(s) were killed by the host and the pass restarts: ${units.map(row=>row.text).join("; ")}`);
  this.name="UnitKilledErrorV1";this.units=units;
  if(!churnProtocolV1().staleImplementationCodes.includes(UNIT_KILLED_CODE_V1))throw Error("Declared restart code of a killed unit required");
 }
}

/** 🧲️ The limits of the error capture: lines kept per error block, lines kept overall and characters kept per line. */
export type ErrorCaptureLimitsV1=Readonly<{blockLines:number;totalLines:number;lineChars:number}>;
/** 🧲️ Keeps the diagnostics of every failed unit out of Cargo's stderr; see {@link createErrorCaptureV1}. */
export type ErrorCaptureV1=Readonly<{push:(line:string)=>void;lines:()=>readonly string[]}>;

const blockStartV1=/^(error(\[[A-Za-z0-9]+\])?:|rustc-LLVM ERROR)/u,blockEndV1=/^(warning\b|\s*(Compiling|Checking|Fresh|Downloaded|Downloading|Finished|Blocking|Building)\b)/u;

/** 🧲️ Cargo and compiler wrappers can report process failures directly on stderr; a stream tail ending in warnings loses them. This capture keeps every block that starts at an `error` line (a rustc error, an LLVM error, Cargo's `could not compile` with its `Caused by`) until the next warning or progress line, bounded per block and overall, so the diagnostics of every failed unit survive whatever the stream does afterwards. */
export function createErrorCaptureV1(limits:ErrorCaptureLimitsV1):ErrorCaptureV1{
 const kept:string[]=[];let inBlock=false,inBlockLines=0;
 return Object.freeze({
  push:(line:string):void=>{
   if(blockStartV1.test(line)){inBlock=true;inBlockLines=0;}
   else if(blockEndV1.test(line))inBlock=false;
   if(!inBlock||line.trim()===""||inBlockLines>=limits.blockLines||kept.length>=limits.totalLines)return;
   inBlockLines++;kept.push(line.length>limits.lineChars?`${line.slice(0,limits.lineChars)}…`:line);
  },
  lines:()=>Object.freeze([...kept]),
 });
}
