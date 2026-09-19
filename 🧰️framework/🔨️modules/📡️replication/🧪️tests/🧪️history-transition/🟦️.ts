import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

/** 🔀️ Validates the neutral `semio.history.transition` payload corpus against its JSON Schema (Ajv) and
 * re-encodes every accepted case with an encoder written from the wire grammar alone, so the corpus is
 * pinned by three independent implementations (fixture generator, Rust codec, this one). */
export async function registerTests3(vitest: NonNullable<ImportMeta["vitest"]>, source: TestSource): Promise<void> {
  const { describe, expect, it } = vitest;

  type Transition = Readonly<Record<string, any>>;
  type Fixture = Readonly<{ schema: string; diffSchema: string; cases: readonly Readonly<{ id: string; payloadHex: string; expect: Readonly<{ outcome: "accepted" | "malformed"; transition?: Transition; detail?: string }> }>[] }>;

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
      readFile(join(root, "🔗️causal/🧫️fixtures/🔀️history-transition-v1/🔣️.json"), "utf8"),
      readFile(join(root, "🔗️causal/🧬️schema/🔀️history-transition-v1/🔣️.json"), "utf8"),
    ]);
    return { fixture: JSON.parse(fixture) as Fixture, schema: JSON.parse(schema) as object };
  }

  describe("history transition payloads", () => {
    it("match the neutral schema and re-encode byte for byte", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      expect(fixture.diffSchema).toBe("semio.history.transition");
      const accepted = fixture.cases.filter((row) => row.expect.outcome === "accepted");
      expect(accepted.map((row) => row.expect.transition?.kind).sort()).toEqual(expect.arrayContaining(["branch", "checkout", "commit", "reinstate", "repin", "revert"]));
      for (const row of accepted) {
        expect(toHex(encode(row.expect.transition!)), row.id).toBe(row.payloadHex);
      }
    });

    it("refuses a transition shape the wire grammar does not define", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      const bogus = { ...fixture, cases: [{ id: "bogus", payloadHex: "06", expect: { outcome: "accepted", transition: { kind: "merge", snapshot: "x" } } }] };
      expect(validate(bogus)).toBe(false);
    });
  });
}
