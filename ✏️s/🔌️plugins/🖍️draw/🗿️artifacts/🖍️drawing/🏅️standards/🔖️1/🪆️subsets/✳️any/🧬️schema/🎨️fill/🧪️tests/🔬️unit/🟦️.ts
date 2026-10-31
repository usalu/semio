/** 🎨️ Fill edits follow shared cases and an independent Immer document update. */
import { expect, test, spyOn } from "bun:test";
import Ajv from "ajv";
import { produce } from "immer";
import { Vector4 } from "three";
import { editFill } from "../../🟦️.ts";
import type { FillEdit } from "../../🟦️.ts";
import type { Fill } from "../../🖌️paint-model/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import cases from "../../../../🧪️tests/🧫️fixtures/🎨️fill/🔣️.json";
test("fill editing preserves stops, geometry, alpha and source on rejection", () => {
  const validate = new Ajv({strict:true}).compile(schema);
  for (const entry of cases) {
    expect(validate(entry.edit)).toBe(true);
    const source = structuredClone(entry.before) as Fill | null;
    if (entry.error) expect(() => editFill(source, entry.edit as FillEdit)).toThrow();
    else {
      const result = editFill(source, entry.edit as FillEdit);
      expect<typeof entry.after>(result).toEqual(entry.after);
      if (entry.edit.kind === "addStop" && source && "stops" in source && result && "stops" in result) {
        const [left,right] = source.stops;
        const color = new Vector4(...left!.color).lerp(new Vector4(...right!.color), (entry.edit.offset!-left!.offset)/(right!.offset-left!.offset)).toArray();
        expect(result.stops.find(stop => stop.offset === entry.edit.offset)!.color).toEqual(color);
      }
      if (["alpha","offset","removeStop"].includes(entry.edit.kind)) {
        const oracle = produce({fill:source}, draft => {
          const fill=draft.fill!;
          if(entry.edit.kind==="alpha") {
            const color=fill.kind==="solid" ? fill.color : fill.stops[entry.edit.index!]!.color;
            color[3]=entry.edit.value as number;
          } else if("stops" in fill && entry.edit.kind==="offset") {
            fill.stops[entry.edit.index!]!.offset=entry.edit.value as number;
            fill.stops.sort((a,b)=>a.offset-b.offset);
          } else if("stops" in fill) fill.stops.splice(entry.edit.index!,1);
        });
        expect(result).toEqual(oracle.fill);
      }
    }
    expect<typeof entry.before>(source).toEqual(entry.before);
  }
});

import sharp from "sharp";
import { PreparedFill, GradientRamp, sampleFill, sampleGradientStops } from "../../🎨️sampling/🟦️.ts";
import retirementCases from "../../../../🧪️tests/🧫️fixtures/🎨️fill/🎨️sampling/🧹️retirement/🔣️.json";
import { retireSchema as retirementSchema } from "@semio-tech/framework/2d/retire/schema";
import samplingCases from "../../../../🧪️tests/🧫️fixtures/🎨️fill/🎨️sampling/🔣️.json";

/** 🧭️ SVG 1.1 degenerate gradients use their last stop; normalize these before the librsvg oracle. */
function referencePaint(fill:Fill,box:readonly number[]=[0,0,120,24]):string {
  if(fill.kind!=="solid" && (fill.kind==="linearGradient" ? fill.x1===fill.x2&&fill.y1===fill.y2 : fill.r===0)) {
    const last=[...fill.stops].sort((a,b)=>a.offset-b.offset).at(-1);
    return referencePaint({kind:"solid",color:last?.color??[0,0,0,0]},box);
  }
  const bounds=`x="${box[0]}" y="${box[1]}" width="${box[2]}" height="${box[3]}"`;
  const rgb=(color:number[])=>`rgb(${color.slice(0,3).map(value=>value*255).join(",")})`;
  if(fill.kind==="solid") return `<rect ${bounds} fill="${rgb(fill.color)}" fill-opacity="${fill.color[3]}"/>`;
  const tag=fill.kind==="linearGradient" ? "linearGradient" : "radialGradient";
  const coordinates=fill.kind==="linearGradient" ? `x1="${fill.x1}" y1="${fill.y1}" x2="${fill.x2}" y2="${fill.y2}"` : `cx="${fill.cx}" cy="${fill.cy}" r="${fill.r}"`;
  const stops=[...fill.stops].sort((a,b)=>a.offset-b.offset).map(stop=>`<stop offset="${stop.offset}" stop-color="${rgb(stop.color)}" stop-opacity="${stop.color[3]}"/>`).join("");
  return `<defs><${tag} id="paint" gradientUnits="userSpaceOnUse" ${coordinates}>${stops}</${tag}></defs><rect ${bounds} fill="url(#paint)"/>`;
}

import {Bank} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧪️tests/🎟️work/🟦️ts/funded.test.ts";
import {RetainedWorkBuffer,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
function tuple(value:readonly number[]):[number,number]{const[x,y]=value;if(x===undefined||y===undefined)throw Error("Original sample coordinate absent");return[x,y];}
function prepare(paint:PreparedFill,bank:Bank,quantum=1):void{const turns=bank.cpu;for(let at=0;at<turns;at++)if(bank.turn(paint,work=>paint.advance(quantum,work),quantum).done)return;throw Error("Prepared fill exceeded original finite fixture turn bound");}
function prepareRamp(ramp:GradientRamp,bank:Bank,quantum=1):void{const turns=bank.cpu;for(let at=0;at<turns;at++)if(bank.turn(ramp,work=>ramp.advance(quantum,work),quantum).done)return;throw Error("Gradient ramp exceeded original finite fixture turn bound");}
function reject(fill:Fill):void{const bank=new Bank(),paint=new PreparedFill(fill);try{prepare(paint,bank);}finally{bank.close(paint.intoRetirement());bank.conserved();}}
test("prepared fill sampling matches shared cases and independent SVG pixels",async()=>{let compared=0;for(const entry of samplingCases){const fill=structuredClone(entry.fill)as Fill;if(entry.error)expect(()=>reject(fill)).toThrow();else{const bank=new Bank(),paint=new PreparedFill(fill);try{prepare(paint,bank);if(entry.samples===undefined)throw Error("Original paint samples absent");for(const sample of entry.samples){const point=tuple(sample.point),color=paint.sample(point);for(let channel=0;channel<4;channel++)expect(color[channel]).toBeCloseTo(sample.color[channel]!,12);const[x,y]=point,svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="${x} ${y} 1 1">${referencePaint(fill,[x,y,1,1])}</svg>`,pixel=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer(),expected=paint.sample([x+.5,y+.5]);expect(Math.abs(pixel[3]!/255-expected[3]),` ${entry.name}: alpha ${pixel} vs ${color}`).toBeLessThanOrEqual(2/255);for(let channel=0;channel<3;channel++)expect(Math.abs(pixel[channel]!*pixel[3]!/255-expected[channel]!*expected[3]*255),` ${entry.name}: rgb ${pixel} vs ${color}`).toBeLessThanOrEqual(2);compared++;}}finally{bank.close(paint.intoRetirement());bank.conserved();}}expect<typeof entry.fill>(fill).toEqual(entry.fill);}expect(compared).toBeGreaterThan(0);});
test("prepared fills own source stops and reject nonfinite paint or sample coordinates",()=>{const fill:Fill={kind:"linearGradient",x1:0,y1:0,x2:100,y2:0,stops:[{offset:0,color:[1,0,0,1]},{offset:1,color:[0,0,1,1]}]},bank=new Bank(),paint=new PreparedFill(fill);try{prepare(paint,bank);fill.stops[0]!.color[0]=0;expect(paint.sample([0,0])).toEqual([1,0,0,1]);for(const invalid of [NaN,Infinity,-Infinity]){expect(()=>paint.sample([invalid,0])).toThrow();expect(()=>reject({...fill,x1:invalid})).toThrow();expect(()=>reject({kind:"solid",color:[0,0,0,invalid]})).toThrow();}}finally{bank.close(paint.intoRetirement());bank.conserved();}});
test("inserting a gradient stop preserves independently rasterized appearance",async()=>{
  let compared=0;
  for(const entry of samplingCases.filter(entry=>!entry.error&&entry.fill.kind!=="solid"&&entry.fill.stops?.length)) {
    const source=entry.fill as Fill;
    const render=async(fill:Fill)=>sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="120" height="24" viewBox="0 0 120 24">${referencePaint(fill)}</svg>`)).ensureAlpha().raw().toBuffer();
    const before=await render(source);
    expect(before.some((value,index)=>index%4===3&&value>0),entry.name).toBe(true);
    for(const offset of [0.25,0.5,0.75]) {
      const result=editFill(source,{kind:"addStop",offset})!;
      const after=await render(result);
      let maximumError=0;
      for(let pixel=0;pixel<before.length;pixel+=4) {
        maximumError=Math.max(maximumError,Math.abs(before[pixel+3]!-after[pixel+3]!));
        for(let channel=0;channel<3;channel++) maximumError=Math.max(maximumError,Math.abs(before[pixel+channel]!*before[pixel+3]!/255-after[pixel+channel]!*after[pixel+3]!/255));
      }
      expect(maximumError,`${entry.name}: inserted stop ${offset}`).toBeLessThanOrEqual(2);
      compared++;
    }
  }
});

test("prepared paint retirement drains actual copied stop owners and composes the real ramp",async()=>{const validProgress=new Ajv({strict:true}).compile(retirementSchema);let comparisons=0;for(const row of retirementCases)for(const grant of[1,7,4096]){const source=samplingCases.find(sample=>sample.name===row.source);if(source===undefined||source.samples===undefined)throw Error("Original paint retirement source absent");const fill=structuredClone(source.fill)as Fill;if(row.repeat&&fill.kind!=="solid"){const stop=fill.stops[0];if(stop===undefined)throw Error("Original repeated stop absent");fill.stops=Array.from({length:row.repeat},()=>structuredClone(stop));}const before=structuredClone(fill),bank=new Bank(),paint=new PreparedFill(fill);prepare(paint,bank,grant);const samples=source.samples.map(sample=>paint.sample(tuple(sample.point))),children:{owner:RetainedWorkRetirementCursor;work:number;rows:RetainedWorkBuffer<unknown>;scratch:RetainedWorkBuffer<unknown>;count:number}[]=[],original=GradientRamp.prototype.intoRetirement;
const adopt=spyOn(GradientRamp.prototype,"intoRetirement").mockImplementation(function(this:GradientRamp){const rows:unknown=Reflect.get(this,"stops"),scratch:unknown=Reflect.get(this,"scratch");if(!(rows instanceof RetainedWorkBuffer)||!(scratch instanceof RetainedWorkBuffer))throw Error("Actual copied ramp pages absent");const count=rows.length;for(let at=0;at<count;at++){const stop=rows.at(at);if(typeof stop!=="object"||stop===null||!("color"in stop))throw Error("Actual owned gradient stop absent");if(fill.kind!=="solid"){expect(fill.stops.some(source=>source===stop)).toBe(false);expect(fill.stops.some(source=>source.color===stop.color)).toBe(false);}}const owner=original.call(this),record={owner,work:0,rows,scratch,count};children.push(record);const prior=owner.advance.bind(owner);owner.advance=(items,work)=>{expect(items).toBe(1);const before=rows.length+scratch.length,progress=prior(items,work);expect(progress.work-record.work).toBe(owner.lastTurnWork());expect(owner.lastTurnWork()).toBeLessThanOrEqual(items);expect(before-rows.length-scratch.length).toBeLessThanOrEqual(owner.lastTurnWork());record.work=progress.work;return progress;};return owner;});
let retired:RetainedWorkRetirementCursor;try{retired=paint.intoRetirement();}finally{adopt.mockRestore();}expect(()=>paint.sample([0,0])).toThrow(/transferred/);expect(()=>paint.constantColor()).toThrow(/transferred/);expect(()=>paint.intoRetirement()).toThrow(/transferred/);for(const bad of[0,-1,.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])expect(()=>bank.turn(retired,work=>retired.advance(bad,work))).toThrow(/grant/);
let work=0;for(let at=0;at<100000;at++){const progress=bank.turn(retired,ledger=>retired.advance(grant,ledger),grant);expect(validProgress({phase:progress.done?"complete":"closing",...progress})).toBe(true);expect(progress.work-work).toBe(retired.lastTurnWork());expect(progress.work-work).toBeLessThanOrEqual(grant);if(!progress.done)expect(progress.work-work).toBeGreaterThan(0);work=progress.work;if(progress.done)break;}expect(retired.terminalIsEmpty()).toBe(true);expect(bank.turn(retired,ledger=>retired.advance(1,ledger))).toEqual({work,done:true});expect(retired.lastTurnWork()).toBe(0);expect(fill).toEqual(before);expect(children).toHaveLength(fill.kind==="solid"?0:1);const ownedCount=children[0]?.count??0;expect(ownedCount).toBe(fill.kind==="solid"?0:fill.stops.length);expect(ownedCount+(fill.kind==="solid"?2:4)).toBe(row.work.prepared.typescript);for(const child of children){expect(child.count+2).toBe(row.work.ramp.typescript);expect(child.rows.length).toBe(0);expect(child.scratch.length).toBe(0);expect(child.owner.terminalIsEmpty()).toBe(true);}source.samples.forEach((sample,at)=>sample.color.forEach((value,c)=>expect(samples[at]![c]).toBeCloseTo(value,12)));
const sample=source.samples[0];if(sample===undefined)throw Error("Original paint comparison absent");const[x,y]=tuple(sample.point),pixel=await sharp(Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1" viewBox="${x} ${y} 1 1">${referencePaint(fill,[x,y,1,1])}</svg>`)).ensureAlpha().raw().toBuffer(),expected=sampleFill(fill,[x+.5,y+.5]);expect(Math.abs(pixel[3]!/255-expected[3])).toBeLessThanOrEqual(2/255);for(let c=0;c<3;c++)expect(Math.abs(pixel[c]!*pixel[3]!/255-expected[c]!*expected[3]*255)).toBeLessThanOrEqual(2);comparisons++;
if(fill.kind!=="solid"){const ramp=new GradientRamp(fill.stops);prepareRamp(ramp,bank,grant);const color=ramp.sample(.5),owner=ramp.intoRetirement();expect(()=>ramp.sample(.5)).toThrow(/transferred/);expect(()=>ramp.constantColor()).toThrow(/transferred/);expect(()=>ramp.intoRetirement()).toThrow(/transferred/);let work=0;for(let at=0;at<100000;at++){const progress=bank.turn(owner,ledger=>owner.advance(grant,ledger),grant);expect(validProgress({phase:progress.done?"complete":"closing",...progress})).toBe(true);expect(progress.work-work).toBe(owner.lastTurnWork());expect(progress.work-work).toBeLessThanOrEqual(grant);work=progress.work;if(progress.done)break;}expect(owner.terminalIsEmpty()).toBe(true);expect(color).toEqual(sampleGradientStops(fill.stops,.5));}bank.conserved();}expect(comparisons).toBeGreaterThan(0);});
