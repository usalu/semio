/** 🧭️ The invocation, roots and re-entry of the test script that its route group functions receive instead of the script instance. */
import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

export type TestRouteContext = {
  readonly invocation: ScriptInvocation;
  readonly repoRoot: string;
  readonly root: string;
  readonly run: (segments: string[]) => Promise<void>;
};
