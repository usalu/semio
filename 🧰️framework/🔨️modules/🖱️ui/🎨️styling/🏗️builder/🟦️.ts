import { readdirSync } from "node:fs";
import { delimiter, join, resolve } from "node:path";
import { BundleScript, resolveTestLevel, runTestBudgeted } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const stylingOwnerRoot = resolve(import.meta.dir, "..");
const pythonWheelPath = (root: string): string => {
  const output = join(root, "dist/build"), wheels = readdirSync(output).filter(name => name.endsWith(".whl"));
  if (wheels.length !== 1) throw new Error(`Expected one styling wheel, got ${wheels.length}`);
  return join(output, wheels[0]!);
};
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
    await runTestBudgeted("uv", ["run", "--locked", "--no-sync", "python", "-c", sourceAssertion], { cwd: this.root, env: { ...process.env, PYTHONPATH: [stylingOwnerRoot, process.env.PYTHONPATH ?? ""].filter(Boolean).join(delimiter) } });
    await runTestBudgeted("uv", ["run", "--locked", "--no-sync", "python", "-c", `import sys; sys.path.insert(0, sys.argv[1]); ${packageAssertion}`, pythonWheelPath(this.root)], { cwd: this.root });
  }
}
