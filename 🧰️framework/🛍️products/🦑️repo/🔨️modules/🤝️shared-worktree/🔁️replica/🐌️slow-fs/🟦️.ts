import {lstatSync,type Stats} from "node:fs";
import {lstat} from "node:fs/promises";

/** 🐌️ What one `lstat` answered: the stats, or the error (`ENOENT` and the like stay values, never throws). */
export type StatResultV1=Stats|NodeJS.ErrnoException;

/** 🔌️ The filesystem calls and the clock of a stat budget; tests replace them with a slow filesystem. */
export type StatBudgetOptionsV1=Readonly<{lstatSync?:(path:string)=>Stats;lstat?:(path:string)=>Promise<Stats>;now?:()=>number;slowMilliseconds?:number;asyncChunk?:number;probeEvery?:number}>;

/** 🔍️ Whether a result is stats and not an error. */
export const isStatsV1=(result:StatResultV1):result is Stats=>!(result instanceof Error);

/** 🐌️ Reads `lstat` for a slice of paths without ever holding the event loop for more than a few stats. While the filesystem is fast (a stat costs less than `slowMilliseconds`, 1 ms, as a moving average) it stats synchronously, which is the cheapest on macOS and Linux; as soon as stats get slow (Windows with a virus scanner, a network share) it moves to parallel asynchronous stats on the libuv pool in chunks, so the loop serves timers, signals and heartbeats between chunks, and it probes the speed again every `probeEvery` chunks to return to the cheap synchronous way. `chunk(budgetMilliseconds)` says how many paths fit in a slice, `many` stats them all.
 * @see ../../../../🔨️modules/🤝️shared-worktree/🛡️resilience/⏳️yielding/🟦️.ts */
export class StatBudgetV1{
 private readonly syncStat:(path:string)=>Stats;
 private readonly asyncStat:(path:string)=>Promise<Stats>;
 private readonly now:()=>number;
 private readonly slow:number;
 private readonly width:number;
 private readonly probe:number;
 private average=0.01;
 private parallel=false;
 private chunks=0;
 private switches=0;
 constructor(options:StatBudgetOptionsV1={}){
  this.syncStat=options.lstatSync??(path=>lstatSync(path));
  this.asyncStat=options.lstat??(path=>lstat(path));
  this.now=options.now??(()=>performance.now());
  this.slow=options.slowMilliseconds??1;this.width=options.asyncChunk??64;this.probe=options.probeEvery??32;
 }
 /** 🧮️ Whether the stats currently run asynchronously, and how often the mode changed. */
 mode():Readonly<{parallel:boolean;averageMs:number;switches:number}>{return {parallel:this.parallel,averageMs:this.average,switches:this.switches};}
 /** 📏️ How many paths to take for the next slice of `budgetMilliseconds`. */
 chunk(budgetMilliseconds:number):number{
  return this.parallel?this.width:Math.max(1,Math.min(512,Math.floor(budgetMilliseconds/Math.max(this.average,0.002))));
 }
 private one(path:string):StatResultV1{
  try{return this.syncStat(path);}catch(error){return error as NodeJS.ErrnoException;}
 }
 private async two(paths:readonly string[]):Promise<StatResultV1[]>{
  return Promise.all(paths.map(path=>this.asyncStat(path).catch((error:unknown)=>error as NodeJS.ErrnoException)));
 }
 /** 📡️ Stats every path, in order. */
 async many(paths:readonly string[]):Promise<StatResultV1[]>{
  if(this.parallel){
   if(++this.chunks%this.probe===0&&paths.length>0){
    const started=this.now(),first=this.one(paths[0]!),took=this.now()-started;
    this.average=took;
    if(took<this.slow/4){this.parallel=false;this.switches++;const rest=await this.many(paths.slice(1));return [first,...rest];}
    return [first,...await this.two(paths.slice(1))];
   }
   return this.two(paths);
  }
  const results:StatResultV1[]=[];
  for(let index=0;index<paths.length;index++){
   const started=this.now(),result=this.one(paths[index]!);
   this.average=this.average*0.5+(this.now()-started)*0.5;
   results.push(result);
   if(this.average>this.slow){
    this.parallel=true;this.switches++;this.chunks=0;
    results.push(...await this.two(paths.slice(index+1)));
    break;
   }
  }
  return results;
 }
}
