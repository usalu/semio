import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { applyPatch } from "fast-json-patch";
import fixture from "../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json";

test("original SDK failed normal turn preserves its actual child receipt", () => {
  const law = fixture.failedNormalReceipt;
  expect(() => JSON.parse(law.source)).toThrow();
  const actual = { copiedItems: 1, copiedBytes: Buffer.byteLength("雪"), retainedCapacityBytes: 64, releasedBytes: 0 };
  const oracle = applyPatch({ ...law.grant }, [
    { op: "replace", path: "/maximumItems", value: law.grant.maximumItems - actual.copiedItems },
    { op: "replace", path: "/maximumCopyBytes", value: law.grant.maximumCopyBytes - actual.copiedBytes },
    { op: "replace", path: "/maximumCapacityBytes", value: law.grant.maximumCapacityBytes - actual.retainedCapacityBytes },
    { op: "replace", path: "/maximumReleaseBytes", value: law.grant.maximumReleaseBytes - actual.releasedBytes },
  ], true).newDocument;
  expect(oracle.maximumCopyBytes).toBe(4093);
  expect(oracle.maximumCapacityBytes).toBe(1048512);
  const source = readFileSync(new URL("../../🦀️.rs", import.meta.url), "utf8");
  expect(source).toContain("self.normal_progress=progress;");
  expect(source).toContain("let receipt=admission.normal_step_progress();");
  expect(source).toContain("retained.normal_step_progress()");
  expect(source).toContain("binding.step(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?");
});

test("original Plugin workers forward the embedding phase policy unchanged",()=>{
 const source=readFileSync(new URL("../../../../../🔌️plugin/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("retained:self.mounted_policy.maintenance");expect(source).toContain("retained:state.mounted_policy.close");expect(source).toContain("decode,completion,self.mounted_policy.maintenance");expect(source).toContain("job,self.mounted_policy.maintenance");
});

test("original command and render views require actual retained authority",()=>{
 const source=readFileSync(new URL("../../../../../🔌️plugin/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub retained:RetainedCloneGrant");expect(source).toContain("pub fn retained_grant(&self)->Result<RetainedCloneGrant,Fault>");expect(source).toContain("Ok(operation.retained)");expect(source).toContain("Ok(operation.mounted_policy.maintenance)");expect(fixture.commandRetainedAuthority.missingAuthorityRefused).toBe(true);
});

test("original inference closes one granted owner turn and retains failed actual receipts",()=>{
 const law=fixture.originalInferenceCloseAuthority;expect(law.outcomeCloseTurns).toBe(1);
 const source=readFileSync(new URL("../../../../../🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs",import.meta.url),"utf8");expect(source).not.toContain("ABSORB_CLOSE_PAGES");expect(source).toContain("self.record_retirement(error.retained_progress())");
 const receipt=source.slice(source.indexOf("fn record_retirement("),source.indexOf("fn fail("));expect(receipt.indexOf("checked_add(progress)")).toBeLessThan(receipt.indexOf("progress.fits(self.request.retained)"));expect(source).toContain("self.phase = InteractivePhase::OutcomeClose");
 const physical=applyPatch({retainedCapacityBytes:0},[{op:"replace",path:"/retainedCapacityBytes",value:784}],true).newDocument;expect(physical.retainedCapacityBytes).toBe(784);
});

test("original SDK plan retains its exact input under original Registry cursor authority",()=>{
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("plan:neural_engine::OperatorPlanCursor");expect(source).toContain("registry.dispatch_job(operator,&mut self.pending_input,&mut self.plan,grant)");expect(source).toContain("self.normal_progress=self.plan.step_progress()");expect(source).not.toContain("match registry.dispatch(operator,&input)");expect(fixture.originalOperatorPlanCustody.coldFallback).toBe(false);
});

test("original SDK finish keeps the same output under original Registry cursor authority",()=>{
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("finish:neural_engine::OperatorFinishCursor");expect(source).toContain("registry.finish_job(operator,&mut self.finished,&mut self.finish,grant)");expect(source).toContain("self.normal_progress=self.finish.step_progress()");expect(source).not.toContain("registry.finish_job(operator,output)");expect(fixture.originalOperatorFinishCustody.coldValidation).toBe(false);
});


test("original SDK round trip debits one incoming wallet and rearms its same owner",()=>{
 const law=fixture.originalTurnWallet;const schema=require("../../🧬️schema/🎟️turn/🔣️.json");const Ajv=require("ajv").default;const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,reuseFullWallet:true})).toBe(false);expect(validate({...law,extra:true})).toBe(false);
 const child=law.children[0]!;const result=applyPatch({...law.grant},[{op:"replace",path:"/maximumItems",value:law.grant.maximumItems-child.copiedItems},{op:"replace",path:"/maximumCopyBytes",value:law.grant.maximumCopyBytes-child.copiedBytes},{op:"replace",path:"/maximumCapacityBytes",value:law.grant.maximumCapacityBytes-child.retainedCapacityBytes},{op:"replace",path:"/maximumReleaseBytes",value:law.grant.maximumReleaseBytes-child.releasedBytes}],true).newDocument;expect(result).toEqual(law.remaining);
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub retained:RetainedCloneGrant");expect(source).toContain("let memory_grant=request.retained");expect(source).toContain("maximum_items:memory_grant.maximum_items.saturating_sub(physical.copied_items)");expect(source).toContain("units,4096,remaining)");expect(source).toContain("physical.fits(memory_grant)");expect(source).toContain("receipt!=RetainedCloneProgress::default()");expect(source).not.toContain("evaluate_step_envelope(registry,request,EVALUATION_INVOKE_MEMORY_POLICY)");expect(source).not.toContain("if request.node_hash==0||(!expired");console.log("[DEBUG] Independent JSON Patch debits every actual original child currency once, preserves depth, and requires rearm after structural credit exhaustion");
});

test("original preview payload forwards its actual retained caller authority",()=>{
 for(const kind of ["🌀️generation2d","🧊️generation3d"]){const source=readFileSync(new URL(`../../../../../../../../../✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/${kind}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`,import.meta.url),"utf8");expect(source).toContain('("retained".to_string(), semio_framework_value::ToValue::to_value(&retained_grant))');expect(source).toContain("retained_grant: semio_framework_value::RetainedCloneGrant");}
});

test("original SDK normal failure retains its typed cause before display",()=>{
 const law=fixture.originalTypedFailure;const Ajv=require("ajv").default;const validate=new Ajv({strict:true}).compile(require("../../🧬️schema/⚠️failure/🔣️.json"));expect(validate(law)).toBe(true);expect(validate({...law,formatDuringNormal:true})).toBe(false);expect(validate({...law,extra:true})).toBe(false);const independent=applyPatch({cause:"native",formatted:false},[{op:"replace",path:"/cause",value:"json"}],true).newDocument;expect(independent.formatted).toBe(false);const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("pub enum EvaluationFailure");expect(source).toContain("fault:Option<EvaluationFailure>");expect(source).toContain("evaluation_admit_original(&mut self.fault,&mut self.active,grant)");expect(source).not.toContain("self.fault=Some(error.to_string())");
});
