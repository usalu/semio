/** 🔗️ Renders the GraphQL twin of a schema module by extending the framework's type renderer: one type per record the schema keeps in GraphQL and the `JSON` scalar once.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/📐️type-render/🟦️.ts
 */
import { renderGraphql } from "../../🪞️parser-projection/📐️type-render/🟦️.ts";
import { aggregateMembers, withoutMetaProperties, withoutTag, type Node, type SchemaModule } from "../🧩️model/🟦️.ts";

/** 🏷️ True when the schema keeps a record out of GraphQL (`x-semio-formats` without `🔗️graphql`). */
export const keptInGraphql = (node: Node): boolean => !Array.isArray(node["x-semio-formats"]) || node["x-semio-formats"].includes("🔗️graphql");

/** 📝️ The generated GraphQL text of one module. */
export function renderGraphqlTwin(module: SchemaModule, index: ReadonlyMap<string, SchemaModule>): string {
  const root = typeof module.schema.title === "string" ? module.schema.title : "Record";
  const taken = new Set<string>([root, ...module.definitions.keys()]);
  const entries: [string, Node][] = [[root, module.schema], ...[...module.definitions].filter(([, node]) => keptInGraphql(node))];
  const aggregate = aggregateMembers(module, index);
  const blocks = entries.flatMap(([name, node]) => {
    if (aggregate !== undefined && node === module.schema) {
      const payloads = aggregate.flatMap((member) => {
        const text = renderGraphql(member.type, withoutMetaProperties(withoutTag(member.payload.schema)), () => false);
        return text === undefined ? [] : [text.replace(/^scalar JSON\n+/u, "").trimEnd()];
      });
      return [`union ${name} = ${aggregate.map((member) => member.type).join(" | ")}`, ...payloads];
    }
    const text = renderGraphql(name, withoutMetaProperties(node), (candidate) => taken.has(candidate) && candidate !== name);
    return text === undefined ? [] : [text.replace(/^scalar JSON\n+/u, "").trimEnd()];
  });
  const usesJson = blocks.some((block) => /\bJSON\b/u.test(block));
  return [...(usesJson ? ["scalar JSON", ""] : []), ...blocks, ""].join("\n");
}
