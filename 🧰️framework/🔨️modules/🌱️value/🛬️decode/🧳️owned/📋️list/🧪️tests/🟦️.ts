import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv/dist/2020.js";
import schema from"../🧬️schema/🔣️.json";
import fixture from"../🧫️fixtures/🔣️.json";
import{NativePersistentListReceiving}from"../🟦️.ts";
function input(count:number):{label:string;rank:number}[]{return Array.from({length:count},(_,i)=>({label:i===0?fixture.prefix.repeat(fixture.firstRepeat):fixture.prefix+i,rank:i*fixture.stride}));}
test("persistent native receiving preserves original inputs and ranked typed output",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const db=new Database(":memory:");
 for(const count of fixture.cases){const original=input(count),snapshot=JSON.stringify(original),receiver=new NativePersistentListReceiving(original);while(!receiver.ready)receiver.advance();const root=receiver.take();const expected=db.query("SELECT json_group_array(json_object('label',json_extract(value,'$.label'),'rank',json_extract(value,'$.rank'))) AS json FROM json_each(?)").get(snapshot)as{json:string};expect(Array.from(root)).toEqual(JSON.parse(expected.json));expect(JSON.stringify(original)).toBe(snapshot);for(let i=0;i<count;i++)expect(root.get(i)).toEqual(original[i]);receiver.close();expect(receiver.empty).toBe(true);root.close();}
 db.close();console.log("[DEBUG] native persistent receiving neutral rows match independent SQLite typed JSON; original input identity/value retained");
});
test("persistent receiving keeps later invalid rows and every cancelled prefix",()=>{
 for(const index of fixture.invalidRows){const original:unknown[]=input(257);original[index]={label:"invalid",rank:"refused"};const receiver=new NativePersistentListReceiving(original);expect(()=>{while(!receiver.ready)receiver.advance();}).toThrow();expect(original[index]).toEqual({label:"invalid",rank:"refused"});receiver.close();expect(receiver.empty).toBe(true);}
 for(const count of fixture.cases)for(const cut of fixture.cuts){const original=input(count),receiver=new NativePersistentListReceiving(original);for(let turn=0;turn<cut&&!receiver.ready;turn++)receiver.advance();receiver.close();expect(receiver.empty).toBe(true);expect(original.length).toBe(count);}
 const invalid=new NativePersistentListReceiving({rows:[]});expect(()=>invalid.advance()).toThrow();invalid.close();expect(invalid.empty).toBe(true);console.log("[DEBUG] native persistent receiving neutral60 cancellations and three later-row refusals preserve original input");
});
