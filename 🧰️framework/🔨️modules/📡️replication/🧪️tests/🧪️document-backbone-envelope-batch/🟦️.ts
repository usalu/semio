import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { DocumentBackboneBatchError, decodeDocumentBackboneEnvelopeBatchExact, encodeDocumentBackboneEnvelopeBatchExact } = dependencies;
  const { describe, expect, it } = vitest;

  type Limits = Readonly<{
    maximumBytes: number;
    maximumEnvelopes: number;
    maximumDependenciesPerEnvelope: number;
    maximumTotalDependencies: number;
    maximumIdentifierBytes: number;
    maximumSchemaBytes: number;
    maximumPayloadBytes: number;
  }>;
  type FixtureEnvelope = Readonly<{
    mutationId: string;
    documentId: string;
    actor: string;
    dependencies: readonly string[];
    diff: Readonly<{ schema: string; payloadHex: string }>;
    inverse: Readonly<{ schema: string; payloadHex: string }>;
    timestamp: Readonly<{ actor: string; physicalMs: string; logical: string }>;
  }>;
  type Fixture = Readonly<{
    schema: string;
    wire: string;
    cases: readonly Readonly<{
      id: string;
      rawHex: string;
      limits: Limits;
      expect: Readonly<{ outcome: "accepted" | "limit" | "malformed"; reason: string; envelopes?: readonly FixtureEnvelope[] }>;
    }>[];
  }>;

  const toHex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  const project = (envelopes: readonly any[]): readonly FixtureEnvelope[] =>
    envelopes.map((envelope) => ({
      mutationId: envelope.mutation_id,
      documentId: envelope.document_id,
      actor: envelope.actor,
      dependencies: envelope.dependencies,
      diff: { schema: envelope.diff.schema, payloadHex: toHex(envelope.diff.payload) },
      inverse: { schema: envelope.inverse.schema, payloadHex: toHex(envelope.inverse.payload) },
      timestamp: { actor: envelope.timestamp.actor.toString(), physicalMs: envelope.timestamp.physical_ms.toString(), logical: envelope.timestamp.logical.toString() },
    }));

  async function load(): Promise<Readonly<{ fixture: Fixture; schema: object }>> {
    const { readFile } = await import("node:fs/promises");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const root = dirname(fileURLToPath(source.url));
    const [fixture, schema] = await Promise.all([
      readFile(join(root, "🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json"), "utf8"),
      readFile(join(root, "🔗️causal/🧬️schema/🧮️document-backbone-batch-v1/🔣️.json"), "utf8"),
    ]);
    return { fixture: JSON.parse(fixture) as Fixture, schema: JSON.parse(schema) as object };
  }

  describe("document backbone envelope batch", () => {
    it("matches the neutral schema and exact bounded causal corpus", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(fixture.schema).toBe("semio.replication.document-backbone-batch.v1");
      expect(fixture.wire).toBe("causal-envelope-batch-v1");

      for (const row of fixture.cases) {
        const bytes = new Uint8Array(Buffer.from(row.rawHex, "hex"));
        expect(toHex(bytes), row.id).toBe(row.rawHex);
        try {
          const decoded = decodeDocumentBackboneEnvelopeBatchExact(bytes, row.limits);
          expect(row.expect.outcome, row.id).toBe("accepted");
          expect(project(decoded), row.id).toEqual(row.expect.envelopes);
          expect(encodeDocumentBackboneEnvelopeBatchExact(decoded), row.id).toEqual(bytes);
        } catch (error) {
          expect(error, row.id).toBeInstanceOf(DocumentBackboneBatchError);
          expect((error as InstanceType<typeof DocumentBackboneBatchError>).outcome, row.id).toBe(row.expect.outcome);
          expect((error as InstanceType<typeof DocumentBackboneBatchError>).reason, row.id).toBe(row.expect.reason);
        }
      }
    });
  });
}
