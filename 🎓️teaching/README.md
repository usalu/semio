# 🎓️ Teaching

The teaching area: interactive learning material built on the domain-neutral framework products. It holds one server,
the proctor, and one site per subject; each site offers a catalog of quizzes whose leaves live in a topic tree.

## Layout

| Path | What lives there |
|---|---|
| `🛂️proctor/` | The proctor: a Rust `ServerInstance` of the framework server product (crate `teaching-proctor`, binary `proctor`, nx `@teaching/proctor`). Identity without passwords, randomized runs, whole-run scoring, badges and the leaderboard as CQRS with event sourcing over one SQLite file; it also serves the built site. |
| `🏛️architecture/` | The site `quizze.architektur-und-technologie.de` (see [its README](🏛️architecture/README.md)). |
| `🏛️architecture/❓️quiz/` | The site's catalog `🔣️.json`, its web package `@teaching/architecture-quiz` and its deployment `🚀️deploy/`. |
| `🏛️architecture/<domain>/<topic>/❓️quiz/🔣️.json` | One quiz per topic leaf. |

## Topic tree

```
🎓️teaching/
  🛂️proctor/                         server for every site
  🏛️architecture/                    site quizze.architektur-und-technologie.de
    ❓️quiz/🔣️.json                   catalog "architecture": introduction, quiz paths, badges
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

- The contract lives in the framework product [`🧰️framework/🛍️products/❓️quiz`](../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json):
  quizzes, catalogs, badges, sheets, answers, results, commands, events and views; TypeScript and Rust cores implement
  it twin for twin.
- The proctor reads a catalog (`PROCTOR_CATALOG`), resolves its quiz paths relative to the catalog file and hashes every
  quiz file into its revision; a run is always scored against the revision it started with.
- The site mounts the React renderer `@semio-tech/quiz-react` against the proctor on the same origin.

## Adding a quiz

1. Pick the topic leaf `🏛️architecture/<domain>/<topic>/❓️quiz/🔣️.json`; every identifying emoji in a path segment is
   followed by U+FE0F, and every new directory is registered in the repository taxonomy.
2. Write a `semio.quiz/v1` document whose `$schema` points relatively to
   `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json#/$defs/Quiz`. Ids are English kebab-case slugs; every
   learner-visible text carries `en` and `de`.
3. Choose the task kinds:
   - `classification`: categories, optionally with `axes` and one `profile` per category (spider diagrams; a similar
     profile earns partial credit),
   - `sorting`: one `quantity` and strictly distinct values,
   - `matching`: one or more `dimensions`, every item with exactly one value per dimension.
   Use the `logarithmic` scale when values span orders of magnitude (then every value must be positive), keep
   neighbouring values a factor of at least 1.25 apart where physics allows, and set `draw` below the item count so that
   runs differ.
4. Give every item an `explanation` with the value, the reasoning and the source: it is what the results screen teaches.
5. Add the path, relative to the catalog, to the catalog's `quizzes` and, if wanted, a `perfect-quiz` badge.
6. Validate with the launch rows of the site: `bun nx run @teaching/architecture-quiz:check` runs `proctor check` on the
   catalog (Rust core), and `bun nx run @teaching/architecture-quiz:test` runs the TypeScript core validation and the
   draft-07 contract through ajv on the catalog and every quiz.
7. Every edit of a quiz file changes its revision, and open runs of the old revision are voided at their next start or
   submission, so batch content changes.
