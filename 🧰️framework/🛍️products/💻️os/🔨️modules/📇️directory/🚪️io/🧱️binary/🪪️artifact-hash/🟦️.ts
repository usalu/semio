import type { ArtifactHash } from "../../../🧬️schema/🟦️.ts";

/** 🔤️ Admits nonzero canonical lowercase SHA256 text as a typed semantic hash. */
export function parseArtifactHashHex(value: string): ArtifactHash | undefined {
  if (!/^[0-9a-f]{64}$/.test(value) || /^0{64}$/.test(value)) return undefined;
  return Array.from({ length: 32 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
}

/** 🔡️ Emits a typed semantic hash into native hexadecimal text. */
export function artifactHashHex(value: ArtifactHash): string {
  return value.map(byte => byte.toString(16).padStart(2, "0")).join("");
}
