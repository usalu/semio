/** 🗂️ Checked table lookup whose admitted Uint32 slots perform every collision probe. */
import {ValueError,sqliteOperation,type SqliteOperation,type SqliteDatabaseOptions} from "../../🟦️.ts";
import {artifactSqliteCheckpoint} from "../🟦️.ts";

function code(value:number):number{return value>=65&&value<=90?value+32:value}

/** 🏛️ Retains literal names while owning only explicit checked table-position backing. */
export class ArtifactSqliteTableIndex {
 private constructor(private readonly names:readonly string[],private readonly slots:Uint32Array,private readonly operation:SqliteOperation,private readonly exact:boolean){}
 /** 🧱️ Admits the actual half-load slot array before any table position is materialized. */
 static async create(tables:readonly {readonly name:string}[],options:SqliteDatabaseOptions={},exact=false):Promise<ArtifactSqliteTableIndex>{
  const operation=sqliteOperation(options);
  if(!Number.isSafeInteger(tables.length)||tables.length<0||tables.length>(operation.maxTables??4096)||tables.length>0x40000000)throw new ValueError("workLimit","SQLite table index size limit");
  if(operation.signal?.aborted)throw new ValueError("canceled","SQLite table index canceled");
  const names=tables.map(table=>table.name);
  for(const name of names)if(typeof name!=="string")throw new ValueError("invalidValue","SQLite table index name must be TEXT");
  await artifactSqliteCheckpoint(operation,"indexTables",0,names.length);
  let capacity=names.length===0?0:2;while(capacity<names.length*2){capacity*=2;if(capacity>0x80000000)throw new ValueError("workLimit","SQLite table index capacity overflow");}
  const backing=operation.allocateBytes(capacity*4);if(backing.byteOffset%4!==0)throw new ValueError("invariantViolated","SQLite table index backing alignment");
  const index=new ArtifactSqliteTableIndex(names,new Uint32Array(backing.buffer,backing.byteOffset,capacity),operation,exact);
  for(let at=0;at<names.length;at++){await index.insert(at);if((at+1)%256===0)await artifactSqliteCheckpoint(operation,"indexTables",at+1,names.length);}
  await artifactSqliteCheckpoint(operation,"indexTables",names.length,names.length);return index;
 }
 private async hash(name:string):Promise<number>{
  let hash=2166136261;for(let at=0;at<name.length;at++){hash=Math.imul(hash^code(name.charCodeAt(at)),16777619)>>>0;if((at+1)%16384===0)await artifactSqliteCheckpoint(this.operation,"indexTables",at+1,name.length);}
  return hash;
 }
 private async equal(left:string,right:string):Promise<boolean>{
  if(left.length!==right.length)return false;
  for(let at=0;at<left.length;at++){if((this.exact?left.charCodeAt(at):code(left.charCodeAt(at)))!==(this.exact?right.charCodeAt(at):code(right.charCodeAt(at))))return false;if((at+1)%16384===0)await artifactSqliteCheckpoint(this.operation,"indexTables",at+1,left.length);}
  return true;
 }
 private async insert(index:number):Promise<void>{
  let slot=(await this.hash(this.names[index]!))&(this.slots.length-1);
  for(let probes=0;probes<this.slots.length;probes++){const found=this.slots[slot]!;if(found===0){this.slots[slot]=index+1;return;}if(await this.equal(this.names[found-1]!,this.names[index]!))throw new ValueError("invalidValue","Duplicate SQLite table index name");slot=(slot+1)&(this.slots.length-1);if((probes+1)%256===0)await artifactSqliteCheckpoint(this.operation,"indexTables",probes+1,this.slots.length);}
  throw new ValueError("invariantViolated","SQLite table index has no vacant slot");
 }
 /** 🔎️ Finds a complete literal key; ASCII folding is explicit and never changes Unicode identity. */
 async find(name:string):Promise<number>{
  if(typeof name!=="string")throw new ValueError("invalidValue","SQLite table index lookup must be TEXT");
  if(this.operation.signal?.aborted)throw new ValueError("canceled","SQLite table index canceled");
  if(this.slots.length===0)return -1;let slot=(await this.hash(name))&(this.slots.length-1);
  for(let probes=0;probes<this.slots.length;probes++){const found=this.slots[slot]!;if(found===0)return -1;if(await this.equal(this.names[found-1]!,name))return found-1;slot=(slot+1)&(this.slots.length-1);if((probes+1)%256===0)await artifactSqliteCheckpoint(this.operation,"indexTables",probes+1,this.slots.length);}
  return -1;
 }
}
