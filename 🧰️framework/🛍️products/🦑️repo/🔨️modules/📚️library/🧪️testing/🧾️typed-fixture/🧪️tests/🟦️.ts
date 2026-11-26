import {expect,test} from "bun:test";
import Ajv from "ajv";
import {typedFixtureV1,readTypedFixtureV1,typedStubV1} from "../🟦️.ts";
import {typedFixtureTextV1,TYPED_FIXTURE_TEXTS_V1} from "../📖️texts/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {placeholdersV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";

const refusal=(body:()=>unknown):Error=>{try{body();}catch(error){return error as Error;}throw Error("no refusal");};

test("the corpus is a valid corpus of its own schema (Ajv)",()=>{
 const ajv=new Ajv({strict:false});expect(ajv.validate(schema,corpus)).toBe(true);
});

test("every case gets the verdict of the independent validator Ajv (language-agnostic corpus)",()=>{
 const ajv=new Ajv({strict:false});
 for(const row of corpus.cases){
  const independent=ajv.validate(row.schema,row.value),ours=(()=>{try{typedFixtureV1(row.value,row.schema);return true;}catch{return false;}})();
  expect({id:row.id,independent,ours}).toEqual({id:row.id,independent:row.valid,ours:row.valid});
 }
});

test("a refusal names the fixture and its problems in English and German",()=>{
 const row=corpus.cases.find(item=>item.id==="row-missing")!,error=refusal(()=>typedFixtureV1(row.value,row.schema,{name:"rows"}));
 expect(error.message).toContain("typed-fixture.fixture.invalid");
 expect(typedFixtureTextV1("en","fixture.invalid",{name:"rows",problems:"x"})).toContain("does not match its schema");
 expect(typedFixtureTextV1("de","fixture.invalid",{name:"rows",problems:"x"})).toContain("entspricht nicht ihrem Schema");
});

test("both languages of every message carry the same placeholders",()=>{
 for(const [key,pair] of Object.entries(texts.messages))expect({key,de:placeholdersV1(pair.de).join()}).toEqual({key,de:placeholdersV1(pair.en).join()});
 expect(TYPED_FIXTURE_TEXTS_V1.id).toBe("typed-fixture");
});

test("the returned value is a clone: editing it never edits the imported fixture",()=>{
 const row=corpus.cases.find(item=>item.id==="list-ok")!,rows=typedFixtureV1<number[]>(row.value,row.schema);
 rows.push(99);expect(row.value).toEqual([0,3,9]);
});

test("a fixture file is read, parsed and admitted; an unreadable or non-JSON file is refused as data",()=>{
 const url=new URL("./🧫️fixtures/🔣️.json",import.meta.url),parsed=readTypedFixtureV1<{cases:unknown[]}>(url,schema);
 expect(parsed.cases.length).toBe(corpus.cases.length);
 expect(refusal(()=>readTypedFixtureV1(new URL("./absent.json",import.meta.url),schema)).message).toContain("typed-fixture.fixture.unreadable");
 expect(refusal(()=>readTypedFixtureV1(new URL("../🧬️schema/🔣️.json",import.meta.url),{type:"array"})).message).toContain("typed-fixture.fixture.invalid");
});

test("a typed stub keeps what the test supplies and refuses every other member on first use",()=>{
 interface Port{read(path:string):string;limit:number;close():void}
 const port=typedStubV1<Port>("port",{read:path=>path+"!",limit:3});
 expect(port.read("a")).toBe("a!");expect(port.limit).toBe(3);
 expect(refusal(()=>port.close()).message).toContain("typed-fixture.stub.unimplemented");
});
