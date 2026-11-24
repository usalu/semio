import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {constants,type BigIntStats} from "node:fs";
import {copyFile,lstat,mkdir,open,readdir,readFile,readlink,rename,rm,stat,symlink,unlink,writeFile} from "node:fs/promises";
import {devNull} from "node:os";
import {delimiter,dirname,isAbsolute,join,relative,resolve} from "node:path";
import {type FleetManifestEntryV1} from "../📒️journal/🟦️.ts";
import {absoluteUnitsV1,caseDependentIgnoresV1,foldNameV1,linkRefusalV1,pathBudgetV1,refusePathV1} from "../🚧️boundary/🟦️.ts";

/** 🌿 The git state of a worktree as the replica shares it: whether a root `.git` and a usable `git` exist, the symbolic ref of `HEAD` (`null` when detached), the commit `HEAD` names (`null` when unborn) and the tree of the current stage (`null` when it cannot be written, e.g. with unmerged entries). */
export type GitStateV1=Readonly<{present:boolean;head:string|null;commit:string|null;indexTree:string|null;observation?:GitStageObservationV1}>;

/** 🔎️ The reason an observed Git stage cannot establish an exact checkpoint. */
export type GitStageObservationV1="index-changed"|"index-missing"|"locked-snapshot-failed"|"write-tree-failed"|"ref-changed"|"observation-failed";

/** 🪟 The working-tree view as a git GUI shows it: the counts of entries with a staged change, a modified, a deleted and an untracked file, the `lines` (`XY<space>path`, sorted bytewise, everything that is not purely staged), the sha256 `digest` over the lines joined by `\n` and the `stage` (`X<space>path` of every entry with a staged change, the input of an incremental update). */
export type GitViewV1=Readonly<{staged:number;modified:number;deleted:number;untracked:number;digest:string;lines:readonly string[];stage:readonly string[]}>;

/** 🧾 One difference between two trees: `mode` is `000000` for a path that is gone. */
export type GitChangeV1=Readonly<{path:string;mode:string;oid:string}>;

/** 🔌 What the index register needs of a replica: the origin id of the archivist whose stage is the stage of the fleet (`archivist`, the group server), the manifest winners of the realm (`each`), the registers this machine has published or applied (`held`, the proof of which state its stage derives from), the queued realm items and the writes (emit an event, mark a register applied); the replica hands in closures, so this module never sees its tables. */
export type IndexHostV1=Readonly<{
 worktree:string;
 server:boolean;
 archivist:string;
 now():number;
 base():FleetManifestEntryV1|undefined;
 each(visit:(path:string,entry:FleetManifestEntryV1)=>void):void;
 held(visit:(path:string,entry:FleetManifestEntryV1)=>void):void;
 pending():ReadonlyMap<string,FleetManifestEntryV1>;
 prepareStageInputs?(oids:readonly string[],paths:readonly string[],signal?:AbortSignal):Promise<boolean>;
 emit(events:readonly Readonly<{operation:"link"|"delete";path:string;target?:string}>[]):Promise<readonly string[]|undefined>;
 settle(paths:readonly string[]):void;
 enqueue(paths:readonly string[]):void;
 say(event:Readonly<{event:"applied"|"published"|"skipped"|"blocked"|"error"|"overwritten"|"repaired";stage:"git";path?:string;reason?:string;count?:number;lost?:string;seconds?:number}>):void;
}>;

/** 🚦 Why stage sharing waits or is degraded on a machine: no base yet, an object has not arrived, the stage has unmerged entries, `git write-tree` or another git step fails, `index.lock` is held (and for how long), or the old stage could not be kept before it was replaced. */
export type StageReasonV1="waiting-for-base"|"objects-missing"|"unmerged"|"write-tree-failed"|"git-failed"|"index-busy"|"index-lock-stale"|"backup-failed";

/** 🩻 Whether stage sharing works on this machine (`shared`), waits for something that will arrive (`waiting`) or needs attention (`degraded`), with the reason; read from `.cache/f/git-index.json`, which the running replica keeps. */
export type StageHealthV1=Readonly<{state:"shared"|"waiting"|"degraded";reason:StageReasonV1|null}>;

/** 📒 What a publish of the stage did. When `waiting` is true the stage may be retried without another index edit. */
export type IndexPublishV1=Readonly<{published:number;base:boolean;waiting:boolean}>;

/** 🧩 What an apply of remote stage registers did. */
export type IndexApplyV1=Readonly<{applied:number;queued:number}>;

/** 🔎 The comparison of the stage with the fold of the registers. */
export type IndexVerifyV1=Readonly<{ok:boolean;skipped:boolean;expected:string|null;actual:string|null}>;

/** 🏷️ The register that carries the base tree (`link` target `tree <oid>`). */
export const INDEX_BASE_V1=".cache/f/index-base";

/** 🗂️ The prefix of the entry registers: `<prefix><worktree path>`, `link` target `<mode> <oid>` or `delete`. */
export const INDEX_PREFIX_V1=".cache/f/index/";

/** 🧭 Tells whether a replica path belongs to the virtual index realm. */
export const indexRealmV1=(path:string):boolean=>path===INDEX_BASE_V1||path.startsWith(INDEX_PREFIX_V1);

const GIT_PROBE_MILLISECONDS=10_000,GIT_RUN_MILLISECONDS=600_000,SHA1_EMPTY="4b825dc642cb6eb9a060e54bf8d69288fbee4904",SHA256_EMPTY="6ef19b41225c5369f1c104d45d8d85efa9b057b53b14b4b9b939dd74decc5321",NEGATIVE_MILLISECONDS=2000,RETRY_MILLISECONDS=1000,MAX_RETRY_MILLISECONDS=30_000,STALE_LOCK_MILLISECONDS=600_000;
/** ⏱️ Delays the next Git stage retry by one second after the first failed attempt, doubling up to thirty seconds. */
export const gitStageRetryDelayV1=(attempts:number):number=>Math.min(MAX_RETRY_MILLISECONDS,RETRY_MILLISECONDS*2**Math.max(0,attempts-1));

const windows=process.platform==="win32";
const token=():string=>randomBytes(6).toString("hex");
const zeros=(length:number):string=>"0".repeat(length);

type Outcome=Readonly<{code:number;out:Buffer;err:string}>;
let executable:string|null|undefined,executableAt=0;

async function works(file:string):Promise<boolean>{
 return new Promise<boolean>(done=>{
  try{
   const child=spawn(file,["--version"],{stdio:["ignore","pipe","ignore"],windowsHide:true,timeout:GIT_PROBE_MILLISECONDS});
   let seen="";
   child.stdout.on("data",chunk=>{seen+=String(chunk);});
   child.on("error",()=>{done(false);});
   child.on("close",code=>{done(code===0&&seen.startsWith("git version"));});
  }catch{done(false);}
 });
}

/** 🔍 The ordered places a git executable is looked up: `SEMIO_GIT`, every `PATH` entry, then the usual install locations of the operating system (Homebrew, Command Line Tools, distribution paths, Git for Windows below `ProgramFiles`, `ProgramFiles(x86)` and `LOCALAPPDATA`). */
export function gitCandidatesV1(environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform):readonly string[]{
 const names=platform==="win32"?["git.exe","git.cmd"]:["git"],found:string[]=[];
 if(environment.SEMIO_GIT)found.push(environment.SEMIO_GIT);
 const pathKey=Object.keys(environment).find(key=>key.toUpperCase()==="PATH"),separator=platform==="win32"?";":delimiter;
 for(const directory of (pathKey===undefined?"":environment[pathKey]??"").split(separator))if(directory!=="")for(const name of names)found.push(platform==="win32"?`${directory.replace(/[\\/]+$/u,"")}\\${name}`:join(directory,name));
 if(platform==="win32"){
  const roots=[environment.ProgramFiles,environment["ProgramFiles(x86)"],environment.ProgramW6432,environment.LOCALAPPDATA===undefined?undefined:`${environment.LOCALAPPDATA}\\Programs`,"C:\\Program Files","C:\\Program Files (x86)"];
  for(const root of roots)if(root)for(const sub of ["Git\\cmd\\git.exe","Git\\bin\\git.exe","Git\\mingw64\\bin\\git.exe"])found.push(`${root}\\${sub}`);
 }else if(platform==="darwin")found.push("/usr/bin/git","/opt/homebrew/bin/git","/usr/local/bin/git","/Library/Developer/CommandLineTools/usr/bin/git","/Applications/Xcode.app/Contents/Developer/usr/bin/git");
 else found.push("/usr/bin/git","/usr/local/bin/git","/bin/git","/usr/local/git/bin/git","/snap/bin/git");
 return [...new Set(found)];
}

/** 🔦 Resolves the git executable once (a miss is retried after two seconds): the first candidate of `gitCandidatesV1` that exists and answers `--version`; `null` when none does. */
export async function resolveGitV1(force:boolean=false):Promise<string|null>{
 if(!force&&executable!==undefined&&(executable!==null||Date.now()-executableAt<NEGATIVE_MILLISECONDS))return executable;
 executableAt=Date.now();executable=null;
 for(const candidate of gitCandidatesV1()){
  if(isAbsolute(candidate)&&!await stat(candidate).then(info=>info.isFile(),()=>false))continue;
  if(await works(candidate)){executable=candidate;break;}
 }
 return executable;
}

type Run=Readonly<{index?:string;stdin?:string|Buffer;signal?:AbortSignal;hash?:number;env?:Readonly<Record<string,string>>;view?:boolean}>;

/** 📌️ The `-c` pins of every git process the replica starts, so that no machine-local configuration changes a result: no excludes file and no attributes file, names printed as they are (`core.quotepath=false`) and composed (`core.precomposeunicode=true`), no line-ending conversion (`core.autocrlf=false`, `core.safecrlf=false`, `core.eol=lf`), long paths on Windows (`core.longpaths=true`, ignored elsewhere), the NTFS and HFS name protection on every system, no file-system monitor and no automatic maintenance. The pins are the same on every operating system, so `gitPinsHashV1` is one value for the fleet. */
export const GIT_PINS_V1:readonly (readonly [string,string])[]=[["core.excludesFile",""],["core.attributesFile",""],["core.quotepath","false"],["core.precomposeunicode","true"],["core.autocrlf","false"],["core.safecrlf","false"],["core.eol","lf"],["core.longpaths","true"],["core.protectNTFS","true"],["core.protectHFS","true"],["core.fsmonitor","false"],["gc.auto","0"]];

/** 🪟️ The additional pins of the attested working-tree view: `core.ignorecase=false` (the `.gitignore` match is case-sensitive on every operating system, like the replica's own matcher and like Linux), `core.filemode=false` (the view ignores the executable bit, which Windows cannot represent; `GitGuardsV1.modeMismatch` counts the disagreements between file and index instead) and `core.symlinks=true` (a tracked link that this volume cannot hold shows as deleted). */
export const GIT_VIEW_PINS_V1:readonly (readonly [string,string])[]=[["core.ignorecase","false"],["core.filemode","false"],["core.symlinks","true"]];

/** 🔖️ The first twelve hex digits of the sha256 over the pins, the environment pins (`GIT_CONFIG_NOSYSTEM`, `GIT_CONFIG_GLOBAL` and `XDG_CONFIG_HOME` pointing at empty places, `GIT_ATTR_NOSYSTEM`, no inherited `GIT_CONFIG_*`) and the oldest git that honours them (2.32): attested so that a fleet in which one machine pins differently is `diverged`. */
export const gitPinsHashV1=():string=>createHash("sha256").update(JSON.stringify({pins:GIT_PINS_V1,view:GIT_VIEW_PINS_V1,environment:["GIT_CONFIG_NOSYSTEM","GIT_CONFIG_GLOBAL","XDG_CONFIG_HOME","GIT_ATTR_NOSYSTEM","-GIT_CONFIG_PARAMETERS","-GIT_CONFIG_COUNT"],minimum:"2.32"})).digest("hex").slice(0,12);

const pinned=new Map<string,string>();
async function pinDirectory(worktree:string):Promise<string|undefined>{
 const known=pinned.get(worktree);
 if(known!==undefined)return known===""?undefined:known;
 const directory=join(worktree,".git","semio-pin");
 try{await mkdir(join(directory,"xdg"),{recursive:true});await writeFile(join(directory,"config"),"");pinned.set(worktree,directory);return directory;}
 catch{pinned.set(worktree,"");return undefined;}
}


async function git(worktree:string,args:readonly string[],options:Run={}):Promise<Outcome|null>{
 const file=await resolveGitV1();
 if(file===null||options.signal?.aborted)return null;
 const pin=await pinDirectory(worktree);
 const environment:Record<string,string|undefined>={...process.env,GIT_DIR:join(worktree,".git"),GIT_TERMINAL_PROMPT:"0",LC_ALL:"C",GIT_CONFIG_NOSYSTEM:"1",GIT_ATTR_NOSYSTEM:"1",GIT_CONFIG_GLOBAL:pin===undefined?devNull:join(pin,"config"),XDG_CONFIG_HOME:pin===undefined?devNull:join(pin,"xdg")};
 for(const name of Object.keys(environment))if(name.startsWith("GIT_CONFIG_KEY_")||name.startsWith("GIT_CONFIG_VALUE_"))delete environment[name];
 for(const name of ["GIT_WORK_TREE","GIT_OBJECT_DIRECTORY","GIT_ALTERNATE_OBJECT_DIRECTORIES","GIT_INDEX_FILE","GIT_CONFIG_PARAMETERS","GIT_CONFIG_COUNT","GIT_CONFIG","GIT_EXTERNAL_DIFF","GIT_ATTR_SOURCE"])delete environment[name];
 Object.assign(environment,options.env);
 if(options.index!==undefined)environment.GIT_INDEX_FILE=options.index;
 return new Promise<Outcome|null>(done=>{
  try{
   const child=spawn(file,[...GIT_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]),...(options.view===true?GIT_VIEW_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]):[]),...(options.index===undefined?[]:["-c","core.splitIndex=false","-c","index.sparse=false"]),...args],{cwd:worktree,env:environment as NodeJS.ProcessEnv,stdio:["pipe","pipe","pipe"],windowsHide:true,signal:options.signal,timeout:GIT_RUN_MILLISECONDS}),out:Buffer[]=[];
   let err="";
   child.stdout.on("data",(chunk:Buffer)=>{out.push(chunk);});
   child.stderr.on("data",chunk=>{err+=String(chunk);});
   child.stdin.on("error",()=>{});
   child.on("error",()=>{done(null);});
   child.on("close",code=>{done({code:code??1,out:Buffer.concat(out),err});});
   child.stdin.end(options.stdin);
  }catch{done(null);}
 });
}

const text=(outcome:Outcome|null):string|null=>outcome!==null&&outcome.code===0?outcome.out.toString("utf8").trim():null;
const oidPattern=/^[0-9a-f]{40}(?:[0-9a-f]{24})?$/u;

type GitPresenceV1="absent"|"unavailable"|"present";
const verdicts=new Map<string,Readonly<{until:number;present:boolean;stamp?:string}>>();
const gitFileStampV1=(info:BigIntStats):string=>`${info.dev}:${info.ino}:${info.mode}:${info.size}:${info.mtimeNs}:${info.ctimeNs}`;
const missingGitEntryV1=(error:NodeJS.ErrnoException):null=>{if(error.code==="ENOENT")return null;throw error;};

/** 🏥️ Distinguishes a missing root from an existing Git root that cannot certify a stage. */
async function probeGitPresenceV1(worktree:string,signal:AbortSignal|undefined,observe:boolean):Promise<GitPresenceV1>{
 const directory=join(worktree,".git"),root=await lstat(directory,{bigint:true}).catch(missingGitEntryV1);
 if(root===null)return "absent";
 if(!root.isDirectory())return "unavailable";
 const head=await lstat(join(directory,"HEAD"),{bigint:true}).catch(missingGitEntryV1);
 if(head===null)return "unavailable";
 const selected=observe&&!abortedV1(signal)?await resolveGitV1():null;
 if(observe&&selected===null)return "unavailable";
 const stamp=selected===null?undefined:`${root.dev}:${root.ino}:${root.mode}|${gitFileStampV1(head)}|${await gitPresenceFilesV1(directory,selected)}`;
 const known=verdicts.get(worktree),now=Date.now();
 if(known!==undefined&&known.until>now&&(!observe||known.stamp===stamp))return known.present?"present":"unavailable";
 if(observe&&known?.stamp!==stamp){stamps.delete(worktree);trees.delete(worktree);}
 const present=text(await git(worktree,["rev-parse","--git-dir"],{signal}))!==null;
 if(!abortedV1(signal))verdicts.set(worktree,{present,until:present?now+60_000:now+NEGATIVE_MILLISECONDS,stamp:stamp??known?.stamp});
 return present?"present":"unavailable";
}

/** 📎️ Stamps the local config and selected absolute Git executable before a warm stage observation. */
async function gitPresenceFilesV1(directory:string,selected:string):Promise<string>{
 const config=await stat(join(directory,"config"),{bigint:true}).catch(missingGitEntryV1);
 const file=isAbsolute(selected)?await stat(selected,{bigint:true}).catch(missingGitEntryV1):null;
 return (config===null?"-":gitFileStampV1(config))+"|"+(file===null?"-":gitFileStampV1(file));
}

/** 🏠 Tells whether a usable Git accepts the root, with the existing short-lived negative and warm positive cache. */
export async function gitPresentV1(worktree:string,signal?:AbortSignal):Promise<boolean>{
 return (await probeGitPresenceV1(worktree,signal,false).catch(()=>"unavailable"))==="present";
}

/** 🌲 Writes the tree of the stage with `git write-tree`; `null` when git, the repository or an object is missing, the stage has unmerged entries or the index lock is held. `index` selects another index file. */
export async function writeTreeV1(worktree:string,options:Run={}):Promise<string|null>{
 const oid=text(await git(worktree,["write-tree"],options));
 return oid!==null&&oidPattern.test(oid)?oid:null;
}

/** 🆔 The empty tree id of the repository's object format. */
export async function emptyTreeV1(worktree:string,signal?:AbortSignal):Promise<string>{
 return text(await git(worktree,["rev-parse","--show-object-format"],{signal}))==="sha256"?SHA256_EMPTY:SHA1_EMPTY;
}

/** 🔬 Lists the differences between two trees with `git diff-tree -r -z --no-renames` (`from` null means the empty tree); `null` when git fails, e.g. for a missing tree object. */
export async function diffTreeV1(worktree:string,from:string|null,to:string,signal?:AbortSignal):Promise<readonly GitChangeV1[]|null>{
 const outcome=await git(worktree,["diff-tree","-r","-z","--no-renames","--no-abbrev",from??await emptyTreeV1(worktree,signal),to],{signal});
 if(outcome===null||outcome.code!==0)return null;
 const parts=outcome.out.toString("utf8").split("\0"),changes:GitChangeV1[]=[];
 for(let index=0;index+1<parts.length;index+=2){
  const header=parts[index]!.split(" "),path=parts[index+1]!;
  if(header.length<5||!header[0]!.startsWith(":"))return null;
  changes.push(header[1]==="000000"?{path,mode:"000000",oid:zeros(header[3]!.length)}:{path,mode:header[1]!,oid:header[3]!});
 }
 return changes;
}

/** ✍️ Applies changes to the stage with one `git update-index -z --replace --index-info` batch (removals first, so a file can become a directory); `false` when git fails or the index lock is held. */
export async function updateIndexV1(worktree:string,changes:readonly GitChangeV1[],options:Run={}):Promise<boolean>{
 if(changes.length===0)return true;
 const ordered=[...changes.filter(change=>change.mode==="000000"),...changes.filter(change=>change.mode!=="000000")];
 const body=ordered.map(change=>change.mode==="000000"?`0 ${zeros(options.hash??40)}\t${change.path}\0`:`${change.mode} ${change.oid}\t${change.path}\0`).join("");
 const outcome=await git(worktree,["update-index","-z","--replace","--index-info"],{...options,stdin:body});
 return outcome!==null&&outcome.code===0;
}

/** 📥 Replaces the stage by a tree with `git read-tree` (index only, the worktree is not touched). */
export async function readTreeV1(worktree:string,oid:string,options:Run={}):Promise<boolean>{
 const outcome=await git(worktree,["read-tree",oid],options);
 return outcome!==null&&outcome.code===0;
}

/** 🕳️ Names the object ids the repository does not hold (`git cat-file --batch-check=%(objectname)`, which does not open the objects: 30 000 loose ids in about 0.2 s instead of about 4 s); `null` when git fails. */
export async function missingObjectsV1(worktree:string,oids:readonly string[],signal?:AbortSignal):Promise<ReadonlySet<string>|null>{
 if(oids.length===0)return new Set();
 const outcome=await git(worktree,["cat-file","--batch-check=%(objectname)"],{stdin:`${oids.join("\n")}\n`,signal});
 if(outcome===null||outcome.code!==0)return null;
 const missing=new Set<string>();
 for(const line of outcome.out.toString("utf8").split("\n"))if(line.endsWith(" missing"))missing.add(line.slice(0,line.length-8));
 return missing;
}

const signatures=new Map<string,string>();
async function signatureOf(worktree:string):Promise<string>{
 const info=await stat(join(worktree,".git","index"),{bigint:true}).catch(()=>undefined);
 return info===undefined?"":`${info.ino}:${info.size}:${info.mtimeNs}`;
}

/** 🧾 The stat signature of `.git/index` (inode, size, modification time in nanoseconds; empty without an index): equal signatures mean the stage did not move, and the call costs one `stat`. */
export const indexSignatureV1=signatureOf;

/** 🔖 Whether the index still has the signature certified by publication or comparison with the register tree. */
export async function indexCertifiedV1(worktree:string):Promise<boolean>{
 const signature=signatures.get(worktree);
 return signature!==undefined&&signature!==""&&await signatureOf(worktree)===signature;
}

const stamps=new Map<string,Readonly<{stamp:string;state:GitStateV1;at:number}>>(),stampRefreshMilliseconds=300_000;
const statStamp=async(file:string):Promise<string>=>{const info=await stat(file,{bigint:true}).catch(()=>undefined);return info===undefined?"-":`${info.ino}:${info.size}:${info.mtimeNs}`;};

async function refStamp(worktree:string):Promise<{refs:string;index:string}>{
 const root=join(worktree,".git"),head=await readFile(join(root,"HEAD"),"utf8").catch(()=>""),ref=/^ref: (.+)\n?$/u.exec(head)?.[1];
 return {refs:[head,ref===undefined?"":await statStamp(join(root,...ref.split("/"))),await statStamp(join(root,"packed-refs"))].join("|"),index:await signatureOf(worktree)};
}

const stampKey=(stamp:Awaited<ReturnType<typeof refStamp>>):string=>stamp.refs+"|"+stamp.index;

type IndexSnapshotOptionsV1=Readonly<{
 signal?:AbortSignal;
 requireLock?:boolean;
 allowAbsent?:boolean;
 observeTree?:typeof writeTreeV1;
 note?:(reason:GitStageObservationV1)=>void;
}>;

/** 🔏 Certifies a stable on-disk index through a private copy without taking its writer lock. */
async function readIndexTreeSnapshotV1(worktree:string,signature:string,{signal,requireLock=true,allowAbsent=false,observeTree=writeTreeV1,note}:IndexSnapshotOptionsV1={}):Promise<string|null>{
 const source=join(worktree,".git","index"),hasLock=()=>lstat(source+".lock").then(info=>info.isFile(),()=>false);
 const failed=async()=>{if(note)note(await hasLock()?"locked-snapshot-failed":"write-tree-failed");return null;};
 if(abortedV1(signal))return null;
 if(signature===""&&(!allowAbsent||!await lstat(source).then(()=>false,(error:NodeJS.ErrnoException)=>error.code==="ENOENT"))){note?.("index-missing");return null;}
 if(requireLock&&!await hasLock()){note?.("write-tree-failed");return null;}
 if(await signatureOf(worktree)!==signature){note?.("index-changed");return null;}
 const cached=trees.get(worktree);if(signature!==""&&cached?.signature===signature)return cached.tree;
 const directory=join(worktree,".cache","f","tmp"),index=join(directory,`index-observe-${token()}`);
 try{
  await mkdir(directory,{recursive:true});
  if(signature!=="")await copyFile(source,index,constants.COPYFILE_FICLONE);
  if(await signatureOf(worktree)!==signature){note?.("index-changed");return null;}
  const tree=await observeTree(worktree,{index,signal});
  if(abortedV1(signal))return null;
  if(await signatureOf(worktree)!==signature){note?.("index-changed");return null;}
  if(tree===null)return await failed();
  if(signature!=="")trees.set(worktree,{signature,tree});
  return tree;
 }catch{return await failed();}
 finally{
  await rm(index,{force:true}).catch(()=>{});
  await rm(index+".lock",{force:true}).catch(()=>{});
 }
}

/** 🩺 Reads a Git stage tied to the same HEAD and published index, using a private index for native tree construction. */
export async function gitStateV1(worktree:string,signal?:AbortSignal,observeTree:typeof writeTreeV1=writeTreeV1):Promise<GitStateV1>{
 const absent:GitStateV1={present:false,head:null,commit:null,indexTree:null};
 try{
  const presence=await probeGitPresenceV1(worktree,signal,true);
  if(presence!=="present"||abortedV1(signal)){
   if(!abortedV1(signal)){stamps.delete(worktree);trees.delete(worktree);}
   return presence==="absent"?absent:{...absent,observation:"observation-failed"};
  }
  const before=await refStamp(worktree),known=stamps.get(worktree);
  if(known!==undefined&&known.stamp===stampKey(before)&&Date.now()-known.at<stampRefreshMilliseconds)return known.state;
  let reason:GitStageObservationV1="observation-failed";
  const signature=before.index,cached=trees.get(worktree),snapshot=(allowAbsent=false)=>readIndexTreeSnapshotV1(worktree,signature,
   {signal,requireLock:false,allowAbsent,observeTree,note:why=>{reason=why;}});
  const [headResult,commitResult,observed]=await Promise.all([
   git(worktree,["symbolic-ref","-q","HEAD"],{signal}),
   git(worktree,["rev-parse","-q","--verify","HEAD^{commit}"],{signal}),
   signature===""?null:cached?.signature===signature?cached.tree:snapshot(),
  ]),head=text(headResult),commit=text(commitResult);
  const state={present:true,head:head===""?null:head,commit:commit!==null&&oidPattern.test(commit)?commit:null};
  const unborn=signature===""&&head!==null&&head!==""&&commitResult?.code===1&&
   (await git(worktree,["rev-parse","-q","--verify","HEAD"],{signal}))?.code===1;
  if(signature===""&&!unborn)reason="index-missing";
  const indexTree=unborn?await snapshot(true):observed;
  if(indexTree===null)return {...state,indexTree:null,observation:reason};
  const after=await refStamp(worktree);
  if(after.index!==signature)return {...state,indexTree:null,observation:"index-changed"};
  if(after.refs!==before.refs)return {...state,head:null,commit:null,indexTree:null,observation:"ref-changed"};
  if(abortedV1(signal))return {...state,indexTree:null,observation:"observation-failed"};
  const exact={...state,indexTree};
  stamps.set(worktree,{stamp:stampKey(after),state:exact,at:Date.now()});
  return exact;
 }catch{return {...absent,observation:"observation-failed"};}
}
/** 🪟 Reads the records of `git status --porcelain=v1 -z --untracked-files=all` (an entry is `XY path`, a rename or copy is followed by a second field with the old path, which is skipped). `staged` counts entries whose index column is a change, `deleted` those deleted in the worktree, `modified` the other worktree changes and `untracked` the `??` entries. `lines` holds every entry that is not purely staged, with NFC paths, sorted by the UTF-8 bytes. */
export function parseGitViewV1(porcelain:string,extra:Readonly<{deleted?:readonly string[];untracked?:readonly string[]}>={}):GitViewV1{
 const parts=porcelain.split("\0"),lines:string[]=[],stage:string[]=[];
 for(let index=0;index<parts.length;index++){
  const record=parts[index]!;
  if(record.length<4||record[2]!==" ")continue;
  const x=record[0]!,y=record[1]!,path=record.slice(3).normalize("NFC");
  if(x==="R"||x==="C"||y==="R"||y==="C")index++;
  if(x==="!"&&y==="!")continue;
  if(x==="?"&&y==="?"){lines.push(`?? ${path}`);continue;}
  if(x!==" ")stage.push(`${x} ${path}`);
  if(y===" ")continue;
  lines.push(`${x}${y} ${path}`);
 }
 const known=new Set(lines);
 for(const path of extra.deleted??[]){const line=` D ${path.normalize("NFC")}`;if(!known.has(line)){known.add(line);lines.push(line);}}
 for(const path of extra.untracked??[]){const line=`?? ${path.normalize("NFC")}`;if(!known.has(line)){known.add(line);lines.push(line);}}
 return assembleViewV1(lines,stage);
}

/** 🧮️ Builds a view from its lines (any order) and its stage entries: sorts the lines bytewise and derives the counts and the digest. An entry of any state whose path the path policy refuses (too long, illegal characters, reserved names) is left out, because the replica never carries such a file: the machine that holds it untracked, or modified, and every machine that adopted its stage and so reads the tracked path as deleted would otherwise differ from the machines that do not. The staged column is kept. */
export function assembleViewV1(lines:readonly string[],stage:readonly string[]):GitViewV1{
 const sorted=lines.filter(line=>refusePathV1(line.slice(3))===undefined).sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));
 let modified=0,deleted=0,untracked=0;
 for(const line of sorted){if(line.startsWith("??"))untracked++;else if(line[1]==="D")deleted++;else modified++;}
 return {staged:stage.length,modified,deleted,untracked,digest:createHash("sha256").update(sorted.join("\n")).digest("hex"),lines:sorted,stage};
}

/** 🔗️ Narrows a view to what the replica can carry: an untracked symbolic link whose target the link policy refuses (absolute, drive-qualified or leaving the worktree) is never carried, so it is not part of the view either; `unsafe-link` stays counted by the guards. */
async function carriedViewV1(worktree:string,view:GitViewV1):Promise<GitViewV1>{
 const refused=new Set<string>();
 for(const line of view.lines){
  if(!line.startsWith("??"))continue;
  const path=line.slice(3);
  try{
   const file=join(worktree,path);
   if((await lstat(file)).isSymbolicLink()&&linkRefusalV1(path,await readlink(file))!==undefined)refused.add(line);
  }catch{}
 }
 return refused.size===0?view:assembleViewV1(view.lines.filter(line=>!refused.has(line)),view.stage);
}

/** 🛑️ Whether the signal has fired by now, read afresh so an abort that lands during an await is seen. */
const abortedV1=(signal?:AbortSignal):boolean=>signal?.aborted===true;

/** 🪟 Reads the working-tree view with `git --no-optional-locks status --porcelain=v1 -z --untracked-files=all --no-renames` under the pins `GIT_PINS_V1` and `GIT_VIEW_PINS_V1`, which never takes `index.lock`; `null` without `git` or a root `.git`, when git fails or when `signal` aborts. Never throws. On a volume that treats names case-insensitively or normalisation-insensitively (APFS, NTFS) git cannot see that the index spells a name differently than the disk does (`Foo.txt` against `foo.txt`, NFC against NFD): such a tracked path is added as ` D <index path>`, and an NFD-only spelling as `?? <path>`, exactly as git shows them on Linux, so the lines are the same on every operating system for the same disk state. `.git/info/exclude` stays a per-machine input that no flag removes: `GitGuardsV1.infoExclude` counts its rules and the fleet verdict fails on a difference. */
export async function gitViewV1(worktree:string,signal?:AbortSignal):Promise<GitViewV1|null>{
 try{
  if(!await gitPresentV1(worktree,signal))return null;
  const outcome=await git(worktree,["--no-optional-locks","status","--porcelain=v1","-z","--untracked-files=all","--no-renames"],{signal,view:true,env:{GIT_OPTIONAL_LOCKS:"0"}});
  if(outcome===null||outcome.code!==0||signal?.aborted===true)return null;
  const volume=await volumeV1(worktree);
  if(!volume.caseInsensitive&&!volume.normalisationInsensitive)return await carriedViewV1(worktree,parseGitViewV1(outcome.out.toString("utf8")));
  const tracked=await trackedEntriesV1(worktree,signal);
  if(tracked===null||abortedV1(signal))return null;
  const spelling=await spellingV1(worktree,tracked.map(entry=>entry.path),signal);
  return await carriedViewV1(worktree,parseGitViewV1(outcome.out.toString("utf8"),{deleted:spelling.absent,untracked:spelling.composed}));
 }catch{return null;}
}

/** 🔬️ What changed since a view was read: the worktree `paths` that were published or applied, and, when the stage or `HEAD` moved (`restage`), the stage tree before and after. */
export type GitViewChangeV1=Readonly<{paths:readonly string[];restage:Readonly<{before:string|null;after:string|null}>|null}>;

const pathspecBytes=16_000,pathspecCount=800;
async function inChunks(worktree:string,head:readonly string[],paths:readonly string[],signal:AbortSignal|undefined):Promise<readonly Buffer[]|null>{
 const outputs:Buffer[]=[];
 for(let start=0;start<paths.length;){
  let end=start,bytes=0;
  while(end<paths.length&&end-start<pathspecCount&&(end===start||bytes+Buffer.byteLength(paths[end]!)<pathspecBytes)){bytes+=Buffer.byteLength(paths[end]!)+1;end++;}
  const outcome=await git(worktree,[...head,"--",...paths.slice(start,end)],{signal,view:true,env:{GIT_OPTIONAL_LOCKS:"0",GIT_LITERAL_PATHSPECS:"1"}});
  if(outcome===null||outcome.code!==0||signal?.aborted===true)return null;
  outputs.push(outcome.out);start=end;
 }
 return outputs;
}

/** 🔬️ Updates a view incrementally: only the `paths` of `change` (and, after a stage or `HEAD` move, the paths whose entry differs between the two stage trees and those whose staged status differs from `previous.stage`, found by `git diff --cached --name-status`) are classified again by `git status` under the same pins with literal pathspecs; every line at or below those paths is replaced, the counts and the digest are derived again. The result equals `gitViewV1` of the same disk state. `null` (the caller reads the whole view) when git fails or aborts, when a `.gitignore` is among the paths (it changes the classification of other paths) or when more than 20 000 paths are named. */
export async function gitViewDeltaV1(worktree:string,previous:GitViewV1,change:GitViewChangeV1,signal?:AbortSignal):Promise<GitViewV1|null>{
 try{
  const affected=new Set<string>();
  for(const path of change.paths){
   const name=path.normalize("NFC");
   if(name===".git"||name.startsWith(".git/"))continue;
   if(name===".gitignore"||name.endsWith("/.gitignore"))return null;
   affected.add(name);
  }
  if(affected.size>20_000)return null;
  let stage=previous.stage;
  if(change.restage!==null){
   const {before:from,after:to}=change.restage;
   if(from===null||to===null)return null;
   if(from!==to){const moved=await diffTreeV1(worktree,from,to,signal);if(moved===null)return null;for(const entry of moved)affected.add(entry.path.normalize("NFC"));}
   const outcome=await git(worktree,["--no-optional-locks","diff","--cached","--name-status","--no-renames","-z"],{signal,view:true,env:{GIT_OPTIONAL_LOCKS:"0"}});
   if(outcome===null||outcome.code!==0||signal?.aborted===true)return null;
   const fields=outcome.out.toString("utf8").split("\0"),next:string[]=[];
   for(let index=0;index+1<fields.length;index+=2)next.push(`${fields[index]![0]} ${fields[index+1]!.normalize("NFC")}`);
   const before=new Set(previous.stage),after=new Set(next);
   for(const entry of before)if(!after.has(entry))affected.add(entry.slice(2));
   for(const entry of after)if(!before.has(entry))affected.add(entry.slice(2));
   stage=next;
  }
  if(affected.size===0)return previous;
  const paths=[...affected].sort(),outputs=await inChunks(worktree,["--no-optional-locks","status","--porcelain=v1","-z","--untracked-files=all","--no-renames"],paths,signal);
  if(outputs===null)return null;
  const volume=await volumeV1(worktree),extra:{deleted?:readonly string[];untracked?:readonly string[]}={};
  if(volume.caseInsensitive||volume.normalisationInsensitive){
   const listed=await inChunks(worktree,["--no-optional-locks","ls-files","-z","--stage"],paths,signal);
   if(listed===null)return null;
   const tracked=new Set<string>();
   for(const output of listed)for(const record of output.toString("utf8").split("\0")){const cut=record.indexOf("\t");if(cut>=0)tracked.add(record.slice(cut+1));}
   const spelling=await spellingV1(worktree,[...tracked],signal);
   extra.deleted=spelling.absent;extra.untracked=spelling.composed;
  }
  const fresh=(await carriedViewV1(worktree,parseGitViewV1(outputs.map(output=>output.toString("utf8")).join(""),extra))).lines;
  const covered=(line:string):boolean=>{
   let path=line.slice(3);
   for(;;){
    if(affected.has(path))return true;
    const cut=path.lastIndexOf("/");
    if(cut<0)return false;
    path=path.slice(0,cut);
   }
  };
  return assembleViewV1([...previous.lines.filter(line=>!covered(line)),...fresh],stage);
 }catch{return null;}
}

/** 📚️ One tracked entry of the stage: mode, object id and path. */
export type GitTrackedV1=Readonly<{mode:string;oid:string;path:string}>;

/** 📚️ Lists the stage with `git ls-files -z --stage` (unmerged entries once, by their first stage; the object id is kept for symbolic links only, `""` otherwise, so that a quarter of a million entries do not hold a quarter of a million ids); `null` when git fails. */
export async function trackedEntriesV1(worktree:string,signal?:AbortSignal):Promise<readonly GitTrackedV1[]|null>{
 const outcome=await git(worktree,["--no-optional-locks","ls-files","-z","--stage"],{signal,view:true,env:{GIT_OPTIONAL_LOCKS:"0"}});
 if(outcome===null||outcome.code!==0)return null;
 const entries:GitTrackedV1[]=[];
 let last="";
 for(const record of outcome.out.toString("utf8").split("\0")){
  const cut=record.indexOf("\t");
  if(cut<0)continue;
  const [mode,oid]=record.slice(0,cut).split(" "),path=record.slice(cut+1);
  if(mode===undefined||oid===undefined||path===last)continue;
  last=path;entries.push({mode,oid:mode==="120000"?oid:"",path});
 }
 return entries;
}

/** 🧿️ What the worktree volume does with names: `caseInsensitive` (a name is found under another case), `normalisationInsensitive` (an NFD name is found under its NFC spelling), `symlinks` (a link can be created there). */
export type GitVolumeV1=Readonly<{caseInsensitive:boolean;normalisationInsensitive:boolean;symlinks:boolean}>;

const volumes=new Map<string,Promise<GitVolumeV1>>();

/** 🧿️ Probes the volume of the worktree once per process (three probe files below `.cache/f/tmp`, removed at once). */
export function volumeV1(worktree:string):Promise<GitVolumeV1>{
 let known=volumes.get(worktree);
 if(known===undefined){
  known=(async()=>{
   const directory=join(worktree,".cache","f","tmp"),name=token();
   await mkdir(directory,{recursive:true}).catch(()=>undefined);
   const file=join(directory,`Ab${name}.\u00e9`);
   let caseInsensitive=false,normalisationInsensitive=false;
   try{
    await writeFile(file,"");
    caseInsensitive=await stat(join(directory,`aB${name}.\u00e9`)).then(()=>true,()=>false);
    normalisationInsensitive=await stat(join(directory,`Ab${name}.e\u0301`)).then(()=>true,()=>false);
   }catch{}finally{await rm(file,{force:true});}
   return {caseInsensitive,normalisationInsensitive,symlinks:await probeSymlinks(worktree)};
  })();
  volumes.set(worktree,known);
 }
 return known;
}

/** 🔤️ The spelling of tracked paths against the names the disk lists: `absent` holds every path whose exact spelling is not in its directory listing (a variant, or a deleted file), `variants` those among them for which the listing holds a name that folds (`foldNameV1`) to the same text (`Foo.txt` against `foo.txt`, NFC against NFD) and `composed` those whose variant is only the NFD spelling of the same NFC name. */
export type GitSpellingV1=Readonly<{absent:readonly string[];variants:readonly string[];composed:readonly string[]}>;

/** 🔤️ Compares the spelling of tracked paths with the names the disk lists. The walk descends only through exactly spelled directories and lists each once, so a missing or differently spelled directory makes every path below it absent without listing anything. */
export async function spellingV1(worktree:string,paths:readonly string[],signal?:AbortSignal):Promise<GitSpellingV1>{
 const listings=new Map<string,ReadonlySet<string>|null>(),absent:string[]=[],variants:string[]=[],composed:string[]=[];
 const names=async(directory:string):Promise<ReadonlySet<string>|null>=>{
  const known=listings.get(directory);
  if(known!==undefined)return known;
  const listing=await readdir(directory===""?worktree:join(worktree,directory)).then(list=>new Set(list),()=>null);
  listings.set(directory,listing);
  return listing;
 };
 let previous:string[]=[];
 for(const path of paths){
  signal?.throwIfAborted();
  const parts=path.split("/");
  let common=0;
  while(common<previous.length-1&&common<parts.length-1&&previous[common]===parts[common])common++;
  for(let level=common+1;level<previous.length;level++)listings.delete(previous.slice(0,level).join("/"));
  previous=parts;
  let prefix="";
  for(const part of parts){
   const listing=await names(prefix);
   if(listing===null){absent.push(path);break;}
   if(listing.has(part)){prefix=prefix===""?part:`${prefix}/${part}`;continue;}
   absent.push(path);
   const wanted=foldNameV1(part),near=[...listing].filter(candidate=>foldNameV1(candidate)===wanted);
   if(near.length>0){variants.push(path);if(near.every(candidate=>candidate.normalize("NFC")===part))composed.push(path);}
   break;
  }
 }
 return {absent,variants,composed};
}

/** 🛡️ What a machine can say about the inputs of its own view, all measured under the pins: `version` (git), `pins` (`gitPinsHashV1`), `volume` (case and normalisation behaviour, link capability, `longPaths` the Windows policy or null elsewhere), `infoExclude` (rules in the per-machine `.git/info/exclude`), `caseIgnore` (shared or tracked paths that a case-insensitive `.gitignore` match would hide, `caseDependentIgnoresV1`; about ten seconds of CPU on a quarter of a million paths, so the answer is kept for an hour while the root `.gitignore` text is the same), `caseMismatch` (tracked paths whose spelling on this disk is a case or normalisation variant), `trackedIgnored` (tracked paths a rule ignores), `trackedRefused` (tracked paths or links the path policy refuses, or members of a case collision), `links` (tracked symbolic links), `overBudget` (tracked paths longer than this operating system holds below this worktree) and `modeMismatch` (tracked files whose executable bit differs from the index mode; null where the bit cannot be read, Windows). */
export type GitGuardsV1=Readonly<{version:string|null;pins:string;volume:GitVolumeV1&Readonly<{longPaths:boolean|null}>;infoExclude:number;caseIgnore:number;caseMismatch:number;trackedIgnored:number;trackedRefused:number;links:number;overBudget:number;modeMismatch:number|null;samples:readonly string[]}>;

let versions=new Map<string,string|null>();
const caseIgnored=new Map<string,Readonly<{key:string;at:number;found:readonly string[]}>>(),CASE_IGNORE_MILLISECONDS=3_600_000;

/** 🔢️ The version of the resolved git (`2.54.0` of `git version 2.54.0 (Apple Git-xxx)`); `null` without git. */
export async function gitVersionV1():Promise<string|null>{
 const file=await resolveGitV1();
 if(file===null)return null;
 if(versions.has(file))return versions.get(file)!;
 const found=await new Promise<string|null>(done=>{try{const child=spawn(file,["--version"],{stdio:["ignore","pipe","ignore"],windowsHide:true});let seen="";child.stdout.on("data",chunk=>{seen+=String(chunk);});child.on("error",()=>{done(null);});child.on("close",()=>{done(/git version (\d+(?:\.\d+)*)/u.exec(seen)?.[1]??null);});}catch{done(null);}});
 versions.set(file,found);
 return found;
}

const listOf=(outcome:Outcome|null):readonly string[]|null=>outcome!==null&&outcome.code===0?outcome.out.toString("utf8").split("\0").filter(item=>item!==""):null;

/** 🛡️ Measures `GitGuardsV1` for one worktree; `null` without git, a repository or when a git step fails or `signal` aborts. Read-only (`--no-optional-locks`); the volume probe leaves nothing behind. `platform` and `longPaths` are injectable for tests; `modeless` (default: Windows) skips the executable-bit comparison. */
export async function gitGuardsV1(worktree:string,options:Readonly<{signal?:AbortSignal;platform?:NodeJS.Platform;longPaths?:boolean|null;modeless?:boolean}>={}):Promise<GitGuardsV1|null>{
 const {signal}=options,platform=options.platform??process.platform;
 try{
  if(!await gitPresentV1(worktree,signal))return null;
  const quiet={signal,view:true,env:{GIT_OPTIONAL_LOCKS:"0"}},tracked=await trackedEntriesV1(worktree,signal);
  if(tracked===null)return null;
  const untracked=listOf(await git(worktree,["--no-optional-locks","ls-files","-z","--others","--exclude-standard"],quiet)),ignored=listOf(await git(worktree,["--no-optional-locks","ls-files","-z","--cached","--ignored","--exclude-standard"],quiet));
  if(untracked===null||ignored===null||signal?.aborted===true)return null;
  const samples:string[]=[],volume=await volumeV1(worktree),ignoreText=await readFile(join(worktree,".gitignore"),"utf8").catch(()=>""),exclude=await readFile(join(worktree,".git","info","exclude"),"utf8").catch(()=>"");
  const ignoreKey=createHash("sha256").update(ignoreText).digest("hex"),remembered=caseIgnored.get(worktree),clock=Date.now();
  const caseIgnore=remembered!==undefined&&remembered.key===ignoreKey&&clock-remembered.at<CASE_IGNORE_MILLISECONDS?remembered.found:await caseDependentIgnoresV1(ignoreText,[...tracked.map(entry=>entry.path),...untracked],{signal});
  if(caseIgnore!==remembered?.found)caseIgnored.set(worktree,{key:ignoreKey,at:clock,found:caseIgnore});
  for(const path of caseIgnore.slice(0,3))samples.push(`case-ignore ${path}`);
  const spelling=await spellingV1(worktree,tracked.map(entry=>entry.path),signal);
  for(const path of spelling.variants.slice(0,3))samples.push(`case-mismatch ${path}`);
  const budget=pathBudgetV1(platform,options.longPaths===true);
  let refused=0,overBudget=0,modeMismatch:number|null=0;
  const folds=new Map<string,number>(),links:GitTrackedV1[]=[];
  for(const entry of tracked){
   folds.set(foldNameV1(entry.path),(folds.get(foldNameV1(entry.path))??0)+1);
   if(entry.mode==="160000")continue;
   if(refusePathV1(entry.path)!==undefined){refused++;if(samples.length<10)samples.push(`refused ${entry.path}`);}
   if(entry.mode==="120000")links.push(entry);
   if(absoluteUnitsV1(budget,worktree,entry.path)>budget.limit){overBudget++;if(samples.length<10)samples.push(`over-budget ${entry.path}`);}
  }
  for(const count of folds.values())if(count>1)refused+=count;
  for(const path of await unsafeLinksV1(worktree,links,signal)){refused++;if(samples.length<10)samples.push(`unsafe-link ${path}`);}
  if(options.modeless??platform==="win32")modeMismatch=null;
  else{
   const files=tracked.filter(entry=>entry.mode==="100644"||entry.mode==="100755");
   for(let start=0;start<files.length;start+=256){
    signal?.throwIfAborted();
    const verdicts=await Promise.all(files.slice(start,start+256).map(async entry=>{const info=await lstat(join(worktree,entry.path)).catch(()=>undefined);return info!==undefined&&info.isFile()&&((info.mode&0o111)!==0)!==(entry.mode==="100755")?entry.path:null;}));
    for(const path of verdicts)if(path!==null){modeMismatch++;if(samples.length<10)samples.push(`mode ${path}`);}
   }
  }
  const infoExclude=exclude.split("\n").filter(line=>line.trim()!==""&&!line.trimStart().startsWith("#")).length;
  return {version:await gitVersionV1(),pins:gitPinsHashV1(),volume:{...volume,longPaths:options.longPaths??null},infoExclude,caseIgnore:caseIgnore.length,caseMismatch:spelling.variants.length,trackedIgnored:ignored.length,trackedRefused:refused,links:links.length,overBudget,modeMismatch,samples};
 }catch{return null;}
}

const trees=new Map<string,Readonly<{signature:string;tree:string}>>();

type StageRecordV1=Readonly<{version:1;state:StageHealthV1["state"];reason:StageReasonV1|null;since:number;applying:Readonly<{base:boolean;paths:readonly string[]}>|null}>;
type Known=Readonly<{tree:string|null;baseHlc:string|null}>;
const stageFile=(worktree:string):string=>join(worktree,".cache","f","git-index.json");
const lostDirectory=(worktree:string):string=>join(worktree,".cache","f","lost");
type LooseRetry=Readonly<{path:string;objects:Map<string,string>}> & {stamp?:string};
type StageApplyRetry=Readonly<{baseHlc:string;registers:ReadonlyMap<string,string>;loose:ReadonlyMap<string,LooseRetry>}>;
type StageRetry=Readonly<{until:number;attempts:number;apply?:StageApplyRetry}>;
const memory=new Map<string,Known>(),records=new Map<string,StageRecordV1>(),retries=new Map<string,StageRetry>();
const reasons:readonly string[]=["waiting-for-base","objects-missing","unmerged","write-tree-failed","git-failed","index-busy","index-lock-stale","backup-failed"];
const idle:StageRecordV1={version:1,state:"shared",reason:null,since:0,applying:null};

function recordOf(value:unknown):StageRecordV1|undefined{
 if(typeof value!=="object"||value===null)return undefined;
 const item=value as Partial<StageRecordV1>,applying=item.applying;
 const marked=applying===null||typeof applying==="object"&&applying!==undefined&&typeof applying.base==="boolean"&&Array.isArray(applying.paths)&&applying.paths.every(path=>typeof path==="string");
 const stated=item.state==="shared"&&item.reason===null||(item.state==="waiting"||item.state==="degraded")&&typeof item.reason==="string"&&reasons.includes(item.reason);
 return item.version===1&&stated&&marked&&typeof item.since==="number"?item as StageRecordV1:undefined;
}

async function readRecord(worktree:string):Promise<StageRecordV1>{
 const cached=records.get(worktree);
 if(cached!==undefined)return cached;
 let value=idle;
 try{value=recordOf(JSON.parse(await readFile(stageFile(worktree),"utf8")))??idle;}catch{}
 records.set(worktree,value);
 return value;
}

async function writeRecord(worktree:string,value:StageRecordV1):Promise<void>{
 records.set(worktree,value);
 const file=stageFile(worktree),temporary=`${file}.${token()}`;
 await mkdir(join(worktree,".cache","f"),{recursive:true});
 try{
  const handle=await open(temporary,"w");
  try{await handle.writeFile(JSON.stringify(value));await handle.sync();}finally{await handle.close();}
  await rename(temporary,file);
 }catch{await rm(temporary,{force:true});}
}

/** 🚦 The stage health this process recorded last, from memory and without a read; `null` before it recorded one. The replica states it among the failures of its attestation, so that `fleet` names why a machine does not follow the stage of the archivist. */
export const stageRecordedV1=(worktree:string):StageHealthV1|null=>{const value=records.get(worktree);return value===undefined?null:{state:value.state,reason:value.reason};};

/** 🚥 Reads the stage health the running replica recorded in `.cache/f/git-index.json`; `shared` when there is no usable record. */
export async function stageHealthV1(worktree:string):Promise<StageHealthV1>{
 try{const value=recordOf(JSON.parse(await readFile(stageFile(worktree),"utf8")));return value===undefined?{state:"shared",reason:null}:{state:value.state,reason:value.reason};}
 catch{return {state:"shared",reason:null};}
}

async function setHealth(host:IndexHostV1,state:StageHealthV1["state"],reason:StageReasonV1|null,seconds?:number):Promise<void>{
 const current=await readRecord(host.worktree);
 if(current.state===state&&current.reason===reason)return;
 await writeRecord(host.worktree,{...current,state,reason,since:host.now()});
 if(state==="shared")host.say({event:"repaired",stage:"git",reason:current.reason??"shared"});
 else host.say({event:"blocked",stage:"git",reason:reason!,...(seconds===undefined?{}:{seconds})});
}

async function clearHealth(host:IndexHostV1,owned:readonly StageReasonV1[]):Promise<void>{
 const current=await readRecord(host.worktree);
 if(current.reason!==null&&owned.includes(current.reason))await setHealth(host,"shared",null);
}

async function mark(worktree:string,applying:StageRecordV1["applying"]):Promise<void>{
 const current=await readRecord(worktree);
 if(applying===null&&current.applying===null)return;
 await writeRecord(worktree,{...current,applying});
}

const entryPath=(change:GitChangeV1):string=>`${INDEX_PREFIX_V1}${change.path}`;
const representable=(path:string):boolean=>path!==""&&!path.includes("\\")&&!path.includes("\0")&&!path.split("/").some(part=>part===""||part==="."||part===".."||part===".git")&&path.length<=1000&&refusePathV1(path)===undefined;

/** 🐚 Lists the tracked shell scripts (`*.sh`) of the worktree that contain a carriage return: the replica copies bytes and never applies `eol=lf`, so a script saved with CRLF on a Windows machine would break bash on Linux and macOS. `null` without git or a repository. */
export async function crlfScriptsV1(worktree:string,signal?:AbortSignal):Promise<readonly string[]|null>{
 const tracked=await trackedEntriesV1(worktree,signal);
 if(tracked===null)return null;
 const scripts=tracked.filter(entry=>entry.path.endsWith(".sh")&&(entry.mode==="100644"||entry.mode==="100755")),found:string[]=[];
 for(let start=0;start<scripts.length;start+=64){
  signal?.throwIfAborted();
  const verdicts=await Promise.all(scripts.slice(start,start+64).map(async entry=>{const bytes=await readFile(join(worktree,...entry.path.split("/"))).catch(()=>undefined);return bytes!==undefined&&bytes.includes(13)?entry.path:null;}));
  for(const path of verdicts)if(path!==null)found.push(path);
 }
 return found;
}

/** 🔗 Lists the paths among the given symbolic-link entries of the stage or of a commit whose target the link policy refuses (absolute or drive-qualified targets, targets that leave the worktree); empty when the blobs cannot be read. */
async function unsafeLinksV1(worktree:string,links:readonly Readonly<{path:string;oid:string}>[],signal?:AbortSignal):Promise<readonly string[]>{
 if(links.length===0)return [];
 const targets=await git(worktree,["cat-file","--batch"],{signal,stdin:`${links.map(entry=>entry.oid).join("\n")}\n`});
 if(targets===null||targets.code!==0)return [];
 const found:string[]=[];
 let offset=0;
 for(const entry of links){
  const end=targets.out.indexOf(10,offset),size=Number(targets.out.subarray(offset,end).toString("utf8").split(" ")[2]);
  const target=targets.out.subarray(end+1,end+1+size).toString("utf8");offset=end+1+size+1;
  if(linkRefusalV1(entry.path,target)!==undefined)found.push(entry.path);
 }
 return found;
}
const treeOf=(entry:FleetManifestEntryV1|undefined):string|undefined=>entry?.operation==="link"&&entry.target!.startsWith("tree ")&&oidPattern.test(entry.target!.slice(5))?entry.target!.slice(5):undefined;

function changeOf(path:string,entry:FleetManifestEntryV1):GitChangeV1|undefined{
 const name=path.slice(INDEX_PREFIX_V1.length);
 if(refusePathV1(name)!==undefined)return undefined;
 if(entry.operation==="delete")return {path:name,mode:"000000",oid:""};
 if(entry.operation!=="link")return undefined;
 const [mode,oid,...extra]=entry.target!.split(" ");
 return extra.length===0&&mode!==undefined&&/^[0-7]{6}$/u.test(mode)&&oid!==undefined&&oidPattern.test(oid)?{path:name,mode,oid}:undefined;
}

const rebaseRequest=(worktree:string):string=>join(worktree,".cache","f","index-rebase");

/** 🧲 Tells whether this machine was asked to publish its current stage as the new base (`.cache/f/index-rebase` exists); the running replica consumes the request on its next publish step. */
export async function rebaseRequestedV1(worktree:string):Promise<boolean>{
 try{await lstat(rebaseRequest(worktree));return true;}catch{return false;}
}

/** 🪝 Asks the running replica of this machine to publish its current stage as the new base of the fleet: every machine then replaces its stage by this one (the old stage is kept under `.cache/f/lost`), and entries older than the new base are void; the replica lists the registers it voided in `.cache/f/lost/rebase-*.tsv`. */
export async function requestRebaseV1(worktree:string):Promise<void>{
 await mkdir(join(worktree,".cache","f"),{recursive:true});
 await writeFile(rebaseRequest(worktree),"");
}

type Register=readonly [string,FleetManifestEntryV1];

/** 🏛️ Whether a register belongs to the stage of the fleet: the base and every entry the archivist (`IndexHostV1.archivist`, the group server) wrote. An entry of any other machine is void: the stage a person made on a member is not a fleet concept, the archivist's stage wins on every machine. */
const archived=(host:IndexHostV1,path:string,entry:FleetManifestEntryV1):boolean=>path===INDEX_BASE_V1||entry.hlc.slice(21)===host.archivist;

/** 🧲️ Names the loose objects that can satisfy pending incremental stage registers before other Git objects are copied. */
export function stageObjectsForApplyV1(host:IndexHostV1,pending:ReadonlyMap<string,FleetManifestEntryV1>):ReadonlySet<string>|undefined{
 if(pending.size===0||pending.has(INDEX_BASE_V1))return;
 const base=host.base();
 if(base===undefined||treeOf(base)===undefined)return;
 const objects=new Set<string>();
 let incremental=false;
 for(const [path,entry] of pending){
  const change=authorStageChange(host,base.hlc,path,entry);
  if(change===undefined)continue;
  incremental=true;
  if(change.mode!=="000000"&&change.mode!=="160000")objects.add(".git/objects/"+change.oid.slice(0,2)+"/"+change.oid.slice(2));
 }
 if(incremental)return objects;
}

const authorStageChange=(host:IndexHostV1,baseHlc:string,path:string,entry:FleetManifestEntryV1):GitChangeV1|undefined=>
 path.startsWith(INDEX_PREFIX_V1)&&entry.hlc>baseHlc&&archived(host,path,entry)?changeOf(path,entry):undefined;

function stageApplyRetry(host:IndexHostV1,baseHlc:string,pending:ReadonlyMap<string,FleetManifestEntryV1>,missing:Iterable<string>):StageApplyRetry{
 const registers=new Map<string,string>(),loose=new Map<string,LooseRetry>(),previous=retries.get(host.worktree)?.apply;
 for(const [path,entry] of pending)if(authorStageChange(host,baseHlc,path,entry)!==undefined)registers.set(path,entry.hlc);
 for(const oid of missing){
  const prefix=oid.slice(0,2),name=oid.slice(2);
  let group=loose.get(prefix);
  if(group===undefined){group={path:join(host.worktree,".git","objects",prefix),objects:new Map()};loose.set(prefix,group);}
  group.objects.set(name,previous?.loose.get(prefix)?.objects.get(name)??"-");
 }
 return {baseHlc,registers,loose};
}

/** 🛎️ Rechecks only Git prerequisites that can make deferred member stage work ready. An unchanged loose-object directory needs no per-object checks. */
async function stageApplyWaiting(host:IndexHostV1,pending:ReadonlyMap<string,FleetManifestEntryV1>,signal?:AbortSignal):Promise<boolean>{
 const retry=retries.get(host.worktree);
 if(retry===undefined||host.now()>=retry.until)return false;
 const waiting=retry.apply;
 if(waiting===undefined)return true;
 if(host.base()?.hlc!==waiting.baseHlc)return false;
 let matches=0;
 for(const [path,entry] of pending){
  if(waiting.registers.get(path)===entry.hlc)matches++;
  else if(authorStageChange(host,waiting.baseHlc,path,entry)!==undefined)return false;
 }
 if(matches!==waiting.registers.size)return false;
 let arrived=false;
 for(const group of waiting.loose.values()){
  signal?.throwIfAborted();
  const stamp=await statStamp(group.path);
  if(stamp===group.stamp)continue;
  for(const [name,before] of group.objects){
   signal?.throwIfAborted();
   const after=stamp==="-"?"-":await statStamp(join(group.path,name));
   if(after!=="-"&&before!==after)arrived=true;
   group.objects.set(name,after);
  }
  group.stamp=stamp;
 }
 return !arrived;
}

const registersOf=(host:IndexHostV1,visit:(each:(path:string,entry:FleetManifestEntryV1)=>void)=>void):Register[]=>{const found:Register[]=[];visit((path,entry)=>{if(archived(host,path,entry))found.push([path,entry]);});return found;};

async function foldAll(host:IndexHostV1,signal?:AbortSignal):Promise<string|null>{
 const base=host.base();
 return base===undefined?null:foldRegistersV1(host.worktree,base,registersOf(host,host.each),signal);
}

async function knownOf(host:IndexHostV1,signal?:AbortSignal):Promise<Known|null>{
 const cached=memory.get(host.worktree);
 if(cached!==undefined)return cached;
 const held=registersOf(host,host.held),base=held.find(([path])=>path===INDEX_BASE_V1)?.[1];
 if(base===undefined||treeOf(base)===undefined)return {tree:null,baseHlc:null};
 const tree=await foldRegistersV1(host.worktree,base,held,signal);
 if(tree===null)return null;
 const value:Known={tree,baseHlc:base.hlc};
 memory.set(host.worktree,value);
 return value;
}

/** 🛟 Counts the unmerged entries of the stage (`git ls-files -u`); 0 when git fails. */
export async function unmergedV1(worktree:string,signal?:AbortSignal):Promise<number>{
 const outcome=await git(worktree,["ls-files","-u","-z"],{signal});
 return outcome===null||outcome.code!==0?0:outcome.out.toString("utf8").split("\0").filter(item=>item!=="").length;
}

async function lockAge(worktree:string):Promise<number|undefined>{
 const info=await stat(join(worktree,".git","index.lock")).catch(()=>undefined);
 return info===undefined?undefined:Math.max(0,Date.now()-info.mtimeMs);
}

const KEPT_STAGES=3;

async function keepStage(host:IndexHostV1,differing:number):Promise<boolean>{
 const name=`index-${Date.now()}-${token()}`;
 try{
  await mkdir(lostDirectory(host.worktree),{recursive:true});
  await copyFile(join(host.worktree,".git","index"),join(lostDirectory(host.worktree),name));
 }catch{await setHealth(host,"degraded","backup-failed");return false;}
 host.say({event:"overwritten",stage:"git",reason:"local-stage-replaced",count:differing,lost:`lost/${name}`});
 const old=(await readdir(lostDirectory(host.worktree)).catch(()=>[] as string[])).filter(item=>item.startsWith("index-")).sort();
 for(const item of old.slice(0,Math.max(0,old.length-KEPT_STAGES)))await rm(join(lostDirectory(host.worktree),item),{force:true}).catch(()=>{});
 return true;
}

async function keepVoided(host:IndexHostV1,voided:readonly Readonly<{path:string;origin:string;was:string}>[]):Promise<string|undefined>{
 const name=`rebase-${Date.now()}-${token()}.tsv`;
 try{
  await mkdir(lostDirectory(host.worktree),{recursive:true});
  await writeFile(join(lostDirectory(host.worktree),name),voided.map(item=>`${item.path}\t${item.origin}\t${item.was}\n`).join(""));
  return `lost/${name}`;
 }catch{return undefined;}
}

async function voidedBy(host:IndexHostV1,local:string,signal?:AbortSignal):Promise<readonly Readonly<{path:string;origin:string;was:string}>[]>{
 const base=host.base(),registers=registersOf(host,host.each),folded=base===undefined?null:await foldRegistersV1(host.worktree,base,registers,signal);
 const delta=folded===null?null:await diffTreeV1(host.worktree,folded,local,signal);
 if(base===undefined||delta===null)return [];
 const changed=new Set(delta.map(change=>change.path)),voided:Readonly<{path:string;origin:string;was:string}>[]=[];
 for(const [path,entry] of registers){
  if(path===INDEX_BASE_V1||entry.hlc<=base.hlc||!changed.has(path.slice(INDEX_PREFIX_V1.length)))continue;
  voided.push({path:path.slice(INDEX_PREFIX_V1.length),origin:entry.hlc.slice(21),was:entry.operation==="delete"?"absent":entry.target!});
 }
 return voided;
}

async function backoff(host:IndexHostV1,reason:StageReasonV1,apply?:StageApplyRetry):Promise<void>{
 const {worktree}=host,attempts=(retries.get(worktree)?.attempts??0)+1,age=reason==="index-busy"?await lockAge(worktree):undefined,stale=age!==undefined&&age>STALE_LOCK_MILLISECONDS;
 if(stale&&!host.server){
  await rm(join(worktree,".git","index.lock"),{force:true}).catch(()=>{});
  retries.delete(worktree);
  host.say({event:"repaired",stage:"git",reason:"index-lock-stale",seconds:Math.floor(age/1000)});
  return;
 }
 retries.set(worktree,{attempts,until:host.now()+gitStageRetryDelayV1(attempts),apply});
 if(reason!=="backup-failed")await setHealth(host,stale?"degraded":"waiting",stale?"index-lock-stale":reason,age===undefined?undefined:Math.floor(age/1000));
}

/** 🕳️ Names the objects the index file `index` refers to that the repository does not hold (every entry but gitlinks; `git write-tree` cannot tell, because `read-tree` primes the cache tree and `write-tree` trusts it); `null` when git fails. */
async function stageMissingV1(worktree:string,index:string,signal?:AbortSignal):Promise<readonly string[]|null>{
 const listing=await git(worktree,["ls-files","-s","-z"],{index,signal});
 if(listing===null||listing.code!==0)return null;
 const oids=new Set<string>();
 for(const record of listing.out.toString("utf8").split("\0")){
  const cut=record.indexOf("\t");
  if(cut<0)continue;
  const [mode,oid]=record.slice(0,cut).split(" ");
  if(mode!==undefined&&oid!==undefined&&mode!=="160000")oids.add(oid);
 }
 const missing=await missingObjectsV1(worktree,[...oids],signal);
 return missing===null?null:[...missing];
}

type StageAdoptionFailure=Readonly<{reason:StageReasonV1;missing?:readonly string[]}>;

/** 🧱 Builds the stage of `tree` on a temporary index file (`GIT_INDEX_FILE`) from a copy of the machine's own index, so that the stat data of every entry that already holds the right object survives (`git read-tree -m <tree>` with one tree keeps the stat data of matching entries; an index that cannot be read starts empty and every entry is checked against the file by `update-index --refresh`, the files that are not there being ignored), and reports missing objects if the new index cannot reproduce `tree`. The worktree is never touched. */
async function buildStage(host:IndexHostV1,tree:string,work:string,signal?:AbortSignal):Promise<StageAdoptionFailure|null>{
 const {worktree}=host,index=join(worktree,".git","index");
 let built=false;
 if(await lstat(index).then(info=>info.isFile(),()=>false)){
  try{await copyFile(index,work);}catch{}
  const merged=await git(worktree,["read-tree","-m",tree],{index:work,signal});
  built=merged!==null&&merged.code===0;
  if(!built)await rm(work,{force:true});
 }
 if(!built&&!await readTreeV1(worktree,tree,{index:work,signal}))return {reason:"objects-missing"};
 await git(worktree,["update-index","-q","--refresh","--ignore-missing"],{index:work,signal});
 const absent=await stageMissingV1(worktree,work,signal);
 if(absent!==null&&absent.length>0)return {reason:"objects-missing",missing:absent};
 return absent!==null&&await writeTreeV1(worktree,{index:work,signal})===tree?null:{reason:"objects-missing"};
}

/** 🪞 Replaces the stage of this machine by `tree`, the stage of the archivist: the new index is built beside the old one (`buildStage`, the old file is only read), the old index file is copied once to `.cache/f/lost/index-*` (the newest three are kept) with an `overwritten` event, and the new file is written into `.git/index.lock`, taken with exclusive creation like git itself does, and renamed over `.git/index`, so a concurrent git sees either the old or the new stage and never a half one. The worktree is never touched. Returns `null` on success, else the reason the stage still waits: `index-busy` (git holds `index.lock`), `objects-missing` (the tree or an object of it has not arrived), `backup-failed` or `git-failed`. */
async function adoptStage(host:IndexHostV1,tree:string,local:string|null,signal?:AbortSignal):Promise<StageAdoptionFailure|null>{
 const {worktree}=host,index=join(worktree,".git","index"),lock=`${index}.lock`;
 let handle:Awaited<ReturnType<typeof open>>|undefined;
 try{handle=await open(lock,"wx");}catch(error){return {reason:(error as NodeJS.ErrnoException).code==="EEXIST"?"index-busy":"git-failed"};}
 const directory=join(worktree,".cache","f","tmp"),work=join(directory,`index-adopt-${token()}`);
 let placed=false;
 try{
  await mkdir(directory,{recursive:true});
  const failure=await buildStage(host,tree,work,signal);
  if(failure!==null)return failure;
  if(!host.server&&abortedV1(signal))return {reason:"git-failed"};
  if(await lstat(index).then(info=>info.isFile(),()=>false)){
   const differing=local===null?0:(await diffTreeV1(worktree,local,tree,signal))?.length??0;
   if(!await keepStage(host,differing))return {reason:"backup-failed"};
  }
  await handle.writeFile(await readFile(work));
  await handle.sync();
  await handle.close();handle=undefined;
  if(!host.server&&abortedV1(signal))return {reason:"git-failed"};
  await rename(lock,index);placed=true;
  return null;
 }catch{return {reason:"git-failed"};}
 finally{
  if(handle!==undefined)await handle.close().catch(()=>{});
  await rm(work,{force:true});await rm(`${work}.lock`,{force:true});
  if(!placed)await rm(lock,{force:true});
 }
}

async function restoreStage(host:IndexHostV1,none:IndexPublishV1,signal?:AbortSignal):Promise<IndexPublishV1>{
 const known=await knownOf(host,signal);
 if(host.base()===undefined||known!==null&&known.tree===null)return none;
 const target=await foldAll(host,signal);
 if(known===null||target===null){await setHealth(host,"waiting","objects-missing");return {...none,waiting:true};}
 if(target===await emptyTreeV1(host.worktree,signal))return none;
 const failed=await adoptStage(host,target,null,signal);
 if(failed!==null){await backoff(host,failed.reason);return {...none,waiting:true};}
 memory.set(host.worktree,{tree:target,baseHlc:host.base()!.hlc});signatures.set(host.worktree,await signatureOf(host.worktree));
 retries.delete(host.worktree);
 host.say({event:"repaired",stage:"git",reason:"index-missing"});
 return none;
}

const STAGE_OWNED:readonly StageReasonV1[]=["objects-missing","index-busy","index-lock-stale","git-failed","backup-failed","waiting-for-base","unmerged","write-tree-failed"];

/** 🧲 The publish step of a member: a member publishes nothing, the stage of the archivist wins. Once every register has been applied, a stage that cannot be written (unmerged entries, a failing git, an index that is absent, corrupt or far off) or whose tree differs from the tree of the registers is replaced by that tree (`adoptStage`: the old index is kept once under `.cache/f/lost`); an `index.lock` older than ten minutes belongs to a dead git and is removed by a member (the archivist's own stays a degraded `index-lock-stale` for its operator). A fleet without a base yet, or a tree that has not arrived, makes it wait with the reason recorded. */
async function followStage(host:IndexHostV1,before:string,pending:ReadonlyMap<string,FleetManifestEntryV1>,signal?:AbortSignal):Promise<IndexPublishV1>{
 const none:IndexPublishV1={published:0,base:false,waiting:false},{worktree}=host;
 if(pending.size>0)return none;
 const base=host.base();
 if(base===undefined||treeOf(base)===undefined){await setHealth(host,"waiting","waiting-for-base");return {...none,waiting:true};}
 const known=await knownOf(host,signal);
 if(known===null){await setHealth(host,"waiting","objects-missing");return {...none,waiting:true};}
 if(known.tree===null)return {...none,waiting:true};
 if(await stageApplyWaiting(host,pending,signal))return {...none,waiting:true};
 const local=before===""?null:await readIndexTreeSnapshotV1(worktree,before,{signal,requireLock:false}),settled=await signatureOf(worktree);
 if(abortedV1(signal)||settled!==before)return {...none,waiting:true};
 if(local!==null&&local===known.tree){signatures.set(worktree,settled);retries.delete(worktree);await clearHealth(host,STAGE_OWNED);return none;}
 const failed=await adoptStage(host,known.tree,local,signal);
 if(failed!==null){
  const retry=failed.reason==="objects-missing"?stageApplyRetry(host,base.hlc,pending,failed.missing??[]):undefined;
  await backoff(host,failed.reason,retry);
  return {...none,waiting:true};
 }
 memory.set(worktree,{...known,tree:known.tree});signatures.set(worktree,await signatureOf(worktree));
 retries.delete(worktree);
 await clearHealth(host,STAGE_OWNED);
 host.say({event:"applied",stage:"git",count:1,reason:local===null?"index-rebuilt":"archivist-stage"});
 return none;
}

/** 📤 The publish step. Only the archivist (the group server, `host.server`) publishes; a member follows (`followStage`). The archivist's stage tree is compared with the tree of the registers this machine has published (`held`, folded as `verifyIndexV1` folds; kept in memory afterwards), and every differing path becomes an entry register in one atomic journal append (`diff-tree -r -z`), however many there are. The tree itself is published as the base (`tree <oid>`) when the machine has no base yet, on an explicit rebase request (which lists the registers it voids), and whenever a changed path cannot be an entry (a path the entry policy cannot represent, or an unsafe link): the base carries the whole tree, so the stage of every machine can equal the archivist's stage exactly. A semantic no-op (a `git status` that only refreshes stat data) leaves the tree equal and publishes nothing; so does an unchanged `.git/index` stat signature without any process. A stage that cannot be written (unmerged entries, a failing git) is never published and is reported as degraded; a missing `.git/index` is restored from the registers. After a crash during an apply, the paths that apply was writing are not re-published as local edits. With `prepareObjects`, only incremental changes on the archivist can publish. Before emitting them, the callback must accept the distinct object IDs referenced by the entry events, excluding gitlinks. A refused batch can retry the same index signature. */
export async function publishIndexV1(host:IndexHostV1,signal?:AbortSignal,prepareObjects?:(oids:readonly string[],paths:readonly string[])=>Promise<boolean>):Promise<IndexPublishV1>{
 const none:IndexPublishV1={published:0,base:false,waiting:false},waiting:IndexPublishV1={...none,waiting:true},{worktree}=host;
 if(prepareObjects!==undefined&&!host.server||!await gitPresentV1(worktree,signal))return none;
 const before=await signatureOf(worktree),asked=await rebaseRequestedV1(worktree);
 if(asked&&!host.server){await rm(rebaseRequest(worktree),{force:true});host.say({event:"skipped",stage:"git",reason:"archivist-only"});}
 const forced=asked&&host.server;
 if(!forced&&before!==""&&signatures.get(worktree)===before)return none;
 if(prepareObjects!==undefined&&(before===""||forced||treeOf(host.base())===undefined))return waiting;
 const pending=host.pending();
 let marker=(await readRecord(worktree)).applying;
 if(marker!==null&&pending.size===0){await mark(worktree,null);marker=null;}
 if(marker?.base===true&&!forced)return waiting;
 if(!host.server)return followStage(host,before,pending,signal);
 if(before==="")return forced?none:restoreStage(host,none,signal);
 const incremental=!forced&&treeOf(host.base())!==undefined,cached=incremental?trees.get(worktree):undefined;
 const locked=incremental&&cached?.signature!==before&&await lstat(join(worktree,".git","index.lock")).then(info=>info.isFile(),()=>false);
 const observed=cached?.signature===before?cached.tree:locked?await readIndexTreeSnapshotV1(worktree,before,{signal}):undefined;
 let local=observed===undefined?await writeTreeV1(worktree,{signal}):observed,settled=await signatureOf(worktree);
 if(observed===undefined&&local===null&&incremental&&settled===before)local=await readIndexTreeSnapshotV1(worktree,before,{signal});
 if((observed!==undefined&&settled!==before)||abortedV1(signal))return waiting;
 if(local===null){
  const conflicted=await unmergedV1(worktree,signal)>0;
  await setHealth(host,"degraded",conflicted?"unmerged":"write-tree-failed");
  return conflicted?none:waiting;
 }
 await clearHealth(host,["unmerged","write-tree-failed","git-failed"]);
 if(settled===before)trees.set(worktree,{signature:settled,tree:local});
 const base=host.base();
 let recertified=false;
 const current=async()=>{
  if(abortedV1(signal))return false;
  const actual=await signatureOf(worktree);
  if(abortedV1(signal))return false;
  if(actual===settled)return true;
  if(!incremental||recertified)return false;
  recertified=true;
  if(await readIndexTreeSnapshotV1(worktree,actual,{signal,requireLock:false})!==local||abortedV1(signal))return false;
  settled=actual;return true;
 };
 const publishBase=async(reason:"genesis"|"rebase"|"unrepresentable"):Promise<IndexPublishV1>=>{
  if(prepareObjects!==undefined)return waiting;
  const voided=reason==="rebase"?await voidedBy(host,local,signal):[];
  if(!await current())return waiting;
  const hlcs=await host.emit([{operation:"link",path:INDEX_BASE_V1,target:`tree ${local}`}]);
  if(hlcs===undefined)return waiting;
  memory.set(worktree,{tree:local,baseHlc:hlcs[0]!});signatures.set(worktree,settled);
  if(forced)await rm(rebaseRequest(worktree),{force:true});
  const lost=voided.length>0?await keepVoided(host,voided):undefined;
  host.say({event:"published",stage:"git",path:INDEX_BASE_V1,count:voided.length,reason,...(lost===undefined?{}:{lost})});
  return {published:1,base:true,waiting:false};
 };
 if(forced||base===undefined||treeOf(base)===undefined)return publishBase(forced?"rebase":"genesis");
 const known=await knownOf(host,signal);
 if(known===null){await setHealth(host,"waiting","objects-missing");return waiting;}
 if(known.tree===null)return waiting;
 if(known.tree===local){signatures.set(worktree,settled);return none;}
 const changes=await diffTreeV1(worktree,known.tree,local,signal);
 if(changes===null){memory.delete(worktree);await setHealth(host,"degraded","git-failed");return waiting;}
 const skip=new Set(marker?.paths??[]),events:Readonly<{operation:"link"|"delete";path:string;target?:string}>[]=[],unsafe=new Set(await unsafeLinksV1(worktree,changes.filter(change=>change.mode==="120000"&&!skip.has(change.path)),signal));
 const prepare=prepareObjects??(host.prepareStageInputs===undefined?undefined:(oids:readonly string[],paths:readonly string[])=>host.prepareStageInputs!(oids,paths,signal));
 const objects=prepare===undefined?undefined:new Set<string>();
 let withheld=0;
 for(const change of changes){
  if(skip.has(change.path))continue;
  const reason=!representable(change.path)?"unrepresentable":unsafe.has(change.path)?"unsafe-link":undefined;
  if(reason!==undefined){
   if(prepareObjects!==undefined)return waiting;
   withheld++;host.say({event:"skipped",stage:"git",path:change.path,reason});continue;
  }
  events.push(change.mode==="000000"?{operation:"delete",path:entryPath(change)}:{operation:"link",path:entryPath(change),target:`${change.mode} ${change.oid}`});
  if(change.mode!=="000000"&&change.mode!=="160000")objects?.add(change.oid);
 }
 if(withheld>0)return publishBase("unrepresentable");
 if(events.length>0&&prepare!==undefined){
  if(signal?.aborted===true)return waiting;
  if(!await prepare([...objects!],events.map(event=>event.path.slice(INDEX_PREFIX_V1.length)))){
   await setHealth(host,"waiting","objects-missing");
   return waiting;
  }
  if(!await current())return waiting;
 }
 if(events.length>0&&(!await current()||await host.emit(events)===undefined)){memory.delete(worktree);return waiting;}
 memory.set(worktree,{...known,tree:local});signatures.set(worktree,settled);
 if(events.length>0){
  if(prepare!==undefined)await clearHealth(host,["objects-missing"]);
  host.say({event:"published",stage:"git",count:events.length});
 }
 return {published:events.length,base:false,waiting:false};
}

/** 📥 The apply step: publishes (the archivist) or follows (a member) first, so an unpublished stage is never attributed to the remote, then applies the queued registers of the archivist (entries of any other machine are void): a new base replaces the stage (the delta from the last known tree when the stage still equals it, so stat data survives; otherwise the stage has no proof of its origin and is replaced by the fold of all registers through `adoptStage`, which keeps the old index file under `.cache/f/lost/index-*` with an `overwritten` event that reports the number of replaced entries) and is overlaid by every entry newer than it; plain entries go into one `update-index --index-info` batch. Git fills their local stat cache when a later command refreshes the index. Entries older than the base are void. A member replaces a stage with unmerged entries too; the archivist's own stage with unmerged entries is never reset and the items stay queued. A missing object or a held `index.lock` leaves the items queued; retries back off from one second to 30 seconds and the state is recorded (`index-lock-stale` after ten minutes). A member waiting for missing objects retries earlier when pending author registers change or an exact loose prerequisite appears. If a member cannot reconstruct its held history, it adopts the complete current fold through the writer lock and keeps its prior index. The paths being written are recorded before the write and cleared after the registers are settled. */
export async function applyIndexV1(host:IndexHostV1,signal?:AbortSignal,hydrateStageObjects?:()=>Promise<void>):Promise<IndexApplyV1>{
 const {worktree}=host,pending=host.pending(),unchecked={applied:0,queued:pending.size};
 if(pending.size===0)return unchecked;
 if(!host.server&&(abortedV1(signal)||await stageApplyWaiting(host,pending,signal)))return unchecked;
 if(!await gitPresentV1(worktree,signal))return unchecked;
 await publishIndexV1(host,signal);
 const base=host.base(),baseTree=treeOf(base);
 if(base===undefined||baseTree===undefined)return unchecked;
 if(host.server&&await stageApplyWaiting(host,pending,signal))return unchecked;
 const later=async(reason:StageReasonV1,missing:Iterable<string>=[],remaining=pending):Promise<IndexApplyV1>=>{
  const apply=!host.server&&reason==="objects-missing"?stageApplyRetry(host,base.hlc,remaining,missing):undefined;
  await backoff(host,reason,apply);
  return unchecked;
 };
 const hydrate=async():Promise<boolean>=>{
  const load=hydrateStageObjects;
  if(load===undefined||abortedV1(signal))return false;
  hydrateStageObjects=undefined;
  await load();
  return !abortedV1(signal);
 };
 let known=await knownOf(host,signal);
 if(host.server&&known===null&&await hydrate())known=await knownOf(host,signal);
 if(host.server&&known===null)return later("objects-missing");
 const recovering=known===null,present=!recovering&&await signatureOf(worktree)!=="";
 if(present&&host.server&&await unmergedV1(worktree,signal)>0)return later("unmerged");
 const baseChange=known?.baseHlc!==base.hlc,candidates=new Map<string,GitChangeV1>(),paths:string[]=[],voided:string[]=[];
 const consider=(path:string,entry:FleetManifestEntryV1):void=>{
  if(path===INDEX_BASE_V1)return;
  const change=authorStageChange(host,base.hlc,path,entry);
  if(change===undefined){voided.push(path);return;}
  candidates.set(path,change);
 };
 if(baseChange)host.each(consider);
 else for(const [path,entry] of pending)consider(path,entry);
 const wanted=new Set<string>();
 if(baseChange)wanted.add(baseTree);
 for(const change of candidates.values())if(change.mode!=="000000"&&change.mode!=="160000")wanted.add(change.oid);
 let missing=await missingObjectsV1(worktree,[...wanted],signal);
 if(missing!==null&&missing.size>0&&await hydrate())missing=await missingObjectsV1(worktree,[...missing],signal);
 if(recovering&&abortedV1(signal))return unchecked;
 if(missing===null||baseChange&&missing.has(baseTree)||recovering&&missing.size>0)return later("objects-missing",missing??[]);
 const ready:GitChangeV1[]=[],stalled:string[]=[];
 for(const [path,change] of candidates){
  if(change.mode!=="000000"&&change.mode!=="160000"&&missing.has(change.oid)){stalled.push(path);continue;}
  ready.push(change);paths.push(path);
 }
 let ok=true,adopted:string|null=null;
 await mark(worktree,{base:baseChange,paths:ready.map(change=>change.path)});
 if(baseChange){
  const local=present?await writeTreeV1(worktree,{signal}):null;
  if(present&&local===null&&host.server){await mark(worktree,null);return later("write-tree-failed");}
  if(known?.tree!=null&&local===known.tree){
   const delta=await diffTreeV1(worktree,known.tree,baseTree,signal);
   const gone=delta===null?null:await missingObjectsV1(worktree,delta.filter(change=>change.mode!=="000000"&&change.mode!=="160000").map(change=>change.oid),signal);
   if(gone===null||gone.size>0){await mark(worktree,null);return later("objects-missing",gone??[]);}
   ok=delta!==null&&await updateIndexV1(worktree,delta,{signal,hash:baseTree.length});
  }else{
   const fold=async():Promise<IndexFold>=>recovering?foldIndex(worktree,baseTree,ready,signal,true):{tree:await foldAll(host,signal)};
   let folded=await fold();
   if(folded.missing?.length&&await hydrate())folded=await fold();
   if(recovering&&abortedV1(signal)){await mark(worktree,null);return unchecked;}
   const target=folded.tree;
   if(target===null){await mark(worktree,null);return later("objects-missing",folded.missing);}
   let failed=await adoptStage(host,target,local,signal);
   if(recovering&&failed?.missing?.length&&await hydrate())failed=await adoptStage(host,target,local,signal);
   if(failed!==null){
    await mark(worktree,null);
    return recovering&&abortedV1(signal)?unchecked:later(failed.reason,failed.missing);
   }
   adopted=target;
  }
 }
 if(ok&&adopted===null)ok=await updateIndexV1(worktree,ready,{signal,hash:baseTree.length});
 if(!ok)return later(await lockAge(worktree)===undefined?"git-failed":"index-busy");
 if(baseChange&&(!recovering||pending.has(INDEX_BASE_V1)))await normaliseGitConfigV1(worktree,undefined,signal);
 const tree=recovering?adopted:await writeTreeV1(worktree,{signal});
 if(tree===null)memory.delete(worktree);
 else{
  memory.set(worktree,{tree,baseHlc:base.hlc});
  if(recovering)signatures.delete(worktree);
  else signatures.set(worktree,await signatureOf(worktree));
 }
 const settled=[...(recovering?paths.filter(path=>pending.has(path)):paths),...voided.filter(path=>pending.has(path)),...(pending.has(INDEX_BASE_V1)?[INDEX_BASE_V1]:[])];
 host.settle(settled);
 await mark(worktree,null);
 if(stalled.length>0){host.enqueue(stalled);await later("objects-missing",missing,host.pending());}
 else{retries.delete(worktree);await clearHealth(host,["objects-missing","index-busy","index-lock-stale","git-failed","backup-failed","waiting-for-base","unmerged","write-tree-failed"]);}
 if(ready.length>0||baseChange)host.say({event:"applied",stage:"git",count:ready.length+(baseChange?1:0)});
 return {applied:settled.length,queued:stalled.length};
}

type IndexFold=Readonly<{tree:string|null;missing?:readonly string[]}>;

/** 🌲 Reports missing tree objects when Git cannot read a base into an index. */
async function missingBaseTrees(worktree:string,base:string,signal?:AbortSignal):Promise<readonly string[]|undefined>{
 const listing=await git(worktree,["rev-list","--objects","--no-object-names","--missing=print","--filter=blob:none",base],{signal});
 if(listing?.code!==0)return;
 return listing.out.toString("utf8").split("\n").filter(line=>line.startsWith("?")).map(line=>line.slice(1)).filter(oid=>oidPattern.test(oid));
}

async function foldIndex(worktree:string,baseTree:string,entries:readonly GitChangeV1[],signal?:AbortSignal,diagnose=false):Promise<IndexFold>{
 const directory=join(worktree,".cache","f","tmp"),index=join(directory,`index-${token()}`);
 await mkdir(directory,{recursive:true});
 try{
  if(!await readTreeV1(worktree,baseTree,{index,signal}))return {tree:null,missing:diagnose?await missingBaseTrees(worktree,baseTree,signal):undefined};
  if(!await updateIndexV1(worktree,entries,{index,signal,hash:baseTree.length}))return {tree:null};
  const tree=await writeTreeV1(worktree,{index,signal});
  return {tree,missing:tree===null&&diagnose?await stageMissingV1(worktree,index,signal)??undefined:undefined};
 }finally{await rm(index,{force:true});await rm(`${index}.lock`,{force:true});}
}

/** 🧮 Folds the base tree and the entries newer than it in a temporary index (`GIT_INDEX_FILE`) and returns the resulting tree; `null` when a step fails or an object is missing. */
export async function foldIndexV1(worktree:string,baseTree:string,entries:readonly GitChangeV1[],signal?:AbortSignal):Promise<string|null>{
 return (await foldIndex(worktree,baseTree,entries,signal)).tree;
}

/** 🧮 Folds a base register and the entry registers into one tree: only entries with an hlc greater than the base's count, the rest are void; `null` when the base is no tree register or a git step fails. */
export async function foldRegistersV1(worktree:string,base:FleetManifestEntryV1,registers:Iterable<readonly [string,FleetManifestEntryV1]>,signal?:AbortSignal):Promise<string|null>{
 const baseTree=treeOf(base);
 if(baseTree===undefined)return null;
 const entries:GitChangeV1[]=[];
 for(const [path,entry] of registers){
  if(path===INDEX_BASE_V1||!path.startsWith(INDEX_PREFIX_V1)||entry.hlc<=base.hlc)continue;
  const change=changeOf(path,entry);
  if(change!==undefined)entries.push(change);
 }
 return foldIndexV1(worktree,baseTree,entries,signal);
}

/** ✅ Verifies the stage against the registers: the fold of base and newer entries must equal `write-tree` of the local stage; skipped without git, a repository or a base. */
export async function verifyIndexV1(host:IndexHostV1,signal?:AbortSignal):Promise<IndexVerifyV1>{
 const skipped:IndexVerifyV1={ok:true,skipped:true,expected:null,actual:null},base=host.base();
 if(base===undefined||treeOf(base)===undefined||!await gitPresentV1(host.worktree,signal))return skipped;
 const registers=registersOf(host,host.each);
 const expected=await foldRegistersV1(host.worktree,base,registers,signal),actual=await writeTreeV1(host.worktree,{signal});
 if(expected===null||actual===null)return {...skipped,skipped:false,ok:false,expected,actual};
 return {ok:expected===actual,skipped:false,expected,actual};
}

async function probeSymlinks(worktree:string):Promise<boolean>{
 const directory=join(worktree,".cache","f","tmp"),target=join(directory,`probe-${token()}`);
 await mkdir(directory,{recursive:true});
 try{await symlink("probe",target);return true;}catch{return false;}finally{await rm(target,{force:true});}
}

async function probeCase(worktree:string):Promise<boolean>{
 const directory=join(worktree,".cache","f","tmp"),name=`CaSe${token()}`,file=join(directory,name);
 await mkdir(directory,{recursive:true});
 try{await writeFile(file,"");return await stat(join(directory,name.toLowerCase())).then(()=>true,()=>false);}catch{return false;}finally{await rm(file,{force:true});}
}

/** 🛠️ Normalises `.git/config` through git itself for this machine: `core.autocrlf=false`, `core.safecrlf=false`, `core.eol=lf`, `core.protectNTFS=true`, `core.protectHFS=true`, `core.longpaths=true` on Windows, `core.filemode` (false on Windows), `core.symlinks` and `core.ignorecase` from probes of the worktree volume, `core.precomposeunicode=true` on macOS, `core.fsmonitor=false` (the file event service is the saturated component of a loaded host), `core.untrackedCache=true` and `index.version=4` (a smaller index and a cache of untracked directories for plain `git status`; `feature.manyFiles` is not used because it is a bundle that may grow other switches), `gc.auto=0`, `gc.autoPackLimit=0`, `gc.autoDetach=false`, `gc.writeCommitGraph=false`, `fetch.writeCommitGraph=false`, `receive.autogc=false`, `maintenance.auto=false` (no git client repacks or rewrites commit graphs on its own under the replica), `core.logallrefupdates=true`, removes a `commit.template` that points outside the worktree, and empties `.git/info/exclude` (its rules hide files from the git status of one machine only and so made its view differ from the others'; the previous text is kept under `.cache/f/lost/<ms>-info-exclude`). Remote, branch and user sections are untouched; only keys that differ are written. Returns the keys it changed; never throws. */
export async function normaliseGitConfigV1(worktree:string,platform:NodeJS.Platform=process.platform,signal?:AbortSignal):Promise<readonly string[]>{
 try{
  if(!await gitPresentV1(worktree,signal))return [];
  const listing=await git(worktree,["config","--local","--list","-z"],{signal});
  const fresh=listing!==null&&listing.code!==0&&!await stat(join(worktree,".git","config")).then(()=>true,()=>false);
  if(listing===null||listing.code!==0&&!fresh)return [];
  const current=new Map<string,string>();
  for(const item of fresh?[]:listing.out.toString("utf8").split("\0")){
   const cut=item.indexOf("\n");
   if(cut>0)current.set(item.slice(0,cut).toLowerCase(),item.slice(cut+1));
  }
  const wanted:[string,string][]=[["core.autocrlf","false"],["core.safecrlf","false"],["core.eol","lf"],["core.filemode",platform==="win32"?"false":"true"],["core.symlinks",String(await probeSymlinks(worktree))],["core.ignorecase",String(await probeCase(worktree))],["core.protectNTFS","true"],["core.protectHFS","true"],["core.fsmonitor","false"],["core.untrackedCache","true"],["index.version","4"],["gc.auto","0"],["gc.autoPackLimit","0"],["gc.autoDetach","false"],["gc.writeCommitGraph","false"],["fetch.writeCommitGraph","false"],["receive.autogc","false"],["maintenance.auto","false"],["core.logallrefupdates","true"]];
  if(platform==="win32")wanted.push(["core.longpaths","true"]);
  if(platform==="darwin")wanted.push(["core.precomposeunicode","true"]);
  const changed:string[]=[];
  for(const [key,value] of wanted){
   if(current.get(key.toLowerCase())===value)continue;
   if((await git(worktree,["config","--local",key,value],{signal}))?.code===0)changed.push(key);
  }
  const exclude=join(worktree,".git","info","exclude"),rules=await readFile(exclude,"utf8").catch(()=>"");
  if(rules.split("\n").some(line=>line.trim()!==""&&!line.trimStart().startsWith("#"))){
   const kept=join(worktree,".cache","f","lost",`${String(Date.now()).padStart(13,"0")}-info-exclude`),temporary=`${exclude}.${token()}.tmp`;
   try{
    await mkdir(dirname(kept),{recursive:true});
    await writeFile(kept,rules);
    await writeFile(temporary,"# semio fleet: rules of this file hide files from the git status of this machine only, so they are not used; the previous rules are under .cache/f/lost\n");
    await rename(temporary,exclude);
    changed.push("info/exclude");
   }catch{await rm(temporary,{force:true}).catch(()=>{});}
  }
  const template=current.get("commit.template");
  if(template!==undefined){
   const expanded=template.startsWith("~")?join(process.env.HOME??process.env.USERPROFILE??"",template.slice(1)):template,inside=!relative(resolve(worktree),resolve(worktree,expanded)).startsWith("..")&&!isAbsolute(relative(resolve(worktree),resolve(worktree,expanded)));
   if(!inside&&(await git(worktree,["config","--local","--unset-all","commit.template"],{signal}))?.code===0)changed.push("commit.template");
  }
  return changed;
 }catch{return [];}
}

/** 🧹 Removes the dead temporary files of git writers from `.git/objects/pack` (`tmp_*` and `.tmp-*`) whose modification and change time is older than `olderThanMilliseconds` (default one hour: a live `pack-objects` or `index-pack` writes continuously, so a file that was silent for an hour belongs to a process that died, like the 3.2 GB `tmp_pack_*` of the non-atomic repack of 2026-11-14); returns the files and bytes removed. The names are local only (`gitSharedV1`), nothing is published for them. Never throws. */
export async function sweepGitGarbageV1(worktree:string,options:Readonly<{olderThanMilliseconds?:number;now?:number}>={}):Promise<Readonly<{files:number;bytes:number}>>{
 const directory=join(worktree,".git","objects","pack"),limit=options.olderThanMilliseconds??3_600_000,now=options.now??Date.now();
 let files=0,bytes=0;
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  if(!name.startsWith("tmp_")&&!name.startsWith(".tmp-"))continue;
  const file=join(directory,name),info=await lstat(file).catch(()=>undefined);
  if(info===undefined||!info.isFile()||now-Math.max(info.mtimeMs,info.ctimeMs)<=limit)continue;
  await unlink(file).then(()=>{files++;bytes+=info.size;},()=>{});
 }
 return {files,bytes};
}

/** 🧭 Tells whether a replica path is a branch ref that concurrent commits can fork: a loose file below `.git/refs/heads/` or the `.git/HEAD` of a detached worktree; remotes, tags and `packed-refs` stay last writer wins. */
export const branchRefV1=(path:string):boolean=>path===".git/HEAD"||path.startsWith(".git/refs/heads/");

/** 🔱 What `linearizeRefV1` decided about an incoming commit against the local one: `retry` (an object is missing or git is busy; `waitedMs` is how long this exact pair has been waiting), `linear` (the local commit is an ancestor, a plain fast-forward), `ahead` (the incoming commit is an ancestor, keep the local one), `duplicate` (every local commit is already in the incoming history, the ref now names the incoming commit) or `rewritten` (the local commits were re-created in line on top of the incoming commit, the ref names `result`, and `conflicts` lists the paths both sides changed differently). */
export type RefLinearV1=Readonly<{state:"retry";waitedMs:number}|{state:"linear"}|{state:"ahead"}|{state:"duplicate";winner:string;loser:string}|{state:"rewritten";winner:string;loser:string;result:string;conflicts:readonly string[]}>;

const waits=new Map<string,number>();

type CommitV1=Readonly<{tree:string;parent:string|null;author:string;committer:string;message:Buffer}>;
type Stamp=Readonly<{name:string;email:string;date:string}>;

async function commitOf(worktree:string,oid:string,signal?:AbortSignal):Promise<CommitV1|null>{
 const outcome=await git(worktree,["cat-file","commit",oid],{signal});
 if(outcome===null||outcome.code!==0)return null;
 const cut=outcome.out.indexOf("\n\n"),head=outcome.out.subarray(0,cut<0?outcome.out.length:cut).toString("utf8").split("\n");
 const find=(key:string):string|undefined=>head.find(line=>line.startsWith(`${key} `))?.slice(key.length+1);
 const tree=find("tree"),author=find("author"),committer=find("committer");
 return tree===undefined||author===undefined||committer===undefined?null:{tree,parent:find("parent")??null,author,committer,message:cut<0?Buffer.alloc(0):outcome.out.subarray(cut+2)};
}

function stampOf(line:string):Stamp|null{
 const match=/^(.*) <([^>]*)> (\d+ [+-]\d{4})$/u.exec(line);
 return match===null?null:{name:match[1]!,email:match[2]!,date:match[3]!};
}

async function ancestor(worktree:string,older:string,newer:string,signal?:AbortSignal):Promise<boolean|null>{
 const outcome=await git(worktree,["merge-base","--is-ancestor",older,newer],{signal});
 return outcome===null||outcome.code>1?null:outcome.code===0;
}

const changeMap=async(worktree:string,from:string|null,to:string,signal?:AbortSignal):Promise<Map<string,GitChangeV1>|null>=>{
 const list=await diffTreeV1(worktree,from,to,signal);
 return list===null?null:new Map(list.map(change=>[change.path,change]));
};

const ancestorsOf=(path:string):string[]=>{const found:string[]=[];for(let cut=path.indexOf("/");cut>=0;cut=path.indexOf("/",cut+1))found.push(path.slice(0,cut));return found;};

/** 🧬 The per-path three-way merge of two trees that both descend from `base`, with plumbing only (`diff-tree`, a temporary index through `GIT_INDEX_FILE`, `update-index --index-info`, `write-tree`): the result starts as `ours`, every path `theirs` changed against `base` and `ours` did not change is taken from `theirs`, a path both changed identically stays, and a path both changed differently (also a file against a directory of the same name) is a conflict that takes `theirs` when `theirsWins` and keeps `ours` otherwise. The result never drops a change of one side on a path the other side left alone; there is no fallback to a whole tree. `null` when a git step fails. */
export async function mergeTreesV1(worktree:string,base:string|null,ours:string,theirs:string,theirsWins:boolean,signal?:AbortSignal):Promise<Readonly<{tree:string;conflicts:readonly string[]}>|null>{
 const mine=await changeMap(worktree,base,ours,signal),yours=await changeMap(worktree,base,theirs,signal);
 if(mine===null||yours===null)return null;
 const directories=new Set<string>();
 for(const path of mine.keys())for(const directory of ancestorsOf(path))directories.add(directory);
 const take:GitChangeV1[]=[],conflicts:string[]=[];
 for(const [path,change] of yours){
  const there=mine.get(path);
  if(there!==undefined&&there.mode===change.mode&&there.oid===change.oid)continue;
  if(there!==undefined||directories.has(path)||ancestorsOf(path).some(directory=>mine.has(directory))){conflicts.push(path);if(theirsWins)take.push(change);continue;}
  take.push(change);
 }
 if(take.length===0)return {tree:ours,conflicts};
 const directory=join(worktree,".cache","f","tmp"),index=join(directory,`index-${token()}`);
 await mkdir(directory,{recursive:true});
 try{
  if(!await readTreeV1(worktree,ours,{index,signal})||!await updateIndexV1(worktree,take,{index,signal,hash:ours.length}))return null;
  const tree=await writeTreeV1(worktree,{index,signal});
  return tree===null?null:{tree,conflicts};
 }finally{await rm(index,{force:true});await rm(`${index}.lock`,{force:true});}
}

const whenOf=(line:string):number=>Number(/ (\d+) [+-]\d{4}$/u.exec(line)?.[1]??0);

async function signedKeys(worktree:string,exclude:string,include:string,signal?:AbortSignal):Promise<ReadonlySet<string>|null>{
 const outcome=await git(worktree,["log","-z","-n","1000","--date=raw","--format=%an <%ae> %ad%n%B",`^${exclude}`,include],{signal});
 if(outcome===null||outcome.code!==0)return null;
 return new Set(outcome.out.toString("utf8").split("\0").filter(item=>item!=="").map(item=>item.trimEnd()));
}

/** 🔱 Linearises concurrent commits of one branch ref without touching the worktree or the stage: when the commit `local` (authored on this machine) and the commit `incoming` (the winning ref value) are not on one line, every local commit missing from the incoming history is re-created on top of `incoming` with `git commit-tree` (author, author date, committer and message kept, one parent, the tree is the per-path three-way merge `mergeTreesV1` of the original parent, the new tip and the original tree: the loser's change to every path the tip left alone, and on a path both changed differently the side whose committer time is later, a tie keeping the tip), and the ref moves to the result with `git update-ref` against `local`. A commit that adds nothing to the tip (merged tree equal) and either repeats the tip's message or repeats an author line and message already in the incoming history is dropped as a duplicate. Deterministic: the same inputs always give the same commits. */
export async function linearizeRefV1(worktree:string,ref:string,local:string,incoming:string,signal?:AbortSignal):Promise<RefLinearV1>{
 const key=`${worktree}\0${ref}\0${local}\0${incoming}`;
 if(waits.size>1000)waits.clear();
 const since=waits.get(key)??Date.now();
 waits.set(key,since);
 const retry:RefLinearV1={state:"retry",waitedMs:Date.now()-since};
 try{
  const done=(result:RefLinearV1):RefLinearV1=>{waits.delete(key);return result;};
  const missing=await missingObjectsV1(worktree,[local,incoming],signal);
  if(missing===null||missing.size>0)return retry;
  const forward=await ancestor(worktree,local,incoming,signal);
  if(forward===null)return retry;
  if(forward)return done({state:"linear"});
  const back=await ancestor(worktree,incoming,local,signal);
  if(back===null)return retry;
  if(back)return done({state:"ahead"});
  const listing=text(await git(worktree,["rev-list","--reverse","--topo-order",`^${incoming}`,local],{signal}));
  if(listing===null)return retry;
  const chain=listing.split("\n").filter(line=>oidPattern.test(line)),seen=await signedKeys(worktree,local,incoming,signal),tipCommit=await commitOf(worktree,incoming,signal);
  if(seen===null||tipCommit===null||chain.length===0)return retry;
  let tip=incoming,tipTree=tipCommit.tree,tipMessage=tipCommit.message.toString("utf8").trimEnd(),tipWhen=whenOf(tipCommit.committer);
  const conflicts:string[]=[];
  for(const oid of chain){
   const commit=await commitOf(worktree,oid,signal),stamp=commit===null?null:stampOf(commit.author),who=commit===null?null:stampOf(commit.committer);
   if(commit===null||stamp===null||who===null)return retry;
   const message=commit.message.toString("utf8").trimEnd();
   const base=commit.parent===null?null:text(await git(worktree,["rev-parse",`${commit.parent}^{tree}`],{signal}));
   if(commit.parent!==null&&base===null)return retry;
   const when=whenOf(commit.committer),merged=await mergeTreesV1(worktree,base,tipTree,commit.tree,when>tipWhen,signal);
   if(merged===null)return retry;
   const tree=merged.tree;
   if(seen.has(`${commit.author}\n${message}`)||tree===tipTree&&message===tipMessage)continue;
   const made=text(await git(worktree,["-c","commit.gpgsign=false","commit-tree",tree,"-p",tip,"-F","-"],{signal,stdin:commit.message,env:{GIT_AUTHOR_NAME:stamp.name,GIT_AUTHOR_EMAIL:stamp.email,GIT_AUTHOR_DATE:stamp.date,GIT_COMMITTER_NAME:who.name,GIT_COMMITTER_EMAIL:who.email,GIT_COMMITTER_DATE:who.date}}));
   if(made===null||!oidPattern.test(made))return retry;
   tip=made;tipTree=tree;tipMessage=message;tipWhen=Math.max(tipWhen,when);conflicts.push(...merged.conflicts);
  }
  const target=ref==="HEAD"?["--no-deref","HEAD"]:[ref];
  const moved=await git(worktree,["update-ref","-m","linearise concurrent commit",...target,tip,local],{signal,env:{GIT_COMMITTER_NAME:"semio fleet",GIT_COMMITTER_EMAIL:"fleet@semio.invalid"}});
  if(moved===null||moved.code!==0)return retry;
  return done(tip===incoming?{state:"duplicate",winner:incoming,loser:local}:{state:"rewritten",winner:incoming,loser:local,result:tip,conflicts:[...new Set(conflicts)]});
 }catch{return retry;}
}

/** 🩹 Why a repository that the fleet holds cannot be read on this machine: `head-unreadable` (no `.git/HEAD` or an empty one), `config-unreadable` (git fails to parse `.git/config`; an absent file is fine), `head-unresolvable` (the branch `HEAD` names has no value), `tip-missing` (the commit `HEAD` resolves to is not in the object store: a pack or object file is absent) and `index-missing` (no `.git/index`). */
export type GitBrokenV1="head-unreadable"|"config-unreadable"|"head-unresolvable"|"tip-missing"|"index-missing";

/** 🚑 Looks at the root `.git` of a worktree with plumbing only and names the first thing that keeps `HEAD` from resolving to a readable commit; `null` when the repository is sound or there is no root `.git` directory at all. Never throws, writes nothing and does not use the cached verdicts of `gitStateV1`. */
export async function gitBrokenV1(worktree:string,signal?:AbortSignal,metadata:<T>(read:()=>Promise<T>)=>Promise<T>=read=>read()):Promise<GitBrokenV1|null>{
 try{
  const root=await metadata(()=>lstat(join(worktree,".git")).catch(()=>undefined));
  if(root===undefined||!root.isDirectory())return null;
  const head=await readFile(join(worktree,".git","HEAD"),"utf8").catch(()=>"");
  if(head.trim()==="")return "head-unreadable";
  const config=await git(worktree,["config","--local","--get","core.bare"],{signal});
  if(config===null||config.code>1)return "config-unreadable";
  const tip=text(await git(worktree,["rev-parse","-q","--verify","HEAD"],{signal}));
  if(tip===null||!oidPattern.test(tip))return "head-unresolvable";
  if((await git(worktree,["cat-file","-e",`${tip}^{commit}`],{signal}))?.code!==0)return "tip-missing";
  if(await metadata(()=>lstat(join(worktree,".git","index")).then(()=>false,()=>true)))return "index-missing";
  return null;
 }catch{return null;}
}

/** 🔧 Repairs a `.git/config` that git cannot parse. The file is machine-local (`gitSharedV1` does not list it), so the store holds no copy to heal it from: the broken file is kept in `.cache/f/lost/<ms>-git-config/config`, a minimal valid one (`repositoryformatversion`, `bare`) takes its place and `normaliseGitConfigV1` pins the keys the fleet needs. Only a parse failure is repaired (`bad config`, `bad boolean`, `unable to read config`); a git that cannot start, a dubious-ownership refusal and a sound config are left alone. Returns whether the file was replaced. */
export async function repairGitConfigV1(worktree:string,now:number,signal?:AbortSignal):Promise<boolean>{
 try{
  const probe=await git(worktree,["config","--local","--get","core.bare"],{signal});
  if(probe===null||probe.code<=1||/dubious ownership|safe\.directory/iu.test(probe.err)||!/bad config|bad boolean|bad numeric|unable to read config|invalid config|key does not contain/iu.test(probe.err))return false;
  const file=join(worktree,".git","config"),lost=join(worktree,".cache","f","lost",`${String(now).padStart(13,"0")}-git-config`),temporary=`${file}.${token()}.tmp`;
  await mkdir(lost,{recursive:true});
  await copyFile(file,join(lost,"config")).catch(()=>{});
  await writeFile(temporary,"[core]\n\trepositoryformatversion = 0\n\tbare = false\n");
  await rename(temporary,file);
  await normaliseGitConfigV1(worktree,undefined,signal);
  return true;
 }catch{return false;}
}

/** 🪢 Removes a `.git/shallow` that cuts history this machine holds in full. The file is machine-local (`gitSharedV1` does not list it): it states which commits the own object store lacks the parents of, so a copy from another machine, or a boundary whose objects arrived since, makes git treat a commit with parents as a root (`git log` ends there, a later history rewrite would drop the parents for good). Every listed commit is read raw (`cat-file commit` ignores the boundary), and when every commit, tree and blob behind all their parents is present the file is kept in `.cache/f/lost/<ms>-git-shallow/shallow` and removed. A boundary whose history is really absent, an unreadable entry and a git that cannot start leave the file alone. Returns whether the file was removed. */
export async function repairGitShallowV1(worktree:string,now:number,signal?:AbortSignal):Promise<boolean>{
 try{
  const file=join(worktree,".git","shallow"),listed=await readFile(file,"utf8").catch(()=>undefined);
  if(listed===undefined)return false;
  const parents=new Set<string>();
  for(const oid of listed.split("\n").map(line=>line.trim()).filter(line=>line!=="")){
   const raw=oidPattern.test(oid)?await git(worktree,["cat-file","commit",oid],{signal}):null;
   if(raw===null||raw.code!==0)return false;
   for(const line of raw.out.toString("utf8").split("\n\n",1)[0]!.split("\n"))if(line.startsWith("parent "))parents.add(line.slice(7).trim());
  }
  if(parents.size>0&&(await git(worktree,["rev-list","--objects","--quiet",...parents],{signal}))?.code!==0)return false;
  const lost=join(worktree,".cache","f","lost",`${String(now).padStart(13,"0")}-git-shallow`);
  await mkdir(lost,{recursive:true});
  await copyFile(file,join(lost,"shallow"));
  await unlink(file);
  return true;
 }catch{return false;}
}

/** ♻️ Forgets every cached verdict, signature and retry timer of a worktree (for tests and after a `.git` replacement). */
export function forgetGitV1(worktree:string):void{
 stamps.delete(worktree);verdicts.delete(worktree);volumes.delete(worktree);versions=new Map();caseIgnored.delete(worktree);pinned.delete(worktree);trees.delete(worktree);signatures.delete(worktree);memory.delete(worktree);records.delete(worktree);retries.delete(worktree);for(const key of waits.keys())if(key.startsWith(`${worktree}\0`))waits.delete(key);
}

