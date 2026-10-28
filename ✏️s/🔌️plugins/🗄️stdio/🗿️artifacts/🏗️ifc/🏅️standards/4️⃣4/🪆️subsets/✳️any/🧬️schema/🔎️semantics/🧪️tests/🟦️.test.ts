import {test as register} from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {decodeIfc4} from "../../../🚪️io/🟦️.ts";
import {applyIfcMutation, inverseIfcMutation} from "../../🧬️mutations/🟦️.ts";
import {editSemantic, parseSemanticIdentity, projectSemantics, semanticIdentity} from "../🟦️.ts";
export const records: {name: string; passed: boolean; error?: string}[] = [];
export const checks: Promise<void>[] = [];
const test = (name: string, run: () => void): void => {checks.push(register(name, () => {try {run(); records.push({name, passed: true});} catch (error) {records.push({name, passed: false, error: String(error)}); throw error;}}));};
const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as {data: string; cases: {id: string; fields: Record<string,string|null>; forward: string[]; back: string[]}[]; edits: {id:string;field:string;value:unknown;index?:number;expected?:unknown;error?:string}[]};
const comparable = (value: unknown): string => JSON.stringify(value, (_key,value) => typeof value === "bigint" ? value.toString() : value);
const input = () => decodeIfc4("ISO-10303-21;HEADER;FILE_DESCRIPTION((''),'2;1');FILE_NAME('','',(''),(''),'','','');FILE_SCHEMA(('IFC4'));ENDSEC;DATA;" + fixture.data + "ENDSEC;END-ISO-10303-21;");
test("semantic IFC4 inherited attributes and reference navigation match neutral vectors", () => {
  const snapshot = input();
  for (const row of fixture.cases) {
    const projected = projectSemantics(snapshot, BigInt(row.id));
    assert.deepEqual(Object.fromEntries(projected.fields.map(field => [field.descriptor.id, field.value])), row.fields);
    assert.deepEqual(projected.forward.map(String), row.forward); assert.deepEqual(projected.back.map(String), row.back);
    for (const field of projected.fields) {assert.ok(field.descriptor.label.en); assert.ok(field.descriptor.label.de); if (field.descriptor.kind === "reference") assert.equal(field.descriptor.editable, false);}
  }
});
test("semantic edits use stable entity identity and exact central mutation inverse", () => {
  const original = input();
  for (const row of fixture.edits) {
    if (row.error) {assert.throws(() => editSemantic(original, BigInt(row.id), row.field, row.value), error => error instanceof Error && error.message === row.error); continue;}
    const edit = editSemantic(original, BigInt(row.id), row.field, row.value);
    assert.equal(edit.index, row.index); assert.deepEqual(edit.value, row.expected);
    const mutation = {mutation: "setEntityArg" as const, ...edit}, next = applyIfcMutation(original, mutation);
    assert.equal(comparable(next.entities.find(entity => entity.id === edit.id)!.args[edit.index]), comparable(row.expected));
    let reverted = next; for (const inverse of inverseIfcMutation(original, mutation)) reverted = applyIfcMutation(reverted, inverse);
    assert.equal(comparable(reverted), comparable(original));
    const reordered = {...original, entities: [...original.entities].reverse()}; assert.deepEqual(editSemantic(reordered, BigInt(row.id), row.field, row.value), edit);
  }
});
test("semantic text limits preserve Unicode and distinguish empty from unset", () => {
  const snapshot = input(); assert.equal(editSemantic(snapshot,10n,"name","😀".repeat(255)).value.kind,"string");
  assert.throws(()=>editSemantic(snapshot,10n,"name","😀".repeat(256)),/semantic-text-limit/);
  assert.throws(()=>editSemantic(snapshot,10n,"description","x".repeat(8193)),/semantic-text-limit/);
  assert.deepEqual(editSemantic(snapshot,10n,"description","").value,{kind:"string",value:""});
  assert.deepEqual(editSemantic(snapshot,10n,"description",null).value,{kind:"unset"});
});
test("semantic graph projection propagates every original cancellation", () => {
  const snapshot = input(); let work=0; projectSemantics(snapshot,10n,()=>{work++;});
  for(let stop=1;stop<=work;stop++){let count=0;assert.throws(()=>projectSemantics(snapshot,10n,()=>{if(++count===stop)throw new Error("cancelled");}),/cancelled/);assert.equal(count,stop);}
});
test("semantic identities preserve u64 and reject ambiguous aliases", () => {
  for(const id of [1n,9007199254740993n,18446744073709551615n])assert.equal(parseSemanticIdentity(semanticIdentity(id)),id);
  for(const value of ["ifc-0","ifc-01","ifc--1","ifc-1-2","ifc-18446744073709551616","1"])assert.equal(parseSemanticIdentity(value),undefined);
});

import Ajv from "ajv";
test("independent JSON Schema validator agrees with nullable semantic scalar projection", () => {
  const ajv = new Ajv(), snapshot = input();
  for(const row of fixture.cases){
    const projected=projectSemantics(snapshot,BigInt(row.id)),properties:Record<string,unknown>={},required:string[]=[];
    for(const field of projected.fields){properties[field.descriptor.id]={type:field.descriptor.optional?["string","null"]:"string",...(field.descriptor.kind==="global-id"?{pattern:"^[0-3][0-9A-Za-z_$]{21}$"}:{}),...(field.descriptor.maxCharacters?{maxLength:field.descriptor.maxCharacters}:{})};required.push(field.descriptor.id);}
    const validate=ajv.compile({type:"object",properties,required,additionalProperties:false}),values=Object.fromEntries(projected.fields.map(field=>[field.descriptor.id,field.value]));
    assert.equal(validate(values),true,ajv.errorsText(validate.errors));assert.equal(validate(row.fields),true,ajv.errorsText(validate.errors));assert.deepEqual(values,row.fields);
  }
});

test("semantic references traverse typed and complex values without guessing inherited slots", () => {
  const snapshot=input(),entity=snapshot.entities.find(row=>row.id===1n)!;
  entity.args.push({kind:"typedValue",value:{name:"CUSTOM",items:[{kind:"aggregate",value:[{kind:"reference",value:10n}]}]}});
  entity.complex=[{name:"CUSTOM",args:[{kind:"reference",value:11n}]}];
  assert.deepEqual(projectSemantics(snapshot,1n).forward,[10n,11n]);assert.deepEqual(projectSemantics(snapshot,1n).fields,[]);
  assert.deepEqual(projectSemantics(snapshot,10n).back,[1n,20n]);
});
test("large source text and malformed scalar values remain read-only diagnostic projections",()=>{
  const snapshot=input(),entity=snapshot.entities.find(row=>row.id===10n)!;
  entity.args[2]={kind:"integer",value:5n};entity.args[3]={kind:"string",value:"x".repeat(1000000)};
  const fields=projectSemantics(snapshot,10n).fields;
  assert.equal(fields.find(field=>field.descriptor.id==="name")!.state,"invalid");
  assert.deepEqual(fields.find(field=>field.descriptor.id==="description")!.value,"…");
  assert.equal(fields.find(field=>field.descriptor.id==="description")!.state,"too-large");
});

test("semantic edit planning checks original control during both lookup and global identity scans",()=>{
  const snapshot=input();for(const [field,value]of [["name","New"],["globalId","0123456789012345678907"]]){let work=0;editSemantic(snapshot,10n,field,value,()=>{work++;});for(let stop=1;stop<=work;stop++){let count=0;assert.throws(()=>editSemantic(snapshot,10n,field,value,()=>{if(++count===stop)throw new Error("cancelled");}),/cancelled/);assert.equal(count,stop);}}
});
test("unresolved references are explicit and excluded from clickable targets",()=>{
  const snapshot=input(),wall=snapshot.entities.find(entity=>entity.id===10n)!;wall.args[5]={kind:"reference",value:999n};const projected=projectSemantics(snapshot,10n);
  assert.deepEqual(projected.forward,[4n]);assert.deepEqual(projected.missing,[999n]);assert.equal(projected.fields.find(field=>field.descriptor.id==="objectPlacement")!.state,"invalid");
});
test("independent AJV scalar domain agrees with semantic edit acceptance",()=>{
  const snapshot=input(),ajv=new Ajv(),fields=projectSemantics(snapshot,10n).fields;
  const vectors=[...fixture.edits.filter(row=>row.id==="10"&&row.error!=="semantic-duplicate-global-id"),{id:"10",field:"name",value:"😀".repeat(255)},{id:"10",field:"name",value:"😀".repeat(256)}];
  for(const row of vectors){const descriptor=fields.find(field=>field.descriptor.id===row.field)?.descriptor;if(!descriptor?.editable)continue;const validate=ajv.compile({type:descriptor.optional?["string","null"]:"string",...(descriptor.maxCharacters?{maxLength:descriptor.maxCharacters}:{}),...(descriptor.kind==="global-id"?{pattern:"^[0-3][0-9A-Za-z_$]{21}$"}:{})});let accepted=true;try{editSemantic(snapshot,10n,row.field,row.value);}catch{accepted=false;}assert.equal(accepted,Boolean(validate(row.value)),row.field);}
});

test("GlobalId uniqueness also covers explicit Root constituents in complex instances",()=>{
  const snapshot=input(),entity=snapshot.entities.find(entity=>entity.id===1n)!;
  entity.complex=[{name:"IFCROOT",args:[{kind:"string",value:"0123456789012345678907"}]}];
  assert.throws(()=>editSemantic(snapshot,10n,"globalId","0123456789012345678907"),/semantic-duplicate-global-id/);
});

import {ifc4Catalog,ifc4AttributeDeclarations} from "../../📚️catalog/🟦️.ts";
test("every inherited semantic field agrees with the full normative IFC4 attribute order",()=>{
  const base=input();for(const entity of Object.values(ifc4Catalog.entities)){
    const attributes=ifc4AttributeDeclarations(entity.name)!,projection=projectSemantics({...base,entities:[{id:1n,name:entity.name,args:attributes.map(()=>({kind:"unset" as const}))}]},1n),keys=new Set<string>();
    for(const field of projection.fields){const declaration=attributes[field.descriptor.index];assert.ok(declaration,entity.name);assert.equal(declaration.name,field.descriptor.id[0].toUpperCase()+field.descriptor.id.slice(1),entity.name);assert.equal(declaration.optional,field.descriptor.optional,entity.name+"."+declaration.name);assert.equal(keys.has(field.descriptor.id),false,entity.name);keys.add(field.descriptor.id);}
  }
});
