/** 🌪️ Evaluate the exact released Body AdvancedSweptSolid concept under explicit applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodyAdvancedSweptSolidRequest,IfcBodyAdvancedSweptSolidReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodyAdvancedSweptSolidDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodyAdvancedSweptSolidItemLabel=Object.freeze(["A representation item is neither a swept area solid nor a swept disk solid.","Ein Repräsentationselement ist weder ein Flächen-Sweep-Körper noch ein Kreis-Sweep-Körper."] as const);
/** 🧾 Report only three selected clauses against private captured data, without schema or view admission. */
export async function validateIfcBodyAdvancedSweptSolid(source:IfcSnapshot,request:IfcBodyAdvancedSweptSolidRequest,control:IfcIoControl={}):Promise<IfcBodyAdvancedSweptSolidReport>{return validateIfcBodyConcept(source,request,control,ifcBodyAdvancedSweptSolidDefinition,ifcBodyAdvancedSweptSolidItemLabel);}
