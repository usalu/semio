/** 🚪️ IO (Html io): typed twin of the `📥️Sniff` region of `🦀️.rs`. */

export function sniffRealBytes(bytes: Uint8Array): boolean {
  return new TextDecoder("utf-8", { fatal: false }).decode(bytes).trimStart().toLowerCase().startsWith("<!doctype html");
}
