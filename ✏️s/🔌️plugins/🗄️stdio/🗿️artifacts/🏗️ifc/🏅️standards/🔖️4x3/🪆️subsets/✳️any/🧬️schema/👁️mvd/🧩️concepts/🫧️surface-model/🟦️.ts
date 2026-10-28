/** 🫧️ Execute only the released selected Body SurfaceModel clauses under explicit applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodySurfaceModelRequest,IfcBodySurfaceModelReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodySurfaceModelDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodySurfaceModelItemLabel=Object.freeze(["A representation item is neither a tessellated item nor a shell-based or face-based surface model.","Ein Repräsentationselement ist weder ein tesselliertes Element noch ein schalen- oder flächenbasiertes Oberflächenmodell."] as const);
/** 🧾 Preserve source/query uncertainty without claiming schema, geometry or view admission. */
export async function validateIfcBodySurfaceModel(source:IfcSnapshot,request:IfcBodySurfaceModelRequest,control:IfcIoControl={}):Promise<IfcBodySurfaceModelReport>{return validateIfcBodyConcept(source,request,control,ifcBodySurfaceModelDefinition,ifcBodySurfaceModelItemLabel);}
