/** 🧊️ Complete persisted Puzzle3d fields shared by its owned Source facets. */
import {parseBinary64,type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type Puzzle3dObjectAnchor="fixed"|"derived";
export type Puzzle3dCompatSpecificity="general"|"object"|"attraction"|"vortex"|"cable";
export type Puzzle3dVector3=[Binary64,Binary64,Binary64];
export type Puzzle3dVector4=[Binary64,Binary64,Binary64,Binary64];
export type Puzzle3dScale=Binary64|Puzzle3dVector3;
export interface Puzzle3dVortex{id:string;vortexKind:string|null;label:string|null;position:Puzzle3dVector3;direction:Puzzle3dVector3|null;radius:Binary64|null;hidden:boolean;locked:boolean}
export interface Puzzle3dObject{id:string;label:string|null;objectKind:string|null;anchor:Puzzle3dObjectAnchor;origin:Puzzle3dVector3;orientation:Puzzle3dVector4|null;scale:Puzzle3dScale|null;meshUrl:string|null;vortices:Puzzle3dVortex[];hidden:boolean;locked:boolean}
export interface Puzzle3dAttraction{id:string;attracting:string;attracted:string;gap:Binary64;shift:Binary64;rise:Binary64;rotation:Binary64;turn:Binary64;tilt:Binary64;x:Binary64;y:Binary64}
export interface Puzzle3dTargetVolume{id:string;origin:Puzzle3dVector3;orientation:Puzzle3dVector4|null;scale:Puzzle3dScale|null;hidden:boolean;locked:boolean}
export interface Puzzle3dReferenceSource{url:string;mediaKind:string|null}
export interface Puzzle3dReference{id:string;source:Puzzle3dReferenceSource;origin:Puzzle3dVector3;widthWorld:Binary64;locked:boolean;hidden:boolean}
export interface Puzzle3dAttribute{id:string;key:string;value:string;definition:string|null}
export interface Puzzle3dAuthor{id:string;name:string;email:string;role:string|null;rank:number|null}
export interface Puzzle3dRepresentation{id:string;name:string;url:string;mime:string;tags:string[];lod:string|null;description:string}
export interface Puzzle3dCatalogVortexTemplate{id:string;name:string;label:string;description:string;icon:string;vortexKind:string|null;point:Puzzle3dVector3;direction:Puzzle3dVector3;t:Binary64|null;mandatory:boolean|null;radius:Binary64|null}
export interface Puzzle3dCatalogObjectKind{id:string;name:string;label:string;description:string;icon:string;image:string;unit:string;abstract:boolean;baseKinds:string[];representations:Puzzle3dRepresentation[];vortices:Puzzle3dCatalogVortexTemplate[];attributes:Puzzle3dAttribute[];authors:Puzzle3dAuthor[]}
export interface Puzzle3dCatalogVortexKind{id:string;code:string|null;label:string|null;order:number|null;compatibleWith:string[];description:string;icon:string;color:string;defaultCableKind:string}
export interface Puzzle3dCatalogCableKind{id:string;label:string;name:string;defaultAttractionKind:string}
export interface Puzzle3dCatalogAttractionKind{id:string;label:string;name:string}
export interface Puzzle3dKindCatalogs{objects:Puzzle3dCatalogObjectKind[];vortices:Puzzle3dCatalogVortexKind[];cables:Puzzle3dCatalogCableKind[];attractions:Puzzle3dCatalogAttractionKind[]}
export interface Puzzle3dKindCompatibility{source:string;target:string;bidirectional:boolean;important:boolean;specificity:Puzzle3dCompatSpecificity}
export interface Puzzle3dMeta{kindCatalogs:Puzzle3dKindCatalogs|null;kindCompatibility:Puzzle3dKindCompatibility[]}
export interface Puzzle3dSnapshot{
 /** @state artifact */ schema:string;
 /** @state artifact */ domain:string;
 /** @state artifact */ meta:Puzzle3dMeta;
 /** @state artifact */ objects:Puzzle3dObject[];
 /** @state artifact */ attractions:Puzzle3dAttraction[];
 /** @state artifact */ targetVolumes:Puzzle3dTargetVolume[];
 /** @state artifact */ references:Puzzle3dReference[];
}
/** 🚪️ Reject an invalid canonical native field. */
export class puzzlePuzzle3dArtifactGuardRefusal extends Error{constructor(readonly at:string,readonly why:string){super(at+": "+why)}}
const fail=(at:string,why:string):never=>{throw new puzzlePuzzle3dArtifactGuardRefusal(at,why)};
const object=(v:unknown,at:string):Record<string,unknown>=>v!==null&&typeof v==="object"&&!Array.isArray(v)?v as Record<string,unknown>:fail(at,"expected an object");
const text=(v:unknown,at:string):string=>typeof v==="string"?v:fail(at,"expected text");
const bool=(v:unknown,at:string):boolean=>typeof v==="boolean"?v:fail(at,"expected Boolean");
const word=(v:unknown,at:string):Binary64=>{try{return parseBinary64(v)}catch{return fail(at,"expected an exact binary64 word")}};
const int32=(v:unknown,at:string):number=>typeof v==="number"&&Number.isInteger(v)&&v>=-2147483648&&v<=2147483647?v:fail(at,"expected signed32");
const list=<T>(v:unknown,at:string,parse:(v:unknown,at:string)=>T):T[]=>Array.isArray(v)?v.map((item,index)=>parse(item,at+"["+index+"]")):fail(at,"expected an array");
const optional=<T>(v:unknown,at:string,parse:(v:unknown,at:string)=>T):T|null=>v===null?null:parse(v,at);
const member=<T extends string>(v:unknown,at:string,values:readonly T[]):T=>values.includes(v as T)?v as T:fail(at,"unexpected enum member");
const vector3=(v:unknown,at:string):Puzzle3dVector3=>{const items=list(v,at,word);if(items.length!==3)return fail(at,"expected three words");return[items[0]!,items[1]!,items[2]!]};
const vector4=(v:unknown,at:string):Puzzle3dVector4=>{const items=list(v,at,word);if(items.length!==4)return fail(at,"expected four words");return[items[0]!,items[1]!,items[2]!,items[3]!]};
/** ⚖️ Admit the native scalar or three-component scale. */
export function parsePuzzle3dScale(v:unknown,at="$"):Puzzle3dScale{return Array.isArray(v)?vector3(v,at):word(v,at)}
/** ⚓️ Admit the native anchor. */
export function parsePuzzle3dObjectAnchor(v:unknown,at="$"):Puzzle3dObjectAnchor{return member(v,at,["fixed","derived"])}
/** 🌀️ Admit the complete persisted vortex. */
export function parsePuzzle3dVortex(v:unknown,at="$"):Puzzle3dVortex{const r=object(v,at);return{id:text(r.id,at+".id"),vortexKind:optional(r.vortexKind,at+".vortexKind",text),label:optional(r.label,at+".label",text),position:vector3(r.position,at+".position"),direction:optional(r.direction,at+".direction",vector3),radius:optional(r.radius,at+".radius",word),hidden:bool(r.hidden,at+".hidden"),locked:bool(r.locked,at+".locked")}}
/** 🧱️ Admit each persisted object field and ordered vortex. */
export function parsePuzzle3dObject(v:unknown,at="$"):Puzzle3dObject{const r=object(v,at);return{id:text(r.id,at+".id"),label:optional(r.label,at+".label",text),objectKind:optional(r.objectKind,at+".objectKind",text),anchor:parsePuzzle3dObjectAnchor(r.anchor,at+".anchor"),origin:vector3(r.origin,at+".origin"),orientation:optional(r.orientation,at+".orientation",vector4),scale:optional(r.scale,at+".scale",parsePuzzle3dScale),meshUrl:optional(r.meshUrl,at+".meshUrl",text),vortices:list(r.vortices,at+".vortices",parsePuzzle3dVortex),hidden:bool(r.hidden,at+".hidden"),locked:bool(r.locked,at+".locked")}}
/** 🔗️ Admit all eight required native attraction parameters. */
export function parsePuzzle3dAttraction(v:unknown,at="$"):Puzzle3dAttraction{const r=object(v,at);return{id:text(r.id,at+".id"),attracting:text(r.attracting,at+".attracting"),attracted:text(r.attracted,at+".attracted"),gap:word(r.gap,at+".gap"),shift:word(r.shift,at+".shift"),rise:word(r.rise,at+".rise"),rotation:word(r.rotation,at+".rotation"),turn:word(r.turn,at+".turn"),tilt:word(r.tilt,at+".tilt"),x:word(r.x,at+".x"),y:word(r.y,at+".y")}}
/** 🧊️ Admit the full target pose without spatial restrictions. */
export function parsePuzzle3dTargetVolume(v:unknown,at="$"):Puzzle3dTargetVolume{const r=object(v,at);return{id:text(r.id,at+".id"),origin:vector3(r.origin,at+".origin"),orientation:optional(r.orientation,at+".orientation",vector4),scale:optional(r.scale,at+".scale",parsePuzzle3dScale),hidden:bool(r.hidden,at+".hidden"),locked:bool(r.locked,at+".locked")}}
/** 🌐️ Admit literal native reference source fields. */
export function parsePuzzle3dReferenceSource(v:unknown,at="$"):Puzzle3dReferenceSource{const r=object(v,at);return{url:text(r.url,at+".url"),mediaKind:optional(r.mediaKind,at+".mediaKind",text)}}
/** 🖼️ Admit the complete persisted reference. */
export function parsePuzzle3dReference(v:unknown,at="$"):Puzzle3dReference{const r=object(v,at);return{id:text(r.id,at+".id"),source:parsePuzzle3dReferenceSource(r.source,at+".source"),origin:vector3(r.origin,at+".origin"),widthWorld:word(r.widthWorld,at+".widthWorld"),locked:bool(r.locked,at+".locked"),hidden:bool(r.hidden,at+".hidden")}}
/** 🏷️ Admit literal owned attribute fields. */
export function parsePuzzle3dAttribute(v:unknown,at="$"):Puzzle3dAttribute{const r=object(v,at);return{id:text(r.id,at+".id"),key:text(r.key,at+".key"),value:text(r.value,at+".value"),definition:optional(r.definition,at+".definition",text)}}
/** ✍️ Admit authors with the full optional signed32 rank. */
export function parsePuzzle3dAuthor(v:unknown,at="$"):Puzzle3dAuthor{const r=object(v,at);return{id:text(r.id,at+".id"),name:text(r.name,at+".name"),email:text(r.email,at+".email"),role:optional(r.role,at+".role",text),rank:optional(r.rank,at+".rank",int32)}}
/** 🖼️ Admit representations with their ordered literal tags. */
export function parsePuzzle3dRepresentation(v:unknown,at="$"):Puzzle3dRepresentation{const r=object(v,at);return{id:text(r.id,at+".id"),name:text(r.name,at+".name"),url:text(r.url,at+".url"),mime:text(r.mime,at+".mime"),tags:list(r.tags,at+".tags",text),lod:optional(r.lod,at+".lod",text),description:text(r.description,at+".description")}}
/** 🌱️ Admit the full native vortex template. */
export function parsePuzzle3dCatalogVortexTemplate(v:unknown,at="$"):Puzzle3dCatalogVortexTemplate{const r=object(v,at);return{id:text(r.id,at+".id"),name:text(r.name,at+".name"),label:text(r.label,at+".label"),description:text(r.description,at+".description"),icon:text(r.icon,at+".icon"),vortexKind:optional(r.vortexKind,at+".vortexKind",text),point:vector3(r.point,at+".point"),direction:vector3(r.direction,at+".direction"),t:optional(r.t,at+".t",word),mandatory:optional(r.mandatory,at+".mandatory",bool),radius:optional(r.radius,at+".radius",word)}}
/** 🗂️ Admit each catalog object kind and every ordered owned collection. */
export function parsePuzzle3dCatalogObjectKind(v:unknown,at="$"):Puzzle3dCatalogObjectKind{const r=object(v,at);return{id:text(r.id,at+".id"),name:text(r.name,at+".name"),label:text(r.label,at+".label"),description:text(r.description,at+".description"),icon:text(r.icon,at+".icon"),image:text(r.image,at+".image"),unit:text(r.unit,at+".unit"),abstract:bool(r.abstract,at+".abstract"),baseKinds:list(r.baseKinds,at+".baseKinds",text),representations:list(r.representations,at+".representations",parsePuzzle3dRepresentation),vortices:list(r.vortices,at+".vortices",parsePuzzle3dCatalogVortexTemplate),attributes:list(r.attributes,at+".attributes",parsePuzzle3dAttribute),authors:list(r.authors,at+".authors",parsePuzzle3dAuthor)}}
/** 🔘️ Admit each vortex kind with signed32 order and literal compatibility. */
export function parsePuzzle3dCatalogVortexKind(v:unknown,at="$"):Puzzle3dCatalogVortexKind{const r=object(v,at);return{id:text(r.id,at+".id"),code:optional(r.code,at+".code",text),label:optional(r.label,at+".label",text),order:optional(r.order,at+".order",int32),compatibleWith:list(r.compatibleWith,at+".compatibleWith",text),description:text(r.description,at+".description"),icon:text(r.icon,at+".icon"),color:text(r.color,at+".color"),defaultCableKind:text(r.defaultCableKind,at+".defaultCableKind")}}
/** 🧵️ Admit each literal cable kind. */
export function parsePuzzle3dCatalogCableKind(v:unknown,at="$"):Puzzle3dCatalogCableKind{const r=object(v,at);return{id:text(r.id,at+".id"),label:text(r.label,at+".label"),name:text(r.name,at+".name"),defaultAttractionKind:text(r.defaultAttractionKind,at+".defaultAttractionKind")}}
/** 🧲️ Admit each literal attraction kind. */
export function parsePuzzle3dCatalogAttractionKind(v:unknown,at="$"):Puzzle3dCatalogAttractionKind{const r=object(v,at);return{id:text(r.id,at+".id"),label:text(r.label,at+".label"),name:text(r.name,at+".name")}}
/** 📚️ Admit all four actual inline catalog branches. */
export function parsePuzzle3dKindCatalogs(v:unknown,at="$"):Puzzle3dKindCatalogs{const r=object(v,at);return{objects:list(r.objects,at+".objects",parsePuzzle3dCatalogObjectKind),vortices:list(r.vortices,at+".vortices",parsePuzzle3dCatalogVortexKind),cables:list(r.cables,at+".cables",parsePuzzle3dCatalogCableKind),attractions:list(r.attractions,at+".attractions",parsePuzzle3dCatalogAttractionKind)}}
/** 🔖️ Admit the native compatibility specificity. */
export function parsePuzzle3dCompatSpecificity(v:unknown,at="$"):Puzzle3dCompatSpecificity{return member(v,at,["general","object","attraction","vortex","cable"])}
/** 🔁️ Admit an ordered compatibility rule without resolving its literal IDs. */
export function parsePuzzle3dKindCompatibility(v:unknown,at="$"):Puzzle3dKindCompatibility{const r=object(v,at);return{source:text(r.source,at+".source"),target:text(r.target,at+".target"),bidirectional:bool(r.bidirectional,at+".bidirectional"),important:bool(r.important,at+".important"),specificity:parsePuzzle3dCompatSpecificity(r.specificity,at+".specificity")}}
/** 🗃️ Admit actual optional inline catalogs and required ordered compatibility. */
export function parsePuzzle3dMeta(v:unknown,at="$"):Puzzle3dMeta{const r=object(v,at);return{kindCatalogs:optional(r.kindCatalogs,at+".kindCatalogs",parsePuzzle3dKindCatalogs),kindCompatibility:list(r.kindCompatibility,at+".kindCompatibility",parsePuzzle3dKindCompatibility)}}
/** 📸️ Admit all seven actual persisted fields without file defaults or normalization. */
export function parsePuzzle3dSnapshot(v:unknown,at="$"):Puzzle3dSnapshot{const r=object(v,at);return{schema:text(r.schema,at+".schema"),domain:text(r.domain,at+".domain"),meta:parsePuzzle3dMeta(r.meta,at+".meta"),objects:list(r.objects,at+".objects",parsePuzzle3dObject),attractions:list(r.attractions,at+".attractions",parsePuzzle3dAttraction),targetVolumes:list(r.targetVolumes,at+".targetVolumes",parsePuzzle3dTargetVolume),references:list(r.references,at+".references",parsePuzzle3dReference)}}
