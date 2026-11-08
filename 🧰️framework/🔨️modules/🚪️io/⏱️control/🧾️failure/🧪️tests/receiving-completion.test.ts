import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import ownerSchema from "../schema.json";
import ioSchema from "../../../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🛂️receiving-completion.json";
/** 🧾️ A published original construction refusal remains independent from an outer receipt refusal. */
test("receiving retains its published completion and outer receipt",()=>{const ajv=new Ajv({strict:true});expect(ajv.compile(ownerSchema)(fixture)).toBe(true);expect(ajv.compile({...ioSchema.$defs.NativeSnapshotFailure,$defs:ioSchema.$defs})(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE causes(channel TEXT PRIMARY KEY,message TEXT NOT NULL)");db.run("INSERT INTO causes VALUES (?,?)",["completion",fixture.cause.receiptCompletion.message]);db.run("INSERT INTO causes VALUES (?,?)",["outer",fixture.cause.receiptRefusal.message]);expect(db.query("SELECT message FROM causes ORDER BY channel").all()).toEqual([{message:fixture.cause.receiptCompletion.message},{message:fixture.cause.receiptRefusal.message}]);expect(fixture.cause.receiptCompletion.retainedProgress.copiedBytes).toBe(7)}finally{db.close()}});
