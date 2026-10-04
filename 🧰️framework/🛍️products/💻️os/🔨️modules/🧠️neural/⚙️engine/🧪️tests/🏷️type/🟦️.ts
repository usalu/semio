/** 🧠️ Proves Neural consumes the canonical lower type through its private domain binding. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");

test("Neural binds the lower type without publishing a product forwarding API", () => {
  const neural = readFileSync(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs"), "utf8");
  expect(neural).not.toContain("pub enum ValueType");
  expect(neural).not.toMatch(/pub use[^;]*ValueType/u);
});
