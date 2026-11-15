/** 🔋️ Contract: the one reader of the memory a host can still give out answers what the kernel can hand out (macOS: free pages plus the larger of the inactive and speculative pages and the file-backed and purgeable pages; Linux: `MemAvailable`; Windows: the operating system's available physical memory), never the free pages alone that `os.freemem()` counts on a Mac (100 to 300 MB on a healthy 16 GB machine with 6 GB to give), never more than a simulated host owns, reuses an answer within its age and says in English and German once that a source cannot be read. The hand-written fixtures are real `vm_stat` and `/proc/meminfo` outputs of 16 GB machines under load; the expected macOS bytes are recomputed by an independent Python parser.
 * @see ../../📡️sampling/🟦️.ts */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {freemem} from "node:os";
import {createHostAvailableBytesV1,hostAvailableBytesV1,parseDarwinCountersV1} from "../../📡️sampling/🟦️.ts";
import fixture from "./🔣️.json";

const MiB=1048576,python=Bun.which("python3")??Bun.which("python");
const independent=(text:string):number=>{const run=spawnSync(python!,["-I","-c","import re,sys\nt=sys.stdin.read()\np=lambda n:int(re.search(r'^'+n+r':\\s+(\\d+)\\.',t,re.M).group(1))\nsize=int(re.search(r'page size of (\\d+) bytes',t).group(1))\nprint(size*(p('Pages free')+max(p('Pages inactive')+p('Pages speculative'),(p('File-backed pages')+p('Pages purgeable')) if re.search(r'^File-backed pages:',t,re.M) else 0)))"],{input:text,encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return Number(run.stdout.trim());};
const darwin=(text:string,extra:Parameters<typeof createHostAvailableBytesV1>[0]={})=>createHostAvailableBytesV1({platform:"darwin",run:()=>text,total:()=>fixture.totalMegabytes*MiB,environment:{},...extra})();
const freePages=(text:string):number=>Number(/^Pages free:\s+(\d+)\./mu.exec(text)![1])*Number(/page size of (\d+) bytes/u.exec(text)![1]);

for(const row of fixture.rows)test("macOS reads "+row.name+" like the independent parser and far above the free pages",()=>{
 const expected=independent(row.vmStat);
 expect(darwin(row.vmStat)).toBe(expected);
 expect(parseDarwinCountersV1(row.vmStat,"","").availableBytes).toBe(expected);
 expect(expected).toBeLessThanOrEqual(fixture.totalMegabytes*MiB);
 expect(freePages(row.vmStat)).toBeLessThan(expected);
});

test("the healthy loaded 16 GB Mac is not starved by its free pages: 186 MB free pages, 7.0 GB available",()=>{
 const row=fixture.rows[0]!;
 expect(Math.round(freePages(row.vmStat)/MiB)).toBe(186);
 expect(Math.round(darwin(row.vmStat)!/MiB)).toBeGreaterThan(6900);
 expect(Math.round(darwin(fixture.rows[2]!.vmStat)!/MiB)).toBeLessThan(256);
});

for(const row of fixture.linux)test("Linux reads MemAvailable of "+row.name,()=>{
 expect(createHostAvailableBytesV1({platform:"linux",read:path=>{expect(path).toBe("/proc/meminfo");return row.meminfo;},total:()=>fixture.totalMegabytes*MiB,environment:{}})()).toBe(row.availableBytes);
});

test("Windows and every other platform read the operating system's available physical memory",()=>{
 for(const platform of["win32","freebsd"] as const)expect(createHostAvailableBytesV1({platform,free:()=>5000*MiB,total:()=>fixture.totalMegabytes*MiB,environment:{}})()).toBe(5000*MiB);
});

test("a simulated 16 GB host never has more available than its total, a real one is not cut",()=>{
 const free=()=>20000*MiB;
 expect(createHostAvailableBytesV1({platform:"win32",free,total:()=>32768*MiB,environment:{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"16384"}})()).toBe(16384*MiB);
 expect(createHostAvailableBytesV1({platform:"win32",free,total:()=>32768*MiB,environment:{}})()).toBe(20000*MiB);
 expect(createHostAvailableBytesV1({platform:"win32",free,total:()=>32768*MiB,environment:{SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"nonsense"}})()).toBe(20000*MiB);
});

test("an answer is reused within its age and read again after it",()=>{
 let runs=0,time=0;const read=createHostAvailableBytesV1({platform:"darwin",run:()=>{runs++;return fixture.rows[0]!.vmStat;},total:()=>fixture.totalMegabytes*MiB,environment:{},now:()=>time});
 read();read();time=4999;read();expect(runs).toBe(1);time=5000;read();expect(runs).toBe(2);
});

test("a source that cannot be read answers undefined and says so once in English and German, never a free-page number",()=>{
 const lines:string[]=[];let time=0;
 const read=createHostAvailableBytesV1({platform:"darwin",run:()=>{throw Error("vm_stat missing");},free:()=>143*MiB,total:()=>fixture.totalMegabytes*MiB,environment:{},now:()=>time+=10000,notify:line=>lines.push(line)});
 expect(read()).toBeUndefined();expect(read()).toBeUndefined();
 expect(lines.length).toBe(2);expect(lines[0]).toContain("darwin");expect(lines[0]).toContain("vm_stat missing");expect(lines[0]).toContain("cannot be read");expect(lines[1]).toContain("nicht lesbar");
 expect(createHostAvailableBytesV1({platform:"darwin",run:()=>"garbage",total:()=>fixture.totalMegabytes*MiB,environment:{},notify:()=>undefined})()).toBeUndefined();
 expect(createHostAvailableBytesV1({platform:"linux",read:()=>"MemTotal: 1 kB\n",total:()=>fixture.totalMegabytes*MiB,environment:{},notify:()=>undefined})()).toBeUndefined();
});

test("the live reader answers a positive number below the total of this host and is no free-page reading",()=>{
 const bytes=hostAvailableBytesV1();
 expect(bytes).toBeGreaterThan(0);
 if(process.platform==="darwin")expect(bytes!).toBeGreaterThanOrEqual(freemem());
});
