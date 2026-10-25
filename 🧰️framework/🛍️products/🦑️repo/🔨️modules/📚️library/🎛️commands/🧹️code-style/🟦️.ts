/** 🧹️ `lint` and `format` commands.
 * @see 📜️script.ts
 */
import { runCmd } from "../../🏃️process/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

//#region 🔖️LintScript
export class LintScript extends Script {
  run(segments: string[]): void {
    if (segments[0] === "repo") {
      console.log("[lint] Completed the repository lint graph.");
      return;
    }
    runCmd("repo", ["run", "@semio-tech/repo-lib:lint-dependency-direction"], { cwd: this.root });
  }
}
//#endregion 🔖️LintScript



//#region 🔖️FormatScript
export class FormatScript extends Script {
  run(_segments: string[]): void {
    runCmd(process.execPath, ["x", "prettier", "-w", "."], { cwd: this.root });
  }
}
//#endregion 🔖️FormatScript
