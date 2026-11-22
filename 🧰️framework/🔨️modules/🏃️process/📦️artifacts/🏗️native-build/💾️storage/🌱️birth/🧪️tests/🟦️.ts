import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{nativeStorageUnbornReservationsV1}from"../🟦️.ts";
import nativeSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import birthSchema from"../🧬️schema/🔣️.json"with{type:"json"};
import schema from"./📋️schema.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
/** 🧟️ Proves the language-agnostic unborn reservation cases against the schema, the owner and an independent SQLite oracle. */
test("unborn funded child reservations are named only when no namespace, directory or child ever existed",()=>{
 const ajv=new Ajv({strict:true}).addSchema(nativeSchema).addSchema(birthSchema);expect(ajv.compile(schema)(fixture)).toBe(true);const db=new Database(":memory:"),retirement=ajv.compile({$ref:birthSchema.$id+"#/definitions/UnbornReservationRetirement"});
 try{for(const row of fixture.cases){const data=JSON.stringify(row.request),ids=nativeStorageUnbornReservationsV1(row.request),oracle=JSON.parse((db.query("select coalesce((select json_group_array(json_extract(value,'$.id')) from json_each(?,'$.state.allocations') where json_extract(?,'$.childPresent')=0 and json_extract(?,'$.namespaceDirectoryPresent')=0 and json_extract(value,'$.phase')='reserved' and json_extract(value,'$.kind')='compiler-state' and json_extract(value,'$.namespace') is null and json_extract(value,'$.funding') is null and json_extract(value,'$.child.directory')=json_extract(?,'$.directory') and json_extract(value,'$.child.allocation')=json_extract(value,'$.id')),'[]') ids").get(data,data,data,data)as{ids:string}).ids)as string[];expect(retirement({version:1,ids:[...ids]}),row.id).toBe(true);expect([...ids],row.id).toEqual(row.ids);expect(oracle,row.id).toEqual(row.ids);}}finally{db.close();}
 expect(()=>nativeStorageUnbornReservationsV1({...fixture.cases[0]!.request,childPresent:"no"})).toThrow("Closed unborn reservation evidence required");
});
