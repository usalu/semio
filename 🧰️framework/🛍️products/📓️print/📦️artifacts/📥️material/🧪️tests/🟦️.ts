import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readdirSync,readFileSync}from"node:fs";
import{preparePrintMaterialV1,withPreparedPrintMaterialV1}from"../📤️producer/🟦️.ts";
import{createHash}from"node:crypto";
import{printPinnedMaterialPolicyV1,printBundlePlanV1,tectonicDistributionV1,readPrintResponseV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{withBuildTestInvocationV1}from"./../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"./../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
test("Print pinned material bounds agree with Ajv and independent SQLite sums",()=>{const validate=new Ajv({strict:true}).compile(schema),database=new Database(":memory:");try{for(const row of corpus.valid){expect(validate(row)).toBe(true);expect(JSON.stringify(printPinnedMaterialPolicyV1(row))).toBe(JSON.stringify(row));}for(const row of corpus.invalid){expect(validate(row)).toBe(false);expect(()=>printPinnedMaterialPolicyV1(row)).toThrow();}const bundle=printBundlePlanV1();database.exec("create table files(name text primary key,bytes integer not null,offset integer not null)");for(const file of bundle.files)database.query("insert into files values(?,?,?)").run(file.name,file.bytes,file.offset);const total=database.query<{total:number,count:number},[]>("select sum(bytes) total,count(*) count from files").get();expect(total).toEqual({total:bundle.bytes,count:bundle.files.length});expect(bundle.files.length).toBe(500);expect(bundle.bytes).toBe(19856791);expect(Object.isFrozen(bundle.files)).toBe(true);for(const[platform,architecture,format]of[["darwin","arm64","tar.gz"],["darwin","x64","tar.gz"],["linux","arm64","tar.gz"],["linux","x64","tar.gz"],["linux","arm","tar.gz"],["linux","ia32","tar.gz"],["win32","x64","zip"]]){const distribution=tectonicDistributionV1(platform??"",architecture??"");expect(String(distribution.format)).toBe(String(format));expect(distribution.bytes).toBeLessThanOrEqual(schema.definitions.Policy.const.maximumArchiveBytes);}expect(()=>tectonicDistributionV1("win32","arm64")).toThrow();}finally{database.close();}});
test("Print exact response guard consumes bounded bytes and closes the actual reader",()=>withBuildTestInvocationV1(async invocation=>{const control=createIssuedResourceControlV1(invocation,"print:material-source",0),content=new TextEncoder().encode("pinned Print bytes"),sha256=createHash("sha256").update(content).digest("hex");expect(await readPrintResponseV1(new Response(content),{bytes:content.length,sha256},control)).toEqual(content);for(const bytes of[content.length-1,content.length+1]){let failed=false;try{await readPrintResponseV1(new Response(content),{bytes,sha256},control);}catch{failed=true;}expect(failed).toBe(true);}let failed=false;try{await readPrintResponseV1(new Response(content),{bytes:content.length,sha256:"0".repeat(64)},control);}catch{failed=true;}expect(failed).toBe(true);}));

test("Print pinned producer rejects fake operations and production script has no free-path extraction",()=>{expect(()=>preparePrintMaterialV1({}as Parameters<typeof preparePrintMaterialV1>[0],"tectonic",{}as Parameters<typeof preparePrintMaterialV1>[2])).toThrow();expect(()=>withPreparedPrintMaterialV1({}as Parameters<typeof withPreparedPrintMaterialV1>[0],"tex",{}as Parameters<typeof withPreparedPrintMaterialV1>[2],async()=>{})).toThrow();const url=new URL("../../../🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts",import.meta.url),source=ts.createSourceFile(url.pathname,readFileSync(url,"utf8"),ts.ScriptTarget.Latest,true);const declarations:string[]=[];function visit(node:ts.Node):void{if(ts.isFunctionDeclaration(node)&&node.name)declarations.push(node.name.text);ts.forEachChild(node,visit);}visit(source);expect(declarations.includes("tectonicBinaryPath")).toBe(false);expect(declarations.includes("preparedTectonic")).toBe(false);expect(declarations.includes("prepareTectonic")).toBe(false);});

import "../../../🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts";
import "../../../🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts";
test("Print response failure preserves Error and undefined while releasing its actual stream reader",()=>withBuildTestInvocationV1(async invocation=>{const control=createIssuedResourceControlV1(invocation,"print:response-failure",0);for(const reason of[Error("response primary"),undefined]){const response=new Response(new ReadableStream<Uint8Array>({start(controller){controller.error(reason);}}));let failed=false,failure:unknown;try{await readPrintResponseV1(response,{bytes:1,sha256:"0".repeat(64)},control);}catch(error){failed=true;failure=error;}expect(failed).toBe(true);expect(failure).toBe(reason);expect(response.body?.locked).toBe(false);}}));

import "../../../🔨️modules/📥️source-staging/🧪️tests/🟦️.ts";

import "../../../🔨️modules/🖨️tectonic-template-compilation/📋️intent/🧪️tests/🟦️.ts";

import "../../../🔨️modules/🖨️tectonic-template-compilation/🪟️panels/🧪️tests/🟦️.ts";

import "../../../🧪️tests/📥️invocation/🧪️tests/🟦️.ts";

import "../🌐️http/🧪️tests/🟦️.ts";

test("Print preparation checks an actual held ready reader before online acquisition",()=>{const url=new URL("../📤️producer/🟦️.ts",import.meta.url),file=ts.createSourceFile(url.pathname,readFileSync(url,"utf8"),ts.ScriptTarget.Latest,true),body=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="preparePrintMaterialV1")?.getText(file)??"";expect(body.indexOf("readyPrintMaterialV1(context,kind)")).toBeGreaterThan(0);expect(body.indexOf("readyPrintMaterialV1(context,kind)")).toBeLessThan(body.indexOf("network.offline"));expect(body).toContain("if(ready!==undefined)return ready");});

import "../../../🧪️tests/🧬️native-chart-grammar/📥️emission/🧪️tests/🟦️.ts";

import "../../🧵️worker/🧪️tests/🟦️.ts";

import "../../../🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/📥️pdf/🧪️tests/🟦️.ts";

test("actual pipeline verification has carried scope and no free compiler or environment output roots",()=>{const pipelineUrl=new URL("../../../🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts",import.meta.url),source=ts.createSourceFile(pipelineUrl.pathname,[pipelineUrl,...readdirSync(new URL("./",pipelineUrl),{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>new URL(entry.name+"/🟦️.ts",pipelineUrl))].map(part=>readFileSync(part,"utf8")).join("\n"),ts.ScriptTarget.Latest,true),calls:string[]=[];function visit(node:ts.Node):void{if(ts.isCallExpression(node))calls.push(node.expression.getText(source));ts.forEachChild(node,visit);}visit(source);expect(calls.includes("compilePrintTexOnce")).toBe(false);expect(calls.includes("printFontSearchPaths")).toBe(false);expect(source.text.includes("SEMIO_PRINT_OUTPUT_DIR")).toBe(false);expect(source.text.includes("SEMIO_PRINT_API_PUBLICATION_REUSE")).toBe(false);expect(source.text.includes("withPrintPipelineVerificationV1")).toBe(true);});

import{readPrintBundleRangeV1}from"../🟦️.ts";
import rangeCases from"../../../🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🧫️cases.json"with{type:"json"};
import{sha256 as independentSha256}from"@noble/hashes/sha2.js";
test("actual pinned range guard refuses neutral HTTP length and digest corruption before a writer exists",()=>withBuildTestInvocationV1(async invocation=>{const control=createIssuedResourceControlV1(invocation,"print:range-source",0),bundle=printBundlePlanV1(),file=bundle.files.find(row=>row.bytes>0);if(!file)throw Error("Actual pinned nonempty range required");const range=`${file.offset}-${file.offset+file.bytes-1}`,validate=new Ajv().compile({type:"object",required:["status","range"],properties:{status:{const:206},range:{const:`bytes ${range}/${bundle.archiveBytes}`}},additionalProperties:false});for(const vector of rangeCases.failures){const bytes=new Uint8Array(file.bytes+(vector.bytes==="oversized"?1:0)),header=`bytes ${vector.range==="matching"?range:"0-0"}/${bundle.archiveBytes}`,response=new Response(bytes,{status:vector.status,headers:{"content-range":header}});expect(validate({status:response.status,range:header})).toBe(vector.status===206&&vector.range==="matching");expect(Buffer.from(independentSha256(bytes)).toString("hex")).not.toBe(file.sha256);let failed=false;try{await readPrintBundleRangeV1(response,file,bundle.archiveBytes,control);}catch{failed=true;}expect(failed).toBe(true);expect(response.body?.locked).toBe(false);}}));

import "../../../🎮️commands/🧪️print-pipeline-verification/📋️evidence/🧪️tests/🟦️.ts";

import "../../📖️pdf/🧪️tests/🟦️.ts";

import "../../../🧪️tests/🧬️native-chart-grammar/🎨️paint/🧪️tests/🟦️.ts";
