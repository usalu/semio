//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🎞️Recorder
/** 🎞️ What a scripted tracker answers. */
type Script = { createdIssueUrl?: string; issueState?: string; milestone?: number | null; milestoneTitle?: string; failures?: string[] };

/** 🎞️ The vector file of this case. */
type Vectors = { goal: string; issueUrl: string; title: string; prompt: string; summary: string; labels: string[]; scripts: Record<string, Script> };

const VECTORS = "shared://🐙️issue-sync-transcripts/🐙️transcripts.json";

function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(readFileSync(ctx.fixture(VECTORS), "utf8")) as Vectors;
}

/** 🎞️ A second recorder of the same scripts, written from the port's contract. */
class Recorder {
  readonly calls: string[] = [];

  constructor(private readonly script: Script) {}

  private fails(method: string): boolean {
    return (this.script.failures ?? []).includes(method);
  }

  createIssue(title: string, body: string, milestone: number | null): string {
    this.calls.push(`create_issue|${title}|${body}|${milestone === null ? "" : String(milestone)}`);
    if (this.fails("create_issue")) throw new Error("create_issue failed");
    return this.script.createdIssueUrl ?? "";
  }

  getIssueDetails(issueUrl: string): { state: string } | null {
    this.calls.push(`get_issue_details|${issueUrl}`);
    if (this.fails("get_issue_details")) throw new Error("get_issue_details failed");
    const state = this.script.issueState ?? "";
    return state === "" ? null : { state };
  }

  reopenIssue(issueUrl: string): void {
    this.calls.push(`reopen_issue|${issueUrl}`);
    if (this.fails("reopen_issue")) throw new Error("reopen_issue failed");
  }

  addComment(issueUrl: string, comment: string): void {
    this.calls.push(`add_comment|${issueUrl}|${comment}`);
    if (this.fails("add_comment")) throw new Error("add_comment failed");
  }

  addLabels(issueUrl: string, labels: string[]): void {
    this.calls.push(`add_labels|${issueUrl}|${labels.join(",")}`);
    if (this.fails("add_labels")) throw new Error("add_labels failed");
  }

  closeIssue(issueUrl: string): void {
    this.calls.push(`close_issue|${issueUrl}`);
    if (this.fails("close_issue")) throw new Error("close_issue failed");
  }

  findMilestoneByTitle(title: string): { number: number } | null {
    this.calls.push(`find_milestone_by_title|${title}`);
    if (this.fails("find_milestone_by_title")) throw new Error("find_milestone_by_title failed");
    return this.script.milestone === undefined || this.script.milestone === null ? null : { number: this.script.milestone };
  }
}

function recorder(ctx: AdapterContext, scenario: string): Recorder {
  const script = vectors(ctx).scripts[scenario];
  if (script === undefined) throw new Error(`no script for scenario ${scenario}`);
  return new Recorder(script);
}

/** 🤖️ The prompt heading of an issue body or comment. */
const promptHeading = (body: string): string => (body === "" ? "# 🤖️ Prompt" : `# 🤖️ Prompt\n\n${body}`);

/** 🔍️ The summary heading of a closing comment. */
const summaryHeading = (body: string): string => (body === "" ? "# 🔍️ Summary" : `# 🔍️ Summary\n\n${body}`);

/** 🐙️ The open/reopen synchronisation rule, restated from the port's contract. */
function syncOpen(tracker: Recorder, existingIssue: string, title: string, prompt: string, issue: string, milestone: number | null, reopenIfClosed: boolean): { issue: string; warnings: string[] } {
  const warnings: string[] = [];
  if (issue.trim() !== "") return { issue: issue.trim(), warnings };
  if (existingIssue !== "") {
    if (reopenIfClosed) {
      try {
        const remote = tracker.getIssueDetails(existingIssue);
        if (remote !== null && remote.state.toLowerCase() === "closed") {
          try {
            tracker.reopenIssue(existingIssue);
          } catch (error) {
            warnings.push(`reopen github issue: ${(error as Error).message}`);
          }
        }
      } catch (error) {
        warnings.push(`read github issue: ${(error as Error).message}`);
      }
    }
    return { issue: existingIssue, warnings };
  }
  try {
    const url = tracker.createIssue(title, promptHeading(prompt), milestone);
    if (url.trim() === "") {
      warnings.push("github issue create returned empty url");
      return { issue: "", warnings };
    }
    return { issue: url, warnings };
  } catch (error) {
    warnings.push(`Failed to create GitHub issue: ${(error as Error).message}`);
    return { issue: "", warnings };
  }
}

/** 📪️ The close synchronisation rule, restated from the port's contract. */
function syncClose(tracker: Recorder, issueUrl: string, summary: string, labels: string[], bulk: boolean): string[] {
  const warnings: string[] = [];
  if (issueUrl === "") return warnings;
  if (!bulk) {
    if (labels.length > 0) {
      try {
        tracker.addLabels(issueUrl, labels);
      } catch (error) {
        warnings.push(`Failed to add labels to GitHub issue: ${(error as Error).message}`);
      }
    }
    try {
      tracker.addComment(issueUrl, summaryHeading(summary));
    } catch (error) {
      warnings.push(`Failed to add summary and metrics comment to GitHub issue: ${(error as Error).message}`);
    }
  }
  try {
    tracker.closeIssue(issueUrl);
  } catch (error) {
    warnings.push(`Failed to close GitHub issue: ${(error as Error).message}`);
  }
  return warnings;
}

function milestoneNumber(tracker: Recorder, title: string): number | null {
  try {
    return tracker.findMilestoneByTitle(title)?.number ?? null;
  } catch {
    return null;
  }
}
//#endregion 🎞️Recorder

//#region 🔮️Oracle
/**
 * 🔮️ TypeScript oracle of the issue synchronisation case.
 *
 * It is a second implementation of the same contract, written from the `IssueTracker` port and the
 * documented rule that a management failure is a warning rather than a fatal error — not from the
 * Rust source. It is registered as a `cross-semio-implementation`, which is exactly what it is: a
 * second reader inside this repository, not independent third-party evidence. The accompanying
 * no-oracle decision records why nothing outside the repository can take its place.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "a-new-ticket-creates-one-issue": {
      oracle: (ctx) => {
        const file = vectors(ctx);
        const tracker = recorder(ctx, "a-new-ticket-creates-one-issue");
        const milestone = milestoneNumber(tracker, file.goal);
        const outcome = syncOpen(tracker, "", file.title, file.prompt, "", milestone, false);
        return { projection: { calls: tracker.calls, issue: outcome.issue, warnings: outcome.warnings, milestone: milestone === null ? "" : String(milestone) } };
      },
    },
    "an-existing-open-issue-is-left-alone": {
      oracle: (ctx) => {
        const file = vectors(ctx);
        const tracker = recorder(ctx, "an-existing-open-issue-is-left-alone");
        const outcome = syncOpen(tracker, file.issueUrl, file.title, file.prompt, "", null, true);
        return { projection: { calls: tracker.calls, issue: outcome.issue, warnings: outcome.warnings } };
      },
    },
    "a-closed-issue-is-reopened": {
      oracle: (ctx) => {
        const file = vectors(ctx);
        const tracker = recorder(ctx, "a-closed-issue-is-reopened");
        const outcome = syncOpen(tracker, file.issueUrl, file.title, file.prompt, "", null, true);
        return { projection: { calls: tracker.calls, issue: outcome.issue, warnings: outcome.warnings } };
      },
    },
    "a-close-comments-labels-and-closes": {
      oracle: (ctx) => {
        const file = vectors(ctx);
        const normal = recorder(ctx, "a-close-comments-labels-and-closes");
        const bulk = recorder(ctx, "a-close-comments-labels-and-closes");
        const normalWarnings = syncClose(normal, file.issueUrl, file.summary, file.labels, false);
        const bulkWarnings = syncClose(bulk, file.issueUrl, file.summary, file.labels, true);
        return { projection: { normalCalls: normal.calls, normalWarnings, bulkCalls: bulk.calls, bulkWarnings } };
      },
    },
    "every-failure-becomes-a-warning": {
      oracle: (ctx) => {
        const file = vectors(ctx);
        const creating = recorder(ctx, "every-failure-becomes-a-warning");
        const reopening = recorder(ctx, "every-failure-becomes-a-warning");
        const closing = recorder(ctx, "every-failure-becomes-a-warning");
        const created = syncOpen(creating, "", file.title, file.prompt, "", null, false);
        const reopened = syncOpen(reopening, file.issueUrl, file.title, file.prompt, "", null, true);
        const closeWarnings = syncClose(closing, file.issueUrl, file.summary, file.labels, false);
        return {
          projection: {
            createWarnings: created.warnings,
            createIssue: created.issue,
            reopenWarnings: reopened.warnings,
            reopenCalls: reopening.calls,
            closeWarnings,
            closeCalls: closing.calls,
          },
        };
      },
    },
  },
});
//#endregion 🔮️Oracle
