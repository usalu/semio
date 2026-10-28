/** 🖥️ Contract: the terminal, desktop, remote-login and IDE session variables of every supported host (macOS, Linux, devcontainer, Windows) are never an input of a plan, whatever their case; build inputs such as PATH, the temporary directory variables and the hosting-environment switches still are. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {isAmbientEnvironmentNameV1,readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessPreloadV1} from "../../🟨️.mjs";
import fixture from "./🧫️fixtures/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),schemaPath=join(here,"..","..","🧬️schema","🔣️.json"),oracle=join(here,"🐍️oracle","🐍️.py"),python=process.platform==="win32"?"python":"python3";
const watchdog=join(here,"..","..","..","..","..","..","..","..","..","🧰️framework","🔨️modules","🏃️process","🧭️routing","📥️invocation","🏃️process","🐕️watchdog","🟦️.ts");
const signal=new AbortController().signal;
const ask=(names:string[])=>{const run=spawnSync(python,["-I",oracle,schemaPath],{input:JSON.stringify(names),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as boolean[];};
function witnessOf(script:string,env:Record<string,string>){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"witness-session-"))),root=join(base,"root");
 mkdirSync(root,{recursive:true});mkdirSync(join(base,"ws"));
 try{
  writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());writeFileSync(join(base,"scenario.mjs"),script);
  const result=Bun.spawnSync([process.execPath,"--preload",join(base,"preload.mjs"),join(base,"scenario.mjs"),root],{cwd:root,env:{...process.env,...worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace:join(base,"ws"),writable:[join(base,"ws")],racyMilliseconds:0}),...env},stdout:"pipe",stderr:"pipe"});
  if(result.exitCode!==0)throw Error(result.stderr.toString());
  return readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8"));
 }finally{rmSync(base,{recursive:true,force:true});}
}

test("the ambient rule equals an independent Python reading for every session variable of every host, in any case, and for every build input",()=>{
 const names=[...fixture.volatile.map(row=>row.name),...fixture.inputs],variants=names.flatMap(name=>[name,name.toLowerCase(),name.toUpperCase()]),expected=ask(variants);
 variants.forEach((name,index)=>expect(isAmbientEnvironmentNameV1(name),name).toBe(expected[index]!));
 for(const row of fixture.volatile)expect(isAmbientEnvironmentNameV1(row.name),row.name+" ("+row.platform+")").toBe(true);
 for(const name of fixture.inputs)expect(isAmbientEnvironmentNameV1(name),name).toBe(false);
});

test("a session variable of any host leaves no row and no enumeration entry, and its change never breaks the witness while a build input still does",async()=>{
 const session=Object.fromEntries(fixture.volatile.map((row,index)=>[row.name,"session-"+index])),given={...session,SEMIO_FIXTURE_VALUE:"a"};
 const witness=witnessOf(`const copy={...process.env};for(const name of Object.keys(copy))process.env[name];globalThis.__semioWorldWitnessV1.finish();`,given);
 const names=new Set(witness.observations.filter(row=>row[0]==="e").map(row=>row[1])),enumeration=witness.observations.find(row=>row[0]==="E")![2];
 for(const name of Object.keys(session)){expect(names.has(name),name).toBe(false);expect(enumeration.includes(name+"="),name).toBe(false);}
 expect(enumeration.includes("SEMIO_FIXTURE_VALUE=")).toBe(true);
 const now={...process.env,...given};
 expect((await verifyWorldWitnessV1(witness,{signal,environment:now})).holds).toBe(true);
 const moved=Object.fromEntries(Object.keys(session).map((name,index)=>[name,index%2?undefined:"changed-"+index]));
 expect((await verifyWorldWitnessV1(witness,{signal,environment:{...now,...moved}})).holds).toBe(true);
 expect((await verifyWorldWitnessV1(witness,{signal,environment:{...now,SEMIO_FIXTURE_VALUE:"b"}})).holds).toBe(false);
});

test("the process table probe of the stall watchdog on Windows (PowerShell) is a declared host probe, as `ps` is elsewhere: it taints no witness",()=>{
 const script=`Object.defineProperty(process,"platform",{value:"win32"});const {liveChildPidsV1}=await import(${JSON.stringify(watchdog)});liveChildPidsV1();globalThis.__semioWorldWitnessV1.finish();`;
 const witness=witnessOf(script,{});
 expect(witness.tainted.filter(reason=>reason.includes("powershell"))).toEqual([]);
});
