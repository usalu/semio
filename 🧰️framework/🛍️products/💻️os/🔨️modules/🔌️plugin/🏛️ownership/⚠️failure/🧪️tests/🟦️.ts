import {test,expect} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
type PairedCause<P>={primary:P;receipt:{kind:string;observed:number;allowed:number}|null};

test("original lifecycle retains whole typed causes and exact independent debit",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);for(const key of ["wholePrimary","separateReceiptRefusal","sameSourcePointers","noClone","noFormat","rawEffectsRecordedOnce","normalContextRequired"]){expect(validate({...fixture,[key]:false})).toBe(false);}expect(validate({...fixture,workForAdmittedZeroPhysicalProgress:0})).toBe(false);expect(validate({...fixture,terminalDrop:1})).toBe(false);for(const kind of fixture.primaryTypes){const primary=kind==="fault"?fixture.fault:fixture.value,original:PairedCause<typeof primary>={primary,receipt:null};const moved=applyPatch<{original:typeof original;held:typeof original|null}>({original,held:null},[{op:"copy",from:"/original",path:"/held"}],true,false).newDocument;const paired=applyPatch(moved,[{op:"replace",path:"/held/receipt",value:{kind:"copyRefusal",observed:fixture.overdrawCopy,allowed:fixture.copy}}],true,false).newDocument;if(!paired.held?.receipt)throw Error("independent JSONPatch did not publish the complete paired cause");expect(paired.held.primary).toEqual(primary);expect(paired.held.receipt.observed).toBe(4);expect(paired.original.receipt).toBe(null);expect(Buffer.from(JSON.stringify(paired.held.primary)).toString("utf8")).toContain("源");}
});

import {readFileSync} from "node:fs";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
test("public lifecycle failure exposes its exact defining owner and original context trait",()=>{
 const root=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8"),owner=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8"),context=readFileSync(new URL("../../../⚛️reactor/🎟️context/🌉️call/🦀️.rs",import.meta.url),"utf8");expect(root).toContain("pub use original_failure::{PluginLifecycleFailure,PluginLifecyclePrimary}");expect(owner.match(/pub struct PluginLifecycleFailure/g)?.length).toBe(1);expect(root).not.toContain("pub struct PluginLifecycleFailure");expect(owner).toContain("failure.receipt_refusal");expect(owner).toContain("failure.context_refusal");expect(context).not.toContain("T:semio_framework_value::RetireOwned");expect(context).toContain("T:semio_framework_value::retirement::RetireOwned");
});

import "../../../👷️job/🚪️session/🫴️refusal/🧪️tests/🟦️.ts";

import {Database} from "bun:sqlite";
test("original work refusal survives lifecycle and job ownership transfers",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{for(const kind of fixture.primaryTypes){
  const original={primary:kind==="fault"?fixture.fault:fixture.value,...fixture.independentRefusals};
  const transferred=applyPatch<{lifecycle:typeof original|null;job:typeof original|null}>({lifecycle:original,job:null},[{op:"move",from:"/lifecycle",path:"/job"},{op:"move",from:"/job",path:"/lifecycle"}],true,false).newDocument;
  expect(transferred.lifecycle).toEqual(original);
  const wire=JSON.stringify(original),row=db.query("SELECT json_extract(?1,'$.receipt') AS receipt,json_extract(?1,'$.context') AS context,json_extract(?1,'$.work') AS work").get(wire) as {receipt:string;context:string;work:string};
  expect(row).toEqual(fixture.independentRefusals);
 }}finally{db.close();}
});
test("mounted receiving reconciles child and whole effects once and keeps both failures",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const law=fixture.publicationReceiving,db=new Database(":memory:");try{for(const body of law.bodyPolicy){expect(body).toBeLessThanOrEqual(law.physicalCopy);for(const axis of ["items","copy","capacity","release"] as const){const child=law.child[axis],whole=law.whole[axis],oracle=db.query<{unposted:number;posted:number;outerUnposted:number},[number,number]>("SELECT ?2-?1 AS unposted,?1+(?2-?1) AS posted,?2-(?1+(?2-?1)) AS outerUnposted").get(child,whole)!;expect(oracle.unposted).toBe(law.unposted[axis]);expect(oracle.posted).toBe(whole);expect(oracle.outerUnposted).toBe(law.outerUnposted[axis]);}}for(const row of law.cases){const original={semantic:row.semantic,receipt:null as string|null};const result=applyPatch(original,[{op:"replace",path:"/receipt",value:row.receipt}],true,false).newDocument;expect(result).toEqual({semantic:row.semantic,receipt:row.receipt});expect(original.receipt).toBe(null);}}finally{db.close();}
});
test("mounted native publication uses the actual parent decoder and unchanged policy",()=>{
 const owner=readFileSync(new URL("../../../🧵️retained-command/🔄️full-operation/🦀️.rs",import.meta.url),"utf8"),source=readModuleTreeSource(new URL("../../../🧵️instance/🧾️typed-operations/🦀️.rs",import.meta.url)).replace(/\s+/g,"");
 expect(owner).toContain("source_identity:semio_framework_job::OriginalSourceBinding");for(const needle of ["original_plugin_turn_grant(self.mounted_policy.preparation,retained)","admit_apply_batch(mounted.operation.operation,mounted.artifact_generation","mounted.publication_ownership_progress=Some((retained,ownership))","letreceipt=cx.consume_receiving_retained(retained,ownership)","returnErr(primary.with_receiving_receipt(receipt))","OriginalPublicationAdvanceFailure::Domain(source)","receipt_refusal:receipt.err()","flush_published_apply_batch(publication)"])expect(source.includes(needle),needle).toBe(true);expect(source).not.toContain("self.store.advance_apply_batch(publication,grant)");
});
