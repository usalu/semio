import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "DOCUMENT_BACKBONE_RETENTION_LIMITS" | "DocumentBackboneBatchError" | "decodeDocumentBackboneEnvelopeBatchExact" | "encodeDocumentBackboneEnvelopeBatchExact" | "encodeClientCommandsFrameExact" | "encodeClientFrame">, source: TestSource): Promise<void> {
  const { DOCUMENT_BACKBONE_RETENTION_LIMITS, DocumentBackboneBatchError, decodeDocumentBackboneEnvelopeBatchExact, encodeDocumentBackboneEnvelopeBatchExact, encodeClientCommandsFrameExact, encodeClientFrame } = dependencies;
  const { describe, expect, it } = vitest;

  type Limits = Readonly<{
    maximumBytes: number;
    maximumEnvelopes: number;
    maximumDependenciesPerEnvelope: number;
    maximumTotalDependencies: number;
    maximumTargetSegmentsPerEnvelope: number;
    maximumTotalTargetSegments: number;
    maximumIdentifierBytes: number;
    maximumSchemaBytes: number;
    maximumPayloadBytes: number;
  }>;
  type FixtureEnvelope = Readonly<{
    mutationId: string;
    documentId: string;
    actor: string;
    dependencies: readonly string[];
    observed: string | null;
    target: readonly string[];
    diff: Readonly<{ schema: string; payloadHex: string }>;
    inverse: Readonly<{ schema: string; payloadHex: string }>;
    timestamp: Readonly<{ actor: string; physicalMs: string; logical: string }>;
    transaction: Readonly<{ id: string; tool: string }> | null;
  }>;
  type Fixture = Readonly<{
    schema: string;
    wire: string;
    retention: Readonly<{ maximumBytes: number; maximumMessages: number }>;
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
      observed: envelope.observed,
      target: envelope.target,
      diff: { schema: envelope.diff.schema, payloadHex: toHex(envelope.diff.payload) },
      inverse: { schema: envelope.inverse.schema, payloadHex: toHex(envelope.inverse.payload) },
      timestamp: { actor: envelope.timestamp.actor.toString(), physicalMs: envelope.timestamp.physical_ms.toString(), logical: envelope.timestamp.logical.toString() },
      transaction: envelope.transaction,
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
      expect(fixture.retention).toEqual(DOCUMENT_BACKBONE_RETENTION_LIMITS);

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

    it("sends exact envelopes as a Commands frame keeping every HLC field exact", () => {
      const envelope = {
        mutation_id: "transition-1",
        document_id: "document-1",
        actor: "actor-1",
        dependencies: ["op-1"],
        observed: null,
        target: ["title"],
        diff: { schema: "semio.history-transition.v1", payload: Uint8Array.of(1, 2, 3) },
        inverse: { schema: "semio.history-transition.v1", payload: new Uint8Array() },
        timestamp: { actor: 0xfedc_ba98_7654_3210n, physical_ms: (1n << 53n) + 1n, logical: 1n << 60n },
        transaction: null,
      } as const;
      expect(toHex(encodeClientCommandsFrameExact(7, [envelope], "command")), "the Rust and Python oracle vector").toBe("000107010c7472616e736974696f6e2d310a646f63756d656e742d31076163746f722d3101046f702d310001057469746c651b73656d696f2e686973746f72792d7472616e736974696f6e2e7631030102031b73656d696f2e686973746f72792d7472616e736974696f6e2e76310090e4d0b287d3aeeefe01818080808080801080808080808080801000");
      const small = { ...envelope, timestamp: { actor: 9n, physical_ms: 1_700_000_000_000n, logical: 3n } };
      const numbers = { ...small, diff: { schema: small.diff.schema, payload: [1, 2, 3] }, inverse: { schema: small.inverse.schema, payload: [] }, timestamp: { actor: 9, physical_ms: 1_700_000_000_000, logical: 3 } };
      expect(encodeClientCommandsFrameExact(7, [small], "command"), "within 2^53 both encoders agree").toEqual(encodeClientFrame({ Commands: { batch_id: 7, envelopes: [numbers] } }, "command"));
    });
  });
}
