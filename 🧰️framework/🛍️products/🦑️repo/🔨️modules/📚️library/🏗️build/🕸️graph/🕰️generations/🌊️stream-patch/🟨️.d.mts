import type{FactCodecOpenIndexV1}from"../🧮️fact-codec/🟨️.mjs";
import type{GenerationRowSourceV1}from"../🟨️.mjs";
/** 🗒️ The empty fact index as an opened view. */
export function emptyFactIndexViewV1():FactCodecOpenIndexV1;
/** 🌊️ The binary document of `view` patched by `drop`, `upserts` and `append` (see the module): the result of `encodeFactIndexV1` over the logical patched index. */
export function patchFactIndexBytesV1(view:FactCodecOpenIndexV1,change:{drop?:readonly string[];upserts?:ReadonlyMap<string,GenerationRowSourceV1>;append?:ReadonlyMap<string,GenerationRowSourceV1>},options?:{maximumBytes?:number;signal?:AbortSignal}):Promise<Buffer>;
