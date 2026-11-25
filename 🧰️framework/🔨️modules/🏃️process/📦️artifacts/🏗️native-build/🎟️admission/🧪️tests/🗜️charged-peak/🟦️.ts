/** 🗜️ Contract: a completed unit is recorded with the larger of its peak resident bytes and its largest charged sample, so a compiler whose pages the OS compressed or swapped is learned at what it was charged with. The pure rule equals an independent Python oracle on every corpus case, and a broker that received charged samples above the resident peak records the charged peak. */
import {spawnSync} from "node:child_process";
import {createConnection} from "node:net";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {startAdmissionBrokerV1,unitPeakBytesV1,type AdmitRequestV1} from "../../🟦️.ts";
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json";

const GiB=1073741824,run=(item:(typeof corpus.cases)[number])=>unitPeakBytesV1(item.peakRssBytes,item.chargedSamples.reduce((peak,sample)=>Math.max(peak,sample),0));

test("every corpus case equals its expected peak and the independent Python oracle",()=>{
 for(const item of corpus.cases)expect(run(item),item.id).toBe(item.expected);
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(JSON.parse(result.stdout)).toEqual(corpus.cases.map(run));
});

test("the broker records a unit at its largest charged sample when that exceeds the resident peak",async()=>{
 const item=corpus.cases.find(row=>row.id==="compressed-final-charged-leads")!,pass={id:"wasm-release-00",profile:"release",target:"wasm32-wasip2"};
 const request:AdmitRequestV1={...(requests.requests.valid[0]! as unknown as AdmitRequestV1),crate:"semio_a",package:"semio-a",crateTypes:["lib"],target:"host",externs:[],pid:1};
 let recorded=-1,completed=()=>{};const ended=new Promise<void>(accept=>{completed=accept;});
 const broker=await startAdmissionBrokerV1({physicalBytes:15*GiB,totalBytes:16*GiB,history:new Map(),available:()=>16*GiB,warden:()=>"calm",cancels:()=>[],onCompleted:unit=>{recorded=unit.peakRssBytes;completed();}});
 broker.registerPass(pass);
 try{
  const socket=createConnection(broker.endpoint);
  await new Promise<void>((accept,reject)=>{socket.on("error",reject);socket.on("connect",()=>{socket.write(JSON.stringify(request)+"\n");});socket.on("data",chunk=>{if(String(chunk).includes('"admitted":true'))accept();});});
  for(const sample of item.chargedSamples)socket.write(JSON.stringify({v:1,op:"rss",bytes:Math.min(sample,item.peakRssBytes),chargedBytes:sample})+"\n");
  socket.write(JSON.stringify({v:1,op:"done",exit:0,peakRssBytes:item.peakRssBytes,userMs:1,sysMs:1,startMs:1,endMs:2,rmetaMs:null})+"\n");
  await ended;
  expect(recorded).toBe(item.expected);
 }finally{await broker.close();}
});
