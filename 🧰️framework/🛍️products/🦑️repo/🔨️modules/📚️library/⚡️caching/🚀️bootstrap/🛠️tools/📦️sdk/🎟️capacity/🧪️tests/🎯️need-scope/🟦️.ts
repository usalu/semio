import{test,expect}from"bun:test";
import{readFileSync,readdirSync}from"node:fs";
import{join,relative}from"node:path";
const library=join(import.meta.dir,"..","..","..","..","..","..","..");
const walk=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?(["node_modules",".cache","🧪️tests"].includes(entry.name)?[]:walk(join(directory,entry.name))):/\.(?:ts|cjs|mjs)$/u.test(entry.name)?[join(directory,entry.name)]:[]);
const callers=(name:string)=>["⚡️caching","🏗️build"].flatMap(root=>walk(join(library,root))).filter(path=>readFileSync(path,"utf8").includes(name+"(")).map(path=>relative(library,path)).sort();
test("the SDK derivative is planned only by the Nx build tool, never by a target of another tool",()=>{
 expect(callers("prepareNxToolingStorageV1")).toEqual(["⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts","⚡️caching/🚀️bootstrap/📜️script.ts","🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts"].sort());
 expect(callers("planNxToolingStorageV1")).toEqual(["⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"]);
});
test("the Nx script prepares the SDK storage once per invocation, before any other storage of the same command",()=>{
 const body=readFileSync(join(library,"⚡️caching/🚀️bootstrap/📜️script.ts"),"utf8");expect(body.split("prepareNxToolingStorageV1(").length-1).toBe(1);expect(body.indexOf("prepareNxToolingStorageV1(")).toBeLessThan(body.indexOf("prepareNxGraphCacheStorageV1("));
});
