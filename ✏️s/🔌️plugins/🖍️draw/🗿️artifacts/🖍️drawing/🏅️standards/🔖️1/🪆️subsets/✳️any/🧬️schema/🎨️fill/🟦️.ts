/** 🎨️ Owned fill edits shared by inspector controls and gradient handles. */
export type Color = [number,number,number,number];
export interface Stop { offset:number; color:Color }
export type Fill = {kind:"solid";color:Color} | {kind:"linearGradient";x1:number;y1:number;x2:number;y2:number;stops:Stop[]} | {kind:"radialGradient";cx:number;cy:number;r:number;stops:Stop[]};
export type FillEdit = {kind:"type";value:"none"|Fill["kind"]} | {kind:"coordinate";axis:"x1"|"y1"|"x2"|"y2"|"cx"|"cy"|"r";value:number} | {kind:"color";index?:number;value:string} | {kind:"alpha";index?:number;value:number} | {kind:"offset";index:number;value:number} | {kind:"addStop";offset:number} | {kind:"removeStop";index:number};
const invalid = (message:string):never => { throw new Error(message); };
const unit = (value:number):number => Number.isFinite(value) && value>=0 && value<=1 ? value : invalid("Value must be between zero and one");
export function editFill(source:Fill|null, edit:FillEdit):Fill|null {
  if (source && "stops" in source && (source.stops.length>64 || source.stops.some(stop => !Number.isFinite(stop.offset) || stop.offset<0 || stop.offset>1 || stop.color.some(value => !Number.isFinite(value) || value<0 || value>1)))) invalid("Invalid gradient stops");
  if (edit.kind==="type") {
    const color:Color = source?.kind==="solid" ? [...source.color] : source?.stops[0] ? [...source.stops[0].color] : [0,0,0,1];
    const stops:Stop[] = source && "stops" in source && source.stops.length ? structuredClone(source.stops) : [{offset:0,color},{offset:1,color:[1,1,1,color[3]]}];
    if (edit.value==="none") return null;
    if (source?.kind===edit.value) return structuredClone(source);
    if (edit.value==="solid") return {kind:"solid",color};
    return edit.value==="linearGradient" ? {kind:"linearGradient",x1:0,y1:0,x2:100,y2:0,stops} : {kind:"radialGradient",cx:50,cy:50,r:50,stops};
  }
  if (!source) return invalid("Enable a fill first");
  const fill=structuredClone(source);
  const stops=():Stop[] => "stops" in fill ? fill.stops : invalid("Select a gradient fill");
  const stop=(index:number):Stop => Number.isInteger(index) && index>=0 && stops()[index] ? stops()[index]! : invalid("Missing gradient stop");
  const color=(index?:number):Color => fill.kind==="solid" && index===undefined ? fill.color : index!==undefined ? stop(index).color : invalid("Select a gradient stop");
  switch(edit.kind) {
    case "coordinate":
      if (!Number.isFinite(edit.value) || (edit.axis==="r" && edit.value<=0) || !(edit.axis in fill)) invalid("Invalid gradient coordinate");
      (fill as unknown as Record<string,unknown>)[edit.axis]=edit.value; break;
    case "color": {
      if (!/^#[0-9a-fA-F]{3}([0-9a-fA-F]{3})?$/.test(edit.value)) invalid("Invalid color");
      const target=color(edit.index),hex=edit.value.length===4 ? [...edit.value.slice(1)].map(value=>value+value).join("") : edit.value.slice(1);
      for(let index=0;index<3;index++) target[index]=parseInt(hex.slice(index*2,index*2+2),16)/255;
      break;
    }
    case "alpha": color(edit.index)[3]=unit(edit.value); break;
    case "offset": stop(edit.index).offset=unit(edit.value); stops().sort((a,b)=>a.offset-b.offset); break;
    case "removeStop": if(stops().length<=2) invalid("A gradient needs at least two stops"); stop(edit.index); stops().splice(edit.index,1); break;
    case "addStop": {
      const offset=unit(edit.offset),items=stops();
      if(!items.length || items.length>=64) invalid("Cannot add another gradient stop");
      items.sort((a,b)=>a.offset-b.offset);
      let right=items.findIndex(stop=>stop.offset>offset); if(right<0) right=items.length;
      const left=items[Math.max(0,right-1)]!,next=items[Math.min(right,items.length-1)]!,t=next.offset>left.offset ? Math.max(0,Math.min(1,(offset-left.offset)/(next.offset-left.offset))) : 0;
      items.splice(right,0,{offset,color:left.color.map((value,index)=>value+(next.color[index]!-value)*t) as Color}); break;
    }
  }
  return fill;
}
