/** 🧬️ Subject adapter of the schema-conformance case: `quizIssues` and `catalogIssues` of `@semio-tech/quiz` accept or reject every quiz document.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/✅️validation/🟦️.ts
 */
import { readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, sep } from "node:path";
import { type AdapterContext, defineTestAdapter } from "../../../../\uD83D\uDD28\uFE0Fmodules/\uD83E\uDDEA\uFE0Ftest/\uD83D\uDD0C\uFE0Fadapter/\uD83D\uDFE6\uFE0F.ts";
import { type Quiz, catalogIssues, quizIssues } from "../../📦️packages/🟦️typescript/🟦️.ts";

const REJECTED = "shared://🧬️schema-conformance/🔣️.json";
const TEACHING = "🎓️teaching";
const SKIPPED = new Set(["node_modules", "target", "dist", "📤️dist", ".git", "🗑️generated", "__pycache__"]);
const QUIZ_DIRECTORY = "❓️quiz";
const DOCUMENT = "🔣️.json";

/** 📋️ The feature's table of `(id, fixture, pointer, definition, quizzes)` rows. */
function rows(ctx: AdapterContext): Record<string, string>[] {
  const [header, ...body] = ctx.scenario.steps.find((step) => step.dataTable !== undefined)!.dataTable!;
  return body.map((row) => Object.fromEntries(header!.map((name, index) => [name, row[index]!])));
}

/** 📍️ Every value a JSON pointer with `*` wildcards reaches, with its concrete pointer. */
function matches(document: unknown, pointer: string): [string, unknown][] {
  let found: [string, unknown][] = [["", document]];
  for (const part of pointer.split("/").filter((segment) => segment !== "")) {
    found = found.flatMap(([path, value]): [string, unknown][] => {
      if (part === "*") return Array.isArray(value) ? value.map((child, index) => [`${path}/${index}`, child]) : value !== null && typeof value === "object" ? Object.entries(value).map(([key, child]) => [`${path}/${key}`, child]) : [];
      if (Array.isArray(value)) return /^\d+$/.test(part) && Number(part) < value.length ? [[`${path}/${part}`, value[Number(part)]]] : [];
      return value !== null && typeof value === "object" && Object.hasOwn(value, part) ? [[`${path}/${part}`, (value as Record<string, unknown>)[part]]] : [];
    });
  }
  return found;
}

/** ⚖️ Whether the owned validator accepts a quiz or a catalog with its quizzes. */
function accepted(definition: string, document: unknown, quizzes: readonly Quiz[]): boolean {
  return (definition === "Catalog" ? catalogIssues(document, quizzes) : quizIssues(document)).length === 0;
}

/** 🎓️ Every `❓️quiz/🔣️.json` under the teaching area, repo-relative. */
function teachingDocuments(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (directory: string): void => {
    const entries = readdirSync(directory, { withFileTypes: true });
    if (directory.endsWith(`${sep}${QUIZ_DIRECTORY}`) && entries.some((entry) => entry.isFile() && entry.name === DOCUMENT)) found.push(relative(repoRoot, join(directory, DOCUMENT)).split(sep).join("/"));
    for (const entry of entries) if (entry.isDirectory() && !SKIPPED.has(entry.name)) walk(join(directory, entry.name));
  };
  walk(join(repoRoot, TEACHING));
  return found.sort();
}

/** 📄️ A JSON document from disk. */
function read(path: string): unknown {
  return JSON.parse(readFileSync(path, "utf8"));
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "fixture-documents": {
      subject: (ctx) => {
        const projection: Record<string, Record<string, boolean>> = {};
        for (const row of rows(ctx).filter((candidate) => candidate.definition === "Quiz" || candidate.definition === "Catalog")) {
          const document = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(row.fixture!)));
          const quizzes = row.quizzes === "-" ? [] : (matches(document, row.quizzes!)[0]?.[1] as Quiz[]);
          projection[row.id!] = Object.fromEntries(matches(document, row.pointer!).map(([path, value]) => [path, accepted(row.definition!, value, quizzes)]));
        }
        return { projection };
      },
    },
    "repository-quizzes": {
      subject: (ctx) => ({
        projection: Object.fromEntries(
          teachingDocuments(ctx.repoRoot).map((path) => {
            const document = read(join(ctx.repoRoot, path)) as { schema?: string; quizzes?: string[] };
            if (document.schema === "semio.quiz/v1") return [path, accepted("Quiz", document, [])];
            if (document.schema !== "semio.quiz.catalog/v1") return [path, false];
            const quizzes = (document.quizzes ?? []).map((quiz) => read(join(ctx.repoRoot, dirname(path), quiz)) as Quiz);
            return [path, accepted("Catalog", document, quizzes)];
          }),
        ),
      }),
    },
    "rejected-quizzes": {
      subject: (ctx) => {
        type Vector = { id: string; definition: string; document: { quizzes?: unknown } };
        const committed = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(REJECTED))) as { quizzes: Record<string, Quiz>; accepted: Vector[]; rejected: Vector[] };
        const quizzesOf = (document: Vector["document"]): Quiz[] => (Array.isArray(document.quizzes) ? document.quizzes.filter((path): path is string => typeof path === "string" && Object.hasOwn(committed.quizzes, path)).map((path) => committed.quizzes[path]!) : []);
        return { projection: Object.fromEntries([...committed.accepted, ...committed.rejected].map((vector) => [vector.id, accepted(vector.definition, vector.document, quizzesOf(vector.document))])) };
      },
    },
  },
});
