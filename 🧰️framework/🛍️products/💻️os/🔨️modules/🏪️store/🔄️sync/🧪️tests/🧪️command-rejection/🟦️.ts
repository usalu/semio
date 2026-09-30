/** 🚫️ LAW: every refused command batch reaches a consumer as ONE typed rejection, decoded once. The language-agnostic
 * `🧫️command-rejection` fixture drives the hub decode and every local refusal; Ajv (third party) admits every expected
 * rejection against the schema, and the Rust actor's runner (`🧪️command-rejection/🦀️.rs`) walks the same rows, so the two
 * actors cannot answer two shapes again (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3: a local counter
 * `[69]` in the old `messages: bytes` slot reached the shell's JSON decode as the text `E` and threw out of
 * `worker.onmessage`). The worker wire round trip is pinned for both producers: the TypeScript actor's numbers and the Rust
 * actor's exact-integer carriers, whose local batch ids count down from `u64::MAX`.
 * @see ../../🧬️schema/🔣️command-rejection/🔣️.json */

type Rejection = { readonly kind: "rejected"; readonly code: string; readonly reason: string; readonly messages: readonly unknown[]; readonly detail?: Readonly<Record<string, number>> };
type LocalCode = "local.read-only" | "local.queue-full" | "local.backbone-capacity" | "local.backbone-pair-unavailable" | "local.backbone-scope-mismatch" | "local.backbone-malformed" | "local.socket-frame-ceiling";
type Fixture = {
  readonly hub: readonly { readonly name: string; readonly reason: string; readonly messages: string; readonly expect: Rejection }[];
  readonly local: readonly { readonly code: LocalCode; readonly reason: string; readonly detail?: Readonly<Record<string, number>>; readonly expect: Rejection }[];
};

export async function registerCommandRejectionTests(
  vitest: NonNullable<ImportMeta["vitest"]>,
  twin: {
    readonly COMMAND_REJECTION_CODES_V1: readonly string[];
    readonly BACKBONE_WORKER_WIRE_MAGIC: number;
    readonly commandAckOutcomeOfValueV1: (value: unknown) => unknown;
    readonly decodeBackboneWorkerResponse: (wire: Uint8Array) => unknown;
    readonly decodeHubRejectionMessagesV1: (bytes: ArrayLike<number>) => readonly unknown[] | null;
    readonly encodePackValue: (value: unknown) => Uint8Array;
    readonly hubCommandRejectionV1: (rejected: { readonly reason: string; readonly messages: ArrayLike<number> }) => unknown;
    readonly localCommandRejectionV1: (code: LocalCode, reason: string, detail?: Readonly<Record<string, number>>) => unknown;
    readonly packUInt: (value: bigint) => unknown;
  },
): Promise<void> {
  const { describe, it, expect } = vitest;
  const { default: corpus } = await import("../../../🧫️fixtures/🧫️command-rejection/🔣️.json");
  const { default: schema } = await import("../../🧬️schema/🔣️command-rejection/🔣️.json");
  const { default: historyPatch } = await import("../../../../../../../🔨️modules/🎠️kernel/🧬️schema/🔣️history-patch/🔣️.json");
  const fixture = corpus as unknown as Fixture;
  const bytes = (text: string): Uint8Array => new TextEncoder().encode(text);
  const wire = (event: unknown): Uint8Array => {
    const packed = twin.encodePackValue({ kind: "event", documentId: "doc-1", clientInstanceId: "7b0c2f4e-5d1a-4c3b-9e8f-0a1b2c3d4e5f", event });
    return new Uint8Array([twin.BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
  };

  describe("CommandRejection", () => {
    it("names the schema's closed code set", () => {
      expect([...twin.COMMAND_REJECTION_CODES_V1]).toEqual((schema as { $defs: { Code: { enum: string[] } } }).$defs.Code.enum);
    });

    it("decodes every hub refusal once, into the rejection Ajv admits, never throwing", async () => {
      const { default: Ajv } = await import("ajv");
      const ajv = new Ajv({ strict: false, allErrors: true });
      ajv.addSchema(historyPatch);
      ajv.addSchema(schema, "rejection");
      const valid = ajv.getSchema("rejection#/$defs/Outcome")!;
      for (const row of fixture.hub) {
        expect(valid(row.expect), `${row.name} ${JSON.stringify(valid.errors)}`).toBe(true);
        const decoded = twin.hubCommandRejectionV1({ reason: row.reason, messages: bytes(row.messages) });
        expect(decoded, row.name).toEqual(row.expect);
        expect(twin.decodeHubRejectionMessagesV1(bytes(row.messages)), row.name).toEqual(row.expect.code === "hub.unreadable" ? null : row.expect.messages);
      }
      for (const row of fixture.local) {
        expect(valid(row.expect), `${row.code} ${JSON.stringify(valid.errors)}`).toBe(true);
        expect(twin.localCommandRejectionV1(row.code, row.reason, row.detail), row.code).toEqual(row.expect);
      }
      expect(valid({ kind: "rejected", code: "local.queue-full", reason: "r", messages: [{ level: "error", code: "c", message: "m" }] })).toBe(false);
      expect(valid({ kind: "rejected", code: "hub.refused", reason: "r", messages: [], detail: { envelopes: 1 } })).toBe(false);
      expect(valid({ kind: "rejected", code: "hub.refused", reason: "r", messages: [69] })).toBe(false);
    });

    it("carries every rejection across the worker wire from either actor", () => {
      const rows = [...fixture.hub.map((row) => row.expect), ...fixture.local.map((row) => row.expect)];
      for (const [index, outcome] of rows.entries()) {
        const typescript = twin.decodeBackboneWorkerResponse(wire({ kind: "commandOutcome", batchId: -1 - index, outcome }));
        expect(typescript, JSON.stringify(outcome)).toEqual({ kind: "event", documentId: "doc-1", clientInstanceId: "7b0c2f4e-5d1a-4c3b-9e8f-0a1b2c3d4e5f", event: { kind: "commandOutcome", batchId: -1 - index, outcome } });
        const carried = { ...outcome, messages: outcome.messages.map((message) => ("opIndex" in (message as object) ? { ...(message as object), opIndex: twin.packUInt(BigInt((message as { opIndex: number }).opIndex)) } : message)), ...(outcome.detail === undefined ? {} : { detail: Object.fromEntries(Object.entries(outcome.detail).map(([key, value]) => [key, twin.packUInt(BigInt(value))])) }) };
        const rust = twin.decodeBackboneWorkerResponse(wire({ kind: "commandOutcome", batchId: twin.packUInt((1n << 64n) - 1n - BigInt(index)), outcome: carried }));
        expect((rust as { event: unknown }).event, JSON.stringify(outcome)).toEqual({ kind: "commandOutcome", batchId: -1 - index, outcome });
      }
      expect((twin.decodeBackboneWorkerResponse(wire({ kind: "commandOutcome", batchId: twin.packUInt(7n), outcome: { kind: "accepted" } })) as { event: unknown }).event).toEqual({ kind: "commandOutcome", batchId: 7, outcome: { kind: "accepted" } });
    });

    it("refuses the old counter-in-messages shape and every foreign outcome at the wire", () => {
      const hostile = [
        { kind: "rejected", reason: "document backbone scope mismatch", messages: [69] },
        { kind: "rejected", code: "local.nope", reason: "r", messages: [] },
        { kind: "rejected", code: "local.queue-full", reason: "r", messages: [], detail: { queued: 1 } },
        { kind: "rejected", code: "local.queue-full", reason: "r", messages: [], detail: { envelopes: -1 } },
        { kind: "accepted", reason: "r" },
        { kind: "rejected", code: "hub.refused", reason: "r", messages: [], extra: 1 },
      ];
      for (const outcome of hostile) {
        expect(twin.commandAckOutcomeOfValueV1(outcome), JSON.stringify(outcome)).toBeNull();
        expect(() => twin.decodeBackboneWorkerResponse(wire({ kind: "commandOutcome", batchId: -1, outcome })), JSON.stringify(outcome)).toThrow("invalid command outcome");
      }
    });
  });
}
