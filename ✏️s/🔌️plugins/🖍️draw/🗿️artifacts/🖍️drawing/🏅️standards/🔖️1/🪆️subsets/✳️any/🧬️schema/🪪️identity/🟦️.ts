/** 🪪️ Domain identity keys and decoded commitment facts. */
export class DrawingIdentity {
 #admitted=true;
 private constructor(readonly key:string){if(typeof key!=="string"||key.length===0)throw Error("Drawing identity key required");Object.freeze(this);}
 static admit(key:string):DrawingIdentity{return new DrawingIdentity(key);}
}
export type DrawingIdentityKind="layer"|"path"|"group"|"boolean"|"trace"|"shape"|"text"|"image"|"svg"|"imageAsset";
export interface DrawingIdentityCommitment {kind:DrawingIdentityKind;digest:number[]}
export interface DrawingIdentityAssignment {source:string;target:string}

import type {DrawingLayerNode} from "../🟦️.ts";
/** 📋️ Clones a subtree using complete distinct authored identity assignments. */
export function cloneDrawingLayerNode(node:DrawingLayerNode,suffix:string,identities:DrawingIdentityAssignment[]):DrawingLayerNode {
 const sources=new Map<string,string>(),targets=new Set<string>();for(const assignment of identities){if(!assignment.source||!assignment.target||sources.has(assignment.source)||targets.has(assignment.target))throw Error("Drawing clone identity repeated or empty");sources.set(assignment.source,assignment.target);targets.add(assignment.target);}
 const observed=new Set<string>();let count=0;
 function identify(node:DrawingLayerNode,depth:number):DrawingLayerNode{if(depth>=64||++count>4096||observed.has(node.id))throw Error("Drawing clone source census invalid");const target=sources.get(node.id);if(!target||sources.has(target))throw Error("Drawing clone identity missing or overlaps source");observed.add(node.id);const copy:DrawingLayerNode=structuredClone(node.kind==="group"?{...node,children:[]}:node);copy.id=target;if(copy.kind==="group"&&node.kind==="group")copy.children=node.children.map(child=>identify(child,depth+1));if(copy.kind==="boolean")copy.children=copy.children.map(id=>sources.get(id)??id);return copy;}
 const copy=identify(node,0);if(observed.size!==identities.length)throw Error("Drawing clone identity census differs");copy.name+=suffix;return copy;
}

function record(value:unknown,keys:string[]):Record<string,unknown>{if(typeof value!=="object"||value===null||Array.isArray(value))throw Error("Drawing identity record required");const object=value as Record<string,unknown>;const fields=Object.keys(object);if(fields.length!==keys.length||fields.some(key=>!keys.includes(key))||keys.some(key=>!Object.hasOwn(object,key)))throw Error("Drawing identity closed record differs");return object;}
function key(value:unknown):string{if(typeof value!=="string"||value.length===0)throw Error("Drawing identity key required");return value;}
export function parseDrawingIdentity(value:unknown):DrawingIdentity{const object=record(value,["key"]);return DrawingIdentity.admit(key(object.key));}
export function parseDrawingIdentityKind(value:unknown):DrawingIdentityKind{if(typeof value!=="string"||!["layer","path","group","boolean","trace","shape","text","image","svg","imageAsset"].includes(value))throw Error("Drawing identity kind unknown");return value as DrawingIdentityKind;}
export function parseDrawingIdentityAssignment(value:unknown):DrawingIdentityAssignment{const object=record(value,["source","target"]);return {source:key(object.source),target:key(object.target)};}
export function parseDrawingIdentityCommitment(value:unknown):DrawingIdentityCommitment{const object=record(value,["kind","digest"]);const kind=parseDrawingIdentityKind(object.kind);if(!Array.isArray(object.digest)||object.digest.length!==32||object.digest.some(byte=>!Number.isInteger(byte)||byte<0||byte>255))throw Error("Drawing identity digest must be32 octets");return {kind,digest:object.digest.map(byte=>byte as number)};}
