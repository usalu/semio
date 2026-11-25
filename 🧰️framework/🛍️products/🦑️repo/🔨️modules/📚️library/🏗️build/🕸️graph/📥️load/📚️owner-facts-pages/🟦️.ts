import{closeSync,openSync,readSync,unlinkSync,writeSync}from"node:fs";
import{randomUUID}from"node:crypto";
import{tmpdir}from"node:os";
import{join}from"node:path";

/** 📚️ An owner memo entry whose facts the pages may keep out of the heap: the facts are resident (an object) or paged out (`undefined`), the rows always stay resident because every hit verifies them. */
export type PagedOwnerEntryV1={facts:unknown;readonly rows:readonly unknown[]};

/** 📚️ The paged store of the owner facts of one process: every admitted entry is written once to an append-only page file, the resident facts are bounded by a character budget (least recently used first), and a paged-out entry is read back whole by its offset. The memo stays a cache: when the page file cannot be written the entry simply stays resident.
 * @see ../🟦️.ts */
export type OwnerFactsPagesV1={
  admit(entry:PagedOwnerEntryV1):void;
  facts(entry:PagedOwnerEntryV1):unknown;
  release(entry:PagedOwnerEntryV1):void;
  stats():{resident:number;residentCharacters:number;paged:number;pageBytes:number};
  close():void;
};

type PageV1={readonly offset:number;readonly bytes:number;readonly characters:number};

/** 🧮️ The resident character budget a heap budget affords the owner facts: a share of the heap, divided by the heap bytes one character of facts costs once parsed.
 * @see ../../../🧠️memory/♻️recycle/🧬️schema/🔣️.json */
export function ownerFactsResidentCharactersV1(heapMegabytes:number,heapDivisor:number,bytesPerCharacter:number):number{
  return Math.max(1,Math.floor(heapMegabytes*1048576/heapDivisor/bytesPerCharacter));
}

/** 🏗️ Creates the pages of one process; the file is created on the first admitted entry, named after the process and removed at once where the platform lets an open file go, and in every case when the pages close or the process exits.
 * @see ./🧪️tests/🟦️.ts */
export function createOwnerFactsPagesV1(options:{residentCharacters:number;maximumPageBytes:number;directory?:string}):OwnerFactsPagesV1{
  const pages=new WeakMap<PagedOwnerEntryV1,PageV1>(),resident=new Map<PagedOwnerEntryV1,number>();
  let file:number|undefined,path:string|undefined,end=0,residentCharacters=0,paged=0,broken=false,exitHook:(()=>void)|undefined;
  const open=():number|undefined=>{
    if(file!==undefined||broken)return file;
    try{
      path=join(options.directory??tmpdir(),"semio-owner-facts-"+process.pid+"-"+randomUUID()+".pages");
      file=openSync(path,"wx+",0o600);
      if(process.platform!=="win32"){unlinkSync(path);path=undefined;}
      exitHook=()=>close();process.once("exit",exitHook);
    }catch{broken=true;file=undefined;path=undefined;}
    return file;
  };
  const close=()=>{
    if(file!==undefined){try{closeSync(file);}catch{}file=undefined;}
    if(path!==undefined){try{unlinkSync(path);}catch{}path=undefined;}
    if(exitHook!==undefined){process.removeListener("exit",exitHook);exitHook=undefined;}
  };
  const write=(text:string):PageV1|undefined=>{
    const descriptor=open();if(descriptor===undefined)return undefined;
    const bytes=Buffer.from(text);
    if(end+bytes.length>options.maximumPageBytes)return undefined;
    try{for(let at=0;at<bytes.length;)at+=writeSync(descriptor,bytes,at,bytes.length-at,end+at);}catch{broken=true;return undefined;}
    const page={offset:end,bytes:bytes.length,characters:text.length};end+=bytes.length;paged++;return page;
  };
  const read=(page:PageV1):unknown=>{
    const buffer=Buffer.allocUnsafe(page.bytes);
    for(let at=0;at<page.bytes;){const count=readSync(file!,buffer,at,page.bytes-at,page.offset+at);if(!count)throw Error("Incomplete owner facts page");at+=count;}
    return JSON.parse(buffer.toString("utf8"));
  };
  const drop=(entry:PagedOwnerEntryV1)=>{const size=resident.get(entry);if(size!==undefined){resident.delete(entry);residentCharacters-=size;}};
  const evict=(keep?:PagedOwnerEntryV1)=>{
    for(const [entry,size] of resident){
      if(residentCharacters<=options.residentCharacters)return;
      if(entry===keep||!pages.has(entry))continue;
      resident.delete(entry);residentCharacters-=size;entry.facts=undefined;
    }
  };
  return{
    admit(entry){
      drop(entry);
      let text:string|undefined;
      try{text=JSON.stringify(entry.facts);}catch{text=undefined;}
      if(typeof text!=="string")return;
      let page=pages.get(entry);
      if(page===undefined){page=write(text);if(page!==undefined)pages.set(entry,page);}
      resident.set(entry,text.length);residentCharacters+=text.length;
      evict(entry);
    },
    facts(entry){
      if(entry.facts!==undefined){const size=resident.get(entry);if(size!==undefined){resident.delete(entry);resident.set(entry,size);}return entry.facts;}
      const page=pages.get(entry);if(page===undefined)return undefined;
      const value=read(page);entry.facts=value;resident.set(entry,page.characters);residentCharacters+=page.characters;evict(entry);return value;
    },
    release(entry){drop(entry);pages.delete(entry);},
    stats(){return{resident:resident.size,residentCharacters,paged,pageBytes:end};},
    close,
  };
}
