import { parseArtifactRef,type ArtifactRef } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
/** 🪪️ Admitted stream words bind to exact logical generations and structural paths. */
import type { ObjRef, PdfOp, PdfObject, PdfIndirectObject, PdfToUnicode, PdfEmbeddedCMap, PdfFontProgram, PdfImage } from "../📸️snapshot/🟦️.ts";
import { parsePdfOp, parsePdfToUnicode, parsePdfEmbeddedCMap, parsePdfFontProgram, parsePdfImage } from "../📸️snapshot/🟦️.ts";
export type PdfGraphPath = { kind: "entry"; key: string } | { kind: "item"; index: number };
export interface PdfGraphIdentity { owner: ObjRef; path: PdfGraphPath[] }
export type PdfStreamRoleValue = { kind: "operators"; content: PdfOp[] } | { kind: "sampledWords"; samples: number[] } | { kind: "calculatorProgram"; code: string } | { kind: "unicodeMap"; mapping: PdfToUnicode } | { kind: "characterMap"; cmap: PdfEmbeddedCMap } | { kind: "fontProgram"; program: PdfFontProgram } | { kind: "image"; image: PdfImage } | { kind: "metadataText"; text: string } | { kind: "attachmentBytes"; bytes: number[] } | { kind: "paletteComponents"; components: number[] } | { kind: "glyphIds"; glyphs: number[] } | { kind: "referenceBody"; reference: ArtifactRef };
export interface PdfAdmittedStreamRole { identity: PdfGraphIdentity; dependencies: PdfGraphIdentity[]; value: PdfStreamRoleValue }

function record(value: unknown, keys: string[]): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("Expected a stream-role object");
  const result = value as Record<string, unknown>;
  if (Object.keys(result).length !== keys.length || keys.some(key => !(key in result))) throw new Error("Stream-role fields do not match the contract");
  return result;
}
function natural(value: unknown, maximum = Number.MAX_SAFE_INTEGER): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error("Invalid stream-role unsigned word");
  return value;
}
function sequence(value: unknown): unknown[] {
  if (!Array.isArray(value)) throw new Error("Expected a stream-role sequence");
  return value;
}
function identity(value: unknown): PdfGraphIdentity {
  const input = record(value, ["owner", "path"]), owner = record(input.owner, ["num", "gen"]);
  return { owner: { num: natural(owner.num, 4294967295), gen: natural(owner.gen, 65535) }, path: sequence(input.path).map(part => {
    const kind = (part as { kind?: unknown })?.kind;
    if (kind === "entry") { const entry = record(part, ["kind", "key"]); if (typeof entry.key !== "string") throw new Error("Invalid stream-role dictionary key"); return { kind, key: entry.key }; }
    if (kind === "item") { const item = record(part, ["kind", "index"]); return { kind, index: natural(item.index) }; }
    throw new Error("Unknown stream-role path component");
  }) };
}
/** 🛂️ Admits exact role contracts without interpreting retained native stream bytes. */
export function parsePdfAdmittedStreamRole(value: unknown): PdfAdmittedStreamRole {
  const input = record(value, ["identity", "dependencies", "value"]);
  const kind = (input.value as { kind?: unknown })?.kind;
  let semantic: PdfStreamRoleValue;
  if (kind === "operators") { const role = record(input.value, ["kind", "content"]); semantic = { kind, content: sequence(role.content).map(parsePdfOp) }; }
  else if (kind === "sampledWords") { const role = record(input.value, ["kind", "samples"]); semantic = { kind, samples: sequence(role.samples).map(word => natural(word, 4294967295)) }; }
  else if (kind === "calculatorProgram") { const role = record(input.value, ["kind", "code"]); if (typeof role.code !== "string") throw new Error("Invalid calculator program"); semantic = { kind, code: role.code }; }
  else if (kind === "unicodeMap") { const role = record(input.value, ["kind", "mapping"]); semantic = { kind, mapping: parsePdfToUnicode(role.mapping) }; }
  else if (kind === "characterMap") { const role = record(input.value, ["kind", "cmap"]); semantic = { kind, cmap: parsePdfEmbeddedCMap(role.cmap) }; }
  else if (kind === "fontProgram") { const role = record(input.value, ["kind", "program"]); semantic = { kind, program: parsePdfFontProgram(role.program) }; }
  else if (kind === "image") { const role = record(input.value, ["kind", "image"]); semantic = { kind, image: parsePdfImage(role.image) }; }
  else if (kind === "metadataText") { const role = record(input.value, ["kind", "text"]); if (typeof role.text !== "string") throw new Error("Invalid metadata text"); semantic = { kind, text: role.text }; }
  else if (kind === "attachmentBytes") { const role = record(input.value,["kind","bytes"]);semantic={kind,bytes:sequence(role.bytes).map(word=>natural(word,255))};}
  else if (kind === "paletteComponents") {const role=record(input.value,["kind","components"]);semantic={kind,components:sequence(role.components).map(word=>natural(word,255))};}
  else if (kind === "glyphIds") {const role=record(input.value,["kind","glyphs"]);semantic={kind,glyphs:sequence(role.glyphs).map(word=>natural(word,65535))};}
  else if (kind === "referenceBody") {const role=record(input.value,["kind","reference"]);semantic={kind,reference:parseArtifactRef(role.reference)};}
  else throw new Error("Unknown admitted stream role");
  return { identity: identity(input.identity), dependencies: sequence(input.dependencies).map(identity), value: semantic };
}
/** 🎯️ Resolves generations, dictionary keys and array ordinals directly. */
export function resolveIdentity(objects: PdfIndirectObject[], identity: PdfGraphIdentity): PdfObject | undefined {
  let value = objects.find(object => object.id.num === identity.owner.num && object.id.gen === identity.owner.gen)?.value;
  for (const part of identity.path) {
    if (!value) return undefined;
    if (part.kind === "entry") value = (value.kind === "dict" ? value.value : value.kind === "stream" ? value.dict : []).find(entry => entry.key === part.key)?.value;
    else value = value.kind === "array" ? value.value[part.index] : undefined;
  }
  return value;
}
function equal(a: unknown, b: unknown): boolean {
  if (Object.is(a, b)) return true;
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((item, index) => equal(item, b[index]));
  if (a === null || b === null || typeof a !== "object" || typeof b !== "object" || Array.isArray(a) || Array.isArray(b)) return false;
  const left = a as Record<string, unknown>, right = b as Record<string, unknown>, keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every(key => Object.hasOwn(right, key) && equal(left[key], right[key]));
}
/** 🧵️ Lists intrinsic native inputs and inherited page font contexts through pure references. */
export function consumedInputs(objects:PdfIndirectObject[],identity:PdfGraphIdentity):PdfGraphIdentity[]{
  const inputs=[identity];
  if(equal(identity.path,[{kind:"entry",key:"Contents"}])){
    let owner=identity.owner;const visited:ObjRef[]=[];
    while(!visited.some(value=>equal(value,owner))&&visited.length<64){
      visited.push(owner);const page=resolveIdentity(objects,{owner,path:[]});const fields=page?.kind==="dict"?page.value:page?.kind==="stream"?page.dict:[];
      const resources=fields.find(entry=>entry.key==="Resources")?.value;
      if(resources){const resourceIdentity=resources.kind==="ref"?{owner:{num:resources.num,gen:resources.gen},path:[] as PdfGraphPath[]}:{owner,path:[{kind:"entry",key:"Resources"}] as PdfGraphPath[]};const fonts={owner:resourceIdentity.owner,path:[...resourceIdentity.path,{kind:"entry",key:"Font"} as PdfGraphPath]};if(resolveIdentity(objects,fonts))inputs.push(fonts);break;}
      const parent=fields.find(entry=>entry.key==="Parent")?.value;if(parent?.kind!=="ref")break;owner={num:parent.num,gen:parent.gen};
    }
  }
  function references(value:PdfObject|undefined):ObjRef[]{if(!value)return [];if(value.kind==="ref")return [{num:value.num,gen:value.gen}];if(value.kind==="array")return value.value.flatMap(references);if(value.kind==="dict"||value.kind==="stream")return (value.kind==="dict"?value.value:value.dict).flatMap(entry=>references(entry.value));return [];}
  for(let index=0;index<inputs.length;index++)for(const owner of references(resolveIdentity(objects,inputs[index]!))){const input={owner,path:[]};if(!inputs.some(current=>equal(current,input)))inputs.push(input);}
  return inputs;
}
/** 🔐️ Known native roles refuse changed inputs unless a corresponding role payload accompanies the edit. */
export function validateRoleInputs(base: PdfIndirectObject[], next: PdfIndirectObject[], roles: PdfAdmittedStreamRole[], replacements: PdfAdmittedStreamRole[]): void {
  for (const role of roles) if ([...role.dependencies,...consumedInputs(base,role.identity)].some(input => !equal(resolveIdentity(base, input), resolveIdentity(next, input))) && !replacements.some(replacement => equal(replacement.identity, role.identity) && replacement.value.kind === role.value.kind)) throw new Error("Known stream role requires a semantic replacement");
  for (const [index,replacement] of replacements.entries()) {if(replacements.slice(0,index).some(current=>equal(current.identity,replacement.identity)&&current.value.kind===replacement.value.kind))throw new Error("Semantic stream role replacement is duplicated");if(consumedInputs(next,replacement.identity).some(input=>!replacement.dependencies.some(dependency=>equal(dependency,input))))throw new Error("Semantic stream role omits a consumed logical dependency");if(!resolveIdentity(next,replacement.identity)||replacement.dependencies.some(input=>!resolveIdentity(next,input)))throw new Error("Semantic stream role identity or dependency does not resolve");}
}
/** 🔁️ Protects retained native inputs while allowing explicit role deletion with its owner. */
export function validateRoleTransition(base: PdfIndirectObject[], next: PdfIndirectObject[], roles: PdfAdmittedStreamRole[], nextRoles: PdfAdmittedStreamRole[], replacements: PdfAdmittedStreamRole[]): void {
  const retained=roles.filter(role=>resolveIdentity(next,role.identity)!==undefined||nextRoles.some(candidate=>equal(candidate.identity,role.identity)&&candidate.value.kind===role.value.kind)||[...role.dependencies,...consumedInputs(base,role.identity)].some(input=>resolveIdentity(next,input)!==undefined&&!equal(resolveIdentity(base,input),resolveIdentity(next,input))));
  validateRoleInputs(base,next,retained,replacements);
  validateRoleInputs(next,next,[],nextRoles);
}
/** 📄️ Reads logical page operators from their owner identity. */
export function pageContent(roles: PdfAdmittedStreamRole[], owner: ObjRef): PdfOp[] | undefined {
  const role = roles.find(role => equal(role.identity, { owner, path: [{ kind: "entry", key: "Contents" }] }));
  return role?.value.kind === "operators" ? role.value.content : undefined;
}
