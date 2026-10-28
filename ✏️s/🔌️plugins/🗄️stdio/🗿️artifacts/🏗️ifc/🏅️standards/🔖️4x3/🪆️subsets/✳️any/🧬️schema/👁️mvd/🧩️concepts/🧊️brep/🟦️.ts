/** 🧊️ Execute exact released basic Body Brep representation clauses under explicit applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodyBrepRequest,IfcBodyBrepReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodyBrepDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodyBrepItemLabel=Object.freeze(["A representation item is not a faceted boundary representation solid.","Ein Repräsentationselement ist kein facettierter BRep-Körper."] as const);
/** 🧾 Preserve selected faceted-family diagnostics without topology, schema or geometry admission. */
export async function validateIfcBodyBrep(source:IfcSnapshot,request:IfcBodyBrepRequest,control:IfcIoControl={}):Promise<IfcBodyBrepReport>{return validateIfcBodyConcept(source,request,control,ifcBodyBrepDefinition,ifcBodyBrepItemLabel);}
