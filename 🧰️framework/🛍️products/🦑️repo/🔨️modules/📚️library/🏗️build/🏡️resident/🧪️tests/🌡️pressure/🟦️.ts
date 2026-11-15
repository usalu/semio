/** 🌡️ Contract: the host pressure watch and the host headroom read what the machine can still hand out (macOS: free, file-backed and purgeable pages, never more than free, inactive and speculative pages), never the free pages alone, so a healthy macOS host whose kernel keeps its memory as file cache does not cancel a graph slice load; a really scarce host still does. The expected bytes are recomputed by an independent Python parser. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {createHostAvailableBytesV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {createPressureWatchV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs";

const available=(vmStat:string)=>createHostAvailableBytesV1({platform:"darwin",run:()=>vmStat,total:()=>2**40,environment:{}})(),here=dirname(fileURLToPath(import.meta.url)),fixture=JSON.parse(readFileSync(join(here,"🔣️.json"),"utf8")),python=Bun.which("python3")??Bun.which("python");
const independent=(text:string)=>Number(Bun.spawnSync([python!,"-I","-c","import re,sys\nt=sys.stdin.read()\np=lambda n:int(re.search(r'^'+n+r':\\s+(\\d+)\\.',t,re.M).group(1))\nprint(p('Pages free')+max(p('Pages inactive')+p('Pages speculative'),p('File-backed pages')+p('Pages purgeable')))"],{stdin:new TextEncoder().encode(text)}).stdout.toString().trim());

for(const row of fixture.rows)test("the available memory of "+row.name,()=>{
 expect(independent(row.vmStat)).toBe(row.availablePages);
 expect(available(row.vmStat)).toBe(row.availablePages*fixture.pageBytes);
});

test("a Linux host keeps answering MemAvailable and a missing vm_stat answers nothing instead of the free pages",()=>{
 expect(createHostAvailableBytesV1({platform:"linux",read:()=>"MemTotal: 1 kB\nMemAvailable:    2048 kB\n",total:()=>2**40,environment:{}})()).toBe(2048*1024);
 expect(createHostAvailableBytesV1({platform:"darwin",run:()=>{throw Error("no vm_stat");},total:()=>2**40,environment:{},notify:()=>undefined})()).toBeUndefined();
});

for(const [index,row] of fixture.rows.entries())test("the watch over "+row.name+" "+(index===2?"trips":"stays quiet")+" after the sustained seconds",()=>{
 let at=0;const watch=createPressureWatchV1({measures:()=>({freeBytes:available(row.vmStat)}),now:()=>at,policy:{fullAvg10Percent:20,throttleEventsPerSecond:50,freeFloorMegabytes:fixture.floorMegabytes,sustainedSeconds:fixture.sustainedSeconds}});
 let verdict={tripped:false} as {tripped:boolean};for(at=0;at<=(fixture.sustainedSeconds+5)*1000;at+=1000)verdict=watch.sample();
 expect(verdict.tripped).toBe(index===2);
});
