import {expect,test} from "bun:test";
import {readFileSync,readdirSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import glob from "fast-glob";

const modules=resolve(import.meta.dir,"../../..");
const owners=["🎒️pack","📡️replication"];
function files(root:string):string[]{return readdirSync(root,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?files(join(root,entry.name)):entry.isFile()?[join(root,entry.name)]:[]);}

test("Pack and Replication examples belong strictly to neutral tests",()=>{
 const actual=owners.flatMap(owner=>files(join(modules,owner))).filter(path=>path.split(/[\\/]/).includes("🧫️fixtures"));
 const independent=glob.sync(owners.map(owner=>`${owner}/**/🧫️fixtures/**/*`),{cwd:modules,absolute:true,onlyFiles:true});
 expect(actual.map(path=>relative(modules,path)).sort()).toEqual(independent.map(path=>relative(modules,path)).sort());
 expect(actual.length).toBeGreaterThan(0);
 const invalid=actual.filter(path=>{const parts=relative(modules,path).split(/[\\/]/),fixtures=parts.indexOf("🧫️fixtures"),tests=parts.indexOf("🧪️tests");return tests<0||tests>=fixtures||parts.slice(0,fixtures).some(part=>["📦️packages","🎯️targets","🧩️implementations"].includes(part));});
 expect(invalid).toEqual([]);
 for(const path of actual){expect(path.length).toBeLessThanOrEqual(256);expect(Array.from(path).length).toBeLessThanOrEqual(256);}
 const native=owners.flatMap(owner=>files(join(modules,owner))).filter(path=>path.endsWith(".rs")),oracle=new Set(independent.map(path=>resolve(path)));
 let includes=0;
 for(const path of native)for(const match of readFileSync(path,"utf8").matchAll(/include_(?:str|bytes)!\(\s*"([^"]*🧫️fixtures[^"]*)"/g)){const target=resolve(dirname(path),match[1]!);if(owners.includes(relative(modules,target).split(/[\\/]/)[0]!)){expect(oracle.has(target)).toBe(true);includes++;}}
 expect(includes).toBeGreaterThan(0);
 console.log(`[DEBUG] Node traversal and independent fast-glob agree on ${actual.length} Pack/Replication examples under neutral tests`);
 console.log(`[DEBUG] ${includes} native fixture includes resolve against the independent neutral example inventory`);
});
