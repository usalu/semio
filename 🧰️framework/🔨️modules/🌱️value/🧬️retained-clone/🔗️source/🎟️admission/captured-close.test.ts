import {expect,test} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "./captured-close.json";
import schema from "./captured-close.schema.json";
test("captured alias closure retains its typed header until a separate funded clear",()=>{expect(new Ajv2020({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE phase(name TEXT,work INTEGER,alias INTEGER,pointer INTEGER)");db.run("INSERT INTO phase VALUES(?,?,?,?),(?,?,?,?),(?,?,?,?)",["alias",law.aliasWork,0,law.pointerWords,"denied",law.deniedWork,0,law.pointerWords,"header",law.headerWork,0,0]);expect(db.query("SELECT sum(work) AS work,max(alias) AS alias,min(pointer) AS terminal FROM phase").get()).toEqual({work:2,alias:0,terminal:0});expect(db.query("SELECT pointer FROM phase WHERE name=?").get("denied")).toEqual({pointer:1});expect(law.allocations+law.releases).toBe(0)}finally{db.close()}});
