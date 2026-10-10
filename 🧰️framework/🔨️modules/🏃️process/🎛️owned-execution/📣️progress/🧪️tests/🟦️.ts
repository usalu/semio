import { expect, test } from "bun:test";
import Ajv from "ajv";
import corpus from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/🟦️.ts";
import { runOwnedCommand } from "../../🟦️.ts";

for (const row of corpus.cases) test(row.id, async () => {
  const started = performance.now(), controller = new AbortController();
  const deadline = setTimeout(() => controller.abort(), corpus.callerCeilingMilliseconds);
  const pending: Promise<void>[] = [];
  let publications = 0, published = false;
  expect(new Ajv({ strict: true }).compile(schema)(corpus)).toBe(true);
  expect(validateJsonSchemaSubset(schema, corpus)).toEqual([]);
  const oracleCode = 'const c=JSON.parse(process.argv[1]),r=JSON.parse(process.argv[2]);let done=false;const publication=new Promise((resolve,reject)=>setTimeout(()=>setTimeout(()=>{done=true;r.rejectPublication?reject(Error("publisher refused")):resolve()},c.publicationMilliseconds),c.progressIntervalMilliseconds));publication.catch(()=>{});const child=new Promise((resolve,reject)=>{const p=require("node:child_process").spawn(process.execPath,["--eval",`setTimeout(()=>{},${c.childMilliseconds})`]);p.once("error",reject);p.once("close",code=>code===0?resolve():reject(Error("child refused")))});Promise.all([child,publication]).then(()=>console.log(JSON.stringify({terminal:"completed",publishedBeforeTerminal:done})),()=>console.log(JSON.stringify({terminal:"rejected",publishedBeforeTerminal:done})));';
  const oracle = Bun.spawn(["node", "--eval", oracleCode, JSON.stringify(corpus), JSON.stringify(row)], { stdout: "pipe", stderr: "pipe" });
  const publish = (): Promise<void> => {
    publications++;
    const publication = new Promise<void>((accept, refuse) => setTimeout(() => { published = true; row.rejectPublication ? refuse(Error("publisher refused")) : accept(); }, corpus.publicationMilliseconds));
    publication.catch(() => {});
    pending.push(publication);
    return publication;
  };
  try {
    let terminal = "completed", failure = "";
    try { await runOwnedCommand(process.execPath, ["--eval", `setTimeout(()=>{},${corpus.childMilliseconds})`], process.cwd(), row.id, corpus.callerCeilingMilliseconds - (performance.now() - started), { stdout: "ignore", signal: controller.signal, onProgress: publish }); }
    catch (error) { terminal = "rejected"; failure = String(error); }
    const actual = { terminal, publishedBeforeTerminal: published };
    const [oracleStdout, oracleStderr, oracleCodeValue] = await Promise.all([new Response(oracle.stdout).text(), new Response(oracle.stderr).text(), oracle.exited]);
    expect(oracleCodeValue, oracleStderr).toBe(0);
    const expected = { terminal: row.terminal, publishedBeforeTerminal: row.publishedBeforeTerminal };
    expect(JSON.parse(oracleStdout)).toEqual(expected);
    console.log("[DEBUG] " + JSON.stringify({ id: row.id, actual, oracle: expected, publications, failure }));
    expect(publications).toBe(1);
    expect(actual).toEqual(expected);
    if (row.rejectPublication) expect(failure).toContain("publisher refused");
  } finally {
    await Promise.allSettled(pending);
    clearTimeout(deadline);
    if (oracle.exitCode === null) { oracle.kill(); await oracle.exited; }
  }
}, corpus.callerCeilingMilliseconds);
