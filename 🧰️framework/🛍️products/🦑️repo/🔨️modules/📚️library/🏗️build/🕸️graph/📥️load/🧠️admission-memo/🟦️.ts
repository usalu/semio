import{randomUUID}from"node:crypto";
import{mkdir,open,rename,rm,type FileHandle}from"node:fs/promises";
import{dirname}from"node:path";
import{readBuildOwnerAdmissionV1,type BuildOwnerAdmissionV1}from"../../🚪️admission/🟦️.ts";
import{streamJsonObjectV1}from"./🌊️json-stream/🟦️.ts";

/** 🧠️ What a selected read remembers on the machine: the closed owner admission of one scope, every observation it was derived from and the admission facts of each owner with the rows (indexes into `observations`) that owner was observed through, under the admission format that derived them. @see ../🟦️.ts */
export type AdmissionMemoV1={version:2;format:string;scope:string;admission:BuildOwnerAdmissionV1;observations:readonly unknown[];owners:readonly {suffix:string;facts:unknown;rows:readonly number[]}[]};

/** 📦️ The admission facts of one owner as the process remembers them. */
export type OwnerFactsEntryV1={suffix:string;facts:unknown;rows:readonly unknown[]};

/** 📖️ A stored memo as a read leaves it: the header and the observations are held, the owners are an async stream still reading the file, so the facts of hundreds of owners never exist at once; `close` ends the stream and releases the file (it must be called, and ending the owners stream closes it too). */
export type StoredAdmissionMemoV1=Omit<AdmissionMemoV1,"owners">&{owners:AsyncIterable<unknown>;close():Promise<void>};

/** 📖️ The remembered admission of a scope, or nothing: a missing, corrupt, foreign (scope, shape) or foreign-order memo, or one of another admission format (`accept`), is a miss, never an error (a memo of another format is refused before its observations are read). The file is read as a stream; whether its observations still hold is asked by the caller, and a memo that does not hold still lends the facts of its owners (`ownersOfAdmissionMemoV1`), each of which revalidates its own rows. */
export async function readAdmissionMemoV1(path:string,scope:string,accept:(value:AdmissionMemoV1)=>boolean):Promise<StoredAdmissionMemoV1|undefined>{
 let file:FileHandle;
 try{file=await open(path,"r");}catch{return undefined;}
 const events=streamJsonObjectV1(file,new Set(["observations","owners"])),header:Record<string,unknown>={},observations:unknown[]=[];
 let closed=false;
 const close=async()=>{if(closed)return;closed=true;try{await events.return(undefined);}catch{}await file.close().catch(()=>undefined);};
 try{
  let checked=false;
  for(;;){
   const next=await events.next();
   if(next.done)break;
   const event=next.value;
   if(event.key==="owners")break;
   if(event.key==="observations"){
    if(!checked){
     checked=true;
     if(header.version!==2||header.scope!==scope)break;
     readBuildOwnerAdmissionV1(header.admission as BuildOwnerAdmissionV1);
     if(!accept({...header,observations:[],owners:[]} as unknown as AdmissionMemoV1))break;
    }
    if("element" in event)observations.push(event.element);
    else if("end" in event){
     const owners=(async function*(){try{for(;;){const more=await events.next();if(more.done)return;const row=more.value;if(row.key!=="owners")continue;if("element" in row)yield row.element;else if("end" in row)return;}}finally{await close();}})();
     return{version:2,format:header.format as string,scope,admission:header.admission as BuildOwnerAdmissionV1,observations,owners,close};
    }else break;
   }else if("value" in event)header[event.key]=event.value;
  }
 }catch{}
 await close();return undefined;
}

/** 🗜️ A memo of the observations and of the owners, the rows of each owner stored once as indexes into the observations (an owner row that the capture did not retain is appended). A row is found by its object first (the rows of a read are shared instances, so no owner row is serialised again) and by its text only when it is another instance of an equal row; the text index is built only then. */
export function packAdmissionMemoV1(input:{format:string;scope:string;admission:BuildOwnerAdmissionV1;observations:readonly unknown[];owners:readonly OwnerFactsEntryV1[]}):AdmissionMemoV1{
 const observations=[...input.observations],byRow=new Map<unknown,number>();let byText:Map<string,number>|undefined;
 observations.forEach((row,at)=>{if(!byRow.has(row))byRow.set(row,at);});
 const texts=()=>{if(byText===undefined){byText=new Map();observations.forEach((row,at)=>{const key=JSON.stringify(row);if(!byText!.has(key))byText!.set(key,at);});}return byText;};
 const at=(row:unknown)=>{
  let found=byRow.get(row);if(found!==undefined)return found;
  const key=JSON.stringify(row),index=texts();found=index.get(key);
  if(found===undefined){found=observations.push(row)-1;index.set(key,found);}
  byRow.set(row,found);return found;
 };
 return {version:2,format:input.format,scope:input.scope,admission:input.admission,observations,owners:input.owners.map(owner=>({suffix:owner.suffix,get facts(){return owner.facts;},rows:owner.rows.map(at)}))};
}

/** 📤️ Resolves the rows of owners missing from the current admission cache, one at a time as the memo yields them. Malformed owner entries are skipped. */
export async function* ownersOfAdmissionMemoV1(memo:{observations:readonly unknown[];owners:AsyncIterable<unknown>|Iterable<unknown>},known?:ReadonlySet<string>):AsyncGenerator<OwnerFactsEntryV1>{
 for await(const entry of memo.owners){
  const owner=entry as {suffix?:unknown;facts?:unknown;rows?:unknown};
  if(typeof owner?.suffix!=="string"||known?.has(owner.suffix)||!Object.hasOwn(owner,"facts")||!Array.isArray(owner.rows))continue;
  if(!owner.rows.every(at=>Number.isSafeInteger(at)&&at>=0&&at<memo.observations.length))continue;
  const rows=owner.rows.map(at=>memo.observations[at]);
  if(rows.every(row=>row!==undefined))yield{suffix:owner.suffix,facts:owner.facts,rows};
 }
}

/** 📏️ The size of one write of a memo: the text of the rows is cut into chunks of about this many characters, so writing a memo of hundreds of megabytes never holds the text of the whole memo. */
const MEMO_CHUNK_CHARACTERS_V1=1<<20;

/** 💾️ Remembers an admission by writing a temporary file and renaming it, so a reader sees a whole memo or none; the file is written in chunks (the text of the memo exists one chunk at a time, never whole) and stops as soon as it would exceed `maximumBytes` (answers false and leaves nothing behind). The bytes are those of `JSON.stringify(value)`. */
export async function writeAdmissionMemoV1(path:string,value:AdmissionMemoV1,maximumBytes:number):Promise<boolean>{
 const temporary=path+"."+randomUUID(),tooLarge=Symbol("memo too large");let file:FileHandle|undefined;
 try{
  await mkdir(dirname(path),{recursive:true});
  file=await open(temporary,"wx");
  let bytes=0,chunk:string[]=[],length=0;
  const flush=async()=>{if(chunk.length===0)return;const buffer=Buffer.from(chunk.join(""));chunk=[];length=0;bytes+=buffer.length;if(bytes>maximumBytes)throw tooLarge;await file!.write(buffer);};
  const put=async(text:string)=>{chunk.push(text);length+=text.length;if(length>=MEMO_CHUNK_CHARACTERS_V1)await flush();};
  await put("{");
  let first=true;
  for(const [key,field] of Object.entries(value)){
   if(field===undefined)continue;
   await put((first?"":",")+JSON.stringify(key)+":");first=false;
   if(Array.isArray(field)&&(key==="observations"||key==="owners")){
    await put("[");
    for(let at=0;at<field.length;at++)await put((at?",":"")+(JSON.stringify(field[at])??"null"));
    await put("]");
   }else await put(JSON.stringify(field));
  }
  await put("}");await flush();await file.close();file=undefined;
  await rename(temporary,path);return true;
 }catch(error){if(error===tooLarge)return false;throw error;}
 finally{await file?.close().catch(()=>undefined);await rm(temporary,{force:true});}
}
