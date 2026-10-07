/** 🧬️ Checks current Pack and OS module ownership for the JSON primitive. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { rustTokens } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
const root = resolve(import.meta.dir, "../../../../../../..");
const read = (path: string): string => readFileSync(resolve(root, path), "utf8");

test("Pack and OS no longer mount or forward the actual JSON primitive owner", () => {
  for (const path of ["🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs", "🧰️framework/🔨️modules/🎒️pack/🦀️.rs", "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"]) {
    const tokens = rustTokens(read(path)).map(token => token.text);
    expect(tokens.join(" "), path).not.toContain("pub mod json");
    expect(tokens.join(" "), path).not.toContain("pub use pack :: json");
    expect(tokens.join(" "), path).not.toContain("pub use crate :: json");
  }
});
