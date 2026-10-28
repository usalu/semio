/** 🧂️ Contract: the text helpers order texts as their UTF-8 bytes order and decode bytes in place, and the SHA-256 helpers digest exactly what `createHash("sha256").update(...)` digests and what an independent Python model (hashlib over the WHATWG UTF-8 encoding) digests, for texts with astral pairs and lone surrogates, for raw bytes, and for delimited text lists (no digest depends on how the text is cut into parts), they refuse a delimiter that is not ASCII, and the module holds no hash object. */
import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{compareUtf8V1,sha256DelimitedHexV1,sha256FirstByteV1,sha256HexV1,utf8TextV1}from"../🟨️.mjs";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py"),reference=(data:string|Uint8Array)=>createHash("sha256").update(data).digest("hex");
const model=()=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify(fixture))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()) as{texts:string[];firstBytes:number[];bytes:string[];delimited:string[];order:number[][];generatedOrder:number[][];views:string[];generated:{texts:string[];digests:string[];firstBytes:number[]}};
};

test("the fixture is closed and valid against its schema",()=>{
 const ajv=new Ajv({strict:true,allErrors:true}),valid=ajv.compile(fixtureSchema);
 expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
});

test("a text digest equals createHash and the Python model, lone surrogates included",()=>{
 const expected=model();
 fixture.texts.forEach((row,index)=>{
  expect(sha256HexV1(row.value),row.name).toBe(expected.texts[index]!);
  expect(reference(row.value),row.name).toBe(expected.texts[index]!);
  expect(sha256FirstByteV1(row.value),row.name).toBe(expected.firstBytes[index]!);
 });
 expect(sha256HexV1("abc")).toBe("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
 expect(sha256HexV1("")).toBe("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
});

test("a byte digest equals createHash and the Python model, for a Buffer and a view into a larger buffer",()=>{
 const expected=model();
 fixture.bytes.forEach((row,index)=>{
  const bytes=Buffer.from(row.hex,"hex"),padded=Buffer.concat([Buffer.from("xx"),bytes,Buffer.from("yy")]),view=padded.subarray(2,2+bytes.length);
  expect(sha256HexV1(bytes),row.name).toBe(expected.bytes[index]!);
  expect(sha256HexV1(new Uint8Array(bytes)),row.name).toBe(expected.bytes[index]!);
  expect(sha256HexV1(view),row.name).toBe(expected.bytes[index]!);
  expect(reference(view),row.name).toBe(expected.bytes[index]!);
 });
});

test("a delimited digest equals the chained updates and the Python model, whatever the text edges hold",()=>{
 const expected=model();
 fixture.delimited.forEach((row,index)=>{
  const chained=createHash("sha256");for(const text of[...("first" in row?[row.first]:[]),...row.texts])chained.update(text).update(row.delimiter);
  const first="first" in row?row.first as string:undefined;
  expect(sha256DelimitedHexV1(row.texts,row.delimiter,first),row.name).toBe(chained.digest("hex"));
  expect(sha256DelimitedHexV1(row.texts,row.delimiter,first),row.name).toBe(expected.delimited[index]!);
 });
});

test("a generated run of 400 texts equals the Python model digest, first byte and chained digest",()=>{
 const expected=model().generated;
 expect(expected.texts.length).toBe(fixture.generated.texts);
 expected.texts.forEach((text,index)=>{
  expect(sha256HexV1(text),String(index)).toBe(expected.digests[index]!);
  expect(sha256FirstByteV1(text),String(index)).toBe(expected.firstBytes[index]!);
 });
 const chained=createHash("sha256");for(const text of expected.texts)chained.update(text).update("\0");
 expect(sha256DelimitedHexV1(expected.texts,"\0")).toBe(chained.digest("hex"));
});

test("the byte order of two texts equals Buffer.compare and the Python model, surrogates and the range above them included",()=>{
 const expected=model(),sign=(value:number)=>Math.sign(value);
 const check=(texts:string[],matrix:number[][])=>texts.forEach((left,row)=>texts.forEach((right,column)=>{
  const answer=compareUtf8V1(left,right);
  expect(sign(answer),JSON.stringify([left,right])).toBe(matrix[row]![column]!);
  expect(sign(answer),JSON.stringify([left,right])).toBe(sign(Buffer.compare(Buffer.from(left),Buffer.from(right))));
 }));
 check(fixture.order,expected.order);
 check(expected.generated.texts.slice(0,90),expected.generatedOrder);
 const shuffled=[...fixture.order].reverse();
 expect([...shuffled].sort(compareUtf8V1)).toEqual([...shuffled].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))));
});

test("bytes decode to the Python text from a Buffer, a Uint8Array and a view into a larger buffer, without touching the source",()=>{
 const expected=model();
 fixture.views.forEach((text,index)=>{
  const bytes=Buffer.from(text,"utf8"),padded=Buffer.concat([Buffer.from("xx"),bytes,Buffer.from("yy")]),before=Buffer.from(padded);
  expect(utf8TextV1(bytes),text).toBe(expected.views[index]!);
  expect(utf8TextV1(new Uint8Array(bytes)),text).toBe(expected.views[index]!);
  expect(utf8TextV1(new Uint8Array(padded.buffer,padded.byteOffset+2,bytes.length)),text).toBe(expected.views[index]!);
  expect(utf8TextV1(padded.subarray(2,2+bytes.length)),text).toBe(expected.views[index]!);
  expect(padded.equals(before)).toBe(true);
 });
});

test("a delimiter that is not ASCII is refused, because a surrogate pair could then form across the cut",()=>{
 for(const delimiter of fixture.refusedDelimiters)expect(()=>sha256DelimitedHexV1(["a","b"],delimiter),JSON.stringify(delimiter)).toThrow(TypeError);
});

test("the module holds no hash object: every digest is one call",()=>{
 const source=readFileSync(join(import.meta.dir,"..","🟨️.mjs"),"utf8");
 expect(source).not.toContain("createHash");
});
