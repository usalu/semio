/** 🔒️ Contract: no component of the build inference declares a module level `Map` or `Set` that lives as long as the process (a daemon keeps it across revisions and workspaces and it grows with the repository); long lived state uses `createBoundedCacheV1`, per run state lives in the run's own object, weak maps are allowed, upper case names are fixed tables.
 * @see ../../../../🧊️bounded/🟨️.mjs */
import{expect,test}from"bun:test";
import{readdirSync,readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";

const inference=join(dirname(fileURLToPath(import.meta.url)),"..","..");
const sources=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?entry.name==="🧪️tests"?[]:sources(join(directory,entry.name)):/\.(mjs|ts)$/u.test(entry.name)&&!entry.name.endsWith(".d.mts")?[join(directory,entry.name)]:[]);
const declarator=/(?:^|,|\b(?:const|let|var)\s+)\s*([A-Za-z_$][\w$]*)\s*=\s*new (?:Map|Set)\(/gu;

test("no component of the build inference declares a module level Map or Set",()=>{
 const found:string[]=[];
 for(const path of sources(inference))readFileSync(path,"utf8").split("\n").forEach((line,at)=>{
  if(!/^(?:export )?(?:const|let|var)\s/u.test(line))return;
  for(const match of line.matchAll(declarator))if(!/[{(]|=>/u.test(line.slice(0,match.index))&&!/^[A-Z][A-Z0-9_]*$/u.test(match[1]!))found.push(path.slice(inference.length)+":"+(at+1)+" "+match[1]);
 });
 expect(found).toEqual([]);
});

test("the lint recognises every declarator shape of a module level collection",()=>{
 const line="const first = new Map(), second = new Set(), THIRD = new Map();";
 expect([...line.matchAll(declarator)].map(match=>match[1])).toEqual(["first","second","THIRD"]);
 expect("export const x=new Set();".replace(/^export /u,"").match(declarator)?.length).toBe(1);
});
