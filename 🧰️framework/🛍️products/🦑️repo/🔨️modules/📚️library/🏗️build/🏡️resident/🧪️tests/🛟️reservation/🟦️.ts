/** 🛟 Contract: a slice load never reserves more memory than the host can hand out and runs with a narrower owner wave instead of waiting for memory that never comes (a 16 GB host whose desktop holds the rest never has 8 GB above the floor: the load waited for ever). The decision is a pure function, checked on a language-agnostic fixture against an independent Python computation. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {sliceAdmissionV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {scaledOwnerWaveV1} from "../../../🕸️graph/🕰️generations/🟨️.mjs";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),fixture=JSON.parse(readFileSync(join(here,"🔣️.json"),"utf8")),python=Bun.which("python3")??Bun.which("python");
const independent=(row:any)=>JSON.parse(Bun.spawnSync([python!,"-I","-c","import json,sys\nd=json.load(sys.stdin)\ne,r,a=d['elasticity'],d['requestedMegabytes'],d['availableMegabytes']\nm=r if a is None else min(r,max(e['minimumReservationMegabytes'],(max(0,a-e['hostFloorMegabytes'])*e['hostShareBasisPoints'])//10000))\nw=max(500,min(10000,(m*10000)//r))\nprint(json.dumps({'megabytes':m,'reduced':m<r,'waveBasisPoints':w,'owners':max(1,d['wave']*w//10000) if w<10000 else d['wave']}))"],{stdin:new TextEncoder().encode(JSON.stringify({...row,elasticity:fixture.elasticity,wave:fixture.wave}))}).stdout.toString());

for(const row of fixture.rows)test(row.name,()=>{
 expect(independent(row)).toEqual(row.expected);
 const found=sliceAdmissionV1({requestedMegabytes:row.requestedMegabytes,availableMegabytes:row.availableMegabytes??undefined,floorMegabytes:fixture.elasticity.hostFloorMegabytes,shareBasisPoints:fixture.elasticity.hostShareBasisPoints,minimumMegabytes:fixture.elasticity.minimumReservationMegabytes});
 expect({megabytes:found.megabytes,reduced:found.reduced,waveBasisPoints:found.waveBasisPoints,owners:scaledOwnerWaveV1(fixture.wave,found.waveBasisPoints)}).toEqual(row.expected);
 expect(found.megabytes).toBeGreaterThan(0);
});

test("the owner wave is never fewer than one and the policy wave without a grant share",()=>{
 expect(scaledOwnerWaveV1(48,undefined)).toBe(48);expect(scaledOwnerWaveV1(48,"10000")).toBe(48);expect(scaledOwnerWaveV1(48,"nonsense")).toBe(48);expect(scaledOwnerWaveV1(48,1)).toBe(1);expect(scaledOwnerWaveV1(48,"5000")).toBe(24);
});

test("a pool on a host that has 5.6 GB reserves the clamped amount, says so in its progress and starts the worker with the reduced wave",async()=>{
 const reserved:number[]=[],progress:any[]=[],waves:string[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused"),elasticity:fixture.elasticity,unlearnedMegabytes:8192,headroom:()=>({limitBytes:Infinity,currentBytes:0,availableBytes:5660*1048576}),reserve:async({megabytes}:{megabytes:number})=>{reserved.push(megabytes);return {megabytes,attach:()=>undefined,release:()=>undefined};},createWorker:(_url:URL,_data:unknown,env:Readonly<Record<string,string|undefined>>)=>{waves.push(env.SEMIO_SLICE_WAVE_BASIS_POINTS!);const listeners:Record<string,(value?:any)=>void>={};return {pid:undefined,on:(name:string,callback:(value?:any)=>void)=>{listeners[name]=callback;if(name==="exit")queueMicrotask(()=>listeners.message?.({type:"done",graph:{}}));},terminate:async()=>undefined};},pressure:()=>({sample:()=>({tripped:false})}),reaper:{reap:async()=>0,killNow:()=>undefined}} as any);
 await pool.run({root:"/w",arguments:[]},{onProgress:(_ms:number,detail:any)=>progress.push(detail)});
 await pool.close();
 expect(reserved).toEqual([3093]);expect(waves).toEqual(["3775"]);expect(progress.some(detail=>detail?.reservation?.granted===3093&&detail.reservation.requested===3093)).toBe(true);
});
