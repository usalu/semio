import type {IfcEntity, IfcSnapshot, IfcValue} from "../📸️snapshot/🟦️.ts";
import {ifc4IsSubtype} from "../📚️catalog/🟦️.ts";
import definition from "./🔣️.json" with {type: "json"};
export interface SemanticDescriptor {id: string; family: string; index: number; label: {en: string; de: string}; kind: string; optional: boolean; editable: boolean; maxCharacters?: number}
export interface SemanticContract {schema: string; domain: string; granularity: string; maximumTextBytes: number; families: Record<string, string[]>; attributes: SemanticDescriptor[]}
export const SEMANTIC_CONTRACT: Readonly<SemanticContract> = definition;
export interface SemanticField {descriptor: SemanticDescriptor; value: string | null; state: "valid" | "unset" | "invalid" | "too-large"; reference?: bigint}
export interface SemanticProjection {id: bigint; type: string; fields: SemanticField[]; forward: bigint[]; back: bigint[]; missing: bigint[]}
export interface SemanticEdit {id: bigint; index: number; value: IfcValue}
const refusal = (reason: string): never => {throw new Error(reason);};
const descriptors = (entity: IfcEntity): SemanticDescriptor[] => entity.complex?.length ? [] : SEMANTIC_CONTRACT.attributes.filter(row => SEMANTIC_CONTRACT.families[row.family]?.some(base=>ifc4IsSubtype(entity.name,base)));
const validGuid = (value: string): boolean => /^[0-3][0-9A-Za-z_$]{21}$/.test(value);
const textBytes = (value: string): number => value.length > SEMANTIC_CONTRACT.maximumTextBytes ? value.length : new TextEncoder().encode(value).length;
export const semanticIdentity = (id: bigint): string => "ifc-" + id;
export function parseSemanticIdentity(value: string): bigint | undefined {
  if (!/^ifc-[1-9][0-9]{0,19}$/.test(value)) return undefined;
  const id = BigInt(value.slice(4)); return id <= 18446744073709551615n ? id : undefined;
}
function references(entity: IfcEntity, tick: () => void): Set<bigint> {
  const ids = new Set<bigint>();
  const visit = (value: IfcValue, depth: number): void => {
    tick(); if (depth > 128) refusal("semantic-value-depth");
    if (value.kind === "reference") ids.add(value.value);
    else if (value.kind === "aggregate") for (const item of value.value) visit(item, depth + 1);
    else if (value.kind === "typedValue") for (const item of value.value.items) visit(item, depth + 1);
  };
  for (const value of entity.args) visit(value, 0);
  for (const constituent of entity.complex ?? []) for (const value of constituent.args) visit(value, 0);
  return ids;
}
/** 🔎️ Project declared IFC4 semantics and graph navigation under original caller control. */
export function projectSemantics(snapshot: IfcSnapshot, id: bigint, tick: () => void = () => {}): SemanticProjection {
  const entities = new Map<bigint, IfcEntity>();
  for (const entity of snapshot.entities) {tick(); if (entities.has(entity.id)) refusal("semantic-duplicate-entity"); entities.set(entity.id, entity);}
  const entity = entities.get(id) ?? refusal("semantic-entity-missing");
  const fields = descriptors(entity).map(descriptor => {
    tick(); const value = entity.args[descriptor.index];
    if (value?.kind === "unset" && descriptor.optional) return {descriptor, value: null, state: "unset" as const};
    if (descriptor.kind === "reference" && value?.kind === "reference") return {descriptor, value: "#" + value.value, reference: value.value, state: entities.has(value.value) ? "valid" as const : "invalid" as const};
    if (descriptor.kind !== "reference" && value?.kind === "string") {
      if (textBytes(value.value) > SEMANTIC_CONTRACT.maximumTextBytes) return {descriptor, value: "…", state: "too-large" as const};
      const valid = (!descriptor.maxCharacters || [...value.value].length <= descriptor.maxCharacters) && (descriptor.kind !== "global-id" || validGuid(value.value));
      return {descriptor, value: value.value, state: valid ? "valid" as const : "invalid" as const};
    }
    return {descriptor, value: "?", state: "invalid" as const};
  });
  const forward: bigint[] = [], missing: bigint[] = [], back: bigint[] = [];
  for(const reference of references(entity,tick)){tick();(entities.has(reference)?forward:missing).push(reference);}
  for (const candidate of snapshot.entities) {tick(); if (references(candidate, tick).has(id)) back.push(candidate.id);}
  const order = (a: bigint, b: bigint): number => a < b ? -1 : a > b ? 1 : 0;
  return {id, type: entity.name, fields, forward: forward.sort(order), back: back.sort(order), missing: missing.sort(order)};
}
/** ✍️ Validate a declared semantic scalar and plan the existing stable-id argument mutation. */
export function editSemantic(snapshot: IfcSnapshot, id: bigint, field: string, value: unknown, tick: () => void = () => {}): SemanticEdit {
  let entity: IfcEntity | undefined;
  for(const candidate of snapshot.entities){tick();if(candidate.id===id){entity=candidate;break;}}
  if(!entity) return refusal("semantic-entity-missing");
  const descriptor = descriptors(entity).find(row => row.id === field) ?? refusal("semantic-unknown-attribute");
  if (!descriptor.editable) refusal("semantic-read-only-attribute");
  const current=entity.args[descriptor.index];
  if (current === undefined) refusal("semantic-attribute-missing");
  if(current.kind!=="string"&&current.kind!=="unset")refusal("semantic-current-value-kind");
  if(current.kind==="string"&&textBytes(current.value)>SEMANTIC_CONTRACT.maximumTextBytes)refusal("semantic-text-limit");
  if (value === null) {if (!descriptor.optional) refusal("semantic-required-value"); return {id, index: descriptor.index, value: {kind: "unset"}};}
  if (typeof value !== "string") refusal("semantic-value-kind");
  const content = value as string;
  if (textBytes(content) > SEMANTIC_CONTRACT.maximumTextBytes || descriptor.maxCharacters && [...content].length > descriptor.maxCharacters) refusal("semantic-text-limit");
  if (descriptor.kind === "global-id") {
    if (!validGuid(content)) refusal("semantic-invalid-global-id");
    for(const candidate of snapshot.entities){tick();if(candidate.id===id)continue;if(ifc4IsSubtype(candidate.name,"IFCROOT")&&candidate.args[0]?.kind==="string"&&candidate.args[0].value===content)refusal("semantic-duplicate-global-id");for(const constituent of candidate.complex??[]){tick();if(constituent.name==="IFCROOT"&&constituent.args[0]?.kind==="string"&&constituent.args[0].value===content)refusal("semantic-duplicate-global-id");}}
  }
  return {id, index: descriptor.index, value: {kind: "string", value: content}};
}
