/** ✂️ Execute only three released Body Clipping representation clauses under explicit applicability. */
import definition from './definition.json';
import {freezeIfcBodyDefinition,validateIfcBodyConcept} from '../🧱️body/🟦️.ts';
import type {IfcBodyConceptDefinition} from '../🧱️body/types.ts';
import type {IfcBodyClippingRequest,IfcBodyClippingReport} from './types.ts';
import type {IfcSnapshot} from '../../../📸️snapshot/🟦️.ts';
import type {IfcIoControl} from '../../../../🚪️io/control.ts';
export type * from './types.ts';
export const ifcBodyClippingDefinition=freezeIfcBodyDefinition(definition as IfcBodyConceptDefinition);
export const ifcBodyClippingItemLabel=Object.freeze(["A representation item is not a boolean clipping result.","Ein Repräsentationselement ist kein boolesches Beschneidungsergebnis."] as const);
/** 🧾 Retain exact selected concept diagnostics without schema operand/operator or geometry admission. */
export async function validateIfcBodyClipping(source:IfcSnapshot,request:IfcBodyClippingRequest,control:IfcIoControl={}):Promise<IfcBodyClippingReport>{return validateIfcBodyConcept(source,request,control,ifcBodyClippingDefinition,ifcBodyClippingItemLabel);}
