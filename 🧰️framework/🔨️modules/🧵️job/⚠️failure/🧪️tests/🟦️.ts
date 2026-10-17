import {test} from "bun:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import {Database} from "bun:sqlite";
import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
import type {ValueRefusalKind,ValueErrorRetainedProgress} from "../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";

/** 📦️ Neutral admission rows preserve the whole domain source using an independent patch oracle. */
test("original domain failure admits only its whole physical frame",()=>{
 const base=resolve(import.meta.dirname,".."),corpus=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/📦️domain.json"),"utf8")),schema=JSON.parse(readFileSync(resolve(base,"🧬️schema/📦️domain.json"),"utf8")),validate=new Ajv({strict:true}).compile(schema);assert.equal(validate(corpus),true,JSON.stringify(validate.errors));
 for(const row of corpus.cases){const transfer=row.items===1&&row.capacity>0&&row.depth>0,initial={source:corpus.domain,primary:null};const observed=applyPatch(initial,transfer?[{op:"move",from:"/source",path:"/primary"}]:[],true,false).newDocument;assert.equal(transfer,row.transfer);assert.deepEqual(transfer?observed.primary:observed.source,corpus.domain);assert.equal(BigInt(corpus.domain.operand),9007199254740993n);}
});

/** 🧬️ The actual canonical transport borrows exact domain integers and octets with independent SQLite evidence. */
test("original domain fault transport preserves intrinsic source and separate receipts",async()=>{
 const {faultFromOriginalTurnFailure}=await import("@semio-tech/framework-diagnostic-ts/retained"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts"),corpus=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/📦️domain.json"),"utf8")),bytes=Uint8Array.of(0,255,3,128),operand=BigInt(corpus.domain.operand);
 const original:Extract<IntrinsicValue,{kind:"object"}>={kind:"object",members:[{name:"operand",value:{kind:"unsigned",value:operand}},{name:"bytes",value:{kind:"bytes",value:bytes}}]},primary={code:corpus.domain.code as string,message:corpus.domain.message as string,retainedProgress:corpus.domain.retainedProgress as ValueErrorRetainedProgress,original};
 const secondary=new ValueError("workLimit","Original receipt 源").withRetainedProgress({copiedItems:2,copiedBytes:5,retainedCapacityBytes:13,releasedBytes:17}),observed=faultFromOriginalTurnFailure({primary:{kind:"domain",domain:primary},receiptRefusal:secondary});
 assert.equal(observed.domain,original);assert.equal(observed.retainedProgress,primary.retainedProgress);assert.equal(observed.code,primary.code);assert.equal(observed.message,primary.message);assert.equal(observed.causes?.[0]?.retainedProgress,secondary.retainedProgress);assert.equal(observed.domain?.members[1]?.value,original.members[1]?.value);
 const db=new Database(":memory:",{safeIntegers:true});try{const row=db.query("SELECT ? AS operand, ? AS bytes").get(operand,bytes) as {operand:bigint;bytes:Uint8Array};assert.equal(row.operand,9007199254740993n);assert.deepEqual([...row.bytes],[...bytes]);}finally{db.close()}
});

/** 🧯️ Independent original failure accounting preserves semantic and receipt causes without cloning. */
test("original turn failure retains primary and receipt refusal",()=>{
 const base=resolve(import.meta.dirname,".."),repo=resolve(base,"../../../.."),read=(path:string|URL)=>JSON.parse(readFileSync(path,"utf8")),ajv=new Ajv({strict:true,allErrors:true});
 ajv.addSchema(read(valueOrderedRetirementSchemaUrl));
 ajv.addSchema(read(cloneWireProgressSchemaUrl),"flow.original-retained-progress");
 const corpus=read(resolve(base,"🧪️tests/🧫️fixtures/🔣️.json")),validate=ajv.compile(read(resolve(base,"🧬️schema/🔣️.json")));assert.equal(validate(corpus),true,JSON.stringify(validate.errors));let checks=1;
 for(const key of Object.keys(corpus)){const malformed={...corpus};delete malformed[key];assert.equal(validate(malformed),false);checks++;}assert.equal(validate({...corpus,alternateFailure:true}),false);checks++;
 for(const row of corpus.cases){const progress=row.primary.retainedProgress,excess=progress.copiedItems>row.grant.maximumItems||progress.copiedBytes>row.grant.maximumCopyBytes||progress.retainedCapacityBytes>row.grant.maximumCapacityBytes||progress.releasedBytes>row.grant.maximumReleaseBytes;const observed=applyPatch({primary:row.primary,receiptRefusal:null,recorded:{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0}},[{op:"replace",path:"/recorded",value:progress},{op:"replace",path:"/receiptRefusal",value:excess?"invariantViolated":null}],true,false).newDocument;assert.deepEqual(observed.primary,row.primary);assert.deepEqual(observed.recorded,progress);assert.equal(observed.receiptRefusal,row.expectedReceiptKind);assert.deepEqual([...Buffer.from(row.primary.message)],[...new TextEncoder().encode(row.primary.message)]);checks+=4;}
 const source=readFileSync(resolve(base,"🦀️.rs"),"utf8");assert(source.includes("pub primary:OriginalTurnPrimary")&&source.includes("pub receipt_refusal:Option<ValueError>")&&source.includes("Fault(Fault)"),"defining original failure owns complete value or diagnostic causes");assert(source.includes("receipt_refusal:cx.consume_retained(primary.retained_progress()).err()"),"failure debit retains the actual secondary refusal");assert(!source.includes("derive(Clone")&&!source.includes("format!("),"the original failure transfer does not clone or format");checks+=3;
});

/** 📬️ Canonical fault transport preserves both complete causes and distinct actual receipts. */
test("original turn failure projects both distinct receipts on normal fault wire",async()=>{
 const diagnostic=await import("@semio-tech/framework-diagnostic-ts/retained"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const original=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).wire,primary=new ValueError(original.primary.kind,original.primary.message).withRetainedProgress(original.primary.retainedProgress),secondary=new ValueError(original.receiptRefusal.kind,original.receiptRefusal.message).withRetainedProgress(original.receiptRefusal.retainedProgress);
 const pair={primary:{kind:"value" as const,value:primary},receiptRefusal:secondary},fault=diagnostic.faultFromOriginalTurnFailure(pair),wire=JSON.parse(JSON.stringify(fault));assert.equal(fault.retainedProgress,primary.retainedProgress);assert.equal(fault.causes?.[0]?.retainedProgress,secondary.retainedProgress);assert.deepEqual(wire.causes,[{message:secondary.message,code:secondary.kind,retainedProgress:secondary.retainedProgress}]);assert.equal(wire.message,primary.message);assert.equal(wire.code,primary.kind);
 const repo=resolve(import.meta.dirname,"../../../../.."),schema=JSON.parse(readFileSync(diagnosticControlledSchemaUrl,"utf8")),valueSchema=JSON.parse(readFileSync(valueRefusalSchemaUrl,"utf8"));const {default:Ajv2020}=await import("ajv/dist/2020");const validate=new Ajv2020({strict:true}).addSchema(valueSchema).compile(schema);assert.equal(validate(wire),true,JSON.stringify(validate.errors));
});

/** 🧳️ A complete application diagnostic and its own causes survive the normal fault projection. */
test("original diagnostic primary preserves every canonical field and receipt cause",async()=>{
 const {faultFromOriginalTurnFailure}=await import("@semio-tech/framework-diagnostic-ts/retained"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const original=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).faultWire,primary=original.primary,secondary=new ValueError(original.receiptRefusal.kind,original.receiptRefusal.message).withRetainedProgress(original.receiptRefusal.retainedProgress);
 assert.equal(faultFromOriginalTurnFailure({primary:{kind:"fault",fault:primary}}),primary);
 const observed=faultFromOriginalTurnFailure({primary:{kind:"fault",fault:primary},receiptRefusal:secondary});
 const expected=applyPatch(primary,[{op:"add",path:"/causes/-",value:{message:secondary.message,code:secondary.kind,retainedProgress:secondary.retainedProgress}}],true,false).newDocument;
 assert.deepEqual(observed,expected);assert.equal(observed.scope,primary.scope);assert.equal(observed.span,primary.span);assert.equal(observed.source,primary.source);assert.equal(observed.params,primary.params);assert.equal(observed.causes?.[0],primary.causes[0]);assert.equal(observed.retainedProgress,primary.retainedProgress);assert.equal(observed.causes?.[1]?.retainedProgress,secondary.retainedProgress);
 const repo=resolve(import.meta.dirname,"../../../../.."),schema=JSON.parse(readFileSync(diagnosticControlledSchemaUrl,"utf8"));const{default:Ajv2020}=await import("ajv/dist/2020");const validate=new Ajv2020({strict:true}).compile(schema);assert.equal(validate(observed),true,JSON.stringify(validate.errors));
});

/** 🪢️ The separate context refusal joins both complete original causes on the canonical wire. */
test("original failure wire retains independent context and receipt refusals",async()=>{
 const {faultFromOriginalTurnFailure}=await import("@semio-tech/framework-diagnostic-ts/retained"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const original=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).contextWire,make=(row:{kind:ValueRefusalKind;message:string;retainedProgress:ValueErrorRetainedProgress})=>new ValueError(row.kind,row.message).withRetainedProgress(row.retainedProgress),receipt=make(original.receiptRefusal),context=make(original.contextRefusal),primary=original.primary;
 const observed=faultFromOriginalTurnFailure({primary:{kind:"fault",fault:primary},receiptRefusal:receipt,contextRefusal:context});
 const expected=applyPatch(primary,[receipt,context].map(cause=>({op:"add" as const,path:"/causes/-",value:{message:cause.message,code:cause.kind,retainedProgress:cause.retainedProgress}})),true,false).newDocument;
 assert.deepEqual(observed,expected);assert.equal(observed.scope,primary.scope);assert.equal(observed.source,primary.source);assert.equal(observed.causes?.[1]?.retainedProgress,receipt.retainedProgress);assert.equal(observed.causes?.[2]?.retainedProgress,context.retainedProgress);
 const only=faultFromOriginalTurnFailure({primary:{kind:"fault",fault:primary},contextRefusal:context});assert.equal(only.causes?.at(-1)?.message,context.message);
});

import "../📦️domain/🤝️closure/🧪️tests/🟦️.ts";
import { diagnosticControlledSchemaUrl } from "@semio-tech/framework-diagnostic-ts/testing/fixtures";
import { valueRefusalSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";
import { valueOrderedRetirementSchemaUrl, cloneWireProgressSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";
