import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import law from "./🔣️.json";

test("metadata aliases preserve the original owner and map values through every cancellation",()=>{
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE source(id INTEGER PRIMARY KEY,text TEXT NOT NULL);CREATE TABLE aliases(id INTEGER PRIMARY KEY,source INTEGER REFERENCES source(id));CREATE TABLE utility(window TEXT PRIMARY KEY,value TEXT NOT NULL)");
  for(const cut of [...law.cancelAfter,null]){
   const original="x".repeat(law.wideTextBytes);
   db.query("INSERT INTO source VALUES (1,?)").run(original);
   for(let i=0;i<law.mapKeys.length;i++)db.query("INSERT INTO utility VALUES (?,?)").run(law.mapKeys[i],law.utilityValues[i]);
   if(cut===null||cut>2)db.exec("INSERT INTO aliases VALUES (1,1)");
   expect(db.query("SELECT text FROM source WHERE id=1").get()).toEqual({text:original});
   expect(db.query("SELECT value FROM utility ORDER BY window").all()).toEqual(law.utilityValues.map(value=>({value})));
   db.exec("DELETE FROM aliases");
   expect(db.query("SELECT COUNT(*) AS originals FROM source").get()).toEqual({originals:1});
   db.exec("DELETE FROM utility;DELETE FROM source");
  }
 }finally{db.close()}
});
