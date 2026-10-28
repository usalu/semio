import {checkIfcValueDomain,ifcDomainEqual,ifcDomainBounds} from "../../../../../../📚️catalog/⚖️domain/🟦️.ts";
import data from "./🔣️.json" with { type: "json" };
import type { IfcEntity, IfcSnapshot, IfcValue } from "../📸️snapshot/🟦️.ts";

import type {IfcTypeExpression,IfcAttributeDeclaration,IfcEntityDeclaration,IfcTypeDeclaration,IfcCatalog} from "../../../../../../📚️catalog/🟦️.ts";
export type {IfcTypeExpression,IfcAttributeDeclaration,IfcEntityDeclaration,IfcTypeDeclaration,IfcCatalog} from "../../../../../../📚️catalog/🟦️.ts";
import {ifcAttributes,ifcSubtype,ifcSubtypeSteps} from "../../../../../../📚️catalog/🟦️.ts";

/** 📍 A structural diagnostic with an entity and recursive argument path. */
export interface IfcDiagnostic {
  code: string;
  message: string;
  entityId?: bigint;
  path: number[];
}

/** 🔒 Seal the fixed released ADD2 metadata before its public export exists. */
export const ifc4x3Catalog:IfcCatalog=(()=>{const pending:object[]=[data],seen=new WeakSet<object>();while(pending.length){const value=pending.pop()!;if(seen.has(value))continue;seen.add(value);for(const child of Object.values(value))if(child!==null&&typeof child==='object')pending.push(child);Object.freeze(value);}return data as IfcCatalog;})();

/** 🧬 Resolve inherited explicit attributes in Part 21 serialization order. */
export function ifcAttributeDeclarations(name:string):readonly IfcAttributeDeclaration[]|undefined{return ifcAttributes(ifc4x3Catalog,name);}

/** 🌳 Resolve entity inheritance without imposing a schema-specific class hierarchy. */
export function ifcIsSubtype(actual:string,expected:string):boolean{return ifcSubtype(ifc4x3Catalog,actual,expected);}

function memberAcceptsType(member: string, actual: string, depth = 0): boolean {
  if (depth > 64) return false;
  if (member === actual || ifcIsSubtype(actual, member)) return true;
  const declaration = ifc4x3Catalog.types[member];
  return declaration?.kind === "select" && declaration.members.some(item => memberAcceptsType(item, actual, depth + 1));
}

/** 🔎 Check structural EXPRESS domains; retained WHERE, UNIQUE and inverse rules are not evaluated. */
export function validateIfc4x3Snapshot(snapshot: IfcSnapshot): IfcDiagnostic[] {
  const diagnostics: IfcDiagnostic[] = [];
  const entities = new Map<bigint, IfcEntity>();
  const report = (code: string, message: string, entityId?: bigint, path: number[] = []) => diagnostics.push({ code, message, entityId, path });
  const schemas = snapshot.header.fileSchema;
  if (schemas.length !== 1 || schemas[0]?.kind !== "aggregate" || schemas[0].value.length !== 1 || schemas[0].value[0]?.kind !== "string" || schemas[0].value[0].value.toUpperCase() !== ifc4x3Catalog.schema) report("file-schema", "FILE_SCHEMA must identify IFC4X3_ADD2.");
  for (const entity of snapshot.entities) {
    if (entity.id <= 0n || entity.id > 18446744073709551615n) report("entity-id", "Entity identifiers must be positive unsigned 64-bit integers.", entity.id);
    if (entities.has(entity.id)) report("duplicate-id", "Entity identifiers must be unique.", entity.id);
    entities.set(entity.id, entity);
  }
  const matches = (value:IfcValue,expression:IfcTypeExpression):boolean=>{try{checkIfcValueDomain(ifc4x3Catalog,expression,value,{constraints:false,reference:function*(id,expected){yield;const target=entities.get(id);if(!target)return true;if(yield* ifcSubtypeSteps(ifc4x3Catalog,target.name,expected))return true;for(const part of target.complex??[]){yield;if(yield* ifcSubtypeSteps(ifc4x3Catalog,part.name,expected))return true;}return false;}});return true;}catch{return false;}};
  const check = (value: IfcValue, expression: IfcTypeExpression, entityId: bigint, path: number[], depth = 0): void => {
    if (depth > 64) return;
    const type = expression.kind === "named" ? ifc4x3Catalog.types[expression.name!] : undefined;
    if (type?.kind === "defined") {
      const inner = value.kind === "typedValue" ? value.value.items[0]! : value;
      check(inner, type.typeExpr!, entityId, path, depth + 1);
      return;
    }
    if (type?.kind === "select" && value.kind === "typedValue") {
      check(value.value.items[0]!, { kind: "named", name: value.value.name.toUpperCase() }, entityId, path, depth + 1);
      return;
    }
    if (expression.kind !== "aggregate" || value.kind !== "aggregate") return;
    const bounded = !ifcDomainBounds(expression,value.value.length).some(Boolean);
    if (!bounded) report("aggregate-bounds", "Aggregate cardinality violates its EXPRESS bounds.", entityId, path);
    if ((expression.unique || expression.aggregate === "SET") && value.value.some((item, index) => value.value.slice(0, index).some(previous => ifcDomainEqual(ifc4x3Catalog,expression.item!,item,previous)))) report("aggregate-unique", "Aggregate values must be unique.", entityId, path);
    value.value.forEach((item, index) => check(item, expression.item!, entityId, [...path, index], depth + 1));
  };
  const references = (value: IfcValue, entityId: bigint, path: number[], depth = 0): void => {
    if (depth > 64) { report("value-depth", "Value nesting exceeds the structural limit.", entityId, path); return; }
    if (value.kind === "reference" && !entities.has(value.value)) report("dangling-reference", `Reference #${value.value} has no target.`, entityId, path);
    if (value.kind === "aggregate") value.value.forEach((item, index) => references(item, entityId, [...path, index], depth + 1));
    if (value.kind === "typedValue") value.value.items.forEach((item, index) => references(item, entityId, [...path, index], depth + 1));
  };
  for (const entity of snapshot.entities) {
    const declaration = ifc4x3Catalog.entities[entity.name.toUpperCase()];
    entity.args.forEach((value, index) => references(value, entity.id, [index]));
    if (!declaration) { report("unknown-entity", `Unknown IFC4X3_ADD2 entity ${entity.name}.`, entity.id); continue; }
    if (entity.complex?.length) { report("complex-instance", "Complex entity structural validation is unsupported.", entity.id); continue; }
    if (declaration.abstractEntity) report("abstract-entity", `${declaration.name} is abstract.`, entity.id);
    const attributes = declaration.allAttributes;
    if (entity.args.length !== attributes.length) report("attribute-count", `Expected ${attributes.length} positional attributes.`, entity.id);
    attributes.forEach((attribute, index) => {
      const value = entity.args[index];
      if (!value) return;
      if (attribute.derived) { if (value.kind !== "derived") report("derived-attribute", `${attribute.name} requires a derived marker.`, entity.id, [index]); return; }
      if (value.kind === "unset") { if (!attribute.optional) report("required-attribute", `${attribute.name} is required.`, entity.id, [index]); return; }
      if (!matches(value, attribute.typeExpr)) report("attribute-type", `${attribute.name} violates its EXPRESS type.`, entity.id, [index]);
      else check(value, attribute.typeExpr, entity.id, [index]);
    });
  }
  return diagnostics;
}
