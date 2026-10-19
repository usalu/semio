import type { ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { ResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type { StylingPythonProviderV1 } from "@semio-tech/ui-styling/builder/python/provider";
import { readNativeCallerAuthorityV1 } from "../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { runStylingPythonV1, withStylingPythonWheelV1 } from "../🟦️.ts";

/** 🔌️ Implements the framework styling Python port with the repository dependency, diagnostics and product stores. */
export const provider: StylingPythonProviderV1<ScriptInvocation, ResourceLeaseControl> = {
  run: (invocation, phase, operation) => runStylingPythonV1(invocation, phase, { version: 1, operation, mode: readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode }),
  withWheel: withStylingPythonWheelV1,
};
