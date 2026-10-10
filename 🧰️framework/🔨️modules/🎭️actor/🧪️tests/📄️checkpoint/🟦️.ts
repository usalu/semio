import{resolveOriginalTurnInputSource}from"../../🎟️retained-turn/🔎️source/🟦️.ts";
import {existsSync as testingSchemaExists} from "node:fs";
/** 📄️ Original checkpoint publication follows separate page and ledger turns. */
import Ajv from "ajv";
import grantSchema from "../../../🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
test("actor checkpoint preserves state and progress through paid original descriptor acknowledgement",()=>{
 const fixture=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8"));
expect(testingSchemaExists(new URL("./🧬️schema/🔣️.json",import.meta.url))).toBe(false); const grant=new Ajv({strict:true}).compile(grantSchema);for(const value of [fixture.originalGrant,fixture.destinationCloseGrant])expect(grant(value)).toBe(true);
 expect(fixture.destinationCloseAuthority.kind).toBe("independentCallerWallet");
 expect(fixture.destinationCloseAuthority.walletId).not.toBe(fixture.originalSourceAuthority.walletId);
 expect(fixture.destinationCloseAuthority.renewsOriginalSource).toBe(false);
 expect(fixture.destinationCloseGrant).toEqual(fixture.originalGrant);
 expect(fixture.producerCloseAuthority.kind).toBe("independentCallerWallet");
 expect(fixture.producerCloseAuthority.renewsOriginalSource).toBe(false);
 const refusalDb=new Database(":memory:");
 refusalDb.run("CREATE TABLE original_wire(operation INTEGER CHECK(operation>0))");
 expect(()=>refusalDb.query("INSERT INTO original_wire VALUES(?)").run(fixture.invalidOriginalOperation)).toThrow();
 expect(refusalDb.query("SELECT count(*) AS n FROM original_wire").get()).toEqual({n:0});
 refusalDb.close();
 const actor=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 const original=resolveOriginalTurnInputSource(actor);
 const grantProducer=actor.slice(actor.indexOf("impl TurnGrant"),actor.indexOf("/// ⏱️ How many envelopes"));expect(grantProducer.includes(original+".validate()")).toBe(true);expect(grantProducer.indexOf(original+".validate()")).toBeLessThan(grantProducer.indexOf("self.actor.pack_encode(out)"));
 const projection=actor.slice(actor.indexOf("struct JobOutcomeProjection"),actor.indexOf("//#endregion 🪪️JobBridge"));
 expect(projection.includes("producer.borrow_outcome(&self.descriptor)")).toBe(true);
 expect(projection.includes("descriptor.acknowledge(cx.retained_grant())")).toBe(true);
 expect(projection.includes("cx.consume_retained")).toBe(true);
 expect(projection.includes("original.close_step")).toBe(false);
 expect(projection.includes("with_capacity")).toBe(false);
 expect(projection.includes("JobPayloadAuthority::admit")).toBe(true);
 expect(projection.includes("StepContext::with_payload_authority")).toBe(true);
 expect(projection.includes("StepContext::new")).toBe(false);
 const db=new Database(":memory:");db.run("CREATE TABLE phase(ordinal INTEGER PRIMARY KEY,name TEXT)");
 fixture.phases.forEach((name:string,ordinal:number)=>db.query("INSERT INTO phase VALUES(?,?)").run(ordinal,name));
 db.run("CREATE TABLE authority(wallet TEXT PRIMARY KEY,source TEXT,renews INTEGER)");
 db.query("INSERT INTO authority VALUES(?,?,?)").run(fixture.originalSourceAuthority.walletId,fixture.originalSourceAuthority.walletId,0);
 for(const authority of [fixture.destinationCloseAuthority,fixture.producerCloseAuthority])db.query("INSERT INTO authority VALUES(?,?,?)").run(authority.walletId,authority.originalSourceWalletId,Number(authority.renewsOriginalSource));
 expect(db.query("SELECT count(*) AS n FROM authority").get()).toEqual({n:3});
 expect(db.query("SELECT count(*) AS n FROM authority WHERE wallet<>source AND renews=0").get()).toEqual({n:2});
 const publications=db.query("SELECT CASE WHEN name='acknowledgement' THEN 'checkpoint' ELSE 'yield' END AS publication FROM phase ORDER BY ordinal").all().map((row:any)=>row.publication);
 expect(publications).toEqual(fixture.publications);
 const bytes=Buffer.from(fixture.state);db.run("CREATE TABLE checkpoint(state BLOB,progress INTEGER)");db.run("INSERT INTO checkpoint VALUES(?,?)",bytes,fixture.appliedProgress);
 const row=db.query("SELECT hex(state) AS state,progress FROM checkpoint").get() as {state:string;progress:number};
 expect(row.state).toBe("040506");expect(row.progress).toBe(73);
 let independent={state:[] as number[],progress:0,calls:0};
 for(const phase of fixture.phases){independent=applyPatch(independent,phase==='job'?[{op:'replace',path:'/calls',value:1}]:phase==='page'?[{op:'replace',path:'/state',value:fixture.state}]:[{op:'replace',path:'/progress',value:fixture.appliedProgress}],true).newDocument;}
 expect(independent).toEqual({state:fixture.state,progress:fixture.appliedProgress,calls:fixture.jobCalls});db.close();
 console.log(`[DEBUG] actor checkpoint SQLite/RFC6902 projection: phases=${fixture.phases.join(',')} originalWallet=${fixture.originalGrant.maximumItems}/${fixture.originalGrant.maximumCopyBytes} borrowedProducer=true destination=${fixture.destination} originalBytes=${row.state} appliedProgress=${row.progress} jobCalls=${fixture.jobCalls}`);
});


test("actual host dispatch preserves issued original authority while applying resource limits",()=>{
 const law=JSON.parse(readFileSync(new URL("./dispatch.json",import.meta.url),"utf8"));expect(testingSchemaExists(new URL("./🧬️schema/dispatch.json",import.meta.url))).toBe(false);expect(new Ajv({strict:true}).compile(grantSchema)(law.originalGrant)).toBe(true);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE issued(operation INTEGER,generation INTEGER,epoch INTEGER,items INTEGER,copy INTEGER,capacity INTEGER,release INTEGER,depth INTEGER,fuel INTEGER)");const i=law.originalInput,g=law.originalGrant;db.query("INSERT INTO issued VALUES(?,?,?,?,?,?,?,?,?)").run(i.operation,i.generation,i.epoch,g.maximumItems,g.maximumCopyBytes,g.maximumCapacityBytes,g.maximumReleaseBytes,g.maximumDepth,law.scheduledFuel);db.query("UPDATE issued SET fuel=?").run(law.hostFuel);expect(db.query("SELECT operation,generation,epoch,items,copy,capacity,release,depth,fuel FROM issued").get()).toEqual({operation:i.operation,generation:i.generation,epoch:i.epoch,items:g.maximumItems,copy:g.maximumCopyBytes,capacity:g.maximumCapacityBytes,release:g.maximumReleaseBytes,depth:g.maximumDepth,fuel:law.hostFuel});}finally{db.close();}
 const paths=["🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation/🦀️.rs","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎠️runtime/🦀️.rs"];
 for(const path of paths){const source=readFileSync(new URL("../../../../../"+path,import.meta.url),"utf8");expect(source.includes("budget_for(grant.actor, grant.budget)")).toBe(true);expect(source.includes("ShardFrame::pack_encode_grant(grant, budget, &mut self.dispatch_bytes).await")).toBe(true);expect(source.includes("budget: budget_for(grant.actor)")).toBe(false);expect(source.includes("envelopes: grant.envelopes.clone()")).toBe(false);}
 console.error("[DEBUG] Independent SQLite host CPU override preserves exact issued91/3/19 and five-axis grant; both actual native dispatch sources retain original receipt, native host execution remains separate");
});


test("original projection allocates exact complete backing before first copied page",()=>{
 const law=JSON.parse(readFileSync(new URL("./allocation.json",import.meta.url),"utf8"));expect(testingSchemaExists(new URL("./🧬️schema/allocation.json",import.meta.url))).toBe(false);const db=new Database(":memory:");try{db.exec("CREATE TABLE allocation(capacity INTEGER,allowed INTEGER)");for(const capacity of [law.oneShortCapacityBytes,law.declaredCapacityBytes])db.query("INSERT INTO allocation VALUES(?,?)").run(capacity,Number(capacity>=Buffer.from(law.originalBytes).length));expect(db.query("SELECT allowed FROM allocation ORDER BY capacity").all()).toEqual([{allowed:0},{allowed:1}]);}finally{db.close();}
 const actor=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");const projection=actor.slice(actor.indexOf("impl JobOutcomeProjection"),actor.indexOf("pub struct JobTurnBridge"));expect(projection.includes("try_reserve_exact")).toBe(false);expect(projection.includes("Layout::array::<u8>(required)")).toBe(true);expect(projection.includes("Vec::from_raw_parts(pointer.as_ptr(), 0, required)")).toBe(true);expect(projection.indexOf("grant.maximum_capacity_bytes < required")).toBeLessThan(projection.indexOf("std::alloc::alloc(layout)"));expect(projection.includes("retained_capacity_bytes: required")).toBe(true);
 console.error("[DEBUG] Original projection exact3byte capacity vs one-short2byte refusal matches independent SQLite; full original extent prepaid before allocator, one page copy per turn, same cumulative recipient");
});
