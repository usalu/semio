import {expect,test} from "bun:test";
import {readFileSync,readdirSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import glob from "fast-glob";
import ts from "typescript";
import {minimatch} from "minimatch";
import {fixtureFacadeTargets} from "@semio-tech/framework/test/source-text";
import ancestry from "../🧫️fixtures/🧭️fixture-placement/🔣️.json";

const products=resolve(import.meta.dir,"../../../../..");
const owners=ancestry.owners;
function files(root:string):string[]{return readdirSync(root,{withFileTypes:true}).filter(entry=>!["node_modules",".git",".nx",".🧬semio","🗑️generated","target"].includes(entry.name)).flatMap(entry=>entry.isDirectory()?files(join(root,entry.name)):entry.isFile()?[join(root,entry.name)]:[]);}
function belongsToTests(path:string):boolean{const parts=path.split(/[\\/]/),fixtures=parts.indexOf("🧫️fixtures"),tests=parts.indexOf("🧪️tests");return tests>=0&&tests<fixtures&&!parts.slice(0,fixtures).some(part=>["🧬️schema","📦️packages","🎯️targets","🧩️implementations"].includes(part));}
function placedFacade(path:string):boolean{const targets=fixtureFacadeTargets(path);return targets!==undefined&&targets.every(target=>belongsToTests(relative(products,target)));}

test("Process, Hub and additional owner examples belong strictly to neutral tests",()=>{
 const all=[...new Set(owners.flatMap(owner=>files(join(products,owner))))],actual=all.filter(path=>path.split(/[\\/]/).includes("🧫️fixtures"));
 const independent=glob.sync(owners.map(owner=>`${owner}/**/🧫️fixtures/**/*`),{cwd:products,absolute:true,onlyFiles:true,followSymbolicLinks:false,ignore:["**/node_modules/**","**/.git/**","**/.nx/**","**/.🧬semio/**","**/🗑️generated/**","**/target/**"]});
 expect(actual.map(path=>relative(products,path)).sort()).toEqual(independent.map(path=>relative(products,path)).sort());
 expect(actual.length).toBeGreaterThan(0);
 expect(actual.filter(path=>!belongsToTests(relative(products,path))&&!placedFacade(path))).toEqual([]);
 for(const path of actual){expect(path.length).toBeLessThanOrEqual(256);expect(Array.from(path).length).toBeLessThanOrEqual(256);}
 const oracle=new Set(independent.map(path=>resolve(path)));let includes=0;
 for(const path of all.filter(path=>path.endsWith(".rs")))for(const match of readFileSync(path,"utf8").matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"/g)){const target=resolve(dirname(path),match[1]!);if(owners.some(owner=>relative(products,target)===owner||relative(products,target).startsWith(owner+"/"))){expect(oracle.has(target),path+" -> "+match[1]).toBe(true);includes++;}}
},30_000);

test("original product fixture imports resolve against independent physical inventory",()=>{
 const all=[...new Set(owners.flatMap(owner=>files(join(products,owner))))],oracle=new Set(glob.sync(owners.map(owner=>`${owner}/**/🧫️fixtures/**/*`),{cwd:products,absolute:true,onlyFiles:true,followSymbolicLinks:false,ignore:["**/node_modules/**","**/.git/**","**/.nx/**","**/.🧬semio/**","**/🗑️generated/**","**/target/**"]}).map(path=>resolve(path)));let imports=0;
 for(const path of all.filter(path=>/\.tsx?$/.test(path))){const text=readFileSync(path,"utf8");if(!text.includes("🧫️fixtures"))continue;const source=ts.createSourceFile(path,text,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);function visit(node:ts.Node):void{if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier)&&node.moduleSpecifier.text.includes("🧫️fixtures")){const target=resolve(dirname(path),node.moduleSpecifier.text);if(owners.some(owner=>relative(products,target)===owner||relative(products,target).startsWith(owner+"/"))){expect(oracle.has(target),path+" -> "+node.moduleSpecifier.text).toBe(true);imports++;}}ts.forEachChild(node,visit);}visit(source);}
},30_000);

test("neutral fixture ancestry vectors agree with independent minimatch",()=>{
 for(const row of ancestry.cases){const oracle=minimatch(row.path,"**/🧪️tests/**/🧫️fixtures/**")&&!["🧬️schema","📦️packages","🎯️targets","🧩️implementations"].some(owner=>minimatch(row.path,`**/${owner}/**/🧫️fixtures/**`));expect(oracle,row.id).toBe(row.accepted);expect(belongsToTests(row.path),row.id).toBe(oracle);}
});
