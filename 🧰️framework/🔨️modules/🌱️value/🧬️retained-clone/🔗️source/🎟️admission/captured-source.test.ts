import {expect,test} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "./captured-source.json";
import schema from "./captured-source.schema.json";
test("typed captured dispatch keeps actual child, callback mode, source and path",()=>{expect(new Ajv2020({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{db.run("CREATE TABLE candidate(name TEXT,child TEXT,mode TEXT,source TEXT,path TEXT)");db.run("INSERT INTO candidate VALUES('sameChild','first','receive','original','root'),('foreignChild','second','receive','original','root'),('wrongMode','first','recover','original','root'),('foreignSource','first','receive','other','root'),('foreignPath','first','receive','original','nested')");const rows=db.query("SELECT name,(child='first' AND mode='receive' AND source='original' AND path='root') AS accepted FROM candidate").all() as {name:keyof typeof law,accepted:number}[];for(const row of rows)expect(Boolean(row.accepted)).toBe(law[row.name]);expect(law.proofWork).toBe(1)}finally{db.close()}});
