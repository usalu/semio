//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import { defineTestAdapter, type AdapterContext } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📄️Codec
/** 📄️ One real committed document and where it came from. */
type Document = { source: string; text: string };

/** 🎫️ The ticket shape this oracle reads a document into, written from the schema, not from the Rust source. */
type Ticket = {
  title: string;
  emoji: string;
  status: "open" | "closed";
  description: string;
  summary: string;
  issue: string;
  goal: string;
  parent: string;
  plan: { client: string; id: string; source: string; local: string } | null;
  sessions: string[];
};

const VECTORS = "shared://📄️ticket-document-codec/📄️documents.json";
const SCHEMA = "schema://repo.tickets/TicketDocument";

function vectors(ctx: AdapterContext): { documents: Document[]; refused: string[] } {
  return JSON.parse(readFileSync(ctx.fixture(VECTORS), "utf8")) as { documents: Document[]; refused: string[] };
}

/** 🔮️ Judges one document against `🧬️schema/🔣️.json` with a real draft 2020-12 validator. */
function validator(ctx: AdapterContext): (value: unknown, source: string) => void {
  const schema = JSON.parse(readFileSync(ctx.fixture(SCHEMA), "utf8")) as { readonly $schema?: string; readonly $defs: Readonly<Record<string, unknown>> };
  const ajv = new Ajv2020({ strict: false, allErrors: true });
  const compiled = ajv.compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/TicketDocument" });
  return (value, source) => {
    if (!compiled(value)) throw new Error(`${source} does not satisfy TicketDocument: ${ajv.errorsText(compiled.errors)}`);
  };
}

const text = (members: Record<string, unknown>, name: string): string => (typeof members[name] === "string" ? (members[name] as string) : "");

/** 📖️ Reads a document into a ticket, absorbing the three pre-schema spellings and refusing a missing status. */
function decode(source: string): Ticket {
  let parsed: unknown;
  try {
    parsed = JSON.parse(source);
  } catch (error) {
    throw new Error(`invalid: ${String(error)}`);
  }
  if (parsed === null || typeof parsed !== "object" || Array.isArray(parsed)) throw new Error("invalid: ticket document must be a JSON object");
  const members = parsed as Record<string, unknown>;
  const status = members.status;
  if (status !== "open" && status !== "closed") throw new Error('invalid: ticket status must be explicitly "open" or "closed"');
  const github = typeof members.github === "object" && members.github !== null ? (members.github as Record<string, unknown>) : null;
  const rawPlan = typeof members.plan === "object" && members.plan !== null ? (members.plan as Record<string, unknown>) : null;
  const interactions = Array.isArray(members.interactions) ? (members.interactions as Record<string, unknown>[]) : [];
  const ticket: Ticket = {
    title: text(members, "title"),
    emoji: text(members, "emoji"),
    status,
    description: text(members, "description") === "" ? text(members, "prompt") : text(members, "description"),
    summary: text(members, "summary"),
    issue: github === null ? "" : text(github, "issue"),
    goal: text(members, "goal"),
    parent: text(members, "parent"),
    plan: rawPlan === null ? null : { client: text(rawPlan, "client"), id: text(rawPlan, "id"), source: text(rawPlan, "source"), local: text(rawPlan, "local") },
    sessions: [],
  };
  const append = (value: unknown): void => {
    const trimmed = typeof value === "string" ? value.trim() : "";
    if (trimmed !== "" && !ticket.sessions.includes(trimmed)) ticket.sessions.push(trimmed);
  };
  const sessions = members.sessions;
  if (Array.isArray(sessions)) {
    for (const entry of sessions) append(typeof entry === "string" ? entry : ((entry as Record<string, unknown> | null)?.session ?? ""));
  }
  if (ticket.summary === "") {
    for (let index = interactions.length - 1; index >= 0; index -= 1) {
      const kind = text(interactions[index]!, "kind");
      const stem = kind.trim().endsWith(".ended") ? kind.trim().slice(0, -".ended".length) : kind.trim();
      if (stem === "ticket.close" && text(interactions[index]!, "summary") !== "") {
        ticket.summary = text(interactions[index]!, "summary");
        break;
      }
    }
  }
  const agents = members.agents;
  if (Array.isArray(agents)) for (const entry of agents) append((entry as Record<string, unknown> | null)?.session ?? "");
  return ticket;
}

/** 🔤️ Escapes one string the way Go's `encoding/json` does, `SetEscapeHTML(true)` included. */
function goString(value: string): string {
  let out = '"';
  for (const character of value) {
    const code = character.codePointAt(0) ?? 0;
    if (character === '"') out += '\\"';
    else if (character === "\\") out += "\\\\";
    else if (character === "\n") out += "\\n";
    else if (character === "\r") out += "\\r";
    else if (character === "\t") out += "\\t";
    else if (character === "<" || character === ">" || character === "&" || code === 0x2028 || code === 0x2029) out += `\\u${code.toString(16).padStart(4, "0")}`;
    else if (code < 0x20) out += `\\u${code.toString(16).padStart(4, "0")}`;
    else out += character;
  }
  return `${out}"`;
}

type Member = { name: string; kind: "text"; value: string } | { name: string; kind: "object"; value: { name: string; value: string }[] } | { name: string; kind: "strings"; value: string[] };

/** 🖨️ Renders members the way `json.MarshalIndent(value, "", "  ")` renders a struct. */
function render(members: Member[]): string {
  if (members.length === 0) return "{}";
  const lines = members.map((member) => {
    if (member.kind === "text") return `  ${goString(member.name)}: ${goString(member.value)}`;
    if (member.kind === "object") {
      if (member.value.length === 0) return `  ${goString(member.name)}: {}`;
      const nested = member.value.map((entry) => `    ${goString(entry.name)}: ${goString(entry.value)}`).join(",\n");
      return `  ${goString(member.name)}: {\n${nested}\n  }`;
    }
    if (member.value.length === 0) return `  ${goString(member.name)}: []`;
    const elements = member.value.map((entry) => `    ${goString(entry)}`).join(",\n");
    return `  ${goString(member.name)}: [\n${elements}\n  ]`;
  });
  return `{\n${lines.join(",\n")}\n}`;
}

/** 🖨️ Writes a ticket back, in declaration order, omitting every empty member but title and status. */
function encode(ticket: Ticket): string {
  const members: Member[] = [{ name: "title", kind: "text", value: ticket.title }];
  if (ticket.emoji !== "") members.push({ name: "emoji", kind: "text", value: ticket.emoji });
  members.push({ name: "status", kind: "text", value: ticket.status });
  if (ticket.description !== "") members.push({ name: "description", kind: "text", value: ticket.description });
  if (ticket.summary !== "") members.push({ name: "summary", kind: "text", value: ticket.summary });
  if (ticket.issue !== "") members.push({ name: "github", kind: "object", value: [{ name: "issue", value: ticket.issue }] });
  if (ticket.goal !== "") members.push({ name: "goal", kind: "text", value: ticket.goal });
  if (ticket.plan !== null) {
    const nested = (["client", "id", "source", "local"] as const).filter((name) => ticket.plan![name] !== "").map((name) => ({ name, value: ticket.plan![name] }));
    members.push({ name: "plan", kind: "object", value: nested });
  }
  if (ticket.sessions.length > 0) members.push({ name: "sessions", kind: "strings", value: ticket.sessions });
  return render(members);
}
//#endregion 📄️Codec

//#region 🔮️Oracle
/**
 * 🔮️ TypeScript oracle of the ticket document codec case.
 *
 * It is a reference twice over. `ajv` — not this repository — decides whether each committed document
 * satisfies `🧬️schema/🔣️.json`, so a schema that has drifted from the implementations fails here
 * rather than agreeing with itself. And the decoder and encoder above are a second, independently
 * written reader and writer of the same bytes, built from the schema and from Go's documented
 * `MarshalIndent` and `SetEscapeHTML` behaviour rather than from the Rust source.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "real-documents-decode-to-the-same-ticket": {
      oracle: (ctx) => {
        const judge = validator(ctx);
        const projection: Record<string, unknown> = {};
        for (const document of vectors(ctx).documents) {
          judge(JSON.parse(document.text), document.source);
          const ticket = decode(document.text);
          projection[document.source] = {
            title: ticket.title,
            emoji: ticket.emoji,
            status: ticket.status,
            description: ticket.description,
            summary: ticket.summary,
            issue: ticket.issue,
            goal: ticket.goal,
            parent: ticket.parent,
            plan: ticket.plan === null ? "" : `${ticket.plan.client}|${ticket.plan.id}|${ticket.plan.source}|${ticket.plan.local}`,
            sessions: ticket.sessions,
          };
        }
        return { projection };
      },
    },
    "encoding-is-go-marshal-indent": {
      oracle: (ctx) => {
        const projection: Record<string, unknown> = {};
        for (const document of vectors(ctx).documents) projection[document.source] = encode(decode(document.text));
        return { projection };
      },
    },
    "unknown-members-are-dropped": {
      oracle: (ctx) => {
        const projection: Record<string, unknown> = {};
        for (const document of vectors(ctx).documents) {
          const first = encode(decode(document.text));
          const second = encode(decode(first));
          const before = Object.keys(JSON.parse(document.text) as Record<string, unknown>);
          const after = Object.keys(JSON.parse(first) as Record<string, unknown>);
          projection[document.source] = { stable: first === second ? "yes" : "no", lost: before.filter((name) => !after.includes(name)) };
        }
        return { projection };
      },
    },
    "a-document-without-a-status-is-refused": {
      oracle: (ctx) => {
        const projection: Record<string, unknown> = {};
        for (const document of vectors(ctx).refused) {
          try {
            decode(document);
            projection[document] = "accepted";
          } catch (error) {
            projection[document] = `refused:${String((error as Error).message).split(":")[0]}`;
          }
        }
        return { projection };
      },
    },
  },
});
//#endregion 🔮️Oracle
