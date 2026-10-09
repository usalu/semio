/** 🧮️ Concrete row-position and consumed-bit backing for semantic relational owners. */
import {ValueError,sqliteOperation,type SqliteOperation,type SqliteDatabaseOptions,type SqliteTable,type SqliteRow,type SqliteValue} from "../../🟦️.ts";
import {artifactSqliteCheckpoint} from "../🟦️.ts";
import {ArtifactSqliteTableIndex} from "../🗂️table/🟦️.ts";

function invalid(reason:string):never{throw new ValueError("invalidValue","Artifact SQLite row index "+reason)}
function words(operation:SqliteOperation,count:number):Uint32Array{if(!Number.isSafeInteger(count)||count<0||count>0xffffffff)throw new ValueError("workLimit","Artifact SQLite row backing size");const bytes=operation.allocateBytes(count*4);if(bytes.byteOffset%4!==0)throw new ValueError("invariantViolated","Artifact SQLite row backing alignment");return new Uint32Array(bytes.buffer,bytes.byteOffset,count)}

class ParentIndex {
 private constructor(private readonly rows:readonly SqliteRow[],private readonly cells:readonly (readonly SqliteValue[])[],private readonly slots:Uint32Array,private readonly offsets:Uint32Array,private readonly positions:Uint32Array,private readonly parentColumn:number,private readonly operation:SqliteOperation){}
 private parent(at:number):bigint|null{const parent=this.cells[at]![this.parentColumn];return parent===null||typeof parent==="bigint"?parent:invalid("parent must be nullable INTEGER")}
 private async find(parent:bigint,insert?:number):Promise<number>{
  if(this.slots.length===0)return -1;let slot=Number((parent^(parent>>32n))&BigInt(this.slots.length-1));
  for(let probes=0;probes<this.slots.length;probes++){const position=this.slots[slot]!;if(position===0){if(insert===undefined)return -1;this.slots[slot]=insert+1;return insert;}if(this.parent(position-1)===parent)return position-1;slot=(slot+1)&(this.slots.length-1);if((probes+1)%256===0)await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",probes+1,this.slots.length);}
  throw new ValueError("invariantViolated","Artifact SQLite parent index exhausted");
 }
 static async create(rows:readonly SqliteRow[],cells:readonly (readonly SqliteValue[])[],ordinal:number|null,parentColumn:number,operation:SqliteOperation):Promise<ParentIndex>{
  let capacity=rows.length===0?0:2;while(capacity<rows.length*2){capacity*=2;if(capacity>0x80000000)throw new ValueError("workLimit","Artifact SQLite parent index capacity");}
  const slots=words(operation,capacity),counts=words(operation,rows.length),offsets=words(operation,rows.length+1),positions=words(operation,rows.length),index=new ParentIndex(rows,cells,slots,offsets,positions,parentColumn,operation);
  for(let at=0;at<rows.length;at++){const parent=index.parent(at);if(parent!==null){const representative=await index.find(parent,at);counts[representative]=counts[representative]!+1;}if((at+1)%256===0)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",at+1,rows.length);}
  for(let at=0;at<rows.length;at++){offsets[at+1]=offsets[at]!+counts[at]!;if((at+1)%256===0)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",at+1,rows.length);}
  counts.fill(0);
  for(let at=0;at<rows.length;at++){const parent=index.parent(at);if(parent!==null){const representative=await index.find(parent),count=offsets[representative+1]!-offsets[representative]!;
   let within:number;if(ordinal===null){within=counts[representative]!;counts[representative]=within+1;}else{const value=cells[at]![ordinal];if(typeof value!=="bigint"||value<0n||value>=BigInt(count))invalid("ordinal range or gap");within=Number(value);}
   const position=offsets[representative]!+within;if(positions[position]!==0)invalid("duplicate ordinal");positions[position]=at+1;}if((at+1)%256===0)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",at+1,rows.length);
  }
  await artifactSqliteCheckpoint(operation,"reconstructSnapshot",rows.length,rows.length);return index;
 }
 async group(parent:bigint):Promise<SqliteRow[]>{const representative=await this.find(parent),start=representative<0?0:this.offsets[representative]!,end=representative<0?0:this.offsets[representative+1]!;await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",0,end-start);const rows:SqliteRow[]=[];for(let at=start;at<end;at++){const position=this.positions[at]!;if(position===0)invalid("ordinal gap");rows.push(this.rows[position-1]!);if((at-start+1)%256===0)await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",at-start+1,end-start);}await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",end-start,end-start);return rows}
}

/** 🗃️ Uses exact bitmap and position buffers while preserving borrowed row-object identity. */
export class ArtifactSqliteRowIndex {
 private readonly identities=new Map<SqliteRow,number>();
 private readonly groups=new Map<number,Map<number,Map<number,ParentIndex>>>();
 private consumed=0;
 private constructor(private readonly tables:readonly SqliteTable[],private readonly cells:readonly (readonly (readonly SqliteValue[])[])[],private readonly starts:Uint32Array,private readonly used:Uint32Array,private readonly names:ArtifactSqliteTableIndex,private readonly operation:SqliteOperation){}
 /** 🧱️ Admits checked table offsets and consumed bits before indexing any row owner. */
 static async create(tables:readonly SqliteTable[],options:SqliteDatabaseOptions={}):Promise<ArtifactSqliteRowIndex>{
  const operation=sqliteOperation(options);if(!Number.isSafeInteger(tables.length)||tables.length<0||tables.length>(operation.maxTables??4096)||tables.length>0x40000000)throw new ValueError("workLimit","Artifact SQLite row table limit");if(operation.signal?.aborted)throw new ValueError("canceled","Artifact SQLite row indexing canceled");
  let total=0;for(const table of tables){total+=table.rows.length;if(!Number.isSafeInteger(total)||total>(operation.maxRows??1000000)||total>0xffffffff)throw new ValueError("workLimit","Artifact SQLite row index limit");}
  const captured=tables.map(table=>({name:table.name,sql:table.sql,rows:[...table.rows]})),cells=captured.map(table=>table.rows.map(row=>[...row.values]));
  await artifactSqliteCheckpoint(operation,"reconstructSnapshot",0,total);
  const starts=words(operation,captured.length+1),used=words(operation,Math.ceil(total/32)),names=await ArtifactSqliteTableIndex.create(captured,operation),index=new ArtifactSqliteRowIndex(captured,cells,starts,used,names,operation);
  let position=0;for(let at=0;at<captured.length;at++){starts[at]=position;for(const row of captured[at]!.rows){if(index.identities.has(row))invalid("same row object has multiple owners");index.identities.set(row,position++);if(position%256===0)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",position,total);}}
  starts[captured.length]=position;await artifactSqliteCheckpoint(operation,"reconstructSnapshot",position,total);return index;
 }
 /** 📍️ Resolves one full borrowed row through checked cumulative table offsets. */
 row(position:number):SqliteRow{const total=this.starts[this.tables.length]!;if(!Number.isSafeInteger(position)||position<0||position>=total)invalid("row position range");let low=0,high=this.tables.length;while(low<high){const middle=Math.floor((low+high)/2);if(this.starts[middle+1]!<=position)low=middle+1;else high=middle;}return this.tables[low]!.rows[position-this.starts[low]!]!}
 /** ✅️ Marks the actual consumed bit and refuses foreign or repeated object identities. */
 async take(row:SqliteRow):Promise<void>{if(this.operation.signal?.aborted)throw new ValueError("canceled","Artifact SQLite row consumption canceled");const position=this.identities.get(row);if(position===undefined)invalid("foreign row owner");const word=position>>>5,mask=1<<(position&31);if((this.used[word]!&mask)!==0)invalid("row consumed twice");this.used[word]=this.used[word]!|mask;this.consumed++;if(this.consumed%256===0)await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",this.consumed,this.starts[this.tables.length]!)}
 /** 🔗️ Compares full bigint parents and returns contiguous ordinals or source occurrences. */
 async grouped(table:string,parent:bigint,ordinal:number|null=2,parentColumn=1):Promise<SqliteRow[]>{
  if(typeof parent!=="bigint"||ordinal!==null&&(!Number.isSafeInteger(ordinal)||ordinal<0)||!Number.isSafeInteger(parentColumn)||parentColumn<0)invalid("relationship index role");const at=await this.names.find(table);if(at<0)invalid("unknown table");let columns=this.groups.get(at);if(columns===undefined){columns=new Map();this.groups.set(at,columns);}let cached=columns.get(parentColumn);if(cached===undefined){cached=new Map();columns.set(parentColumn,cached);}const key=ordinal??-1;let group=cached.get(key);if(group===undefined){group=await ParentIndex.create(this.tables[at]!.rows,this.cells[at]!,ordinal,parentColumn,this.operation);cached.set(key,group);}return group.group(parent);
 }
 /** 🏁️ Scans every consumed bit before allowing the complete semantic owner to escape. */
 async finish():Promise<void>{const total=this.starts[this.tables.length]!;await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",0,total);for(let at=0;at<total;at++){if((this.used[at>>>5]!&(1<<(at&31)))===0)invalid("unconsumed row");if((at+1)%256===0)await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",at+1,total);}await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",total,total)}
}
