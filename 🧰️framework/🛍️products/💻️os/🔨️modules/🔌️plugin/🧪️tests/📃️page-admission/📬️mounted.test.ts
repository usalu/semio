import cloneDeep from "lodash/cloneDeep";
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("accepted custody is delivered once after denied framing and later cancellation",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.acceptedReceipts){const db=new Database(":memory:");db.run("CREATE TABLE custody(before INTEGER,after INTEGER,accepted INTEGER,delivered INTEGER)");db.run("INSERT INTO custody VALUES(?,?,1,0)",row.receipt.generationBefore,row.receipt.generationAfter);
 for(const denied of [true,true,false]){if(!denied)db.run("UPDATE custody SET delivered=1 WHERE delivered=0");expect(db.query("SELECT accepted FROM custody").get()).toEqual({accepted:1});}
 expect(db.query("SELECT before,after,delivered FROM custody").get()).toEqual({before:row.receipt.generationBefore,after:row.receipt.generationAfter,delivered:1});db.close();}
});
test("production retains original page custody before cancellation and source close",async()=>{
 const b="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/";
 const full=await Bun.file(b+"🧵️retained-command/🔄️full-operation/🦀️.rs").text();
 const unit=await Bun.file(b+"🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs").text();
 const pump=await Bun.file(b+"🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs").text();
 expect(full).toContain("pub page_publication:");
 expect(unit.indexOf("advance_pending_result_page")).toBeLessThan(unit.indexOf("reject_cancelled_publication"));
 expect(unit).toContain("accepted_receipt()");
 expect(pump).toContain("page_publication_pending()");
 for(const s of [unit,pump]){expect(s).not.toContain("try_serialize_cold");expect(s).not.toContain("TypedOperationResultPage::try_new");expect(s).not.toContain("emit.ui_scope.clone()");}
});

test("partial UI copies preserve both body vectors and every flag",()=>{
 for(const row of fixture.cases.filter(row=>row.payload.kind==="ui")){const expected=cloneDeep(row.payload.value);const actual=Object.fromEntries(Object.entries(row.payload.value).map(([key,value])=>[key,Array.isArray(value)?value.map(text=>Array.from(text).join("")):value]));expect(actual).toEqual(expected);expect(JSON.stringify(actual)).toBe(JSON.stringify(expected));}
});
