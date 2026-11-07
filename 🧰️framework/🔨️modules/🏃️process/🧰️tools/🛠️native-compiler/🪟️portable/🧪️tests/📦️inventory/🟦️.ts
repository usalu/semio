import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../../🧬️schema/📦️inventory.json" with {type:"json"};
import fixture from "../../🧫️fixtures/📦️inventory.json" with {type:"json"};
import {portableMsvcInventoryV1} from "../../📦️inventory/🟦️.ts";

test("untrusted compiler inventory is bounded canonical metadata only",()=>{
 const oracle=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{for(const row of fixture.cases){const value={version:1,recipeSha256:fixture.recipeSha256,files:row.paths.map(path=>({path,bytes:0,sha256:fixture.emptySha256}))};expect(oracle(value),row.name).toBe(true);const order=db.query("select count(*) n from json_each(?1) a join json_each(?1) b on cast(a.key as integer)<cast(b.key as integer) where a.value>=b.value collate binary or lower(a.value)=lower(b.value)").get(JSON.stringify(row.paths)) as {n:number};if(row.accepted)expect(order.n,row.name).toBe(0);if(row.accepted){const result=portableMsvcInventoryV1(value);expect(result.text).toBe(JSON.stringify(value)+"\n");expect(result.files).toBe(row.paths.length);expect(result.bytes).toBe(0);}else expect(()=>portableMsvcInventoryV1(value),row.name).toThrow();}expect(()=>portableMsvcInventoryV1({version:1,recipeSha256:fixture.recipeSha256,files:[],ready:true})).toThrow();}finally{db.close();}
});
