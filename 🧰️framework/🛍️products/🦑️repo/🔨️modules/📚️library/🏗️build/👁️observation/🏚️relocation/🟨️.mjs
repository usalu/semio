const table=new Map(),lives=new Set(),blocked=new Set();
const FOLLOW="(?=[/\"\\\\\\s,\\]\\}:]|$)",PRECEDE="(?<![\\p{L}\\p{N}._@+\\-/])";
const escape=text=>text.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&");
const absolute=text=>typeof text==="string"&&text.length>1&&(text.startsWith("/")||/^[A-Za-z]:[\\/]/u.test(text));
const nested=(left,right)=>/[\\/]/u.test(left.charAt(right.length))&&left.startsWith(right)||/[\\/]/u.test(right.charAt(left.length))&&right.startsWith(left);
const escaped=text=>JSON.stringify(text).slice(1,-1);
const patternOf=text=>new RegExp(PRECEDE+escape(text)+FOLLOW,"gu");
const block=former=>{table.delete(former);blocked.add(former);};

/** 🏚️ Teaches this process that facts recorded under `former` are about the tree now at `live`: a workspace that moved (or a generation made on a machine whose checkout lives elsewhere) keeps every recorded fact checkable by content. The mapping is refused, and any earlier one for the same root dropped for good, whenever the text cannot tell the trees apart: a root that is itself the live root of a workspace this process serves, a root taught with two different live roots, a root that contains or sits inside another former or live root, and a chain (a former root that is another one's live root). A refused root only leaves its rows to be checked as stale, never wrongly fresh.
 * @see ../🟨️.mjs `observationsHold` */
export function noteFormerRootV1(former,liveRoot){
 if(!absolute(former)||!absolute(liveRoot)||former===liveRoot)return false;
 noteLiveRootV1(liveRoot);
 if(blocked.has(former))return false;
 if(lives.has(former)||nested(former,liveRoot)){block(former);return false;}
 for(const other of[...table.keys()])if(other!==former&&nested(other,former)){block(other);block(former);return false;}
 for(const known of lives)if(known!==liveRoot&&nested(former,known)){block(former);return false;}
 const known=table.get(former);
 if(known!==undefined){if(known.live===liveRoot)return false;block(former);return false;}
 table.set(former,{live:liveRoot,pattern:patternOf(former),jsonPattern:escaped(former)===former?undefined:patternOf(escaped(former)),jsonLive:escaped(liveRoot),jsonFormer:escaped(former)});return true;
}

/** 🏡️ Registers the live root of a workspace this process serves: no mapping may ever rewrite rows that sit under it, so a mapping from that root is dropped and refused. */
export function noteLiveRootV1(root){
 if(!absolute(root))return;
 lives.add(root);
 if(table.has(root))block(root);
}

/** 🧹️ Forgets every taught root (a law starts from none). */
export const forgetFormerRootsV1=()=>{table.clear();lives.clear();blocked.clear();};

/** 🗺️ The roots taught so far as `[former, live]` pairs. */
export const formerRootsV1=()=>[...table].map(([former,{live}])=>[former,live]);

/** ✏️ The text with every taught former root replaced by the live one; the text itself when none occurs. */
export function rebaseTextV1(text){
 if(table.size===0||typeof text!=="string"||text.length<2)return text;
 let out=text;
 for(const[former,entry]of table){
  if(out.includes(former))out=out.replace(entry.pattern,()=>entry.live);
  if(entry.jsonPattern!==undefined&&out.includes(entry.jsonFormer))out=out.replace(entry.jsonPattern,()=>entry.jsonLive);
 }
 return out;
}

/** 🔁️ The value with the former roots replaced in every string, copying only what changed. */
export function rebaseValueV1(value){
 if(table.size===0||value===null||typeof value!=="object")return typeof value==="string"?rebaseTextV1(value):value;
 if(Array.isArray(value)){let out;for(let at=0;at<value.length;at++){const next=rebaseValueV1(value[at]);if(next!==value[at]){out??=value.slice();out[at]=next;}}return out??value;}
 let out;for(const key of Object.keys(value)){const next=rebaseValueV1(value[key]);if(next!==value[key]){out??={...value};out[key]=next;}}
 return out??value;
}

/** 🧾️ One recorded row as it reads in the live tree; the row itself when nothing in it names a former root. */
export function rebaseRowV1(row){
 if(table.size===0||row===null||typeof row!=="object")return row;
 const path=rebaseTextV1(row.path),fact=rebaseValueV1(row.fact);
 return path===row.path&&fact===row.fact?row:{...row,path,fact};
}

/** 📚️ The rows as they read in the live tree, the same array when none changed. */
export function rebaseRowsV1(rows){
 if(table.size===0)return rows;
 let out;for(let at=0;at<rows.length;at++){const next=rebaseRowV1(rows[at]);if(next!==rows[at]){out??=Array.from(rows);out[at]=next;}}
 return out??rows;
}

const FIRST=/^\["((?:[^"\\]|\\.)*)"/u,WALK=/"workspaceRoot":"((?:[^"\\]|\\.)*)"/u;
/** 🔎️ The workspace roots a set of recorded rows names (the first element of a closure or registry question, the `workspaceRoot` of a derived walk): the roots the facts were recorded under. */
export function recordedRootsV1(rows){
 const found=new Set();
 for(let at=0;at<rows.length;at++){
  const row=rows[at],kind=row.kind;
  if(kind==="closure"||kind==="registry"){const hit=FIRST.exec(row.path);if(hit)found.add(hit[1]);}
  else if(kind==="derived"){const hit=WALK.exec(row.path.slice(0,400));if(hit)found.add(hit[1]);}
 }
 const roots=[];for(const text of found){try{const root=JSON.parse('"'+text+'"');if(absolute(root))roots.push(root);}catch{}}
 return roots.sort();
}

const KINDS=new Set(["closure","registry","derived"]);
/** 🔎️ The same roots read from a lazily opened fact index by kind and path id, so only the few rows that name a root are decoded. */
export function recordedRootsOfIndexV1(index){
 if(typeof index.rowKindOf!=="function"||typeof index.rowPathIdOf!=="function"||typeof index.textOf!=="function")return recordedRootsV1(index.rows);
 const seen=new Set(),rows=[];
 for(let id=0;id<index.rows.length;id++){
  if(!KINDS.has(index.rowKindOf(id)))continue;
  const at=index.rowPathIdOf(id);if(seen.has(at))continue;seen.add(at);
  rows.push({kind:index.rowKindOf(id),path:index.textOf(at)});
 }
 return recordedRootsV1(rows);
}

/** 🏡️ Teaches the process every root a fact index was recorded under that is not `live`; whether any root was new. */
export function adoptRecordedRootsV1(index,live){
 if(typeof live!=="string")return false;
 noteLiveRootV1(live);
 let added=false;for(const root of recordedRootsOfIndexV1(index))added=noteFormerRootV1(root,live)||added;return added;
}
