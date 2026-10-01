import { describe, expect, it } from "bun:test";
import type { WavMutation, WavSnapshot } from "../../../../🧬️schema/🧬️mutations/🟦️";
import { assertWavAudioRevision, wavAudioEditMutations } from "../🟦️";

const fixture = await Bun.file(new URL("../🧫️fixtures/🎚️natural-edit/🔣️.json", import.meta.url)).json() as Readonly<{
  before: WavSnapshot;
  insertChannel: Readonly<{ channel: number; expectedData: readonly number[]; expectedChannels: number; expectedBlockAlign: number; expectedByteRate: number }>;
  setSample: Readonly<{ frame: number; channel: number; value: string; expectedData: readonly number[] }>;
  insertFrame: Readonly<{ frame: number; expectedData: readonly number[] }>;
}>;

const apply = (source: WavSnapshot, mutations: readonly WavMutation[]): WavSnapshot => mutations.reduce<WavSnapshot>((snapshot, mutation) => {
  if (mutation.mutation === "setFmt") return { ...snapshot, fmt: mutation.fmt };
  if (mutation.mutation !== "patchData") throw new Error(`unexpected mutation ${mutation.mutation}`);
  if (snapshot.data.kind !== mutation.data.kind) throw new Error("sample kind changed during patch");
  if (snapshot.data.kind === "float32" && mutation.data.kind === "float32") {
    const value = Array.from(snapshot.data.value);
    value.splice(mutation.index, mutation.removeCount, ...mutation.data.value);
    return { ...snapshot, data: { kind: "float32", value } };
  }
  if (snapshot.data.kind === "float32" || mutation.data.kind === "float32") throw new Error("sample kinds differ");
  const value = Array.from(snapshot.data.value);
  value.splice(mutation.index, mutation.removeCount, ...mutation.data.value);
  return { ...snapshot, data: { kind: snapshot.data.kind, value } };
}, source);

describe("WAV natural audio editing", () => {
  it("uses the neutral fixture to address a sample by frame and channel", () => {
    const mutations = wavAudioEditMutations({ kind: "setSample", frame: fixture.setSample.frame, channel: fixture.setSample.channel, revision: "r", value: fixture.setSample.value }, fixture.before);
    expect(apply(fixture.before, mutations).data.value).toEqual(fixture.setSample.expectedData);
  });

  it("inserts a complete silent frame instead of breaking interleaving", () => {
    const mutations = wavAudioEditMutations({ kind: "insertFrame", frame: fixture.insertFrame.frame, revision: "r" }, fixture.before);
    expect(apply(fixture.before, mutations).data.value).toEqual(fixture.insertFrame.expectedData);
  });

  it("rewrites interleaved channels and derived format metadata coherently", () => {
    const mutations = wavAudioEditMutations({ kind: "insertChannel", channel: fixture.insertChannel.channel, revision: "r" }, fixture.before);
    const edited = apply(fixture.before, mutations);
    expect(edited.data.value).toEqual(fixture.insertChannel.expectedData);
    expect([edited.fmt.channels, edited.fmt.blockAlign, edited.fmt.byteRate]).toEqual([fixture.insertChannel.expectedChannels, fixture.insertChannel.expectedBlockAlign, fixture.insertChannel.expectedByteRate]);
  });

  it("preserves every neutral IEEE word while inserting a silent channel", async () => {
    const corpus = await Bun.file(new URL("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔢️float32.json", import.meta.url)).json() as {ieee754Binary32Bits: number[]};
    const source: WavSnapshot = {
      ...fixture.before,
      fmt: {...fixture.before.fmt, audioFormat: 3, channels: 2, bitsPerSample: 32, blockAlign: 8, byteRate: fixture.before.fmt.sampleRate * 8},
      data: {kind: "float32", value: corpus.ieee754Binary32Bits.map(bits => ({bits}))},
    };
    const edited = apply(source, wavAudioEditMutations({kind: "insertChannel", channel: 1, revision: "r"}, source));
    if (edited.data.kind !== "float32") throw new Error("float sample kind changed");
    expect(edited.data.value.map(word => word.bits)).toEqual(corpus.ieee754Binary32Bits.flatMap((bits, index) => index % 2 === 0 ? [bits, 0] : [bits]));
    expect([edited.fmt.channels, edited.fmt.blockAlign, edited.fmt.byteRate]).toEqual([3, 12, source.fmt.sampleRate * 12]);
  });

  it("pages wide frames within the retained payload bound", () => {
    const channels = 9_000;
    const wide: WavSnapshot = {
      ...fixture.before,
      fmt: { ...fixture.before.fmt, channels, sampleRate: 1, blockAlign: channels * 2, byteRate: channels * 2 },
      data: { kind: "pcm16", value: [...Array.from({ length: channels }, () => 1), ...Array.from({ length: channels }, () => 2)] },
    };
    const appended = wavAudioEditMutations({ kind: "appendFrame", revision: "r" }, wide);
    expect(appended).toHaveLength(2);
    expect(appended.every((mutation) => mutation.mutation === "patchData" && mutation.data.value.length <= 8_192)).toBe(true);
    expect(apply(wide, appended).data.value).toHaveLength(channels * 3);
    const removed = wavAudioEditMutations({ kind: "removeFrame", frame: 0, revision: "r" }, wide);
    expect(removed).toHaveLength(2);
    expect(apply(wide, removed).data.value).toEqual(Array.from({ length: channels }, () => 2));
    expect(() => wavAudioEditMutations({ kind: "insertChannel", channel: 0, revision: "r" }, wide)).toThrow(/transformed frame/u);
  });

  it("rejects negative addresses and incoherent derived format metadata", () => {
    expect(() => wavAudioEditMutations({ kind: "setSample", frame: -1, channel: 0, revision: "r", value: "0" }, fixture.before)).toThrow(/no longer exists/u);
    expect(() => wavAudioEditMutations({ kind: "appendFrame", revision: "r" }, { ...fixture.before, fmt: { ...fixture.before.fmt, blockAlign: 1 } })).toThrow(/metadata differ/u);
  });

  it("rejects raw payloads, stale revisions, and removal of the final channel", () => {
    expect(() => wavAudioEditMutations({ kind: "appendFrame", revision: "r" }, { ...fixture.before, data: { kind: "raw", value: [1, 2] } })).toThrow(/Raw WAV/u);
    expect(() => assertWavAudioRevision({ kind: "appendFrame", revision: "old" }, "new")).toThrow(/changed/u);
    expect(() => wavAudioEditMutations({ kind: "removeChannel", channel: 0, revision: "r" }, { ...fixture.before, fmt: { ...fixture.before.fmt, channels: 1, blockAlign: 2, byteRate: 96_000 }, data: { kind: "pcm16", value: [1, 2, 3] } })).toThrow(/one channel/u);
  });
});
