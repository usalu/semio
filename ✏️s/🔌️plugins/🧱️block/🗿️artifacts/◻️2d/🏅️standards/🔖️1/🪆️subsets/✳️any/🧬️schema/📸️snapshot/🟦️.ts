/** ◻️ Complete Block2d persisted fields with one canonical shared Block authority. */
import * as shared from "../../../../../../../../🧬️schema/🧱️shared/🟦️.ts";
import * as p from "../../../../../../../../🧬️schema/🧱️shared/📐️scalar/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export * from "../../../../../../../../🧬️schema/🧱️shared/🟦️.ts";
export interface Block2dPresentation{shape:string|null;radius:Binary64|null;width:Binary64|null;height:Binary64|null;color:string|null;iconKind:string|null}
export interface Block2dHandleKind{id:string;name:string;label:string;color:string;defaultWireKind:string}
export interface Block2dHandleTemplate{id:string;handleKind:string;angle:Binary64;radius:Binary64}
export interface Block2dSnapshot{
 /** @state artifact */ schema:string;
 /** @state artifact */ nodeKind:shared.BlockKindIdentity;
 /** @state artifact */ presentation:Block2dPresentation;
 /** @state artifact */ handleKinds:Block2dHandleKind[];
 /** @state artifact */ handles:Block2dHandleTemplate[];
 /** @state artifact */ compatibility:shared.BlockCompatibilityRule[];
 /** @state artifact */ attributes:shared.BlockAttribute[];
 /** @state artifact */ authors:shared.BlockAuthor[];
 /** @state artifact */ camera2d:shared.BlockCamera2d;
 /** @state artifact */ meta:shared.BlockMeta;
}
/** 🔵️ Admit actual optional presentation fields without numeric alternatives. */
export function parseBlock2dPresentation(v:unknown):Block2dPresentation{const r=p.row(v);return{shape:p.optional(r.shape,p.text),radius:p.optional(r.radius,p.word),width:p.optional(r.width,p.word),height:p.optional(r.height,p.word),color:p.optional(r.color,p.text),iconKind:p.optional(r.iconKind,p.text)}}
/** 🔘️ Admit each required owned handle-kind string. */
export function parseBlock2dHandleKind(v:unknown):Block2dHandleKind{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),label:p.text(r.label),color:p.text(r.color),defaultWireKind:p.text(r.defaultWireKind)}}
/** 🌱️ Admit exact handle words and literal native kind references. */
export function parseBlock2dHandleTemplate(v:unknown):Block2dHandleTemplate{const r=p.row(v);return{id:p.text(r.id),handleKind:p.text(r.handleKind),angle:p.word(r.angle),radius:p.word(r.radius)}}
/** 📸️ Admit the complete canonical persisted parent without file defaults. */
export function parseBlock2dSnapshot(v:unknown):Block2dSnapshot{const r=p.row(v);return{schema:p.text(r.schema),nodeKind:shared.parseBlockKindIdentity(r.nodeKind),presentation:parseBlock2dPresentation(r.presentation),handleKinds:p.list(r.handleKinds,parseBlock2dHandleKind),handles:p.list(r.handles,parseBlock2dHandleTemplate),compatibility:p.list(r.compatibility,shared.parseBlockCompatibilityRule),attributes:p.list(r.attributes,shared.parseBlockAttribute),authors:p.list(r.authors,shared.parseBlockAuthor),camera2d:shared.parseBlockCamera2d(r.camera2d),meta:shared.parseBlockMeta(r.meta)}}
