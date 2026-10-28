/** 🧭️ Execute the released selected Body Tessellation clauses under explicit caller applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodyTessellationRequest,IfcBodyTessellationReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodyTessellationDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodyTessellationItemLabel=Object.freeze(["A representation item is not a tessellated face set.","Ein Repräsentationselement ist kein tessellierter Flächensatz."] as const);
/** 🧾 Report only the selected released representation clauses against privately captured source. */
export async function validateIfcBodyTessellation(source:IfcSnapshot,request:IfcBodyTessellationRequest,control:IfcIoControl={}):Promise<IfcBodyTessellationReport>{return validateIfcBodyConcept(source,request,control,ifcBodyTessellationDefinition,ifcBodyTessellationItemLabel);}
