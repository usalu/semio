import{expect,test}from"bun:test";
import{existsSync,readdirSync,readFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
const root=resolve(import.meta.dir,"../.."),skip=new Set(["node_modules",".cache"]);
const bases:ReadonlyMap<string,string>=new Map([["🔌️backends/📋️registry/🔣️.json","🔌️backends"]]);
function walk(directory:string,out:string[]=[]):string[]{for(const entry of readdirSync(directory,{withFileTypes:true})){if(skip.has(entry.name))continue;const path=join(directory,entry.name);if(entry.isDirectory())walk(path,out);else if(entry.isFile())out.push(path);}return out;}
const specifier=String.raw`(["'\x60])(\.{1,2}\/[^"'\x60\\\r\n$]+)\1`,statements=[new RegExp(String.raw`^(?:import|export)\b[^;"'\x60]*?\bfrom\s*${specifier}`,"gmu"),new RegExp(String.raw`^import\s*${specifier}`,"gmu")],runtime=[new RegExp(String.raw`\bimport\s*\(\s*${specifier}`,"gu"),new RegExp(String.raw`new\s+URL\(\s*${specifier}\s*,\s*import\.meta\.url`,"gu")],virtual=/^\.\/\.semio-/u;
function declared(path:string):readonly{value:string;base:string}[]{
 const text=readFileSync(path,"utf8"),inside=path.slice(root.length+1);
 if(path.endsWith(".json")){const base=bases.get(inside);if(!base)return[];const rows:string[]=[],visit=(value:unknown)=>{if(typeof value==="string"){if(/^\.\.?\/.+\.(?:[cm]?[jt]s|json)$/u.test(value))rows.push(value);}else if(Array.isArray(value))value.forEach(visit);else if(value&&typeof value==="object")Object.values(value).forEach(visit);};visit(JSON.parse(text));return rows.map(value=>({value,base:join(root,base)}));}
 const sites=path.split("/").includes("🧪️tests")?statements:[...statements,...runtime];return sites.flatMap(site=>[...text.matchAll(site)].map(match=>({value:match[2]!,base:dirname(path)}))).filter(row=>!virtual.test(row.value));
}
const files=walk(root).filter(path=>/\.(?:[cm]?[jt]s|json)$/u.test(path));
test("every relative import, dynamic import, URL and registry path in build resolves to an existing file",()=>{
 const missing:string[]=[];
 for(const file of files)for(const row of declared(file))if(!existsSync(resolve(row.base,row.value)))missing.push(`${file.slice(root.length)} -> ${row.value}`);
 expect([...new Set(missing)]).toEqual([]);
});
