/** 🔃️ SQLite independently separates schema map byte order and intrinsic occurrence order. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import fixture from"../../🧫️fixtures/🔃️ordering/🔣️.json";

test("closed schema maps sort byte keys while intrinsic occurrences retain duplicates",()=>{
 
 const database=new Database(":memory:");try{
  database.exec("CREATE TABLE map_entry(parent_key BLOB NOT NULL,entry_key BLOB NOT NULL,value INTEGER NOT NULL);CREATE TABLE literal_occurrence(ordinal INTEGER PRIMARY KEY,entry_key BLOB NOT NULL,value TEXT NOT NULL)");
  for(const row of fixture.fieldMap.unsorted)for(const entry of row.entries)database.query("INSERT INTO map_entry VALUES(?,?,?)").run(Buffer.from(row.key),Buffer.from(entry.key),entry.value);
  fixture.literalObject.forEach((entry,ordinal)=>database.query("INSERT INTO literal_occurrence VALUES(?,?,?)").run(ordinal,Buffer.from(entry.key),entry.value));
  const file=database.serialize(),reopened=Database.deserialize(file);try{
   const expected=fixture.fieldMap.canonical.flatMap(row=>row.entries.map(entry=>({parent_key:Buffer.from(row.key).toString("hex").toUpperCase(),entry_key:Buffer.from(entry.key).toString("hex").toUpperCase(),value:entry.value})));
   expect(reopened.query("SELECT hex(parent_key) AS parent_key,hex(entry_key) AS entry_key,value FROM map_entry ORDER BY parent_key,entry_key").all()).toEqual(expected);
   expect(reopened.query("SELECT hex(entry_key) AS key,value FROM literal_occurrence ORDER BY ordinal").all()).toEqual(fixture.literalObject.map(entry=>({key:Buffer.from(entry.key).toString("hex").toUpperCase(),value:entry.value})));
   expect(reopened.query("SELECT COUNT(*) AS n FROM literal_occurrence WHERE entry_key=?").get(Buffer.from("z"))).toEqual({n:2});
  }finally{reopened.close();}
 }finally{database.close();}
 console.log("[DEBUG] independent SQLite schema map ordering and duplicate intrinsic occurrence contract");
});
