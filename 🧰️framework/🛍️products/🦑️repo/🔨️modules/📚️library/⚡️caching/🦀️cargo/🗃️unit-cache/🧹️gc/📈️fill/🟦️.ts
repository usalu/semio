import {appendFileSync,existsSync,readFileSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
/** 📐️ The authored policy of the fill-rate telemetry.
 * @see ./🧬️schema/🔣️.json */
export const FILL_POLICY_V1=policy;
/** 📍️ One measurement: the free bytes of the image and, when a unit scan ran, the allocated bytes of every build profile. */
export type FillSampleV1=Readonly<{atMs:number;freeBytes:number;volumeBytes:number;profiles?:Readonly<Record<string,number>>}>;
/** 📈️ How fast the image fills: bytes per hour (positive = the free space shrinks, the profile grows) over the samples inside the window. */
export type FillRatesV1=Readonly<{spanMs:number;freeBytesPerHour:number;profileBytesPerHour:Readonly<Record<string,number>>}>;
const hour=3600000;

/** 🧮️ Computes the rates over the samples inside `windowMilliseconds` before `nowMs`: the free-space rate between the first and last sample, a profile's rate between its first and last scan; fewer than two samples or a span below the minimum give a zero rate. Pure and deterministic. */
export function fillRatesV1(samples:readonly FillSampleV1[],nowMs:number,override:Partial<typeof policy>={}):FillRatesV1{
 const rules={...policy,...override},inside=[...samples].filter(row=>nowMs-row.atMs<=rules.windowMilliseconds&&row.atMs<=nowMs).sort((left,right)=>left.atMs-right.atMs),first=inside[0],last=inside[inside.length-1];
 if(!first||!last||last.atMs-first.atMs<rules.minimumSpanMilliseconds)return Object.freeze({spanMs:first&&last?last.atMs-first.atMs:0,freeBytesPerHour:0,profileBytesPerHour:Object.freeze({})});
 const profiles:Record<string,number>={},scans=inside.filter(row=>row.profiles!==undefined),firstScan=scans[0],lastScan=scans[scans.length-1];
 if(firstScan&&lastScan&&lastScan.atMs-firstScan.atMs>=rules.minimumSpanMilliseconds)for(const name of Object.keys(lastScan.profiles!).sort())profiles[name]=Math.round(((lastScan.profiles![name]??0)-(firstScan.profiles![name]??0))*hour/(lastScan.atMs-firstScan.atMs));
 return Object.freeze({spanMs:last.atMs-first.atMs,freeBytesPerHour:Math.round((first.freeBytes-last.freeBytes)*hour/(last.atMs-first.atMs)),profileBytesPerHour:Object.freeze(profiles)});
}

/** 📚️ Appends one sample to the telemetry file of a cache root; a file above its maximum keeps its newest rows. Never throws. */
export function recordFillSampleV1(root:string,sample:FillSampleV1):void{
 try{
  const file=join(root,policy.file);appendFileSync(file,JSON.stringify(sample)+"\n");
  if(statSync(file).size>policy.maximumBytes)writeFileSync(file,readFileSync(file,"utf8").split("\n").filter(Boolean).slice(-policy.keepRows).join("\n")+"\n");
 }catch{}
}
/** 📖️ Reads the samples of a cache root, skipping rows that do not parse. */
export function readFillSamplesV1(root:string):FillSampleV1[]{
 const file=join(root,policy.file);if(!existsSync(file))return [];
 return readFileSync(file,"utf8").split("\n").flatMap(line=>{try{const row=JSON.parse(line);return Number.isSafeInteger(row.atMs)&&Number.isFinite(row.freeBytes)&&Number.isFinite(row.volumeBytes)?[row as FillSampleV1]:[];}catch{return [];}});
}
/** 💬️ The one-line report of the current rates in the requested language, naming the three profiles that grew fastest. */
export function fillReportV1(rates:FillRatesV1,freeBytes:number,language:"en"|"de"):string{
 if(!rates.spanMs)return messages.none[language];
 const gib=(bytes:number)=>(bytes/1073741824).toFixed(1),top=Object.entries(rates.profileBytesPerHour).sort((left,right)=>right[1]-left[1]||(left[0]<right[0]?-1:1)).slice(0,3).filter(row=>row[1]>0).map(row=>`${row[0]} ${gib(row[1])}`).join(", ")||"-";
 return messages.rate[language].replace("{minutes}",String(Math.round(rates.spanMs/60000))).replace("{rate}",gib(rates.freeBytesPerHour)).replace("{free}",gib(freeBytes)).replace("{profiles}",top);
}
