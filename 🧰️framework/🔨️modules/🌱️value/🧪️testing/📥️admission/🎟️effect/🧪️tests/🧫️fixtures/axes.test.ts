import {expect,test} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "./axes.json";
import schema from "./axes.schema.json";
test("original finite bank refuses every excessive grant axis before producer entry",()=>{expect(new Ajv2020({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE axis(name TEXT PRIMARY KEY,stock INTEGER,request INTEGER)");for(const row of law.refused)db.run("INSERT INTO axis VALUES(?,?,?)",[row.axis,law.stock[row.axis as keyof typeof law.stock],row.request]);expect(db.query("SELECT count(*) AS refused FROM axis WHERE request>stock").get()).toEqual({refused:5});expect(db.query("SELECT count(*) AS called FROM axis WHERE request<=stock").get()).toEqual({called:law.producerCalls});expect(law.remainingWork).toBe(law.stock.fuel);expect(law.idle).toBe(true)}finally{db.close()}});
