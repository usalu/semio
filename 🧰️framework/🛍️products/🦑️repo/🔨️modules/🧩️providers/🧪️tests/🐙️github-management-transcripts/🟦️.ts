//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🎞️Runner
/** 🎞️ One recorded exchange of a process transcript. */
type Exchange = { argv: string[]; stdout?: string; stderr?: string; status: number };

/** 🎞️ A transcript replayer written independently of the Rust runner, against the same fixture schema. */
class Replayer {
  private readonly pending: Exchange[];
  readonly issued: string[][] = [];

  constructor(exchanges: readonly Exchange[]) {
    this.pending = exchanges.map((exchange) => ({ ...exchange }));
  }

  run(argv: readonly string[]): { stdout: string; stderr: string; status: number } {
    this.issued.push([...argv]);
    const index = this.pending.findIndex((exchange) => exchange.argv.length === argv.length && exchange.argv.every((word, at) => word === argv[at]));
    if (index < 0) return { stdout: "", stderr: `no recorded exchange for ${JSON.stringify(argv)}`, status: 127 };
    const [found] = this.pending.splice(index, 1);
    return { stdout: found!.stdout ?? "", stderr: found!.stderr ?? "", status: found!.status };
  }
}

const TRANSCRIPTS = "local://🎞️gh-transcripts.json";

function replayer(ctx: AdapterContext): Replayer {
  const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(TRANSCRIPTS))) as Record<string, { exchanges: Exchange[] }>;
  const entry = file[ctx.scenario.id];
  if (entry === undefined) throw new Error(`transcript fixture carries no entry for scenario ${ctx.scenario.id}`);
  return new Replayer(entry.exchanges);
}

/** 🔗️ The url extraction rule, restated from the contract rather than copied from the Rust source. */
function extractIssueUrl(output: string): string {
  const trimmed = output.trim();
  if (trimmed === "") return "";
  const first = trimmed.split(/\s+/u)[0] ?? "";
  if (first.startsWith("https://")) return first;
  const at = trimmed.indexOf("https://github.com/");
  if (at < 0) return "";
  const rest = trimmed.slice(at).split(/[\s]/u)[0] ?? "";
  return rest.includes("/issues/") ? rest : "";
}
//#endregion 🎞️Runner

//#region 🧭️Adapter
/** 🟦️ TypeScript side of the GitHub management transcript case: a second reader of the same `gh` output. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "issue-view-is-parsed": {
      oracle: (ctx) => {
        const gh = replayer(ctx);
        const url = "https://github.com/usalu/semio/issues/412";
        const outcome = gh.run(["gh", "issue", "view", url, "--json", "url,state,milestone,labels,title,body"]);
        if (outcome.status !== 0) throw new Error(`gh issue view failed: ${outcome.stderr.trim()}`);
        const raw = JSON.parse(outcome.stdout) as { url?: string; state?: string; title?: string; body?: string; milestone?: { number: number; title: string } | null; labels?: { name: string }[] };
        return {
          projection: {
            issue: {
              url: raw.url ?? "",
              state: raw.state ?? "",
              title: raw.title ?? "",
              body: raw.body ?? "",
              milestone: raw.milestone == null ? null : { number: raw.milestone.number, title: raw.milestone.title },
              labels: (raw.labels ?? []).map((label) => label.name),
            },
            argv: gh.issued,
          },
        };
      },
    },
    "milestone-list-is-scanned-for-a-title": {
      oracle: (ctx) => {
        const gh = replayer(ctx);
        const outcome = gh.run(["gh", "api", "repos/:owner/:repo/milestones", "-X", "GET", "-F", "state=all", "-F", "per_page=100", "--paginate", "--jq", ".[]"]);
        if (outcome.status !== 0) throw new Error(`gh api milestone list failed: ${outcome.stderr.trim()}`);
        let matched: Record<string, unknown> | null = null;
        for (const line of outcome.stdout.split("\n")) {
          if (line.trim() === "") continue;
          let candidate: Record<string, unknown>;
          try {
            candidate = JSON.parse(line) as Record<string, unknown>;
          } catch {
            continue;
          }
          if (candidate.title === "26/09") {
            matched = candidate;
            break;
          }
        }
        if (matched === null) throw new Error("no milestone matched");
        return {
          projection: {
            milestone: {
              number: matched.number as number,
              title: (matched.title as string) ?? "",
              description: (matched.description as string) ?? "",
              url: (matched.url as string) ?? "",
              dueOn: (matched.due_on as string) ?? "",
              state: (matched.state as string) ?? "",
            },
            argv: gh.issued,
          },
        };
      },
    },
    "label-catalog-is-listed": {
      oracle: (ctx) => {
        const gh = replayer(ctx);
        const outcome = gh.run(["gh", "label", "list", "--json", "name", "--limit", "1000"]);
        if (outcome.status !== 0) throw new Error(`gh label list failed: ${outcome.stderr.trim()}`);
        return { projection: { labels: (JSON.parse(outcome.stdout) as { name: string }[]).map((label) => label.name), argv: gh.issued } };
      },
    },
    "create-issue-resolves-the-milestone-title-first": {
      oracle: (ctx) => {
        const gh = replayer(ctx);
        const resolved = gh.run(["gh", "api", "repos/{owner}/{repo}/milestones/17", "--jq", ".title"]);
        const create = ["gh", "issue", "create", "--title", "Split the godfile", "--body", "One package per domain.", "--label", "ticket"];
        if (resolved.status === 0) create.push("--milestone", resolved.stdout.trim());
        const created = gh.run(create);
        if (created.status !== 0) throw new Error(`gh issue create failed: ${created.stderr.trim()}`);
        const url = extractIssueUrl(created.stdout) || extractIssueUrl(created.stderr);
        if (url === "") throw new Error("gh issue create succeeded but no issue url in output");
        gh.run(["gh", "project", "item-add", "2", "--owner", "usalu", "--url", url]);
        const user = gh.run(["gh", "api", "user", "--jq", ".login"]);
        if (user.status === 0 && user.stdout.trim() !== "") gh.run(["gh", "issue", "edit", url, "--add-assignee", user.stdout.trim()]);
        return { projection: { url, argv: gh.issued } };
      },
    },
    "a-failing-call-carries-the-trimmed-stderr": {
      oracle: (ctx) => {
        const gh = replayer(ctx);
        const url = "https://github.com/usalu/semio/issues/999";
        const outcome = gh.run(["gh", "issue", "view", url, "--json", "url,state,milestone,labels,title,body"]);
        const failed = outcome.status !== 0;
        return { projection: { failed, message: failed ? `gh issue view failed: ${outcome.stderr.trim()}` : "", argv: gh.issued } };
      },
    },
  },
});
//#endregion 🧭️Adapter
