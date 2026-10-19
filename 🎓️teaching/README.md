# 🎓️ Teaching

The teaching area: interactive learning material built on the domain-neutral framework products. It holds one server,
the proctor, and one site per subject; each site offers a course of quizzes whose leaves live in a topic tree.

## Layout

| Path | What lives there |
|---|---|
| `🛂️proctor/` | The proctor: a Rust `ServerInstance` of the framework server product (crate `teaching-proctor`, binary `proctor`, project `@teaching/proctor`). Identity without passwords, randomized runs, whole-run scoring, badges and the leaderboard as CQRS with event sourcing over one SQLite file; an API only, deployed on Docker behind Caddy at `https://semio.iek.uni-hannover.de`. |
| `🏛️architecture/` | The site `https://architektur-und-technologie.de` (see [its README](🏛️architecture/README.md)). |
| `🏛️architecture/📚️course/` | The site's course `🔣️.json`, its web package `@teaching/architecture-course` (a static build published to a CDN) and its deployment files. |
| `🏛️architecture/<domain>/<topic>/📚️course/🔣️.json` | One quiz per topic leaf. |

## Topic tree

```
🎓️teaching/
  🛂️proctor/                         API for every site (semio.iek.uni-hannover.de)
  🏛️architecture/                    site architektur-und-technologie.de
    📚️course/🔣️.json                 course "architecture": introduction, quiz paths, badges
    ⚡️energy/                         domain
      🧲️physics/   ❓️quiz  🎬️clip    Physical Understanding / Physikalisches Verständnis
      🔥️heating/   ❓️quiz  🎬️clip    Heating / Heizen
      ❄️cooling/   ❓️quiz  🎬️clip    Cooling / Kühlen
      📊️demand/    ❓️quiz  🎬️clip    Energy Demand / Energiebedarf
```

A topic has two kinds of leaves: `❓️quiz/🔣️.json`, a `semio.quiz/v1` document, and `🎬️clip`, the intended place for
short explanatory videos of the same topic (not built yet). Topics group by domain so that further domains (e.g.
structure, materials) slot in beside `⚡️energy` without touching existing paths.

## How the pieces fit

- The contract lives in the framework product [`🧰️framework/🛍️products/📚️course`](../🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json):
  quizzes, courses, badges, sheets, answers, results, commands, events and views; TypeScript and Rust cores implement
  it twin for twin.
- The proctor reads a course (`PROCTOR_COURSE`), resolves its quiz paths relative to the course file and hashes every
  quiz file into its revision; a run is always scored against the revision it started with.
- The site is a static build on a CDN (`https://architektur-und-technologie.de`) that mounts the React renderer
  `@semio-tech/course-react` against the proctor origin baked in at build time
  (`https://semio.iek.uni-hannover.de`); the proctor admits exactly the site origin (CORS). In
  development the Vite dev server proxies the gateway routes to the local proctor, so the client talks to one origin.

## Adding a quiz

1. Pick the topic leaf `🏛️architecture/<domain>/<topic>/📚️course/🔣️.json`; every identifying emoji in a path segment is
   followed by U+FE0F, and every new directory is registered in the repository taxonomy.
2. Write a `semio.quiz/v1` document whose `$schema` points relatively to
   `🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json#/$defs/Quiz`. Ids are English kebab-case slugs; every
   learner-visible text carries `en` and `de`; the required `emoji` is the topic folder's emoji (fully qualified, e.g.
   `🔥`, `❄️`) and matches the quiz's `perfect-quiz` badge.
3. Choose the task kinds:
   - `classification`: categories, optionally with `axes` and one `profile` per category (spider diagrams; a similar
     profile earns partial credit),
   - `sorting`: one `quantity` and strictly distinct values,
   - `matching`: one or more `dimensions`, every item with exactly one value per dimension.
   Use the `logarithmic` scale when values span orders of magnitude (then every value must be positive), keep
   neighbouring values a factor of at least 1.25 apart where physics allows, and set `draw` below the item count so that
   runs differ.
4. Give every item an `explanation` with the value, the reasoning and the source: it is what the results screen teaches.
5. Add the path, relative to the course, to the course's `quizzes` and, if wanted, a `perfect-quiz` badge.
6. Validate with the dashboard commands of the site: `bun repo run @teaching/architecture-course:check` runs `proctor check` on the
   course (Rust core), and `bun repo run @teaching/architecture-course:test` runs the TypeScript core validation and the
   draft-07 contract through ajv on the course and every quiz.
7. Every edit of a quiz file changes its revision, and open runs of the old revision are voided at their next start or
   submission, so batch content changes.
