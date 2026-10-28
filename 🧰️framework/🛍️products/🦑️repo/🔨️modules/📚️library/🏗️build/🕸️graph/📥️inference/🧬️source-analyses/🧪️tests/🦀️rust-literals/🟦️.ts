/** 🦀️ Contract: the Rust tokenizer reads a string literal without an escape as its inner text and one with an escape through JSON, so the module and include references of every source equal the golden taken from the module before this fast path, whether a literal is plain, escaped, raw, a byte or C string, unterminated or holds a raw line break. */
import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

const inference=await import(new URL("../../../🟨️.mjs",import.meta.url).href) as any;
await inference.libraryBootstrap;
const observation=await import(new URL("../../../../../👁️observation/🟨️.mjs",import.meta.url).href) as any;

test("the fixture is closed and valid against its schema",()=>{
 const valid=new Ajv({strict:true,allErrors:true}).compile(fixtureSchema);
 expect(valid(fixture),JSON.stringify(valid.errors)).toBe(true);
});

test("the references of every source equal the golden of the module before the fast path",()=>{
 const directory=mkdtempSync(join(tmpdir(),"rust-literals-")),rust=inference.prepareSourceAnalysesV1().rust;
 try{
  for(const row of fixture.cases){
   const file=join(directory,"x.rs");writeFileSync(file,row.source);
   let references:unknown;try{references=observation.observedSourceAnalysisV1(rust,file);}catch(error){references={thrown:String((error as Error).message).slice(0,80)};}
   expect(references,row.name).toEqual(row.references);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("sources with a thousand plain and escaped literals keep every module name and include",()=>{
 const directory=mkdtempSync(join(tmpdir(),"rust-literals-many-")),rust=inference.prepareSourceAnalysesV1().rust;
 try{
  const lines:string[]=[];for(let index=0;index<1000;index++)lines.push(index%2===0?`const S${index}: &str = "plain ${index}";`:`const S${index}: &str = "escaped \\"${index}\\" \\u{41}";`);
  lines.push("mod last;",'const I: &str = include_str!("end/z.txt");');
  const file=join(directory,"many.rs");writeFileSync(file,lines.join("\n"));
  expect(observation.observedSourceAnalysisV1(rust,file)).toEqual({includes:["end/z.txt"],modules:[{name:"last"}]});
 }finally{rmSync(directory,{recursive:true,force:true});}
});
