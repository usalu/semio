import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import fixture from"./🌍️path.json" with{type:"json"};
import{projectV1}from"../🟦️.ts";

test("Windows compiler projection emits one case-insensitive PATH",()=>{
 const db=new Database(":memory:");try{for(const row of fixture.cases){const result=projectV1(row.environment,"D:\\tools"),keys=Object.keys(result).filter(key=>key.toLowerCase()==="path"),oracle=db.query("select 1 count").get() as{count:number};expect(keys.length,row.name).toBe(oracle.count);const value=result[keys[0]!]!;if(row.suffix)expect(value.endsWith(";"+row.suffix),row.name).toBe(true);}}finally{db.close();}
});
