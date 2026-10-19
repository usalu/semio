import type { ScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { resolveTestLevel, testLevelBudgetMs } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { withAdmittedStylingPythonWheelV1 } from "../../🏗️builder/🐍️python/📜️script.ts";
import { resolve } from "node:path";

/** 🏃️ Runs the `StylingPythonGenerateScript` command under the caller's roots and invocation. */
export function runStylingPythonGenerateScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): void {
  console.log("[nx-generate] styling artifacts ready");
}

/** 🏃️ Runs the `StylingPythonTestScript` command under the caller's roots and invocation. */
export async function runStylingPythonTestScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  resolveTestLevel(segments);
  const sourceAssertion = "from importlib import import_module; styling = import_module('🔤️tokens.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']; print('styling Python source import resolved')";
  const packageAssertion = "from importlib import import_module; styling = import_module('🎨️styling.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']; print('styling Python wheel import resolved')";
  const control = scriptCompilerAdmissionControlV1(context.invocation, "styling:python-test", testLevelBudgetMs());
  await withAdmittedStylingPythonWheelV1(context.invocation, control, async view => {
    await captureOwnedCommandV1(view.python, ["-I", "-B", "-c", `import sys; sys.path.insert(0, sys.argv[1]); ${sourceAssertion}`, resolve(view.project, "../..")], view.project, { control: view.control, env: view.environment, output: { maximumBytes: 65536, maximumLines: 128 } });
    await captureOwnedCommandV1(view.python, ["-I", "-B", "-c", `import sys; sys.path.insert(0, sys.argv[1]); ${packageAssertion}`, view.wheel], view.project, { control: view.control, env: view.environment, output: { maximumBytes: 65536, maximumLines: 128 } });
  });
}
