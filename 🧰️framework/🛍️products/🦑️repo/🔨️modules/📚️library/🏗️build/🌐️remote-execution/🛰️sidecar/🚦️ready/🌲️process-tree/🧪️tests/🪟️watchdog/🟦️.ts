import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {readyReadTableV1,readyWindowsTreeSampleV1,type ReadyWindowsWatchdogSampleV1} from "../../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

test("the readiness CIM observation credits verified kernel and user CPU with exact birth, and retains bounded memory evidence",async()=>{
 let previous:ReadyWindowsWatchdogSampleV1|undefined,calls=0;
 const results=[];
 for(const row of fixture.samples){
  const abort=new AbortController();
  const table=await readyReadTableV1(1000,"win32",()=>1794000000000+(row.elapsedMilliseconds??0),async (program,args)=>{
   calls++;
   expect(program).toBe("powershell.exe");
   expect(args.at(-1)?.match(/Get-CimInstance/g)).toHaveLength(1);
   for(const metric of ["CreationDate","KernelModeTime","UserModeTime","WorkingSetSize"])expect(args.at(-1)).toContain(metric);
   if(row.cancelled)abort.abort();
   if(row.rows===null)return null;
   return row.rows.map(process=>process.join("\t")).join("\r\n");
  },abort.signal);
  const sample=readyWindowsTreeSampleV1({table,root:fixture.root,members:table.filter(process=>fixture.members.includes(process.pid)),previous,maximum:row.maximum??fixture.bound,minimumCpuMilliseconds:fixture.cpuMilliseconds});
  results.push(sample===undefined?null:{progress:sample.progress,megabytes:sample.megabytes,cpu:sample.checkpoint.cpu.length});
  if(sample!==undefined)previous=sample.checkpoint;
 }
 expect(calls).toBe(fixture.samples.length);
 expect(results).toEqual(fixture.samples.map(row=>row.expected));
 const oracle=spawnSync(process.platform==="win32"?"python":"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(oracle.status).toBe(0);expect(results).toEqual(JSON.parse(oracle.stdout));
});

test("non-Windows process observations retain the existing ps shape and add no Windows evidence",async()=>{
 const table=await readyReadTableV1(1000,"linux",()=>1794000000010,async(program,args)=>{
  expect(program).toBe("ps");expect(args).toEqual(["-A","-o","pid=,ppid=,etime=,comm="]);return "100 5 00:00 /bin/repo\n";
 });
 expect(table).toEqual([{pid:100,ppid:5,startMs:1794000000010,name:"repo"}]);
 expect(readyWindowsTreeSampleV1({table,root:100,members:table,maximum:512,minimumCpuMilliseconds:500})).toBeUndefined();
});

test.skipIf(process.platform!=="win32")("real Windows kernel and user counters advance for the running process",async()=>{
 const first=(await readyReadTableV1()).find(row=>row.pid===process.pid);
 expect(first?.windows?.workingSetBytes).toBeGreaterThan(0);
 const starting=process.cpuUsage();
 while(process.cpuUsage(starting).user+process.cpuUsage(starting).system<800000){Math.sqrt(performance.now());}
 const second=(await readyReadTableV1()).find(row=>row.pid===process.pid);
 expect(second?.windows?.birth).toBe(first?.windows?.birth);
 expect(second!.windows!.cpu!.kernel+second!.windows!.cpu!.user-first!.windows!.cpu!.kernel-first!.windows!.cpu!.user).toBeGreaterThan(5_000_000);
},60000);
