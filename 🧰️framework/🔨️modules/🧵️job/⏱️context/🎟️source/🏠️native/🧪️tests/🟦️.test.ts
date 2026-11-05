import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "../🔣️.json";
import corpus from "./cases.json";
test("native recipient vectors agree with independent relational custody",()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);
 const graph=new Database(":memory:");graph.run("CREATE TABLE roots(slot INTEGER,loans INTEGER)");graph.run("INSERT INTO roots VALUES(?,?),(?,?)",[corpus.graph.childSlot,corpus.graph.childLoans,corpus.graph.parentSlot,corpus.graph.parentLoans]);const callbacks:number[]=[];while(callbacks.length<2){const root=graph.query("SELECT slot FROM roots WHERE loans=0 ORDER BY slot LIMIT 1").get() as {slot:number};callbacks.push(root.slot);graph.run("DELETE FROM roots WHERE slot=?",[root.slot]);if(root.slot===corpus.graph.parentSlot)graph.run("UPDATE roots SET loans=0 WHERE slot=?",[corpus.graph.childSlot]);}expect(callbacks).toEqual(corpus.graph.closeCallbacks);graph.close();
 for(const row of corpus.cases){
  const db=new Database(":memory:");db.run("CREATE TABLE recipient(root INTEGER,owner INTEGER,children INTEGER)");db.run("INSERT INTO recipient VALUES(0,0,0)");
  const actual:string[]=[];
  for(const event of row.events){
   if(event==="birth"){db.run("UPDATE recipient SET root=1,owner=1");actual.push("admitted");}
   else if(event==="lend"){db.run("UPDATE recipient SET children=children+1 WHERE root=1");actual.push("borrowed");}
   else if(event==="foreignBorrow")actual.push("refused");
   else if(event==="cancel")actual.push("retained");
   else if(event==="dropCaller"){db.run("UPDATE recipient SET owner=0,children=0");actual.push("captured");}
   else if(event==="return"){db.run("UPDATE recipient SET owner=CASE WHEN children=0 THEN 0 ELSE owner END,children=MAX(0,children-1)");actual.push("acknowledged");}
   else{db.run("UPDATE recipient SET owner=0");const {children}=db.query("SELECT children FROM recipient").get() as {children:number};actual.push(children?"blocked":"closed");if(!children)db.run("UPDATE recipient SET root=0");}
  }
  expect(actual).toEqual(row.outputs);db.close();
 }
});
