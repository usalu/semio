import{withStylingPythonWheelV1}from"./🐍️python/🟦️.ts";
import{scriptCompilerAdmissionControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import{captureOwnedCommandV1}from"../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { testLevelBudgetMs, resolveTestLevel } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { resolve } from "node:path";

import { BundleScript } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🎨️ Confirms that the shared styling generator dependency completed. */
export class StylingPythonGenerateScript extends BundleScript {
  run(): void {
    console.log("[nx-generate] styling artifacts ready");
  }
}

/** 🧪️ Imports the canonical Python source and its wheel projection. */
export class StylingPythonTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    resolveTestLevel(segments);
    const sourceAssertion = "from importlib import import_module; styling = import_module('🔤️tokens.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']; print('styling Python source import resolved')";
    const packageAssertion = "from importlib import import_module; styling = import_module('🎨️styling.🐍️'); assert styling.BOARD_LIGHT; assert styling.STYLING_TOKENS['primary']; print('styling Python wheel import resolved')";
    const control=scriptCompilerAdmissionControlV1(this.invocation,"styling:python-test",testLevelBudgetMs());
    await withStylingPythonWheelV1(this.invocation,control,async view=>{
      await captureOwnedCommandV1(view.python,["-I","-B","-c",`import sys; sys.path.insert(0, sys.argv[1]); ${sourceAssertion}`,resolve(view.project,"../..")],view.project,{control:view.control,env:view.environment,output:{maximumBytes:65536,maximumLines:128}});
      await captureOwnedCommandV1(view.python,["-I","-B","-c",`import sys; sys.path.insert(0, sys.argv[1]); ${packageAssertion}`,view.wheel],view.project,{control:view.control,env:view.environment,output:{maximumBytes:65536,maximumLines:128}});
    });
  }
}
