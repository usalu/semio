import {describe,expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../../🔣️.json";
import {parseTiffSnapshot,parseTiffStorage} from "../../🟦️.ts";

const fixture = await Bun.file(new URL("../../🧫️fixtures/🧬️canonical-storage/🔣️.json",import.meta.url)).json() as {
  cases:{id:string;snapshot:any;rgba:number[]}[];
  refusals:{id:string;storage:unknown}[];
};

function nativeWords(value:any):any {
  if(Array.isArray(value)) return value.map(nativeWords);
  if(value!==null&&typeof value==="object"){
    if(Object.keys(value).length===1&&typeof value.bits==="string") return {bits:BigInt(value.bits)};
    return Object.fromEntries(Object.entries(value).map(([key,item])=>[key,nativeWords(item)]));
  }
  return value;
}

describe("TIFF canonical storage schema",()=>{
  test("the independent JSON schema admits every neutral case",()=>{
    const validate=new Ajv({strict:false}).compile(schema);
    for(const item of fixture.cases) expect(validate(item.snapshot),item.id+": "+JSON.stringify(validate.errors)).toBe(true);
  });
  test("the source twin owns chunks and exact IEEE words",()=>{
    for(const item of fixture.cases){
      const snapshot=parseTiffSnapshot(nativeWords(item.snapshot));
      expect(snapshot.ifds.length).toBe(item.snapshot.ifds.length);
      expect(snapshot.ifds[0]!.storage.chunks).toEqual(item.snapshot.ifds[0]!.storage.chunks);
    }
    const exact=parseTiffSnapshot(nativeWords(fixture.cases[2]!.snapshot));
    expect(exact.ifds[0]!.entries[1]!.values).toEqual({kind:"float",value:[{bits:2143289345},{bits:2147483648}]});
    expect(exact.ifds[0]!.entries[2]!.values).toEqual({kind:"double",value:[{bits:9221120237041090561n},{bits:9223372036854775808n}]});
  });
  test("invalid storage authority is refused",()=>{
    for(const item of fixture.refusals) expect(()=>parseTiffStorage(item.storage)).toThrow();
  });
});
