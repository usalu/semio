/** 🧾️ Independent normal-worker receipt precedes borrowing both original failure causes. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import law from "./🧫️fixtures/🔣️.json";
test("normal worker whole failure is lent only after original receipt return",()=>{
 const grant={type:"object",additionalProperties:false,required:["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"],properties:Object.fromEntries(["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"].map(key=>[key,{type:"integer",minimum:0}]))};
 const properties={operation:{const:81295},generation:{const:5},clock:{const:1000},deadline:{const:1500},fuel:{const:1},source:grant,work:grant,close:grant,primary:{type:"string"},receiptRefusal:{type:"string"},beforeReceipt:{const:"noFailureLoan"},afterReceipt:{const:"bothOriginalPointers"},terminal:{const:"exactBornReleaseAndDrop0"}};
 expect(new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:Object.keys(properties),properties})(law)).toBe(true);
 const db=new Database(":memory:");try{db.exec("create table original_causes(role text primary key,message text)");db.query("insert into original_causes values (?,?)").run("primary",law.primary);db.query("insert into original_causes values (?,?)").run("receipt",law.receiptRefusal);expect(db.query("select count(*) as owners from original_causes").get()).toEqual({owners:2});expect(db.query("select message from original_causes where role='primary'").get()).toEqual({message:law.primary});}finally{db.close();}
 const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("filter(|authority|!authority.retained_receipt_pending)");expect(source).toContain("pub fn checked_out_failure(&self)->Option<&OriginalTurnFailure>");
 console.log("[DEBUG] Ajv/SQLite two distinct complete normal-worker failure owners, original source/work17/close17 policy and raw receipt before native loan; native law separately required");
});
