import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {STALE_IMPLEMENTATION_CODES_V1,concurrentEditOfV1,churnProtocolV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {createNxFailureTailV1,nxFailureChurnV1,settleNxRunV1,NX_FAILURE_TAIL_POLICY_V1} from "../🟦️.ts";
import fixture from "./🔣️.json";

const churn=fileURLToPath(new URL("../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/",import.meta.url));
const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);

test("the tail keeps the last characters of everything the process printed, also across a byte chunk that cuts a character",()=>{
 const tail=createNxFailureTailV1(10);
 tail.push("0123456789");tail.push("abc");expect(tail.text()).toBe("3456789abc");
 const bytes=new TextEncoder().encode("ä🧬x");tail.push(bytes.subarray(0,1));tail.push(bytes.subarray(1,4));tail.push(bytes.subarray(4));expect(tail.text()).toBe("789abcä🧬x".slice(-10));
 const two=createNxFailureTailV1(20),emoji=new TextEncoder().encode("🧬");two.push(emoji.subarray(0,2),"stdout");two.push(new TextEncoder().encode("é").subarray(0,1),"stderr");two.push(emoji.subarray(2),"stdout");two.push(new TextEncoder().encode("é").subarray(1),"stderr");expect(two.text()).toBe("🧬é");
 expect(()=>createNxFailureTailV1(0)).toThrow("build.nx.failure-tail-size-required");
 expect(()=>createNxFailureTailV1(NX_FAILURE_TAIL_POLICY_V1.maximumTailCharacters+1)).toThrow("build.nx.failure-tail-size-required");
 const big=createNxFailureTailV1();big.push("x".repeat(NX_FAILURE_TAIL_POLICY_V1.maximumTailCharacters+5000));expect(big.text().length).toBe(NX_FAILURE_TAIL_POLICY_V1.maximumTailCharacters);
});

test("an Nx transcript is classified like the failed Bazel task it corresponds to: by the refusal class table, and like an independent oracle, in both languages",()=>{
 let oracle:Record<string,{class:string;disposition:string}|null>|undefined;
 if(python){
  const root=fileURLToPath(new URL("../../../../../../../../../../",import.meta.url)),run=spawnSync(python,["-I",churn+"🧪️tests/🧾️refusals/🐍️.py",root,churn+"🧬️schema/🧾️refusals.json",churn+"🧬️schema/🔣️.json",fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);oracle=JSON.parse(run.stdout).texts;
 }
 for(const row of fixture.texts){
  const error=nxFailureChurnV1(row.text,fixture.label);
  if(oracle){expect(oracle[row.id]?.class??null,row.id).toBe(row.class);}
  if(row.disposition==="reexec"){expect(STALE_IMPLEMENTATION_CODES_V1.has((error as Error&{code:string}).code),row.id).toBe(true);expect(error!.message,row.id).toContain(fixture.label);}
  else if(row.disposition==="reobserve"){const edit=concurrentEditOfV1(error);expect(edit,row.id).toMatchObject({reason:"input",path:fixture.label,rank:churnProtocolV1().profiles.build.rank-1});}
  else expect(error,row.id).toBeUndefined();
 }
});

const rerunOf=(results:{code:number;tail:string}[])=>{let calls=0;return{calls:()=>calls,rerun:async()=>{calls++;const next=results.shift();if(!next)throw Error("unexpected rerun");return next;}};};
const options=(over:object={})=>({environment:{LANG:"en_US.UTF-8"},sleep:async()=>undefined,onRetry:()=>undefined,...over});
const text=(id:string)=>fixture.texts.find(row=>row.id===id)!.text;

test("a finished invocation keeps its exit code unless a task names a refusal class: success, plain failure and fatal failure never run again",async()=>{
 for(const [id,code] of [["nx-ordinary-compile-error",1],["nx-plain-type-error",2],["nx-lockfile-frozen",1],["nx-own-custody",1],["nx-no-output",137]] as const){
  const again=rerunOf([]);expect(await settleNxRunV1({code,tail:text(id)},fixture.label,again.rerun,options()),id).toBe(code);expect(again.calls(),id).toBe(0);
 }
 const again=rerunOf([]);expect(await settleNxRunV1({code:0,tail:text("nx-stale-reader")},fixture.label,again.rerun,options())).toBe(0);expect(again.calls()).toBe(0);
});

test("a task that yielded for an outdated implementation ends the invocation with the stale implementation error and nothing runs again",async()=>{
 for(const id of ["nx-moved-stamp-en","nx-moved-stamp-de","nx-stale-reader","nx-stale-observation"]){
  const again=rerunOf([]),failure=await settleNxRunV1({code:1,tail:text(id)},fixture.label,again.rerun,options()).catch(error=>error);
  expect(STALE_IMPLEMENTATION_CODES_V1.has(failure.code),id).toBe(true);expect(again.calls(),id).toBe(0);
 }
});

test("a task whose inputs were edited while it ran makes the whole invocation run again, announced in English and German, until it passes",async()=>{
 for(const [language,fragment] of [["en_US.UTF-8","Taking a new snapshot, attempt 2 of 4"],["de_DE.UTF-8","Neuer Schnappschuss, Versuch 2 von 4"]] as const){
  const announced:string[]=[],again=rerunOf([{code:0,tail:"done"}]);
  expect(await settleNxRunV1({code:1,tail:text("nx-generation-moved")},fixture.label,again.rerun,options({environment:{LANG:language},onRetry:(event:{text:string})=>announced.push(event.text)}))).toBe(0);
  expect(again.calls()).toBe(1);expect(announced.join("\n")).toContain(fragment);
 }
 const second=rerunOf([{code:1,tail:text("nx-bun-manifest")},{code:1,tail:text("nx-plain-type-error")}]);
 expect(await settleNxRunV1({code:1,tail:text("nx-generation-moved")},fixture.label,second.rerun,options())).toBe(1);expect(second.calls()).toBe(2);
});

test("edits that never stop end the retries at the bound of the build layer with the exhausted error, and cancellation ends the pause at once",async()=>{
 const maximum=churnProtocolV1().profiles.build.maximumAttempts,same=()=>({code:1,tail:text("nx-topology")}),again=rerunOf(Array.from({length:maximum},same));
 const failure=await settleNxRunV1(same(),fixture.label,again.rerun,options()).catch(error=>error);
 expect(failure.message).toContain("kept changing in all "+maximum+" attempts");expect(again.calls()).toBe(maximum-1);
 const controller=new AbortController(),waiting=rerunOf([same()]);
 const cancelled=settleNxRunV1(same(),fixture.label,waiting.rerun,options({signal:controller.signal,sleep:(_:number,signal?:AbortSignal)=>new Promise<void>((_accept,reject)=>signal?.addEventListener("abort",()=>reject(signal.reason),{once:true}))})).catch(error=>error);
 await Promise.resolve();controller.abort(Error("stop"));expect((await cancelled).message).toBe("stop");expect(waiting.calls()).toBe(0);
});

test("the Nx lane feeds every chunk of the nx process into the tail and settles a failed invocation through it before it reports the exit code",()=>{
 const source=readFileSync(fileURLToPath(new URL("../../../../../⚡️caching/🚀️bootstrap/📜️script.ts",import.meta.url)),"utf8");
 expect(source).toContain('from "../../🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/🟦️.ts"');
 expect(source).toMatch(/state\.output\.push\(chunk, output === process\.stderr \? "stderr" : "stdout"\);\s*state\.tail/u);
 const settled=source.indexOf("code = await settleNxRunV1("),loop=source.indexOf("graphChanges.get(completed)?.changed"),generation=source.indexOf("if(pendingGeneration!==undefined&&code===0");
 expect(loop).toBeGreaterThan(0);expect(settled).toBeGreaterThan(loop);expect(generation).toBeGreaterThan(settled);
 const block=source.slice(source.lastIndexOf("if (code !== 0",settled),generation);expect(block).toContain("!cancelled && !invocation.watch");expect(block).toContain("signal: actualPhase.signal");
});
