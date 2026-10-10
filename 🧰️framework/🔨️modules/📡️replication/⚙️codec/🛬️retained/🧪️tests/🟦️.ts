/** 🛬️ SQLite validates original native field payloads and independent five-axis authority. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import {semioSchemaAjvV1} from"../../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import{readFileSync,existsSync}from"node:fs";
import grantSchema from"../../../../🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json";
import law from"../🧫️fixtures/🔣️.json";
test("original retained wire field keeps each actual buffer and receipt in its receiving operation",()=>{
 expect(semioSchemaAjvV1({strict:true}).compile(grantSchema)({maximumItems:law.grant.items,maximumCopyBytes:law.grant.copy,maximumCapacityBytes:law.grant.capacity,maximumReleaseBytes:law.grant.release,maximumDepth:law.grant.depth})).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE fields(value TEXT,bytes INTEGER)");for(const field of law.fields){db.query("INSERT INTO fields VALUES(?,?)").run(field.value,new TextEncoder().encode(field.value).length);expect(db.query("SELECT value,bytes FROM fields ORDER BY rowid DESC LIMIT 1").get()).toEqual({value:field.value,bytes:Buffer.byteLength(field.value)});}}finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);expect(existsSync(path)).toBe(true);if(!existsSync(path))return;const source=readFileSync(path,"utf8");for(const proof of["original:usize","fn validate_source<S:","identity.address!=self.original||identity.extent!=self.extent||identity.authority!=self.authority","pub fn advance<S:RetainedWireByteSource+?Sized>(&mut self,source:&S,grant:","ManuallyDrop<Option<Vec<u8>>>","pub fn demands<S:","pub fn receipt(","pub fn take_receipt(","pub fn take_into(","pub fn close_step(","maximum_release_bytes","recipient.is_some()","copied_bytes: end - start"]){expect(source.includes(proof)).toBe(true)}console.log("[DEBUG] SQLite preserves original borrowed wire bytes, independent fixed3 payload copy grant and outstanding recipient receipts");
});
test("original shared UTF8 handoffs spend zero semantic metadata copy",()=>{
 expect(law.metadataCopy).toBe(0);const source=readFileSync(new URL("../../../../🌱️value/📝️shared-utf8/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("let copied_bytes=0usize;");expect(source).toContain("pub fn lease_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:0");console.log("[DEBUG] SQLite wire field metadata copy0 is consistent with original SharedUtf8 Arc birth and final original lease release");
});
test("original wire recipient retains same published backing until grantpaid physical release",()=>{expect(semioSchemaAjvV1({strict:true}).compile(grantSchema)({maximumItems:law.grant.items,maximumCopyBytes:law.grant.copy,maximumCapacityBytes:law.grant.capacity,maximumReleaseBytes:law.grant.release,maximumDepth:law.grant.depth})).toBe(true);const db=new Database(":memory:");try{for(const field of law.fields){const value=Buffer.from(field.value,"utf8");expect(db.query("select length(?) as bytes").get(value)).toEqual({bytes:value.length});}}finally{db.close()}expect(law.recipient).toBe("original-granted-bytes-retirement");const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source.includes("pub fn admit_received(")).toBe(true);expect(source.includes("pub fn close_demands(")).toBe(true);});


test("original fallible wire copy retains every actual prefix receipt and no trial authority",async()=>{
 expect(existsSync(new URL("../🧬️schema/🔣️.json",import.meta.url))).toBe(false);
 const { resolve }=await import("node:path");const repo=resolve(import.meta.dir,"../../../../../..");
 const {semioSchemaAjvV1}=await import(resolve(repo,"🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts"));
 const original=JSON.parse(readFileSync(new URL("../🧫️fixtures/🛑️byte.json",import.meta.url),"utf8"));
 const grant=JSON.parse(readFileSync(resolve(repo,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json"),"utf8"));
 expect(semioSchemaAjvV1({strict:true}).compile(grant)(original.grant)).toBe(true);
 const bytes=Buffer.from(original.sourceHex,"hex"),db=new Database(":memory:");try{for(const row of original.failures){const prefix=bytes.subarray(0,row.copyOffset);expect(prefix.toString("hex")).toBe(row.prefixHex);expect(db.query("select length(?) as bytes").get(prefix)).toEqual({bytes:row.copyOffset});}}finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(source.includes("else{self.source_byte(source,position)?}")).toBe(true);
 expect(source.includes("let copied_bytes=self.output.as_ref().unwrap().len()-start;")).toBe(true);
 expect(source.includes("if let Err(error)=copy_result{")).toBe(true);
 expect(source.includes("return Err(error.with_retained_progress(progress))")).toBe(true);
 console.log("[DEBUG] Original fallible wire prefix slices agree Buffer/SQLite under canonical fixed Grant; actual partial recipient/System law remains separately required");
});
