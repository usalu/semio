/** 🎨️ Owned complex-color variants with optional book and name metadata. */
import type { DwgComplexColor } from "../../../🟦️.ts";
import { DwgProjection,dwgInteger,type DwgCell } from "../🔢️number/🟦️.ts";
import { DwgReader,dwgByte,dwgWord,dwgOptionalText } from "../🫳️reader/🟦️.ts";
import { dwgRequireNull } from "../🏷️xrecord/🟦️.ts";
export async function dwgProjectColor(p:DwgProjection,table:string,id:bigint,color:DwgComplexColor):Promise<void>{
  let kind:string,red:DwgCell=null,green:DwgCell=null,blue:DwgCell=null,index:DwgCell=null;const value=color.value;
  switch(value.kind){
    case"none":kind="none";break;case"byLayer":kind="by_layer";break;case"byBlock":kind="by_block";break;
    case"foreground":kind="foreground";break;case"layerOff":kind="layer_off";break;case"layerFrozen":kind="layer_frozen";break;
    case"byColor":kind="by_color";red=dwgInteger(value.red,0,255);green=dwgInteger(value.green,0,255);blue=dwgInteger(value.blue,0,255);break;
    case"byAci":kind="by_aci";index=dwgInteger(value.index,0,65535);break;
    case"byPen":kind="by_pen";index=dwgInteger(value.index,0,255);break;
    default:throw new Error("DWG complex color kind is unknown");
  }
  await p.insert(table,[dwgInteger(color.index,0,65535),kind,red,green,blue,index,color.name??null,color.bookName??null],id);
}
export async function dwgReconstructColor(reader:DwgReader,table:string,id:bigint):Promise<DwgComplexColor>{
  const row=await reader.component(table,id);let value:DwgComplexColor["value"];
  switch(row.text(2)){
    case"by_color":dwgRequireNull(row,[6]);value={kind:"byColor",red:dwgByte(row,3),green:dwgByte(row,4),blue:dwgByte(row,5)};break;
    case"by_aci":dwgRequireNull(row,[3,4,5]);value={kind:"byAci",index:dwgWord(row,6)};break;
    case"by_pen":dwgRequireNull(row,[3,4,5]);value={kind:"byPen",index:dwgByte(row,6)};break;
    case"none":dwgRequireNull(row,[3,4,5,6]);value={kind:"none"};break;
    case"by_layer":dwgRequireNull(row,[3,4,5,6]);value={kind:"byLayer"};break;
    case"by_block":dwgRequireNull(row,[3,4,5,6]);value={kind:"byBlock"};break;
    case"foreground":dwgRequireNull(row,[3,4,5,6]);value={kind:"foreground"};break;
    case"layer_off":dwgRequireNull(row,[3,4,5,6]);value={kind:"layerOff"};break;
    case"layer_frozen":dwgRequireNull(row,[3,4,5,6]);value={kind:"layerFrozen"};break;
    default:throw new Error("DWG complex color kind is unknown");
  }
  return{index:dwgWord(row,1),value,name:dwgOptionalText(row,7),bookName:dwgOptionalText(row,8)};
}
