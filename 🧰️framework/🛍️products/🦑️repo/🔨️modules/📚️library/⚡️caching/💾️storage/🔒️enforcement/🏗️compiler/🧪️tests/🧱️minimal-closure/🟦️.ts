/** 🧱️ Contract: the compiler entries (the compiler store and the unit compiler script) never reach the JavaScript dependency bootstrap or the Bun workspace module; the rustup install import once pulled both in through the system tool port and the package-manager catalog. Oracles: esbuild's bundle graph (what the unit producer census consumes) and a plain import walker whose result must contain the bundle graph. */
import{expect,test}from"bun:test";
import{existsSync,readFileSync}from"node:fs";
import{createRequire}from"node:module";
import{dirname,join,resolve}from"node:path";
import corpus from"./🔣️.json"with{type:"json"};
const root=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const pattern=/(?:^|[;\s}])(?:import|export)\s*(?:type\s+)?(?:[^"'`;]*?from\s*)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)/g;
function walk(entry:string):Set<string>{const seen=new Set<string>([entry]),pending=[entry];while(pending.length){const file=pending.pop()!;if(!/\.(ts|mts|js|mjs)$/u.test(file))continue;for(const match of readFileSync(file,"utf8").matchAll(pattern)){const specifier=(match[1]??match[2])!;if(specifier.startsWith("node:")||specifier.startsWith("bun:"))continue;let target:string;try{target=Bun.resolveSync(specifier,dirname(file));}catch{continue;}if(target.includes("/node_modules/")||seen.has(target))continue;seen.add(target);pending.push(target);}}return seen;}
for(const name of corpus.entries)test("the bundle graph of "+name.split("/").slice(-3).join("/")+" excludes the JavaScript dependency bootstrap and the Bun workspace module",async()=>{
 const entry=join(root,name),{build}=createRequire(import.meta.url)("esbuild"),graph=await build({absWorkingDir:root,entryPoints:[entry],bundle:true,write:false,metafile:true,platform:"node",format:"esm",logLevel:"silent",external:["bun:*"],plugins:[{name:"first-party",setup(build:any){build.onResolve({filter:/^@semio-tech\//},(request:any)=>({path:Bun.resolveSync(request.path,request.resolveDir)}));}}]});
 const bundled=Object.keys(graph.metafile.inputs).map(path=>resolve(root,path.replace(/ with \{.*$/u,""))).filter(path=>!path.includes("/node_modules/")),walked=walk(entry);
 expect(bundled.length).toBeGreaterThanOrEqual(corpus.minimumFiles);
 for(const path of bundled){for(const segment of corpus.forbidden)expect(path.includes(segment),path).toBe(false);if(/\.(ts|mts|js|mjs)$/u.test(path))expect(walked.has(path),path).toBe(true);}
},120000);
