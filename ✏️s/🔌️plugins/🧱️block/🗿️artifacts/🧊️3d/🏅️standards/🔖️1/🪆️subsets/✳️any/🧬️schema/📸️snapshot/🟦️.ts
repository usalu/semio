/** 🧊️ Complete Block3d persisted fields and literal independent child identity. */
import * as shared from "@semio-tech/block-js/schema/document-fields";
import * as p from "@semio-tech/block-js/schema/document-fields/scalar";
import type {Binary64} from "@semio-tech/framework/value/ieee754";
export * from "@semio-tech/block-js/schema/document-fields";
export interface Block3dVortexKindExtra{id:string;name:string;label:string;color:string;defaultCableKind:string}
export interface Block3dVortexTemplate{id:string;vortexKind:string;position:p.BlockVector3;direction:p.BlockVector3;radius:Binary64;label:string|null}
export interface Block3dSnapshot{
 /** @state artifact */ schema:string;
 /** @state artifact */ objectKind:shared.BlockKindIdentity;
 /** @state artifact */ representations:shared.BlockRepresentation[];
 /** @state artifact @child kind=s.stdio.semio */ catalog:ArtifactChild;
 /** @state artifact */ vortexKindExtra:Block3dVortexKindExtra[];
 /** @state artifact */ vortices:Block3dVortexTemplate[];
 /** @state artifact */ compatibility:shared.BlockCompatibilityRule[];
 /** @state artifact */ attributes:shared.BlockAttribute[];
 /** @state artifact */ authors:shared.BlockAuthor[];
 /** @state artifact */ camera3d:shared.BlockCamera3d;
 /** @state artifact */ meta:shared.BlockMeta;
}
/** 🧩️ Admit literal overflow metadata independent of child materialization. */
export function parseBlock3dVortexKindExtra(v:unknown):Block3dVortexKindExtra{const r=p.row(v);return{id:p.text(r.id),name:p.text(r.name),label:p.text(r.label),color:p.text(r.color),defaultCableKind:p.text(r.defaultCableKind)}}
/** 🌱️ Admit exact words, optional label and unresolved semantic source references. */
export function parseBlock3dVortexTemplate(v:unknown):Block3dVortexTemplate{const r=p.row(v);return{id:p.text(r.id),vortexKind:p.text(r.vortexKind),position:p.vector3(r.position),direction:p.vector3(r.direction),radius:p.word(r.radius),label:p.optional(r.label,p.text)}}
/** 🪆️ Admit literal independent child identities in the actual native text domain. */
function catalog(v:unknown):ArtifactChild{const child=parseArtifactChild(v);p.text(child.childId);p.text(child.target.artifactId);p.text(child.target.dialect.artifactKind);p.text(child.target.dialect.standard);p.text(child.target.dialect.subset);validateBlock3dCatalogDialect(child.target.dialect);return child}
/** 📸️ Admit every canonical persisted field and exclude local child caches. */
export function parseBlock3dSnapshot(v:unknown):Block3dSnapshot{const r=p.row(v);return{schema:p.text(r.schema),objectKind:shared.parseBlockKindIdentity(r.objectKind),representations:p.list(r.representations,shared.parseBlockRepresentation),catalog:catalog(r.catalog),vortexKindExtra:p.list(r.vortexKindExtra,parseBlock3dVortexKindExtra),vortices:p.list(r.vortices,parseBlock3dVortexTemplate),compatibility:p.list(r.compatibility,shared.parseBlockCompatibilityRule),attributes:p.list(r.attributes,shared.parseBlockAttribute),authors:p.list(r.authors,shared.parseBlockAuthor),camera3d:shared.parseBlockCamera3d(r.camera3d),meta:shared.parseBlockMeta(r.meta)}}


import { requireArtifactDialect, type ArtifactDialect, parseArtifactChild, type ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";
const catalogDialects:readonly ArtifactDialect[]=[{artifactKind:"s.stdio.semio",standard:"v1",subset:"kit"},{artifactKind:"s.stdio.semio",standard:"v1",subset:"*"}];
/** 🪆️ Admit the genuine typed catalog owner before any domain publication. */
export function validateBlock3dCatalogDialect(dialect:ArtifactDialect):void{requireArtifactDialect(dialect,catalogDialects,"Block3d catalog")}
