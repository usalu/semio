//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
type ContributorDocument = {
  alias?: string;
  aliases?: string[];
  github?: string;
  name?: string;
  names?: string[];
  email?: string;
  emails?: string[];
};

/** 🤝️ The identity rule, restated with a JavaScript regular expression rather than a hand written scanner. */
const identity = (line: string): { name: string; email: string } | null => {
  const match = /(?<!\d)\d{4}(?!\d)\s+(.+?)\s*<([^>]+)>/u.exec(line);
  if (match === null) return null;
  return { name: match[1]!.trim(), email: match[2]!.trim() };
};

/** ✂️ The `Name <email>` split. */
const author = (line: string): { name: string; email: string } => {
  const index = line.indexOf(" <");
  if (index === -1) return { name: line, email: "" };
  const rest = line.slice(index + 2);
  return { name: line.slice(0, index).trim(), email: rest.endsWith(">") ? rest.slice(0, -1) : rest };
};

const fold = (left: string, right: string): boolean => left.toLowerCase() === right.toLowerCase();

/**
 * 🔮️ TypeScript oracle of the contributor identity case.
 *
 * `ajv` is a real draft 2020-12 validator and decides whether each committed
 * `🧑️‍💻️contributor.json` satisfies `🧬️schema/🔣️.json`, so a drifted schema fails here instead of
 * agreeing with itself. The identity rule itself is restated with a JavaScript regular expression
 * — a different engine from either hand written scanner — over real `git log` and `git shortlog`
 * lines, and the alias resolution is re-derived straight from the parsed documents.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "author-lines-resolve-to-aliases": {
      oracle: (ctx) => {
        const schema = JSON.parse(readFileSync(ctx.fixture("asset://🧬️schema/🔣️.json"), "utf8")) as Record<string, unknown>;
        const fixture = JSON.parse(readFileSync(ctx.fixture("shared://🧑️‍💻️contributor-documents.json"), "utf8")) as { documents: { directory: string; json: string }[] };
        const log = JSON.parse(readFileSync(ctx.fixture("shared://🏁️checkpoint-log.json"), "utf8")) as { identities: string[]; malformed: string[] };
        const ajv = new Ajv2020({ strict: false, allErrors: true });
        const validate = ajv.compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/ContributorDocument" });

        const people = fixture.documents.map((entry) => {
          const value = JSON.parse(entry.json) as ContributorDocument;
          if (!validate(value)) throw new Error(`${entry.directory} does not satisfy ContributorDocument: ${ajv.errorsText(validate.errors)}`);
          return {
            alias: value.alias === undefined || value.alias === "" ? entry.directory : value.alias,
            github: value.github === undefined || value.github === "" ? entry.directory : value.github,
            name: value.name ?? "",
            names: value.names ?? [],
            email: value.email ?? "",
            emails: value.emails ?? [],
          };
        });

        const resolve = (name: string, email: string): string => {
          for (const person of people) {
            if (email !== "" && [person.email, ...person.emails].some((candidate) => fold(candidate, email))) return person.alias;
            if (name !== "" && [person.name, ...person.names].some((candidate) => fold(candidate, name))) return person.alias;
          }
          if (name !== "" && email !== "") return `${name} <${email}>`;
          if (name !== "") return name;
          return email;
        };

        const lines = [...log.identities, ...log.malformed];
        const identities = lines.map((line) => {
          const parsed = identity(line);
          return `${line}=${parsed === null ? "-" : `${parsed.name}|${parsed.email}`}`;
        });
        const aliases = lines.map((line) => {
          const parsed = identity(line);
          return `${line}=${parsed === null ? "-" : resolve(parsed.name, parsed.email)}`;
        });
        const authors = [...people.map((person) => `${person.name} <${person.email}>`), ...lines].map((line) => {
          const parsed = author(line);
          const rendered = parsed.email === "" ? parsed.name : `${parsed.name} <${parsed.email}>`;
          return `${line}=${parsed.name}|${parsed.email}|${rendered}`;
        });

        return {
          projection: {
            identities,
            aliases,
            contributors: people.map((person) => [person.alias, person.github, person.name, person.email, person.emails.length, person.names.length].join("|")),
            authors,
          },
        };
      },
    },
  },
});
//#endregion 🔮️Oracle
