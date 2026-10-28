/** 📜️ Contract: one snapshot is validated, serialized and hashed once; the text is exactly `JSON.stringify` and the identity is its sha256 as an independent implementation (Python `hashlib`) computes it; only a deeply frozen snapshot is remembered, a mutable one is serialized afresh; a snapshot outside the schema is refused. The fixture lives in `./🔣️.json`. */
import {expect,test} from "bun:test";
import {execFileSync} from "node:child_process";
import {buildSourceClosureIdentityV1,buildSourceClosureTextV1,type BuildSourceClosureV1} from "../../🟦️.ts";
import fixture from "./🔣️.json";

const python=(text:string):string=>execFileSync("python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())"],{input:Buffer.from(text,"utf8"),encoding:"utf8"}).trim();
const frozen=():BuildSourceClosureV1=>Object.freeze({...fixture,version:1 as const,files:Object.freeze(fixture.files.map(row=>Object.freeze({...row,imports:Object.freeze({...row.imports} as Record<string,string>)})))});

test("a frozen snapshot is serialized once: the same text object, JSON.stringify exactly, and the sha256 Python computes",()=>{
 const value=frozen(),first=buildSourceClosureTextV1(value);
 expect(buildSourceClosureTextV1(value)).toBe(first);
 expect(first.text).toBe(JSON.stringify(value));
 expect(first.sha256).toBe(python(JSON.stringify(value)));
 expect(buildSourceClosureIdentityV1(value)).toBe(first.sha256);
});

test("a snapshot with a mutable part is never remembered: a change between two calls changes text and identity",()=>{
 const imports:Record<string,string>={"./🟦️.ts":"🏗️build/🟦️.ts"},value=Object.freeze({...fixture,version:1 as const,files:Object.freeze([Object.freeze({...fixture.files[0]!,imports}),Object.freeze({...fixture.files[1]!,imports:Object.freeze({})})])}) as BuildSourceClosureV1,first=buildSourceClosureTextV1(value);
 imports["./other.ts"]="🏗️build/🟦️.ts";
 const second=buildSourceClosureTextV1(value);
 expect(second).not.toBe(first);
 expect(second.text).toBe(JSON.stringify(value));
 expect(second.sha256).toBe(python(JSON.stringify(value)));
 expect(second.sha256).not.toBe(first.sha256);
});

test("a snapshot outside the schema is refused before anything is serialized",()=>{
 expect(()=>buildSourceClosureTextV1(Object.freeze({...frozen(),files:Object.freeze([])}))).toThrow("Closed implementation Source snapshot required");
 expect(()=>buildSourceClosureIdentityV1({...frozen(),extra:true} as unknown as BuildSourceClosureV1)).toThrow("Closed implementation Source snapshot required");
});
