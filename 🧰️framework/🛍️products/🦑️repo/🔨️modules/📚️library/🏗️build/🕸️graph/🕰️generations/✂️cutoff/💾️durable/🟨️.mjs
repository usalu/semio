import{createHash}from"node:crypto";
import{brotliCompressSync,brotliDecompressSync}from"node:zlib";
import{join}from"node:path";
import{canonicalJsonV1}from"../../🧮️fact-codec/🟨️.mjs";

/** 📏️ The finite limits of the durable answers: the directory below the store root, the version of a record and the largest expanded record. */
export const DURABLE_ANSWERS_V1=Object.freeze({directory:"a",version:1,maximumBytes:67108864,retainMilliseconds:604800000});

const digestOf=question=>createHash("sha256").update(question).digest("hex");

/** 🧠️ The bounded in-process state of the durable answers: the keys known to be recorded, the last few decoded answers and one table of rows shared by all answers, so the thousands of rows a closure reads (almost all the same files) exist once however many closures were verified. */
export function createDurableStateV1(){return{keys:new Set(),recent:new Map(),rows:new Map()};}

const RECENT=2;

/**
 * 💾️ The memo of the derived-fact cutoff made durable and shared: every answer is a content addressed record (`<sha256 of the question>.json.br`: question, answer and the rows it read), written atomically as soon as it is computed, so a plan that was killed or runs beside a peer resumes with the answers already found. A record is only a hint: the cutoff still checks that every row it read holds on the disk before it takes the answer, so a stale or foreign record costs one derivation and never a wrong result.
 * Memory stays bounded: decoded answers are not kept (only the last `RECENT`), and `intern` maps equal rows of different answers to one object. `get(question, true)` reads what a peer may have written meanwhile and `claim` makes one question single-flight across processes through the store lease: the holder derives, a peer waits and then finds the record instead of deriving the same closure again.
 * @see ../🟨️.mjs
 */
export function createDurableAnswersV1({root,fs,lease,resource,state=createDurableStateV1(),limits=DURABLE_ANSWERS_V1}){
 const path=question=>join(root,limits.directory,digestOf(question)+".json.br"),{keys,recent,rows:table}=state;
 const intern=list=>list.map(row=>{const key=row.kind+"\0"+row.path+"\0"+JSON.stringify(row.fact);let known=table.get(key);if(known===undefined)table.set(key,known=row);return known;});
 const remember=(question,answer)=>{keys.add(question);recent.delete(question);recent.set(question,answer);while(recent.size>RECENT)recent.delete(recent.keys().next().value);};
 const read=async question=>{
  try{
   const bytes=await fs?.read(path(question));if(!bytes)return undefined;
   const value=JSON.parse(brotliDecompressSync(bytes,{maxOutputLength:limits.maximumBytes}).toString("utf8"));
   return value?.version===limits.version&&value.question===question&&Array.isArray(value.rows)?{fact:value.fact,rows:intern(value.rows)}:undefined;
  }catch{return undefined;}
 };
 return Object.freeze({
  has:question=>keys.has(question),
  get size(){return keys.size;},
  intern,
  async get(question,shared=false){
   const known=(shared?undefined:recent.get(question))??await read(question);
   if(known)remember(question,known);
   return known;
  },
  async set(question,answer){
   const stored={fact:answer.fact,rows:intern(answer.rows)};remember(question,stored);
   try{const text=canonicalJsonV1({version:limits.version,question,fact:answer.fact,rows:answer.rows});if(fs&&Buffer.byteLength(text)<=limits.maximumBytes)await fs.write(path(question),brotliCompressSync(Buffer.from(text,"utf8")));}catch{}
   return stored;
  },
  claim:(question,signal,work)=>lease?lease.exclusive(resource+":answer:"+digestOf(question),work,{signal}):work(),
 });
}
