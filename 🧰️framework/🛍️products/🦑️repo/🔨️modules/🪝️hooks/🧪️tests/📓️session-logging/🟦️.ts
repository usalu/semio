//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { basename, extname } from "node:path";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region ⚙️Configuration
/**
 * ⚙️ A second implementation of the `[logging]` parse and of the session record, written from the
 * rules stated in `🥒️.feature` and `🧬️schema/🔣️.json`. The store is a plain map here, matching the
 * memory store the subject records into, so neither side reaches a disk or a clock.
 */
type Logging = { session: boolean; operations: boolean; plan: boolean; detail: string };

function defaults(): Logging {
  return { session: false, operations: true, plan: true, detail: "standard" };
}

function affirmative(value: string): boolean {
  return ["true", "yes", "1", "on"].includes(value.trim().toLowerCase());
}

function unquote(value: string): string {
  const trimmed = value.trim();
  if (trimmed.length >= 2 && ((trimmed.startsWith('"') && trimmed.endsWith('"')) || (trimmed.startsWith("'") && trimmed.endsWith("'")))) return trimmed.slice(1, -1);
  return trimmed;
}

function parseLogging(document: string): Logging {
  const config = defaults();
  let section = "";
  for (const line of document.split("\n")) {
    const trimmed = line.trim();
    if (trimmed === "" || trimmed.startsWith("#")) continue;
    if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
      section = trimmed.replace(/^\[+|\]+$/gu, "").toLowerCase();
      continue;
    }
    const at = trimmed.indexOf("=");
    if (at < 0 || section !== "logging") continue;
    const key = trimmed.slice(0, at).trim().toLowerCase();
    const value = unquote(trimmed.slice(at + 1));
    if (key === "session") config.session = affirmative(value);
    else if (key === "operations") config.operations = affirmative(value);
    else if (key === "plan") config.plan = affirmative(value);
    else if (key === "detail" && value !== "") config.detail = value;
  }
  return config;
}

const includeResponse = (logging: Logging): boolean => logging.detail.trim().toLowerCase() !== "minimal";
const includeNative = (logging: Logging): boolean => logging.detail.trim().toLowerCase() === "full";
//#endregion ⚙️Configuration

//#region 🧭️Normalisation
function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** 🪆️ The first non-blank string under any of `keys`, this level first, then every child. */
function nested(value: unknown, keys: readonly string[]): string {
  if (Array.isArray(value)) {
    for (const item of value) {
      const found = nested(item, keys);
      if (found !== "") return found;
    }
    return "";
  }
  if (!isObject(value)) return "";
  for (const key of keys) {
    const candidate = value[key];
    if (typeof candidate === "string" && candidate.trim() !== "") return candidate.trim();
  }
  for (const child of Object.values(value)) {
    const found = nested(child, keys);
    if (found !== "") return found;
  }
  return "";
}

function topLevel(value: unknown, keys: readonly string[]): string {
  if (!isObject(value)) return "";
  for (const key of keys) {
    const candidate = value[key];
    if (typeof candidate === "string" && candidate !== "") return candidate.trim();
  }
  return "";
}

const transcriptOf = (input: unknown): string => nested(input, ["transcript", "transcript_path", "transcriptPath", "log_path", "logPath"]);

function sessionOf(input: unknown): string {
  const named = nested(input, ["session_id", "sessionId", "trajectory_id", "trajectoryId", "conversation_id", "conversationId", "agent_id", "agentId"]);
  if (named !== "") return named;
  const transcript = transcriptOf(input);
  if (transcript === "") return "";
  const base = basename(transcript.trim());
  const extension = extname(base);
  return (extension === "" ? base : base.slice(0, base.length - extension.length)).trim();
}

const VERSION_EVENTS = new Set(["version.checkpoint.starting", "version.checkpoint.ended", "version.checkin.starting", "version.checkin.ended", "version.checkout.starting", "version.checkout.ended"]);

const BLOCKED_GIT_VERBS = ["add", "branch", "checkout", "cherry-pick", "clone", "commit", "config", "fetch", "init", "merge", "mv", "pull", "push", "rebase", "remote", "reset", "restore", "revert", "rm", "stash", "switch", "tag"];

const CONCURRENCY_NOTE = "; other developers and agents may be editing the same files concurrently";

/** 🧰️ The `tool_input` object, flat or nested one or two levels down. */
function toolInputOf(input: unknown): Record<string, unknown> | null {
  if (!isObject(input)) return null;
  if (isObject(input["tool_input"])) return input["tool_input"];
  const event = input["event"];
  if (isObject(event) && isObject(event["tool_input"])) return event["tool_input"];
  const native = input["native"];
  if (isObject(native) && isObject(native["event"]) && isObject((native["event"] as Record<string, unknown>)["tool_input"])) return (native["event"] as Record<string, unknown>)["tool_input"] as Record<string, unknown>;
  return null;
}

/**
 * 🛡️ The refusal reason for the one blocking shape this case exercises: a terminal start whose
 * command is a state-changing git verb. The full policy has its own case; re-deriving all of it here
 * would test the policy twice and the session record not at all.
 */
function refusal(event: string, input: unknown): string | null {
  if (event !== "agent.tool.terminal.starting" && event !== "agent.tool.starting") return null;
  const toolInput = toolInputOf(input);
  const command = typeof toolInput?.["command"] === "string" ? (toolInput["command"] as string) : nested(input, ["command", "command_line", "commandLine"]);
  const match = /^\s*git\s+([a-z-]+)\b/iu.exec(command);
  if (match === null) return null;
  const verb = (match[1] as string).toLowerCase();
  if (!BLOCKED_GIT_VERBS.includes(verb)) return null;
  return `blocked: git ${verb}${CONCURRENCY_NOTE}`;
}
//#endregion 🧭️Normalisation

//#region 📓️Recording
type PlanStep = { name: string; status: string };
type RecordedStep = { id: string; name: string; description?: string; status?: string; ideated?: string; started?: string; completed?: string; abandoned?: string };
type Entry = { event: Record<string, unknown>; native?: { event: unknown }; response?: Record<string, unknown> };
type Meta = { id: string; uri?: string; client?: string; second?: string; checkpoint?: string; contributor?: string; transcript?: string; events?: Entry[]; plan?: { steps?: RecordedStep[] } };

function planStepsOf(input: unknown): PlanStep[] {
  const source = toolInputOf(input) ?? (isObject(input) && !("tool_input" in input) ? input : null);
  if (source === null) return [];
  const steps: PlanStep[] = [];
  for (const [key, nameKey] of [["todoList", "title"], ["steps", "name"]] as const) {
    const raw = source[key];
    if (!Array.isArray(raw)) continue;
    for (const item of raw) {
      if (!isObject(item)) continue;
      const name = typeof item[nameKey] === "string" ? (item[nameKey] as string) : "";
      const status = typeof item["status"] === "string" ? (item["status"] as string) : "";
      if (name !== "") steps.push({ name, status });
    }
  }
  return steps;
}

function trimmedStep(step: RecordedStep): RecordedStep {
  const out: RecordedStep = { id: step.id, name: step.name };
  for (const key of ["description", "status", "ideated", "started", "completed", "abandoned"] as const) if ((step[key] ?? "") !== "") out[key] = step[key];
  return out;
}

function mergePlanSteps(existing: readonly RecordedStep[], incoming: readonly PlanStep[], second: string): RecordedStep[] {
  const byName = new Map(existing.map((step) => [step.name, step]));
  const active = new Set<string>();
  const merged: RecordedStep[] = [];
  for (const step of incoming) {
    const name = step.name.trim();
    if (name === "") continue;
    const previous = byName.get(name);
    const resolved: RecordedStep = { id: previous?.id ?? "", name, description: previous?.description ?? "", status: step.status, ideated: previous?.ideated ?? "", started: previous?.started ?? "", completed: previous?.completed ?? "", abandoned: previous?.abandoned ?? "" };
    if (resolved.ideated === "") resolved.ideated = second;
    const status = step.status.trim().toLowerCase();
    if (["in-progress", "in_progress", "started"].includes(status)) {
      if (resolved.started === "") resolved.started = second;
    } else if (["completed", "done"].includes(status)) {
      if (resolved.started !== "" && resolved.completed === "") resolved.completed = second;
    }
    active.add(name);
    merged.push(trimmedStep(resolved));
  }
  for (const step of existing) {
    if (active.has(step.name) || step.name === "") continue;
    const resolved: RecordedStep = { ...step };
    if ((resolved.abandoned ?? "") !== "" || (resolved.completed ?? "") !== "" || (resolved.started ?? "") !== "") {
      merged.push(trimmedStep(resolved));
      continue;
    }
    if ((resolved.ideated ?? "") === "") resolved.ideated = second;
    resolved.abandoned = second;
    merged.push(trimmedStep(resolved));
  }
  return merged;
}

type Invocation = { event: string; second: string; toolArgs: string; input: unknown };
type Session = { id: string; client: string; kiroParentPid?: number; invocations: Invocation[] };

/** ▶️ Replays one session and answers the record that was written, or `null` when none was. */
function replay(session: Session, logging: Logging): Meta | null {
  const store = new Map<string, Meta>();
  let last: Meta | null = null;
  for (const invocation of session.invocations) {
    if (VERSION_EVENTS.has(invocation.event) || !logging.session) continue;
    const input = invocation.input;
    const named = sessionOf(input);
    const sessionId = named !== "" ? named : session.client === "kiro-cli" && session.kiroParentPid !== undefined ? `kiro-${session.kiroParentPid}` : "unknown";
    const second = [invocation.second, nested(input, ["second"])].map((value) => value.trim()).find((value) => value !== "") ?? "";
    const checkpoint = topLevel(input, ["sha", "checkpoint_sha", "checkpointSha", "hash"]);
    const transcript = transcriptOf(input);
    const meta: Meta = store.get(sessionId) ?? { id: "" };
    if (meta.id === "") {
      meta.id = sessionId;
      meta.uri = `repo://session/${sessionId}`;
      meta.client = session.client;
      meta.second = second;
      meta.checkpoint = checkpoint;
      meta.contributor = "unknown";
      meta.transcript = transcript;
    } else {
      if ((meta.client ?? "") === "") meta.client = session.client;
      if ((meta.second ?? "") === "") meta.second = second;
      if ((meta.checkpoint ?? "") === "") meta.checkpoint = checkpoint;
      if ((meta.transcript ?? "") === "") meta.transcript = transcript;
    }
    const neutral: Record<string, unknown> = { kind: invocation.event, client: session.client, session: sessionId, second, checkpoint, contributor: "unknown" };
    if (transcript !== "") neutral["transcript"] = transcript;
    const entry: Entry = { event: neutral };
    if (includeNative(logging) && input !== null && input !== undefined) entry.native = { event: input };
    if (includeResponse(logging)) {
      const reason = refusal(invocation.event, input);
      entry.response = reason === null ? {} : { blocked: true, message: reason, reason };
    }
    meta.events = [...(meta.events ?? []), entry];
    if (logging.plan && invocation.event === "agent.tool.plan.updating.ended") {
      const steps = planStepsOf(input);
      if (steps.length > 0) meta.plan = { steps: mergePlanSteps(meta.plan?.steps ?? [], steps, second) };
    }
    store.set(sessionId, meta);
    last = meta;
  }
  return last;
}
//#endregion 📓️Recording

//#region 🧭️Adapter
type Vectors = { configs: { id: string; document: string }[]; sessions: Session[] };

function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(readFileSync(ctx.fixture("local://📓️sessions.json"), "utf8")) as Vectors;
}

function sessionById(all: Vectors, id: string): Session {
  const found = all.sessions.find((session) => session.id === id);
  if (found === undefined) throw new Error(`the fixture has no session ${id}`);
  return found;
}

const on = (detail: string, plan: boolean): Logging => ({ session: true, operations: true, plan, detail });

/** 🟦️ The independently written session log oracle. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "the-configuration-decides-whether-anything-is-recorded": {
      oracle: (ctx) => {
        const all = vectors(ctx);
        const session = sessionById(all, "an-agent-session-that-plans-and-is-refused");
        const projection: Record<string, unknown> = {};
        for (const config of all.configs) {
          const logging = parseLogging(config.document);
          const recorded = replay(session, logging);
          projection[config.id] = { session: logging.session, operations: logging.operations, plan: logging.plan, detail: logging.detail, includeResponse: includeResponse(logging), includeNative: includeNative(logging), recordedEntries: recorded?.events?.length ?? 0 };
        }
        return { projection };
      },
    },
    "detail-decides-how-much-of-each-entry-survives": {
      oracle: (ctx) => {
        const all = vectors(ctx);
        const session = sessionById(all, "an-agent-session-that-plans-and-is-refused");
        const refused = session.invocations.find((invocation) => invocation.event === "agent.tool.terminal.starting");
        if (refused === undefined) throw new Error("the fixture has no refused invocation");
        const projection: Record<string, unknown> = {};
        for (const detail of ["minimal", "standard", "full"]) {
          const recorded = replay({ id: session.id, client: session.client, invocations: [refused] }, on(detail, true));
          const entry = recorded?.events?.[0];
          if (entry === undefined) throw new Error("session logging was on but nothing was recorded");
          projection[detail] = { carriesNative: entry.native !== undefined, carriesResponse: entry.response !== undefined, blocked: entry.response?.["blocked"] === true, entry };
        }
        return { projection };
      },
    },
    "a-session-is-recorded-under-a-resolvable-identity": {
      oracle: (ctx) => {
        const projection: Record<string, unknown> = {};
        for (const session of vectors(ctx).sessions) {
          const recorded = replay(session, on("standard", true));
          projection[session.id] = recorded === null ? { id: "", uri: "", client: "", second: "", transcript: "", contributor: "", entries: 0 } : { id: recorded.id, uri: recorded.uri ?? "", client: recorded.client ?? "", second: recorded.second ?? "", transcript: recorded.transcript ?? "", contributor: recorded.contributor ?? "", entries: recorded.events?.length ?? 0 };
        }
        return { projection };
      },
    },
    "a-version-hook-is-never-recorded": {
      oracle: (ctx) => ({ projection: { recordedNothing: replay(sessionById(vectors(ctx), "a-version-hook-is-never-recorded"), on("full", true)) === null } }),
    },
    "the-recorded-plan-follows-the-plan-switch": {
      oracle: (ctx) => {
        const session = sessionById(vectors(ctx), "an-agent-session-that-plans-and-is-refused");
        const withPlan = replay(session, on("standard", true));
        const withoutPlan = replay(session, on("standard", false));
        if (withPlan === null || withoutPlan === null) throw new Error("nothing was recorded");
        return { projection: { withPlan: withPlan.plan ?? null, withoutPlanIsAbsent: withoutPlan.plan === undefined } };
      },
    },
  },
});
//#endregion 🧭️Adapter
