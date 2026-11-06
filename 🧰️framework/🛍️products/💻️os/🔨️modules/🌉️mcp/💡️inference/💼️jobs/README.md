# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

💼️ One inference service model for every declared service. A plugin declares an inference in its
descriptor; this module resolves WHERE it executes and runs it as one job whose lifecycle, events,
cancellation, proposal and approval read the same whichever site runs it:

- **guest** — the service's own plugin guest, in this gateway (the engine `inference_run` uses).
  Its result becomes a PROPOSAL when the service declares a commit action
  (`InferencePayloadContract.commit`); an approval commits that proposal by invoking the action on
  the document through the ordinary `ActionAdapter` path, so the edit reaches the document ledger
  (and, hub-bound, every collaborator) exactly like any other agent edit.
- **hub** — a service the bound hub executes server-side and publishes in its readiness
  (`features.inferenceServices[]`), relayed through the route family the hub publishes with it.

Schema: `🧬️schema/🔣️.json` (`InferenceJobPageV1`, `HubInferenceServiceV1`).
Law: `🧫️fixtures/💼️inference-service-law.json`, replayed by `🧪️tests/💼️inference-service-law`.
