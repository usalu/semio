import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {brotliCompressSync,brotliDecompressSync,constants as zlib} from "node:zlib";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🧠️ The authored limits of the remembered path verdicts.
 * @see ../🧬️schema/🔣️.json */
export const GRAPH_VERDICT_POLICY_V1=schema.definitions.VerdictPolicy.const;
const policy=GRAPH_VERDICT_POLICY_V1;

/** 🔑️ The key a remembered verdict stays valid under: the text of the ignore rules and the digest of the code that judges a path; a rule edit or a code edit gives another key and with it an empty memory. */
export function verdictKeyV1(parts:Readonly<{ignoreText:string;code:string}>):string{
 return createHash("sha256").update(JSON.stringify([policy.header,parts.ignoreText,parts.code])).digest("hex").slice(0,policy.keyLength);
}

/** 🧾️ The digest of the files whose code decides a verdict; undefined where one cannot be read, so a memory that cannot be tied to its code is not kept. */
export function verdictCodeDigestV1(files:readonly string[]):string|undefined{
 const hash=createHash("sha256");
 try{for(const file of files){hash.update(file.slice(file.lastIndexOf("/")+1)+"\n");hash.update(readFileSync(file));}}catch{return undefined;}
 return hash.digest("hex");
}

/** 🧠️ The remembered verdicts of one key on disk: `get` answers a path judged before (by this or any earlier process), `set` records a new judgement, `save` writes the memory atomically when enough is new. A damaged, foreign or oversized file is an empty memory. */
export function createVerdictMemoV1(options:Readonly<{directory:string;key:string|undefined}>){
 const{directory,key}=options,file=key===undefined?undefined:join(directory,key+policy.extension),known=new Map<string,boolean>();let fresh=0,loaded=false;
 const load=()=>{
  loaded=true;
  if(file===undefined)return;
  try{
   const bytes=readFileSync(file);if(bytes.length>policy.maximumBytes)return;
   const text=brotliDecompressSync(bytes,{maxOutputLength:policy.maximumBytes}).toString("utf8"),lines=text.split("\n");
   if(lines[0]!==policy.header||lines[1]!=="key\t"+key||lines.length>policy.maximumEntries+3)return;
   for(let at=2;at<lines.length;at++){const line=lines[at]!;if(line==="")continue;const mark=line[0];if(mark!=="0"&&mark!=="1")continue;known.set(line.slice(1),mark==="1");}
  }catch{known.clear();}
 };
 return Object.freeze({
  get(path:string):boolean|undefined{if(!loaded)load();return known.get(path);},
  set(path:string,verdict:boolean):void{
   if(file===undefined)return;
   if(!loaded)load();
   if(known.size>=policy.maximumEntries||known.has(path))return;
   known.set(path,verdict);fresh++;
  },
  size:()=>known.size,
  pending:()=>fresh,
  save():void{
   if(file===undefined||fresh===0||(fresh<policy.saveMinimum&&statOf(file)))return;
   try{
    mkdirSync(directory,{recursive:true});
    const lines=[policy.header,"key\t"+key];for(const[path,verdict]of known)lines.push((verdict?"1":"0")+path);
    const text=Buffer.from(lines.join("\n")+"\n","utf8"),temporary=file+"."+randomBytes(6).toString("hex")+".tmp";
    try{writeFileSync(temporary,brotliCompressSync(text,{params:{[zlib.BROTLI_PARAM_QUALITY]:policy.brotliQuality,[zlib.BROTLI_PARAM_LGWIN]:policy.brotliWindow,[zlib.BROTLI_PARAM_SIZE_HINT]:text.length}}));renameSync(temporary,file);}finally{rmSync(temporary,{force:true});}
    fresh=0;prune(directory,file);
   }catch{}
  },
 });
}
const statOf=(file:string):boolean=>{try{return statSync(file).isFile();}catch{return false;}};
const prune=(directory:string,keep:string):void=>{
 try{
  const rows=readdirSync(directory).filter(name=>name.endsWith(policy.extension)).map(name=>({path:join(directory,name),at:statSync(join(directory,name)).mtimeMs})).sort((a,b)=>b.at-a.at);
  for(const row of rows.slice(policy.kept))if(row.path!==keep)rmSync(row.path,{force:true});
 }catch{}
};
