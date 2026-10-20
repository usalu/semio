import Ajv from "ajv";
import {expect,test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import law from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("original scalar source and exact payload agree with independent codecs",()=>{
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,deniedLeaseCountChanges:1})).toBe(false);expect(validate({...law,depth:0})).toBe(false);
  for(const value of law.values){const original={value},copy=applyPatch<{scalar?:unknown}>({},[{op:"add",path:"/scalar",value:original}],true,false).newDocument.scalar;expect(copy).toEqual(original);const bytes=Buffer.alloc(law.payloadBytes);bytes.writeBigUInt64LE(BigInt(value));const independent=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);expect(independent.getBigUint64(0,true).toString()).toBe(value);expect(bytes.byteLength).toBe(BigUint64Array.BYTES_PER_ELEMENT);expect(original.value).toBe(value);}
});

test("scalar binding admission quotes the original atom while its metadata receipt remains zero",()=>{expect(law.bindingAdmissionBytes).toBe(Math.max(law.bindingCopyBytes,BigUint64Array.BYTES_PER_ELEMENT));expect(law.bindingCopyBytes).toBe(0);for(const budget of[0,7,8]){const admitted=budget>=law.bindingAdmissionBytes;expect(admitted).toBe(budget>=Buffer.alloc(law.payloadBytes).byteLength);expect(budget<law.bindingAdmissionBytes).toBe(!admitted);}});

import {Database} from "bun:sqlite";
import {scalarCloneOutputReady} from "../../🧬️contract/🔢️scalar-cursor/🟦️.ts";
test("scalar lifecycle matches an independent SQL state predicate",()=>{const db=new Database(":memory:");try{const oracle=db.query("SELECT ? AND NOT ? AND NOT ? AS ready");for(const kind of law.compounds)expect(new Set(["scalar-vector","owned-vector","optional-scalar","optional-vector","empty-option","boxed-scalar","boxed-owned","tuple"]).has(kind)).toBe(true);for(const row of law.lifecycle){expect(scalarCloneOutputReady(row)).toBe(row.ready);expect(Boolean((oracle.get(Number(row.output),Number(row.closing),Number(row.spent))as{ready:number}).ready)).toBe(row.ready);}}finally{db.close();}});

import {childCloneCloseStage} from "../../🧬️contract/🔢️scalar-cursor/🟦️.ts";
test("child close begins before a close-only quote is requested",()=>{const db=new Database(":memory:");try{const oracle=db.query("SELECT CASE WHEN NOT ? THEN 'begin' WHEN ? THEN 'retire' ELSE 'advance' END AS stage");for(const row of law.childClose){expect(childCloneCloseStage(row)).toBe(row.stage);expect((oracle.get(Number(row.started),Number(row.empty))as{stage:string}).stage).toBe(row.stage);}}finally{db.close();}});

import {cloneDepthRefused} from "../../🧬️contract/🔢️scalar-cursor/🟦️.ts";
test("structural depth denial has no allocation or release effect",()=>{const db=new Database(":memory:");try{const oracle=db.query("SELECT ? < ? AS refused, 0 AS allocatedBytes, 0 AS releasedBytes");for(const row of law.depthAdmission){expect(cloneDepthRefused(row.maximumDepth,row.requiredDepth)).toBe(row.refused);const result=oracle.get(row.maximumDepth,row.requiredDepth)as{refused:number;allocatedBytes:number;releasedBytes:number};expect(Boolean(result.refused)).toBe(row.refused);expect(result.allocatedBytes).toBe(row.allocatedBytes);expect(result.releasedBytes).toBe(row.releasedBytes);}}finally{db.close();}});

import Ajv2020 from "ajv/dist/2020";
import dispatchSchema from "../../🧬️contract/🧪️tests/♻️close/🎯️dispatch/🧬️schema/🔣️.json";
import dispatchLaw from "../../🧬️contract/🧪️tests/♻️close/🎯️dispatch/🧫️fixtures/🔣️.json";
import {selectedCloseDemand} from "../../🧬️contract/🧪️tests/♻️close/🎯️dispatch/🟦️.ts";
test("combined close dispatch selects one correlated event rather than independent axes",()=>{const validate=new Ajv2020({strict:true}).compile(dispatchSchema);expect(validate(dispatchLaw)).toBe(true);expect(validate({...dispatchLaw,maximumTurns:0})).toBe(false);const db=new Database(":memory:");try{const select=db.query("SELECT json_extract(?,CASE WHEN ?>=? THEN '$.payload' ELSE '$.structural' END) AS demand");for(const row of dispatchLaw.cases){const selected=selectedCloseDemand(row.body,row.minimumPayloadBytes,row.structural,row.payload);expect(selected).toEqual(row.selected);const result=select.get(JSON.stringify(row),row.body,row.minimumPayloadBytes)as{demand:string};expect(JSON.parse(result.demand)).toEqual(row.selected);const zero=selectedCloseDemand(0,row.minimumPayloadBytes,row.structural,row.payload),mixed={copyBytes:zero.copyBytes,capacityBytes:selected.capacityBytes,releaseBytes:zero.releaseBytes,depth:zero.depth};expect(mixed).toEqual(row.independent);expect(mixed.copyBytes===selected.copyBytes&&mixed.capacityBytes===selected.capacityBytes&&mixed.releaseBytes===selected.releaseBytes&&mixed.depth===selected.depth).toBe(row.correlated);}}finally{db.close();}});
