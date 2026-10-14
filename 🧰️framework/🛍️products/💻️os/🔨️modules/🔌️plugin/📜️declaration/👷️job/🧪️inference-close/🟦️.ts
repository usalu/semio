import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json";
import law from "./🧫️fixtures/🔣️.json";
test("Original inference close retains the concrete source and pays every independent owner",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE original_session(operation INTEGER,generation INTEGER,payload TEXT,closed INTEGER)");db.query("INSERT INTO original_session VALUES(?,?,?,0)").run(law.operation,law.generation,law.payload);expect(db.query("SELECT payload FROM original_session WHERE operation=? AND generation=?").get(71,4)).toBeNull();expect(db.query("SELECT payload FROM original_session WHERE operation=? AND generation=?").get(71,3)).toEqual({payload:law.payload});expect(Buffer.byteLength(law.payload)).toBe(18);}finally{db.close();}const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");for(const name of ["OriginalInferenceSessionOwner","ControlledRetirement<T>","close_granted","original_inference_close_demands","original_inference_close_step"])expect(source).toContain(name);expect(source).not.toContain("sessions: BTreeMap<&'static str, Box<dyn std::any::Any + Send>>");console.log("[DEBUG] Original inference schema/Ajv/SQLite same71/3 source identity; typed native close is separately qualified");
});

test("Original second inference alias retains the peer and final typed source until full paid close",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE alias_custody(alias TEXT,source TEXT,live INTEGER)");db.query("INSERT INTO alias_custody VALUES(?,?,?)").run("command",law.payload,1);db.query("INSERT INTO alias_custody VALUES(?,?,?)").run("peer",law.payload,1);db.query("UPDATE alias_custody SET live=0 WHERE alias=?").run("command");expect(db.query("SELECT source FROM alias_custody WHERE live=1").get()).toEqual({source:law.payload});expect(db.query("SELECT sum(live) AS count FROM alias_custody").get()).toEqual({count:1});}finally{db.close();}const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(source).toContain("inference_owned:ManuallyDrop<Option<std::sync::Mutex<InstanceInferenceSessions>>>");expect(source).toContain("std::sync::Arc::into_inner(self.inference.take().unwrap())");expect(source).toContain("sessions.close_granted(grant)");expect(source).toContain("original instance alias abandoned inferred owner custody");console.log("[DEBUG] second original inference Arc SQLite peer/source custody oracle; native exact Arc/System close remains separately qualified");
});
