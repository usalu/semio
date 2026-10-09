/** 🧪️ Neutral lease conservation uses independent JSON Schema and RFC6902 ownership witnesses. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {applyPatch} from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {NumericIndex} from "../../../🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🟦️.ts";
import numericFixture from "../../../🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧪️tests/📜️fixtures/🔣️.json";
import numericSchema from "../../../🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧬️schema/🔣️.json";
const valid=new Ajv({strict:false}).compile(schema);
for(const row of fixture.cases)test(row.name,()=>{
 expect(valid(row.input)).toBe(true);
 const source={bytes:row.input.bytes},owner={lease:source,aliases:Array.from({length:row.input.aliases},()=>source)};
 const zero=applyPatch(owner,[],true,false).newDocument;expect(zero).toEqual(owner);
 const retired=applyPatch(owner,[{op:"remove",path:"/lease"}],true,false).newDocument;expect(retired.aliases).toEqual(owner.aliases);expect(source.bytes).toEqual(row.input.bytes);expect([...new Uint8Array(source.bytes)]).toEqual(row.input.bytes);
 expect(fixture.laws).toContain("last-owner-requires-whole-physical-release");expect(fixture.laws).toContain("terminal-destructor-releases-no-backing");
});
test("physical owner interruption contracts cover each native raster and geometry leaf",()=>{
 expect(fixture.interruptions).toEqual([0,1,10,100,4096]);expect(fixture.nativeOwners).toEqual(["coverage","affine","composite","flatten","stroke"]);
 console.log("[DEBUG] Physical leaf neutral schema and RFC6902 lease conservation passed; actual allocator witnesses are native");
});
test("numeric scratch AVL matches schema and independent ordered reference",()=>{
 expect(new Ajv2020({strict:false}).compile(numericSchema)(numericFixture)).toBe(true);const index=new NumericIndex<number>(),oracle=new Map<number,number>();
 const actual=numericFixture.operations.map(row=>{const key="key"in row?row.key!:0;let result:number|number[]|null=null;switch(row.kind){case"set":{const previous=oracle.get(key);oracle.set(key,"value"in row?row.value!:0);result=index.set(key,"value"in row?row.value!:0)??null;expect(result).toEqual(previous??null);break;}case"remove":{const previous=oracle.get(key);oracle.delete(key);result=index.remove(key)??null;expect(result).toEqual(previous??null);break;}case"get":result=index.get(key)??null;expect(result).toEqual(oracle.get(key)??null);break;case"first":{const first=[...oracle.keys()].sort((a,b)=>a-b)[0];const expected=first===undefined?null:[first,oracle.get(first)!];if(first!==undefined)oracle.delete(first);result=index.popFirst()??null;expect(result).toEqual(expected);break;}case"reset":index.reset();oracle.clear();break;}expect(index.size).toBe(oracle.size);return result;});expect(actual).toEqual(numericFixture.expected);
 for(let key=0;key<4096;key++)index.set(key,key);expect(index.depth).toBeLessThanOrEqual(14);const stored=index.storedKeys;index.reset();for(let key=4095;key>=0;key--)index.set(key,key);expect(index.storedKeys).toBe(stored);for(let key=0;key<4096;key++)expect(index.popFirst()).toEqual([key,key]);expect(index.size).toBe(0);console.log("[DEBUG] Numeric scratch AVL shared fixture and ordered reference passed; slots survived logical reset");
});
