/** 🧭️ Contract: the cold estimate of a third-party crate is calibrated from the ledger (it covers every measured peak and overshoots none by more than the stated factor, equal to an independent Python formula), and an external crate without measurement under its own profile is predicted from the rows of its other profile even when they carry no recorded size.
 * @see ../../🟦️.ts (`predictPeakBytesV1`, `admissionSiblingHistoryV1`, `externalSourceV1`) */
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createConnection} from "node:net";
import {admissionSiblingHistoryV1,externalSourceV1,predictPeakBytesV1,startAdmissionBrokerV1,type AdmitRequestV1,type UnitObservationV1} from "../../🟦️.ts";
import requests from "../🧫️fixtures/🔣️.json" with {type:"json"};
import corpus from "./🔣️.json";

const MiB=1<<20,megabytes=(lines:number)=>predictPeakBytesV1([],{kloc:lines/1000,component:null,external:true,budgetBytes:corpus.budgetMiB*MiB});

test("the external cold model equals the independent formula, covers every measured peak and overshoots no crate of 250 MiB or more beyond the limit",()=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url)),fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(corpus.measured.map(row=>megabytes(row.lines))).toEqual(JSON.parse(result.stdout));
 for(const row of corpus.measured){const predicted=megabytes(row.lines)/MiB;expect(predicted,row.crate).toBeGreaterThanOrEqual(row.peakMiB);if(row.peakMiB>=250)expect(predicted/row.peakMiB,row.crate).toBeLessThanOrEqual(corpus.overestimateLimit);}
});

test("a registry, git or vendored source is external, a workspace source is not",()=>{
 expect(externalSourceV1("/home/u/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/syn-2.0.0/src/lib.rs")).toBe(true);
 expect(externalSourceV1("C:\\Users\\u\\.cargo\\git\\checkouts\\x\\abc\\src\\lib.rs")).toBe(true);
 expect(externalSourceV1("/repo/🧰️framework/vendor/x/src/lib.rs")).toBe(true);
 expect(externalSourceV1("/repo/🧰️framework/🔨️modules/x/🦀️.rs")).toBe(false);
});

test("sibling rows without a recorded size are evidence for an external crate only",()=>{
 for(const item of corpus.sibling){
  const history=new Map<string,UnitObservationV1[]>();
  for(const row of item.rows)history.set(row.key,[...history.get(row.key)??[],{v:1,key:row.key,peakRssBytes:row.peakMiB*MiB,outputBytes:0,wallMs:1,cpuMs:1,remainingMs:0,atMs:1,..."lines" in row?{lines:row.lines}:{}}]);
  const peaks=admissionSiblingHistoryV1(item.key,item.lines,history,undefined,item.external).map(row=>row.peakRssBytes/MiB).sort((left,right)=>left-right);
  expect(peaks,item.name).toEqual(item.expectedPeaksMiB);
 }
});

test("the cold estimate of syn fits the 0.6 GiB cargo grant that wedged it as a workspace-sized guess",()=>{
 expect(megabytes(51000)/MiB).toBeLessThan(0.6*1024);
 expect(predictPeakBytesV1([],{kloc:51,component:null})/MiB).toBeGreaterThan(2500);
});

test("a cold syn-sized registry source is admitted under a 0.6 GiB grant at once, with no growth wait and no refusal",async()=>{
 const root=mkdtempSync(join(tmpdir(),"cold-syn-")),manifestDir=join(root,"registry","src","index","syn-2.0.0"),source=join(manifestDir,"src","lib.rs");
 mkdirSync(join(manifestDir,"src"),{recursive:true});
 writeFileSync(source,"pub fn f() {}\n".repeat(51000));
 const unit:AdmitRequestV1={...(requests.requests.valid[0]! as unknown as AdmitRequestV1),pass:"check-00",crate:"syn",package:"syn",crateTypes:["lib"],target:"host",externs:[],pid:1,source,manifestDir};
 const broker=await startAdmissionBrokerV1({physicalBytes:16*1024*MiB,history:new Map(),parentMemoryAllowanceBytes:Math.floor(0.6*1024*MiB),available:()=>undefined,warden:()=>"calm",cancels:()=>[]});
 broker.registerPass({id:"check-00",profile:"prototype",target:"host"});
 const socket=createConnection(broker.endpoint);
 try{
  const reply=await new Promise<Record<string,unknown>>((accept,reject)=>{socket.once("error",reject);socket.once("data",chunk=>accept(JSON.parse(String(chunk).split("\n")[0]!)));socket.write(JSON.stringify(unit)+"\n");});
  expect(reply).toMatchObject({op:"verdict",admitted:true});
 }finally{socket.destroy();await broker.close();rmSync(root,{recursive:true,force:true});}
});
