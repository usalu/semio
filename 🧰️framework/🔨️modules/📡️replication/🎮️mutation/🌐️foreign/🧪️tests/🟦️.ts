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
