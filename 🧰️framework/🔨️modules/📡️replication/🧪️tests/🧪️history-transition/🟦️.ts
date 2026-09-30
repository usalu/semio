import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

/** 🔀️ Validates the neutral `semio.history.transition` payload corpus against its JSON Schema (Ajv) and
 * re-encodes every accepted case with an encoder written from the wire grammar alone, so the corpus is
 * pinned by three independent implementations (the Python generator `./🐍️.py`, the Rust codec, this one). */
export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, source: TestSource, diffSchema: string, dependencies: Pick<typeof import("../../🟦️.ts"), "historyTransitionId">): Promise<void> {
  const { historyTransitionId } = dependencies;
  const { describe, expect, it } = vitest;

  type Transition = Readonly<Record<string, any>>;
  type Fixture = Readonly<{ schema: string; diffSchema: string; idClock: Readonly<{ actor: number; physicalMs: number; logical: number }>; cases: readonly Readonly<{ id: string; payloadHex: string; expect: Readonly<{ outcome: "accepted" | "malformed"; transition?: Transition; transitionId?: string; detail?: string }> }>[] }>;

  const varint = (value: number): number[] => {
    const out: number[] = [];
    let rest = value;
    while (rest >= 0x80) {
      out.push((rest & 0x7f) | 0x80);
      rest = Math.floor(rest / 0x80);
    }
    out.push(rest);
    return out;
  };
  const text = (value: string): number[] => {
    const bytes = Array.from(new TextEncoder().encode(value));
    return [...varint(bytes.length), ...bytes];
  };
  const optional = (value: string | null): number[] => (value === null ? [0] : [1, ...text(value)]);
  const ids = (values: readonly string[]): number[] => [...varint(values.length), ...values.flatMap(text)];
  const hex = (value: string): number[] => Array.from({ length: value.length / 2 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
  const replacement = (value: Transition): number[] => {
    if (value.kind === "withdrawn") return [1];
    const payload = hex(value.payloadHex);
    return [0, ...text(value.schema), ...varint(payload.length), ...payload];
  };
  const encode = (transition: Transition): number[] => {
    switch (transition.kind) {
      case "revert":
        return [...varint(0), ...ids(transition.mutationIds)];
      case "reinstate":
        return [...varint(1), ...ids(transition.mutationIds)];
      case "commit":
        return [
          ...varint(2),
          ...text(transition.checkpointId),
          ...optional(transition.parentId),
          ...text(transition.changeId),
          ...ids(transition.mutationIds),
          ...optional(transition.description),
          ...text(transition.savedAt),
          ...varint(transition.authors.length),
          ...transition.authors.flatMap((author: Transition) => [...text(author.id), ...text(author.name), ...optional(author.avatar)]),
          ...optional(transition.message),
          ...text(transition.timestamp),
        ];
      case "branch":
        return [...varint(3), ...text(transition.alternativeId), ...text(transition.name), ...text(transition.checkpointId)];
      case "checkout":
        return [...varint(4), ...text(transition.checkpointId), ...optional(transition.alternativeId)];
      case "repin":
        return [...varint(5), ...text(transition.checkpointId), ...text(transition.pinnedCheckpointId), ...varint(transition.pins.length), ...transition.pins.flatMap((pin: Transition) => [...text(pin.childUri), ...text(pin.checkpointId)])];
      case "supersede":
        return [...varint(6), ...optional(transition.scope), ...varint(transition.inputs.length), ...transition.inputs.flatMap((input: Transition) => [...text(input.target), ...replacement(input.replacement)])];
      default:
        throw new Error(`unknown transition kind ${transition.kind}`);
    }
  };
  const toHex = (bytes: readonly number[]): string => bytes.map((byte) => byte.toString(16).padStart(2, "0")).join("");

  async function load(): Promise<Readonly<{ fixture: Fixture; schema: object }>> {
    const { readFile } = await import("node:fs/promises");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const root = dirname(fileURLToPath(source.url));
    const [fixture, schema] = await Promise.all([
      readFile(join(root, "🔗️causal/🧫️fixtures/🧫️history-transition/🔣️.json"), "utf8"),
      readFile(join(root, "🔗️causal/🧬️schema/🔣️history-transition/🔣️.json"), "utf8"),
    ]);
    return { fixture: JSON.parse(fixture) as Fixture, schema: JSON.parse(schema) as object };
  }

  describe("history transition payloads", () => {
    it("match the neutral schema and re-encode byte for byte", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(fixture.diffSchema).toBe(diffSchema);
      const accepted = fixture.cases.filter((row) => row.expect.outcome === "accepted");
      expect(accepted.map((row) => row.expect.transition?.kind).sort()).toEqual(expect.arrayContaining(["branch", "checkout", "commit", "reinstate", "repin", "revert", "supersede"]));
      const clock = { actor: fixture.idClock.actor, physical_ms: fixture.idClock.physicalMs, logical: fixture.idClock.logical };
      for (const row of accepted) {
        expect(toHex(encode(row.expect.transition!)), row.id).toBe(row.payloadHex);
        expect(historyTransitionId(clock, hex(row.payloadHex)), row.id).toBe(row.expect.transitionId);
      }
    });

    it("refuses a transition shape the wire grammar does not define", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      const bogus = { ...fixture, cases: [{ id: "bogus", payloadHex: "07", expect: { outcome: "accepted", transition: { kind: "merge", snapshot: "x" } } }] };
      expect(validate(bogus)).toBe(false);
      const empty = { ...fixture, cases: [{ id: "empty", payloadHex: "060000", expect: { outcome: "accepted", transition: { kind: "supersede", scope: null, inputs: [] } } }] };
      expect(validate(empty)).toBe(false);
      const halfInput = { ...fixture, cases: [{ id: "half", payloadHex: "06", expect: { outcome: "accepted", transition: { kind: "supersede", scope: null, inputs: [{ target: "op", replacement: { kind: "input", schema: "s" } }] } } }] };
      expect(validate(halfInput)).toBe(false);
    });
  });
}
