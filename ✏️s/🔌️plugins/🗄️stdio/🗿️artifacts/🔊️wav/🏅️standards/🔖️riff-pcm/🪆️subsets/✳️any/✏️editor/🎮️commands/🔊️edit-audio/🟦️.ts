import {binary32} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type { WavData, WavFmt, WavMutation, WavSnapshot } from "../../../🧬️schema/🧬️mutations/🟦️";

export const WAV_AUDIO_EDIT_PAYLOAD_SCHEMA = "s.stdio.wav.command.edit-audio.v1" as const;
export const WAV_AUDIO_PATCH_PAYLOAD_BYTES = 16_384;
export const WAV_AUDIO_MAXIMUM_MUTATIONS = 128;

export type WavAudioEdit =
  | Readonly<{ kind: "setSample"; frame: number; channel: number; revision: string; value: string }>
  | Readonly<{ kind: "appendFrame"; revision: string }>
  | Readonly<{ kind: "insertFrame"; frame: number; revision: string }>
  | Readonly<{ kind: "removeFrame"; frame: number; revision: string }>
  | Readonly<{ kind: "appendChannel"; revision: string }>
  | Readonly<{ kind: "insertChannel"; channel: number; revision: string }>
  | Readonly<{ kind: "removeChannel"; channel: number; revision: string }>
  | Readonly<{ kind: "setSampleRate"; revision: string; value: string }>;

export class WavAudioEditRefusal extends Error {
  constructor(readonly code: string, message: string) {
    super(message);
  }
}

const refuse = (code: string, message: string): never => {
  throw new WavAudioEditRefusal(code, message);
};

const sampleBytes = (data: WavData): number => {
  if (data.kind === "pcm8") return 1;
  if (data.kind === "pcm16") return 2;
  if (data.kind === "float32") return 4;
  return refuse("stdio.wav.raw-samples", "Raw WAV payloads do not have a safe frame or channel interpretation");
};

const shape = (snapshot: WavSnapshot): Readonly<{ channels: number; frames: number; bytes: number }> => {
  const bytes = sampleBytes(snapshot.data);
  const channels = snapshot.fmt.channels;
  if (!Number.isSafeInteger(channels) || channels <= 0) refuse("stdio.wav.zero-channels", "WAV channel count must be positive");
  if (snapshot.data.value.length % channels !== 0) refuse("stdio.wav.partial-frame", "WAV sample data contains a partial frame");
  const format = snapshot.data.kind === "float32" ? 3 : 1;
  const blockAlign = channels * bytes;
  const byteRate = snapshot.fmt.sampleRate * blockAlign;
  if (!Number.isSafeInteger(snapshot.fmt.sampleRate) || snapshot.fmt.sampleRate <= 0 || blockAlign > 0xffff || byteRate > 0xffff_ffff) refuse("stdio.wav.format-data-mismatch", "Typed WAV format arithmetic is not representable");
  if (snapshot.fmt.audioFormat !== format || snapshot.fmt.bitsPerSample !== bytes * 8 || snapshot.fmt.blockAlign !== blockAlign || snapshot.fmt.byteRate !== byteRate) refuse("stdio.wav.format-data-mismatch", "Typed WAV data and format metadata differ");
  return { channels, frames: snapshot.data.value.length / channels, bytes };
};

const empty = (data: WavData): WavData => ({ kind: data.kind, value: [] } as WavData);
const zero = (data: WavData): number => data.kind === "pcm8" ? 128 : 0;
const samples = (data: WavData, value: readonly number[]): WavData => data.kind === "float32" ? { kind: "float32", value: value.map(binary32) } : { kind: data.kind, value };

const sample = (data: WavData, value: string): WavData => {
  const parsed = Number(value);
  if (!Number.isFinite(parsed)) refuse("stdio.wav.sample-range", "Sample must be finite");
  if (data.kind === "pcm8" && (!Number.isInteger(parsed) || parsed < 0 || parsed > 255)) refuse("stdio.wav.sample-range", "PCM8 sample is outside 0..255");
  if (data.kind === "pcm16" && (!Number.isInteger(parsed) || parsed < -32_768 || parsed > 32_767)) refuse("stdio.wav.sample-range", "PCM16 sample is outside i16");
  if (data.kind === "float32" && Math.abs(parsed) > 3.4028234663852886e38) refuse("stdio.wav.sample-range", "Float32 sample is outside the finite range");
  return samples(data, [data.kind === "float32" ? Math.fround(parsed) : parsed]);
};

const format = (source: WavFmt, channels: number, bytes: number, sampleRate: number): WavFmt => {
  if (!Number.isSafeInteger(sampleRate) || sampleRate <= 0 || sampleRate > 0xffff_ffff) refuse("stdio.wav.sample-rate", "Sample rate must be a positive u32");
  const blockAlign = channels * bytes;
  const byteRate = sampleRate * blockAlign;
  if (blockAlign > 0xffff) refuse("stdio.wav.block-align-overflow", "Block alignment exceeds u16");
  if (byteRate > 0xffff_ffff) refuse("stdio.wav.byte-rate-overflow", "Byte rate exceeds u32");
  return { ...source, channels, sampleRate, blockAlign, byteRate };
};

function rewriteValues<T>(values:readonly T[],zero:T,startFrame:number,frameCount:number,oldChannels:number,channel:number,insert:boolean):T[]{
  const output:T[]=[];
  for(let frame=startFrame;frame<startFrame+frameCount;frame++){const start=frame*oldChannels;for(let index=0;index<channel;index++)output.push(values[start+index]!);if(insert)output.push(zero);for(let index=channel+(insert?0:1);index<oldChannels;index++)output.push(values[start+index]!)}
  return output;
}
const rewriteChannels=(data:WavData,startFrame:number,frameCount:number,oldChannels:number,channel:number,insert:boolean):WavData=>data.kind==="float32"?{kind:"float32",value:rewriteValues(data.value,binary32(0),startFrame,frameCount,oldChannels,channel,insert)}:{kind:data.kind,value:rewriteValues(data.value,zero(data),startFrame,frameCount,oldChannels,channel,insert)};

export function assertWavAudioRevision(command: WavAudioEdit, canonicalRevision: string): void {
  if (command.revision !== canonicalRevision) refuse("stdio.wav.audio-conflict", "The WAV document changed before this audio edit was applied");
}

export function wavAudioEditMutations(command: WavAudioEdit, snapshot: WavSnapshot): readonly WavMutation[] {
  const { channels, frames, bytes } = shape(snapshot);
  const maximumSamples = Math.max(1, Math.floor(WAV_AUDIO_PATCH_PAYLOAD_BYTES / bytes));
  const mutations: WavMutation[] = [];
  if (command.kind === "setSample") {
    if (!Number.isSafeInteger(command.frame) || command.frame < 0 || !Number.isSafeInteger(command.channel) || command.channel < 0 || command.frame >= frames || command.channel >= channels) refuse("stdio.wav.sample-stale", "Sample address no longer exists");
    mutations.push({ mutation: "patchData", index: command.frame * channels + command.channel, removeCount: 1, data: sample(snapshot.data, command.value) });
  } else if (command.kind === "appendFrame" || command.kind === "insertFrame" || command.kind === "removeFrame") {
    const frame = command.kind === "appendFrame" ? frames : command.frame;
    if (!Number.isSafeInteger(frame) || frame < 0 || frame > frames || (command.kind === "removeFrame" && frame === frames)) refuse("stdio.wav.frame-stale", "Frame address no longer exists");
    for (let offset = 0; offset < channels; offset += maximumSamples) {
      const count = Math.min(maximumSamples, channels - offset);
      if (command.kind === "removeFrame") mutations.push({ mutation: "patchData", index: frame * channels, removeCount: count, data: empty(snapshot.data) });
      else mutations.push({ mutation: "patchData", index: frame * channels + offset, removeCount: 0, data: samples(snapshot.data, Array.from({ length: count }, () => zero(snapshot.data))) });
    }
  } else if (command.kind === "setSampleRate") {
    mutations.push({ mutation: "setFmt", fmt: format(snapshot.fmt, channels, bytes, Number(command.value)) });
  } else {
    const insert = command.kind === "appendChannel" || command.kind === "insertChannel";
    const channel = command.kind === "appendChannel" ? channels : command.channel;
    if (!Number.isSafeInteger(channel) || channel < 0 || (insert && channel > channels) || (!insert && channel >= channels)) refuse("stdio.wav.channel-stale", "Channel address no longer exists");
    if (!insert && channels === 1) refuse("stdio.wav.last-channel", "A WAV document must retain at least one channel");
    const nextChannels = channels + (insert ? 1 : -1);
    const widest = Math.max(channels, nextChannels);
    if (widest * bytes > WAV_AUDIO_PATCH_PAYLOAD_BYTES) refuse("stdio.wav.frame-too-wide", `One transformed frame exceeds ${WAV_AUDIO_PATCH_PAYLOAD_BYTES} bytes`);
    const framesPerChunk = Math.floor(maximumSamples / widest);
    const chunks = Math.ceil(frames / framesPerChunk);
    for (let ordinal = 0; ordinal < chunks; ordinal += 1) {
      const reverse = chunks - 1 - ordinal;
      const startFrame = reverse * framesPerChunk;
      const localFrames = Math.min(framesPerChunk, frames - startFrame);
      mutations.push({
        mutation: "patchData",
        index: startFrame * channels,
        removeCount: localFrames * channels,
        data: rewriteChannels(snapshot.data, startFrame, localFrames, channels, channel, insert),
      });
    }
    mutations.push({ mutation: "setFmt", fmt: format(snapshot.fmt, nextChannels, bytes, snapshot.fmt.sampleRate) });
  }
  if (mutations.length > WAV_AUDIO_MAXIMUM_MUTATIONS) refuse("stdio.wav.audio-edit-too-large", "Audio edit exceeds the retained mutation ceiling");
  return mutations;
}
