import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{createRequire}from"node:module";
import{isUtf8}from"node:buffer";
import ts from"typescript";
import Ajv from"ajv";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"../🔣️.json";
const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("capture.ts",source,ts.ScriptTarget.Latest,true),names=["decodeOwnedCommandTextV1"],nodes=tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&names.includes(node.name?.text??""));
const crossSpawn=createRequire(import.meta.url)("cross-spawn"),validate=new Ajv({strict:true}).compile(schema.definitions.OutputEncoding);
test("bounded encoding diagnostic owner exists",()=>expect(nodes).toHaveLength(1));
for(const row of fixture.encoding)test("owned strict UTF8 "+row.name,()=>{
 expect(nodes).toHaveLength(1);
 const code=ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n")+";return decodeOwnedCommandTextV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,decode=new Function("schema",code)(schema),decoder=new TextDecoder("utf-8",{fatal:true}),bytes=Buffer.from(row.chunks.flat()),independent=crossSpawn.sync(process.execPath,["-e","process.stdout.write(Buffer.from("+JSON.stringify([...bytes])+"))"],{encoding:null,timeout:5000});
 expect(independent.status).toBe(0);expect(Buffer.from(independent.stdout)).toEqual(bytes);expect(isUtf8(independent.stdout)).toBe(row.accepted);
 let output="",observed=0,error:unknown;
 try{for(const chunk of row.chunks){observed+=chunk.length;output+=decode(decoder,Buffer.from(chunk),"p".repeat(270),row.stream,"chunk",observed,observed);}output+=decode(decoder,undefined,"p".repeat(270),row.stream,"final",observed,observed);}catch(failure){error=failure;}
 if(row.accepted){expect(error).toBeUndefined();expect(output).toBe(row.text);expect(output).toBe(new TextDecoder("utf-8",{fatal:true}).decode(independent.stdout));}
 else{expect(error).toBeInstanceOf(TypeError);const cause=(error as Error).cause as Record<string,unknown>;expect(validate(cause)).toBe(true);expect(Object.isFrozen(cause)).toBe(true);expect(cause).toEqual({version:1,kind:"output-encoding",program:"p".repeat(256),programTruncated:true,stream:row.stream,phase:row.phase,streamBytes:observed,observedBytes:observed});expect((error as Error).message).toContain(schema.definitions.EncodingProtocol.const.marker+JSON.stringify(cause));}
});

test("encoding context survives stderr tail truncation without adding authority",()=>{
 const owner=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="readOwnedCommandEncodingContextV1");expect(owner).toBeDefined();
 const code=ts.transpileModule(owner!.getText(tree).replace(/^export /u,"")+";return readOwnedCommandEncodingContextV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,read=new Function("schema","validateJsonSchemaSubset",code)(schema,validateJsonSchemaSubset),row={version:1,kind:"output-encoding",program:"/owned/program",programTruncated:false,stream:"stderr",phase:"final",streamBytes:2,observedBytes:7},marker=schema.definitions.EncodingProtocol.const.marker,text="TypeError: invalid "+marker+JSON.stringify(row)+"\n"+"x".repeat(6000);
 expect(text.slice(-2048)).not.toContain(marker);expect(read(text)).toEqual(row);expect(Object.isFrozen(read(text))).toBe(true);expect(read(marker+JSON.stringify({...row,args:["forbidden"]}))).toBeUndefined();expect(read(marker+"{")).toBeUndefined();
});
test("encoding annotation preserves the exact decoder error object",()=>{
 const code=ts.transpileModule(nodes[0]!.getText(tree)+";return decodeOwnedCommandTextV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,decode=new Function("schema",code)(schema),reason=new TypeError("actual decoder refusal");let failure:unknown;
 try{decode({decode:()=>{throw reason;}},undefined,"/owned/program","stdout","final",2,2);}catch(error){failure=error;}
 expect(failure).toBe(reason);expect(validate(reason.cause)).toBe(true);
});
