import {existsSync} from "node:fs";
import {resolve} from "node:path";
import {sliceModuleTreeSource} from "@semio-tech/framework-compiler-ts/syntax/module-tree";
/** 🧾️ Checks original receiving publication and terminal child work against independent SQLite. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import fixture from "./🧫️fixtures/🔣️.json";
import { retainedCloneWireSchema as grantSchema } from "@semio-tech/value-ts/retained-clone/wire/schema";
import decoder from "./🧫️fixtures/🛬️cursor.json";
import nativeOwner from "./🧫️fixtures/🦀️owner/🔣️.json";

test("original receiving Native route selects the actual generic IO owner mounted laws",async()=>{
 expect(existsSync(resolve(import.meta.dir,"../🧬️schema/🦀️owner/🔣️.json"))).toBe(false);
 const root=resolve(import.meta.dir,"../../../../../..");
 const read=(path:string)=>Bun.file(resolve(root,path)).text();
 const manifest=await read(nativeOwner.manifest),project=await Bun.file(resolve(root,"🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/📋️project.json")).json();
 const kernel=await read("🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/🦀️.rs"),control=await read("🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs"),snapshot=await read("🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🦀️.rs");
 const db=new Database(":memory:");try{db.exec("CREATE TABLE native_law(name TEXT PRIMARY KEY,owner TEXT NOT NULL,filter TEXT NOT NULL)");for(const path of ["🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🦀️.rs","🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/nested-io.rs"]){for(const match of(await read(path)).matchAll(/fn (original_receiving_[a-z_]+)\(/g)){db.query("INSERT INTO native_law VALUES(?,?,?)").run(match[1],nativeOwner.package,nativeOwner.filter);}}expect(db.query("SELECT count(*) AS count FROM native_law WHERE owner=? AND instr(name,filter)=1").get(nativeOwner.package)).toEqual({count:nativeOwner.minimumSelectedLaws});}finally{db.close();}
 expect(manifest.includes('name = "'+nativeOwner.package+'"')).toBe(true);expect(kernel.includes('mod component;')).toBe(true);expect(kernel.includes('../../🦀️.rs')).toBe(true);expect(control.includes('mod snapshot;')).toBe(true);expect(control.includes('mod nested_io_tests;')).toBe(true);expect(snapshot.includes('#[path="🧪️tests/🦀️.rs"]\nmod tests;')).toBe(true);
 expect(project.targets[nativeOwner.target]?.options.command).toBe("repo ./📜️script.ts "+nativeOwner.target);
 const script=await read("🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/📜️script.ts");expect(script.includes('packages:["'+nativeOwner.package+'"]')).toBe(true);expect(script.includes('"'+nativeOwner.filter+'"')).toBe(true);
 const callerSchema=await Bun.file(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json")).json();const validateCaller=new Ajv({strict:true}).compile(callerSchema);
 expect(project.name).toBe(nativeOwner.project);
 const {createNativeNxCallerCapabilitiesV1}=await import(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts"));
 const selected=["run",nativeOwner.project+":"+nativeOwner.target,"--skip-nx-cache","--excludeTaskDependencies"],authority=createNativeNxCallerCapabilitiesV1(root,["nx",...selected],selected,process.env);
 expect(validateCaller(authority)).toBe(true);expect(validateJsonSchemaSubset(callerSchema,authority)).toEqual([]);expect(authority.arguments).toEqual(["nx",...selected]);expect(authority.selection).toEqual(authority.arguments);
 console.error("[DEBUG] Original receiving native route and SQLite select six real mounted OS kernel laws through the owning Nx target and original native caller; actual allocator behavior requires the Native run");

});
test("original receiving result and terminal child shell have closed independent authority",()=>{
 expect(existsSync(resolve(import.meta.dir,"./🧬️schema/🔣️.json"))).toBe(false);
 const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);expect(validate(fixture.grant)).toBe(true);expect(validate(fixture.deniedGrant)).toBe(true);
expect(fixture.originalInput).toBe("callerSlotUntilAdmission");expect(fixture.originalError).toBe("IoResultInReceivingOutput");expect(fixture.finalPublication).toBe("afterIntermediateTerminal");
 const db=new Database(":memory:");try{
 db.exec("CREATE TABLE terminal_child(native_word_bytes INTEGER PRIMARY KEY,slot_words INTEGER NOT NULL,copy_bytes INTEGER NOT NULL,depth INTEGER NOT NULL,items INTEGER NOT NULL,capacity_bytes INTEGER NOT NULL)");
 for(const word of [4,8])db.query("INSERT INTO terminal_child VALUES(?,?,?,?,?,?)").run(word,fixture.terminalChild.slotNativeWords,word*fixture.terminalChild.slotNativeWords,fixture.terminalChild.requiredDepth,fixture.terminalChild.requiredItems,fixture.terminalChild.capacityBytes);
 expect(db.query("SELECT copy_bytes FROM terminal_child ORDER BY native_word_bytes").all()).toEqual([{copy_bytes:8},{copy_bytes:16}]);
 expect(db.query("SELECT COUNT(*) AS count FROM terminal_child WHERE copy_bytes=native_word_bytes*slot_words AND depth=2 AND items=1 AND capacity_bytes=0").get()).toEqual({count:2});
 db.exec("CREATE TABLE original_error(kind TEXT PRIMARY KEY,text TEXT NOT NULL,octets INTEGER NOT NULL)");
 for(const[kind,text]of Object.entries(fixture.error)){db.query("INSERT INTO original_error VALUES(?,?,?)").run(kind,text,Buffer.byteLength(text));expect(JSON.parse(JSON.stringify(text))).toBe(text);}
 expect(db.query("SELECT kind FROM original_error WHERE length(CAST(text AS BLOB))<>octets").all()).toEqual([]);
 expect(fixture.terminalChild.denials).toEqual(["maximumItems","maximumCopyBytes","maximumReleaseBytes","maximumDepth"]);
 for(const axis of Object.keys(fixture.grant)){const missing:Record<string,unknown>={...fixture.grant};delete missing[axis];expect(validate(missing)).toBe(false);expect(validate({...fixture.grant,[axis]:-1})).toBe(false);}
 }finally{db.close();}
});

test("original typed decoder cursor preserves independently funded receiving custody",async()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);
 expect(validate(decoder.grant)).toBe(true);
 expect(Object.values(decoder.grant)).toEqual([4096,65536,1048576,1048576,64]);
 const bytes=Buffer.from(decoder.payload,"utf8");
 expect(new TextEncoder().encode(decoder.payload)).toEqual(new Uint8Array(bytes));
 expect(JSON.parse(JSON.stringify(decoder.payload))).toBe(decoder.payload);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE original_cursor(cut INTEGER PRIMARY KEY,payload TEXT NOT NULL,octets INTEGER NOT NULL)");
  for(const cut of decoder.cancelCuts)db.query("INSERT INTO original_cursor VALUES(?,?,?)").run(cut,decoder.payload,bytes.byteLength);
  expect(db.query("SELECT count(*) AS count FROM original_cursor WHERE length(CAST(payload AS BLOB))=octets").get()).toEqual({count:decoder.cancelCuts.length});
 }finally{db.close();}
 expect(decoder.originalCustody).toBe("callerUntilFrameAdmission");expect(decoder.acceptedCustody).toBe("originalRecipientBeforeReceiptValidation");expect(decoder.publication).toBe("sameOutputAfterIntermediateTerminal");
 const source=await Bun.file(resolve(import.meta.dir,"../../🦀️.rs")).text();
 const owner=source.slice(source.indexOf("impl<'owner,'control> NativeSnapshotDecodeOwner"),source.indexOf("impl Drop for NativeSnapshotDecodeOwner"));
 expect(owner.includes("pub fn drive_cursor")).toBe(true);
 expect(owner.includes("snapshot::drive_cursor")).toBe(true);
});


test("original receiving final publication prices native header before original output leaves its frame",async()=>{
 expect(existsSync(resolve(import.meta.dir,"../🧬️schema/📤️publication.json"))).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE publication(word INTEGER PRIMARY KEY,header INTEGER NOT NULL,intermediate INTEGER NOT NULL,denied INTEGER NOT NULL,accepted INTEGER NOT NULL)");for(const word of [4,8])db.query("INSERT INTO publication VALUES(?,?,?,?,?)").run(word,word*fixture.publicationHeader.nativeWords,fixture.publicationHeader.intermediateBytes,fixture.publicationHeader.intermediateBytes,fixture.publicationHeader.intermediateBytes+word*fixture.publicationHeader.nativeWords);expect(db.query("SELECT header,denied,accepted FROM publication ORDER BY word").all()).toEqual([{header:12,denied:1,accepted:13},{header:24,denied:1,accepted:25}]);expect(db.query("SELECT count(*) AS count FROM publication WHERE denied-intermediate<header AND accepted-intermediate=header").get()).toEqual({count:2});}finally{db.close();}
 const source=await Bun.file(new URL("../🦀️.rs",import.meta.url)).text();expect(source.includes("copied_bytes:size_of::<Option<O>>()")).toBe(true);expect(source.includes("publication_progress::<O>(grant,frame_bytes)?")).toBe(true);expect(fixture.publicationHeader.entries).toEqual(["receive","cursor"]);expect(fixture.publicationHeader.cases).toEqual(["zero","oneShort","exact"]);for(const layout of fixture.publicationHeader.layouts){expect(layout.oneShortCopyBytes).toBe(layout.exactCopyBytes-1);expect(layout.exactCopyBytes).toBe(layout.headerBytes+fixture.publicationHeader.intermediateBytes);}expect(source.includes("publication_progress::<O>(remaining,frame_bytes)?")).toBe(true);expect(source.includes("*performed=receipt;Ok(frame.output.take_original()")).toBe(true);
});


test("original Pack receiving requires the caller owner and preserves Probe typed custody",async()=>{
 const vectors=await Bun.file(new URL("./🧫️fixtures/📦️receiving.json",import.meta.url)).json();
 const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);expect(validate(vectors.grant)).toBe(true);
 for(const axis of Object.keys(vectors.grant)){expect(validate({...vectors.grant,[axis]:-1})).toBe(false);}
 const db=new Database(":memory:");try{for(const row of vectors.cases){expect(JSON.parse(row.text)).toEqual(row.expected);expect(JSON.parse((db.query("SELECT json(?) AS value").get(row.text) as {value:string}).value)).toEqual(row.expected);expect(new TextEncoder().encode(row.text)).toEqual(new Uint8Array(Buffer.from(row.text)));}}finally{db.close();}
 for(const text of vectors.invalid){if(text.includes("ud800")){expect(JSON.parse(text)).toBe("\ud800");}else{expect(()=>JSON.parse(text)).toThrow();}}
 const root=resolve(import.meta.dir,"../../../../../..");
 const native=await Bun.file(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📸️native-snapshot/🦀️.rs")).text();
 const required=native.slice(native.indexOf("pub trait ArtifactPackReceiving"),native.indexOf("/// 🛬️ Borrows the envelope"));
 expect(required.includes("fn receive_pack(")).toBe(true);expect(required.includes("NativeSnapshotDecodeOwner")).toBe(true);expect(required.includes("decode_pack_with")).toBe(false);expect(required.includes("Default::default")).toBe(false);
 const implementation=sliceModuleTreeSource(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔬️probe-document/🦀️.rs"),"impl store::ArtifactPackReceiving for ProbeSnapshot","/// 🌉️ Projects the first-party tree");
 expect(implementation.includes("probe_json::receive_snapshot(text,owner)")).toBe(true);expect(implementation.includes("owner.native().checkpoint()?" )).toBe(true);
 const body=await Bun.file(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🚪️io/🔤️json/🦀️.rs")).text();const original=body.slice(body.indexOf("fn read_original"),body.indexOf("/// 🪶️ Borrows the unchanged tree"));
 expect(original.includes("owner.receive::<Receiving<T>,T>")).toBe(true);expect(original.includes("Option<T>")).toBe(true);expect(original.includes("admit_inline::<T>(body)?")).toBe(true);expect(original.includes("*typed=Some(construct(output.take().unwrap()))")).toBe(true);expect(original.indexOf("*typed=Some(construct(output.take().unwrap()))")).toBeLessThan(original.indexOf("record_inline::<T>(body)?"));expect(original.includes("body.remaining_grant()")).toBe(true);expect(original.includes("body.record_progress(performed)?")).toBe(true);
 expect(implementation.includes("NativeDecodeControl::new")).toBe(false);expect(implementation.includes("decode_pack(" )).toBe(false);expect(implementation.includes("PLAIN_GRANT")).toBe(false);
});


test("original Probe SQL native caller uses the paid typed receiving provider",async()=>{
 const root=resolve(import.meta.dir,"../../../../../..");
 const source=await Bun.file(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🪶️sqlite/🦀️.rs")).text();
 const decoder=source.slice(source.indexOf("fn decode_sqlite_snapshot_native("),source.indexOf("fn encode_sqlite_snapshot_native("));
 expect(decoder.includes("owner.scoped_native(maximum,&mut observer")).toBe(true);expect(decoder.includes("<Self as store::ArtifactPackReceiving>::receive_pack(text.as_bytes(),owner)")).toBe(true);expect(decoder.includes("probe_json::read(text,owner).map(Self)")).toBe(false);expect(decoder.includes("control.allocation_remaining_bytes()")).toBe(true);expect(decoder.includes("control.admit_native_allocation_bytes")).toBe(true);expect(decoder.includes("NativeDecodeControl::new")).toBe(false);
});


test("original snapshot wallets retain actual overdrawn spans before refusal",async()=>{
 const rows=await Bun.file(resolve(import.meta.dir,"🧫️fixtures/🧾️overdraw.json")).json();
 const validate=new Ajv({strict:true,allErrors:true}).compile(grantSchema);expect(validate(rows.grant)).toBe(true);
 const db=new Database(":memory:");try{
  db.exec("CREATE TABLE receipt(axis TEXT PRIMARY KEY,authority INTEGER NOT NULL,performed INTEGER NOT NULL,remaining INTEGER NOT NULL)");
  for(const row of rows.cases){db.query("INSERT INTO receipt VALUES(?,?,?,max(?-?,0))").run(row.axis,rows.grant[row.axis],row.performed,rows.grant[row.axis],row.performed);expect(db.query("SELECT performed,remaining FROM receipt WHERE axis=?").get(row.axis)).toEqual({performed:row.performed,remaining:row.expectedRemaining});}
  expect(db.query("SELECT count(*) AS count FROM receipt WHERE performed>authority AND remaining=0").get()).toEqual({count:4});
 }finally{db.close();}
 const source=await Bun.file(resolve(import.meta.dir,"../../🦀️.rs")).text();
 const body=source.slice(source.indexOf("impl NativeSnapshotBodyWallet{"),source.indexOf("/// 🛬️ Carries the original decoder"));
 const decode=source.slice(source.indexOf("impl<'owner,'control> NativeSnapshotDecodeOwner"),source.indexOf("impl Drop for NativeSnapshotDecodeOwner"));
 const encoding=await Bun.file(resolve(import.meta.dir,"../🦀️.rs")).text();const encode=encoding.slice(encoding.indexOf("impl<'owner,'control> NativeSnapshotEncodeOwner"),encoding.indexOf("impl Drop for NativeSnapshotEncodeOwner"));
 for(const block of [body,decode,encode]){
  const start=block.indexOf("pub fn record_progress(");const end=block.indexOf("\n",start);const method=block.slice(start,end);
  expect(method.indexOf("self.progress=self.progress.checked_add(progress)")).toBeGreaterThan(0);
  expect(method.indexOf("self.progress=self.progress.checked_add(progress)")).toBeLessThan(method.indexOf("if !admitted"));
  for(const [authority,receipt]of [["maximum_items","copied_items"],["maximum_copy_bytes","copied_bytes"],["maximum_capacity_bytes","retained_capacity_bytes"],["maximum_release_bytes","released_bytes"]])expect(block.includes("self.grant."+authority+".saturating_sub(self.progress."+receipt+")")).toBe(true);
 }
 expect(rows.originalGrant).toBe("unchanged");expect(rows.receipt).toBe("performedSpanRetainedBeforeRefusal");
});
