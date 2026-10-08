/** 🧮️ Literal byte-edit histories agree with independent SQLite ordinal mutation. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";

const owner=resolve(import.meta.dir,"../.."),read=(path:string)=>JSON.parse(readFileSync(resolve(owner,path),"utf8"));
const vectors=[
 {kind:"✂️replace-byte-range",id:"🧮️middle",mutation:{mutation:"replaceByteRange",offset:1,remove_len:2,insert:[17,0,255]},before:[0,64,128,255],after:[0,17,0,255,255]},
 {kind:"➕️append-bytes",id:"🧮️tail",mutation:{mutation:"appendBytes",data:[17,255]},before:[0,64,128,255],after:[0,64,128,255,17,255]},
 {kind:"🔪️truncate-at",id:"🧮️prefix",mutation:{mutation:"truncateAt",offset:2},before:[0,64,128,255],after:[0,64]}
];

test("Binary committed byte histories preserve independent ordinal splice intent",()=>{
 const Ajv=require("ajv").default,ajv=new Ajv({strict:false});
 for(const row of vectors){
  const schema=read(`🧬️schema/🧬️mutations/${row.kind}/🧬️schema/🔣️.json`);
  expect(ajv.compile(schema)(row.mutation)).toBe(true);
  const db=new Database(":memory:");
  try{
   db.run("CREATE TABLE octets(ordinal INTEGER PRIMARY KEY,value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255))");
   row.before.forEach((value,index)=>db.run("INSERT INTO octets VALUES(?,?)",[index,value]));
   const offset="offset" in row.mutation?row.mutation.offset:row.before.length;
   const remove="remove_len" in row.mutation?row.mutation.remove_len:row.mutation.mutation==="truncateAt"?row.before.length-offset:0;
   const inserted="insert" in row.mutation?row.mutation.insert:"data" in row.mutation?row.mutation.data:[];
   db.run("DELETE FROM octets WHERE ordinal>=? AND ordinal<?",[offset,offset+remove]);
   db.run("UPDATE octets SET ordinal=-ordinal-1 WHERE ordinal>=?",[offset+remove]);
   db.run("UPDATE octets SET ordinal=-ordinal-1+? WHERE ordinal<0",[inserted.length-remove]);
   inserted.forEach((value,index)=>db.run("INSERT INTO octets VALUES(?,?)",[offset+index,value]));
   expect(db.query("SELECT value FROM octets ORDER BY ordinal").all()).toEqual(row.after.map(value=>({value})));
   const file=`🧫️fixtures/🧬️history-edits/${row.kind}/${row.id}/🦠️mutation/🔣️.json`;
   expect(existsSync(resolve(owner,file)),"committed genuine byte intent").toBe(true);
   const actual=read(file);
   expect(actual).toEqual({mutation:row.mutation,before:{schema:"stdio.binary",bytes:row.before},after:{schema:"stdio.binary",bytes:row.after}});
   console.log(`[DEBUG] Binary committed ${row.mutation.mutation} SQLite ordinal output ${JSON.stringify(row.after)}`);
  }finally{db.close();}
 }
});
