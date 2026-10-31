/** 🔴️ The existing admission API must commit the real supplied loan when it creates an actual body. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {RetainedWorkBuffer} from "../../🟦️.ts";
import law from "./admission.json";
import schema from "./admission.schema.json";
import {wallet,run,owning} from "./admission-fixture.ts";
test("actual first body admission commits its supplied genuine accounting loan before returning",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const owner=new RetainedWorkBuffer<number>(law.elementLogicalBytes),funding=wallet();owning(owner,funding,()=>{const actual=run(funding,(work,turn)=>Reflect.apply(work.slot,work,[owner,turn]));expect(actual.error).toBeUndefined();expect(actual.result).toBe(false);expect(owner.backingFor(0)).toBeDefined();expect(actual.work.receipt.retainedCapacityBytes).toBe(law.pageElements*law.elementLogicalBytes);const db=new Database(":memory:");try{db.run("CREATE TABLE actual(action TEXT NOT NULL,work INTEGER NOT NULL)");db.run("INSERT INTO actual VALUES('actual-body-birth',?)",[actual.cpu]);expect(db.query("SELECT work AS actual,1 AS expected FROM actual").get()).toEqual({actual:1,expected:1});}finally{db.close();}});});
