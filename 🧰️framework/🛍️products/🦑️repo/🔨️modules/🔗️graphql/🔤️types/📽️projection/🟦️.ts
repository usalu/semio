/** 📽️ TypeScript projection of the served GraphQL schema and of the operations canonical repository clients send.
 *
 * The output follows the shape of the `typescript` and `typescript-operations` plugins of GraphQL Code Generator (the canonical client contract):
 * `Maybe`/`InputMaybe`/`Exact` helpers, a `Scalars` map, one `export type` per object, input, interface and union, one `export enum` per enum,
 * one `…Args` type per field with arguments, and one `…Variables` plus one result type per operation.
 * Types and fields are sorted by name, as the plugin's default `sort` does. See {@link ../📖️parse/🟦️.ts | the reader}.
 */
import { parseOperation, parseSchema, type FieldDefinition, type OperationModel, type SchemaModel, type Selection, type TypeDefinition, type TypeRef } from "../📖️parse/🟦️.ts";

//#region 🧾️Contract
export type GraphqlTypesSource = {
  readonly schemaRel: string;
  readonly schemaSha256: string;
  readonly schema: string;
  readonly operationsRel: string;
  readonly operationsSha256: string;
  readonly operations: readonly { readonly name: string; readonly source: string }[];
};

export const GENERATOR_ID = "repo-graphql-types";
export const REFRESH_COMMAND = "repo run @semio-tech/repo-graphql-rs:generate-types";
//#endregion 🧾️Contract

//#region 🏷️Names
const BUILT_IN_SCALARS = ["ID", "String", "Boolean", "Int", "Float"] as const;
const PRIMITIVE: Readonly<Record<string, string>> = { ID: "string", String: "string", Boolean: "boolean", Int: "number", Float: "number" };

const byName = <T extends { readonly name: string }>(left: T, right: T): number => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0);

function pascalCase(value: string): string {
  return value.split(/[^A-Za-z0-9]+/u).filter(Boolean).map((part) => part[0]!.toUpperCase() + part.slice(1).toLowerCase()).join("");
}

function capitalized(value: string): string {
  return value[0]!.toUpperCase() + value.slice(1);
}
//#endregion 🏷️Names

//#region 🧬️Schema
type Direction = "input" | "output";

function namedTypeText(schema: SchemaModel, name: string, direction: Direction): string {
  const definition = schema.types.get(name);
  if (!definition && !(BUILT_IN_SCALARS as readonly string[]).includes(name)) throw new Error(`GraphQL schema: unknown type ${name}`);
  return definition?.kind === "scalar" || !definition ? `Scalars['${name}']['${direction}']` : name;
}

function typeText(schema: SchemaModel, ref: TypeRef, direction: Direction): string {
  const wrap = direction === "input" ? "InputMaybe" : "Maybe";
  const inner = (node: TypeRef): string => (node.kind === "list" ? `Array<${typeText(schema, node.of, direction)}>` : node.kind === "named" ? namedTypeText(schema, node.name, direction) : inner(node.of));
  return ref.kind === "nonNull" ? inner(ref.of) : `${wrap}<${inner(ref)}>`;
}

function fieldLines(schema: SchemaModel, fields: readonly FieldDefinition[], direction: Direction): string[] {
  return [...fields].sort(byName).map((field) => `  ${field.name}${field.type.kind === "nonNull" ? "" : "?"}: ${typeText(schema, field.type, direction)};`);
}

function argumentsType(schema: SchemaModel, owner: string, field: FieldDefinition): string {
  const lines = [...field.arguments].sort(byName).map((argument) => `  ${argument.name}${argument.type.kind === "nonNull" && !argument.hasDefault ? "" : "?"}: ${typeText(schema, argument.type, "input")};`);
  return `export type ${owner}${capitalized(field.name)}Args = {\n${lines.join("\n")}\n};`;
}

function typeDeclaration(schema: SchemaModel, definition: TypeDefinition): string {
  switch (definition.kind) {
    case "scalar": return "";
    case "enum": return `export enum ${definition.name} {\n${[...definition.values].sort().map((value) => `  ${pascalCase(value)} = '${value}'`).join(",\n")}\n}`;
    case "union": return `export type ${definition.name} = ${[...definition.members].sort().join(" | ")};`;
    case "input": return `export type ${definition.name} = {\n${fieldLines(schema, definition.fields, "input").join("\n")}\n};`;
    case "interface": {
      const head = `export type ${definition.name} = {\n${fieldLines(schema, definition.fields, "output").join("\n")}\n};`;
      return [head, ...[...definition.fields].sort(byName).filter((field) => field.arguments.length > 0).map((field) => argumentsType(schema, definition.name, field))].join("\n\n\n");
    }
    case "object": {
      const implemented = definition.interfaces.length > 0 ? `${[...definition.interfaces].sort().join(" & ")} & ` : "";
      const head = `export type ${definition.name} = ${implemented}{\n  __typename?: '${definition.name}';\n${fieldLines(schema, definition.fields, "output").join("\n")}\n};`;
      return [head, ...[...definition.fields].sort(byName).filter((field) => field.arguments.length > 0).map((field) => argumentsType(schema, definition.name, field))].join("\n\n\n");
    }
  }
}

function scalarsDeclaration(schema: SchemaModel): string {
  const custom = [...schema.types.values()].filter((definition) => definition.kind === "scalar").map((definition) => definition.name).sort();
  const lines = [...BUILT_IN_SCALARS.map((name) => `  ${name}: { input: ${PRIMITIVE[name]}; output: ${PRIMITIVE[name]}; }`), ...custom.map((name) => `  ${name}: { input: any; output: any; }`)];
  return `/** All built-in and custom scalars, mapped to their actual values */\nexport type Scalars = {\n${lines.join("\n")}\n};`;
}
//#endregion 🧬️Schema

//#region 🕸️Operations
function fieldOf(schema: SchemaModel, owner: TypeDefinition, name: string): FieldDefinition {
  if (owner.kind !== "object" && owner.kind !== "interface") throw new Error(`GraphQL operation: ${owner.name} has no fields to select ${name} from`);
  const field = owner.fields.find((candidate) => candidate.name === name);
  if (!field) throw new Error(`GraphQL operation: field ${name} does not exist on ${owner.name}`);
  return field;
}

function definitionOf(schema: SchemaModel, ref: TypeRef): TypeDefinition | undefined {
  const named = ref.kind === "named" ? ref : definitionRef(ref);
  return schema.types.get(named.name);
}

function definitionRef(ref: TypeRef): { readonly name: string } {
  return ref.kind === "named" ? ref : definitionRef(ref.of);
}

function resultScalar(schema: SchemaModel, name: string): string {
  const definition = schema.types.get(name);
  if (definition?.kind === "enum") return name;
  return PRIMITIVE[name] ?? "any";
}

function resultObject(schema: SchemaModel, owner: TypeDefinition, selections: readonly Selection[]): string {
  const concrete = (type: TypeDefinition, chosen: readonly Selection[]): string => {
    const members: string[] = [`__typename?: '${type.name}'`];
    const seen = new Set<string>();
    const visit = (nodes: readonly Selection[]): void => {
      for (const node of nodes) {
        if (node.kind === "inline") { if (node.typeCondition === undefined || node.typeCondition === type.name) visit(node.selections); continue; }
        if (node.name === "__typename") continue;
        const key = node.alias ?? node.name;
        if (seen.has(key)) continue;
        seen.add(key);
        const field = fieldOf(schema, type, node.name);
        members.push(`${key}${field.type.kind === "nonNull" ? "" : "?"}: ${resultType(schema, field.type, node.selections)}`);
      }
    };
    visit(chosen);
    return `{ ${members.join(", ")} }`;
  };
  if (owner.kind === "object") return concrete(owner, selections);
  const branches = new Map<string, TypeDefinition>();
  for (const node of selections) if (node.kind === "inline" && node.typeCondition) { const type = schema.types.get(node.typeCondition); if (!type) throw new Error(`GraphQL operation: unknown type ${node.typeCondition}`); branches.set(type.name, type); }
  if (branches.size === 0) return `{ __typename?: '${owner.name}' }`;
  return [...branches.values()].map((type) => concrete(type, selections)).join(" | ");
}

function resultType(schema: SchemaModel, ref: TypeRef, selections: readonly Selection[] | undefined): string {
  if (ref.kind === "nonNull") return resultNonNull(schema, ref.of, selections);
  return `${resultNonNull(schema, ref, selections)} | null`;
}

function resultNonNull(schema: SchemaModel, ref: TypeRef, selections: readonly Selection[] | undefined): string {
  if (ref.kind === "nonNull") return resultNonNull(schema, ref.of, selections);
  if (ref.kind === "list") return `Array<${resultType(schema, ref.of, selections)}>`;
  const definition = definitionOf(schema, ref);
  if (!definition || definition.kind === "scalar") return resultScalar(schema, ref.name);
  if (definition.kind === "enum") return definition.name;
  if (selections === undefined) throw new Error(`GraphQL operation: ${definition.name} needs a selection set`);
  return resultObject(schema, definition, selections);
}

function variablesDeclaration(schema: SchemaModel, operation: OperationModel, name: string): string {
  if (operation.variables.length === 0) return `export type ${name} = Exact<{ [key: string]: never; }>;`;
  const lines = operation.variables.map((variable) => `  ${variable.name}${variable.type.kind === "nonNull" && !variable.hasDefault ? "" : "?"}: ${typeText(schema, variable.type, "input")};`);
  return `export type ${name} = Exact<{\n${lines.join("\n")}\n}>;`;
}

/** 🏷️ The TypeScript names of one operation: `Foo` as a query gives `FooQuery` and `FooQueryVariables`. */
export function operationTypeNames(operation: Pick<OperationModel, "kind" | "name">): { readonly result: string; readonly variables: string } {
  const result = `${operation.name}${capitalized(operation.kind)}`;
  return { result, variables: `${result}Variables` };
}

function operationDeclarations(schema: SchemaModel, source: string): string {
  const operation = parseOperation(source);
  const names = operationTypeNames(operation);
  const rootName = schema.roots[operation.kind];
  const root = rootName ? schema.types.get(rootName) : undefined;
  if (!root || root.kind !== "object") throw new Error(`GraphQL operation ${operation.name}: the schema declares no ${operation.kind} root`);
  return `${variablesDeclaration(schema, operation, names.variables)}\n\n\nexport type ${names.result} = ${resultObject(schema, root, operation.selections)};`;
}
//#endregion 🕸️Operations

//#region 📤️Emit
const HELPERS = [
  "export type Maybe<T> = T | null;",
  "export type InputMaybe<T> = Maybe<T>;",
  "export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };",
  "export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };",
  "export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };",
  "export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };",
  'export type Incremental<T> = T | { [P in keyof T]?: P extends " $fragmentName" | "__typename" ? T[P] : never };',
];

/** 📤️ Renders the complete generated module. Pure and deterministic: equal sources give equal bytes. */
export function emitTypeScript(source: GraphqlTypesSource): string {
  const schema = parseSchema(source.schema);
  const definitions = [...schema.types.values()].filter((definition) => definition.kind !== "scalar").sort(byName);
  const operations = [...source.operations].sort(byName).map((operation) => operationDeclarations(schema, operation.source));
  const banner = [
    `/** 🤖️ @generated by \`${GENERATOR_ID}\` from \`${source.schemaRel}\` (sha256 ${source.schemaSha256}) and \`${source.operationsRel}\` (sha256 ${source.operationsSha256}) — do not edit.`,
    ` * Refresh with \`${REFRESH_COMMAND}\`. */`,
  ];
  return `${[banner.join("\n"), HELPERS.join("\n"), scalarsDeclaration(schema), ...definitions.map((definition) => typeDeclaration(schema, definition)), ...operations].join("\n\n")}\n`;
}
//#endregion 📤️Emit
