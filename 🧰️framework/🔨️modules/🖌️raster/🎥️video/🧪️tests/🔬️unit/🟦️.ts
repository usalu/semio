import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { AvcPcmEncoder, avcCodecString, avcConfigurationFromRecord, avcDecoderConfigurationRecord, avcLevelIdc, bt601Cb, bt601Cr, bt601Luma, type EncodedVideoSample, encodeVideoRuns, VideoEncodeError, type VideoEncoderPort, videoDurationMilliseconds, videoMacroblocks, writeAvcMp4 } from "../../🟦️.ts";

/** 🎥️ TypeScript twin of `🧪️tests/🔬️unit/🦀️.rs`, driven from the SAME fixture (`🧫️fixtures/🔣️.json`): both twins of
 * the raster video tier write byte-identical H.264 + MP4 for every case (the same `mp4Sha256`), and
 * `🧪️tests/🎞️ffmpeg-decode` decodes exactly those bytes with FFmpeg. */

type Rgb = readonly [number, number, number];

interface VideoCase {
  readonly id: string;
  readonly width: number;
  readonly height: number;
  readonly fps: number;
  readonly runs: readonly { readonly rgb?: Rgb; readonly quadrants?: readonly Rgb[]; readonly frames: number }[];
  readonly expected: {
    readonly frameCount: number;
    readonly syncSamples: readonly number[];
    readonly durationMs: number;
    readonly macroblocks: { readonly width: number; readonly height: number };
    readonly levelIdc: number;
    readonly codec: string;
    readonly spsHex: string;
    readonly ppsHex: string;
    readonly sampleSizes: readonly number[];
    readonly mp4Bytes: number;
    readonly mp4Sha256: string;
    readonly yuv: readonly (readonly Rgb[])[];
    readonly mp4Hex?: string;
  };
}

interface VideoFixture {
  readonly cases: readonly VideoCase[];
  readonly refusals: readonly { readonly id: string; readonly width: number; readonly height: number; readonly fps: number; readonly error: string }[];
}

function loadFixture(): VideoFixture {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🔣️.json"), "utf8")) as VideoFixture;
}

function picture(testCase: VideoCase, run: VideoCase["runs"][number]): Uint8Array {
  const rgba = new Uint8Array(testCase.width * testCase.height * 4);
  for (let y = 0; y < testCase.height; y += 1) {
    for (let x = 0; x < testCase.width; x += 1) {
      const rgb = run.quadrants ? run.quadrants[(y < testCase.height / 2 ? 0 : 2) + (x < testCase.width / 2 ? 0 : 1)]! : run.rgb!;
      rgba.set([rgb[0], rgb[1], rgb[2], 255], (y * testCase.width + x) * 4);
    }
  }
  return rgba;
}

const hex = (bytes: Uint8Array): string => [...bytes].map((byte) => byte.toString(16).padStart(2, "0")).join("");

/** ⚖️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export function testRasterVideoAvcPcmContract(): void {
  const fixture = loadFixture();
  for (const testCase of fixture.cases) {
    const parameters = { width: testCase.width, height: testCase.height, fps: testCase.fps };
    const encoder = new AvcPcmEncoder(parameters);
    const samples: EncodedVideoSample[] = [];
    const record = (sample: EncodedVideoSample): EncodedVideoSample => (samples.push(sample), sample);
    const recording: VideoEncoderPort = { parameters, configuration: () => encoder.configuration(), encode: (rgba) => record(encoder.encode(rgba)), repeat: () => record(encoder.repeat()) };
    const reported: [number, number][] = [];
    const mp4 = encodeVideoRuns(recording, testCase.runs.map((run) => ({ rgba: picture(testCase, run), frames: run.frames })), (done, total) => reported.push([done, total]));
    const { expected } = testCase;
    assert.equal(samples.length, expected.frameCount, `${testCase.id}: frame count`);
    assert.deepEqual(samples.flatMap((sample, index) => (sample.sync ? [index + 1] : [])), expected.syncSamples, `${testCase.id}: every run starts on an IDR, every repeat is a P picture`);
    assert.deepEqual(reported.at(-1), [expected.frameCount, expected.frameCount], `${testCase.id}: progress ends at total`);
    assert.equal(videoDurationMilliseconds(samples.length, parameters.fps), expected.durationMs, `${testCase.id}: duration`);
    assert.deepEqual(videoMacroblocks(parameters), expected.macroblocks, `${testCase.id}: macroblocks`);
    assert.equal(avcLevelIdc(parameters), expected.levelIdc, `${testCase.id}: level`);
    assert.equal(avcCodecString(encoder.configuration()), expected.codec, `${testCase.id}: codec string`);
    assert.equal(hex(encoder.configuration().sps), expected.spsHex, `${testCase.id}: SPS bytes`);
    assert.equal(hex(encoder.configuration().pps), expected.ppsHex, `${testCase.id}: PPS bytes`);
    assert.deepEqual(samples.map((sample) => sample.data.length), expected.sampleSizes, `${testCase.id}: sample sizes`);
    assert.equal(mp4.length, expected.mp4Bytes, `${testCase.id}: container size`);
    assert.equal(createHash("sha256").update(mp4).digest("hex"), expected.mp4Sha256, `${testCase.id}: container digest`);
    if (expected.mp4Hex !== undefined) assert.equal(hex(mp4), expected.mp4Hex, `${testCase.id}: container bytes`);
    testCase.runs.forEach((run, index) => assert.deepEqual((run.quadrants ?? [run.rgb!]).map((rgb) => [bt601Luma(...rgb), bt601Cb(...rgb), bt601Cr(...rgb)]), expected.yuv[index], `${testCase.id}: BT.601 samples`));
    const avcC = avcDecoderConfigurationRecord(encoder.configuration());
    const parsed = avcConfigurationFromRecord(avcC);
    assert.deepEqual([hex(parsed.sps), hex(parsed.pps)], [expected.spsHex, expected.ppsHex], `${testCase.id}: avcC round trip`);
    assert.equal(avcDecoderConfigurationRecord(parsed), avcC, `${testCase.id}: a platform record is written verbatim`);
  }
  for (const refusal of fixture.refusals) {
    assert.throws(() => new AvcPcmEncoder(refusal), (error: unknown) => error instanceof VideoEncodeError && error.code === refusal.error, refusal.id);
  }
  const parameters = { width: 32, height: 16, fps: 30 };
  const encoder = new AvcPcmEncoder(parameters);
  assert.throws(() => encoder.repeat(), (error: unknown) => error instanceof VideoEncodeError && error.code === "nothingToRepeat", "a repeat needs a picture first");
  const rgba = new Uint8Array(32 * 16 * 4);
  let done = 0;
  assert.throws(
    () => encodeVideoRuns(encoder, [{ rgba, frames: 3 }], (frames) => (done = frames), () => done >= 1),
    (error: unknown) => error instanceof VideoEncodeError && error.code === "cancelled",
  );
  assert.equal(done, 1, "cancellation stops before the next frame");
  assert.throws(() => encoder.encode(rgba.subarray(4)), (error: unknown) => error instanceof VideoEncodeError && error.code === "frameBytes");
  assert.throws(() => writeAvcMp4(parameters, encoder.configuration(), []), (error: unknown) => error instanceof VideoEncodeError && error.code === "empty");
  console.log(`raster-video cases=${fixture.cases.length} refusals=${fixture.refusals.length}`);
}
