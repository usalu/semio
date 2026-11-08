import { test, expect } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import cases from "./🧫️fixtures/🔣️.json";

test("persistent splice fixture agrees with SQLite rank edits",()=>{
 expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);
 const db=new Database(":memory:");
 try{db.exec("CREATE TABLE source(rank INTEGER PRIMARY KEY,value INTEGER NOT NULL)");
  for(const row of cases){db.exec("DELETE FROM source");row.before.forEach((value,rank)=>db.run("INSERT INTO source VALUES (?,?)",[rank,value]));
   const query=row.kind==="insert"?"SELECT value FROM (SELECT rank*2 AS position,value FROM source UNION ALL SELECT ?1*2-1,?2) ORDER BY position":"SELECT value FROM source WHERE rank<>?1 ORDER BY rank";
   const result=(row.kind==="insert"?db.query(query).all(row.index,row.authored):db.query(query).all(row.index)) as {value:number}[];
   expect(result.map(entry=>entry.value)).toEqual(row.expected);
  }
 }finally{db.close()}
});
