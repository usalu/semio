import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Buffer } from "node:buffer";

/** ⛽️ Checks portable row contributor wire words through independent system Buffer primitives. */
export function testNumericalRestoreFundingWire(): void {
  const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/⛽️restore-funding/🔣️.json"), "utf8"));
  const bytes = Buffer.alloc(fixture.count * fixture.scalarWidth);
  fixture.columns.forEach((column: number, index: number) => {
    bytes.writeUInt32LE(column, index * fixture.scalarWidth);
    bytes.writeBigUInt64LE(index === 0 ? 0xffffffffffffffffn : BigInt(fixture.next[index]), index * fixture.scalarWidth + 4);
  });
  fixture.columns.forEach((column: number, index: number) => {
    if (bytes.readUInt32LE(index * fixture.scalarWidth) !== column || bytes.readBigUInt64LE(index * fixture.scalarWidth + 4) !== (index === 0 ? 0xffffffffffffffffn : BigInt(fixture.next[index]))) throw new Error("row contributor wire reconstruction differs");
  });
  console.log(`[DEBUG] Numerical restore Buffer oracle: ${fixture.count} contributors, ${bytes.length} wire bytes`);
}
