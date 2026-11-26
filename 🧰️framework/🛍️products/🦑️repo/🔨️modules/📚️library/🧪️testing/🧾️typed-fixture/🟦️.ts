import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {refuseTypedFixtureV1} from "./📖️texts/🟦️.ts";

/** 🧾️ Options of a typed fixture: the schema document its `$ref`s resolve in (the schema itself by default), further documents, and the name the refusal shows. */
export interface TypedFixtureOptionsV1{readonly root?:unknown;readonly documents?:readonly unknown[];readonly name?:string}

/** 🧾️ Admits one parsed fixture value against its schema and returns it as the type `T` the schema describes; the value is cloned, so a test that edits its rows never edits the imported module. @see https://json-schema.org/draft-07 */
export function typedFixtureV1<T>(value:unknown,schema:unknown,options:TypedFixtureOptionsV1={}):T{
 const name=options.name??"fixture",problems=validateJsonSchemaSubset(schema,value,options.root??schema,options.documents??[]);
 if(problems.length>0)throw refuseTypedFixtureV1("invalid","fixture.invalid",{name,problems:problems.slice(0,8).join("; ")});
 return structuredClone(value) as T;
}

/** 📄️ Reads one JSON fixture file (a `file:` URL or an absolute path) and admits it like {@link typedFixtureV1}. */
export function readTypedFixtureV1<T>(location:URL|string,schema:unknown,options:TypedFixtureOptionsV1={}):T{
 const name=options.name??(location instanceof URL?location.pathname:location);let text:string;
 try{text=readFileSync(location instanceof URL?fileURLToPath(location):location,"utf8");}catch(error){throw refuseTypedFixtureV1("missing","fixture.unreadable",{name,reason:error instanceof Error?error.message:String(error)});}
 let value:unknown;
 try{value=JSON.parse(text);}catch(error){throw refuseTypedFixtureV1("invalid","fixture.not-json",{name,reason:error instanceof Error?error.message:String(error)});}
 return typedFixtureV1<T>(value,schema,{...options,name});
}

/** 🎭️ A typed stub of the port `P`: it keeps the members the test supplies and refuses every other member on first use, so a fake that is narrower than its port fails loudly instead of being cast. */
export function typedStubV1<P extends object>(name:string,implemented:Partial<P>):P{
 const known=Object.freeze({...implemented});
 return new Proxy(known as P,{get(target,member,receiver){
  if(typeof member==="symbol"||member==="then"||member==="toJSON"||Reflect.has(target,member))return Reflect.get(target,member,receiver);
  throw refuseTypedFixtureV1("unsupported","stub.unimplemented",{name,member});
 }});
}
