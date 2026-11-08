import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import cases from"./allocation-custody.json";
import schema from"./allocation-custody.schema.json";
test("original retirement keeps allocation custody separate from work, receipt and nested progress",()=>{
 expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);
 const db=new Database(":memory:");
 try{for(const c of cases){
 const own={roles:[c.primary,c.work,c.receipt,c.allocation].filter(x=>x!==null).length,progress:c.override??c.selected,retained:Number(c.allocation==="retained")};
 expect(db.query("SELECT (? IS NOT NULL)+(? IS NOT NULL)+(? IS NOT NULL)+1 AS roles, COALESCE(?,?) AS progress, ?='retained' AS retained").get(c.primary,c.work,c.receipt,c.override,c.selected,c.allocation)).toEqual(own);
 }}finally{db.close();}
});
