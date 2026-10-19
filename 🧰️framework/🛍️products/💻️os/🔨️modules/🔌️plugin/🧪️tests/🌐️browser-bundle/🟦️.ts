import { createBrowserCodegenTests } from "./🧬️browser-codegen-tests/🟦️.ts";
import { createBrowserActorBundleTests } from "./🛡️actor-bundle-tests/🟦️.ts";
import { createBrowserHostActivationTests } from "./🪢️host-activation-tests/🟦️.ts";
import { createClosedBrowserComponentFactoryTest } from "./🏭️component-factory-test/🟦️.ts";

/** 🌐️ Composes the browser bundle tests: codegen, closed actor bundle, host activation and the closed component factory over one dependency bag. */
export function createBrowserBundleTests(dependencies: import("../../🌐️browser-bundle/📜️script.ts").BrowserBundleTestDependencies, source: { directory: string; url: string }) {
  const { testBrowserCodegenCapsule, testBrowserCodegenSources, testBrowserCodegenPolicy, testAsyncTaskReturnLift, testBrowserActorCodegenManifest } = createBrowserCodegenTests(dependencies, source);
  const { testClosedBrowserActorBundle } = createBrowserActorBundleTests(dependencies, source);
  const { testBrowserHostActivation } = createBrowserHostActivationTests(dependencies, source);
  const { testClosedBrowserComponentFactory } = createClosedBrowserComponentFactoryTest(dependencies, source, { testBrowserCodegenCapsule, testBrowserCodegenSources, testBrowserCodegenPolicy, testAsyncTaskReturnLift, testBrowserActorCodegenManifest }, { testBrowserHostActivation }, { testClosedBrowserActorBundle });
  return { testClosedBrowserComponentFactory, testBrowserCodegenCapsule, testBrowserCodegenSources, testBrowserCodegenPolicy, testBrowserActorCodegenManifest, testClosedBrowserActorBundle, testBrowserHostActivation };
}
