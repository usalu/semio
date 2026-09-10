import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { configDefaults, defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../../../../../../../../..");

const wasmEngineStub = resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts");
const testLevel = process.env.SEMIO_TEST_LEVEL ?? "fundamental";
const includeBackboneWorker = process.env.SEMIO_INCLUDE_BACKBONE_WORKER === "1";
const includeAgentBridge = process.env.SEMIO_INCLUDE_AGENT_BRIDGE === "1";
const backboneWorkerSuite = resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts");
const engineSuite = (name: string, extension = "ts") => resolve(repoRoot, `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/${name}/🟦️.${extension}`);
const engineTestSuites = [
  engineSuite("⚡️quick"),
  engineSuite("🎮️browser-interactive-job-port"),
  engineSuite("🏛️space-administration", "tsx"),
  engineSuite("👥️scoped-presence", "tsx"),
  engineSuite("📇️directory-home-bootstrap", "tsx"),
  engineSuite("📇️session-authority-notice", "tsx"),
  engineSuite("📡️actor-backbone"),
  engineSuite("📨️browser-frame-transport"),
  engineSuite("📥️wgpu-intake-budget"),
  engineSuite("🔬️artifact-creation-ready-opening"),
  engineSuite("🔬️document-opening"),
  engineSuite("🔬️engine-contract"),
  engineSuite("🚪️opening"),
  engineSuite("🧩️package-integration"),
  engineSuite("🧯️router-plugin-faults"),
  engineSuite("🩺️window-fault"),
] as const;
const playwrightEngineTestSuites = [engineSuite("📚️storybook-hosts-no-wasm"), engineSuite("📚️storybook-hosts-wasm")] as const;
const rootPolicySelfTestSuites = ["interactivity-live-reconcile", "interactivity-mounted-engine-surface-lifetime", "interactivity-mounted-frame-transaction"].map((id) => resolve(repoRoot, `./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️${id}/🟦️.ts`));
const quickTestSuite = resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts");
const agentBridgeTestSuite = resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts");
const longInSourceSuites = [
  resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/📥️intake/🟦️.ts"),
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🟦️.ts"),
] as const;
const exhaustiveInSourceSuites = [
  ...longInSourceSuites,
  resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx"),
] as const;

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react/test", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🖌️render.ts") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      { find: "@semio-tech/assets", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript") },
      { find: "@semio-tech/framework-os", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/framework-surface-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework-editor-rs", replacement: wasmEngineStub },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "./🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/infinite-canvas-react-renderer", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/infinite-world-r3f", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/flow-core/🌐️flow-browser.js", replacement: resolve(repoRoot, "./🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🌐️flow-browser.js") },
      { find: "@semio-tech/flow-core", replacement: wasmEngineStub },
    ],
  },
  test: {
    name: "@semio-tech/framework-renderer-react",
    environment: "jsdom",
    coverage: { include: ["index.tsx"] },
    exclude: [...configDefaults.exclude, ...playwrightEngineTestSuites, ...rootPolicySelfTestSuites],
    include: includeAgentBridge ? [agentBridgeTestSuite] : testLevel === "fundamental" || testLevel === "quick" ? [quickTestSuite] : [...engineTestSuites],
    testNamePattern: testLevel === "fundamental" ? /validates the language-neutral renderer resident capacity with the Node oracle/ : undefined,
    // 🧪️ In-source (`import.meta.vitest`) suites in the `🧑‍🎨engine/🧱️elements/` co-location dirs —
    // NOT under this package's own `root`, so the default `include` glob never finds them. Fundamental
    // and quick deliberately select the bounded resident-composition file; long restores the default
    // package corpus plus moderate in-source suites; exhaustive adds the expensive incremental ownership
    // matrices. A file must never appear in both `include` and `includeSource`, which would double-count it.
    includeSource: includeAgentBridge ? [] : includeBackboneWorker ? [backboneWorkerSuite] : testLevel === "exhaustive" ? [...exhaustiveInSourceSuites] : testLevel === "long" ? [...longInSourceSuites] : [],
  },
});
