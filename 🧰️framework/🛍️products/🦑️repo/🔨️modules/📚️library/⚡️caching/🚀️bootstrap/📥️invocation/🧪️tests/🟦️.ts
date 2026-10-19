import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import ts from "typescript";
import * as bootstrap from "../../📜️script.ts";
import law from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("bootstrap wire keeps the inherited deadline, policy and capabilities",()=>{
  const validate=new Ajv({strict:true}).compile<typeof law>(schema);expect(validate(law)).toBe(true);
  const db=new Database(":memory:");
  try{
    db.run("CREATE TABLE accepted(id TEXT PRIMARY KEY, wire TEXT)");
    for(const row of law.cases){
      const environment:NodeJS.ProcessEnv={SEMIO_SCRIPT_PROCESS_INVOCATION:JSON.stringify(law.wire),...(row.policy===null?{}:{SEMIO_SCRIPT_POLICY:JSON.stringify(row.policy)}),...(row.capabilities===null?{}:{SEMIO_SCRIPT_CAPABILITIES:JSON.stringify(row.capabilities)})};
      const original=JSON.stringify(environment);
      const run=()=>bootstrap.nxBootstrapEnvelopeV1(process.cwd(),["nx","run","owner:target"],environment,row.now);
      if(row.expected==="accept"){const actual:unknown=run();expect(actual).toEqual(law.wire);db.run("INSERT INTO accepted VALUES(?,?)",[row.id,JSON.stringify(actual)]);}
      else expect(run).toThrow(row.expected==="policy"?"Original Nx process policy changed":row.expected==="capabilities"?"Original Nx process capabilities changed":"Original Script process deadline refused");
      expect(JSON.stringify(environment)).toBe(original);
    }
    const rows=db.query("SELECT id, json_extract(wire,'$.deadlineEpochMilliseconds') AS deadline, json_extract(wire,'$.policy.maximumElapsedMilliseconds') AS budget, json_extract(wire,'$.capabilities.original') AS origin FROM accepted ORDER BY id").all() as {id:string;deadline:number;budget:number;origin:string}[];
    expect(rows).toHaveLength(law.cases.filter(row=>row.expected==="accept").length);
    for(const row of rows){expect(row.deadline).toBe(law.wire.deadlineEpochMilliseconds);expect(row.budget).toBe(law.wire.policy.maximumElapsedMilliseconds);expect(row.origin).toBe(law.wire.capabilities.original);}
    console.log(`[DEBUG] ${law.cases.length} bootstrap wire boundaries preserve the exact inherited deadline and closed policy/capabilities against SQLite; no live build authority is fabricated`);
  }finally{db.close();}
});

test("both bootstrap dispatch branches live inside original scope and shared cleanup",()=>{
  const source=readFileSync(new URL("../../📜️script.ts",import.meta.url),"utf8"),file=ts.createSourceFile("bootstrap.ts",source,ts.ScriptTarget.Latest,true);
  const main=file.statements.find((node):node is ts.IfStatement=>ts.isIfStatement(node)&&node.expression.getText(file)==="import.meta.main");expect(main).toBeDefined();
  if(!main||!ts.isBlock(main.thenStatement))throw Error("actual bootstrap main block required");
  const guard=main.thenStatement.statements.find((node):node is ts.TryStatement=>ts.isTryStatement(node)&&node.finallyBlock!==undefined);expect(guard).toBeDefined();
  if(!guard?.finallyBlock)throw Error("bootstrap original environment cleanup required");
  const body=guard.tryBlock.getText(file),cleanup=guard.finallyBlock.getText(file);
  expect(body).toContain("withScriptProcessEnvelope(envelope");expect(body).toContain("native.launchDashboard(");expect(body).toContain('register("nx",NxScript).run(args,original)');
  expect(cleanup).toContain("delete process.env[SCRIPT_PROCESS_INVOCATION_ENV]");expect(cleanup).toContain("process.env[SCRIPT_PROCESS_INVOCATION_ENV]=inherited");
  expect(body).toContain("original.control.signal.aborted?130:1");expect(body).toContain("scriptCompilerAdmissionControlV1(original");
});

test("Process suite runner keeps the incoming per-test timeout and exact law type check",()=>{
  const source=readFileSync(new URL("../../../../../../../../🔨️modules/🏃️process/📜️script.ts",import.meta.url),"utf8");
  expect(source).toContain('["test", "--timeout", String(scriptInvocationBudget(this.invocation, suite.budgetMs)), ...sources');
  const check=source.split("\n").find(line=>line.includes("typescript/bin/tsc")&&line.includes("suite.budgetMs"));expect(check).toContain('"exact-cargo-laws"');
  expect(source).toContain("signal: this.invocation.control.signal");
});


test("actual manifest custody recheck uses the bounded walker without losing rows",async()=>{
  const source=readFileSync(new URL("../../📜️script.ts",import.meta.url),"utf8");
  expect(source).toContain("await boundedWalk(admitted, NX_ADMISSION_CONCURRENCY, async file =>");
  const selected=source.slice(source.indexOf("await boundedWalk(admitted, NX_ADMISSION_CONCURRENCY"),source.indexOf("const custodyFinished"));
  for(const predicate of["signal?.throwIfAborted()","await real(dirname(join(root, file.path)))","!now.isFile()","now.isSymbolicLink()","now.dev !== file.device","now.ino !== file.inode"])expect(selected).toContain(predicate);
  const {boundedAdmissionWalkV1}=await import("../../📥️admission/🟦️.ts");
  const db=new Database(":memory:");
  try{
    db.run("CREATE TABLE visited(batch INTEGER, rank INTEGER, PRIMARY KEY(batch,rank))");
    for(const count of law.walk.counts){let active=0,peak=0;
      await boundedAdmissionWalkV1(Array.from({length:count},(_,rank)=>rank),law.walk.concurrency,async rank=>{active++;peak=Math.max(active,peak);await Promise.resolve();db.run("INSERT INTO visited VALUES(?,?)",[count,rank]);active--;});
      expect(active).toBe(0);expect(peak).toBe(Math.min(count,law.walk.concurrency));
      const rows=db.query("SELECT rank FROM visited WHERE batch=? ORDER BY rank").all(count) as {rank:number}[];expect(rows.map(row=>row.rank)).toEqual(Array.from({length:count},(_,rank)=>rank));
    }
    console.log(`[DEBUG] Actual admission walker preserves ${law.walk.counts.length} finite manifest batches under ${law.walk.concurrency} concurrent callbacks against SQLite`);
  }finally{db.close();}
});
