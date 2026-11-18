/** 🧹️ Contract: the reaper of a slice pool never kills a graph service (or anything below it) that is not the pool's own, even when a command a worker ran started that service and so it carries the load token and sits in the process tree of the load: a service outlives the load that happened to start it, while the children of the load, the pool's own service and unrelated processes are treated as before.
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🧹️reap/🟨️.mjs */
import {expect,test} from "bun:test";
import {createReaperV1,listProcessesV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧹️reap/🟨️.mjs";

const sent=(table:any[],scope:any)=>{const killed:number[]=[];createReaperV1({list:()=>table,send:(pid:number)=>{killed.push(pid);}}).killNow(scope);return killed.sort((a,b)=>a-b);};

test("a service started under a worker survives the reaping of that worker, the worker's other children do not",()=>{
 const me=process.pid,table=[
  {pid:1,ppid:0},{pid:me,ppid:900},{pid:900,ppid:1,service:true},
  {pid:100,ppid:me,token:"7.1.aaaa"},{pid:101,ppid:100,token:"7.1.aaaa"},
  {pid:200,ppid:101,token:"7.1.aaaa",service:true},{pid:201,ppid:200,token:"7.1.aaaa"},{pid:202,ppid:201},
  {pid:300,ppid:1},
 ];
 expect(sent(table,{pids:[100]})).toEqual([100,101]);
 expect(sent(table,{match:(row:any)=>row.token==="7.1.aaaa"})).toEqual([100,101]);
});

test("the service the pool belongs to is not foreign: its workers are reaped",()=>{
 const me=process.pid,table=[{pid:1,ppid:0},{pid:900,ppid:1,service:true},{pid:me,ppid:900},{pid:100,ppid:me,token:"7.1.aaaa"}];
 expect(sent(table,{pids:[100]})).toEqual([100]);
});

test("the process table of this host marks a launcher by its command line and no ordinary process",()=>{
 const rows=listProcessesV1();
 expect(rows.length).toBeGreaterThan(1);
 expect(rows.every((row:any)=>row.service===undefined||row.service===true)).toBe(true);
 expect(rows.find((row:any)=>row.pid===process.pid)?.service).toBeUndefined();
});
