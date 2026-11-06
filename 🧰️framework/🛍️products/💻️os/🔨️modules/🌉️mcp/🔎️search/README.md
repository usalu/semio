# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

🔎️ Deterministic capability search — packet `P2-catalog`. BM25 (`k1=1.2`, `b=0.75`) over seven
weighted fields (id ×3, title ×3, use_when ×2, description ×2, parameter names ×1.5, artifact
kind ×1, category/owner/plugin display name ×0.5), a
camelCase/kebab-case-aware tokenizer, en+de stopword filtering, and structural filters. **No LLM,
no randomness, no `HashMap` iteration anywhere in this file** — every collection that could leak
nondeterminism into the ranking is a `Vec`/`BTreeMap`/`BTreeSet`, and every tie is broken by
capability id (`Ord` on `String`) so two searches over the same catalog always agree byte-for-byte.
