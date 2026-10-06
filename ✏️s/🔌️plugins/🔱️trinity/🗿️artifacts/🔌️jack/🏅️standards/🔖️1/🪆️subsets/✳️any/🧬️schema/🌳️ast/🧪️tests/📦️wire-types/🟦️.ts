
/** 🔌️ Jack owns its portable wire verdicts and the independent declaration compiler. */
import{expect,test}from"bun:test";
import{readFileSync,mkdirSync,mkdtempSync,writeFileSync,rmSync}from"node:fs";
import{join,resolve}from"node:path";
import{tmpdir}from"node:os";
import{fileURLToPath}from"node:url";
import ts from"typescript";

type WireCase=Readonly<{id:string;name:string;value:unknown;accept:boolean}>;
const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/📦️wire-types/🔣️.json",import.meta.url),"utf8")) as {schemaVersion:number;cases:WireCase[]};
test("Jack portable wire cases preserve the closed three-case contract",()=>{
 
 expect(fixture["schemaVersion"]).toEqual(1);
 expect(new Set(fixture.cases.map(row=>row.id)).size).toBe(3);
});
test("Jack declaration compiler accepts the original table and rejects both hostile wires",()=>{
 const output=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"jack-wire-types-"));
 try{for(const[index,row]of fixture.cases.entries()){
  const entry=join(root,String(index)+".ts"),owner=fileURLToPath(new URL("../../🟦️.ts",import.meta.url));
  writeFileSync(entry,"import type { "+row.name+" } from "+JSON.stringify(owner)+"; const value: "+row.name+" = "+JSON.stringify(row.value)+"; void value;");
  const program=ts.createProgram([entry],{noEmit:true,strict:true,allowImportingTsExtensions:true,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,target:ts.ScriptTarget.ES2022,types:[]}),diagnostics=ts.getPreEmitDiagnostics(program);
  expect(diagnostics.length===0,row.id+": "+diagnostics.map(x=>ts.flattenDiagnosticMessageText(x.messageText," ")).join("; ")).toBe(row.accept);
 }}finally{rmSync(root,{recursive:true,force:true});}
});
