import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readModuleTreeSource, readSourceFamilySource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import "../../../🎟️context/🪙️loan/🧪️tests/🟦️.ts";

const job = resolve(import.meta.dirname, "../../..");
const workspace = resolve(job, "../../..");
const read = (file: string) => readFileSync(resolve(job, file), "utf8");
const closeAuthority = read("♻️retirement/🎟️context/🦀️.rs");
const corpus = JSON.parse(read("♻️retirement/🎟️context/🧪️tests/🧫️fixtures/🔣️.json"));
const schema = JSON.parse(read("♻️retirement/🎟️context/🧬️schema/🔣️.json"));
const validate = new Ajv({ strict: true }).compile(schema);

function section(source: string, marker: string, next: string): string {
  const start = source.indexOf(marker);
  const end = source.indexOf(next, start + marker.length);
  assert(start >= 0 && end > start, marker);
  return source.slice(start, end);
}

/** 🎟️ Independent accounting preserves CPU, absolute deadline and source identity. */
test("original close retains caller context and charges actual work once", () => {
  assert.equal(validate(corpus), true);
  const db = new Database(":memory:");
  try {
    for (const work of corpus.workUnits) {
      const row = db.query("SELECT ?1 AS operation, ?2 AS generation, ?3 AS deadline, ?4 AS remaining, CASE WHEN ?4 > 0 AND ?5 < ?3 THEN 1 ELSE 0 END AS spent").get(corpus.operation, corpus.generation, corpus.deadlineUs, work, corpus.nowUs) as { operation: number; generation: number; deadline: number; remaining: number; spent: number };
      assert.equal(row.operation, corpus.operation);
      assert.equal(row.generation, corpus.generation);
      assert.equal(row.deadline, corpus.deadlineUs);
      assert.equal(row.spent, Math.min(work, 1));
      assert.equal(row.remaining - row.spent, Math.max(work - 1, 0));
    }
  } finally { db.close(); }
  const protocol = read("🚦️clock-stride-and-job-protocol/🦀️.rs");
  assert.match(protocol, /fn close_step\(&mut self,\s*cx:\s*&mut StepContext<'_>\)\s*->\s*Result<InteractiveJobCloseStep,\s*OriginalTurnFailure>/);
});

test("original worker close retains complete failures instead of a copied refusal marker", () => {
  const worker = read("👷️worker/🎟️authority/🦀️.rs");
  const wake = read("♻️retirement/🔔️wake/🦀️.rs");
  const declaration = section(worker, "pub enum WorkerJobCloseStep", "impl WorkerJobCloseStep");
  assert(!declaration.includes("Refused"));
  assert(!readSourceFamilySource(job).includes("WorkerJobCloseStep::Refused"));
  assert.match(worker, /fn close_original_worker_params\(&mut self,\s*cx:\s*&mut StepContext<'_>\)\s*->\s*Result<WorkerJobCloseStep,\s*ValueError>/);
  assert.match(wake, /fn close_session_retained_wake<J>\(inner:\s*&WorkerJobSessionInner<J>,\s*cx:\s*&mut StepContext<'_>\)\s*->\s*Result<Option<WorkerJobCloseStep>,\s*ValueError>/);
});

test("original step and native preparation retain the complete canonical failure", () => {
  const source = read("🧪️conformance/🦀️.rs");
  const worker = read("👷️worker/🎟️authority/🦀️.rs");
  const tortureJob = section(source, "impl InteractiveJob for TortureJob", "//#endregion");
  assert.match(source, /fn step_original<'a>\(&'a mut self,cx:&mut StepContext<'_>\)->Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>/);
  assert.match(tortureJob, /&mut OriginalJobReturnClaim<'_>\)->InteractiveJobStep<'a>/);
  assert(tortureJob.includes("InteractiveJobStep::from_callback(self.step_original(cx))"));
  assert.match(read("🏃️headless/🦀️.rs"), /pub fn original_failure\(&self\)\s*->\s*Option<&OriginalTurnFailure>/);
  assert(worker.includes("self.original.retain_initialization_failure(error)"));
  assert.match(worker, /pub fn retain_initialization_failure[^\n]+\*self\.step_fault=Some\(error\.into\(\)\)/);
});

test("original Store close checks denial before mutation and charges work across receipt failure", () => {
  const store = readFileSync(resolve(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/📄️decode-page-state-and-authority/🦀️.rs"), "utf8");
  const raw = section(store, "pub fn close_granted(", "self.pending_field=None");
  assert(raw.includes("if grant.maximum_items == 0 || grant.maximum_depth == 0"));
  assert(raw.includes("return Ok(Close::Pending { progress: empty });"));
  const close = section(store, "fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)", "fn next_close_copy_byte_demand");
  assert(close.includes("cx.with_original_effect(grant,|_,effect|self.close_granted(effect))"));
  assert(close.includes("receive_original_retained_work(cx,turn)"));
  const settlement = section(closeAuthority, "fn receive_original_work", "pub fn receive_original_retained_work");
  assert.match(settlement, /let receipt_refusal=cx\.consume_retained\(progress\)\.err\(\);cx\.consume_work\(turn\.work\);/);
  assert(settlement.includes("effects:progress,primary,receipt_refusal,context_refusal"));
});

test("latest-wins close preserves its complete original failure and mounted caller context", () => {
  const plugin = resolve(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const admission = readFileSync(resolve(plugin, "🎮️command/📥️admission/🦀️.rs"), "utf8");
  const ladder = readFileSync(resolve(plugin, "🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs"), "utf8");
  const close = section(admission, "pub fn close_step(&mut self,effect:", "pub fn close_key_step");
  assert.match(close, /&mut RetainedAllocationEffect<'_>\)->Result<PluginLifecycleStep,PluginLifecycleFailure>/);
  assert.match(ladder, /fn close_ladder_step\(&mut self, cx: &mut semio_framework_job::StepContext<'_>, grant: RetainedCloneGrant\) -> Result<RetainedCloneStep, PluginLifecycleFailure>/);
  assert(ladder.includes("A::mounted_job_close_step(instance_id, cx, grant)"));
  assert(!close.includes("InteractiveJobCloseStep::Refused") && !close.includes('format!("latest-wins original wire refused'));
});

test("granted close body is debited once and never runs on exhausted fuel", () => {
  assert.equal(validate(corpus), true);
  assert.equal(validate({ ...corpus, granted: { ...corpus.granted, work: "fixed-cost-per-loop" } }), false);
  const db = new Database(":memory:");
  try {
    for (const fuel of [0, 1, 3]) {
      for (const work of corpus.workUnits) {
        const row = db.query("SELECT ?1 > 0 AS ran, MAX(?1 - CASE WHEN ?1 > 0 THEN ?2 ELSE 0 END, 0) AS remaining").get(fuel, work) as { ran: number; remaining: number };
        assert.equal(row.ran, Math.min(fuel, 1));
        assert.equal(row.remaining, Math.max(fuel - work, 0));
      }
    }
  } finally { db.close(); }
  const granted = section(closeAuthority, "pub fn close_original_granted_step", "pub fn close_original_nested_step");
  assert.match(granted, /retirement_should_yield\(\)[\s\S]*let turn=close\(cx\.retained_grant\(\)\)/);
  assert.equal(granted.match(/fold_original_close\(cx,/g)?.length, 1);
  assert(granted.includes("work:turn.work"));
});

test("nested close body owes only the remainder its nested callers did not post", () => {
  assert.equal(validate(corpus), true);
  assert.equal(validate({ ...corpus, nested: { ...corpus.nested, underReport: "ignored" } }), false);
  const db = new Database(":memory:");
  try {
    for (const [reported, before, after, owed] of [[2, 0, 1, 1], [2, 0, 0, 2], [1, 0, 1, 0], [0, 0, 1, -1]]) {
      const row = db.query("SELECT ?1-(?3-?2) AS owed").get(reported, before, after) as { owed: number };
      assert.equal(row.owed, owed);
    }
  } finally { db.close(); }
  const nested = closeAuthority.slice(closeAuthority.indexOf("pub fn close_original_nested_step"));
  assert(nested.includes("reported.checked_sub(posted)") && nested.includes("InvariantViolated"));
  assert(nested.includes("fold_original_close(cx,RetainedWorkOutcome{result:Ok(InteractiveJobCloseStep::Pending{progress}),work:0})"));
});

test("a genuine close caller lends its exact grant and retains its finite lifetime", () => {
  assert.equal(validate(corpus), true);
  assert.equal(validate({ ...corpus, genuineCaller: { ...corpus.genuineCaller, fuel: "unbounded" } }), false);
  assert.equal(validate({ ...corpus, freshRootPerTurn: true }), false);
  const caller = read("⏱️context/🧪️testing/🎟️caller/🦀️.rs");
  assert(caller.includes("parent:ManuallyDrop::new(Some(parent))"));
  assert(caller.includes("StepBudget::new(self.fuel,self.deadline,self.stock)"));
  assert(caller.includes("parent.borrow_context(budget,self.clock"));
  assert(caller.includes("cx.with_retained_loan(grant,|cx|"));
  assert(caller.includes("self.fuel=cx.original_source_fuel_remaining().min(issued_fuel)"));
});

test("original close five-currency vectors agree with independent SQLite", () => {
  expect(validateJsonSchemaSubset(schema, corpus)).toEqual([]);
  expect(semioSchemaAjvV1({ strict: true }).compile(schema)(corpus)).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of corpus.cases) {
      const actual = row.grant.every((value: number, index: number) => value >= row.quote[index]);
      const result = db.query("SELECT ?1>=?6 AND ?2>=?7 AND ?3>=?8 AND ?4>=?9 AND ?5>=?10 AS admitted").get(...row.grant, ...row.quote) as { admitted: number };
      expect(actual).toBe(row.admitted);
      expect(Boolean(result.admitted)).toBe(row.admitted);
    }
  } finally { db.close(); }
});

test("original close keeps whole diagnostic or value and Raster keeps original stages", () => {
  assert(closeAuthority.includes("RetainedWorkOutcome<RetainedCloneStep,crate::OriginalTurnPrimary>"));
  assert(closeAuthority.includes("Err(OriginalTurnFailure{effects:progress,primary,receipt_refusal,context_refusal"));
  const raster = resolve(workspace, "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands");
  for (const name of ["🥞️flatten-layers", "🎭️mask-from-selection"]) {
    const work = readModuleTreeSource(resolve(raster, name, "🦀️.rs"));
    assert.match(work, /fn close_step\(&mut self,\s*cx:\s*&mut semio_framework_job::StepContext<'_>\)/);
    assert(work.includes("cx.retained_grant()") && work.includes("fn terminal_is_empty(&self)->bool"));
    assert(work.includes("OriginalTurnFailure"));
  }
});
