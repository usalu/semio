import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/prepared-instance.json";
import schema from "./🧫️fixtures/prepared-instance.schema.json";

test("the independent VM applies the tracked initializer and keeps actor instances independent",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const bytes=[0,97,115,109,1,0,0,0];
 for(const [id, payload] of fixture.sections as [number,number[]][])bytes.push(id,payload.length,...payload);
 const module=new WebAssembly.Module(Uint8Array.from(bytes));
 type FixtureExports={memory:WebAssembly.Memory,value:WebAssembly.Global,table:WebAssembly.Table,dataKeep:()=>number,dataDrop:()=>number,elemKeep:()=>number,elemDrop:()=>number};
 const open=()=>new WebAssembly.Instance(module).exports as FixtureExports;
 const first=open(), second=open();
 const memory=new Uint8Array(first.memory.buffer);
 expect([...memory.slice(fixture.offset,fixture.offset+fixture.bytes.length)]).toEqual(fixture.bytes);
 expect(first.value.value).toBe(fixture.global);
 expect((first.table.get(0) as ()=>number)()).toBe(fixture.tableResult);
 memory[fixture.offset]=3;
 first.value.value=1;
 first.table.set(0,null);
 expect(new Uint8Array(second.memory.buffer)[fixture.offset]).toBe(fixture.bytes[0]);
 expect(second.value.value).toBe(fixture.global);
 expect((second.table.get(0) as ()=>number)()).toBe(fixture.tableResult);
 expect(second.dataKeep()).toBe(fixture.dataSegments[1]?.[0]);
 expect(()=>second.dataDrop()).toThrow();
 expect(first.elemKeep()).toBe(fixture.tableResult);
 expect(()=>first.elemDrop()).toThrow();
});
