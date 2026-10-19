import {binary64Value} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {parseBinary64Transport} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/📝️text/🔢️ieee754/🟦️.ts";
import type {EnergyModelMutation} from "../🟦️.ts";
import type {EnergyModelDiff} from "../../🔺️diff/🟦️.ts";
import type {EnergyGlazingMaterial} from "../../📸️snapshot/⚡️model/🟦️.ts";
export type EnergyGlazingOpticalMutation=Extract<EnergyModelMutation,{mutation:"changeGlazingMaterialSolarReflectanceFront"|"changeGlazingMaterialSolarReflectanceBack"|"changeGlazingMaterialVisibleReflectanceFront"|"changeGlazingMaterialVisibleReflectanceBack"|"changeGlazingMaterialInfraredTransmittance"}>;
const fields={changeGlazingMaterialSolarReflectanceFront:{field:"solar_reflectance_front",value:"newSolarReflectanceFront"},changeGlazingMaterialSolarReflectanceBack:{field:"solar_reflectance_back",value:"newSolarReflectanceBack"},changeGlazingMaterialVisibleReflectanceFront:{field:"visible_reflectance_front",value:"newVisibleReflectanceFront"},changeGlazingMaterialVisibleReflectanceBack:{field:"visible_reflectance_back",value:"newVisibleReflectanceBack"},changeGlazingMaterialInfraredTransmittance:{field:"infrared_transmittance",value:"newInfraredTransmittance"}} as const;
/** 🪟️ Plans an optical field intent and its base-derived inverse without replacing a snapshot. */
export function planGlazingOpticalMutation(base:{readonly model:{readonly glazing_materials:readonly EnergyGlazingMaterial[]}},mutation:EnergyGlazingOpticalMutation):{outcome:"applied"|"no-op"|"rejected";diff:EnergyModelDiff;inverse:EnergyGlazingOpticalMutation[]}{
 const intent=fields[mutation.mutation];const original=base.model.glazing_materials.find(row=>row.id===mutation.id);const value=(mutation as unknown as Record<string,unknown>)[intent.value];
 if(!original||typeof value!=="number"||!Number.isFinite(value)||value<0||value>1)return{outcome:"rejected",diff:{},inverse:[]};
 const word=parseBinary64Transport(value);const originalValue=binary64Value(parseBinary64Transport(original[intent.field]));
 if(originalValue===value)return{outcome:"no-op",diff:{},inverse:[]};
 return{outcome:"applied",diff:{model:{glazing_materials:{modified:[{id:mutation.id,[intent.field]:word}]}}},inverse:[inverseGlazingOpticalMutation(mutation,originalValue)]};
}

/** ↩️ Captures the original scalar in the same typed field intent. */
function inverseGlazingOpticalMutation(mutation:EnergyGlazingOpticalMutation,value:number):EnergyGlazingOpticalMutation{
 switch(mutation.mutation){
  case "changeGlazingMaterialSolarReflectanceFront":return{mutation:mutation.mutation,id:mutation.id,newSolarReflectanceFront:value};
  case "changeGlazingMaterialSolarReflectanceBack":return{mutation:mutation.mutation,id:mutation.id,newSolarReflectanceBack:value};
  case "changeGlazingMaterialVisibleReflectanceFront":return{mutation:mutation.mutation,id:mutation.id,newVisibleReflectanceFront:value};
  case "changeGlazingMaterialVisibleReflectanceBack":return{mutation:mutation.mutation,id:mutation.id,newVisibleReflectanceBack:value};
  case "changeGlazingMaterialInfraredTransmittance":return{mutation:mutation.mutation,id:mutation.id,newInfraredTransmittance:value};
 }
}
