import type {ChildAdmissionSource,ChildAdmissionGrant,ChildAdmissionDemand,ChildAdmissionProgress} from '../🟦️.ts';
import type {VectorSegment,VectorCanvas} from '../../🧵️lower/🟦️.ts';
export type VectorFillHead={kind:'solid';color:readonly number[]}|{kind:'linear';x1:number;y1:number;x2:number;y2:number}|{kind:'radial';cx:number;cy:number;r:number};
export interface VectorGradientStop{offset:number;color:readonly number[]}
export interface VectorStrokeHead{color:readonly number[];width:number;cap:string;join:string}
export interface VectorPaintSourceV1{fillRule():string;fill():VectorFillHead|undefined;stopCount():number;stop(index:number):VectorGradientStop|undefined;stroke():VectorStrokeHead|undefined;dashCount():number;dash(index:number):number|undefined}
export interface VectorPathSourceV1{segmentCount():number;segment(index:number):VectorSegment|undefined}
export interface VectorPolygonSourceV1{pointCount():number;point(index:number):readonly number[]|undefined}
export type VectorShapeRef={kind:'rect';x:number;y:number;width:number;height:number}|{kind:'ellipse';cx:number;cy:number;rx:number;ry:number}|{kind:'circle';cx:number;cy:number;r:number}|{kind:'line';x1:number;y1:number;x2:number;y2:number}|{kind:'polygon';source:VectorPolygonSourceV1};
export interface VectorShapeSourceV1{primitive():VectorShapeRef|undefined}
export interface VectorRgbaSourceV1{width():number;height():number;sampleCount():number;sample(index:number):readonly number[]|undefined}
export interface VectorAssetRef{identity:object;key:string;samples:VectorRgbaSourceV1}
export type VectorContentRef={kind:'path';source:VectorPathSourceV1}|{kind:'shape';source:VectorShapeSourceV1}|{kind:'text';content:string;x:number;y:number;size:number;fontFamily:string}|{kind:'image';key:string;width:number;height:number}|{kind:'group';isolation:boolean}|{kind:'boolean';operation:string}|{kind:'trace';key:string;threshold:number;simplifyEpsilon:number};
export interface VectorNodeRef{identity:object;id:string;name:string;transform:readonly number[];opacity:number;blendMode:string;visible:boolean;locked:boolean;paint:VectorPaintSourceV1;content:VectorContentRef}
export interface VectorNodeSourceV1{descriptor():VectorNodeRef;childCount():number;child(index:number):VectorNodeSourceV1|undefined;referenceCount():number;reference(index:number):string|undefined}
export interface VectorGraphSourceV1{schema():string;id():string;canvas():VectorCanvas|undefined;rootCount():number;root(index:number):VectorNodeSourceV1|undefined;assetCount():number;asset(index:number):VectorAssetRef|undefined}
export interface VectorGraphAdvance{progress:ChildAdmissionProgress;done:boolean;graph:VectorGraphSourceV1|undefined}
export interface EmbeddedChildVectorSourceV1{readDemand():ChildAdmissionDemand;read(source:ChildAdmissionSource,g:ChildAdmissionGrant):VectorGraphAdvance}

export {OriginalChildVectorPreparation} from './🧳️prepare/🟦️.ts';
export type {OriginalChildVectorAdvance} from './🧳️prepare/🟦️.ts';
