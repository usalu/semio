import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 📈️ The declared names and bounds of the wave row. @see ./🧬️schema/🔣️.json */
export const GRAPH_WAVE_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));
/** 🗣️ The bilingual words of the server log line of one wave. */
export const GRAPH_WAVE_TEXTS_V1=freeze(structuredClone(schema.definitions.Texts.const));
const ROW_KEYS=schema.definitions.Row.required,FARM_KEYS=schema.definitions.Row.properties.farm.required,P=GRAPH_WAVE_POLICY_V1,MEGABYTE=1048576,natural=value=>Number.isSafeInteger(value)&&value>=0;

/** 📏️ Whether a read writes wave rows: only a read the graph server started asks for them. */
export function graphWaveRowsWantedV1(environment=process.env){return environment[P.environment]==="1";}

/**
 * 🧮️ Counts what happens inside one wave and hands it to the row at its end: the parent reports its collector at the end of the inference, the farm reports its pool when it closes, the inference reports the managed bytes it sees together with the projects that run, and `take` answers the row of the wave and starts the next one.
 * @param {{now?:()=>number}} [options]
 */
export function createGraphWaveMeterV1({now=Date.now}={}){
 const fresh=()=>({peakBytes:0,peakProjects:[],liveBytes:0,collections:0,collectMilliseconds:0,workers:0,projects:0,adopted:0,failed:0,lost:0,declined:0});
 let wave=fresh(),lastAt=now(),total=0,done=0,phases={};
 return Object.freeze({
  observe(bytes,names){if(bytes>wave.peakBytes){wave.peakBytes=bytes;wave.peakProjects=[...names].slice(0,P.maximumPeakProjects);}},
  parent(memory){wave.peakBytes=Math.max(wave.peakBytes,memory.peakBytes);wave.liveBytes=memory.liveBytes;wave.collections+=memory.collections;wave.collectMilliseconds+=memory.collectMilliseconds;},
  farm(tally){wave.workers=Math.max(wave.workers,tally.workers);for(const key of["projects","adopted","failed","lost","declined"])wave[key]+=tally[key]??0;},
  progress(info){if(natural(info?.total))total=info.total;if(natural(info?.done))done=info.done;if(info?.phases!==null&&typeof info?.phases==="object")phases=Object.fromEntries(Object.entries(info.phases).filter(([name,ms])=>name!==""&&name.length<=64&&natural(Math.round(ms))).slice(0,P.maximumPhases).map(([name,ms])=>[name,Math.round(ms)]));},
  take({generation,parentMegabytes=null,done:doneNow=done,total:totalNow=total}){
   const at=now(),row={version:1,generation,done:doneNow,total:Math.max(totalNow,doneNow),seconds:Math.max(0,Math.round((at-lastAt)/1000)),parentMegabytes,managedMegabytes:Math.round(wave.liveBytes/MEGABYTE),peakMegabytes:Math.round(wave.peakBytes/MEGABYTE),peakProjects:wave.peakProjects,collections:wave.collections,collectMilliseconds:Math.round(wave.collectMilliseconds),farm:{workers:wave.workers,projects:wave.projects,adopted:wave.adopted,failed:wave.failed,lost:wave.lost,declined:wave.declined},...(Object.keys(phases).length?{phases}:{})};
   wave=fresh();lastAt=at;phases={};return row;
  },
 });
}

/** 🌊️ The meter of this process: the inference, the farm and the host of one graph read all reach the same one. */
export const graphWaveMeterV1=createGraphWaveMeterV1();

/** 🖨️ The stderr line of one wave row: the marker, then the row as JSON. */
export function renderGraphWaveRowV1(row){return P.marker+JSON.stringify(row);}

/** 🔎️ Reads the row out of one stderr line; undefined for every other line and for a row that breaks the schema. */
export function parseGraphWaveRowV1(line){
 const at=line.indexOf(P.marker);if(at<0)return undefined;
 let row;try{row=JSON.parse(line.slice(at+P.marker.length));}catch{return undefined;}
 const farm=row?.farm,same=(value,keys)=>value!==null&&typeof value==="object"&&Object.keys(value).sort().join()===[...keys].sort().join();
 const phased=row?.phases;if(phased!==undefined&&(phased===null||typeof phased!=="object"||Array.isArray(phased)||Object.keys(phased).length>P.maximumPhases||Object.values(phased).some(value=>!natural(value))))return undefined;
 if(!same(Object.fromEntries(Object.entries(row??{}).filter(([key])=>key!=="phases")),ROW_KEYS)||row.version!==1||!["generation","done","total","seconds","managedMegabytes","peakMegabytes","collections","collectMilliseconds"].every(key=>natural(row[key])))return undefined;
 if(!(row.parentMegabytes===null||natural(row.parentMegabytes))||!Array.isArray(row.peakProjects)||row.peakProjects.length>P.maximumPeakProjects||row.peakProjects.some(name=>typeof name!=="string"||name===""))return undefined;
 if(!same(farm,FARM_KEYS)||!["workers","projects","adopted","failed","lost","declined"].every(key=>natural(farm[key])))return undefined;
 return row;
}

/** 💬️ Words one wave row as one line of the server log in the language of the machine. */
export function renderGraphWaveTextV1(row,language){
 const texts=GRAPH_WAVE_TEXTS_V1[language],fill=(template,fields)=>template.replace(/\{([a-zA-Z]+)\}/gu,(_token,name)=>String(fields[name]));
 return fill(texts.wave,{generation:row.generation,done:row.done,total:row.total,seconds:row.seconds,parent:row.parentMegabytes??texts.unread,managed:row.managedMegabytes,peak:row.peakMegabytes,peakAt:row.peakProjects.length?fill(texts.peakAt,{projects:row.peakProjects.join(", ")}):"",collections:row.collections,adopted:row.farm.adopted,projects:row.farm.projects,failed:row.farm.failed,lost:row.farm.lost,workers:row.farm.workers})+(row.phases&&Object.keys(row.phases).length?fill(texts.phases,{list:Object.entries(row.phases).map(([name,ms])=>name+" "+Math.round(ms/1000)+" s").join(", ")}):"");
}

const observeTreeV1=async pid=>(await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts")).observeTreeMegabytesV1(pid);

/** 📣️ Writes the row of the wave just published when the read was started by the graph server (`SEMIO_GRAPH_WAVE_ROWS=1`): the footprint of the read process tree is read from the system with a time limit (a reading that fails or hangs leaves it null), the meter hands over the wave, one line goes to the write port; nothing here can fail a build. */
export async function announceGraphWaveV1({generation,info,environment=process.env,write=line=>{process.stderr.write(line+"\n");},meter=graphWaveMeterV1,observe=observeTreeV1,pid=process.pid,limitMilliseconds=5000}){
 if(!graphWaveRowsWantedV1(environment))return undefined;
 try{
  if(info)meter.progress(info);
  let parentMegabytes=null,timer;
  try{const reading=await Promise.race([Promise.resolve(observe(pid)),new Promise(resolve=>{timer=setTimeout(()=>resolve(null),limitMilliseconds);timer.unref?.();})]);parentMegabytes=Number.isFinite(reading)&&reading>=0?Math.round(reading):null;}catch{}finally{clearTimeout(timer);}
  const row=meter.take({generation,parentMegabytes});write(renderGraphWaveRowV1(row));return row;
 }catch{return undefined;}
}
