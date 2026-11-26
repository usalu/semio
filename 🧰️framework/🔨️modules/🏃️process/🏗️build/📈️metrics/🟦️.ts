import {closeSync,existsSync,fstatSync,ftruncateSync,mkdirSync,openSync,readSync,renameSync,unlinkSync,writeSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import catalogueFile from "./📇️catalogue/🔣️.json" with {type:"json"};

export type MetricKindV1="gauge"|"counter"|"state";
export type MetricPlatformV1="darwin"|"linux"|"win32";
export interface MetricV1{readonly id:string;readonly kind:MetricKindV1;readonly unit:string;readonly scope:"origin"|"fleet";readonly source:string;readonly platforms?:readonly MetricPlatformV1[];readonly label:{readonly en:string;readonly de:string}}
export interface MetricsTierV1{readonly id:string;readonly bucketMs:number;readonly buckets:number}
/** 📈️ One point of a merged series: `[start, width, tier, count, min, max, sum, last, partial]` (mean is `sum / count`); `partial` is 1 when the bucket straddles the start of the finer tier's coverage: `width` is then only its visible part before that start, the statistics are those of the whole bucket. @see ./🧬️schema/🔣️.json#/definitions/Point */
export type MetricsPointV1=readonly [start:number,width:number,tier:number,count:number,min:number,max:number,sum:number,last:number,partial:number];
export interface MetricsSeriesV1{readonly fromMs:number;readonly toMs:number;readonly metrics:Readonly<Record<string,readonly MetricsPointV1[]>>}
export interface MetricsRingsV1{readonly directory:string;readonly ids:readonly string[];readonly tiers:readonly MetricsTierV1[]}
export interface MetricsRecordV1{readonly accepted:number;readonly dropped:number;readonly unknown:number}
export interface MetricsRingRowV1{readonly start:number;readonly slots:readonly (readonly [count:number,min:number,max:number,sum:number,last:number])[]}
export interface MetricsRingFileV1{readonly tier:number;readonly bucketMs:number;readonly bucketCount:number;readonly ids:readonly string[];readonly head:number;readonly rows:readonly MetricsRingRowV1[]}

/** 📐️ The ring layout, the file naming and the tier table, straight from the schema. @see ./🧬️schema/🔣️.json */
export const METRICS_RING_V1=schema.definitions.Ring.const;
export const METRICS_FILES_V1=schema.definitions.Files.const;
export const METRICS_TIERS_V1:readonly MetricsTierV1[]=schema.definitions.Tiers.const;
export const METRICS_SOURCES_V1:Readonly<Record<string,{readonly label:{readonly en:string;readonly de:string}}>>=schema.definitions.Sources.const;
export const METRICS_CATALOGUE_V1:readonly MetricV1[]=catalogueFile.metrics as readonly MetricV1[];

/** 🧭️ Whether a machine of `platform` can measure the metric `id` at all (the catalogue lists `platforms` only for a metric that some platform has no source for): a metric a platform cannot measure is `n/a` there, not a gap. An id the catalogue does not know is answered `true`.
 * @see ./🧬️schema/🔣️.json */
export function metricSupportedOnV1(id:string,platform:string):boolean{
 const platforms=METRICS_CATALOGUE_V1.find(metric=>metric.id===id)?.platforms;
 return platforms===undefined||(platforms as readonly string[]).includes(platform);
}

const R=METRICS_RING_V1,F=METRICS_FILES_V1,SLOT=R.slotBytes,STAMP=R.stampBytes,TWO32=4294967296,NONE=-1;
const fail=(en:string,de:string):never=>{throw new Error(`${en} / ${de}`);};
const put64=(view:DataView,at:number,value:number):void=>{view.setUint32(at,value%TWO32,true);view.setUint32(at+4,Math.floor(value/TWO32),true);};
const get64=(view:DataView,at:number):number=>view.getUint32(at,true)+view.getUint32(at+4,true)*TWO32;
const viewOf=(buffer:Buffer):DataView=>new DataView(buffer.buffer,buffer.byteOffset,buffer.byteLength);
const alignDown=(value:number,width:number):number=>value-value%width;
const filePath=(directory:string,tier:MetricsTierV1):string=>join(directory,tier.id+F.extension);

function digestV1(raw:Uint8Array):readonly [number,number]{
 let hash=0xcbf29ce484222325n;
 for(const byte of raw){hash^=BigInt(byte);hash=(hash*0x100000001b3n)&0xffffffffffffffffn;}
 return [Number(hash&0xffffffffn),Number(hash>>32n)];
}

function layoutV1(ids:readonly string[]){
 const raw=new TextEncoder().encode(ids.join("\n")),padded=Math.ceil(raw.length/R.idsAlignment)*R.idsAlignment,dataOffset=R.headerBytes+padded,rowBytes=2*STAMP+ids.length*SLOT;
 return {raw,dataOffset,rowBytes,digest:digestV1(raw)};
}

function headerV1(ids:readonly string[],index:number,tier:MetricsTierV1,createdMs:number):Buffer{
 const lay=layoutV1(ids),buffer=Buffer.alloc(lay.dataOffset),view=viewOf(buffer);
 buffer.write(R.magic,0,"ascii");view.setUint16(4,R.version,true);view.setUint16(6,index,true);
 view.setUint32(8,tier.bucketMs,true);view.setUint32(12,tier.buckets,true);view.setUint32(16,ids.length,true);view.setUint32(20,lay.raw.length,true);view.setUint32(24,lay.rowBytes,true);view.setUint32(28,lay.dataOffset,true);
 view.setUint32(32,lay.digest[0],true);view.setUint32(36,lay.digest[1],true);put64(view,48,createdMs);buffer.set(lay.raw,R.headerBytes);
 return buffer;
}

interface HeaderV1{readonly index:number;readonly bucketMs:number;readonly buckets:number;readonly metrics:number;readonly idsBytes:number;readonly rowBytes:number;readonly dataOffset:number;readonly digest:string;readonly head:number;readonly ids:readonly string[]}

function readHeaderV1(fd:number):HeaderV1|null{
 const fixed=Buffer.alloc(R.headerBytes);
 if(readSync(fd,fixed,0,R.headerBytes,0)!==R.headerBytes||fixed.toString("ascii",0,4)!==R.magic)return null;
 const view=viewOf(fixed);
 if(view.getUint16(4,true)!==R.version)return null;
 const metrics=view.getUint32(16,true),idsBytes=view.getUint32(20,true),rowBytes=view.getUint32(24,true),dataOffset=view.getUint32(28,true),buckets=view.getUint32(12,true);
 if(metrics>R.maximumMetrics||rowBytes!==2*STAMP+metrics*SLOT||dataOffset<R.headerBytes+idsBytes||buckets===0||view.getUint32(8,true)===0)return null;
 const raw=Buffer.alloc(idsBytes);
 if(readSync(fd,raw,0,idsBytes,R.headerBytes)!==idsBytes)return null;
 const ids=metrics===0?[]:raw.toString("utf8").split("\n");
 if(ids.length!==metrics||fstatSync(fd).size!==dataOffset+buckets*rowBytes)return null;
 return {index:view.getUint16(6,true),bucketMs:view.getUint32(8,true),buckets,metrics,idsBytes,rowBytes,dataOffset,digest:`${view.getUint32(32,true)}:${view.getUint32(36,true)}`,head:get64(view,40),ids};
}

function rowOf(buffer:Buffer,at:number,header:HeaderV1,expected:number|null):number{
 const view=viewOf(buffer),start=get64(view,at),tail=get64(view,at+header.rowBytes-STAMP);
 if(start<=0||start!==tail||start%header.bucketMs!==0)return 0;
 if(expected!==null&&start!==expected)return 0;
 return start;
}

function slotsOf(buffer:Buffer,at:number,metrics:number):Float64Array{
 const view=viewOf(buffer),out=new Float64Array(5*metrics);
 for(let metric=0;metric<metrics;metric++){
  const slot=at+STAMP+metric*SLOT,count=view.getUint32(slot,true);
  if(count===0)continue;
  out[5*metric]=count;out[5*metric+1]=view.getFloat64(slot+4,true);out[5*metric+2]=view.getFloat64(slot+12,true);out[5*metric+3]=view.getFloat64(slot+20,true);out[5*metric+4]=view.getFloat64(slot+28,true);
 }
 return out;
}

function readRowV1(fd:number,header:HeaderV1,start:number):Float64Array|null{
 const buffer=Buffer.alloc(header.rowBytes),at=header.dataOffset+((start/header.bucketMs)%header.buckets)*header.rowBytes;
 if(readSync(fd,buffer,0,header.rowBytes,at)!==header.rowBytes||rowOf(buffer,0,header,start)===0)return null;
 return slotsOf(buffer,0,header.metrics);
}

function scanHeadV1(fd:number,header:HeaderV1):number{
 const stamps=Buffer.alloc(2*STAMP),view=viewOf(stamps);
 let head=0;
 for(let index=0;index<header.buckets;index++){
  const at=header.dataOffset+index*header.rowBytes;
  if(readSync(fd,stamps,0,STAMP,at)!==STAMP)break;
  const start=get64(view,0);
  if(start<=head||start%header.bucketMs!==0||(start/header.bucketMs)%header.buckets!==index)continue;
  if(readSync(fd,stamps,STAMP,STAMP,at+header.rowBytes-STAMP)!==STAMP)break;
  if(get64(view,STAMP)===start)head=start;
 }
 return head;
}

function headOfV1(fd:number,header:HeaderV1):number{
 let head=header.head;
 if(head===0||readRowV1(fd,header,head)===null)return scanHeadV1(fd,header);
 for(let step=0;step<header.buckets&&readRowV1(fd,header,head+header.bucketMs)!==null;step++)head+=header.bucketMs;
 return head;
}

function foldInto(accumulator:Float64Array,row:Float64Array):boolean{
 let any=false;
 for(let at=0;at<row.length;at+=5){
  const count=row[at]!;
  if(count===0)continue;
  any=true;
  if(accumulator[at]===0){accumulator[at]=count;accumulator[at+1]=row[at+1]!;accumulator[at+2]=row[at+2]!;accumulator[at+3]=row[at+3]!;accumulator[at+4]=row[at+4]!;continue;}
  accumulator[at]=accumulator[at]!+count;
  if(row[at+1]!<accumulator[at+1]!)accumulator[at+1]=row[at+1]!;
  if(row[at+2]!>accumulator[at+2]!)accumulator[at+2]=row[at+2]!;
  accumulator[at+3]=accumulator[at+3]!+row[at+3]!;accumulator[at+4]=row[at+4]!;
 }
 return any;
}

interface LevelV1{readonly tier:MetricsTierV1;readonly fd:number;readonly header:HeaderV1;readonly buffer:Buffer;readonly view:DataView;readonly stamp:Buffer;readonly accumulator:Float64Array;open:number;head:number;dirty:boolean}
interface StateV1{readonly levels:LevelV1[];readonly index:Map<string,number>;closed:boolean}
const states=new WeakMap<MetricsRingsV1,StateV1>();

function openLevelV1(directory:string,ids:readonly string[],index:number,tier:MetricsTierV1,createdMs:number):LevelV1{
 const path=filePath(directory,tier),lay=layoutV1(ids),digest=`${lay.digest[0]}:${lay.digest[1]}`,size=lay.dataOffset+tier.buckets*lay.rowBytes;
 let fd=existsSync(path)?openSync(path,"r+"):-1,header=fd<0?null:readHeaderV1(fd),created=false;
 if(header!==null&&(header.index!==index||header.bucketMs!==tier.bucketMs||header.buckets!==tier.buckets||header.digest!==digest||header.metrics!==ids.length)){closeSync(fd);fd=-1;header=null;}
 if(header===null){
  if(fd>=0)closeSync(fd);
  const temporary=`${path}.${process.pid}.tmp`,out=openSync(temporary,"w+");
  try{writeSync(out,headerV1(ids,index,tier,createdMs),0,lay.dataOffset,0);ftruncateSync(out,size);}finally{closeSync(out);}
  renameSync(temporary,path);
  fd=openSync(path,"r+");header=readHeaderV1(fd);created=true;
  if(header===null){closeSync(fd);return fail(`The ring file ${path} could not be created.`,`Die Ringdatei ${path} konnte nicht angelegt werden.`);}
 }
 const buffer=Buffer.alloc(header.rowBytes);
 return {tier,fd,header,buffer,view:viewOf(buffer),stamp:Buffer.alloc(STAMP),accumulator:new Float64Array(5*ids.length),open:NONE,head:created?0:headOfV1(fd,header),dirty:false};
}

function restoreV1(levels:readonly LevelV1[]):void{
 for(let index=1;index<levels.length;index++){
  const lower=levels[index-1]!,level=levels[index]!;
  if(lower.head===0)continue;
  level.open=alignDown(lower.head,level.tier.bucketMs);
  for(let start=level.open;start<=lower.head;start+=lower.tier.bucketMs){
   const row=readRowV1(lower.fd,lower.header,start);
   if(row!==null&&foldInto(level.accumulator,row))level.dirty=true;
  }
 }
}

function writeRowV1(level:LevelV1,start:number):void{
 const {buffer,view,accumulator,header}=level;
 buffer.fill(0);put64(view,0,start);put64(view,header.rowBytes-STAMP,start);
 for(let metric=0;metric<header.metrics;metric++){
  const count=accumulator[5*metric]!;
  if(count===0)continue;
  const slot=STAMP+metric*SLOT;
  view.setUint32(slot,count,true);view.setFloat64(slot+4,accumulator[5*metric+1]!,true);view.setFloat64(slot+12,accumulator[5*metric+2]!,true);view.setFloat64(slot+20,accumulator[5*metric+3]!,true);view.setFloat64(slot+28,accumulator[5*metric+4]!,true);
 }
 writeSync(level.fd,buffer,0,header.rowBytes,header.dataOffset+((start/header.bucketMs)%header.buckets)*header.rowBytes);
 if(start>level.head){level.head=start;put64(viewOf(level.stamp),0,start);writeSync(level.fd,level.stamp,0,STAMP,40);}
}

function closeLevelV1(levels:readonly LevelV1[],index:number):void{
 const level=levels[index]!;
 if(level.dirty){
  writeRowV1(level,level.open);
  const upper=levels[index+1];
  if(upper!==undefined){
   const window=alignDown(level.open,upper.tier.bucketMs);
   if(upper.open!==window){if(upper.open!==NONE)closeLevelV1(levels,index+1);upper.open=window;}
   upper.dirty=foldInto(upper.accumulator,level.accumulator)||upper.dirty;
  }
  level.accumulator.fill(0);level.dirty=false;
 }
}

/** 📂️ Opens (creating when absent) the ring files of one machine in `directory`, one per tier, and rebuilds the open windows of the coarser tiers from the rows on disk. A directory has one writer. `createdMs` stamps new files (tests pin it for byte-identical files). Tier ids are `t0`, `t1`, ... in order. Rings whose metric ids differ from `ids` start empty: a catalogue change has no migration. The handle holds file descriptors until `closeMetricsRingsV1`. @see ./🧬️schema/🔣️.json#/definitions/Ring */
export function openMetricsRingsV1(options:{readonly directory:string;readonly ids?:readonly string[];readonly tiers?:readonly MetricsTierV1[];readonly createdMs?:number}):MetricsRingsV1{
 const ids=options.ids??METRICS_CATALOGUE_V1.map(metric=>metric.id),tiers=options.tiers??METRICS_TIERS_V1;
 if(ids.length===0||ids.length>R.maximumMetrics||new Set(ids).size!==ids.length)fail("The metric ids must be unique and not empty.","Die Metrik-Kennungen müssen eindeutig und nicht leer sein.");
 if(tiers.length===0||tiers.length>R.maximumTiers||tiers.some((tier,index)=>tier.id!==`t${index}`))fail("The tiers must be named t0, t1, ... and not exceed the maximum.","Die Stufen müssen t0, t1, ... heißen und das Maximum einhalten.");
 tiers.forEach((tier,index)=>{if(index>0&&tier.bucketMs%tiers[index-1]!.bucketMs!==0)fail(`The bucket of tier ${tier.id} is not a multiple of the tier before.`,`Das Zeitfenster der Stufe ${tier.id} ist kein Vielfaches der vorherigen.`);});
 mkdirSync(options.directory,{recursive:true});
 const levels=tiers.map((tier,index)=>openLevelV1(options.directory,ids,index,tier,options.createdMs??Date.now()));
 restoreV1(levels);
 const handle:MetricsRingsV1={directory:options.directory,ids,tiers};
 states.set(handle,{levels,index:new Map(ids.map((id,at)=>[id,at])),closed:false});
 return handle;
}

/** ✍️ Records one sample: `values` maps metric ids to this tick's value (a counter's value is its delta since the previous tick, see `createCounterDeltasV1`). The sample joins the open 1-tick window; a sample of a later window closes the open one (row written, then folded up the tiers). Samples older than the newest written row, non-finite values and unknown ids are counted, never thrown. @see ./🧬️schema/🔣️.json#/definitions/Tiers */
export function recordSampleV1(handle:MetricsRingsV1,atMs:number,values:Readonly<Record<string,number>>):MetricsRecordV1{
 const state=states.get(handle);
 if(state===undefined||state.closed)return fail("The metrics rings are closed.","Die Metrikringe sind geschlossen.");
 const first=state.levels[0]!,window=alignDown(Math.floor(atMs),first.tier.bucketMs);
 let accepted=0,dropped=0,unknown=0;
 if(!Number.isFinite(atMs)||window<=first.head||(first.open!==NONE&&window<first.open)){return {accepted,dropped:Object.keys(values).length,unknown};}
 if(first.open!==window){if(first.open!==NONE)closeLevelV1(state.levels,0);first.open=window;}
 const accumulator=first.accumulator;
 for(const id in values){
  const metric=state.index.get(id),value=values[id]!;
  if(metric===undefined){unknown++;continue;}
  if(!Number.isFinite(value)){dropped++;continue;}
  const at=5*metric;
  if(accumulator[at]===0){accumulator[at]=1;accumulator[at+1]=value;accumulator[at+2]=value;accumulator[at+3]=value;accumulator[at+4]=value;}
  else{accumulator[at]=accumulator[at]!+1;if(value<accumulator[at+1]!)accumulator[at+1]=value;if(value>accumulator[at+2]!)accumulator[at+2]=value;accumulator[at+3]=accumulator[at+3]!+value;accumulator[at+4]=value;}
  accepted++;first.dirty=true;
 }
 return {accepted,dropped,unknown};
}

/** 🔚️ Writes the open finest window and closes the files; the coarser open windows are rebuilt from disk by the next open. */
export function closeMetricsRingsV1(handle:MetricsRingsV1):void{
 const state=states.get(handle);
 if(state===undefined||state.closed)return;
 closeLevelV1(state.levels,0);state.closed=true;
 for(const level of state.levels)closeSync(level.fd);
}

/** ➕️ Turns cumulative counter readings into the per-tick deltas `recordSampleV1` takes: the first reading of an id is only a baseline (`null`), a reading below the previous one is a restart of the counter and counts as its whole value. */
export function createCounterDeltasV1():{delta(id:string,reading:number):number|null}{
 const previous=new Map<string,number>();
 return {delta(id,reading){
  const before=previous.get(id);previous.set(id,reading);
  return before===undefined||!Number.isFinite(reading)?null:reading<before?reading:reading-before;
 }};
}

/** 🗜️ Reads one ring file completely: its header facts and every valid row inside the ring horizon, ordered by start. Used by tests and tools; `readSeriesV1` reads only the rows it needs. */
export function readRingFileV1(path:string):MetricsRingFileV1|null{
 const fd=openSync(path,"r");
 try{
  const header=readHeaderV1(fd);
  if(header===null)return null;
  const size=header.buckets*header.rowBytes,buffer=Buffer.alloc(size);
  readSync(fd,buffer,0,size,header.dataOffset);
  const rows:MetricsRingRowV1[]=[];
  for(let index=0;index<header.buckets;index++){
   const start=rowOf(buffer,index*header.rowBytes,header,null);
   if(start===0||(start/header.bucketMs)%header.buckets!==index)continue;
   const slots=slotsOf(buffer,index*header.rowBytes,header.metrics),out:[number,number,number,number,number][]=[];
   for(let metric=0;metric<header.metrics;metric++)out.push([slots[5*metric]!,slots[5*metric+1]!,slots[5*metric+2]!,slots[5*metric+3]!,slots[5*metric+4]!]);
   rows.push({start,slots:out});
  }
  const head=headOfV1(fd,header),horizon=head+header.bucketMs-header.buckets*header.bucketMs;
  rows.sort((left,right)=>left.start-right.start);
  return {tier:header.index,bucketMs:header.bucketMs,bucketCount:header.buckets,ids:header.ids,head,rows:rows.filter(row=>row.start>=horizon)};
 }finally{closeSync(fd);}
}

interface ReadTierV1{readonly fd:number;readonly header:HeaderV1}

/** 📖️ Reads `[fromMs, toMs)` from the ring files of `directory` as one merged, non-uniform series per metric: the finest tier answers the newest part, each coarser tier only the part older than the oldest row of the finer ones, so points never overlap and get coarser with age. Absent or foreign tier files are skipped; `ids` narrows the metrics (default all of the files). A row that starts before `fromMs` is included when it ends after it. A coarse bucket that straddles the start of the finer coverage is kept with its whole-bucket statistics, `partial` 1 and a width clipped to the part before that start, so a series has no gap at tier boundaries. @see ./🧬️schema/🔣️.json#/definitions/Point */
export function readSeriesV1(options:{readonly directory:string;readonly fromMs:number;readonly toMs:number;readonly ids?:readonly string[]}):MetricsSeriesV1{
 const {directory,fromMs,toMs}=options;
 if(!(toMs>fromMs))fail(`The range from ${fromMs} to ${toMs} is empty or reversed.`,`Der Bereich von ${fromMs} bis ${toMs} ist leer oder umgekehrt.`);
 const opened:ReadTierV1[]=[];
 try{
  for(let index=0;index<R.maximumTiers;index++){
   const path=filePath(directory,{id:`t${index}`,bucketMs:1,buckets:1});
   if(!existsSync(path))continue;
   const fd=openSync(path,"r"),header=readHeaderV1(fd);
   if(header===null||header.index!==index||(opened.length>0&&header.digest!==opened[0]!.header.digest)){closeSync(fd);continue;}
   opened.push({fd,header});
  }
  const ids=opened[0]?.header.ids??[],wanted=options.ids??ids,columns=wanted.map(id=>ids.indexOf(id));
  const unknown=wanted.filter((_,at)=>columns[at]===-1);
  if(unknown.length>0)fail(`The metric ids ${unknown.join(", ")} are not in the rings.`,`Die Metrik-Kennungen ${unknown.join(", ")} stehen nicht in den Ringen.`);
  const chunks:{readonly tier:number;readonly width:number;readonly rows:readonly {readonly start:number;readonly width:number;readonly partial:number;readonly slots:Float64Array}[]}[]=[];
  let boundary=toMs,covered=false;
  for(const {fd,header} of opened){
   const width=header.bucketMs,head=headOfV1(fd,header);
   if(head===0)continue;
   const horizon=head+width-header.buckets*width,low=Math.max(alignDown(fromMs,width),horizon),straddle=covered&&boundary%width!==0?alignDown(boundary,width):-1,high=Math.min(head,straddle>=0?straddle:alignDown(boundary-width,width));
   if(high<low)continue;
   const count=(high-low)/width+1,rows:{start:number;width:number;partial:number;slots:Float64Array}[]=[];
   const first=(low/width)%header.buckets,run=Math.min(count,header.buckets-first),buffer=Buffer.alloc(count*header.rowBytes);
   readSync(fd,buffer,0,run*header.rowBytes,header.dataOffset+first*header.rowBytes);
   if(run<count)readSync(fd,buffer,run*header.rowBytes,(count-run)*header.rowBytes,header.dataOffset);
   for(let step=0;step<count;step++){
    const start=low+step*width,at=step*header.rowBytes;
    if(rowOf(buffer,at,header,start)===0)continue;
    rows.push({start,width:start===straddle?boundary-start:width,partial:start===straddle?1:0,slots:slotsOf(buffer,at,header.metrics)});
   }
   if(rows.length===0)continue;
   chunks.push({tier:header.index,width,rows});
   boundary=Math.min(boundary,rows[0]!.start);covered=true;
  }
  const metrics:Record<string,MetricsPointV1[]>={};
  wanted.forEach(id=>{metrics[id]=[];});
  for(let at=chunks.length-1;at>=0;at--){
   const chunk=chunks[at]!;
   for(const row of chunk.rows)wanted.forEach((id,position)=>{
    const slot=5*columns[position]!,count=row.slots[slot]!;
    if(count>0)metrics[id]!.push([row.start,row.width,chunk.tier,count,row.slots[slot+1]!,row.slots[slot+2]!,row.slots[slot+3]!,row.slots[slot+4]!,row.partial]);
   });
  }
  return {fromMs,toMs,metrics};
 }finally{for(const {fd} of opened)closeSync(fd);}
}

/** 🧹️ Deletes the ring files of `directory` (used by tests and by a machine that leaves the fleet). */
export function removeMetricsRingsV1(directory:string):void{
 for(const tier of METRICS_TIERS_V1){const path=filePath(directory,tier);if(existsSync(path))unlinkSync(path);}
}
