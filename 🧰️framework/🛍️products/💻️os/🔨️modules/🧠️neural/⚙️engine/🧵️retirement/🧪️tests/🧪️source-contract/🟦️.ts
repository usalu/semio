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



//#region 🔣️DomainFixture
const fixture = await Bun.file(new URL("../../🧫️fixtures/🔣️value-retirement.json", import.meta.url)).json();


const bytes = (value: any): number => typeof value === "string" ? Buffer.byteLength(value) : value && typeof value === "object" ? Object.entries(value).reduce((sum, [key, child]) => sum + Buffer.byteLength(key) + bytes(child), 0) : 0;
for (const row of fixture.cases) {
  const value = JSON.parse(row.json.replaceAll("$text", row.expandedText.text.repeat(row.expandedText.repetitions)));
  assert.equal(bytes(value), row.expectedBytes);
  assert.deepEqual(JSON.parse(stableStringify(value)), value);
  for (const grant of fixture.grants) {
    let remaining = row.expectedBytes; let released = 0;
    while (remaining) { const step = Math.min(grant, remaining); remaining -= step; released += step; }
    assert.equal(released, row.expectedBytes);
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
