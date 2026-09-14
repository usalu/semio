//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🗺️Plans
/**
 * 🗺️ A second implementation of plan extraction and plan folding, written from the rules stated in
 * `🥒️.feature` and `🧬️schema/🔣️.json`. It keeps the recorded step as a plain object built key by key
 * in the declared order and omits an empty optional member, which is what the subject's serializer
 * does — the record, not the language, is the contract.
 */
type PlanStep = { name: string; status: string };
type RecordedStep = { id: string; name: string; description?: string; status?: string; ideated?: string; started?: string; completed?: string; abandoned?: string };

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseArgs(toolArgs: string): Record<string, unknown> | null {
  if (toolArgs === "") return null;
  try {
    const parsed: unknown = JSON.parse(toolArgs);
    return isObject(parsed) ? parsed : null;
  } catch {
    return null;
  }
}

/** 📥️ The object the steps are read out of: `tool_input` when the payload has one, else the payload. */
function planSource(input: unknown, toolArgs: string): Record<string, unknown> | null {
  const data = input === null || input === undefined ? parseArgs(toolArgs) : isObject(input) ? input : parseArgs(toolArgs);
  if (data === null) return null;
  if (!("tool_input" in data)) return data;
  const nested = data["tool_input"];
  return isObject(nested) ? nested : null;
}

function entriesOf(source: Record<string, unknown>, key: string, nameKey: string): PlanStep[] {
  const raw = source[key];
  if (!Array.isArray(raw)) return [];
  const steps: PlanStep[] = [];
  for (const entry of raw) {
    if (!isObject(entry)) continue;
    const name = typeof entry[nameKey] === "string" ? (entry[nameKey] as string) : "";
    const status = typeof entry["status"] === "string" ? (entry["status"] as string) : "";
    if (name !== "") steps.push({ name, status });
  }
  return steps;
}

/** 🗺️ VS Code's `todoList` first, then the neutral `steps`. */
function extractPlanSteps(input: unknown, toolArgs: string): PlanStep[] {
  const source = planSource(input, toolArgs);
  if (source === null) return [];
  return [...entriesOf(source, "todoList", "title"), ...entriesOf(source, "steps", "name")];
}

/** 🧾️ Rebuilds a recorded step with the omissions the wire record uses. */
function recorded(step: RecordedStep): RecordedStep {
  const out: RecordedStep = { id: step.id ?? "", name: step.name };
  if ((step.description ?? "") !== "") out.description = step.description;
  if ((step.status ?? "") !== "") out.status = step.status;
  if ((step.ideated ?? "") !== "") out.ideated = step.ideated;
  if ((step.started ?? "") !== "") out.started = step.started;
  if ((step.completed ?? "") !== "") out.completed = step.completed;
  if ((step.abandoned ?? "") !== "") out.abandoned = step.abandoned;
  return out;
}

/** 🔀️ Folds an incoming plan into a recorded one at `second`. */
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
    if (status === "in-progress" || status === "in_progress" || status === "started") {
      if (resolved.started === "") resolved.started = second;
    } else if (status === "completed" || status === "done") {
      if (resolved.started !== "" && resolved.completed === "") resolved.completed = second;
    }
    active.add(name);
    merged.push(recorded(resolved));
  }
  for (const step of existing) {
    if (active.has(step.name) || step.name === "") continue;
    const resolved: RecordedStep = { ...step };
    if ((resolved.abandoned ?? "") !== "" || (resolved.completed ?? "") !== "" || (resolved.started ?? "") !== "") {
      merged.push(recorded(resolved));
      continue;
    }
    if ((resolved.ideated ?? "") === "") resolved.ideated = second;
    resolved.abandoned = second;
    merged.push(recorded(resolved));
  }
  return merged;
}
//#endregion 🗺️Plans

//#region 🧭️Adapter
type Payload = { id: string; input: unknown; toolArgs: string };
type Merge = { id: string; second: string; existing: RecordedStep[]; incoming: PlanStep[] };
type Vectors = { payloads: Payload[]; merges: Merge[] };

function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(readFileSync(ctx.fixture("local://🗺️plans.json"), "utf8")) as Vectors;
}

function sameRecord(left: readonly RecordedStep[], right: readonly RecordedStep[]): boolean {
  return JSON.stringify(left) === JSON.stringify(right);
}

/** 🟦️ The independently written plan extraction and folding oracle. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "every-payload-shape-yields-the-same-steps": {
      oracle: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).payloads.map((entry) => [entry.id, extractPlanSteps(entry.input, entry.toolArgs)])) }),
    },
    "folding-a-plan-keeps-its-history": {
      oracle: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).merges.map((entry) => [entry.id, mergePlanSteps(entry.existing, entry.incoming, entry.second)])) }),
    },
    "folding-the-same-plan-twice-changes-nothing": {
      oracle: (ctx) => {
        const projection: Record<string, unknown> = {};
        for (const entry of vectors(ctx).merges) {
          const first = mergePlanSteps(entry.existing, entry.incoming, entry.second);
          const second = mergePlanSteps(first, entry.incoming, "2099-01-01T00:00:00Z");
          projection[entry.id] = { stable: sameRecord(first, second), second };
        }
        return { projection };
      },
    },
  },
});
//#endregion 🧭️Adapter
