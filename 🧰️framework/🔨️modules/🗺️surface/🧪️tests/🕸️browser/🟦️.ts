import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {pathToFileURL as fileUrl} from "node:url";
import ts from "typescript";
import corpus from "../🧫️fixtures/🔌️ownership/🔣️.json";

test("General Surface generated browser owners initialize without Specific graph exports",async()=>{
 const files=new Map<string,string>(JSON.parse(process.env.SEMIO_SURFACE_WASM_PACKAGE??"[]")),file=(name:string)=>{const path=files.get(name);if(!path)throw Error(`Produced surface package lacks ${name}`);return path};
 const source=ts.createSourceFile("framework_surface.d.ts",readFileSync(file("framework_surface.d.ts"),"utf8"),ts.ScriptTarget.Latest,true);
 const classes=source.statements.filter(ts.isClassDeclaration).filter(statement=>statement.modifiers?.some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword)).map(statement=>statement.name!.text).sort();
 expect(classes).toEqual([...corpus.browserSessions].sort());
 const module=await import(fileUrl(file("framework_surface.js")).href);
 const bytes=readFileSync(file("framework_surface_bg.wasm"));
 const compiled=await WebAssembly.compile(bytes);
 const reflected=WebAssembly.Module.exports(compiled).map(entry=>entry.name);
 const initialized=await module.default({module_or_path:compiled});
 for(const name of corpus.browserSessions){expect(typeof module[name]).toBe("function");expect(reflected.some(symbol=>symbol.startsWith(name.toLowerCase()+"_"))).toBe(true);}
 for(const name of corpus.specificBrowserSessions){expect(name in module).toBe(false);expect(reflected.some(symbol=>symbol.toLowerCase().includes(name.toLowerCase()))).toBe(false);}
 expect(Object.keys(initialized).sort()).toEqual(reflected.sort());
});
