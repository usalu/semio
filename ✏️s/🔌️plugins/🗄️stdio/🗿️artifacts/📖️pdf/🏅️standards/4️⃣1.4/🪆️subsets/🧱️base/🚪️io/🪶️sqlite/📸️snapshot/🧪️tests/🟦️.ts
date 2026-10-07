/** 📖️ PDF1.4 owned dimensions, ordered page text and independent SQL laws. */
import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { parsePdfSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PDF14_SQLITE_SCHEMA, pdf14SnapshotToSqliteDatabase, pdf14SnapshotFromSqliteDatabase } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("PDF1.4 complete concrete backing contract keeps all page words and literal text", async () => {
  const plan = await Bun.file(new URL("../🧫️fixtures/🔢ieee.json", import.meta.url)).json();
  expect(plan.backing).toEqual({ authority: "completeSystemAllocatorRequests", phases: ["projectSnapshot", "reconstructSnapshot"], ceilings: ["zero", "exact", "oneBelow", "cumulative"], cancellation: ["start", "materializedInterior"], diagnosticOwnership: "actualCapacity", retirementRefund: false });
  const { default: Ajv } = await import("ajv/dist/2020");
  
  
  expect(plan.backing["authority"]).toEqual("completeSystemAllocatorRequests");expect(plan.backing["phases"]).toEqual(["projectSnapshot","reconstructSnapshot"]);expect(plan.backing["ceilings"]).toEqual(["zero","exact","oneBelow","cumulative"]);expect(plan.backing["cancellation"]).toEqual(["start","materializedInterior"]);expect(plan.backing["diagnosticOwnership"]).toEqual("actualCapacity");expect(plan.backing["retirementRefund"]).toEqual(false);
  for (const invalid of [{ ...plan.backing, extra: true }, { ...plan.backing, retirementRefund: true }, { ...plan.backing, authority: "estimatedSlots" }, { ...plan.backing, cancellation: ["start"] }]) 
  for (const item of plan.binary64) {
    const raw = BigInt(`0x${item.bits}`);
    const buffer = Buffer.alloc(8); buffer.writeBigUInt64BE(raw);
    const decoded = buffer.readDoubleBE();
    expect(Number.isNaN(decoded) ? "nan" : decoded === Infinity ? "positiveInfinity" : decoded === -Infinity ? "negativeInfinity" : "finite").toBe(item.class);
    const expected = { schema: "literal\0世界", pages: [{ width: { bits: raw }, height: { bits: raw }, text: "page\0😀" }] };
    const sql = Database.deserialize(await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(expected)), { safeIntegers: true });
    try {
      expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(sql.query("SELECT ordinal,text,CAST(width_bits AS TEXT) AS bits FROM pdf14_page").get()).toEqual({ ordinal: 0n, text: expected.pages[0]!.text, bits: BigInt.asIntN(64, raw).toString() });
      expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()))).toEqual(expected);
    } finally { sql.close(); }
  }
});

test("PDF1.4 native-neutral fixture is complete and independently editable", async () => {
  expect(PDF14_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const snapshot = parsePdfSnapshot(await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json());
  const bytes = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
  const sql = Database.deserialize(bytes);
  expect(sql.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
  expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(sql.query("SELECT ordinal,text FROM pdf14_page ORDER BY ordinal").all()).toEqual(snapshot.pages.map((page, ordinal) => ({ ordinal, text: page.text })));
  expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
  sql.run("UPDATE pdf14_page SET text='Independent Ω edit' WHERE ordinal=1");
  const restored = await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()));
  expect(restored.schema).toBe(snapshot.schema);
  expect(restored.pages[1]!.text).toBe("Independent Ω edit");
  sql.close();
});

test("PDF1.4 preserves every neutral IEEE word without numeric canonicalization", async () => {
  const cases = await Bun.file(new URL("../🧫️fixtures/🔢ieee.json", import.meta.url)).json();
  for (const item of cases.binary64) {
    const scalar = { bits: BigInt(`0x${item.bits}`) };
    const snapshot = { schema: "pdf14.custom-schema", pages: [{ width: scalar, height: scalar, text: item.bits }] };
    const bytes = await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(snapshot));
    expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);
    const sql = Database.deserialize(bytes);
    expect(sql.query("SELECT CAST(width_bits AS TEXT) AS bits,width_class AS class,width IS NULL AS nullQuery FROM pdf14_page").get()).toEqual({ bits: BigInt.asIntN(64,scalar.bits).toString(), class: item.class, nullQuery: item.class === "nan" ? 1 : 0 });
    sql.close();
  }
});

test("PDF1.4 rejects noncontiguous pages, weakened schema and bounded cancellation", async () => {
  const snapshot = parsePdfSnapshot(await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json());
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { maxRows: 1 })).rejects.toThrow();
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { maxValueBytes: 1 })).rejects.toThrow();
  const controller = new AbortController(); controller.abort();
  await expect(pdf14SnapshotToSqliteDatabase(snapshot, { signal: controller.signal })).rejects.toThrow();
  const valid = await pdf14SnapshotToSqliteDatabase(snapshot);
  const changed = { tables: valid.tables.map(table => table.name === "pdf14_page" ? { ...table, rows: table.rows.map((row, index) => index === 0 ? { ...row, values: row.values.map((value, index) => index === 2 ? 999n : value) } : row) } : table) };
  await expect(pdf14SnapshotFromSqliteDatabase(changed)).rejects.toThrow();
  const weakened = { tables: valid.tables.map(table => ({ ...table, sql: table.sql.replace("schema TEXT NOT NULL", "schema TEXT \"NOT NULL\"") })) };
  await expect(pdf14SnapshotFromSqliteDatabase(weakened)).rejects.toThrow();
});

import { Buffer } from "node:buffer";
test("PDF14 own Set literal frame independently retains all accepted page words",async()=>{
  const fixture=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🧫️fixtures/📸️set-patch-exact-words/🔣️.json",import.meta.url)).json();
  expect(fixture).toEqual({"schema":"stdio.pdf","binaryPayload":"u64-le schema UTF8 length + schema bytes + u64-le page count; each page width u64 word + height u64 word + u64-le text UTF8 length + text bytes","textPayload":"original own14 snapshot DSL with exact binary64 token spelling","pages":[{"widthBits":"0","heightBits":"4607182418800017408","text":"positive zero"},{"widthBits":"9223372036854775808","heightBits":"4607182418800017408","text":"negative zero"},{"widthBits":"9218868437227405312","heightBits":"4607182418800017408","text":"positive infinity"},{"widthBits":"18442240474082181120","heightBits":"4607182418800017408","text":"negative infinity"},{"widthBits":"9221120237041090626","heightBits":"4607182418800017408","text":"quiet payload Ω\u0000"},{"widthBits":"9218868437227405313","heightBits":"4607182418800017408","text":"signaling payload"},{"widthBits":"1","heightBits":"4607182418800017408","text":"least subnormal"},{"widthBits":"9218868437227405311","heightBits":"4607182418800017408","text":"maximum finite"}],"identityRefusal":{"nextSchema":"foreign\u0000schema","path":"/schema","originalBaseSchemaPreserved":true},"patch":{"pointer":"/pages/0/text","before":"positive zero","after":"edited Ω\u0000","inverseRestoresAllWords":true},"preservedOriginalTags":[0,1,2,3,4],"newTags":{"set-snapshot":5,"patch-snapshot":6},"binaryFrameHex":"01050900000000000000737464696f2e70646608000000000000000000000000000000000000000000f03f0d00000000000000706f736974697665207a65726f0000000000000080000000000000f03f0d000000000000006e65676174697665207a65726f000000000000f07f000000000000f03f1100000000000000706f73697469766520696e66696e697479000000000000f0ff000000000000f03f11000000000000006e6567617469766520696e66696e697479420000000000f87f000000000000f03f11000000000000007175696574207061796c6f616420cea900010000000000f07f000000000000f03f11000000000000007369676e616c696e67207061796c6f61640100000000000000000000000000f03f0f000000000000006c65617374207375626e6f726d616cffffffffffffef7f000000000000f03f0e000000000000006d6178696d756d2066696e697465"});
  const pieces:Buffer[]=[Buffer.from([1,5])],word=(value:bigint)=>{const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(value);pieces.push(bytes);},text=(value:string)=>{const bytes=Buffer.from(value,"utf8");word(BigInt(bytes.length));pieces.push(bytes);};
  text(fixture.schema);word(BigInt(fixture.pages.length));for(const page of fixture.pages){word(BigInt(page.widthBits));word(BigInt(page.heightBits));text(page.text);}expect(Buffer.concat(pieces).toString("hex")).toBe(fixture.binaryFrameHex);
  const owner={schema:fixture.schema,pages:fixture.pages.map((page:{widthBits:string;heightBits:string;text:string})=>({width:{bits:BigInt(page.widthBits)},height:{bits:BigInt(page.heightBits)},text:page.text}))};
  const bytes=await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(owner)),sql=Database.deserialize(bytes);expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const restored=await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(bytes));expect(restored).toEqual(owner);sql.close();
});

import { applySnapshotPatch, inverseSnapshotPatches } from "../../../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts";

test("PDF14 snapshot path contract independently agrees with fast-json-patch and exact owner words",async()=>{
  const {applyPatch}=await import("fast-json-patch");
  const fixture=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🧫️fixtures/📸️set-patch-exact-words/🔣️.json",import.meta.url)).json();
  const base={schema:fixture.schema,pages:fixture.pages.map((page:{widthBits:string;heightBits:string;text:string})=>({width:{bits:BigInt(page.widthBits)},height:{bits:BigInt(page.heightBits)},text:page.text}))};
  const original=structuredClone(base),replacement="independent Ω\0patch";
  const independent=applyPatch(structuredClone(base),[{op:"replace",path:"/pages/0/text",value:replacement}],true,true).newDocument;
  const patch={operation:"set" as const,path:"/pages/0/text",value:replacement};const subject=applySnapshotPatch(base,patch);expect(subject).toEqual(independent);expect(base).toEqual(original);
  let restored=subject;for(const inverse of inverseSnapshotPatches(base,patch))restored=applySnapshotPatch(restored,inverse);expect(restored).toEqual(original);
  const frame=Buffer.from(fixture.binaryFrameHex,"hex");expect(frame.subarray(0,2)).toEqual(Buffer.from([1,5]));let at=2;const word=()=>{const value=frame.readBigUInt64LE(at);at+=8;return value;},text=()=>{const count=Number(word()),value=frame.subarray(at,at+count).toString("utf8");at+=count;return value;};expect(text()).toBe(base.schema);expect(word()).toBe(BigInt(base.pages.length));for(const page of base.pages){expect(word()).toBe(page.width.bits);expect(word()).toBe(page.height.bits);expect(text()).toBe(page.text);}expect(at).toBe(frame.length);
  const sql=Database.deserialize(await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(base)),{safeIntegers:true});try{expect(sql.query("SELECT CAST(width_bits AS TEXT) AS widthBits,CAST(height_bits AS TEXT) AS heightBits,text FROM pdf14_page ORDER BY ordinal").all()).toEqual(base.pages.map((p:{width:{bits:bigint};height:{bits:bigint};text:string})=>({widthBits:BigInt.asIntN(64,p.width.bits).toString(),heightBits:BigInt.asIntN(64,p.height.bits).toString(),text:p.text})));}finally{sql.close();}
});

test("PDF14 populated page UI corpus is closed and independently SQL editable",async()=>{
  const fixture=await Bun.file(new URL("../../../../✏️editor/\u{1f9eb}️fixtures/\u{1f4c4}️resolved-page-domain/\u{1f523}️.json",import.meta.url)).json();
  
  expect(fixture["dialect"]).toEqual({"artifactKind":"s.stdio.pdf","standard":"1.4","subset":"*"});expect(fixture["schema"]).toEqual("stdio.pdf");expect(fixture["pages"]).toEqual([{"width":200,"height":300,"text":"Semio page one"},{"width":400,"height":500,"text":"Seite zwei"}]);expect(fixture["setPage"]).toEqual({"page":0,"item":0,"text":"Edited own14 text"});expect(fixture["geometryEdit"]).toEqual({"index":1,"width":8,"height":9});expect(fixture["invalidAddress"]).toEqual({"page":2,"item":0});expect(fixture["invalidItem"]).toEqual(1);expect(fixture["nativeHeader"]).toEqual("%PDF-1.4");expect(fixture["forbiddenOwnFields"]).toEqual(["mediaBox","cropBox","rotate","objects","trailer"]);expect(fixture["requiredActionVocabulary"]).toEqual(["set-page","insert-page","remove-page","move-page","set-page-size","replace-page-text"]);expect(fixture["artifactSchemaId"]).toEqual("s.stdio.pdf");expect(fixture["sqlEdit"]).toEqual({"page":1,"width":8,"height":9,"text":"Independent page edit"});
  
  const owner=parsePdfSnapshot(fixture),sql=Database.deserialize(await exportSqliteDatabase(await pdf14SnapshotToSqliteDatabase(owner)));
  try{
    expect(sql.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(sql.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(sql.query("SELECT p.ordinal,p.width,p.height,p.text FROM pdf14_document d JOIN pdf14_page p ON p.document_id=d.id ORDER BY p.ordinal").all()).toEqual(fixture.pages.map((p:{width:number;height:number;text:string},ordinal:number)=>({ordinal,...p})));
    const expected=parsePdfSnapshot({...fixture,pages:fixture.pages.map((p:unknown,index:number)=>index===fixture.sqlEdit.page?{width:fixture.sqlEdit.width,height:fixture.sqlEdit.height,text:fixture.sqlEdit.text}:p)});
    const word=(value:number)=>{const bytes=Buffer.alloc(8);bytes.writeDoubleLE(value);return bytes.readBigInt64LE().toString();};
    sql.query("UPDATE pdf14_page SET width=?,height=?,text=?,width_bits=CAST(? AS INTEGER),height_bits=CAST(? AS INTEGER),width_class='finite',height_class='finite' WHERE ordinal=?").run(fixture.sqlEdit.width,fixture.sqlEdit.height,fixture.sqlEdit.text,word(fixture.sqlEdit.width),word(fixture.sqlEdit.height),fixture.sqlEdit.page);
    expect(await pdf14SnapshotFromSqliteDatabase(await importSqliteDatabase(sql.serialize()))).toEqual(expected);
    expect(owner).toEqual(parsePdfSnapshot(fixture));
  }finally{sql.close();}
});
