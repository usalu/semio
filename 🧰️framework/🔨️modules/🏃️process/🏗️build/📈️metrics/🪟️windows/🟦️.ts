import {readSeriesV1,type MetricsPointV1} from "../🟦️.ts";

/** 🪟️ What a counter added up to inside each of several windows that end now. */
export type MetricWindowTotalsV1=Readonly<{windowsSeconds:readonly number[];totals:Readonly<Record<string,readonly number[]>>}>;

/** 🧮️ The part of a point's sum that lies inside `[fromMs, toMs]`, by the share of its span that overlaps (a bucket is never split by its content, so only the one bucket the window starts in is estimated). */
export function pointShareV1(point:MetricsPointV1,fromMs:number,toMs:number):number{
 const [start,width,,,,,sum]=point,overlap=Math.min(start+width,toMs)-Math.max(start,fromMs);
 return overlap<=0?0:overlap>=width?sum:sum*overlap/width;
}

/** 📊️ Adds up counters over trailing windows from the multi-resolution rings of one machine (a counter ring holds the growth of each sample, so the sum of a window is the number of events in it): one read of the longest window answers every window, the finest tier answering the newest part. A machine without rings, or without one of the ids, answers `null`; a counter with no sample in a window counts 0.
 * @see ../🟦️.ts the rings and their merged series */
export function metricWindowTotalsV1(input:Readonly<{directory:string;ids:readonly string[];nowMs:number;windowsSeconds:readonly number[]}>):MetricWindowTotalsV1|null{
 const longest=Math.max(...input.windowsSeconds)*1000;
 try{
  const series=readSeriesV1({directory:input.directory,fromMs:input.nowMs-longest,toMs:input.nowMs,ids:input.ids});
  const totals:Record<string,number[]>={};
  for(const id of input.ids)totals[id]=input.windowsSeconds.map(seconds=>{let sum=0;for(const point of series.metrics[id]??[])sum+=pointShareV1(point,input.nowMs-seconds*1000,input.nowMs);return Math.round(sum);});
  return Object.freeze({windowsSeconds:Object.freeze([...input.windowsSeconds]),totals:Object.freeze(totals)});
 }catch{return null;}
}
