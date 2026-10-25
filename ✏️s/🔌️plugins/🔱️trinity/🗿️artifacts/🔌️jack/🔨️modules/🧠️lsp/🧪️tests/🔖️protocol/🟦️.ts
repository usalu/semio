import { describe, expect, test } from "vitest";
import schema from "../../🔖️protocol/🔣️.json" with { type: "json" };
import vector from "./🔣️.json" with { type: "json" };

type Step = { send?: { method?: string }; loadSnapshot?: string; raw?: string; expect: unknown[] };
const steps = (vector.sessions as { steps: Step[] }[]).flatMap(session => session.steps);

describe("jack lsp conformance vectors against the protocol schema", () => {
  test("every method a vector sends is declared by the schema, except the one deliberately unknown request", () => {
    const declared = new Set([...schema.requests, ...schema.notifications]);
    const sent = steps.flatMap(step => (step.send?.method ? [step.send.method] : step.loadSnapshot ? ["jack/loadSnapshot"] : []));
    expect(sent.filter(method => !declared.has(method))).toEqual(["textDocument/unknownThing"]);
  });

  test("every declared request has a vector", () => {
    const sent = new Set(steps.flatMap(step => (step.send?.method ? [step.send.method] : step.loadSnapshot ? ["jack/loadSnapshot"] : [])));
    expect(schema.requests.filter(method => !sent.has(method))).toEqual([]);
  });

  test("the initialize vector pins the schema legend, triggers and sync mode", () => {
    const initialize = steps.find(step => step.send?.method === "initialize")!.expect[0] as { result: { capabilities: { semanticTokensProvider: { legend: { tokenTypes: string[] } }; completionProvider: { triggerCharacters: string[] }; textDocumentSync: { change: number } } } };
    expect(initialize.result.capabilities.semanticTokensProvider.legend.tokenTypes).toEqual(schema.semanticTokenTypes);
    expect(initialize.result.capabilities.completionProvider.triggerCharacters).toEqual(schema.capabilities.completionProvider.triggerCharacters);
    expect(initialize.result.capabilities.textDocumentSync.change).toBe(schema.capabilities.textDocumentSync.change);
  });

  test("semantic token classes index the legend", () => {
    for (const index of Object.values(schema.semanticTokenClasses)) expect(schema.semanticTokenTypes[index]).toBeTypeOf("string");
  });
});
