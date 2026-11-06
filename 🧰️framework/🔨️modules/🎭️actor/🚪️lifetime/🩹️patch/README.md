# Design Notes

## Preserved Source Rationale

<a id="source-notes-58"></a>

### 🦀️.rs:58

🔗️ The typed producer and consumer each validate the exact logical patch count.

🧾️ ONE receipt authorizes the WHOLE patch batch a turn carries: a turn result publishes every
surface that was ready and fits its byte budget, and each of those patches is addressed by its
own `(receipt, surface, revision)` when the host acknowledges it. Before 2026-09-15 a turn could
carry at most one patch, so the cardinality here was `1` and that count cap was the reason N
published surfaces cost N + 1 host round trips.
