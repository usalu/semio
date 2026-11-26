import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import cachePolicy from"../../../⚡️caching/🔣️policy.json"with{type:"json"};

const rules=schema.$defs.rules.const,generated=new Set(cachePolicy[rules.generatedDirectories.field]),noise=new Set(rules.machineLocalFileNames),suffixes=rules.machineLocalFileSuffixes,local=new Set(rules.machineLocalDirectoryNames),pathKinds=new Set(rules.machineLocalPathKinds),directoryKinds=new Set(rules.machineLocalDirectoryKinds);

/** 🧿️ The entries of a directory listing that belong to the source tree as the build sees it: operating-system noise never, a generated directory (the cache policy list) or a machine-local one (`machineLocalDirectoryNames`: git ignores it, a generator fills it on the machines that ran it) never where `isDirectory` says the entry is a directory or a link, a hollow directory (`isHollow`: nothing a repository can hold lies below it, so it is what a removed file leaves behind on one replica and never on another) never, everything else always. https://git-scm.com/docs/gitignore
 * @template T
 * @param {readonly T[]} entries
 * @param {(entry:T)=>string} nameOf
 * @param {(entry:T)=>boolean} isDirectory
 * @param {(entry:T)=>boolean} [isHollow]
 * @returns {T[]}
 * @see ./🧪️tests/🕳️hollow/🟦️.ts */
export function sourceTreeFilterV1(entries,nameOf,isDirectory,isHollow){
 return entries.filter(entry=>{const name=nameOf(entry);return!noise.has(name)&&!suffixes.some(suffix=>name.endsWith(suffix))&&!((generated.has(name)||local.has(name))&&isDirectory(entry))&&!(isHollow!==undefined&&isHollow(entry));});
}

/** 🕳️ Whether a directory holds no source file anywhere below it: every entry it lists is noise, generated output or a directory that is hollow itself (a link, a file and any entry that cannot be read count as source, so a doubt keeps the entry). `entriesOf(path)` lists a directory with its types; `depth` bounds the walk (a deeper tree is not hollow).
 * @see ./🧪️tests/🕳️hollow/🟦️.ts */
export function hollowDirectoryV1(path,entriesOf,join,depth=32){
 if(depth<=0)return false;
 let entries;try{entries=entriesOf(path);}catch{return false;}
 for(const entry of entries){
  const name=String(entry.name),isDirectory=entry.isDirectory()||entry.isSymbolicLink();
  if(sourceTreeFilterV1([entry],()=>name,()=>isDirectory).length===0)continue;
  if(!entry.isDirectory()||!hollowDirectoryV1(join(path,name),entriesOf,join,depth-1))return false;
 }
 return true;
}

/** 🧠️ The verdicts of {@link isMachineLocalPathV1} by path, per reading (file or directory), each cleared whole at this many entries: the verification of a stored record asks the same few thousand paths for every row of every owner (600 owners of a slice, millions of calls), and a verdict depends on the path text alone. */
const MACHINE_LOCAL_MEMO_ROWS=65536,machineLocalFileMemo=new Map(),machineLocalDirectoryMemo=new Map();

/** 🤖️ Whether a path lies below a machine-local directory (`machineLocalDirectoryNames`: git ignores it, a generator fills it on the machines that ran it), so that whatever the filesystem says about it names the machine and not the project; any directory component counts, in either spelling of the separator. The last component counts only for `directory` (the path is read as a directory: a file of that name is source, as in a listing).
 * @see ./🧪️tests/🤖️machine-local-paths/🟦️.ts */
export function isMachineLocalPathV1(path,directory=false){
 if(typeof path!=="string")return false;
 const memo=directory?machineLocalDirectoryMemo:machineLocalFileMemo,known=memo.get(path);
 if(known!==undefined)return known;
 const parts=path.split(/[\\/]/u);
 let found=false;
 for(let at=0;at<parts.length-(directory?0:1)&&!found;at++)found=local.has(parts[at]);
 if(memo.size>=MACHINE_LOCAL_MEMO_ROWS)memo.clear();
 memo.set(path,found);
 return found;
}

/** 🤖️ Whether an observation row (its kind and its path) is a fact of the filesystem about a machine-local path: the one predicate that recording and judging share for every kind (`machineLocalPathKinds`: `exists`, `stat`, `file`, `field`, `analysis`, the type, real-path and TypeScript probes, the listings), so a record made on a machine that ran the generators and one made on a machine that did not carry the same rows and every machine holds them; the kinds that read a directory (`machineLocalDirectoryKinds`) count the directory itself too. */
export function machineLocalRowV1(kind,path){return pathKinds.has(kind)&&isMachineLocalPathV1(path,directoryKinds.has(kind));}
