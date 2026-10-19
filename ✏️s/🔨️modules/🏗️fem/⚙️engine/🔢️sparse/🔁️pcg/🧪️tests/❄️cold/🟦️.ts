import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "bun:test";
import Ajv from "ajv/dist/2020";
import { lusolve } from "mathjs";
import corpus from "../🧫️fixtures/❄️cold/🔣️.json";
import schema from "../🧫️fixtures/❄️cold/🧬️schema.json";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

function arithmetic(sample: typeof corpus.cases[number]) {
  const x = [...sample.initial], n = x.length;
  const multiply = (values: number[]) => sample.matrix.map(row => row.reduce((sum, value, column) => sum + value * values[column]!, 0));
  const dot = (left: number[], right: number[]) => left.reduce((sum, value, i) => sum + value * right[i]!, 0);
  const diagonal = sample.matrix.map((row, i) => row[i]!);
  const precondition = (r: number[]) => r.map((value, i) => Math.abs(diagonal[i]!) > 1e-300 ? value / diagonal[i]! : value);
  const ax = multiply(x), r = sample.rhs.map((value, i) => value - ax[i]!);
  const norm = Math.max(Math.sqrt(dot(sample.rhs, sample.rhs)), 1e-300);
  let residual = Math.sqrt(dot(r, r)) / norm, iterations = 0;
  if (residual < sample.tolerance) return { x, residual, iterations, converged: true };
  if (sample.maximumIterations === 0) return { x, residual, iterations, converged: false };
  let z = precondition(r), p = [...z], rz = dot(r, z);
  for (iterations = 1; iterations <= sample.maximumIterations; iterations++) {
    const ap = multiply(p), curvature = dot(p, ap);
    if (Math.abs(curvature) < 1e-300) break;
    const alpha = rz / curvature;
    for (let i = 0; i < n; i++) { x[i] = x[i]! + alpha * p[i]!; r[i] = r[i]! - alpha * ap[i]!; }
    residual = Math.sqrt(dot(r, r)) / norm;
    if (residual < sample.tolerance) return { x, residual, iterations, converged: true };
    if (iterations === sample.maximumIterations) break;
    z = precondition(r);
    const next = dot(r, z), beta = next / rz;
    p = z.map((value, i) => value + beta * p[i]!); rz = next;
  }
  return { x, residual, iterations, converged: false };
}

export function verifyNumericalExecutionOracle(): void {
  const validate = new Ajv({ strict: true }).compile(schema);
  assert(validate(corpus), JSON.stringify(validate.errors));
  for (const sample of corpus.cases) {
    const n = sample.initial.length;
    assert.equal(sample.matrix.length, n); assert.equal(sample.rhs.length, n); assert.equal(sample.solution.length, n);
    sample.matrix.forEach(row => assert.equal(row.length, n));
    const actual = arithmetic(sample);
    assert.equal(actual.converged, sample.converged, sample.name); assert.equal(actual.iterations, sample.iterations, sample.name);
    sample.solution.forEach((value, i) => assert(Math.abs(actual.x[i]! - value) < 1e-10, sample.name));
    if (sample.converged && n > 0) {
      const independent = lusolve(sample.matrix, sample.rhs).valueOf() as number[][];
      independent.forEach((row, i) => assert(Math.abs(actual.x[i]! - row[0]!) < 1e-10, sample.name));
    }
  }
  console.log(`[DEBUG] ${corpus.cases.length} neutral PCG cases retain convergence, iteration-limit, curvature and scaling semantics; converged outputs match mathjs LU`);
}

function source(relative: string): string { return readModuleTreeSource(new URL(relative, import.meta.url)); }
function body(text: string, name: string): string {
  const start = text.indexOf(`pub fn ${name}(`); assert(start >= 0, name);
  const open = text.indexOf("{", start); let depth = 1, cursor = open + 1;
  while (depth > 0) { const token = text[cursor++]!; if (token === "{") depth++; else if (token === "}") depth--; assert(cursor <= text.length); }
  return text.slice(open + 1, cursor - 1);
}

test("cold numerical outcomes match language-neutral fixtures and independent mathjs LU", verifyNumericalExecutionOracle);
test("mathematical wrappers share arithmetic without minting native authority; native batch requires actual caller", () => {
  const pcg = source("../../🦀️.rs"), subspace = source("../../../🎵️subspace/🦀️.rs"), assembly = source("../../../../🧮️analyses/🧩️assembly/🦀️.rs"), common = source("../../../🦀️.rs");
  for (const [text, name] of [[pcg, "pcg"], [subspace, "subspace_iteration"], [assembly, "assemble_system"]] as const) {
    const mathematical = body(text, name);
    for (const forbidden of ["StepContext", "root_cancel_token", "numerical_batch", "allocate_operation_id", "Job::new", ".step("]) assert(!mathematical.includes(forbidden), `${name}: ${forbidden}`);
  }
  assert(pcg.includes("let advanced = self.state.advance_one()")); assert(body(pcg, "pcg").includes("state.advance_one()"));
  assert(subspace.includes("self.state.advance_non_positive_certificate()")); assert(subspace.includes("self.advance_non_positive_certificate()?"));
  assert(assembly.includes("Some(element_contribution(&self.plan, dynamic")); assert(body(assembly, "assemble_system").includes("element_contribution(&plan, model"));
  assert(!common.includes("StepContext::new")); assert(!common.includes("root_cancel_token")); assert(!common.includes("numerical_owned_close_turn"));
  assert(body(common, "numerical_batch_step<J: InteractiveJob>").includes("job.step(context)?"));
});
