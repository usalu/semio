# Design Notes

## Preserved Source Rationale

<a id="source-notes-100"></a>

### 🦀️.rs:100

📝️ One applied mutation of a history row: its replica-independent id, applied position and index inside its edit,
its localized kind label, its outcome (the time-travel replay's while a session holds a report, else the durable
one) and its editing state. `editable` = the op has an input schema and emits no foreign steps; `withdrawable` = the
store's supersede law lets this actor withdraw it and it is not withdrawn yet (design §22.1); `pending` = it is
downstream of the mutation being edited and not applied in the preview; `edited` = the session holds a draft for it;
`introduced` = the session's replay outcome carries a message (level and code) its pre-edit durable outcome does not,
so a host marks what the edit made new (design §16.5); `store` = the composed member store that holds it
(`<slot>/<childId>`, design §12), absent for the document's own.

<a id="source-notes-406"></a>

### 🦀️.rs:406

📢️ Framework-owned EN/DE notices of the store and runtime refusals a dispatch answers outside a history-edit session
(an open tool transaction, an exhausted or still replaying history, a whole-document load in flight, a history step whose
replay would leave errors, a withdrawal of a step that also changed other documents), by fault code — every shell shows
these bytes; `{n}` is the edit count a refusal names. Fixture `🧫️fixtures/🧫️history-notices/🔣️.json`; TS twin
`HISTORY_NOTICE_LABELS`.

<a id="source-notes-546"></a>

### 🦀️.rs:546

🗯️ The notice a fault earns: its own code, then each cause's, is looked up in the framework's tables
([`HISTORY_NOTICE_LABELS`], [`FRAMEWORK_FAULT_NOTICE_LABELS`]) first and the refusing app's published table
(`AppDefinition::fault_notices`) second; the first declared code decides, its `{name}` placeholders filled from
`Fault.params` only. `None` when no code is declared or a placeholder has no value — the shell then shows its generic
refusal, never a raw code. Corpus `🛂️manifest/🧫️fixtures/🧫️fault-notices`; TS twin `faultNotice`.

<a id="source-notes-565"></a>

### 🦀️.rs:565

🦜️ Framework-owned EN/DE copy of a history change replaying before this replica adopts it ([`HistoryReprojection`]), by
`<kind>.<part>` key: every shell announces it in a polite status region outside the History panel and the history body
titles its reprojection section with it (design §16.6, gap N17, audit W1E-3). `{done}`/`{total}` are replayed
operations, `{reason}` a refusal's notice. Fixture `🧫️fixtures/🧫️history-reprojection/🔣️.json`; TS twin
`HISTORY_REPROJECTION_LABELS`.

<a id="source-notes-598"></a>

### 🦀️.rs:598

📛️ The one status copy of a history change replaying before adoption, for both shells and the history body: a refused
adoption (a fault with nothing left to replay) reads `<kind>.refused` with the fault's placeholder-free history notice
([`HISTORY_NOTICE_LABELS`]) or `reason.unnamed` — never a raw code; a paused remote change reads `remote.paused` (a step or
a load is never paused); anything else its kind's progress. Labels [`HISTORY_REPROJECTION_LABELS`]; TS twin
`historyReprojectionStatus`.

<a id="source-notes-632"></a>

### 🦀️.rs:632

🎨️ One app-declared fault notice (design §20.12): the `code` a guest refusal carries (`<app>.<area>.<name>`) and its
text in every shell locale × terminology, whose `{name}` placeholders are filled from the fault's `params` — never from
its English `message`. Published in [`AppDefinition::fault_notices`]; schema `🛂️manifest/🧬️schema` `$defs.FaultNoticeDefinition`,
TS twin `FaultNoticeDefinition` (`🤖️generated/🪪️manifest`), corpus `🧫️fixtures/🧫️fault-notices`.
