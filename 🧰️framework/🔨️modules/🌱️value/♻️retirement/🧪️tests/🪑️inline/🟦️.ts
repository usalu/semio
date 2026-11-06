import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv/dist/2020";
import schema from"./🔣️.json";
import law from"./📐️.json";
test("inline retirement inputs agree with the independent SQLite value and UTF8 oracle",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{const row=db.query("SELECT json_array_length(?) AS values_count,length(CAST(? AS BLOB)) AS bytes").get(JSON.stringify(law.numbers),law.text)as{values_count:number,bytes:number};expect(row.values_count).toBe(law.expected.values);expect(row.bytes).toBe(law.expected.utf8Bytes);}finally{db.close();}
});
