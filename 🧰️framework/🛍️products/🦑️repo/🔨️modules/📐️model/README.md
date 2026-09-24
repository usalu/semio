# 📐️ Model

The domain nouns every other module speaks: repo, technology, bundle, folder, file, section,
definition, ticket, goal, breach, contributor, checkpoint, draft and todo, their JSON wire encoding in
declaration order, the LLM/effort/client slug vocabulary and its normalisation, and the derivation of
a definition kind from a raw declaration keyword.

## 📦️ Packages

- `📦️packages/🦀️rust` — `semio-framework-repo-model`
- `📦️packages/🐹️go` — `github.com/usalu/semio/repo/model`

## 🧪️ Tests

One `🥒️.feature` per case under `🧪️tests/` with an adapter per implementation, run through the
`🧪️test` harness; `ajv` judges every golden document against `🧬️schema/🔣️.json` with a real JSON
Schema draft 2020-12 implementation in `🔮️oracles/🔣️.json`, and the slug vocabulary and the definition
kind taxonomy rest on recorded no-oracle decisions.

`🔣️json-encoding-conformance`, `🔤️slug-normalisation`, `🧬️definition-kind-derivation`.
