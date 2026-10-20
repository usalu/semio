import assert from "node:assert/strict";
import Ajv from "ajv";
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { valueOrderedRetirementSchemaUrl } from "../../../../../../../../../../🔨️modules/🌱️value/📦️packages/🟦️typescript/🧪️testing/🧫️fixtures/🟦️.ts";
import { jobSnapshotSchemaUrl } from "../../../../../../../../../../📦️packages/🟦️typescript/🧵️job/🧪️testing/🧫️fixtures/🟦️.ts";

/** 🧪️ Executes flow selected copy policy assertions. */
export function flowSelectedCopySelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained");
  const fixture = JSON.parse(readFileSync(join(base, "📑️copy/🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  let probes=0;
  const schema=JSON.parse(readFileSync(join(base,"🧬️schema/🔣️.json"),"utf8"));
  const grant=JSON.parse(readFileSync(valueOrderedRetirementSchemaUrl,"utf8"));
  const ajv=new Ajv({strict:true,allErrors:true}).addSchema(grant).addSchema(schema);
  const validate=ajv.compile({$ref:schema.$id+"#/$defs/OwnedFields"});
  const allocationAdmission=ajv.compile({$ref:schema.$id+"#/$defs/CopyAdmission"});assert.equal(allocationAdmission(fixture.allocationAdmission),true);assert.equal(allocationAdmission({...fixture.allocationAdmission,originalContextRequired:false}),false);assert.equal(allocationAdmission({...fixture.allocationAdmission,singleBytes:16777216}),false);probes+=3;
  const document = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8")).hostSnapshot;
  assert.equal(validate(document),true);assert.equal(validate({...document,camera:null}),false);
  const requireTest = createRequire(import.meta.url);
  const stable = requireTest("fast-json-stable-stringify");
  
  
  
  for (const test of fixture.cases) {
    const selected = test.kind === "hostSnapshot" ? document : document[test.kind === "widget" ? "widgets" : "synapses"][test.index];
    const copied = structuredClone(selected);
    const domain=test.kind==="hostSnapshot"?"OwnedFields":test.kind==="widget"?"Widget":"Synapse";
    const admit=ajv.compile({$ref:schema.$id+"#/$defs/"+domain});assert.equal(admit(copied),true);assert.equal(admit(test.kind==="hostSnapshot"?{...copied,camera:null}:{...copied,id:null}),false);
    if (stable(copied) !== stable(JSON.parse(JSON.stringify(selected)))) throw new Error("Flow selected copy third-party canonical oracle disagrees");
    const pointer = test.pointer === "" ? document : test.pointer.slice(1).split("/").reduce((value: any, key: string) => value[key], document);
    if (stable(pointer) !== stable(copied)) throw new Error("Flow selected copy fixture pointer disagrees");
  }
  const intakeSchema=JSON.parse(readFileSync(jobSnapshotSchemaUrl,"utf8"));
  const intakeCorpus=JSON.parse(readFileSync(join(base,"📑️copy/🧪️tests/🧫️fixtures/📥️intake.json"),"utf8"));
  const admitIntake=ajv.compile(intakeSchema);assert.equal(admitIntake(intakeCorpus),true);
  const altered=structuredClone(intakeCorpus);altered.cases[0].expected.sameSourceAllocation=false;assert.equal(admitIntake(altered),false);probes+=2;
  const source = readFileSync(join(base, "📑️copy/🦀️.rs"), "utf8");
  assert.ok(source.includes("factory_close:Option<Box<dyn semio_framework_value::FactoryRetirementTicket>>"),"the original parent retains the actual partially prepared ticket");
  assert.ok(source.includes("state.factory_source=rejected.original"));assert.ok(source.includes("state.factory_close=rejected.ticket"));assert.ok(source.includes("rejected.error.with_retained_progress(rejected.progress)"));
  assert.ok(source.includes("factory.factory_retirement_copy_byte_demand()"));probes+=5;
  const exact = (value: string) => value.includes("owned: ManuallyDrop<CopyState<R, T>>")
    && value.includes("source: Option<Arc<R>>") && value.includes("source_lease: Option<RetainedCloneSource<T,SnapshotRead<R>>>") && value.includes("unsafe impl<T: Sync> Send for Rooted<T>")
    && value.includes("maximum_bytes.min(source.len() - start)") && value.includes("if !state.started")
    && value.includes("state.failed=true") && value.includes("!std::thread::panicking()")
    && value.includes("state.tasks.pop_front()") && value.includes('expect("selected copy retirement factory").retire(root,grant)')
    && value.includes("admit_retained_clone_close(grant,step,terminal,owner)") && value.includes("state.root_retirement.is_none()") && value.includes("factory.preborn_factory_retirement(grant)")
    && value.includes("grant.maximum_release_bytes<d.release_bytes") && value.includes("released_bytes:bytes")
    && !value.includes("released_bytes.min(page_bytes)")
    && value.includes("target.try_reserve_exact(count)") && value.includes("grant.maximum_capacity_bytes<demand.capacity_bytes") && !value.includes("maximum_total_bytes")
    && value.includes("source: Rooted<T>") && value.includes("self.source.get().clone()")
    && !/BTreeMap|BTreeSet|\.nth\(|serde_json::to_|Arc::make_mut|target\.insert\(/.test(value);
  if (!exact(source)) throw new Error("Flow selected copy ownership source linkage missing");
  const mutants = [
    source.replace("owned: ManuallyDrop<CopyState<R, T>>", "owned: CopyState<R, T>"),
    source.replace("maximum_bytes.min(source.len() - start)", "source.len() - start"),
    source.replaceAll("state.failed=true", "state.failed=false"),
    source.replaceAll("!std::thread::panicking()", "true"),
    source.replace('expect("selected copy retirement factory").retire(root,grant)', 'expect("selected copy retirement factory").drop(root,grant)'),
    source.replace("admit_retained_clone_close(grant,step,terminal,owner)", "false"),
    source.replaceAll("grant.maximum_capacity_bytes<demand.capacity_bytes", "false"),
  ];
  for (const value of mutants) if (exact(value)) throw new Error("Flow selected copy accepted hostile ownership source");
  return fixture.cases.length + mutants.length+probes;
}
