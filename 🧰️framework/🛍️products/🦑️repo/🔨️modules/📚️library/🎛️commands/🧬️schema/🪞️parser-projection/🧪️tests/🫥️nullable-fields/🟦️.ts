/** 🫥️ Nullable object fields render as nullable GraphQL fields and `optional` protobuf fields, and the rendered text is accepted by the independent GraphQL and protobuf parsers. */
import { expect, test } from "bun:test";
import { parse } from "graphql";
import protobuf from "protobufjs";
import cases from "./🔣️.json";
import { renderGraphql, renderProto } from "../../📐️type-render/🟦️.ts";

test("nullable fields render as nullable GraphQL and optional protobuf and agree with the independent parsers", () => {
  for (const row of cases.cases) {
    const graphql = renderGraphql("Row", row.definition);
    expect(graphql, row.name).toBe(row.graphql);
    expect(() => parse(`scalar Owner\n${graphql}`), row.name).not.toThrow();
    const proto = renderProto("Row", row.definition);
    expect(proto ?? null, row.name).toBe(row.protobuf);
    if (proto !== undefined) expect(() => protobuf.parse(`syntax = "proto3";\nmessage Owner { string value = 1; }\n${proto}`), row.name).not.toThrow();
  }
});
