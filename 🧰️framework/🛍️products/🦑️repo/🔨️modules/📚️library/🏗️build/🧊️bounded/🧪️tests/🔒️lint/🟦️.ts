/** 🔒️ Contract: no module of the index and generation code declares a module level `Map` or `Set` (it would live as long as the process and grow with the repository); long lived state uses `createBoundedCacheV1`, per call state lives inside a function. Constants in upper case are allowed. */
import{expect,test}from"bun:test";
import{readdirSync,readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";

const build=join(dirname(fileURLToPath(import.meta.url)),"..","..",".."),roots=[join(build,"🕸️graph","🕰️generations"),join(build,"🕸️graph","⏱️phases")];
const sources=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?entry.name==="🧪️tests"?[]:sources(join(directory,entry.name)):/\.(mjs|ts)$/u.test(entry.name)&&!entry.name.endsWith(".d.mts")?[join(directory,entry.name)]:[]);
const declarator=/(?:^|,)\s*([A-Za-z_$][\w$]*)\s*=\s*new (?:Map|Set)\(/gu;

test("the index and generation modules declare no module level Map or Set",()=>{
 const found:string[]=[];
 for(const root of roots)for(const path of sources(root))readFileSync(path,"utf8").split("\n").forEach((line,at)=>{
  if(!/^(?:export )?(?:const|let|var)\s/u.test(line))return;
  for(const match of line.matchAll(declarator))if(!/^[A-Z][A-Z0-9_]*$/u.test(match[1]!))found.push(path.slice(build.length)+":"+(at+1)+" "+match[1]);
 });
 expect(found).toEqual([]);
});
