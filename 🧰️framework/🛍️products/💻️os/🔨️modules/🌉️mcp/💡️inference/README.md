# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

💡️ Inference access: discovery, execution and the job quartet over ONE service model. Every
plugin's committed `🔣️.json` declares its inference roster — `contributions.inference_services`
plus every `artifact_contributions[].inferences` row, each with its published payload contract —
and that roster is the only source this module reads services from; no plugin id is hardcoded.

- `inference_list`/`inference_get` answer discovery from the declared roster.
- `inference_submit`/`inference_events`/`inference_cancel`/`inference_approve` run one declared
  service as a job (`💼️jobs`): in its plugin's guest here, or — for a service the bound hub
  publishes in `features.inferenceServices` — on the hub, through the route family it publishes.
  A guest result whose contract declares a commit action is offered as a proposal; approving it
  invokes that action on the document through `ActionAdapter`, the normal edit path.
- `inference_run` is the synchronous sibling: same resolution, same guest engine, no proposal.

Schema: `🧬️schema/🔣️.json`. Laws: `🧫️fixtures/💼️inference-service-law.json`,
`🧫️fixtures/⏱️binding-cancellation-law.json`, and the hub wire corpus
Installed owner declarations bind request and reply codecs.
