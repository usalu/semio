/** 🏭️ The self-contained program a hermetic Bazel action runs under the declared Bun toolchain: it holds no repository import, no absolute path and no environment inheritance, only the manifest, the dependency snapshots and the declared sources of its execution root. */
export const BAZEL_HERMETIC_RUNNER=String.raw`import{createHash}from"node:crypto";
import{chmodSync,copyFileSync,lstatSync,mkdirSync,readFileSync,readdirSync,writeFileSync}from"node:fs";
import{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";
const[manifestPath,outputPath,...snapshots]=process.argv.slice(2),sha=value=>createHash("sha256").update(value).digest("hex"),slash=value=>value.split(sep).join("/"),fail=message=>{throw new Error("[hermetic] "+message);},order=(left,right)=>left<right?-1:left>right?1:0;
if(!manifestPath||!outputPath)fail("manifest and output required");
const manifest=JSON.parse(readFileSync(manifestPath,"utf8")),workspace=resolve("s"),executable=process.execPath,scratch=resolve(".hermetic");
if(manifest.version!==1)fail("manifest version");
try{if(lstatSync(workspace).isSymbolicLink())fail("an isolated execution root is required: run the action sandboxed or remote");}catch(error){if(error.code==="ENOENT")mkdirSync(workspace,{recursive:true});else throw error;}
const inside=path=>{const target=resolve(workspace,path),inner=relative(workspace,target);if(inner===".."||inner.startsWith(".."+sep)||isAbsolute(inner))fail("path escapes the execution root: "+path);return target;},
 expand=value=>value.replaceAll("{workspaceRoot}",workspace).replaceAll("{projectRoot}",inside(manifest.root||".")).replaceAll("{projectName}",manifest.project),
 normalized=info=>info.isDirectory()||(info.mode&0o111)?0o755:0o644,
 sorted=directory=>readdirSync(directory).sort(order);
for(const snapshot of snapshots){
 const content=JSON.parse(readFileSync(join(snapshot,"snapshot.json"),"utf8"));if(content.version!==1||!Array.isArray(content.entries))fail("snapshot rows");
 for(const entry of [...content.entries].sort((left,right)=>left.path.length-right.path.length||order(left.path,right.path))){
  const target=inside(entry.path);if(!/^[a-f0-9]{32}$/.test(entry.file??"0".repeat(32)))fail("snapshot file identity");
  if(entry.kind==="directory"){mkdirSync(target,{recursive:true});continue;}
  if(entry.kind!=="file")fail("snapshot kind refused: "+entry.kind);
  try{lstatSync(target);fail("a dependency output overlaps a declared input: "+entry.path);}catch(error){if(error.code!=="ENOENT")throw error;}
  mkdirSync(dirname(target),{recursive:true});copyFileSync(join(snapshot,"files",entry.file),target);chmodSync(target,entry.mode&0o755|0o600);
 }
}
const files=()=>{const found=new Map(),visit=directory=>{for(const name of sorted(directory)){const path=join(directory,name),info=lstatSync(path);if(info.isDirectory())visit(path);else found.set(slash(relative(workspace,path)),info.size+":"+info.mtimeMs+":"+info.ino);}};visit(workspace);return found;};
const before=files();
mkdirSync(join(scratch,"home"),{recursive:true});mkdirSync(join(scratch,"tmp"),{recursive:true});
const environment={PATH:dirname(executable),HOME:join(scratch,"home"),TMPDIR:join(scratch,"tmp"),TEMP:join(scratch,"tmp"),TMP:join(scratch,"tmp"),LC_ALL:"C",TZ:"UTC",SOURCE_DATE_EPOCH:"0",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",SEMIO_BUILD_TOOL:"bazel",SEMIO_HERMETIC:"1",SEMIO_WORKSPACE_ROOT:workspace,SEMIO_TASK_PROJECT:manifest.project,SEMIO_TASK_TARGET:manifest.target,...(manifest.configuration?{SEMIO_TASK_CONFIGURATION:manifest.configuration}:{}),...(manifest.env??{})};
const cwd=manifest.cwd?resolve(workspace,expand(manifest.cwd)):workspace;inside(relative(workspace,cwd)||".");mkdirSync(cwd,{recursive:true});
const quote=value=>"'"+value.replaceAll("'","'\\''")+"'",suffix=manifest.args.length&&manifest.forwardAllArgs!==false?" "+manifest.args.map(quote).join(" "):"";
const run=async command=>{const child=Bun.spawn([executable,"exec",expand(command)+suffix],{cwd,env:environment,stdin:"ignore",stdout:"inherit",stderr:"inherit"}),code=await child.exited;if(code!==0)fail("command failed ("+code+"): "+command);};
if(manifest.parallel===false)for(const command of manifest.commands)await run(command);else await Promise.all(manifest.commands.map(run));
const roots=new Set();
for(const pattern of manifest.outputs){
 const path=slash(relative(workspace,resolve(workspace,expand(pattern))));inside(path);
 if(/[?*{[]/.test(path)){for(const match of new Bun.Glob(path).scanSync({cwd:workspace,onlyFiles:false,dot:true}))roots.add(slash(match));}
 else try{lstatSync(inside(path));roots.add(path);}catch(error){if(error.code!=="ENOENT")throw error;}
}
const ordered=[...roots].sort(order).filter(path=>![...roots].some(parent=>parent!==path&&path.startsWith(parent+"/"))),entries=[];
mkdirSync(join(outputPath,"files"),{recursive:true});
const visit=path=>{
 const source=inside(path),info=lstatSync(source);
 if(info.isSymbolicLink())fail("a hermetic output must not be a symbolic link: "+path);
 if(info.isDirectory()){entries.push({path,kind:"directory",mode:0o755});for(const name of sorted(source))visit(path+"/"+name);return;}
 if(!info.isFile())fail("a hermetic output must be a file or directory: "+path);
 const bytes=readFileSync(source),id=sha(path).slice(0,32);writeFileSync(join(outputPath,"files",id),bytes);entries.push({path,kind:"file",mode:normalized(info),file:id,sha256:sha(bytes)});
};
for(const path of ordered)visit(path);
entries.sort((left,right)=>order(left.path,right.path));
const claimed=new Set(entries.map(entry=>entry.path)),undeclared=[...files()].filter(([path,identity])=>before.get(path)!==identity&&!claimed.has(path)&&![...roots].some(root=>path.startsWith(root+"/"))).map(([path])=>path).sort(order);
if(undeclared.length)fail("undeclared writes outside the declared outputs: "+undeclared.slice(0,16).join(", "));
writeFileSync(join(outputPath,"snapshot.json"),JSON.stringify({version:1,roots:[...roots].sort(order),entries}));
`;
