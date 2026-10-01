import { describe, expect, it } from "bun:test";
import type { Mp4Mutation } from "../🟦️.ts";
import type { Mp4Mutation as BinaryMp4Mutation } from "../💾️binary/🟦️.ts";
import type { Mp4Mutation as TextMp4Mutation } from "../📝️text/🟦️.ts";
import {parseMp4Snapshot,type Mp4Snapshot,type Mp4Track} from "../../📸️snapshot/🟦️.ts";

type Equal<Left, Right> = (<Value>() => Value extends Left ? 1 : 2) extends <Value>() => Value extends Right ? 1 : 2 ? true : false;

const facets: readonly [Equal<Mp4Mutation, TextMp4Mutation>, Equal<Mp4Mutation, BinaryMp4Mutation>] = [true, true];
const ftyp = { majorBrand: "isom", minorVersion: 512, compatibleBrands: ["isom", "avc1"] };
const sample = { data: [0, 0, 0, 1, 101], duration: 1_000, ctsOffset: 0, sync: true };
const track: Mp4Track = {
  trackId: 1,
  timescale: 1_000,
  codec: {format:"avc1",sps:[[103,66,0,10]],pps:[[104,206,56,128]],nalLengthSize:4,extension:null,hevc:null},
  width: 16,
  height: 16,
  metadata: {
    creationTime:0n,modificationTime:0n,flags:3,duration:1000n,layer:0,alternateGroup:0,volume:0,
    matrix:[65_536,0,0,0,65_536,0,0,0,1_073_741_824],mediaDuration:1000n,mediaCreationTime:0n,
    mediaModificationTime:0n,language:"und",quality:0,handlerName:"VideoHandler",edits:[],color:null,pixelAspectRatio:null,bitrate:null,
    visual: { dataReferenceIndex: 1, version: 0, revisionLevel: 0, vendor: 0, temporalQuality: 0, spatialQuality: 0, horizontalResolution: 4_718_592, verticalResolution: 4_718_592, frameCount: 1, compressorName: "", depth: 24, colorTableId: -1 },
  },
  chunkSampleCounts: [1],
  samples: [sample],
};
const snapshot: Mp4Snapshot = {
  schema: "stdio.mp4",
  ftyp,
  movie:{creationTime:0n,modificationTime:0n,timescale:1000,duration:1000n,rate:65_536,volume:256,matrix:[65_536,0,0,0,65_536,0,0,0,1_073_741_824],nextTrackId:2,title:null,encoder:null},
  tracks: [track],
};
const operations = [
  { mutation: "setSnapshot", snapshot },
  { mutation: "patchSnapshot", patch: { edits: [{ path: ["tracks", "0", "width"], edit: { operation: "set", value: 32 } }] } },
  { mutation: "setFtyp", ftyp },
  { mutation: "insertTrack", index: 0, track },
  { mutation: "removeTrack", index: 0 },
  { mutation: "setTrackDimensions", trackIndex: 0, width: 32, height: 24 },
  { mutation: "setTrackCodec", trackIndex: 0, codec: track.codec },
  { mutation: "insertSample", trackIndex: 0, index: 0, sample },
  { mutation: "removeSample", trackIndex: 0, index: 0 },
  { mutation: "setSampleSync", trackIndex: 0, index: 0, sync: false },
] satisfies readonly Mp4Mutation[];
const tags = ["setSnapshot", "patchSnapshot", "setFtyp", "insertTrack", "removeTrack", "setTrackDimensions", "setTrackCodec", "insertSample", "removeSample", "setSampleSync"] as const satisfies readonly Mp4Mutation["mutation"][];
const fields: Readonly<Record<Mp4Mutation["mutation"], readonly string[]>> = {
  setSnapshot: ["mutation", "snapshot"], patchSnapshot: ["mutation", "patch"], setFtyp: ["mutation", "ftyp"], insertTrack: ["mutation", "index", "track"], removeTrack: ["mutation", "index"], setTrackDimensions: ["mutation", "trackIndex", "width", "height"], setTrackCodec: ["mutation", "trackIndex", "codec"], insertSample: ["mutation", "trackIndex", "index", "sample"], removeSample: ["mutation", "trackIndex", "index"], setSampleSync: ["mutation", "trackIndex", "index", "sync"],
};

describe("MP4 mutation TypeScript facets", () => {
  it("preserves the native tagged union across aggregate, text, and binary facets", () => {
    expect(facets).toEqual([true, true]);
    expect(operations.map(({ mutation }) => mutation)).toEqual(Array.from(tags));
    for (const operation of operations) expect(Object.keys(operation)).toEqual(Array.from(fields[operation.mutation]));
    const json=JSON.parse(JSON.stringify(operations,(_key,value:unknown)=>typeof value==="bigint"?value.toString():value));
    expect(parseMp4Snapshot(json[0].snapshot)).toEqual(snapshot);
    expect(json.map(({mutation}:{mutation:string})=>mutation)).toEqual(tags);
  });
});
