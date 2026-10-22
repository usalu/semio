import "./🔒️producer/🟦️.ts";
/** 🔗️ Language-agnostic proof of the generated GraphQL types: graphql-js reads the same schema and operations, and every claim of the owned reader and projection is compared against it. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildSchema, isEnumType, isInputObjectType, isInterfaceType, isNonNullType, isObjectType, isScalarType, isUnionType, parse, validate, type GraphQLArgument, type GraphQLNamedType, type OperationDefinitionNode, type SelectionSetNode } from "graphql";
import ts from "typescript";
import { generatedTargets } from "../📋️plan/🟦️.ts";
import { parseOperation, parseSchema } from "../📖️parse/🟦️.ts";
import { emitTypeScript, operationTypeNames } from "../📽️projection/🟦️.ts";
import { readGraphqlTypesSource } from "../📥️source/🟦️.ts";

//#region 🔌️Adapters
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const source = await readGraphqlTypesSource(repoRoot);
const generated = emitTypeScript(source);
const reference = buildSchema(source.schema);
const syntax = ts.createSourceFile("generated.ts", generated, ts.ScriptTarget.Latest, true);
const referenceTypes = Object.values(reference.getTypeMap()).filter((type): type is GraphQLNamedType => !type.name.startsWith("__"));
//#endregion 🔌️Adapters

//#region 🧾️Readers
function declarationOf(name: string): ts.Node {
  const found = syntax.statements.find((statement) => (ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement)) && statement.name.text === name);
  if (!found) throw new Error(`generated module declares no ${name}`);
  return found;
}

function literalOf(node: ts.TypeNode): ts.TypeLiteralNode {
  let current = node;
  if (ts.isTypeReferenceNode(current) && current.typeName.getText(syntax) === "Exact") current = current.typeArguments![0]!;
  if (ts.isIntersectionTypeNode(current)) current = current.types.find(ts.isTypeLiteralNode) ?? current;
  if (!ts.isTypeLiteralNode(current)) throw new Error(`not an object type: ${node.getText(syntax)}`);
  return current;
}

function members(literal: ts.TypeLiteralNode): Map<string, { optional: boolean; type: ts.TypeNode | undefined }> {
  return new Map(literal.members.filter(ts.isPropertySignature).map((member) => [member.name.getText(syntax), { optional: member.questionToken !== undefined, type: member.type }] as const));
}

function selectionKeys(set: SelectionSetNode): Set<string> {
  const keys = new Set<string>();
  for (const selection of set.selections) {
    if (selection.kind === "Field") { if (selection.name.value !== "__typename") keys.add((selection.alias ?? selection.name).value); }
    else if (selection.kind === "InlineFragment") for (const key of selectionKeys(selection.selectionSet)) keys.add(key);
  }
  return keys;
}

function resultLiteral(node: ts.TypeNode): ts.TypeLiteralNode[] {
  if (ts.isTypeLiteralNode(node)) return [node];
  if (ts.isUnionTypeNode(node)) return node.types.flatMap(resultLiteral);
  if (ts.isTypeReferenceNode(node) && node.typeName.getText(syntax) === "Array") return resultLiteral(node.typeArguments![0]!);
  return [];
}

function compareSelection(set: SelectionSetNode, node: ts.TypeNode, path: string): void {
  const literals = resultLiteral(node);
  expect(literals.length, `${path} has an object type`).toBeGreaterThan(0);
  const expected = selectionKeys(set);
  const actual = new Set(literals.flatMap((literal) => [...members(literal).keys()]).filter((key) => key !== "__typename"));
  expect([...actual].sort(), path).toEqual([...expected].sort());
  for (const selection of set.selections) {
    if (selection.kind !== "Field" || !selection.selectionSet) continue;
    const key = (selection.alias ?? selection.name).value;
    const child = literals.map((literal) => members(literal).get(key)?.type).find(Boolean);
    expect(child, `${path}.${key}`).toBeDefined();
    compareSelection(selection.selectionSet, child!, `${path}.${key}`);
  }
}
//#endregion 🧾️Readers

//#region 🧪️Tests
test("every operation canonical repository clients send validates against the served schema in graphql-js", () => {
  expect(source.operations.length).toBeGreaterThan(0);
  for (const operation of source.operations) expect(validate(reference, parse(operation.source)).map((error) => error.message), operation.name).toEqual([]);
});

test("the owned reader and graphql-js agree on every type, field, argument, member and enum value of the schema", () => {
  const owned = parseSchema(source.schema);
  const builtIn = new Set(["ID", "String", "Boolean", "Int", "Float"]);
  expect([...owned.types.keys()].sort()).toEqual(referenceTypes.filter((type) => !builtIn.has(type.name)).map((type) => type.name).sort());
  for (const type of referenceTypes) {
    const definition = owned.types.get(type.name);
    if (builtIn.has(type.name)) continue;
    if (isScalarType(type)) expect(definition?.kind).toBe("scalar");
    else if (isEnumType(type)) expect(definition).toEqual({ kind: "enum", name: type.name, values: type.getValues().map((value) => value.name) });
    else if (isUnionType(type)) expect(definition).toEqual({ kind: "union", name: type.name, members: type.getTypes().map((member) => member.name) });
    else {
      const kind = isObjectType(type) ? "object" : isInterfaceType(type) ? "interface" : "input";
      expect(definition?.kind).toBe(kind);
      const fields = Object.values(type.getFields());
      const ownedFields = (definition as unknown as { fields: { name: string; type: { kind: string }; arguments: { name: string }[] }[] }).fields;
      expect(ownedFields.map((field) => field.name)).toEqual(fields.map((field) => field.name));
      for (const field of fields) {
        const mine = ownedFields.find((candidate) => candidate.name === field.name)!;
        expect(mine.type.kind === "nonNull", `${type.name}.${field.name} nullability`).toBe(isNonNullType(field.type));
        if ("args" in field) expect(mine.arguments.map((argument) => argument.name)).toEqual(field.args.map((argument: GraphQLArgument) => argument.name));
      }
    }
  }
});

test("the generated module declares exactly the schema types, with graphql-js nullability on every field and the enum values of the schema", () => {
  const declared = syntax.statements.filter((statement) => ts.isTypeAliasDeclaration(statement) || ts.isEnumDeclaration(statement)).map((statement) => (statement as ts.TypeAliasDeclaration).name.text);
  for (const type of referenceTypes) {
    if (isScalarType(type)) continue;
    expect(declared, type.name).toContain(type.name);
    const declaration = declarationOf(type.name);
    if (isEnumType(type)) {
      const values = (declaration as ts.EnumDeclaration).members.map((member) => (member.initializer as ts.StringLiteral).text);
      expect(values.sort()).toEqual(type.getValues().map((value) => value.name).sort());
    } else if (isUnionType(type)) expect(((declaration as ts.TypeAliasDeclaration).type as ts.UnionTypeNode).types.map((member) => member.getText(syntax)).sort()).toEqual(type.getTypes().map((member) => member.name).sort());
    else if (isObjectType(type) || isInterfaceType(type) || isInputObjectType(type)) {
      const found = members(literalOf((declaration as ts.TypeAliasDeclaration).type));
      const fields = Object.values(type.getFields());
      expect([...found.keys()].filter((name) => name !== "__typename").sort()).toEqual(fields.map((field) => field.name).sort());
      for (const field of fields) expect(found.get(field.name)!.optional, `${type.name}.${field.name} optionality`).toBe(!isNonNullType(field.type));
      if (isObjectType(type) || isInterfaceType(type)) for (const field of fields) if (field.args.length > 0) {
        const argumentsDeclaration = declarationOf(`${type.name}${field.name[0]!.toUpperCase()}${field.name.slice(1)}Args`);
        expect([...members(literalOf((argumentsDeclaration as ts.TypeAliasDeclaration).type)).keys()].sort(), `${type.name}.${field.name} arguments`).toEqual(field.args.map((argument: GraphQLArgument) => argument.name).sort());
      }
    }
  }
});

test("every operation result type selects exactly the fields of its document in graphql-js, and its variables are the declared ones", () => {
  for (const operation of source.operations) {
    const document = parse(operation.source).definitions[0] as OperationDefinitionNode;
    const names = operationTypeNames(parseOperation(operation.source));
    expect(names.result).toBe(`${document.name!.value}${document.operation[0]!.toUpperCase()}${document.operation.slice(1)}`);
    compareSelection(document.selectionSet, (declarationOf(names.result) as ts.TypeAliasDeclaration).type, names.result);
    const variables = literalOf((declarationOf(names.variables) as ts.TypeAliasDeclaration).type);
    expect([...members(variables).keys()].filter((key) => key !== "[key: string]").sort(), names.variables).toEqual((document.variableDefinitions ?? []).map((variable) => variable.variable.name.value).sort());
  }
});

test("the generated file on disk is the exact byte plan of the generator", () => {
  for (const target of generatedTargets(source)) {
    expect(existsSync(target.path), target.path).toBe(true);
    expect(readFileSync(target.path, "utf8")).toBe(target.content);
  }
});

test("the owned reader refuses constructs it does not model instead of guessing", () => {
  expect(() => parseOperation("query A { ...Spread }")).toThrow("fragment spreads are not supported");
  expect(() => parseOperation("query A { a } query B { b }")).toThrow("exactly one operation");
  expect(() => parseSchema("type A { a: Int } type A { b: Int }")).toThrow("duplicate definition of A");
  expect(() => parseSchema("directive @x on FIELD")).toThrow("unsupported schema definition");
  expect(() => parseSchema("type A { a: }")).toThrow("expected a name");
});
//#endregion 🧪️Tests
