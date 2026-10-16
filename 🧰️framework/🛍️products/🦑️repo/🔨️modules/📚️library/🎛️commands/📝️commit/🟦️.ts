/** 🧭️ `micro-commit` and `commit` commands.
 * @see 📜️script.ts
 */
import { runCommit, runMicroCommit } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

//#region 🔖️MicroCommitScript
/** 🎆️Writes micro-commit templates from the index that is already staged. */
export class MicroCommitScript extends Script {
  run(segments: string[]): void {
    runMicroCommit(this.root, segments);
  }
}
//#endregion 🔖️MicroCommitScript

//#region 🔖️CommitScript
/** 🔀️Bundle micro-commits into a signed squash commit with per-bundle summaries. */
export class CommitScript extends Script {
  run(segments: string[]): void {
    runCommit(this.root, segments);
  }
}
//#endregion 🔖️CommitScript
