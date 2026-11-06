# Design Notes

## Preserved Source Rationale

<a id="source-notes-23"></a>

### 🦀️.rs:23

📜️ The published contract check, BEFORE a single guest is touched. Three named refusals, each
carrying the exact field a client has to fix:

1. an artifact-bound inference with neither `artifactId` nor a caller-authored payload field —
   the case that used to spend a whole component compile and 240 s of guest time before the
   guest answered `missing field 'snapshot'` (`📓️pz2-…md` §5.2, `📓️ce3-…md` §3.3);
2. a caller payload missing a field the contract's own `input_schema` declares `required`;
3. an inference that publishes NO contract at all AND was called with no payload — nothing in
   the tree can tell the caller what to send, so saying so beats dispatching `{}`.

🧾️ `input_schema` is the plugin's own JSON Schema TEXT. Only its top-level `required` list is
enforced here: that is what makes the difference between "answers in 200 ms naming the field"
and "hangs" — a full JSON Schema evaluator belongs to the guest that owns the schema.

<a id="source-notes-103"></a>

### 🦀️.rs:103

🔗️ The named artifact's canonical pair, for an inference whose contract binds one. `None` when
no artifact was named or the contract declares no binding; a NAMED artifact this workspace
cannot read is `NOT_FOUND`, never a silent empty document.

⏳️ Binding is the job's own first expensive step — on the first call of a session it activates
the plugin guest the artifact lives in — so it runs as a job: each phase it actually runs is
pushed to the client as progress (`crate::actions::ActivationPhase`), and a cancel is answered as
soon as it lands. Laws: `🧫️fixtures/⏱️binding-cancellation-law.json` and
`🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json`.
