import {expect,test} from "bun:test";
import {readFileSync,readdirSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import glob from "fast-glob";
import {minimatch} from "minimatch";
import ancestry from "../🧫️fixtures/🧭️fixture-placement/🔣️.json";

const modules=resolve(import.meta.dir,"../../..");
const owners=["🎭️actor","🧵️job"];
function files(root:string):string[]{return readdirSync(root,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?files(join(root,entry.name)):entry.isFile()?[join(root,entry.name)]:[]);}
function belongsToTests(path:string):boolean{const parts=path.split(/[\\/]/),fixtures=parts.indexOf("🧫️fixtures"),tests=parts.indexOf("🧪️tests");return tests>=0&&tests<fixtures&&!parts.slice(0,fixtures).some(part=>["📦️packages","🎯️targets","🧩️implementations"].includes(part));}

test("Actor and Job examples belong strictly to neutral tests",()=>{
 const all=owners.flatMap(owner=>files(join(modules,owner))),actual=all.filter(path=>path.split(/[\\/]/).includes("🧫️fixtures"));
 const independent=glob.sync(owners.map(owner=>`${owner}/**/🧫️fixtures/**/*`),{cwd:modules,absolute:true,onlyFiles:true});
 expect(actual.map(path=>relative(modules,path)).sort()).toEqual(independent.map(path=>relative(modules,path)).sort());
 expect(actual.length).toBeGreaterThan(0);
 expect(actual.filter(path=>!belongsToTests(relative(modules,path)))).toEqual([]);
 for(const path of actual){expect(path.length).toBeLessThanOrEqual(256);expect(Array.from(path).length).toBeLessThanOrEqual(256);}
 const oracle=new Set(independent.map(path=>resolve(path)));let includes=0;
 for(const path of all.filter(path=>path.endsWith(".rs")))for(const match of readFileSync(path,"utf8").matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"/g)){const target=resolve(dirname(path),match[1]!);if(owners.includes(relative(modules,target).split(/[\\/]/)[0]!)){expect(oracle.has(target)).toBe(true);includes++;}}
});

test("neutral fixture ancestry vectors agree with independent minimatch",()=>{
 for(const row of ancestry.cases){const oracle=minimatch(row.path,"**/🧪️tests/**/🧫️fixtures/**")&&!["📦️packages","🎯️targets","🧩️implementations"].some(owner=>minimatch(row.path,`**/${owner}/**/🧫️fixtures/**`));expect(oracle,row.id).toBe(row.accepted);expect(belongsToTests(row.path),row.id).toBe(oracle);}
});
