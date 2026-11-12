/** 📌️ Child-group laws pins: records the laws, mounted source closure and lock edges of every `*-laws` crate beside it. `write` regenerates `📋️.json`, `check` fails on drift. Paths are repo-relative with forward slashes on every platform. See `🧬️.json`. */
import{readFileSync,readdirSync,existsSync,writeFileSync}from"node:fs";
import{resolve,dirname,relative,sep}from"node:path";
import{createHash}from"node:crypto";
import{TOML}from"bun";
import Parser from"web-tree-sitter";
const plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin",hostRoot=plugin+"/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs",originalRoot=plugin+"/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs";
const here=import.meta.dir,domain=resolve(here,".."),pinsFile=resolve(here,"📋️.json");
const findRoot=(dir:string):string=>existsSync(resolve(dir,"AGENTS.md"))&&existsSync(resolve(dir,"Cargo.lock"))?dir:dirname(dir)===dir?(()=>{throw Error("Repository root required")})():findRoot(dirname(dir));
const root=findRoot(here),read=(path:string)=>readFileSync(path,"utf8"),sha=(text:string)=>createHash("sha256").update(text).digest("hex"),rel=(path:string)=>relative(root,path).split(sep).join("/");
const asserts=(text:string)=>(text.match(/\bassert(?:_eq|_ne)?!/g)??[]).length,nonblank=(text:string)=>text.split("\n").filter(line=>line.trim()).length;
const table=(value:unknown):Record<string,unknown>=>value&&typeof value==="object"&&!Array.isArray(value)?Object.fromEntries(Object.entries(value)):(()=>{throw Error("Table required")})();
const parser=async()=>{await Parser.init({wasmBinary:readFileSync(Bun.resolveSync("web-tree-sitter/tree-sitter.wasm",root))});const p=new Parser();p.setLanguage(await Parser.Language.load(Bun.resolveSync("tree-sitter-wasms/out/tree-sitter-rust.wasm",root)));return p;};
const closure=(start:string)=>{const files:string[]=[];const walk=(path:string)=>{if(files.includes(path))return;files.push(path);for(const m of read(path).matchAll(/#\[path\s*=\s*"([^"\n]+)"\]|include!\(\s*"([^"\n]+)"\s*\)/g)){const child=m[1]??m[2];if(!child)throw Error("Literal mount required");walk(resolve(dirname(path),child));}};walk(start);return files;};
const functions=(p:Parser,text:string)=>{const tree=p.parse(text);try{return tree.rootNode.descendantsOfType("function_item").map(fn=>({name:fn.childForFieldName("name")?.text??"",sha256:sha(fn.text)}));}finally{tree.delete();}};
const dependencyNames=(spec:Record<string,unknown>)=>[...new Set(["dependencies","dev-dependencies"].flatMap(key=>Object.entries(table(spec[key]??{})).map(([name,value])=>typeof value==="string"?name:String(table(value).package??name))))].sort();
/** 📌️ Computes the pins from the working tree. */
export async function generate(){
 const p=await parser(),lock=(TOML.parse(read(resolve(root,"Cargo.lock"))) as{package:Record<string,unknown>[]}).package.map(table),crates=[];
 try{
  for(const name of readdirSync(domain).sort()){
   const manifestPath=resolve(domain,name,"📦️packages/🦀️rust/Cargo.toml");if(!existsSync(manifestPath))continue;
   const spec=table(TOML.parse(read(manifestPath))),pkg=table(spec.package);if(!String(pkg.name).endsWith("-laws"))continue;
   const target=resolve(dirname(manifestPath),String(table((spec.test as unknown[])[0]).path)),files=closure(target),laws:{name:string;sha256:string;assertions:number}[]=[];
   for(const file of files){const tree=p.parse(read(file));try{if(tree.rootNode.hasError()&&file.startsWith(domain))throw Error("Parseable Rust required: "+rel(file));for(const fn of tree.rootNode.descendantsOfType("function_item")){let isLaw=false;for(let attr=fn.previousNamedSibling;attr?.type==="attribute_item";attr=attr.previousNamedSibling)if(/^#\[\s*(?:\w+::)*(?:async_)?test\s*[(\]]/.test(attr.text))isLaw=true;const fnName=fn.childForFieldName("name")?.text;if(isLaw&&fnName&&file.startsWith(domain))laws.push({name:fnName,sha256:sha(fn.text),assertions:asserts(fn.text)});}}finally{tree.delete();}}
   const entries=lock.filter(row=>row.name===pkg.name);if(entries.length!==1)throw Error("Exactly one lock entry required: "+pkg.name);
   const dependencies=(entries[0]!.dependencies as string[]).map(row=>row.split(" ")[0]!).sort(),declared=dependencyNames(spec);
   if(JSON.stringify(dependencies)!==JSON.stringify(declared))throw Error("Lock edges differ from manifest: "+pkg.name);
   crates.push({package:String(pkg.name),manifest:rel(manifestPath),target:rel(target),laws:laws.sort((a,b)=>a.name<b.name?-1:1),closure:files.map(file=>{const text=read(file);return{path:rel(file),bytes:Buffer.byteLength(text),nonblank:nonblank(text),sha256:sha(text)};}),dependencies});
  }
  const lawFiles=new Set(crates.flatMap(crate=>crate.closure.map(file=>file.path))),remaining=[...new Set([hostRoot,originalRoot].flatMap(start=>closure(resolve(root,start)).map(rel)))].filter(path=>!lawFiles.has(path)).sort().map(path=>{const text=read(resolve(root,path));if(/mod\s+child_complete_group_candidate_tests\b/.test(text))throw Error("Law module must live only in a laws crate: "+path);return{path,sha256:sha(text),functions:functions(p,text)};});
  const laws=crates.flatMap(crate=>crate.laws),names=new Set(laws.map(law=>law.name));
  const debugStatements=[...lawFiles].reduce((n,path)=>n+(read(resolve(root,path)).match(/\beprintln!|\[DEBUG\]/g)??[]).length,0);
  return{version:1,uniqueObligations:names.size,activeInstances:laws.length,preservedFromInstances:32,assertions:laws.reduce((n,law)=>n+law.assertions,0),debugStatements,crates,host:hostRoot,original:originalRoot,remaining};
 }finally{p.delete();}
}
const render=(value:unknown)=>JSON.stringify(value,null,2)+"\n";
if(import.meta.main){
 const mode=process.argv[2],next=render(await generate());
 if(mode==="write"){writeFileSync(pinsFile,next);console.log("wrote "+rel(pinsFile));}
 else if(mode==="check"){if(!existsSync(pinsFile)||read(pinsFile)!==next){console.error("Child-group pins drifted: run `repo run @semio-tech/framework-plugin:child-group-pins`");process.exit(1);}console.log("child-group pins current");}
 else{console.error("Usage: 📜️.ts write|check");process.exit(2);}
}
