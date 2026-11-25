import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {offloadCostRecordV1,OFFLOAD_POLICY_V1} from "../../📐️policy/🟦️.ts";
import {readUnitCostV1,writeUnitCostV1} from "../../🚦️admission/🟦️.ts";

type Provenance={wallMilliseconds:number;userMilliseconds:number;systemMilliseconds:number}|null;
const vectors=JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️.json",import.meta.url)),"utf8")) as {policy:unknown;cases:{name:string;before:number|null;ranMilliseconds:number;provenance:Provenance;expected:number|null}[]};
const measured=(row:Provenance)=>row===null?undefined:{wallMs:row.wallMilliseconds,userMs:row.userMilliseconds,sysMs:row.systemMilliseconds};

test("the authored cost policy is the one the vectors are written against",()=>{
 expect(OFFLOAD_POLICY_V1.cost).toEqual(vectors.policy as typeof OFFLOAD_POLICY_V1.cost);
});

test("a recorded local cost takes the admitted run of calm compiles only: no admission wait, no starved raise",()=>{
 for(const row of vectors.cases)expect(offloadCostRecordV1(row.before??undefined,row.ranMilliseconds,measured(row.provenance))??null,row.name).toBe(row.expected);
});

test("the independent python oracle computes the same records",()=>{
 const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{encoding:"utf8",input:JSON.stringify(vectors)});
 expect(oracle.stderr+oracle.stdout).toBe("ok\n");
 expect(oracle.status).toBe(0);
});

test("the record file moves with calm compiles and ignores a starved one",()=>{
 const root=mkdtempSync(join(tmpdir(),"unit-cost-"));
 try{
  writeUnitCostV1(root,"hub",496697,measured({wallMilliseconds:496697,userMilliseconds:480000,systemMilliseconds:16000}));
  expect(readUnitCostV1(root,"hub")).toBe(496697);
  writeUnitCostV1(root,"hub",2822325,measured({wallMilliseconds:2700000,userMilliseconds:500000,systemMilliseconds:100000}));
  expect(readUnitCostV1(root,"hub"),"the memory-starved compile of hub-15 is not averaged in").toBe(496697);
  writeUnitCostV1(root,"fresh",2822325,measured({wallMilliseconds:2700000,userMilliseconds:500000,systemMilliseconds:100000}));
  expect(readUnitCostV1(root,"fresh")).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});
