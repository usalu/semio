import{basename}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{registerSliceReportSourceV1}from"../📋️slice-report/🟨️.mjs";

/** 🔬️ The declared numbers of the sampling profile of a slice worker: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const SLICE_SAMPLER_POLICY_V1=Object.freeze({...schema.definitions.Policy.const});
const keyOf=frame=>(frame?.name||"(anonymous)")+"\0"+basename(String(frame?.sourceURL??""))+"\0"+(Number(frame?.line)||0);
const rowOf=(key,samples)=>{const[name,file,line,parent]=key.split("\0");return{name,file,line:Number(line),samples,...parent===undefined?{}:{parent}};};
const scriptFrame=frame=>typeof frame?.sourceURL==="string"&&frame.sourceURL!=="";

/** 🧮️ Adds the stack traces of one drain to `book` (`{samples,dropped,self:Map,inclusive:Map,attributed:Map}`): the first frame of a trace is its self function, every distinct function of a trace counts once as inclusive, the first function of the script itself (with the one that called it) is charged for the time the engine spent in its natives (file reads, hashing, parsing); a key beyond `maximumKeys` is not added and counted as dropped. */
export function addSampleTracesV1(book,traces,policy=SLICE_SAMPLER_POLICY_V1){
 const bump=(map,key)=>{const held=map.get(key);if(held!==undefined){map.set(key,held+1);return;}if(map.size>=policy.maximumKeys){book.dropped++;return;}map.set(key,1);};
 for(const trace of traces??[]){
  const frames=(trace?.frames??[]).slice(0,policy.maximumFrames);if(frames.length===0)continue;
  book.samples++;bump(book.self,keyOf(frames[0]));
  const first=frames.findIndex(scriptFrame);if(first>=0){const next=frames.slice(first+1).find(scriptFrame);bump(book.attributed,keyOf(frames[first])+"\0"+(next?.name||"(top)"));}
  const seen=new Set();for(const frame of frames){const key=keyOf(frame);if(seen.has(key))continue;seen.add(key);bump(book.inclusive,key);}
 }
 return book;
}

/** 📒️ An empty book of samples. */
export const createSampleBookV1=()=>({samples:0,dropped:0,self:new Map(),inclusive:new Map(),attributed:new Map()});

/** 🧾️ The `top` rows of a book by samples (ties by name), as the schema's `Summary`. */
export function summarizeSampleBookV1(book,policy=SLICE_SAMPLER_POLICY_V1){
 const top=map=>[...map].sort((a,b)=>b[1]-a[1]||(a[0]<b[0]?-1:1)).slice(0,policy.top).map(([key,count])=>rowOf(key,count));
 return{samples:book.samples,dropped:book.dropped,self:top(book.self),inclusive:top(book.inclusive),attributed:top(book.attributed)};
}

/** 🪣️ The index of the uptime bucket of `seconds`. */
export const sampleBucketV1=(seconds,policy=SLICE_SAMPLER_POLICY_V1)=>{let at=0;while(at<policy.bucketsUptimeSeconds.length&&seconds>=policy.bucketsUptimeSeconds[at])at++;return at;};

/** 🔬️ Starts the sampling profiler of the JavaScript engine of this process (when it has one) and drains it every `drainMilliseconds` into one book per uptime bucket, so the slice report shows where the CPU of a worker goes early and late; the report source is `profile`. Off unless `SEMIO_SLICE_SAMPLER=on` (the engine's profiler walks the stack of the running thread; a worker does not carry it by default). Answers the function that stops it.
 * @param {{engine?:{startSamplingProfiler?:()=>void;samplingProfilerStackTraces?:()=>{traces?:unknown}|undefined};environment?:Record<string,string|undefined>;uptime?:()=>number;policy?:typeof SLICE_SAMPLER_POLICY_V1;setTimer?:(run:()=>void,milliseconds:number)=>{unref?:()=>void};clearTimer?:(timer:unknown)=>void}} [options]
 * @returns {()=>void} */
export function startSliceSamplerV1({engine,environment=process.env,uptime=()=>performance.now()/1000,policy=SLICE_SAMPLER_POLICY_V1,setTimer=setInterval,clearTimer=clearInterval}={}){
 if(environment.SEMIO_SLICE_SAMPLER!=="on")return()=>{};
 let jsc=engine;if(jsc===undefined){try{jsc=globalThis.Bun!==undefined?process.getBuiltinModule?.("bun:jsc")??undefined:undefined;}catch{}}
 if(typeof jsc?.startSamplingProfiler!=="function"||typeof jsc?.samplingProfilerStackTraces!=="function")return()=>{};
 const books=Array.from({length:policy.bucketsUptimeSeconds.length+1},createSampleBookV1);
 const drain=()=>{try{const taken=jsc.samplingProfilerStackTraces();addSampleTracesV1(books[sampleBucketV1(uptime(),policy)],taken?.traces,policy);}catch{}};
 try{jsc.startSamplingProfiler();}catch{return()=>{};}
 const timer=setTimer(drain,policy.drainMilliseconds);timer.unref?.();
 const off=registerSliceReportSourceV1("profile",()=>({description:schema.definitions.Texts.const,buckets:policy.bucketsUptimeSeconds,books:books.map(book=>summarizeSampleBookV1(book,policy))}));
 return()=>{clearTimer(timer);drain();off();};
}
