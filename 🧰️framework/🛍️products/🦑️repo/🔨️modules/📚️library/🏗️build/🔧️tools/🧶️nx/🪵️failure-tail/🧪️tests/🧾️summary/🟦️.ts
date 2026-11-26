/** 🧾️ Contract: when an Nx run ends in failure the parent says, per failed task, why it failed and whether another rerun of the whole run is likely to help (a peer edited its inputs, the build implementation is outdated, or the task itself failed), lists the tasks that did not run, and does so in the declared languages; an independent Python oracle reads the same transcripts to the same rows; a failure that stands is reported once, and a rerun that succeeds is silent. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {NX_FAILURE_SUMMARY_POLICY_V1 as policy,nxFailedTaskTailsV1,nxFailureSummaryLinesV1,nxFailureSummaryOfTailV1,nxFinalStatusV1,settleNxRunReportedV1} from "../../🟦️.ts";
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
  expect<unknown>(own??null,row.id).toEqual(row.expect??null);
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

test("a failed task whose output carries no header is read from the output before the summary, with its error line kept, and a task without any output says so",()=>{
 const warnings=Array.from({length:80},(_,index)=>"warning: unused "+index).join("\n"),text=warnings+"\nerror: failed to write `/x/invoked.timestamp`\nCaused by:\n  No such file or directory (os error 2)\n"+Array.from({length:12},(_,index)=>"  at frame "+index).join("\n")+"\n✔  nx run a:ok\n NX   failed\nFailed tasks:\n- a:build\n",rows=nxFailedTaskTailsV1(text,5)[0]!.lines;
 expect(rows[0]).toContain((policy.texts.en as Record<string,string>).unattributed);
 expect(rows).toContain("error: failed to write `/x/invoked.timestamp`");expect(rows).toContain("  No such file or directory (os error 2)");expect(rows.at(-1)).toBe("  at frame 11");
 expect(nxFailedTaskTailsV1("✔  nx run a:ok\n NX   failed\nFailed tasks:\n- a:build\n")[0]!.lines.length).toBe(1);
});
test("a coloured transcript is read like a plain one: reason and the bounded own output tail of every failed task, colour codes and progress rows removed",()=>{
 const text="\u001b[2m> \u001b[22mnx run a:build\n[script] cache:entry · x\n"+Array.from({length:70},(_,index)=>"\u001b[1mline "+index+"\u001b[0m").join("\n")+"\n\u001b[31merror\u001b[0m: boom\n\n NX   failed\nFailed tasks:\n- a:build\n- b:build\n",tails=nxFailedTaskTailsV1(text,5);
 expect(tails.map(row=>row.task)).toEqual(["a:build","b:build"]);
 expect(tails[0]!.lines).toEqual(["line 66","line 67","line 68","line 69","error: boom"]);expect(tails[1]!.lines).toEqual([(policy.texts.en as Record<string,string>).silent+" / "+(policy.texts.de as Record<string,string>).silent]);
 expect(nxFailureSummaryOfTailV1(text)!.failed[0]!.reason).toBe("error: boom");
 expect(nxFailedTaskTailsV1("no footer")).toEqual([]);
 expect(nxFailedTaskTailsV1(text)[0]!.lines.length).toBeLessThanOrEqual(policy.maximumTailLines);
});

test("a signal that reaches the run after Nx ended in a failure is reported as a consequence and the failure keeps its status; a signal during the run keeps 130 or 143; the lines are bilingual",()=>{
 const tail=fixture.cases[0]!.tail,reference=(row:{code:number;signal?:string;cancelledAtMs?:number;closedAtMs:number})=>row.signal===undefined?row.code:row.code!==0&&(row.cancelledAtMs??Infinity)>=row.closedAtMs?row.code:row.signal==="SIGINT"?130:143;
 for(const row of [{code:1,signal:"SIGINT",cancelledAtMs:200,closedAtMs:100},{code:1,signal:"SIGINT",cancelledAtMs:50,closedAtMs:100},{code:1,signal:"SIGTERM",cancelledAtMs:50,closedAtMs:100},{code:0,signal:"SIGINT",cancelledAtMs:200,closedAtMs:100},{code:1,closedAtMs:100}]){
  const own=nxFinalStatusV1({...row,tail},{});
  expect(own.status,JSON.stringify(row)).toBe(reference(row));
  expect(own.lines).toHaveLength(row.signal===undefined?0:2);
 }
 const late=nxFinalStatusV1({code:1,signal:"SIGINT",cancelledAtMs:200,closedAtMs:100,tail},{});
 expect(late.lines[0]!.startsWith("[nx] SIGINT reached this run after Nx had already ended with exit status 1 (failed: ")).toBe(true);
 expect(late.lines[0]).toContain("workspace:deps-cargo:prototype");
 expect(late.lines[1]!.startsWith("[nx] SIGINT erreichte diesen Lauf, nachdem Nx schon mit Exit-Status 1 geendet hatte (fehlgeschlagen: ")).toBe(true);
 expect(nxFinalStatusV1({code:1,signal:"SIGINT",cancelledAtMs:50,closedAtMs:100,tail},{}).lines[0]).toContain("exit status 130");
});
