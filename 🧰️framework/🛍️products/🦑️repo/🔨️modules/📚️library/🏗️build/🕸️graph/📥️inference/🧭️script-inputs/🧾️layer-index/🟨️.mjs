import{closeSync,fstatSync,ftruncateSync,mkdirSync,openSync,readFileSync,readSync,readdirSync,renameSync,rmSync,statSync,writeFileSync,writeSync,existsSync}from"node:fs";
import{createHash,hash as hashOnce,randomBytes}from"node:crypto";
import{join}from"node:path";
import{observationRowJsonV1}from"../../../../👁️observation/🟨️.mjs";
import{INFERENCE_LIMITS_V1}from"../../🧬️schema/🟨️.mjs";

const BATCH_MAGIC=0x32425752,IDS_MAGIC=0x3258494c,DICT_MAGIC=Buffer.from("SEMIOLD2","latin1"),DICT_HEAD=16,BATCH_HEAD=8,RECORD_HEAD=14,IDS_HEAD=16,IDS_CHECK=16,EXTENSION=".ix";

/** 🔑️ The text a row is deduplicated by: kind, path and the fact, each length-prefixed or tagged so that two different rows never share it (rows that differ only in the order of the members of an object fact are not merged, which costs one dictionary entry, never a wrong verdict). */
const rowKeyOf=row=>{const kind=String(row.kind),path=String(row.path),fact=row.fact;return kind.length+":"+kind+path.length+":"+path+(typeof fact==="string"?"s"+fact:fact===undefined?"u":"j"+JSON.stringify(fact));};

/** 🧿️ The fingerprint of one row key: the first 48 bits address the table, the next 32 bits confirm a hit, so two different rows share an entry only with a chance of 2^-80 per pair. */
const sha256=typeof hashOnce==="function"?value=>hashOnce("sha256",value,"buffer"):value=>createHash("sha256").update(value).digest(),fingerprint=key=>{const digest=sha256(key);return{key:digest.readUIntBE(0,6),check:digest.readUInt32BE(6),bytes:digest.subarray(0,10)};};

/** 🗝️ An open-addressing table from fingerprints to dictionary ids: 16 bytes per slot, grown by doubling before two-thirds occupancy. */
function createFingerprintTableV1(){
 let size=1<<16,keys=new Float64Array(size).fill(-1),checks=new Uint32Array(size),ids=new Uint32Array(size),count=0;
 const find=(key,check)=>{for(let at=(key>>>0)&(size-1);;at=(at+1)&(size-1)){if(keys[at]===-1)return undefined;if(keys[at]===key&&checks[at]===check)return ids[at];}};
 const place=(key,check,id)=>{let at=(key>>>0)&(size-1);while(keys[at]!==-1)at=(at+1)&(size-1);keys[at]=key;checks[at]=check;ids[at]=id;};
 const put=(key,check,id)=>{
  if((count+1)*3>size*2){const old={keys,checks,ids,size};size*=2;keys=new Float64Array(size).fill(-1);checks=new Uint32Array(size);ids=new Uint32Array(size);for(let at=0;at<old.size;at++)if(old.keys[at]!==-1)place(old.keys[at],old.checks[at],old.ids[at]);}
  place(key,check,id);count++;
 };
 return{find,put,size:()=>count};
}

/** 🪪️ The persisted row dictionary and the row-id list of every stored closure layer (`x/rows.dict`, `x/<layer digest>.ids`): each distinct row of all layers is written once with its fingerprint, a layer is the list of the dictionary ids of its rows, so a later process decides a layer by looking up the verdicts of its distinct rows and parses neither the layer nor its rows again. Both files are derived from the layers and append-only, every list names the dictionary it belongs to, every row read is checked against its fingerprint; a lost, replaced or damaged file only costs the parse it saves.
 * @see ../../🧬️schema/🔣️.json */
export function createLayerIndexV1({directory,limits=INFERENCE_LIMITS_V1}){
 const root=join(directory,"x"),dictionary=join(root,"rows.dict"),lockPath=join(root,"lock"),idsPath=digest=>join(root,digest+EXTENSION);
 let readDescriptor,readIdentity,readGeneration,readSize=0,blockAt=-1,blockLength=0,writable=true,table,tableIdentity;
 const counts={indexed:0,read:0,rowReads:0,full:0,refused:0},block=Buffer.allocUnsafe(limits.layerIndexReadBlockBytes);
 const closeRead=()=>{if(readDescriptor!==undefined){try{closeSync(readDescriptor);}catch{}}readDescriptor=undefined;readIdentity=undefined;readGeneration=undefined;blockAt=-1;blockLength=0;};
 const openRead=()=>{
  if(readDescriptor!==undefined)return true;
  try{
   const descriptor=openSync(dictionary,"r"),head=Buffer.allocUnsafe(DICT_HEAD);
   if(readSync(descriptor,head,0,DICT_HEAD,0)!==DICT_HEAD||!head.subarray(0,8).equals(DICT_MAGIC)){closeSync(descriptor);return false;}
   readDescriptor=descriptor;readIdentity=Buffer.from(head.subarray(8));readGeneration=Object.freeze({});readSize=fstatSync(descriptor).size;blockAt=-1;blockLength=0;return true;
  }catch{return false;}
 };
 const bytesAt=(at,length)=>{
  if(length>block.length){const out=Buffer.allocUnsafe(length);let got=0;while(got<length){const read=readSync(readDescriptor,out,got,length-got,at+got);if(read<=0)return undefined;got+=read;}return out;}
  if(!(at>=blockAt&&at+length<=blockAt+blockLength)){blockLength=readSync(readDescriptor,block,0,block.length,at);blockAt=at;if(blockLength<length)return undefined;}
  return block.subarray(at-blockAt,at-blockAt+length);
 };
 const recordAt=id=>{
  try{
   if(!openRead()||id<DICT_HEAD)return undefined;
   if(id+RECORD_HEAD>readSize){readSize=fstatSync(readDescriptor).size;if(id+RECORD_HEAD>readSize)return undefined;}
   const head=bytesAt(id,RECORD_HEAD);if(head===undefined)return undefined;
   const length=head.readUInt32LE(0),print=Buffer.from(head.subarray(4,RECORD_HEAD));if(length===0||length>limits.layerIndexMaximumRowBytes)return undefined;
   if(id+RECORD_HEAD+length>readSize){readSize=fstatSync(readDescriptor).size;if(id+RECORD_HEAD+length>readSize)return undefined;}
   const body=bytesAt(id+RECORD_HEAD,length);return body===undefined?undefined:{text:body.toString("utf8"),print};
  }catch{return undefined;}
 };
 const lock=()=>{
  const deadline=Date.now()+limits.layerIndexLockWaitMilliseconds;
  for(;;){
   try{mkdirSync(lockPath);return()=>{try{rmSync(lockPath,{recursive:true,force:true});}catch{}};}
   catch(error){
    if(error?.code==="ENOENT"){try{mkdirSync(root,{recursive:true});continue;}catch{return undefined;}}
    if(error?.code!=="EEXIST")return undefined;
    try{if(Date.now()-statSync(lockPath).mtimeMs>limits.layerIndexLockStaleMilliseconds)rmSync(lockPath,{recursive:true,force:true});}catch{}
    if(Date.now()>=deadline)return undefined;
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,2);
   }
  }
 };
 const header=(descriptor,size)=>{
  const head=Buffer.allocUnsafe(DICT_HEAD);
  if(size>=DICT_HEAD&&readSync(descriptor,head,0,DICT_HEAD,0)===DICT_HEAD&&head.subarray(0,8).equals(DICT_MAGIC))return{identity:Buffer.from(head.subarray(8)),size};
  if(size>0)ftruncateSync(descriptor,0);
  const fresh=Buffer.concat([DICT_MAGIC,randomBytes(8)]);let wrote=0;while(wrote<fresh.length)wrote+=writeSync(descriptor,fresh,wrote,fresh.length-wrote);
  return{identity:fresh.subarray(8),size:DICT_HEAD};
 };
 const seed=(descriptor,size,identity)=>{
  const found=createFingerprintTableV1();let position=DICT_HEAD;
  while(position+BATCH_HEAD<=size){
   const head=Buffer.allocUnsafe(BATCH_HEAD);if(readSync(descriptor,head,0,BATCH_HEAD,position)!==BATCH_HEAD||head.readUInt32LE(0)!==BATCH_MAGIC)break;
   const length=head.readUInt32LE(4),end=position+BATCH_HEAD+length;if(end>size)break;
   const region=Buffer.allocUnsafe(length);let got=0;while(got<length){const read=readSync(descriptor,region,got,length-got,position+BATCH_HEAD+got);if(read<=0)break;got+=read;}
   if(got<length)break;
   let at=0;
   while(at+RECORD_HEAD<=length){const row=region.readUInt32LE(at);if(row===0||at+RECORD_HEAD+row>length)break;const key=region.readUIntBE(at+4,6),check=region.readUInt32BE(at+10);if(found.find(key,check)===undefined)found.put(key,check,position+BATCH_HEAD+at);at+=RECORD_HEAD+row;}
   position=end;
  }
  if(position<size){ftruncateSync(descriptor,position);size=position;}
  table=found;tableIdentity=identity;return size;
 };
 return Object.freeze({
  /** 📖️ The dictionary ids of the rows of the layer `digest`, in layer order, or undefined while the list is absent, damaged or belongs to another dictionary than the one this process reads. */
  idsSync(digest){
   let bytes;try{bytes=readFileSync(idsPath(digest));}catch{return undefined;}
   if(bytes.length<IDS_HEAD+IDS_CHECK||bytes.readUInt32LE(0)!==IDS_MAGIC)return undefined;
   const count=bytes.readUInt32LE(4),body=IDS_HEAD+count*4;
   if(bytes.length!==body+IDS_CHECK||!createHash("sha256").update(digest).update(bytes.subarray(0,body)).digest().subarray(0,IDS_CHECK).equals(bytes.subarray(body)))return undefined;
   if(!openRead())return undefined;
   if(!readIdentity.equals(bytes.subarray(8,16))){closeRead();if(!openRead()||!readIdentity.equals(bytes.subarray(8,16)))return undefined;}
   const out=new Uint32Array(count);for(let at=0;at<count;at++)out[at]=bytes.readUInt32LE(IDS_HEAD+at*4);
   counts.read++;return out;
  },
  /** 📑 Reads a listing against one open dictionary. After replacement its outstanding offsets can only fall back to the content-addressed layer. */
  listingSync(digest){
   const ids=this.idsSync(digest);
   if(ids===undefined)return undefined;
   const dictionary=readGeneration;
   return {ids,dictionary,rowSync:id=>readGeneration===dictionary?this.rowSync(id):undefined};
  },
  /** 🔍️ The row of one dictionary id, or undefined while the id names no intact record: the row read must have the fingerprint its record carries. */
  rowSync(id){
   const record=recordAt(id);if(record===undefined)return undefined;
   try{const row=JSON.parse(record.text);if(!sha256(rowKeyOf(row)).subarray(0,10).equals(record.print)){counts.refused++;return undefined;}counts.rowReads++;return row;}catch{return undefined;}
  },
  /** ✍️ Writes the ids list of the layer `digest` from its parsed `rows` (rows the dictionary lacks are appended once), under the lock of the dictionary; undefined while it cannot be written (locked, full, read only), which only means the layer is parsed again next time. */
  indexSync(digest,rows){
   if(!writable||rows.length<limits.layerIndexMinimumRows||existsSync(idsPath(digest)))return undefined;
   const release=lock();if(release===undefined)return undefined;
   try{
    mkdirSync(root,{recursive:true});
    const descriptor=openSync(dictionary,"a+");
    try{
     const opened=header(descriptor,fstatSync(descriptor).size);let size=opened.size;
     if(table===undefined||tableIdentity===undefined||!tableIdentity.equals(opened.identity))size=seed(descriptor,size,opened.identity);
     if(size>=limits.layerIndexDictionaryMaximumBytes){counts.full++;return undefined;}
     const ids=new Uint32Array(rows.length),pending=[];let used=0;
     for(let at=0;at<rows.length;at++){
      const print=fingerprint(rowKeyOf(rows[at]));let id=table.find(print.key,print.check);
      if(id===undefined){const bytes=Buffer.from(observationRowJsonV1(rows[at]),"utf8");id=size+BATCH_HEAD+used;table.put(print.key,print.check,id);const head=Buffer.allocUnsafe(RECORD_HEAD);head.writeUInt32LE(bytes.length,0);print.bytes.copy(head,4);pending.push(head,bytes);used+=RECORD_HEAD+bytes.length;}
      ids[at]=id;
     }
     if(size+BATCH_HEAD+used>=limits.layerIndexDictionaryMaximumBytes){table=undefined;counts.full++;return undefined;}
     if(used>0){
      const head=Buffer.allocUnsafe(BATCH_HEAD);head.writeUInt32LE(BATCH_MAGIC,0);head.writeUInt32LE(used,4);
      const batch=Buffer.concat([head,...pending]);let wrote=0;
      try{while(wrote<batch.length)wrote+=writeSync(descriptor,batch,wrote,batch.length-wrote);}catch(error){table=undefined;throw error;}
     }
     const body=Buffer.allocUnsafe(IDS_HEAD+ids.length*4);body.writeUInt32LE(IDS_MAGIC,0);body.writeUInt32LE(ids.length,4);opened.identity.copy(body,8);
     for(let at=0;at<ids.length;at++)body.writeUInt32LE(ids[at],IDS_HEAD+at*4);
     const check=createHash("sha256").update(digest).update(body).digest().subarray(0,IDS_CHECK),temporary=idsPath(digest)+"."+process.pid+".tmp";
     writeFileSync(temporary,Buffer.concat([body,check]));renameSync(temporary,idsPath(digest));
     counts.indexed++;return ids;
    }finally{closeSync(descriptor);}
   }catch(error){table=undefined;if(error?.code==="EACCES"||error?.code==="EROFS"||error?.code==="EPERM")writable=false;return undefined;}
   finally{release();}
  },
  /** 🧹️ Removes the id lists (`<digest>.ix`) of layers that no longer exist in `layers` (a directory of `<digest>.json.br`) and every file of the index directory that is none of the dictionary, the lock and such a list, and the whole index once the dictionary outgrew its declared bound, which the next parses rebuild under a new dictionary identity. */
  sweepSync(layers){
   let names;try{names=readdirSync(root);}catch{return{removed:0,reset:false};}
   let removed=0;
   try{
    if(statSync(dictionary).size>=limits.layerIndexDictionaryMaximumBytes){
     const release=lock();
     if(release!==undefined)try{for(const name of names)if(name!=="lock")rmSync(join(root,name),{recursive:true,force:true});table=undefined;closeRead();return{removed:names.length-1,reset:true};}finally{release();}
    }
   }catch{}
   for(const name of names){
    if(name==="rows.dict"||name==="lock"||name.endsWith(EXTENSION)&&existsSync(join(layers,name.slice(0,-EXTENSION.length)+".json.br")))continue;
    try{rmSync(join(root,name),{force:true});removed++;}catch{}
   }
   return{removed,reset:false};
  },
  stats:()=>({...counts,fingerprints:table?.size()??0}),
 });
}
