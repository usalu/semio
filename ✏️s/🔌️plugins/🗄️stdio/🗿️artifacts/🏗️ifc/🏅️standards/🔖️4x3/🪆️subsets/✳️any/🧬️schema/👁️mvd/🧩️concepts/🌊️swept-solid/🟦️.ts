/** 🧭️ Execute the released selected Body SweptSolid clauses under explicit caller applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodySweptSolidRequest,IfcBodySweptSolidReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodySweptSolidDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodySweptSolidItemLabel=Object.freeze(["A representation item is not a basic extruded or revolved area solid.","Ein Repräsentationselement ist kein einfacher extrudierter oder gedrehter Flächenkörper."] as const);
/** 🧾 Report only the selected released representation clauses against privately captured source. */
export async function validateIfcBodySweptSolid(source:IfcSnapshot,request:IfcBodySweptSolidRequest,control:IfcIoControl={}):Promise<IfcBodySweptSolidReport>{return validateIfcBodyConcept(source,request,control,ifcBodySweptSolidDefinition,ifcBodySweptSolidItemLabel);}
