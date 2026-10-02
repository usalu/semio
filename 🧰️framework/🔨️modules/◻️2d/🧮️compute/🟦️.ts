import { Blake3Hasher } from "../../🔏️hash/🟦️.ts";

/** 🔑️ Frames the exact UTF-8 identity and input bytes with canonical unsigned 64-bit lengths. */
export function engineKeyPreimage(engineId: string, input: Uint8Array): Uint8Array {
  for(let index=0;index<engineId.length;index++){
    const unit=engineId.charCodeAt(index);
    if(unit>=0xd800&&unit<=0xdbff){const next=engineId.charCodeAt(++index);if(!(next>=0xdc00&&next<=0xdfff))throw Error("engine identity must contain Unicode scalar values");}
    else if(unit>=0xdc00&&unit<=0xdfff)throw Error("engine identity must contain Unicode scalar values");
  }
  const identity = new TextEncoder().encode(engineId), output = new Uint8Array(16 + identity.length + input.length), view = new DataView(output.buffer);
  view.setBigUint64(0, BigInt(identity.length), true);
  output.set(identity, 8);
  view.setBigUint64(8 + identity.length, BigInt(input.length), true);
  output.set(input, 16 + identity.length);
  return output;
}

/** 🔏️ Derives a neutral compute key without restricting or rewriting the caller's identity. */
export function engineKey(engineId: string, input: Uint8Array): Uint8Array {
  const hash = new Blake3Hasher();
  hash.update(engineKeyPreimage(engineId, input));
  return hash.digest();
}
