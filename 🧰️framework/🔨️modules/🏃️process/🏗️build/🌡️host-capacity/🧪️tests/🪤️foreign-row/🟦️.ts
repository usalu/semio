/** 🪤️ Contract: a grant row of another format, or one that does not validate, is isolated as the held reservation of its owner and never refuses the whole ledger; the format identity moves with the grant definition.
 * @see ./🧫️fixtures/🔣️.json
 * @see ../../📏️decision/🟦️.ts */
import {createHash} from "node:crypto";
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {describeForeignHostGrantV1,formatHostCapacityV1,foreignHostGrantV1,hostGrantFormatV1,isCurrentFormatHostGrantV1,readHostCapacityPolicyV1,readHostCapacityStatusV1,type HostSampleV1} from "../../🟦️.ts";
import {hostBootAtV1,prepare,readGrants} from "../../🎫️gate/📖️status/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const python=testPythonV1()!,oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const model=(value:unknown):any=>{const done=Bun.spawnSync([python,"-I",oracle,JSON.stringify(value)]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout));};
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"fr-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,staleMilliseconds:60000}});
const protocol=schema.definitions.Protocol.const;
const host:HostSampleV1={version:1,atMs:0,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10};
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};

const sidecar=(item:{format:unknown})=>item.format===null?undefined:{format:item.format};
const pack=()=>({current:fixture.current,fallbackMegabytes:fixture.fallbackMegabytes,rows:fixture.rows.map(item=>({row:item.row,format:item.format}))});
const sorted=(value:unknown):unknown=>Array.isArray(value)?value.map(sorted):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).filter(([key])=>key!=="description").sort(([a],[b])=>a<b?-1:1).map(([key,item])=>[key,sorted(item)])):value;

test("the fixture and the shipped protocol name the same current format",()=>{
 expect(fixture.current).toBe(protocol.grantFormat);
 expect(fixture.fallbackMegabytes).toBe(protocol.foreignGrantFallbackMegabytes);
});

test("a change of the grant definition raises the format identity",()=>{
 const{Grant,Handover,Phase,GrantFormat}=schema.definitions,fingerprint=createHash("sha256").update(JSON.stringify(sorted({Grant,Handover,Phase,GrantFormat}))).digest("hex");
 const known=(fixture.fingerprints as Record<string,string>)[String(protocol.grantFormat)];
 expect(known,`Grant, Handover, Phase or GrantFormat changed: raise Protocol.grantFormat and record the new fingerprint ${fingerprint} for it in the fixture`).toBe(fingerprint);
});

test("the stand-in of every foreign row equals the independent model",()=>{
 const expected=model(pack());
 fixture.rows.forEach((item,index)=>{
  const format=hostGrantFormatV1(sidecar(item)),row=foreignHostGrantV1({id:"fixturerow1",value:item.row,format,modifiedAtMs:5}),want=expected.rows[index];
  expect(isCurrentFormatHostGrantV1(item.row,format),item.name).toBe(false);
  expect({relation:row.foreign!.relation,megabytes:row.reservedMegabytes,declared:row.foreign!.declared},item.name).toEqual(want);
  expect({relation:item.relation,megabytes:item.megabytes,declared:item.declared},item.name).toEqual(want);
  expect(row.observedMegabytes,item.name).toBeNull();
 });
 expect(expected.held).toBe(fixture.rows.reduce((sum,item)=>sum+item.megabytes,0));
});

test("a live foreign row beside a current one is counted, never refuses the ledger and `repo capacity status` lists it with its holder",async()=>{
 const directory=prepare(fresh()),now=Date.now(),boot=hostBootAtV1(directory,now),grants=join(directory,"grants");
 mkdirSync(grants,{recursive:true});
 fixture.rows.forEach((item,index)=>{const id=`foreignrow${index}`;if(item.format!==null)writeFileSync(join(grants,`${id}.format`),JSON.stringify({format:item.format}));writeFileSync(join(grants,`${id}.json`),JSON.stringify(Array.isArray(item.row)?item.row:{...item.row,id,pid:process.pid,bootAtMs:boot,admittedAtMs:now}));});
 writeFileSync(join(grants,"currentrow0.format"),JSON.stringify({format:protocol.grantFormat}));writeFileSync(join(grants,"currentrow0.json"),JSON.stringify({id:"currentrow0",operation:"run",jobs:1,cpu:1,reservedMegabytes:1000,admittedAtMs:now,observedMegabytes:null,pid:process.pid,bootAtMs:boot}));
 const rows=readGrants(directory,now,policy,undefined,undefined,false);
 expect(rows).toHaveLength(fixture.rows.length+1);
 expect(rows.filter(row=>row.foreign===undefined).map(row=>row.id)).toEqual(["currentrow0"]);
 expect(rows.reduce((sum,row)=>sum+row.reservedMegabytes,0)).toBe(1000+model(pack()).held);
 const status=await readHostCapacityStatusV1({directory,policy,control,sample:async()=>({host:{...host,atMs:Date.now()},swappedOutMegabytes:0})});
 expect(status.grants).toHaveLength(rows.length);
 const inline=status.grants.find(row=>row.id==="foreignrow0")!;
 for(const language of ["en","de"] as const){
  const line=formatHostCapacityV1(language,"grantForeign",{id:inline.id,pid:inline.pid,reserved:inline.reservedMegabytes,seconds:1,...describeForeignHostGrantV1(language,inline.foreign!)});
  expect(line).toContain(`pid ${process.pid}`);
  expect(line).toContain(language==="en"?"older format":"älteres Format");
 }
 expect(formatHostCapacityV1("en","foreignGrant",{id:"a",pid:7,reserved:1,...describeForeignHostGrantV1("en",foreignHostGrantV1({id:"a",value:{},format:2,modifiedAtMs:1}).foreign!)})).toContain("newer format 2 (this reader: 1)");
});

test("a foreign row is released when its owner ends and a row without an owner when its heartbeat is stale",()=>{
 const directory=prepare(fresh()),now=Date.now(),boot=hostBootAtV1(directory,now),grants=join(directory,"grants"),dead=join(grants,"deadowner001.json"),orphan=join(grants,"ownerless001.json");
 writeFileSync(dead,JSON.stringify({...fixture.rows[0]!.row,id:"deadowner001",pid:2147483647,bootAtMs:boot,admittedAtMs:now}));
 writeFileSync(orphan,"{");
 expect(readGrants(directory,now,policy).map(row=>row.id)).toEqual(["ownerless001"]);
 expect(existsSync(dead)).toBe(false);
 utimesSync(orphan,new Date(now-policy.wait.staleMilliseconds-1000),new Date(now-policy.wait.staleMilliseconds-1000));
 expect(readGrants(directory,now,policy)).toEqual([]);
 expect(existsSync(orphan)).toBe(false);
});
