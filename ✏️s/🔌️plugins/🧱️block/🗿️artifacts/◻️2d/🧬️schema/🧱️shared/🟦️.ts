/** 🧱️ Complete shared Block document fields mirror the adjacent native authority. */
import * as p from "./📐️scalar/🟦️.ts";
import type {Binary64} from "../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type {BlockVector3,BlockVector4} from "./📐️scalar/🟦️.ts";
export interface BlockKindIdentity{id:string;name:string;label:string;variant:string|null;description:string;icon:string|null;unit:string|null}
export interface BlockAttribute{key:string;value:string;definition:string|null}
export interface BlockAuthor{id:string;name:string;email:string|null}
export interface BlockCompatibilityRule{id:string;source:string;target:string;bidirectional:boolean}
export interface BlockRepresentation{id:string;name:string;meshUrl:string|null;tags:string[];lod:string|null;description:string;attributes:BlockAttribute[]}
export interface BlockCamera2d{x:Binary64;y:Binary64;zoom:Binary64}
export interface BlockCamera3d{position:p.BlockVector3;target:p.BlockVector3;zoom:Binary64}
export interface BlockMeta{description:string}
/** 🪪️ Retain the complete literal kind identity. */
export function parseBlockKindIdentity(v:unknown):BlockKindIdentity{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),label:p.text(r.label),variant:p.optional(r.variant,p.text),description:p.text(r.description),icon:p.optional(r.icon,p.text),unit:p.optional(r.unit,p.text)}}
/** 🏷️ Retain actual attribute strings and absence. */
export function parseBlockAttribute(v:unknown):BlockAttribute{const r=p.row(v);return{key:p.text(r.key),value:p.text(r.value),definition:p.optional(r.definition,p.text)}}
/** 👤️ Retain actual author fields without inventing rank. */
export function parseBlockAuthor(v:unknown):BlockAuthor{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),email:p.optional(r.email,p.text)}}
/** 🔗️ Retain unresolved ordered compatibility identities. */
export function parseBlockCompatibilityRule(v:unknown):BlockCompatibilityRule{const r=p.row(v);return{id:p.text(r.id),source:p.text(r.source),target:p.text(r.target),bidirectional:p.boolean(r.bidirectional)}}
/** 🖼️ Retain representation metadata and its ordered attributes. */
export function parseBlockRepresentation(v:unknown):BlockRepresentation{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),meshUrl:p.optional(r.meshUrl,p.text),tags:p.list(r.tags,p.text),lod:p.optional(r.lod,p.text),description:p.text(r.description),attributes:p.list(r.attributes,parseBlockAttribute)}}
/** 🔵️ Retain every native two-dimensional camera word. */
export function parseBlockCamera2d(v:unknown):BlockCamera2d{const r=p.row(v);return{x:p.word(r.x),y:p.word(r.y),zoom:p.word(r.zoom)}}
/** 🧊️ Retain every native three-dimensional camera word. */
export function parseBlockCamera3d(v:unknown):BlockCamera3d{const r=p.row(v);return{position:p.vector3(r.position),target:p.vector3(r.target),zoom:p.word(r.zoom)}}
/** 📝️ Retain the actual persisted description. */
export function parseBlockMeta(v:unknown):BlockMeta{const r=p.row(v);return{description:p.text(r.description)}}
