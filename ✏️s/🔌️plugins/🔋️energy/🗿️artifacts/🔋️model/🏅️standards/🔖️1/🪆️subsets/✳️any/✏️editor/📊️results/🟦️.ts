/** 📊️ Prepared ESF2 rows expose original indexed color reads and explicit localized result legends. */
export interface IndexedResultBytes{readonly complete:boolean;readonly totalBytes:number;read(offset:number,count:number):Uint8Array|undefined}
export type ResultField="conductionLoss"|"conductionGain"|"solarTransmitted"|"solarAbsorbed";
export type ResultLocale="en"|"de";
export const RESULT_FIELDS=["conductionLoss","conductionGain","solarTransmitted","solarAbsorbed"] as const;
export const RESULT_BANDS=["#1d4ed8","#2563eb","#0ea5e9","#22c55e","#eab308","#f97316","#ef4444","#b91c1c"] as const;
const labels={en:["Conduction loss","Conduction gain","Solar transmitted","Solar absorbed"],de:["Wärmeleitungsverlust","Wärmeleitungsgewinn","Solare Transmission","Solare Absorption"]}as const;
function word(source:IndexedResultBytes,offset:number):number|undefined{const bytes=source.read(offset,4);return bytes===undefined?undefined:new DataView(bytes.buffer,bytes.byteOffset,4).getUint32(0,true);}
/** 🪪️ The complete publication header is fixed metadata; per-face paint only reads the prepared original row. */
export class IndexedEnergyResults{
 readonly counts:readonly[number,number];readonly bounds:readonly(readonly[number,number])[];
 constructor(readonly source:IndexedResultBytes){
  const magic=source.read(0,4),opaque=word(source,4),windows=word(source,8);
  if(!source.complete||!magic||String.fromCharCode(...magic)!=="ESF2"||opaque===undefined||windows===undefined||source.totalBytes!==76+(opaque+windows)*24)throw Error("invalid-energy-results-header");
  this.counts=[opaque,windows];const bounds:(readonly[number,number])[]=[];
  for(let field=0;field<4;field++){const bytes=source.read(12+field*16,16);if(!bytes)throw Error("invalid-energy-results-bounds");const view=new DataView(bytes.buffer,bytes.byteOffset,16),low=view.getFloat64(0,true),high=view.getFloat64(8,true);if(!Number.isFinite(low)||!Number.isFinite(high)||low>high)throw Error("invalid-energy-results-bounds");bounds.push([low,high]);}this.bounds=bounds;
 }
 color(id:number,field:ResultField):string|undefined{
  const column=RESULT_FIELDS.indexOf(field);if(column<0||!Number.isSafeInteger(id)||id<0||id>0xffffffff)return undefined;
  let start=0;
  for(const count of this.counts){let low=0,high=count;while(low<high){const middle=low+Math.floor((high-low)/2),candidate=word(this.source,76+(start+middle)*24);if(candidate===undefined)return undefined;if(candidate<id)low=middle+1;else high=middle;}if(low<count&&word(this.source,76+(start+low)*24)===id){const band=this.source.read(76+(start+low)*24+20+column,1)?.[0];return band===undefined||band>7?undefined:RESULT_BANDS[band];}start+=count;}return undefined;
 }
 caption(field:ResultField,locale:ResultLocale):string{const index=RESULT_FIELDS.indexOf(field),[low,high]=this.bounds[index]!;const number=(value:number)=>locale==="de"?value.toFixed(1).replace(".",","):value.toFixed(1);return`${labels[locale][index]} · ${number(low)} – ${number(high)} kWh`;}
 scaleLabel(locale:ResultLocale):string{return locale==="en"?"Result scale, low to high":"Ergebnisskala, niedrig bis hoch";}
}
