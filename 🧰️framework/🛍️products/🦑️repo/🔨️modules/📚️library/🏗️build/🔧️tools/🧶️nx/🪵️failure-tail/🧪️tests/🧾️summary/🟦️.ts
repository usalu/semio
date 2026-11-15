/** 🧾️ Contract: when an Nx run ends in failure the parent says, per failed task, why it failed and whether another rerun of the whole run is likely to help (a peer edited its inputs, the build implementation is outdated, or the task itself failed), lists the tasks that did not run, and does so in the declared languages; an independent Python oracle reads the same transcripts to the same rows; a failure that stands is reported once, and a rerun that succeeds is silent. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {NX_FAILURE_SUMMARY_POLICY_V1 as policy,nxFailureSummaryLinesV1,nxFailureSummaryOfTailV1,settleNxRunReportedV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0),churn=fileURLToPath(new URL("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🧾️refusals.json",import.meta.url));
const oracle=python?JSON.parse(spawnSync(python,["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),churn,fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8",maxBuffer:1<<24}).stdout) as Record<string,unknown>:undefined;
const run=(code:number,tail:string)=>({code,tail});

test("the summary schema is closed: bilingual texts with the same placeholders, three verdicts, bounded numbers",()=>{
 expect(new Ajv({strict:true}).validateSchema(schema)).toBe(true);
 const placeholders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort().join(",");
 expect(Object.keys(policy.texts).sort()).toEqual(["de","en"]);
 for(const key of Object.keys(policy.texts.en)){const en=(policy.texts.en as Record<string,string>)[key]!,de=(policy.texts.de as Record<string,string>)[key]!;expect(de,key).toBeDefined();expect(placeholders(en),key).toBe(placeholders(de));expect(en===de,key).toBe(key==="task");}
 for(const verdict of ["inputs","implementation","no"])expect((policy.texts.en as Record<string,string>)[verdict]).toBeDefined();
 expect(policy.maximumTasks).toBeGreaterThan(0);expect(policy.maximumReasonCharacters).toBeGreaterThan(40);
});

test("a transcript is read to the failed tasks, the reason of each and the verdict of the refusal class table, exactly like the independent oracle",()=>{
 for(const row of fixture.cases){
  const own=nxFailureSummaryOfTailV1(row.tail);
  expect(own??null,row.id).toEqual(row.expect??null);
  if(oracle)expect(oracle[row.id]??null,row.id).toEqual(row.expect??null);
 }
});

test("the lines name the tasks with their reason and verdict in both languages, count the tasks beyond the bound, and list the tasks that did not run",()=>{
 const summary=nxFailureSummaryOfTailV1(fixture.cases[0]!.tail)!,lines=nxFailureSummaryLinesV1(summary,2,{});
 expect(lines.filter(line=>line.startsWith("Nx run failed: 6 task(s) did not succeed after 2 rerun(s)"))).toHaveLength(1);
 expect(lines.filter(line=>line.startsWith("Nx-Lauf fehlgeschlagen: 6 Aufgabe(n) gelang(en) nach 2 Wiederholung(en)"))).toHaveLength(1);
 expect(lines.some(line=>line.startsWith("- workspace:deps-cargo:prototype: RepoError: stale build.closure.source-differs")&&line.endsWith("(another rerun is likely to help: the command starts again with the current build implementation)"))).toBe(true);
 expect(lines.some(line=>line.startsWith("- workspace:deps-tools:prototype:")&&line.endsWith("(eine weitere Wiederholung hilft wahrscheinlich: ein anderer Prozess bearbeitete Dateien, die die Aufgabe liest)"))).toBe(true);
 expect(lines.some(line=>line.startsWith("- @semio-tech/repo-lib:cpp-setup:prototype: error: Single-link")&&line.endsWith("(another rerun is unlikely to help: the task itself failed)"))).toBe(true);
 expect(lines.some(line=>line==="Not run because a task they depend on failed (2): workspace:setup:prototype, workspace:deps-cpp:prototype")).toBe(true);
 const many={failed:Array.from({length:policy.maximumTasks+3},(_,index)=>({task:"t"+index,reason:"error: x",rerun:"no" as const})),notRun:[]};
 const text=nxFailureSummaryLinesV1(many,1,{});expect(text.filter(line=>line.startsWith("- t"))).toHaveLength(policy.maximumTasks*2);expect(text.some(line=>line==="- and 3 more failed task(s)")).toBe(true);
});

test("a failure that stands is reported once after its reruns, a classified failure that was rerun is reported once with the count of reruns, and a success is silent",async()=>{
 const tail=fixture.cases[0]!.tail,written:string[][]=[];
 const options={environment:{},sleep:async()=>undefined,onRetry:()=>undefined};
 const code=await settleNxRunReportedV1(run(1,"\n> nx run a:b\nerror: nope\n\nFailed tasks:\n\n- a:b\n"),"label",async()=>{throw Error("no rerun");},options,line=>{written.push([line]);});
 expect(code).toBe(1);expect(written.length).toBeGreaterThan(1);expect(written.some(row=>row[0]!.startsWith("- a:b: error: nope"))).toBe(true);
 const quiet:string[]=[];expect(await settleNxRunReportedV1(run(0,tail),"label",async()=>run(0,""),options,line=>quiet.push(line))).toBe(0);expect(quiet).toEqual([]);
 const noFooter:string[]=[];expect(await settleNxRunReportedV1(run(1,"error: nope"),"label",async()=>run(0,""),options,line=>noFooter.push(line))).toBe(1);expect(noFooter).toEqual([]);
 const moved="\n> nx run a:b\nRepoError: unserved build.closure.moving {\"count\":0}\n\nFailed tasks:\n\n- a:b\n";let reruns=0;const lines:string[]=[];
 const failure=await settleNxRunReportedV1(run(1,moved),"label",async()=>{reruns++;return run(1,moved);},options,line=>lines.push(line)).catch(error=>error as Error);
 expect(failure).toBeInstanceOf(Error);expect(reruns).toBeGreaterThan(0);expect(lines.filter(line=>line.startsWith("Nx run failed: 1 task(s) did not succeed after "+reruns+" rerun(s)"))).toHaveLength(1);
 const ok:string[]=[];expect(await settleNxRunReportedV1(run(1,moved),"label",async()=>run(0,""),options,line=>ok.push(line))).toBe(0);expect(ok).toEqual([]);
});
