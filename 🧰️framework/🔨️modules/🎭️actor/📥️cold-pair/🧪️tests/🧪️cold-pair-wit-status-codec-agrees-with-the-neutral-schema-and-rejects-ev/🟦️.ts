type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { parseWitColdPairIngressStatus } = dependencies;

  const { it, expect } = vitest;
  it("cold pair WIT status codec agrees with the neutral schema and rejects every hostile authority", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const validate = new Ajv({ strict: true }).compile(schema);
    const authority = (value: Record<string, unknown>): Record<string, unknown> => ({
      ...value,
      ...(Object.hasOwn(value, "activationGeneration") ? { activationGeneration: BigInt(value.activationGeneration as number) } : {}),
      ...(Object.hasOwn(value, "guestLifetime") ? { guestLifetime: BigInt(value.guestLifetime as number) } : {}),
      ...(Object.hasOwn(value, "transferGeneration") ? { transferGeneration: BigInt(value.transferGeneration as number) } : {}),
    });
    const cursor = (value: Record<string, unknown>): Record<string, unknown> => ({ ...authority(value), lifetime: authority(value.lifetime as Record<string, unknown>) });
    const frontier = (value: Record<string, unknown>): Record<string, unknown> => ({
      ...value,
      headEditOrdinal: BigInt(value.headEditOrdinal as number),
      lastCommitSeq: BigInt(value.lastCommitSeq as number),
    });
    const wit = (value: Record<string, unknown>): Record<string, unknown> => {
      if (value.kind === "idle") return { tag: "idle" };
      if (value.kind === "applied") {
        const receipt = value.receipt as Record<string, unknown>;
        return {
          tag: "applied",
          val: { ...authority(receipt), lifetime: authority(receipt.lifetime as Record<string, unknown>), baselineFrontier: frontier(receipt.baselineFrontier as Record<string, unknown>) },
        };
      }
      if (value.kind === "fault") return { tag: "fault", val: { cursor: cursor(value.cursor as Record<string, unknown>), fault: value.fault } };
      return { tag: value.kind === "pageAccepted" ? "page-accepted" : value.kind, val: cursor(value.cursor as Record<string, unknown>) };
    };
    for (const row of fixture.statusRows) {
      expect(validate(row)).toBe(true);
      expect(parseWitColdPairIngressStatus(wit(row)).kind).toBe(row.kind);
    }
    let structurallyRejected = 0;
    for (const row of fixture.hostileRows) {
      if (!validate(row)) structurallyRejected++;
      expect(() => parseWitColdPairIngressStatus(wit(row))).toThrow();
    }
    expect(structurallyRejected).toBeGreaterThanOrEqual(3);
    expect(() => parseWitColdPairIngressStatus({ tag: "idle", extra: true })).toThrow("cold-pair.status");
    console.log(`cold-pair-status-codec: schema=1 valid=${fixture.statusRows.length} structural-hostile=${structurallyRejected} semantic-hostile=${fixture.hostileRows.length} unknown-fields=refused`);
  });

}
