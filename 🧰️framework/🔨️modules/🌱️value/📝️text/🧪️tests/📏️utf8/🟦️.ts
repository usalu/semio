import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {textUtf8ByteLength} from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/📏️utf8/🔣️.json";

test("semantic Unicode text length agrees with independent UTF and SQLite engines",()=>{
  const database=new Database(":memory:");
  try{for(const row of fixture.cases){
    const accepted=Array.from(row.text).every(scalar=>{const word=scalar.codePointAt(0)!;return word<0xd800||word>0xdfff});
    expect(accepted).toBe(row.accepted);
    if(!accepted){expect(()=>textUtf8ByteLength(row.text)).toThrow();continue;}
    expect(textUtf8ByteLength(row.text)).toBe(row.bytes);
    expect(Buffer.byteLength(row.text,"utf8")).toBe(row.bytes);
    expect(database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(row.text)).toEqual({bytes:row.bytes});
  }}finally{database.close();}
  console.log("[DEBUG] semantic-unicode-byte-length oracle=SQLite+Buffer");
});
