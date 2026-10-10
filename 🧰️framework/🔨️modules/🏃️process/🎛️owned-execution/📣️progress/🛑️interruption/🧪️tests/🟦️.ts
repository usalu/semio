import { expect, test } from "bun:test";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/🟦️.ts";
import { runOwnedCommand } from "../../../🟦️.ts";

const childCode = `console.log("[DEBUG] interruption child PID "+process.pid);setTimeout(()=>console.log("[DEBUG] interruption child complete"),${corpus.childMilliseconds});`;
const oracleCode = 'const c=JSON.parse(process.argv[1]),r=JSON.parse(process.argv[2]),code=process.argv[3],started=performance.now(),controller=new AbortController(),parent=setTimeout(()=>controller.abort(),c.callerCeilingMilliseconds),cancel=r.interruption==="cancelled"?setTimeout(()=>controller.abort(),c.interruptionMilliseconds):null,signal=r.interruption==="deadline"?AbortSignal.any([controller.signal,AbortSignal.timeout(c.interruptionMilliseconds)]):controller.signal;const {Readable,Writable}=require("node:stream"),{pipeline}=require("node:stream/promises"),child=require("node:child_process").spawn(process.execPath,["--eval",code]);let childClosed=false,published=false,publications=0,pending;child.stdout.resume();child.stderr.resume();signal.addEventListener("abort",()=>{if(!childClosed)child.kill()},{once:true});const closed=new Promise((accept,refuse)=>{child.once("error",refuse);child.once("close",(status,signal)=>{childClosed=status===0&&signal===null;accept()})}),sink=new Writable({write(_bytes,_encoding,accept){publications++;pending=new Promise(resolve=>setTimeout(()=>{published=true;accept();resolve()},c.publicationMilliseconds))}}),source=Readable.from((async function*(){await new Promise(resolve=>setTimeout(resolve,c.progressIntervalMilliseconds));yield "progress"})()),publication=pipeline(source,sink,{signal});publication.catch(()=>{});(async()=>{try{await closed;let terminal="completed";try{await publication}catch{terminal="rejected"}console.log(JSON.stringify({terminal,publishedBeforeTerminal:published,childClosedBeforeTerminal:childClosed,publications,elapsedMilliseconds:performance.now()-started}));}finally{await pending;clearTimeout(parent);if(cancel)clearTimeout(cancel);if(child.exitCode===null&&child.signalCode===null)child.kill()}})().catch(error=>{console.error(error);process.exitCode=1});';

for (const row of corpus.cases) test(row.id, async () => {
  const started = performance.now(), controller = new AbortController();
  const parent = setTimeout(() => controller.abort(), corpus.callerCeilingMilliseconds);
  const cancel = row.interruption === "cancelled" ? setTimeout(() => controller.abort(), corpus.interruptionMilliseconds) : undefined;
  const pending: Promise<void>[] = [];
  let publications = 0, published = false, childPid = 0, childCompleted = false;
  const oracle = Bun.spawn(["node", "--eval", oracleCode, JSON.stringify(corpus), JSON.stringify(row), childCode], { stdout: "pipe", stderr: "pipe" });
  try {
    expect(new Ajv({ strict: true }).compile(schema)(corpus)).toBe(true);
    expect(validateJsonSchemaSubset(schema, corpus)).toEqual([]);
    const phaseCeiling = row.interruption === "deadline" ? corpus.interruptionMilliseconds : corpus.callerCeilingMilliseconds;
    let terminal = "completed", failure = "";
    try {
      await runOwnedCommand(process.execPath, ["--eval", childCode], process.cwd(), row.id, phaseCeiling - (performance.now() - started), {
        signal: controller.signal,
        onLine: line => { const match = /^\[DEBUG\] interruption child PID ([0-9]+)$/u.exec(line); if (match) childPid = Number(match[1]); if (line === "[DEBUG] interruption child complete") childCompleted = true; },
        onProgress: () => { publications++; const publication = new Promise<void>(accept => setTimeout(() => { published = true; accept(); }, corpus.publicationMilliseconds)); pending.push(publication); return publication; }
      });
    } catch (error) { terminal = "rejected"; failure = String(error); }
    const elapsedMilliseconds = performance.now() - started;
    let childClosed = false;
    if (childPid > 0) try { process.kill(childPid, 0); } catch (error) { childClosed = (error as NodeJS.ErrnoException).code === "ESRCH"; }
    const actual = { terminal, publishedBeforeTerminal: published, childClosedBeforeTerminal: childCompleted && childClosed };
    const [stdout, stderr, status] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
    expect(status, stderr).toBe(0);
    const observed = JSON.parse(stdout), expected = { terminal: row.terminal, publishedBeforeTerminal: row.publishedBeforeTerminal, childClosedBeforeTerminal: row.childClosedBeforeTerminal };
    expect({ terminal: observed.terminal, publishedBeforeTerminal: observed.publishedBeforeTerminal, childClosedBeforeTerminal: observed.childClosedBeforeTerminal }).toEqual(expected);
    console.log("[DEBUG] " + JSON.stringify({ id: row.id, actual, oracle: observed, publications, childPid, elapsedMilliseconds, failure }));
    expect(childPid).toBeGreaterThan(0);
    expect(publications).toBe(1);
    expect(observed.publications).toBe(1);
    expect(elapsedMilliseconds).toBeLessThan(corpus.callerCeilingMilliseconds);
    expect(observed.elapsedMilliseconds).toBeLessThan(corpus.callerCeilingMilliseconds);
    expect(controller.signal.aborted).toBe(row.interruption === "cancelled");
    expect(failure).toContain(row.interruption === "cancelled" ? "cancelled" : "timeout");
    expect(actual).toEqual(expected);
  } finally {
    await Promise.allSettled(pending);
    clearTimeout(parent);
    if (cancel !== undefined) clearTimeout(cancel);
    if (oracle.exitCode === null) { oracle.kill(); await oracle.exited; }
  }
}, corpus.callerCeilingMilliseconds);
