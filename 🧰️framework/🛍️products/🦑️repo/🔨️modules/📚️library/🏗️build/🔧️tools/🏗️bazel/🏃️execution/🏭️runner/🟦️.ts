/** 🏭️ The self-contained program a hermetic Bazel action runs under the declared Bun toolchain: it holds no repository import, no absolute path and no environment inheritance, only the manifest, the optional `--modules=<dependency tree>`, the dependency snapshots and the declared inputs of its execution root. It builds its own isolated workspace from the declared input list when Bazel did not isolate one (a sandbox symlinks the declared files back into the checkout, a local strategy links the whole checkout as the root), so an undeclared read fails on every host and strategy; it links the workspace packages and the dependency-tree entries into `node_modules` directories, issues the original process authority through the manifest's entry module and refuses any write outside the declared outputs. */
export const BAZEL_HERMETIC_RUNNER=String.raw`import{createHash}from"node:crypto";
import{chmodSync,constants,copyFileSync,lstatSync,mkdirSync,readFileSync,readdirSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";
import{pathToFileURL}from"node:url";
const flag=process.argv.slice(2).find(argument=>argument.startsWith("--modules=")),[manifestPath,outputArgument,...snapshots]=process.argv.slice(2).filter(argument=>argument!==flag),sha=value=>createHash("sha256").update(value).digest("hex"),slash=value=>value.split(sep).join("/"),fail=message=>{throw new Error("[hermetic] "+message);},order=(left,right)=>left<right?-1:left>right?1:0,clone=(from,to)=>{try{copyFileSync(from,to,constants.COPYFILE_FICLONE);}catch{copyFileSync(from,to);}};
if(!manifestPath||!outputArgument)fail("manifest and output required");
const manifest=JSON.parse(readFileSync(manifestPath,"utf8")),executable=process.execPath,outputPath=resolve(outputArgument),modules=flag?resolve(flag.slice(10)):undefined,scratch=resolve(".hermetic",sha(outputArgument).slice(0,16)),declared=manifest.inputs??[];
if(manifest.version!==1)fail("manifest version");
const source=resolve("s"),linked=declared.length>0&&(lstatSync(source).isSymbolicLink()||lstatSync(join(source,declared[0])).isSymbolicLink()),workspace=linked?join(scratch,"root","s"):source;
try{if(!linked&&lstatSync(workspace).isSymbolicLink())fail("an isolated execution root is required: declare the inputs");}catch(error){if(error.code==="ENOENT")mkdirSync(workspace,{recursive:true});else throw error;}
const inside=path=>{const target=resolve(workspace,path),inner=relative(workspace,target);if(inner===".."||inner.startsWith(".."+sep)||isAbsolute(inner))fail("path escapes the execution root: "+path);return target;},
 expand=value=>value.replaceAll("{workspaceRoot}",workspace).replaceAll("{projectRoot}",inside(manifest.root||".")).replaceAll("{projectName}",manifest.project),
 normalized=info=>info.isDirectory()||(info.mode&0o111)?0o755:0o644,
 sorted=directory=>readdirSync(directory).sort(order);
if(linked){rmSync(join(scratch,"root"),{recursive:true,force:true});for(const path of declared){const target=inside(path);mkdirSync(dirname(target),{recursive:true});clone(join(source,path),target);}}
for(const marker of manifest.markers??[]){const target=inside(marker);try{lstatSync(target);}catch(error){if(error.code!=="ENOENT")throw error;mkdirSync(dirname(target),{recursive:true});writeFileSync(target,"{}");}}
const link=(target,path)=>{mkdirSync(dirname(path),{recursive:true});symlinkSync(target,path,"junction");};
const shim=(target,path,platform=process.platform)=>{if(platform!=="win32"){symlinkSync(target,path);return;}const real=resolve(dirname(path),target);if(/[%"\r\n]/.test(real+executable))fail("a tool path with a percent sign, a quote or a line break cannot be shimmed: "+real);if(/\.(exe|com)$/i.test(real)){copyFileSync(real,path+".exe");return;}readFileSync(real);writeFileSync(path+".cmd",'@"'+executable+'" "'+real+'" %*\r\n');};
for(const row of manifest.nodeModules??[]){
 const directory=inside(row.path);
 for(const[name,path]of Object.entries(row.workspace))link(inside(path),join(directory,name));
 for(const[name,path]of Object.entries(row.tree)){if(!modules)fail("a dependency tree is required for "+row.path);link(join(modules,path),join(directory,name));}
 for(const[name,target]of Object.entries(row.bin)){mkdirSync(join(directory,".bin"),{recursive:true});shim(target,join(directory,".bin",name));}
}
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
const binaries=[join(workspace,"node_modules",".bin"),...(manifest.nodeModules??[]).map(row=>join(inside(row.path),".bin"))].filter((path,index,all)=>all.indexOf(path)===index);
const environment={PATH:[dirname(executable),...binaries].join(process.platform==="win32"?";":":"),HOME:join(scratch,"home"),TMPDIR:join(scratch,"tmp"),TEMP:join(scratch,"tmp"),TMP:join(scratch,"tmp"),LC_ALL:"C",TZ:"UTC",SOURCE_DATE_EPOCH:"0",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",SEMIO_BUILD_TOOL:"bazel",SEMIO_HERMETIC:"1",SEMIO_WORKSPACE_ROOT:workspace,SEMIO_TASK_PROJECT:manifest.project,SEMIO_TASK_TARGET:manifest.target,...(manifest.configuration?{SEMIO_TASK_CONFIGURATION:manifest.configuration}:{}),...(manifest.inputEnv??{}),...(manifest.env??{})};
if(manifest.entry){const entry=await import(pathToFileURL(inside(manifest.entry)).href);Object.assign(environment,entry.hermeticEnvironmentV1({workspace,providers:manifest.providers??{},maximumElapsedMilliseconds:manifest.maximumElapsedMilliseconds,buildMode:environment.SEMIO_BUILD_MODE??"prototype"}));}
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
 const target=inside(path),info=lstatSync(target);
 if(info.isSymbolicLink())fail("a hermetic output must not be a symbolic link: "+path);
 if(info.isDirectory()){entries.push({path,kind:"directory",mode:0o755});for(const name of sorted(target))visit(path+"/"+name);return;}
 if(!info.isFile())fail("a hermetic output must be a file or directory: "+path);
 const bytes=readFileSync(target),id=sha(path).slice(0,32);writeFileSync(join(outputPath,"files",id),bytes);entries.push({path,kind:"file",mode:normalized(info),file:id,sha256:sha(bytes)});
};
for(const path of ordered)visit(path);
entries.sort((left,right)=>order(left.path,right.path));
const claimed=new Set(entries.map(entry=>entry.path)),undeclared=[...files()].filter(([path,identity])=>before.get(path)!==identity&&!claimed.has(path)&&![...roots].some(root=>path.startsWith(root+"/"))).map(([path])=>path).sort(order);
if(undeclared.length)fail("undeclared writes outside the declared outputs: "+undeclared.slice(0,16).join(", "));
writeFileSync(join(outputPath,"snapshot.json"),JSON.stringify({version:1,roots:[...roots].sort(order),entries}));
rmSync(scratch,{recursive:true,force:true});
`;

/** 🏭️ The self-contained program that builds the content-addressed dependency tree of a hermetic action: it copies the named store entries of the installed generations into one directory `<store index>/<identity>`, keeps every symbolic link relative and inside its store (so the tree is relocatable) and fails on an absent entry or an escaping link. It reads the installed stores by the absolute paths of its manifest, so it is a host-state action; its output is what hermetic actions take as an input by content. */
export const BAZEL_HERMETIC_DEPENDENCIES=String.raw`import{constants,copyFileSync,lstatSync,mkdirSync,readFileSync,readdirSync,readlinkSync,symlinkSync}from"node:fs";
import{dirname,isAbsolute,join,relative,resolve,sep}from"node:path";
const[manifestPath,outputArgument]=process.argv.slice(2),fail=message=>{throw new Error("[hermetic-dependencies] "+message);};
if(!manifestPath||!outputArgument)fail("manifest and output required");
const manifest=JSON.parse(readFileSync(manifestPath,"utf8")),output=resolve(outputArgument);
if(manifest.version!==1)fail("manifest version");
const copy=(from,to,store)=>{
 const info=lstatSync(from);
 if(info.isSymbolicLink()){const target=readlinkSync(from);if(isAbsolute(target))fail("an absolute link is not relocatable: "+from);const inner=relative(store,resolve(dirname(from),target));if(inner===".."||inner.startsWith(".."+sep)||isAbsolute(inner))fail("a link leaves its store: "+from);mkdirSync(dirname(to),{recursive:true});symlinkSync(target,to);return;}
 if(info.isDirectory()){mkdirSync(to,{recursive:true});for(const name of readdirSync(from).sort())copy(join(from,name),join(to,name),store);return;}
 if(!info.isFile())fail("a dependency entry must be a file, directory or link: "+from);
 mkdirSync(dirname(to),{recursive:true});try{copyFileSync(from,to,constants.COPYFILE_FICLONE);}catch{copyFileSync(from,to);}
};
mkdirSync(output,{recursive:true});
for(const id of manifest.ids){const cut=id.indexOf("/"),store=manifest.stores[Number(id.slice(0,cut))];if(!store)fail("store index "+id);copy(join(store,id.slice(cut+1)),join(output,id),store);}
`;
