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
 const {faultFromOriginalTurnFailure}=await import("../🟦️.ts"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts"),corpus=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/📦️domain.json"),"utf8")),bytes=Uint8Array.of(0,255,3,128),operand=BigInt(corpus.domain.operand);
 const original:Extract<IntrinsicValue,{kind:"object"}>={kind:"object",members:[{name:"operand",value:{kind:"unsigned",value:operand}},{name:"bytes",value:{kind:"bytes",value:bytes}}]},primary={code:corpus.domain.code as string,message:corpus.domain.message as string,retainedProgress:corpus.domain.retainedProgress as ValueErrorRetainedProgress,original};
 const secondary=new ValueError("workLimit","Original receipt 源").withRetainedProgress({copiedItems:2,copiedBytes:5,retainedCapacityBytes:13,releasedBytes:17}),observed=faultFromOriginalTurnFailure({effects:primary.retainedProgress,primary:{kind:"domain",domain:primary},receiptRefusal:secondary});
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
 const diagnostic=await import("../🟦️.ts"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const original=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).wire,primary=new ValueError(original.primary.kind,original.primary.message).withRetainedProgress(original.primary.retainedProgress),secondary=new ValueError(original.receiptRefusal.kind,original.receiptRefusal.message).withRetainedProgress(original.receiptRefusal.retainedProgress);
 const pair={effects:primary.retainedProgress,primary:{kind:"value" as const,value:primary},receiptRefusal:secondary},fault=diagnostic.faultFromOriginalTurnFailure(pair),wire=JSON.parse(JSON.stringify(fault));assert.equal(fault.retainedProgress,primary.retainedProgress);assert.equal(fault.causes?.[0]?.retainedProgress,secondary.retainedProgress);assert.deepEqual(wire.causes,[{message:secondary.message,code:secondary.kind,retainedProgress:secondary.retainedProgress}]);assert.equal(wire.message,primary.message);assert.equal(wire.code,primary.kind);
 const repo=resolve(import.meta.dirname,"../../../../.."),schema=JSON.parse(readFileSync(diagnosticControlledSchemaUrl,"utf8")),valueSchema=JSON.parse(readFileSync(valueRefusalSchemaUrl,"utf8"));const {default:Ajv2020}=await import("ajv/dist/2020");const validate=new Ajv2020({strict:true}).addSchema(valueSchema).compile(schema);assert.equal(validate(wire),true,JSON.stringify(validate.errors));
});

/** 🧳️ A complete application diagnostic and its own causes survive the normal fault projection. */
test("original diagnostic primary preserves every canonical field and receipt cause",async()=>{
 const {faultFromOriginalTurnFailure}=await import("../🟦️.ts"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const original=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).faultWire,primary=original.primary,secondary=new ValueError(original.receiptRefusal.kind,original.receiptRefusal.message).withRetainedProgress(original.receiptRefusal.retainedProgress);
 assert.equal(faultFromOriginalTurnFailure({effects:primary.retainedProgress,primary:{kind:"fault",fault:primary}}),primary);
 const observed=faultFromOriginalTurnFailure({effects:primary.retainedProgress,primary:{kind:"fault",fault:primary},receiptRefusal:secondary});
 const expected=applyPatch(primary,[{op:"add",path:"/causes/-",value:{message:secondary.message,code:secondary.kind,retainedProgress:secondary.retainedProgress}}],true,false).newDocument;
 assert.deepEqual(observed,expected);assert.equal(observed.scope,primary.scope);assert.equal(observed.span,primary.span);assert.equal(observed.source,primary.source);assert.equal(observed.params,primary.params);assert.equal(observed.causes?.[0],primary.causes[0]);assert.equal(observed.retainedProgress,primary.retainedProgress);assert.equal(observed.causes?.[1]?.retainedProgress,secondary.retainedProgress);
 const repo=resolve(import.meta.dirname,"../../../../.."),schema=JSON.parse(readFileSync(diagnosticControlledSchemaUrl,"utf8"));const{default:Ajv2020}=await import("ajv/dist/2020");const validate=new Ajv2020({strict:true}).compile(schema);assert.equal(validate(observed),true,JSON.stringify(validate.errors));
});

/** 🪢️ The separate context refusal joins both complete original causes on the canonical wire. */
test("original failure wire retains independent context and receipt refusals",async()=>{
 const {faultFromOriginalTurnFailure}=await import("../🟦️.ts"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const original=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).contextWire,make=(row:{kind:ValueRefusalKind;message:string;retainedProgress:ValueErrorRetainedProgress})=>new ValueError(row.kind,row.message).withRetainedProgress(row.retainedProgress),receipt=make(original.receiptRefusal),context=make(original.contextRefusal),primary=original.primary;
 const observed=faultFromOriginalTurnFailure({effects:primary.retainedProgress,primary:{kind:"fault",fault:primary},receiptRefusal:receipt,contextRefusal:context});
 const expected=applyPatch(primary,[receipt,context].map(cause=>({op:"add" as const,path:"/causes/-",value:{message:cause.message,code:cause.kind,retainedProgress:cause.retainedProgress}})),true,false).newDocument;
 assert.deepEqual(observed,expected);assert.equal(observed.scope,primary.scope);assert.equal(observed.source,primary.source);assert.equal(observed.causes?.[1]?.retainedProgress,receipt.retainedProgress);assert.equal(observed.causes?.[2]?.retainedProgress,context.retainedProgress);
 const only=faultFromOriginalTurnFailure({effects:primary.retainedProgress,primary:{kind:"fault",fault:primary},contextRefusal:context});assert.equal(only.causes?.at(-1)?.message,context.message);
});

import "../📦️domain/🤝️closure/🧪️tests/🟦️.ts";
import { diagnosticControlledSchemaUrl } from "@semio-tech/framework-diagnostic-ts/testing/fixtures";
import { valueRefusalSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";
import { valueOrderedRetirementSchemaUrl, cloneWireProgressSchemaUrl } from "@semio-tech/value-ts/testing/fixtures";

/** 🛬️ Every original receiving role survives transport alongside both enclosing refusals. */
test("original receiving keeps all five simultaneous causes and actual effects",async()=>{
 const {faultFromOriginalTurnFailure}=await import("../🟦️.ts"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts");
 const wire=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).receivingWire;
 const make=(row:{kind:ValueRefusalKind;message:string;retainedProgress:ValueErrorRetainedProgress})=>new ValueError(row.kind,row.message).withRetainedProgress(row.retainedProgress),original=wire.primary.original,primary=make(original.primary),quoteRefusal=make(original.quoteRefusal),allocationRefusal=make(original.allocationRefusal),receipt=make(wire.receiptRefusal),context=make(wire.contextRefusal);
 const fault=faultFromOriginalTurnFailure({effects:original.retainedProgress,primary:{kind:"receiving",receiving:{primary,quoteRefusal,allocationRefusal,retainedProgress:original.retainedProgress}},receiptRefusal:receipt,contextRefusal:context});
 assert.equal(fault.retainedProgress,original.retainedProgress);assert.equal(fault.message,primary.message);assert.deepEqual(fault.domain?.members.map(member=>member.name),["primary","quoteRefusal","allocationRefusal","retainedProgress"]);
 for(const [index,cause]of [primary,quoteRefusal,allocationRefusal].entries()){const member:IntrinsicValue|undefined=fault.domain?.members[index]?.value;assert(member?.kind==="object");assert.deepEqual(member.members.slice(0,2).map(field=>field.value),[{kind:"text",value:cause.kind},{kind:"text",value:cause.message}]);}
 assert.deepEqual(fault.causes?.map(cause=>cause.message),[receipt.message,context.message]);assert.equal(fault.causes?.[0]?.retainedProgress,receipt.retainedProgress);assert.equal(fault.causes?.[1]?.retainedProgress,context.retainedProgress);
 const db=new Database(":memory:");try{const row=db.query("SELECT json_extract(?, '$.primary.original.primary.message') AS semantic, json_extract(?, '$.primary.original.quoteRefusal.message') AS quote, json_extract(?, '$.primary.original.allocationRefusal.message') AS allocation").get(...Array(3).fill(JSON.stringify(wire))) as {semantic:string;quote:string;allocation:string};assert.deepEqual([row.semantic,row.quote,row.allocation],[primary.message,quoteRefusal.message,allocationRefusal.message]);}finally{db.close()}
 const repo=resolve(import.meta.dirname,"../../../../.."),projection=readFileSync(resolve(import.meta.dirname,"../🫴️projection/🦀️.rs"),"utf8"),invoke=readFileSync(resolve(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/📨️invoke/⚠️failure/🦀️.rs"),"utf8");
 assert(projection.includes("impl super::OriginalDomainProjection for OriginalReceivingPrimary"),"the defining Receiving owner lends its exact nested projection");
 const receivingOwner=readFileSync(resolve(repo,"🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🦀️.rs"),"utf8"),jobFacade=readFileSync(resolve(import.meta.dirname,"../🛬️receiving/🦀️.rs"),"utf8");
 assert(receivingOwner.includes("pub struct OriginalReceivingPrimary"),"native construction owns all three inline causes below Job");
 assert(jobFacade.includes("pub use semio_framework_value::native_decoding::OriginalReceivingPrimary")&&!jobFacade.includes("pub struct OriginalReceivingPrimary"),"Job reexports the actual lower cause without duplicating its body");
 assert(invoke.includes("Some(receiving as &dyn FieldProjectionSource)"),"actual extension failure output borrows the defining Receiving source");
 assert(invoke.includes('("original.receiving",receiving.primary.as_ref()')&&invoke.includes('Self::Progress(error.retained_progress())'),"normal extension wire keeps Receiving code and its own physical effects");
 console.log("[DEBUG] receiving primaryQuoteAllocationReceiptContext=true distinctOriginalFields=true actualProgress=true independentSQLite=true");
});

/** 🧾️ A complete turn span stays independent of its last primary and both enclosing refusal owners. */
test("original turn effect span preserves last semantic receipt and cumulative accounting",async()=>{
 const {faultFromOriginalTurnFailure}=await import("../🟦️.ts"),{ValueError}=await import("../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts"),law=JSON.parse(readFileSync(resolve(import.meta.dirname,"🧫️fixtures/🔣️.json"),"utf8")).effectSpan;
 const primary=new ValueError("invalidValue",law.primary).withRetainedProgress(law.last),receipt=new ValueError("invariantViolated",law.receipt),context=new ValueError("workLimit",law.context),failure={effects:law.whole,primary:{kind:"value" as const,value:primary},receiptRefusal:receipt,contextRefusal:context};
 const db=new Database(":memory:");try{for(const key of Object.keys(law.whole)){const row=db.query("SELECT ? + ? AS whole, ? - ? AS unposted").get(law.prior[key],law.last[key],law.whole[key],law.whole[key])as{whole:number;unposted:number};assert.equal(row.whole,law.whole[key]);assert.equal(row.unposted,0)}}finally{db.close()}
 const fault=faultFromOriginalTurnFailure(failure);assert.equal(fault.retainedProgress,law.whole);assert.equal(primary.retainedProgress,law.last);assert.equal(failure.primary.value,primary);assert.deepEqual(fault.causes?.map(cause=>cause.message),[law.receipt,law.context]);
 const wire=JSON.parse(JSON.stringify(fault));assert.deepEqual(wire.retainedProgress,law.whole);assert.deepEqual(primary.retainedProgress,law.last);assert.notDeepEqual(wire.retainedProgress,primary.retainedProgress);
 const owner=readFileSync(resolve(import.meta.dirname,"../🦀️.rs"),"utf8"),projection=readFileSync(resolve(import.meta.dirname,"../🫴️projection/🦀️.rs"),"utf8");assert(owner.includes("pub effects:super::RetainedCloneProgress"));assert(owner.includes("pub fn retained_progress(&self)->super::RetainedCloneProgress{self.effects}"));assert(projection.includes('"effects"=>Self::Progress(f.effects)'));
 console.log("[DEBUG] same original primary owner and last receipt; cumulative turn effects independently projected; SQLite sum/subtract and JSON transport agree; native law separate");
});
