/** 🛰️ Renders the protobuf twin of a schema module by extending the framework's type renderer: the file header, one message per record the schema keeps in protobuf, and the state lane of each field as a comment.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📐️type-render/🟦️.ts
 */
import { renderProto } from "../../🪞️parser-projection/📐️type-render/🟦️.ts";
import { snake } from "../../🪞️parser-projection/🔡️json-text/🟦️.ts";
import { aggregateMembers, resolveNode, withIntegerFormats, withoutMetaProperties, withoutTag, type Node, type SchemaModule } from "../🧩️model/🟦️.ts";

/** 📦️ The protobuf package a document id names: `…/s/norm/en1990/artifact.json` to `semio.s.norm.en1990`. */
export function protoPackage(id: string): string {
  const path = id.replace(/^https?:\/\/[^/]+\//u, "").split("/").slice(0, -1);
  return ["semio", ...path.map((segment) => segment.replace(/[^A-Za-z0-9_]/gu, "_"))].join(".");
}

/** 🏷️ True when the schema keeps a record out of protobuf (`x-semio-formats` without `🛰️protobuf`). */
export const keptInProtobuf = (node: Node): boolean => !Array.isArray(node["x-semio-formats"]) || node["x-semio-formats"].includes("🛰️protobuf");

/** 🔢️ The protobuf scalar a nullable scalar property keeps instead of the framework's `bytes` carrier, or `undefined` when the property is no scalar. */
function nullableScalar(member: Node, module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): string | undefined {
  const kinds: unknown[] = Array.isArray(member.type) ? member.type : [member.type];
  const alternatives: Node[] | undefined = member.anyOf ?? member.oneOf;
  const picked: Node | undefined = alternatives === undefined ? { ...member, type: kinds.find((kind) => kind !== "null") } : alternatives.find((alternative) => alternative.type !== "null");
  const base = picked === undefined ? undefined : resolveNode(picked, module, index);
  if (base?.enum !== undefined) return "string";
  switch (base?.type) {
    case "string":
      return "string";
    case "number":
      return "double";
    case "boolean":
      return "bool";
    case "integer":
      return typeof base.minimum === "number" && base.minimum >= 0 ? (typeof base.maximum === "number" && base.maximum <= 4294967295 ? "uint32" : "uint64") : "int64";
    default:
      return undefined;
  }
}

/** 🔤️ The SCREAMING_SNAKE spelling of a name (`hotRolled` to `HOT_ROLLED`). */
const screaming = (name: string): string => snake(name).replace(/[^A-Za-z0-9]+/gu, "_").replace(/^_+|_+$/gu, "").toUpperCase();

/** 🔢️ A string enum whose members are protobuf names as an `enum` (zero-based, each member prefixed by the enum name), or `undefined` when a member has no name or two share one. */
function enumDeclaration(name: string, node: Node): string | undefined {
  const members: unknown[] | undefined = node.type === "string" && Array.isArray(node.enum) ? node.enum : undefined;
  if (members === undefined || members.length === 0 || !members.every((member) => typeof member === "string")) return undefined;
  const names = members.map((member) => `${screaming(name)}_${screaming(member as string)}`);
  if (members.some((member) => screaming(member as string) === "") || new Set(names).size !== names.length) return undefined;
  return `enum ${name} {\n${names.map((entry, at) => `  ${entry} = ${at};`).join("\n")}\n}`;
}

/** 🧺️ The `oneof` message of a mutation aggregate: one field per payload. */
function aggregateMessage(name: string, members: readonly { readonly variant: string; readonly type: string }[]): string {
  return `message ${name} {\n  oneof kind {\n${members.map((member, at) => `    ${member.type} ${snake(member.variant)} = ${at + 1};`).join("\n")}\n  }\n}`;
}

/** 🏛️ Puts the `@state` comment of each property above its field line and keeps a nullable scalar a scalar, when the message declares the properties in order. */
function withStates(message: string, node: Node, module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): string {
  const properties = Object.entries<Node>(node.properties ?? {}).map(([key, member]) => ({ key: snake(key), state: member["x-semio-state"], scalar: nullableScalar(member, module, index) }));
  if (properties.length === 0) return message;
  return message
    .split("\n")
    .flatMap((line) => {
      const field = /^ {2}(?:optional |repeated )?(?:map<[^>]+>|[A-Za-z0-9_.]+) ([a-z0-9_]+) = \d+;$/u.exec(line)?.[1];
      const entry = field === undefined ? undefined : properties.find((property) => property.key === field);
      if (entry === undefined) return [line];
      const typed = entry.scalar !== undefined && line.startsWith("  optional bytes ") ? line.replace("optional bytes", `optional ${entry.scalar}`) : line;
      return typeof entry.state === "string" ? [`  // @state ${entry.state}`, typed] : [typed];
    })
    .join("\n");
}

/** 📝️ The generated protobuf text of one module. */
export function renderProtobuf(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): string {
  const root = typeof module.schema.title === "string" ? module.schema.title : "Record";
  const taken = new Set<string>([root, ...module.definitions.keys()]);
  const entries: [string, Node][] = [[root, module.schema], ...[...module.definitions].filter(([, node]) => keptInProtobuf(node))];
  const aggregate = aggregateMembers(module, index);
  const messages = entries.flatMap(([name, node]) => {
    if (aggregate !== undefined && node === module.schema) {
      const payloads = aggregate.flatMap((member) => {
        const text = renderProto(member.type, withIntegerFormats(withoutMetaProperties(withoutTag(member.payload.schema))), () => false);
        return text === undefined ? [] : [withStates(text.trimEnd(), member.payload.schema, member.payload, index)];
      });
      return [aggregateMessage(name, aggregate), ...payloads];
    }
    const declared = enumDeclaration(name, node);
    if (declared !== undefined) return [declared];
    const text = renderProto(name, withIntegerFormats(withoutMetaProperties(node)), (candidate) => taken.has(candidate) && candidate !== name);
    return text === undefined ? [] : [withStates(text.trimEnd(), node, module, index)];
  });
  return ["syntax = \"proto3\";", `package ${protoPackage(module.id)};`, ...messages, ""].join("\n");
}
