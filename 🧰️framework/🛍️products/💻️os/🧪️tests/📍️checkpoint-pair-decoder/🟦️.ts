/** 📍️ The canonical checkpoint pair as the worker decodes it: the hub's own framing, every refusal and the genesis applied receipt. */
import type { BackboneWorkerTestDependencies } from "../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../🧰️worker-law-fakes/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerCheckpointPairDecoderTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  describe("canonical checkpoint pair", () => {
    beforeEach(resetWorkerLaw);
    // 🪢️ The hub's `active-checkpoint/pair` body is the ONLY transport that seeds a cold browser
    // client: no hub code path produces `Bootstrap.ArtifactBootstrap`, so the socket's `Welcome`
    // never carries a pair (measured on a live hub: `Welcome.bootstrap = None`, ticket 26/09/18 C7).
    // These laws pin the decoder to the hub's own framing — `u32be length | payload` frames, a
    // selection header, strictly ordered pack-then-SPR records at contiguous offsets, a `Complete`
    // terminal — and to the GENESIS baseline every newly created document's first checkpoint has.
    const frame = (payload: readonly number[]): number[] => [(payload.length >>> 24) & 0xff, (payload.length >>> 16) & 0xff, (payload.length >>> 8) & 0xff, payload.length & 0xff, ...payload];
    const u32 = (value: number): number[] => [(value >>> 24) & 0xff, (value >>> 16) & 0xff, (value >>> 8) & 0xff, value & 0xff];
    const u64 = (value: number): number[] => [0, 0, 0, 0, ...u32(value)];
    const lp = (text: string): number[] => [...u32(new TextEncoder().encode(text).length), ...new TextEncoder().encode(text)];
    const digest = async (bytes: Uint8Array): Promise<number[]> => [...new Uint8Array(await crypto.subtle.digest("SHA-256", bytes.slice().buffer))];

    const body = async (options: { readonly headEditId?: string; readonly headEditOrdinal?: number; readonly lastCommitSeq?: number; readonly chainHash?: number[]; readonly packLength?: number } = {}): Promise<Uint8Array> => {
      const pack = new Uint8Array(options.packLength ?? 5_000).fill(7);
      const spr = new Uint8Array(300).fill(3);
      const header = [
        1,
        ...u32(1),
        ...lp("space-c7"),
        ...lp("artifact-c7"),
        ...new Array(32).fill(11),
        ...new Array(32).fill(12),
        ...lp("artifact-c7"),
        ...u64(options.headEditOrdinal ?? 0),
        ...lp(options.headEditId ?? ""),
        ...u64(options.lastCommitSeq ?? 0),
        ...(options.chainHash ?? new Array(32).fill(0)),
        ...(await digest(pack)),
        ...u64(pack.length),
        ...(await digest(spr)),
        ...u64(spr.length),
        ...(await digest(Uint8Array.from([...pack, ...spr]))),
      ];
      const out = [...frame(header)];
      let ordinal = 0;
      for (const [part, bytes] of [
        [1, pack],
        [2, spr],
      ] as const)
        for (let offset = 0; offset < bytes.length; offset += 4096, ordinal += 1) {
          const slice = [...bytes.subarray(offset, Math.min(offset + 4096, bytes.length))];
          out.push(...frame([2, part, ...u32(ordinal), ...u64(offset), ...u32(slice.length), ...slice]));
        }
      out.push(...frame([3, 0]));
      return Uint8Array.from(out);
    };

    it("decodes the hub's own framing and accepts the genesis baseline a new document's first checkpoint carries", async () => {
      const { decodeCanonicalCheckpointPairV1 } = await import("../../🔨️modules/📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts");
      const decoded = decodeCanonicalCheckpointPairV1(await body());
      expect(decoded.scope).toEqual({ spaceId: "space-c7", documentId: "artifact-c7" });
      expect(decoded.baselineFrontier).toEqual({ documentId: "artifact-c7", headEditOrdinal: 0, headEditId: "", lastCommitSeq: 0, chainHash: new Array(32).fill(0) });
      expect([decoded.packBytes.length, decoded.sprBytes.length]).toEqual([decoded.pack.byteLength, decoded.spr.byteLength]);
      expect([...new Set(decoded.packBytes)]).toEqual([7]);
      expect([...new Set(decoded.sprBytes)]).toEqual([3]);
      const edited = decodeCanonicalCheckpointPairV1(await body({ headEditId: "edit-9", headEditOrdinal: 9, lastCommitSeq: 4, chainHash: new Array(32).fill(6) }));
      expect(edited.baselineFrontier.headEditId).toBe("edit-9");
    });

    it("names every refusal and never half-accepts a body", async () => {
      const { decodeCanonicalCheckpointPairV1 } = await import("../../🔨️modules/📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts");
      const exact = await body();
      const refusals: [string, Uint8Array][] = [
        ["canonical-checkpoint-pair.truncated", exact.subarray(0, exact.length - 1)],
        ["canonical-checkpoint-pair.terminal", Uint8Array.from([...exact.subarray(0, exact.length - 1), 1])],
        ["canonical-checkpoint-pair.baseline-frontier", await body({ headEditOrdinal: 9 })],
        ["canonical-checkpoint-pair.baseline-frontier", await body({ headEditId: "edit-9", headEditOrdinal: 9, lastCommitSeq: 4 })],
      ];
      for (const [code, input] of refusals) expect(() => decodeCanonicalCheckpointPairV1(input)).toThrow(code);
      const reordered = await body();
      // 🔀️ The first data record's part byte flipped to SPR: the decoder must refuse a body whose
      // records do not run pack-then-SPR at contiguous offsets, rather than silently mis-splitting.
      const headerLength = (reordered[0]! << 24) + (reordered[1]! << 16) + (reordered[2]! << 8) + reordered[3]!;
      reordered[4 + headerLength + 4 + 1] = 2;
      expect(() => decodeCanonicalCheckpointPairV1(reordered)).toThrow("canonical-checkpoint-pair.record-part");
    });

    it("accepts a genesis applied receipt from the guest, which the host twin used to refuse", async () => {
      const { parseColdDocumentPairFrontier } = await import("@semio-tech/framework-actor/cold-pair");
      const genesis = { documentId: "artifact-c7", headEditOrdinal: 0n, headEditId: "", lastCommitSeq: 0n, chainSha256: new Uint8Array(32) };
      expect(parseColdDocumentPairFrontier(genesis).headEditId).toBe("");
      expect(() => parseColdDocumentPairFrontier({ ...genesis, headEditOrdinal: 9n })).toThrow("cold-pair.frontier");
      expect(parseColdDocumentPairFrontier({ documentId: "artifact-c7", headEditOrdinal: 9n, headEditId: "edit-9", lastCommitSeq: 4n, chainSha256: new Uint8Array(32).fill(6) }).headEditOrdinal).toBe(9n);
    });
  });
}
