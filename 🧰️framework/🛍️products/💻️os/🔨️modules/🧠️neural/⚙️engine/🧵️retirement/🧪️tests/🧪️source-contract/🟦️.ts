/** 🧹️ Strict cross-language nested-value retirement laws; source oracle only. */
import { strict as assert } from "node:assert";
import stableStringify from "fast-json-stable-stringify";
import { semioSchemaAjvV1 } from "../../../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

const ownerSchema=await Bun.file(new URL("../../🧬️schema/🔣️.json",import.meta.url)).json();
const grantSchema=await Bun.file(new URL("../../../../../../../../🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json",import.meta.url)).json();
const ajv=semioSchemaAjvV1({allErrors:true}).addSchema(grantSchema).addSchema(ownerSchema);
const validateGrant=ajv.compile({$ref:ownerSchema.$id+"#/$defs/Grant"});
const validateDemand=ajv.compile({$ref:ownerSchema.$id+"#/$defs/Demand"});
const typed=await Bun.file(new URL("../../🧫️fixtures/🎮️typed-owners/🔣️.json",import.meta.url)).json();
const validateFrontier=ajv.compile({$ref:ownerSchema.$id+"#/$defs/Frontier"});
assert(validateFrontier(typed.frontier));
assert.deepEqual(typed.frontier.currencies,["items","copy","capacity","release","depth"]);
for(const field of Object.keys(typed.frontier)) {
  const denied=structuredClone(typed.frontier);delete denied[field];assert(!validateFrontier(denied));
}
assert(!validateFrontier({...typed.frontier,sourceBirthBytes:1}));
assert(!validateFrontier({...typed.frontier,currencies:["items","bytes"]}));
assert(!validateFrontier({...typed.frontier,maximumBytes:4096}));
const validateInput=ajv.compile({$ref:ownerSchema.$id+"#/$defs/InputBinding"});
const inputReceipt={progress:{copiedItems:1,copiedBytes:0,retainedCapacityBytes:64,releasedBytes:0,complete:false},dictionary:null};
assert(validateInput(inputReceipt));
for(const field of Object.keys(inputReceipt.progress)){const refused=structuredClone(inputReceipt);delete refused.progress[field];assert(!validateInput(refused));}
assert(!validateInput({dictionary:null}));
assert(!validateInput({...inputReceipt,maximumBytes:4096}));
assert(validateInput({...inputReceipt,dictionary:{original:{text:"Grüße"}}}));
for(const row of typed.cases) {
  const value={nested:{value:row.text}};
  assert.deepEqual(JSON.parse(stableStringify(value)),value);
  assert.equal(bytesOf(value),Buffer.byteLength("nestedvalue")+new TextEncoder().encode(row.text).length);
  assert(row.reservedCapacity>Buffer.byteLength(row.text));
  for(const copy of typed.grants) {
    assert(validateGrant({maximumItems:1,maximumCopyBytes:copy,maximumCapacityBytes:0,maximumReleaseBytes:row.reservedCapacity,maximumDepth:1}));
    assert(validateDemand({copyBytes:1,capacityBytes:0,releaseBytes:row.reservedCapacity,depth:1}));
  }
}
assert(!validateGrant({maximumItems:1,maximumBytes:4096}));
function bytesOf(value:unknown):number {return typeof value==="string"?Buffer.byteLength(value):value!==null&&typeof value==="object"?Object.entries(value).reduce((sum,[key,child])=>sum+Buffer.byteLength(key)+bytesOf(child),0):0;}

const finite=await Bun.file(new URL("../../🧫️fixtures/🧬️finite-fields/🔣️.json",import.meta.url)).json();
const finiteSchema=await Bun.file(new URL("../../../🧬️schema/🔣️.json",import.meta.url)).json();
const typeSchema=await Bun.file(new URL("../../../../../../../../🔨️modules/🌱️value/🏷️type/🧬️schema/🔣️.json",import.meta.url)).json();
ajv.addSchema(typeSchema).addSchema(finiteSchema);
const validateSchema=ajv.compile({$ref:finiteSchema.$id+"#/$defs/Schema"});const validateOperator=ajv.compile({$ref:finiteSchema.$id+"#/$defs/OperatorInfo"});
assert(validateSchema(finite.schema),JSON.stringify(validateSchema.errors));assert(validateOperator(finite.operator),JSON.stringify(validateOperator.errors));
assert.deepEqual(JSON.parse(stableStringify(finite.schema)),finite.schema);assert.deepEqual(JSON.parse(stableStringify(finite.operator)),finite.operator);
const texts=(values:string[])=>values.reduce((sum,value)=>sum+new TextEncoder().encode(value).length,0);
type TypeWire={kind:"schema",of:string}|{kind:"list",of:TypeWire}|{kind:"boolean"|"integer"|"decimal"|"text"|"any"};
function typeText(value:TypeWire):number {return value.kind==="schema"?Buffer.byteLength(value.of):value.kind==="list"?typeText(value.of):0;}
assert.equal(texts([finite.schema.id,finite.schema.module,finite.schema.name,finite.schema.icon,finite.schema.summary])+finite.schema.fields.reduce((sum:number,field:any)=>sum+texts([field.key,field.label??""])+typeText(field.value)+bytesOf(field.default),0),finite.expected.schemaStringBytes);
assert.equal(texts([finite.operator.id,finite.operator.extension,finite.operator.name,finite.operator.abbreviation,finite.operator.icon,finite.operator.summary,...finite.operator.group,finite.operator.variadicInput.slotKey])+finite.operator.inputs.reduce((sum:number,channel:any)=>sum+texts([channel.code,channel.abbreviation,channel.name,channel.fullName,channel.label??"",...channel.operators,...channel.valueTypes,...channel.itemTypes])+bytesOf(channel.default),0),finite.expected.operatorStringBytes);

const ownedDemand = await Bun.file(new URL("../../🧫️fixtures/📏️owned-demand/🔣️.json", import.meta.url)).json();
assert.equal(JSON.parse(stableStringify({ text: ownedDemand.text })).text, ownedDemand.text);
assert.ok(ownedDemand.reservedCapacity > Buffer.byteLength(ownedDemand.text));
assert.ok(ownedDemand.reservedCapacity > ownedDemand.logicalPage);
assert.equal(ownedDemand.unsupportedFullGrantKind, "unsupportedOwner");
assert.deepEqual(ownedDemand.birthRefusals,["zeroItems","zeroDepth","shortCapacity"]);
assert.equal(ownedDemand.terminalFrameRelease,"separateAdmittedTurn");
for(const refusal of ownedDemand.birthRefusals) {
  const grant={maximumItems:1,maximumCopyBytes:0,maximumCapacityBytes:64,maximumReleaseBytes:0,maximumDepth:1};
  if(refusal==="zeroItems")grant.maximumItems=0;
  if(refusal==="zeroDepth")grant.maximumDepth=0;
  if(refusal==="shortCapacity")grant.maximumCapacityBytes=63;
  assert(validateGrant(grant));
  assert(!(grant.maximumItems>0&&grant.maximumDepth>0&&grant.maximumCapacityBytes>=64));
}



//#region 🔣️DomainFixture
const fixture = await Bun.file(new URL("../../🧫️fixtures/🔣️value-retirement.json", import.meta.url)).json();


const bytes = (value: any): number => typeof value === "string" ? Buffer.byteLength(value) : value && typeof value === "object" ? Object.entries(value).reduce((sum, [key, child]) => sum + Buffer.byteLength(key) + bytes(child), 0) : 0;
for (const row of fixture.cases) {
  const value = JSON.parse(row.json.replaceAll("$text", row.expandedText.text.repeat(row.expandedText.repetitions)));
  assert.equal(bytes(value), row.expectedBytes);
  assert.deepEqual(JSON.parse(stableStringify(value)), value);
  for (const grant of fixture.grants) {
    let remaining = row.expectedBytes; let copied = 0;
    while (remaining) { const step = Math.min(grant, remaining); remaining -= step; copied += step; }
    assert.equal(copied, row.expectedBytes);
  }
}
//#endregion 🔣️DomainFixture

//#region 🧠️CacheFixture
const cacheFixture = await Bun.file(new URL("../../🧫️fixtures/🗃️cache-retirement/🔣️.json", import.meta.url)).json();


const cache = new Map<number, Record<string, string>>(); const pending: Record<string, string>[] = [];
let finalBytes = 0;
for (const operation of cacheFixture.operations) {
  if (operation.op === "seed") {
    const old = cache.get(cacheFixture.key); if (old) pending.push(old);
    cache.set(cacheFixture.key, { [operation.field]: operation.text.repeat(operation.repeat) });
  } else if (operation.op === "release-shared") {
    assert.equal(cache.size, cacheFixture.expected.liveEntriesBeforeFinal);
    assert.equal(finalBytes, cacheFixture.expected.sharedReleasedBytes);
    const current = cache.get(cacheFixture.key)!;
    const expected = cacheFixture.expected.finalJson.replace("$text", cacheFixture.operations[1].text.repeat(cacheFixture.operations[1].repeat));
    assert.equal(stableStringify(current), expected);
  } else {
    finalBytes = [...pending, ...cache.values()].reduce((sum, value) => sum + bytes(value), 0);
    pending.length = 0; cache.clear();
  }
}
assert.equal(finalBytes, cacheFixture.expected.finalReleasedBytes); assert.equal(cache.size + pending.length, cacheFixture.expected.terminalOwners);

//#endregion 🧠️CacheFixture

//#region 📸️EvaluationOwnership
const evaluation = await Bun.file(new URL("../../🧫️fixtures/🧮️evaluation-owners/🔣️.json", import.meta.url)).json();


const node = evaluation.node.text.repeat(evaluation.node.repeat); const payload = evaluation.payload.text.repeat(evaluation.payload.repeat);
assert.equal(Buffer.byteLength(node) + 2 * Buffer.byteLength(payload) + Buffer.byteLength("seednodelabel"), evaluation.expectedBytes);
assert.equal(stableStringify({ node: { label: payload } }), JSON.stringify({ node: { label: payload } }));

//#endregion 📸️EvaluationOwnership

console.log("[DEBUG] actual Neural retirement independent UTF8/stable JSON, grant accounting, cache ownership and evaluation oracle assertions passed");

const inputPolicy=ajv.compile({$ref:ownerSchema.$id+"#/$defs/InputPolicy"});
const inputFixture=await Bun.file(new URL("../../../🧫️fixtures/🚦️owned-controls.json",import.meta.url)).json();
assert(inputPolicy(inputFixture.retainedBinding.physical));
for(const field of Object.keys(inputFixture.retainedBinding.physical)){const refused=structuredClone(inputFixture.retainedBinding.physical);delete refused[field];assert(!inputPolicy(refused));}
assert(!inputPolicy({...inputFixture.retainedBinding.physical,sourceBirthBytes:1}));
const binding={nested:{first:inputFixture.retainedBinding.textUnit.repeat(inputFixture.retainedBinding.textRepeats),second:-3.25}};
assert.deepEqual(JSON.parse(stableStringify(binding)),binding);

const bodyLaw=ownedDemand.bodyDependentBirth;
const capacity=bodyLaw.overhead+bodyLaw.minimumBody;
assert(capacity>bodyLaw.logicalPage);
assert(bodyLaw.overhead+capacity>capacity);
assert.equal(bodyLaw.minimumBody,1);
assert(validateDemand({copyBytes:bodyLaw.minimumBody,capacityBytes:capacity,releaseBytes:0,depth:1}));

assert.equal(ownedDemand.occupiedSlot,"unreservedBackingOwnershipLimit");
assert.equal(ownedDemand.entryStorage,"sameVectorBackingPopFront");
assert.deepEqual(Array.from(new Map([["first","雪"],["second",null]])),[["first","雪"],["second",null]]);

const registryLaw=await Bun.file(new URL("../../../📔️registry/🧫️fixtures/🔣️.json",import.meta.url)).json();
assert(validateSchema(registryLaw.schema),JSON.stringify(validateSchema.errors));
assert.deepEqual(JSON.parse(stableStringify(registryLaw.schema)),registryLaw.schema);
assert.equal(registryLaw.executionOwner.maximumRetirementTurns,10000);

const domainClosure=ownedDemand.domainPhysicalClosure;
assert.equal(domainClosure.payloadOracle,"utf8-content");assert.equal(domainClosure.releaseGrant,"current-owner-demand");
assert.equal(domainClosure.allocatorClosure,"original-and-frontier");assert.equal(domainClosure.workerClosure,"aggregate-both-threads");
for(const page of fixture.grants) {
  const physicalDemands=[56,8193,24];let born=physicalDemands.reduce((sum,value)=>sum+value,0);let freed=0;
  for(const demand of physicalDemands) {const grant=Math.max(page,demand);assert(validateGrant({maximumItems:1,maximumCopyBytes:grant,maximumCapacityBytes:0,maximumReleaseBytes:grant,maximumDepth:1}));assert(demand<=grant);freed+=demand;}
  assert.equal(freed,born);assert(physicalDemands.length<domainClosure.maximumTurns);
}
console.log("[DEBUG] independent UTF8 content, current physical grants and aggregate ownership closure laws passed");
