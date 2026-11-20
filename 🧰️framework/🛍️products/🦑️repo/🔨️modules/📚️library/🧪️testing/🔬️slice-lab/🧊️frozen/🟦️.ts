import {spawnSync} from "node:child_process";
import {closeSync,constants,copyFileSync,existsSync,fstatSync,ftruncateSync,futimesSync,lstatSync,mkdirSync,mkdtempSync,openSync,readdirSync,readFileSync,readlinkSync,realpathSync,renameSync,rmSync,symlinkSync,writeSync,writeFileSync} from "node:fs";
import {dirname,join,resolve,sep} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const F=schema.definitions.Policy.const.frozen,MEGABYTE=1048576,CLONE_SCRIPT="import ctypes,sys\nl=ctypes.CDLL(\"/usr/lib/libSystem.B.dylib\",use_errno=True)\nr=l.clonefile(sys.argv[1].encode(),sys.argv[2].encode(),0)\nif r!=0:\n  sys.stderr.write(\"clonefile errno %d\\n\"%ctypes.get_errno())\n  sys.exit(1)\n";

/** 🧊️ What a freeze cloned: the files of the frozen root (`tracked` of them in its git index), the logical size, the paths left out and the entries that vanished or were no files while the listing was taken. */
export type FrozenRootV1=Readonly<{root:string;files:number;tracked:number;megabytes:number;seconds:number;excluded:number;vanished:number;linked:number;cloned:number;ambient:number;ambientDependencies:boolean;reused:boolean}>;

const git=(root:string,args:readonly string[],input?:string):Buffer=>{
 const run=spawnSync("git",["-c","core.fsmonitor=false","-c","core.quotepath=off",...args],{cwd:root,maxBuffer:1024*MEGABYTE,...input===undefined?{}:{input}});
 if(run.status!==0)throw Error(`git ${args[0]} failed in ${root}: ${String(run.stderr).trim().slice(0,300)}`);
 return run.stdout;
};
const names=(bytes:Buffer):string[]=>bytes.toString("utf8").split("\0").filter(Boolean);
const below=(path:string,prefixes:readonly string[]):boolean=>prefixes.some(prefix=>path===prefix||path.startsWith(prefix+"/"));

/** 🧊️ Freezes the tree a build reads: every file git lists as tracked or as untracked-and-not-ignored (the set `git ls-files` hands the build), minus `Policy.frozen.exclude`, is cloned below `destination` (a copy-on-write clone where the file system has one, so no space and no time per byte; a plain copy elsewhere), then a git repository of its own is made there whose index holds the tracked files and whose `HEAD` is one commit of them. Nothing is changed in `source`: only read-only git commands run in it, and a later edit of `source` never reaches `destination`. @see https://developer.apple.com/documentation/kernel/1558433-clonefile @see https://git-scm.com/docs/git-update-index */
export function freezeSliceLabRootV1(source:string,destination:string,options:Readonly<{exclude?:readonly string[];include?:readonly string[];ambientDependencies?:boolean}>={}):FrozenRootV1{
 const started=Date.now(),exclude=options.exclude??F.exclude,cached=names(git(source,["ls-files","-z","--cached"])),others=names(git(source,["ls-files","-z","--others","--exclude-standard"]));
 const index=new Set(cached),wanted=new Set<string>([...cached,...others,...options.include??F.include]),tracked:string[]=[],made=new Set<string>();
 let files=0,excluded=0,vanished=0,linked=0,cloned=0,bytes=0;
 mkdirSync(destination,{recursive:true});
 for(const path of [...wanted].sort()){
  if(below(path,exclude)){excluded++;continue;}
  const info=lstatSync(join(source,path),{throwIfNoEntry:false});
  if(info===undefined){vanished++;continue;}
  if(!info.isFile()&&!info.isSymbolicLink()){vanished++;continue;}
  const target=join(destination,path),parent=dirname(target);
  if(!made.has(parent)){mkdirSync(parent,{recursive:true});made.add(parent);}
  if(info.isSymbolicLink()){symlinkSync(readlinkSync(join(source,path)),target);linked++;}
  else{copyFileSync(join(source,path),target,constants.COPYFILE_FICLONE|constants.COPYFILE_EXCL);cloned++;bytes+=info.size;}
  files++;
  if(index.has(path))tracked.push(path);
 }
 git(destination,["init","-q"]);
 git(destination,["update-index","--add","--info-only","-z","--stdin"],tracked.join("\0")+"\0");
 const tree=git(destination,["write-tree","--missing-ok"]).toString().trim();
 const commit=git(destination,["-c",`user.name=${F.author}`,"-c",`user.email=${F.author}@localhost`,"commit-tree",tree,"-m","frozen"]).toString().trim();
 git(destination,["update-ref","HEAD",commit]);
 return Object.freeze({root:destination,files,tracked:tracked.length,megabytes:Math.round(bytes/MEGABYTE),seconds:Math.round((Date.now()-started)/100)/10,excluded,vanished,linked,cloned,ambient:options.ambientDependencies===false?0:linkAmbientV1(source,destination),ambientDependencies:options.ambientDependencies!==false,reused:false});
}

/** 🔗️ Makes the root `node_modules` of the frozen root a directory of its own whose entries link to the entries of the live one, except the links of the tree's own workspace packages (the launcher accepts a root `node_modules` link only when the dependency receipt binds it to the root it was installed for, so a frozen root has a directory, which the workspace link step fills with its own `@semio-tech` links), recreates the ignored symbolic links of the live tree and links the paths `Policy.frozen.ambient` (the dependency generation store `.cache/b/d/m/j` the launcher finds below the root; `.cache` itself stays a real directory, the lease opener refuses a linked one) to those of the live tree (the `node_modules` of every package point into the content-addressed dependency store of the machine, the one ambient tree every build reads) as the same links in the frozen root, and returns how many it made; a link that exists stays. The store they name is addressed by the digest of its content, so a link never sees a moving target. */
export function linkAmbientV1(source:string,destination:string):number{
 let made=0;
 for(const name of F.entries){
  const from=join(source,name),to=join(destination,name);
  if(!existsSync(from)||existsSync(to))continue;
  try{
   const store=realpathSync(from);mkdirSync(to,{recursive:true});
   const own=realpathSync(source)+sep,ownLink=(path:string):boolean=>{try{return lstatSync(path).isSymbolicLink()&&realpathSync(path).startsWith(own);}catch{return false;}};
   for(const entry of readdirSync(store))if(!F.entrySkips.includes(entry)&&!ownLink(join(store,entry))){symlinkSync(join(store,entry),join(to,entry));made++;}
  }catch{}
 }
 for(const name of names(git(source,["ls-files","-z","--others","--ignored","--exclude-standard","--directory"]))){
  const path=name.replace(/\/$/u,""),from=join(source,path),to=join(destination,path);
  let info;try{info=lstatSync(from);}catch{continue;}
  if(!info.isSymbolicLink()||existsSync(to))continue;
  try{mkdirSync(dirname(to),{recursive:true});symlinkSync(readlinkSync(from),to);made++;}catch{}
 }
 for(const name of F.ambient){
  const from=join(source,name),to=join(destination,name);
  if(!existsSync(from)||existsSync(to))continue;
  try{mkdirSync(dirname(to),{recursive:true});symlinkSync(from,to);made++;}catch{}
 }
 return made;
}

/** 🧊️ The frozen root of a named directory that outlives a run: the file `<directory>.frozen.json` beside it says the freeze was completed, so a later run reuses the clone as it is (a freeze under load takes minutes); without it the directory is frozen now and the file written. */
export function frozenRootV1(source:string,directory:string,ambientDependencies=true):FrozenRootV1{
 const sidecar=`${directory}.frozen.json`;
 if(existsSync(sidecar)&&existsSync(directory)){
  const prior=JSON.parse(readFileSync(sidecar,"utf8")) as FrozenRootV1;
  if(prior.ambientDependencies!==ambientDependencies)throw Error("Frozen dependency bindings differ / Eingefrorene Abhängigkeitsbindungen unterscheiden sich");
  return Object.freeze({...prior,root:directory,reused:true});
 }
 const existing=lstatSync(directory,{throwIfNoEntry:false});
 if(existsSync(sidecar)||existing!==undefined&&(!existing.isDirectory()||readdirSync(directory).length>0))throw Error(`Incomplete frozen root at ${directory} / Unvollständige eingefrorene Sicht unter ${directory}`);
 mkdirSync(dirname(directory),{recursive:true});
 const staging=mkdtempSync(join(dirname(directory),".fr-"));
 try{
  const root=existing===undefined?join(staging,"r"):directory,receipt=join(staging,"receipt");
  const made=Object.freeze({...freezeSliceLabRootV1(source,root,{ambientDependencies}),root:directory});
  writeFileSync(receipt,JSON.stringify(made),{flag:"wx"});
  if(existing===undefined)renameSync(root,directory);
  renameSync(receipt,sidecar);
  return made;
 }finally{rmSync(staging,{recursive:true,force:true});}
}

/** 🧬️ Gives one machine a checkout of its own: the frozen root is cloned below `destination` (a copy-on-write clone of the whole directory in one call on macOS, so no space and no time per file; a copy elsewhere; the links keep their targets), the `.cache` of the clone is emptied (a machine starts without state, whatever an earlier run left in the frozen root) and the ambient links of a freeze are made again. The frozen root itself is never run in: a real `dependencies` stage rewrites the `node_modules` links of the root it runs in, and two machines that share one root share its `.nx` and its checkout paths, which the Nx tooling cannot tell apart. @see https://developer.apple.com/documentation/kernel/1558433-clonefile */
export function cloneFrozenRootV1(frozen:string,live:string,destination:string,ambientDependencies=true):Readonly<{root:string;seconds:number}>{
 const started=Date.now(),target=resolve(destination);
 if(existsSync(target))throw Error(`The machine root ${target} exists / Das Maschinenverzeichnis ${target} existiert`);
 mkdirSync(dirname(target),{recursive:true});
 const run=process.platform==="darwin"?spawnSync("/usr/bin/python3",["-I","-c",CLONE_SCRIPT,frozen,target],{encoding:"utf8"}):spawnSync("cp",["-a","--reflink=auto",frozen,target],{encoding:"utf8"});
 if(run.status!==0)throw Error(`The clone of ${frozen} failed: ${String(run.stderr).trim().slice(0,300)} / Der Klon von ${frozen} ist fehlgeschlagen`);
 rmSync(join(target,".cache"),{recursive:true,force:true});
 mkdirSync(join(target,".cache"),{recursive:true});
 if(ambientDependencies)linkAmbientV1(live,target);
 return Object.freeze({root:target,seconds:Math.round((Date.now()-started)/100)/10});
}

/** ✏️ Appends one line feed to a file of a frozen root (the one edited file of a two-machine run) and returns the function that writes the original bytes and times back; the write leaves the other clones of the file untouched. */
export function editFrozenFileV1(root:string,path:string):()=>void{
 const base=realpathSync(root),file=resolve(base,path);
 if(!file.startsWith(base+sep)||realpathSync(dirname(file))!==dirname(file))throw Error("The edited file must be inside the frozen root / Die bearbeitete Datei muss in der eingefrorenen Sicht liegen");
 const named=lstatSync(file,{bigint:true});
 if(!named.isFile()||named.nlink!==1n)throw Error("A private regular file is required / Eine private reguläre Datei ist erforderlich");
 const descriptor=openSync(file,constants.O_RDWR|constants.O_APPEND|(constants.O_NOFOLLOW??0));
 try{
  const before=fstatSync(descriptor,{bigint:true});
  if(before.dev!==named.dev||before.ino!==named.ino||before.ctimeNs!==named.ctimeNs||realpathSync(file)!==file)throw Error("The edit target changed / Das Bearbeitungsziel hat sich geändert");
  if(writeSync(descriptor,Buffer.from("\n"))!==1)throw Error("Incomplete lab edit / Unvollständige Laborbearbeitung");
  const after=fstatSync(descriptor,{bigint:true});
  let closed=false;
  return()=>{
   if(closed)return;
   closed=true;
   try{
    const current=fstatSync(descriptor,{bigint:true});
    if(current.nlink!==1n||current.ctimeNs!==after.ctimeNs||current.size!==before.size+1n)return;
    ftruncateSync(descriptor,Number(before.size));futimesSync(descriptor,before.atime,before.mtime);
   }finally{closeSync(descriptor);}
  };
 }catch(error){closeSync(descriptor);throw error;}
}
