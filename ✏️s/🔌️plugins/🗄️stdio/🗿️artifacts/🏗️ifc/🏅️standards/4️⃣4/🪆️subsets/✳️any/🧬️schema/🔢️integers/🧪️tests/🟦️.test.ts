/** 🧪️ Independent GraphQL/protobuf parsing retains every native integer bit. */
import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {test} from "node:test";
import {buildSchema, GraphQLObjectType, GraphQLScalarType, Kind, parseValue, validateSchema, valueFromAST} from "graphql";
import protobuf from "protobufjs";
import {parseIfcGraphqlInteger, printIfcGraphqlInteger, ifcGraphqlIndex, type IfcGraphqlIntegerScalar} from "../🟦️.ts";

const source = async (relative: string): Promise<string> => readFile(new URL(relative, import.meta.url), "utf8");
const fixture = JSON.parse(await source("🧫️fixtures/🔣️.json")) as {fields: Record<string, string>; valid: {scalar: IfcGraphqlIntegerScalar; value: string}[]; invalid: {scalar: IfcGraphqlIntegerScalar; value: unknown}[]};

/** 🔗️ Binds the repository-owned codecs through the independent GraphQL scalar API for verification only. */
function scalar(name: IfcGraphqlIntegerScalar): GraphQLScalarType<bigint, string> {
  return new GraphQLScalarType({name, serialize: value => printIfcGraphqlInteger(name, value as bigint), parseValue: value => parseIfcGraphqlInteger(name, value), parseLiteral: value => { if (value.kind !== Kind.STRING) throw new Error("decimal string required"); return parseIfcGraphqlInteger(name, value.value); }});
}

test("IFC4 GraphQL facets expose owned full-width integer scalars", async () => {
  const documents = await Promise.all(["../../📸️snapshot/🔗️.graphql", "../../🔺️diff/🔗️.graphql", "../../🧬️mutations/🔗️.graphql", "../../🔗️.graphql"].map(source));
  const schema = buildSchema('enum StateClass { ARTIFACT } directive @state(class: StateClass!) on FIELD_DEFINITION\n' + documents.join("\n") + '\ntype Query { artifact: IfcArtifact snapshot: IfcSnapshot diff: IfcDiff mutation: IfcMutation }');
  assert.equal(validateSchema(schema).length, 0);
  for (const [path, expected] of Object.entries(fixture.fields)) {
    const [typeName, fieldName] = path.split(".");
    const type = schema.getType(typeName!) as GraphQLObjectType;
    assert.equal(type.getFields()[fieldName!]!.type.toString(), expected, path);
  }
  assert.equal((schema.getType("IfcArtifact") as GraphQLObjectType).getFields().entities!.type.toString(), "[IfcEntity!]!");
});

test("GraphQL values preserve full-width decimal strings and refuse numeric coercion", () => {
  for (const item of fixture.valid) {
    const type = scalar(item.scalar), decoded = valueFromAST(parseValue(JSON.stringify(item.value)), type);
    assert.equal(decoded, BigInt(item.value));
    assert.equal(type.serialize(decoded), item.value);
    assert.equal(valueFromAST(parseValue(item.value), type), undefined);
  }
  for (const item of fixture.invalid) assert.throws(() => scalar(item.scalar).parseValue(item.value));
  assert.equal(ifcGraphqlIndex("9007199254740991"), Number.MAX_SAFE_INTEGER);
  assert.throws(() => ifcGraphqlIndex("9007199254740992"));
  assert.throws(() => printIfcGraphqlInteger("IfcUInt64", 9007199254740992 as unknown as bigint));
});

test("protobuf facets independently retain unsigned identities and signed integer extremes", async () => {
  const root = new protobuf.Root();
  for (const file of ["../../📸️snapshot/🛰️.proto", "../../🔺️diff/🛰️.proto", "../../🧬️mutations/🛰️.proto", "../../🛰️.proto"]) protobuf.parse(await source(file), root);
  root.resolveAll();
  const artifact = root.lookupType("semio.s_stdio_ifc.artifact.IfcArtifact");
  assert.equal(artifact.fields.header!.resolvedType!.name, "IfcHeader");
  assert.equal(artifact.fields.entities!.resolvedType!.name, "IfcEntity");
  const entity = root.lookupType("semio.s_stdio_ifc.snapshot.IfcEntity"), value = root.lookupType("semio.s_stdio_ifc.snapshot.IfcValue"), removed = root.lookupType("semio.s_stdio_ifc.diff.IfcEntityRemoved");
  for (const [type, object] of [[entity, {id: "18446744073709551615"}], [removed, {id: "18446744073709551615", index: "4294967296"}], [value, {intValue: "-9223372036854775808", referenceValue: "18446744073709551615"}], [value, {intValue: "9223372036854775807"}]] as const) {
    const decoded = type.toObject(type.decode(type.encode(type.fromObject(object)).finish()), {longs: String});
    for (const [key, expected] of Object.entries(object)) assert.equal(String(decoded[key]), expected);
  }
});
