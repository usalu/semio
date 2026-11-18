import {spawnSync} from "node:child_process";
import {constants,copyFileSync,existsSync,lstatSync,mkdirSync,readdirSync,readFileSync,readlinkSync,realpathSync,symlinkSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join,sep} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const F=schema.definitions.Policy.const.frozen,MEGABYTE=1048576;

/** 🧊️ What a freeze cloned: the files of the frozen root (`tracked` of them in its git index), the logical size, the paths left out and the entries that vanished or were no files while the listing was taken. */
export type FrozenRootV1=Readonly<{root:string;files:number;tracked:number;megabytes:number;seconds:number;excluded:number;vanished:number;linked:number;cloned:number;ambient:number;reused:boolean}>;

const git=(root:string,args:readonly string[],input?:string):Buffer=>{
 const run=spawnSync("git",["-c","core.fsmonitor=false","-c","core.quotepath=off",...args],{cwd:root,maxBuffer:1024*MEGABYTE,...input===undefined?{}:{input}});
 if(run.status!==0)throw Error(`git ${args[0]} failed in ${root}: ${String(run.stderr).trim().slice(0,300)}`);
 return run.stdout;
};
const names=(bytes:Buffer):string[]=>bytes.toString("utf8").split("\0").filter(Boolean);
const below=(path:string,prefixes:readonly string[]):boolean=>prefixes.some(prefix=>path===prefix||path.startsWith(prefix+"/"));

/** 🧊️ Freezes the tree a build reads: every file git lists as tracked or as untracked-and-not-ignored (the set `git ls-files` hands the build), minus `Policy.frozen.exclude`, is cloned below `destination` (a copy-on-write clone where the file system has one, so no space and no time per byte; a plain copy elsewhere), then a git repository of its own is made there whose index holds the tracked files and whose `HEAD` is one commit of them. Nothing is changed in `source`: only read-only git commands run in it, and a later edit of `source` never reaches `destination`. @see https://developer.apple.com/documentation/kernel/1558433-clonefile @see https://git-scm.com/docs/git-update-index */
export function freezeSliceLabRootV1(source:string,destination:string,options:Readonly<{exclude?:readonly string[];include?:readonly string[]}>={}):FrozenRootV1{
 const started=Date.now(),exclude=options.exclude??F.exclude,cached=names(git(source,["ls-files","-z","--cached"])),others=names(git(source,["ls-files","-z","--others","--exclude-standard"]));
 const index=new Set(cached),wanted=new Set<string>([...cached,...others,...options.include??F.include]),tracked:string[]=[],made=new Set<string>();
 let files=0,excluded=0,vanished=0,linked=0,cloned=0,bytes=0;
 mkdirSync(destination,{recursive:true});
 for(const path of [...wanted].sort()){
  if(below(path,exclude)){excluded++;continue;}
  let info;try{info=lstatSync(join(source,path));}catch{vanished++;continue;}
  if(!info.isFile()&&!info.isSymbolicLink()){vanished++;continue;}
  const target=join(destination,path),parent=dirname(target);
  if(!made.has(parent)){mkdirSync(parent,{recursive:true});made.add(parent);}
  try{
   if(info.isSymbolicLink()){symlinkSync(readlinkSync(join(source,path)),target);linked++;}
   else{copyFileSync(join(source,path),target,constants.COPYFILE_FICLONE);cloned++;bytes+=info.size;}
  }catch{vanished++;continue;}
  files++;
  if(index.has(path))tracked.push(path);
 }
 git(destination,["init","-q"]);
 git(destination,["update-index","--add","--info-only","-z","--stdin"],tracked.join("\0")+"\0");
 const tree=git(destination,["write-tree","--missing-ok"]).toString().trim();
 const commit=git(destination,["-c",`user.name=${F.author}`,"-c",`user.email=${F.author}@localhost`,"commit-tree",tree,"-m","frozen"]).toString().trim();
 git(destination,["update-ref","HEAD",commit]);
 return Object.freeze({root:destination,files,tracked:tracked.length,megabytes:Math.round(bytes/MEGABYTE),seconds:Math.round((Date.now()-started)/100)/10,excluded,vanished,linked,cloned,ambient:linkAmbientV1(source,destination),reused:false});
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
export function frozenRootV1(source:string,directory:string):FrozenRootV1{
 const sidecar=`${directory}.frozen.json`;
 if(existsSync(sidecar)&&existsSync(directory))return Object.freeze({...JSON.parse(readFileSync(sidecar,"utf8")) as FrozenRootV1,root:directory,reused:true});
 const made=freezeSliceLabRootV1(source,directory);
 writeFileSync(sidecar,JSON.stringify(made));
 return made;
}

/** 🔁️ Gives a reusable frozen root back its links after a run whose `dependencies` stage was real: that stage links every `node_modules` of the root into the generations of the lab's own state, which the end of the run deletes; each such link (found among the ignored paths of the root, and the root `node_modules` itself) is made again as the live tree has it, and the root `node_modules` is rebuilt as the directory of entry links of a freeze. Returns how many links were made again. */
export function relinkFrozenRootV1(source:string,root:string,work:string):number{
 const below=work+sep,pointsIntoWork=(path:string):boolean=>{try{return lstatSync(path).isSymbolicLink()&&readlinkSync(path).startsWith(below);}catch{return false;}};
 let made=0;
 for(const name of names(git(root,["ls-files","-z","--others","--ignored","--exclude-standard","--directory"]))){
  const path=name.replace(/\/$/u,""),at=join(root,path);
  if(path==="node_modules"||!pointsIntoWork(at))continue;
  const from=join(source,path);
  unlinkSync(at);made++;
  try{if(lstatSync(from).isSymbolicLink())symlinkSync(readlinkSync(from),at);}catch{}
 }
 if(pointsIntoWork(join(root,"node_modules"))){unlinkSync(join(root,"node_modules"));made++;}
 linkAmbientV1(source,root);
 return made;
}

/** ✏️ Appends one line feed to a file of a frozen root (the one edited file of a two-machine run) and returns the function that writes the original bytes and times back; the write leaves the other clones of the file untouched. */
export function editFrozenFileV1(root:string,path:string):()=>void{
 const file=join(root,path),before=readFileSync(file),info=lstatSync(file);
 writeFileSync(file,Buffer.concat([before,Buffer.from("\n")]));
 return()=>{writeFileSync(file,before);utimesSync(file,info.atime,info.mtime);};
}
