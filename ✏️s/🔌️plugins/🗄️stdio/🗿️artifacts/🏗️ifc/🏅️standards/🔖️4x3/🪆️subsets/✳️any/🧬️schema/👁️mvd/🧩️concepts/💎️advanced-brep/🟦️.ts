/** 💎️ Execute exact released Body AdvancedBrep alternatives under explicit applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodyAdvancedBrepRequest,IfcBodyAdvancedBrepReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodyAdvancedBrepDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodyAdvancedBrepItemLabel=Object.freeze(["A representation item is neither an advanced nor a faceted boundary representation solid.","Ein Repräsentationselement ist weder ein erweiterter noch ein facettierter BRep-Körper."] as const);
/** 🧾 Retain selected advanced/faceted-family diagnostics without topology, schema or geometry admission. */
export async function validateIfcBodyAdvancedBrep(source:IfcSnapshot,request:IfcBodyAdvancedBrepRequest,control:IfcIoControl={}):Promise<IfcBodyAdvancedBrepReport>{return validateIfcBodyConcept(source,request,control,ifcBodyAdvancedBrepDefinition,ifcBodyAdvancedBrepItemLabel);}
