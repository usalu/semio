import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {pathToFileURL as fileUrl} from "node:url";
import ts from "typescript";
import corpus from "../../🧫️fixtures/🔌️ownership/🔣️.json";

test("General Surface generated browser owners initialize without Specific graph exports",async()=>{
 const owner=resolve(import.meta.dir,"../../📦️packages/🦀️rust/🕸️bindings");
 const source=ts.createSourceFile("framework_surface.d.ts",readFileSync(resolve(owner,"framework_surface.d.ts"),"utf8"),ts.ScriptTarget.Latest,true);
 const classes=source.statements.filter(ts.isClassDeclaration).filter(statement=>statement.modifiers?.some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword)).map(statement=>statement.name!.text).sort();
 expect(classes).toEqual([...corpus.browserSessions].sort());
 const module=await import(fileUrl(resolve(owner,"framework_surface.js")).href);
 const bytes=readFileSync(resolve(owner,"framework_surface_bg.wasm"));
 const compiled=await WebAssembly.compile(bytes);
 const reflected=WebAssembly.Module.exports(compiled).map(entry=>entry.name);
 const initialized=await module.default({module_or_path:compiled});
 for(const name of corpus.browserSessions){expect(typeof module[name]).toBe("function");expect(reflected.some(symbol=>symbol.startsWith(name.toLowerCase()+"_"))).toBe(true);}
 for(const name of corpus.specificBrowserSessions){expect(name in module).toBe(false);expect(reflected.some(symbol=>symbol.toLowerCase().includes(name.toLowerCase()))).toBe(false);}
 expect(Object.keys(initialized).sort()).toEqual(reflected.sort());
 console.log("[DEBUG] General Surface JS declarations, initialized Wasm and independent WebAssembly export reflection retain only Map/Raster/Terrain sessions");
});
