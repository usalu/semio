import{createHash}from"node:crypto";
import{join,relative,sep}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const P=schema.definitions.Protocol.const,hex=(value,length)=>typeof value==="string"&&new RegExp("^[a-f0-9]{"+length+"}$","u").test(value),count=value=>Number.isSafeInteger(value)&&value>=0,sha256=bytes=>createHash("sha256").update(bytes).digest("hex");
const referencePattern=/^(?:source|lock):[a-f0-9]{64}$/u,stampPattern=/^[0-9]+:[0-9]+:[0-9]+$/u,selectionPattern=/^inference\/(selections|contexts)\/([a-f0-9]{64})\/(.+)$/u,generationPattern=/^generations\/(.+)$/u,canonicalPattern=/^inference\/((?!(?:selections|contexts)\/).+)$/u;

/** 🗂️ The file name of one group's append-only member index, inside the group directory. */
export const MEMBER_INDEX_FILE_V1=P.file;
/** 🔢️ Hashes bytes the way every member row and every leaf name is hashed. */
export const memberHashV1=sha256;

/** 🪪️ Reduces a physical stamp to what survives remounts and ctime changes: inode, size and modification time. */
export function memberStampV1(row){return[row.ino,row.size,row.mtimeNs].join(":");}
/** 🪪️ Reduces the proof's seven-part row stamp (`dev:ino:mode:nlink:size:mtimeNs:ctimeNs`) to the member stamp. */
export function memberStampOfRowStampV1(value){const parts=value.split(":");return parts.length===7?[parts[1],parts[4],parts[5]].join(":"):"";}

function canonical(value){if(Array.isArray(value))return"["+value.map(canonical).join(",")+"]";if(value&&typeof value==="object")return"{"+Object.keys(value).sort().map(key=>JSON.stringify(key)+":"+canonical(value[key])).join(",")+"}";return JSON.stringify(value);}
/** 🔏️ Seals a row against torn, merged or hand-edited lines with a short digest of its canonical form. */
export function memberCheckV1(row){const{c:_,...bare}=row;return sha256(canonical(bare)).slice(0,P.checkHexDigits);}

/** 🧭️ Splits an image-relative leaf path into the group that keeps its index and the member path inside it: `selections/<id>` and `contexts/<id>` hold the isolated optional namespaces, `canonical` the optional and publication leaves of the shared namespace, `generations` the content-addressed generation documents; the mutable generation pointer and everything else has no index. */
export function memberGroupOfPathV1(name){
 const selection=selectionPattern.exec(name);if(selection)return Object.freeze({group:selection[1]+"/"+selection[2],directory:"inference/"+selection[1]+"/"+selection[2],member:selection[3]});
 const generation=generationPattern.exec(name);if(generation)return generation[1]==="current.json"?undefined:Object.freeze({group:"generations",directory:"generations",member:generation[1]});
 const canonical=canonicalPattern.exec(name);return canonical?Object.freeze({group:"canonical",directory:"inference",member:canonical[1]}):undefined;
}

/** 🏷️ Classifies an image-relative leaf path exactly like the closure proof's role table; anything else is not indexable. */
export function memberKindOfV1(name){
 if(/^(?:inference\/(?:(?:selections|contexts)\/[a-f0-9]{64}\/)?|generations\/)members\.jsonl$/u.test(name))return"member-index";
 if(name==="generations/current.json")return"generation-pointer";
 if(/^generations\/m\/[a-f0-9]{64}\.json$/u.test(name))return"generation-manifest";
 if(/^generations\/o\/[a-f0-9]{64}\.json\.br$/u.test(name))return"generation-fragment";
 if(/^generations\/x\/[a-f0-9]{64}\.facts\.br$/u.test(name))return"generation-index";
 if(/^generations\/f\/[a-f0-9]{64}\.json\.br$/u.test(name))return"generation-frontier";
 if(!name.startsWith("inference/"))return undefined;
 if(/\/ip\/[a-f0-9]{24}\.json$/u.test(name))return"precision";
 if(/\/rr\/[a-f0-9]{24}\.json$/u.test(name))return"cargo-receipt";
 if(/\/np\/[a-f0-9]{24}\.json$/u.test(name)||/\/(?:emoji-project-json|test-cases)\.json$/u.test(name))return"inference";
 if(/\/ob\/[a-f0-9]{64}\.br$/u.test(name))return"observation";
 if(/\/[0-9a-z]+-[0-9]+-[a-f0-9]{16}\.pack$/u.test(name))return"pack";
 if(/\/[0-9a-z]+-[0-9]+-[a-f0-9]{16}\.idx$/u.test(name))return"index";
 if(/\/(?:emoji-project-json|test-cases)\.receipt\.json$/u.test(name))return"receipt";
 if(/\/(?:emoji-project-json|test-cases)\.publication\.json$/u.test(name))return"publication";
 return undefined;
}

const safeMember=value=>typeof value==="string"&&value.length>0&&value.length<=P.maximumNameUnits&&!value.startsWith("/")&&!/[\\:\u0000-\u001f]/u.test(value)&&value.split("/").every(part=>part&&part!=="."&&part!=="..");
/** ✅️ Admits one closed, sealed index row; a light row names a leaf by its hash, a full row also carries the references and links the leaf contributes. */
export function readMemberRowV1(value){
 if(!value||typeof value!=="object"||Array.isArray(value))return undefined;
 const keys=Object.keys(value).sort().join(),light="b,c,h,k,n,s",full="b,c,g,h,k,l,n,r,s,x";
 if(keys!==light&&keys!==full)return undefined;
 if(!safeMember(value.n)||!count(value.b)||value.b>P.maximumFileBytes||!P.kinds.includes(value.k)||typeof value.s!=="string"||!stampPattern.test(value.s)||!hex(value.h,64)||!hex(value.c,P.checkHexDigits)||value.c!==memberCheckV1(value))return undefined;
 if(keys===full){
  if(!count(value.x)||!(value.g===null||hex(value.g,64))||!Array.isArray(value.r)||value.r.length>P.maximumReferencesPerRow||value.r.some(reference=>typeof reference!=="string"||!referencePattern.test(reference))||!Array.isArray(value.l)||value.l.length>P.maximumLinksPerRow||value.l.some(link=>!link||typeof link!=="object"||Object.keys(link).sort().join()!==(link.m===undefined?"p":"m,p")||!safeMember(link.p)||link.m!==undefined&&link.m!=="block"))return undefined;
 }
 return Object.freeze({...value});
}
/** 🧱️ Builds a sealed light row from facts measured at write time. */
export function lightMemberRowV1(facts){const row={n:facts.n,b:facts.b,h:facts.h,k:facts.k,s:facts.s};return Object.freeze({...row,c:memberCheckV1(row)});}
/** 🧱️ Builds a sealed full row once a leaf's references and links were read. */
export function fullMemberRowV1(facts){const row={n:facts.n,b:facts.b,h:facts.h,k:facts.k,s:facts.s,x:facts.x,g:facts.g??null,r:[...facts.r].sort(),l:facts.l.map(link=>link.m===undefined?{p:link.p}:{m:link.m,p:link.p})};return Object.freeze({...row,c:memberCheckV1(row)});}
/** 📝️ Encodes the one header line of an index. */
export function encodeMemberHeaderV1(group){return JSON.stringify({group,semio:P.header,version:1});}
/** 📝️ Encodes one row as a line without its newline. */
export function encodeMemberRowV1(row){return JSON.stringify(row);}

/** 📖️ Reads an index: rows are checked one by one, the last row of a name wins, a torn tail or a damaged line is counted and ignored, a foreign group invalidates the whole file. */
export function parseMemberIndexV1(text,group){
 const lines=text.split("\n"),tail=lines.pop()??"",rows=new Map();let rejected=0,header=false;
 if(!lines.length||lines[0]==="")return Object.freeze({valid:false,rows,rejected:0,tornTail:tail!=="",count:0});
 try{const value=JSON.parse(lines[0]);header=!!value&&Object.keys(value).sort().join()==="group,semio,version"&&value.semio===P.header&&value.version===1&&value.group===group;}catch{header=false;}
 if(!header)return Object.freeze({valid:false,rows,rejected:0,tornTail:tail!=="",count:0});
 for(let index=1;index<lines.length;index++){if(lines[index]==="")continue;let row;try{row=readMemberRowV1(JSON.parse(lines[index]));}catch{row=undefined;}if(!row){rejected++;continue;}rows.set(row.n,row);}
 return Object.freeze({valid:true,rows,rejected,tornTail:tail!=="",count:lines.length-1});
}

/** 🔎️ Finds the row that vouches for a physical leaf: the same kind, size and stamp; anything else leaves the leaf unproven. */
export function lookupMemberV1(index,name,kind,bytes,stamp){const row=index.rows.get(name);return row&&row.k===kind&&row.b===bytes&&row.s===stamp?row:undefined;}

/** ⚖️ Proves a group from its index alone: each physical leaf is full, light, stale (a row exists but the leaf changed) or unindexed; no leaf byte is read. The index file itself is not a member. */
export function proveGroupV1(index,physical){
 const members=[],references=new Set(),links=[];let full=0,light=0,stale=0,unindexed=0;
 for(const leaf of physical){
  if(leaf.kind==="member-index")continue;
  const row=index.valid?lookupMemberV1(index,leaf.name,leaf.kind,leaf.bytes,leaf.stamp):undefined;
  if(!row){const known=index.valid&&index.rows.has(leaf.name);members.push({name:leaf.name,state:known?"stale":"unindexed"});if(known)stale++;else unindexed++;continue;}
  if(row.r===undefined){light++;members.push({name:leaf.name,state:"light",hash:row.h});continue;}
  full++;for(const reference of row.r)references.add(reference);for(const link of row.l)links.push(link);members.push({name:leaf.name,state:"full",hash:row.h});
 }
 return Object.freeze({members,full,light,stale,unindexed,complete:stale===0&&unindexed===0,referencesComplete:stale===0&&unindexed===0&&light===0,references:[...references].sort(),links});
}

/** 🧹️ Rewrites an index down to the rows of leaves that still exist with the same stamp, newest first-wins collapsed, in name order. */
export function compactMemberIndexV1(index,group,live){
 const rows=[...index.rows.values()].filter(row=>live.get(row.n)===row.s).sort((left,right)=>left.n<right.n?-1:left.n>right.n?1:0);
 return[encodeMemberHeaderV1(group),...rows.map(encodeMemberRowV1)].join("\n")+"\n";
}

/** ➕️ Appends rows to a group's index with one O_APPEND write: the header when the file is new, a repairing newline when a crash left a torn line, never a rewrite. */
export function appendMemberRowsV1(fs,path,group,rows){
 const fd=fs.openSync(path,"a+",0o644);let failed=false;
 try{
  const size=Number(fs.fstatSync(fd).size);let prefix="";
  if(size===0)prefix=encodeMemberHeaderV1(group)+"\n";else{const last=Buffer.alloc(1);fs.readSync(fd,last,0,1,size-1);if(last[0]!==10)prefix="\n";}
  const text=prefix+rows.map(encodeMemberRowV1).join("\n")+"\n";if(Buffer.byteLength(text)>P.maximumAppendBytes)throw Error("Bounded member index append required");
  fs.writeSync(fd,text);
 }catch(error){failed=true;throw error;}finally{try{fs.closeSync(fd);}catch(error){if(!failed)throw error;}}
}

/** ✍️ Records one freshly published leaf at write time: measures its stamp, hashes its bytes once and appends a light row to its group's index; a leaf outside a group, of unknown kind, changed meanwhile or too large is left for the collector. */
export function recordMemberLeafV1(fs,storeRoot,path){
 const name=relative(storeRoot,path).split(sep).join("/"),group=memberGroupOfPathV1(name),kind=memberKindOfV1(name);
 if(!group||!kind||kind==="member-index"||kind==="generation-pointer")return false;
 const before=fs.lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size>BigInt(P.maximumFileBytes))return false;
 const bytes=fs.readFileSync(path),after=fs.lstatSync(path,{bigint:true});
 if(BigInt(bytes.length)!==before.size||memberStampV1(after)!==memberStampV1(before))return false;
 appendMemberRowsV1(fs,join(storeRoot,...group.directory.split("/"),MEMBER_INDEX_FILE_V1),group.group,[lightMemberRowV1({n:group.member,b:bytes.length,h:sha256(bytes),k:kind,s:memberStampV1(before)})]);
 return true;
}

/** 🚧️ Keeps a group the proof must leave alone: one touched within the protected idle time is in use. */
export function memberGroupProtectedV1(newestModifiedMs,nowMs,protectedMs){return nowMs-newestModifiedMs<protectedMs;}
