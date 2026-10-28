import type {IfcValue} from '../../📸️snapshot/🟦️.ts';
export type IfcTemplateReference=Extract<IfcValue,{kind:'reference'}>;
interface Selected {name:string;description?:string}
interface Measured extends Selected {unit?:IfcTemplateReference}
export type IfcTemplateSelection=
 | Measured&{kind:'P_SINGLEVALUE';value:IfcValue}
 | Selected&{kind:'P_REFERENCEVALUE';value:IfcTemplateReference;usageName?:string}
 | Measured&{kind:'P_ENUMERATEDVALUE';values:IfcValue[]}
 | Measured&{kind:'P_LISTVALUE';values:IfcValue[]}
 | Measured&{kind:'P_BOUNDEDVALUE';lower?:IfcValue;upper?:IfcValue;setPoint?:IfcValue}
 | Selected&{kind:'P_TABLEVALUE';defining:IfcValue[];defined:IfcValue[];definingUnit?:IfcTemplateReference;definedUnit?:IfcTemplateReference;expression?:string;interpolation?:'LINEAR'|'LOG_LINEAR'|'LOG_LOG'|'NOTDEFINED'}
 | Measured&{kind:'Q_LENGTH'|'Q_AREA'|'Q_VOLUME'|'Q_WEIGHT'|'Q_COUNT'|'Q_TIME';value:IfcValue;formula?:string};
/** 🪪️ Identities are command data, never regenerated while replaying or undoing. */
export interface IfcTemplateIdentity {key:string;id:bigint;guid?:string}
/** ✍️ Selected published template entries with explicit retained identities and lifecycle. */
export interface IfcTemplateApplication {
 template:string;
 target:bigint;
 entries:IfcTemplateSelection[];
 identities:IfcTemplateIdentity[];
 ownerHistory?:IfcTemplateReference;
 description?:string;
 methodOfMeasurement?:string;
 performance?:{lifeCyclePhase:string;predefinedType:'USERDEFINED'|'NOTDEFINED';objectType?:string;identification?:string;name?:string;description?:string};
}
