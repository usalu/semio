import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoRoot = resolve(root, "../../../../../../../../../..");

const wasmEngineStub = resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts");
const testLevel = process.env.SEMIO_TEST_LEVEL ?? "fundamental";
const longInSourceSuites = [
  resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🟦️.tsx"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️.tsx"),
] as const;
const exhaustiveInSourceSuites = [
  ...longInSourceSuites,
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/UiDocumentStore/🟦️.tsx"),
] as const;

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react/test", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️render.ts") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      { find: "@semio-tech/assets", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-surface-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-editor-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/flow-core/🟨️flow-browser.js", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🟨️flow-browser.js") },
      { find: "@semio-tech/flow-core", replacement: wasmEngineStub },
    ],
  },
  test: {
    name: "@semio-tech/framework-renderer-react",
    environment: "jsdom",
    coverage: { include: ["index.tsx"] },
    ...(testLevel === "fundamental" || testLevel === "quick" ? { include: ["🧪️quick.test.ts"] } : {}),
    testNamePattern: testLevel === "fundamental" ? /validates the language-neutral renderer resident capacity with the Node oracle/ : undefined,
    // 🧪️ In-source (`import.meta.vitest`) suites in the `🧑️‍🎨️engine/🧱️elements/` co-location dirs —
    // NOT under this package's own `root`, so the default `include` glob never finds them. Fundamental
    // and quick deliberately select the bounded resident-composition file; long restores the default
    // package corpus plus moderate in-source suites; exhaustive adds the expensive incremental ownership
    // matrices. A file must never appear in both `include` and `includeSource`, which would double-count it.
    includeSource: testLevel === "exhaustive" ? exhaustiveInSourceSuites : testLevel === "long" ? longInSourceSuites : [],
  },
});
