/** 🏛️ Enforces one neutral SQLite definition against independent Rust grammar and TOML. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {parse as parseToml} from "smol-toml";
import {rustTokens,inspectRustCompileReferences} from "../../../../📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
const root=resolve(import.meta.dir,"../../../../../.."),owner=resolve(import.meta.dir,"../..");
const fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🏛️ownership/🔣️.json"),"utf8")) as {owner:string;definition:string;requiredExports:string[];cases:{id:string;kind:"definition";path:string;binding:string}[]};
function publicUses(source:string):string[]{
 const tokens=rustTokens(source),result:string[]=[];
 for(let i=0;i<tokens.length;i++)if(tokens[i]?.text==="pub"&&tokens[i+1]?.text==="use"){
  let end=i+2;while(end<tokens.length&&tokens[end]?.text!==";")end++;
  result.push(tokens.slice(i,end+1).map(token=>token.text).join(""));
 }
 return result.sort();
}
test("closed neutral ownership corpus refuses undeclared and duplicate authority",()=>{
 expect(new Set(fixture.cases.map(row=>row.id)).size).toBe(1);
 expect(fixture.cases.filter(row=>row.kind==="definition").length).toBe(1);
});
test("the neutral SQLite owner exposes its complete controlled surface",async()=>{
 await Parser.init({locateFile:()=>resolve(dirname(fileURLToPath(import.meta.resolve("web-tree-sitter"))),"tree-sitter.wasm")});
 const parser=new Parser();parser.setLanguage(await Parser.Language.load(resolve(dirname(fileURLToPath(import.meta.resolve("tree-sitter-wasms/package.json"))),"out/tree-sitter-rust.wasm")));
 try{
  for(const row of fixture.cases){
   const source=readFileSync(resolve(root,row.path),"utf8"),tree=parser.parse(source);
   try{
    expect(tree.rootNode.hasError()).toBe(false);
    const independent=tree.rootNode.descendantsOfType("use_declaration").filter(node=>node.text.startsWith("pub ")).map(node=>node.text.replace(/\s/gu,"")).sort();
    expect(publicUses(source)).toEqual(independent);
     const refs=inspectRustCompileReferences(source).filter(ref=>ref.kind==="path"&&!ref.directory&&resolve(dirname(resolve(root,row.path)),ref.path)===resolve(root,fixture.definition));
     expect(refs.length).toBe(1);
     const exports=publicUses(source).find(value=>value.startsWith("pubusecomponent::{"));
     for(const symbol of fixture.requiredExports)expect(rustTokens(exports??"").some(token=>token.kind==="identifier"&&token.text===symbol)).toBe(true);
    console.log("[DEBUG] SQLite owner "+JSON.stringify({id:row.id,definition:fixture.owner}));
   }finally{tree.delete();}
  }
 }finally{parser.delete();}
});
test("the SQLite package has only neutral normal dependencies",()=>{
 const manifest=parseToml(readFileSync(resolve(owner,"📦️packages/🦀️rust/Cargo.toml"),"utf8"));
 expect(Object.keys(manifest.dependencies as Record<string,unknown>)).toEqual(["semio-framework-value"]);
 expect(readFileSync(resolve(owner,"📦️packages/🦀️rust/🦀️.rs"),"utf8")).not.toContain("semio_framework_os_kernel");
});
