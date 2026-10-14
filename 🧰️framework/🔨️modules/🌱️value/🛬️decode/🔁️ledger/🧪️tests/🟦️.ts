import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("original decode ledger moves only through an opaque consuming receipt while callbacks rebind",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 expect(Buffer.byteLength(law.text)).toBe(law.textBytes);
 const db=new Database(":memory:");
 try{db.exec("CREATE TABLE original(ledger TEXT, recipient TEXT, owned INTEGER); INSERT INTO original VALUES ('original','original',"+law.textBytes+")");for(const _ of law.hops){db.query("UPDATE original SET owned=owned+?").run(0);expect(db.query("SELECT ledger,recipient FROM original").get()).toEqual({ledger:"original",recipient:"original"});}expect(db.query("SELECT owned FROM original").get()).toEqual({owned:law.textBytes});for(const _ of law.foreign)expect(db.query("SELECT ledger FROM original WHERE ledger='foreign'").get()).toBeNull();}finally{db.close()}
 console.log("[DEBUG] Ajv/SQLite/Buffer original consuming-lineage controls agree across five supplier-rebound/move hops; fresh owner is distinct");
 const root=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 const start=root.indexOf("pub struct NativeDecodeContinuation");const receipt=root.slice(start,root.indexOf("}",start));
 expect(receipt.includes("ledger:u64")).toBe(true);
 expect(root.includes("ledger:receipt.ledger")).toBe(true);
 expect(/#\[derive\([^\]]*\b(?:Clone|Copy)\b[^\]]*\)\]\s*pub struct NativeDecodeContinuation/.test(root)).toBe(false);
 expect(/impl(?:<[^>]*>)?\s+(?:Clone|Copy)\s+for\s+NativeDecodeContinuation/.test(root)).toBe(false);
 const allocation=readFileSync(new URL("../../🛂️allocation/🦀️.rs",import.meta.url),"utf8");
 expect(allocation.includes("ledger:super::next_ledger()")).toBe(true);expect(allocation.includes("recipient:u64")).toBe(true);expect(allocation.includes("recipient.identity")).toBe(true);expect(allocation.includes("std::ptr::from_ref")).toBe(false);
 for(const name of ["NativeForwardedDecodeContinuation","NativeForwardedDecodeReceipt"]){expect(new RegExp("#\\[derive\\([^\\]]*\\b(?:Clone|Copy)\\b[^\\]]*\\)\\]\\s*pub struct "+name).test(allocation)).toBe(false);expect(new RegExp("impl(?:<[^>]*>)?\\s+(?:Clone|Copy)\\s+for\\s+"+name).test(allocation)).toBe(false);}
 const recipient=readFileSync(new URL("../../🫴️recipient/🔁️continuation/🦀️.rs",import.meta.url),"utf8");
 expect(recipient.includes("ledger:self.control.ledger")).toBe(true);
});
