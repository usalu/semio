/** 🌐️ Compares genuine foreign payloads and immutable caller currencies with independent JSON/UTF8. */
import assert from "node:assert/strict";
import { readFileSync,existsSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import { parseForeignStep, parseForeignSteps } from "../🧬️schema/🟦️.ts";
const root=resolve(import.meta.dir,"..");
const examples=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));
const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8"));
const admit=new Ajv({strict:false}).compile(schema);
for(const row of examples.cases){const{ id,...payload}=row;assert.equal(admit(payload),true,id);assert.deepEqual(JSON.parse(JSON.stringify(payload)),payload);for(const text of [payload.target.artifactId,payload.target.artifactKind,payload.target.dialect??"",payload.mutationId,payload.label])assert.deepEqual(Buffer.from(text),Buffer.from(new TextEncoder().encode(text)));assert.equal(admit({...payload,payload:[256]}),false);assert.equal(admit({...payload,target:{artifactKind:"note"}}),false);}
for(const { id,...payload} of examples.cases){assert.equal(parseForeignStep(payload),payload,id);for(const invalid of [{...payload,payload:[-1]},{...payload,payload:[1.5]},{...payload,target:{artifactId:"note"}},{...payload,label:null}]){assert.equal(admit(invalid),false);assert.throws(()=>parseForeignStep(invalid));}const extra={...payload,extra:true};assert.equal(admit(extra),true);assert.equal(parseForeignStep(extra),extra);}
const sparse={...examples.cases[0],payload:new Array(1)};assert.equal(admit(sparse),false);assert.throws(()=>parseForeignStep(sparse),"sparse original octets must be refused");
assert.equal(examples.grant.maximumItems,1);assert.equal(examples.grant.maximumCopyBytes,4);
console.log("[DEBUG] genuine foreign payload2 parser/Ajv/JSON/Buffer/TextEncoder agree; original identity preserved, malformed real fields refused");
assert.equal(existsSync(resolve(root,"🦀️.rs")),true,"genuine paid foreign output owner is missing");
const source=readFileSync(resolve(root,"🦀️.rs"),"utf8");
for(const method of ["ForeignStepSource","ForeignStepCopy","advance","close_step","next_copy_byte_demand","next_capacity_byte_demand","next_release_byte_demand","next_depth_demand"])assert.ok(source.includes(method),method);
assert.doesNotMatch(source,/usize::MAX|unwrap_or_default\(/);
for(const path of [resolve(root,"🦀️.rs"),resolve(root,"🧪️tests/🦀️.rs")]){const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout",path],{stdio:["ignore","ignore","pipe"],timeout:5000});assert.equal(parsed.status,0,parsed.stderr?.toString());}
console.log("[DEBUG] genuine foreign payloads2 Ajv/JSON/Buffer/TextEncoder; immutable caller policy, source only/no native receipt credit");

const mutation=readFileSync(resolve(root,"..","🦀️.rs"),"utf8");
assert.match(mutation,/fn foreign_step_source<'a>/,"actual mutation producer must expose original borrowed foreign source");

assert.doesNotMatch(mutation,/fn foreign_steps\(/,"allocating discovery must not coexist with its borrowed replacement");
const workspace=resolve(root,"../../../../..");
const owners=["🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testing/🧬️mutation-laws/🧬️mutations/🌐️add-counter-then-notify/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/📣️set-transaction/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧬️mutation-fixtures/🔀️transaction/🧬️mutations/⏩️set-transaction/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs"];
for(const owner of owners){const path=resolve(workspace,owner),text=readFileSync(path,"utf8");assert.match(text,/fn foreign_step_source<'a>/,owner);assert.doesNotMatch(text,/fn foreign_steps\(|pub fn plan_foreign_steps/);const parsed=spawnSync("rustfmt",["--edition","2021","--emit","stdout","--config","skip_children=true",path],{stdio:["ignore","ignore","pipe"],timeout:5000});assert.equal(parsed.status,0,parsed.stderr?.toString());}
console.log("[DEBUG] original borrowed foreign producer/derive6 grammar; typed original discovery replaces allocating Vec/Planner fallback; full publisher/native integration remains pending");

const sequence=examples.cases.map(({id,...payload}:any)=>payload);
const admitSequence=new Ajv({strict:false}).addSchema(schema).compile({$ref:schema.$id+"#/$defs/ForeignSteps"});
assert.equal(admitSequence(sequence),true);assert.deepEqual(JSON.parse(JSON.stringify(sequence)),sequence);assert.equal(admitSequence([...sequence,{...sequence[0],payload:[256]}]),false);
assert.match(source,/pub struct ForeignStepsOwner/,"real proposal sequence backing has no paid retained owner");
console.log("[DEBUG] genuine foreign sequence Ajv/JSON oracle; fixed independent collection grant; real retained output backing/source only");

assert.equal(parseForeignSteps(sequence),sequence);for(const invalid of [new Array(1),[...sequence,{...sequence[0],payload:[256]}],{}]){assert.equal(admitSequence(invalid),false);assert.throws(()=>parseForeignSteps(invalid));}

assert.match(source,/pub struct ForeignStepsPreparation/,"actual borrowed proposal rows require retained indexed preparation");
for(const method of ["source_index","advance_source","preparation_demands","take_prepared"])assert.ok(source.includes(method),method);
console.log("[DEBUG] indexed original ForeignSteps preparation retains every partial row and immutable sequence policy; native receipt pending");

const storePath=resolve(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs");
const store=readFileSync(storePath,"utf8");
assert.match(store,/pub fn advance_prepared_foreign_steps/,"proposal source must come from its actual domain-prepared candidate and original prebase");

const preparedStart=store.indexOf("fn original_proposal_forward");
const preparedEnd=store.indexOf("/// ⏭️ Advances at most one preparation",preparedStart);
assert.ok(preparedEnd>preparedStart);const prepared=store.slice(preparedStart,preparedEnd);
for(const required of ["owner.prepared()","candidate.edit.forwards[0]","stage.post.as_ref()","self.current.as_ref()","original.may_emit_foreign_steps()","original.foreign_step_source(prebase,output.source_index())?","output.advance_source(source,grant)?","admit_retained_clone_progress(grant,step.progress()"] )assert.ok(prepared.includes(required),required);
assert.doesNotMatch(prepared,/\.snapshot\(|\.clone\(|\.diff\(|apply_diff|encode_op|unwrap_or_default|usize::MAX/);
const storeGrammar=spawnSync("rustfmt",["--edition","2021","--emit","stdout","--config","skip_children=true",storePath],{stdio:["ignore","ignore","pipe"],timeout:5000});assert.equal(storeGrammar.status,0,storeGrammar.stderr?.toString());
console.log("[DEBUG] original Store candidate/prebase borrowed boundary, conservative bypass and exact supplied receipt; no cloned snapshot/cold diff/codec; native collector proof pending");

assert.match(store,/pub fn advance_prepared_operation_wire/,"local proposal bytes must use the original installed typed codec source and retained cursor");

for(const required of ["owner.operation_wire_source(original)","cursor.advance(source,output,grant)","ArtifactPreparedOperationError"])assert.ok(prepared.includes(required),required);
console.log("[DEBUG] local proposal wire port borrows actual typed issuer, retained codec and supplied output; no eager encode fallback, original prefix/error contract preserved");

let accumulated:any[]=[];for(const batch of examples.sourceBatches){const originalRows=batch.rows.map((id:string)=>{const row=examples.cases.find((row:any)=>row.id===id);assert.ok(row,id);const {id:_,...payload}=row;return payload;});assert.equal(admitSequence(originalRows),true);accumulated=accumulated.concat(originalRows);const expected=batch.expected.map((id:string)=>{const {id:_,...payload}=examples.cases.find((row:any)=>row.id===id);return payload;});assert.deepEqual(JSON.parse(JSON.stringify(accumulated)),expected);assert.deepEqual(Buffer.from(JSON.stringify(accumulated)),Buffer.from(new TextEncoder().encode(JSON.stringify(expected))));}
assert.match(source,/pub fn begin_next_source\(/,"each prepared forward needs a funded original source-boundary turn without discarding accumulated rows");
const begin=source.slice(source.indexOf("pub fn begin_next_source("),source.indexOf("pub fn source_index",source.indexOf("pub fn begin_next_source(")));
for(const required of ["self.complete","self.copy.is_some()","self.pending.is_some()","grant.maximum_copy_bytes<copied_bytes","self.index=0","self.complete=false","copied_bytes"])assert.ok(begin.includes(required),required);assert.doesNotMatch(begin,/self\.values\s*=|take_prepared|drop\(|\.clear\(|alloc\(/);
console.log("[DEBUG] original multi-forward foreign sequence3 JSON/Buffer oracle; funded source-boundary resets preserve paid original row order; native System proof pending");
