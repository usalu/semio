import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./scope.schema.json";
import fixture from "./scope.json";
/** 🧵️ The same source dispatch and each actual authority must rejoin a pending physical effect. */
test("pending scoped encoding keeps all original authority bindings",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");try{db.exec("CREATE TABLE pending(allocation INTEGER,work INTEGER,construction INTEGER,source INTEGER,transaction_id INTEGER,authority_source INTEGER)");for(const row of fixture.cases){db.exec("DELETE FROM pending");db.query("INSERT INTO pending VALUES(1,1,1,?,?,?)").run(row.capturedTurn,1,row.authorityTurn);const found=db.query("SELECT 1 FROM pending WHERE allocation=? AND work=? AND construction=? AND source=? AND transaction_id=? AND source=authority_source").get(row.allocation,row.work,row.construction,row.sourceTurn,row.transaction);expect(found!==null).toBe(row.accepted)}expect(db.query("SELECT count(*) AS n FROM pending").get()?.n).toBe(1)}finally{db.close()}});
