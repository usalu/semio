import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
import law from "./routing-alias.json";
import schema from "./routing-alias.schema.json";
test("original metadata routing aliases preserve the shared source across denied grants",()=>{
 expect(new Ajv({strict:true}).validate(schema,law)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE roots(id INTEGER PRIMARY KEY,body TEXT NOT NULL);CREATE TABLE metadata(id INTEGER PRIMARY KEY,root INTEGER NOT NULL REFERENCES roots(id));");
  db.query("INSERT INTO roots VALUES(1,?)").run("x".repeat(law.bodyBytes));
  db.exec("INSERT INTO metadata VALUES(1,1)");
  for(const row of law.cases){
   if(row.grant==="exact")db.exec("INSERT INTO metadata SELECT 2,root FROM metadata WHERE id=1");
   expect(db.query("SELECT COUNT(*) AS aliases FROM metadata WHERE id=2").get()).toEqual({aliases:Number(row.admitted)});
   expect(db.query("SELECT COUNT(*) AS originals FROM metadata WHERE id=1").get()).toEqual({originals:Number(row.originalRetained)});
   expect(db.query("SELECT COUNT(*)-1 AS copies FROM roots").get()).toEqual({copies:row.bodyCopies});
   db.exec("DELETE FROM metadata WHERE id=2");
  }
 }finally{db.close()}
});
