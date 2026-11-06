# Design Notes

## Preserved Source Rationale

<a id="source-notes-1"></a>

### 🦀️.rs:1

✨️ `semio_framework_value_derive` — `#[derive(ToValue, FromValue)]` with `#[value(...)]`
container/field attributes, mirroring the subset of `#[serde(...)]` actually used under `✏️s/`
(see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
🔍️research/📓️serde-replacement-surface.md` §Survey).

Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
`fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright — see
`semio-framework-schema-derive`'s identical header note, this crate follows the same shape.

`#[value(crate = "path::to::value_root")]` (container): overrides the crate path every emitted
call site (`ToValue`, `FromValue`, `DslValue`, `ValueError`) is qualified with, defaulting to
`::semio_framework_value` when absent — a container with no `crate` attribute emits
byte-identical code to before this attribute existed. Mirrors `#[serde(crate = "…")]`. Exists so
a crate BELOW `os-kernel` in the dependency DAG (e.g. `semio-framework-actor`, which
`os-kernel` itself depends on, so depending back would be a Cargo cycle) can still use this
derive by pointing it at wherever it reexports `DslValue`/`ToValue`/`FromValue`/`ValueError`
from instead — `#[value(crate = "crate::value")]` etc.

Supported container attributes: `rename_all = "camelCase" | "kebab-case" | "lowercase" |
"snake_case"`, `tag = "…"` (internally-tagged enum), `tag = "…" + content = "…"`
(adjacently-tagged enum). A `tag`-less enum derives too: an all-unit-variant enum becomes a
bare `DslValue::String` of the variant's wire name, matching serde's own default
representation for a data-less enum (`SelectionMode::Single` → `"single"`, not
`{"tag":"single"}`); a `tag`-less enum with at least one data-carrying variant derives as
EXTERNALLY-tagged (serde's own default enum representation when no `#[serde(tag = …)]` is
present) — a unit variant is still the bare wire-name string, a single-unnamed-field or
named-field variant becomes a one-key object `{"VariantName": <payload>}`. `default`
(struct-only; every field on the struct falls back to its own `Default::default()`, or the
type's own if the type itself is `Default`, on a missing key), `deny_unknown_fields`.

🛂️ Container `validate = "path"` runs `fn(&Self) -> Result<(), ValueError>` after cold
construction. `validate_controlled = "path"` runs the corresponding borrowed hook with the
original NativeDecodeControl in an isolated progress stage. A semantic refusal retires the
decoded owner exactly once. A cold hook without a controlled hook or explicit controlled
constructor refuses controlled construction before it allocates.

Supported field attributes: `rename = "…"`, `required` (a present wire key is mandatory even
for `Option<T>`), `default` (bare), `default = "path"`,
`skip_serializing_if = "path"`, `serialize_with = "path"` (`fn(&FieldType) ->
DslValue`, replaces the `ToValue::to_value` call for that field), `deserialize_with = "path"`
(`fn(DslValue) -> Result<FieldType, ValueError>`, replaces the `FromValue::from_value` call —
combine with bare `default` for a "missing key defaults, present key goes through the custom
fn" split, the `deserialize_double_option` shape), `with = "path"` (shorthand for
`serialize_with = "path::to_value"` + `deserialize_with = "path::from_value"`; an explicit
`serialize_with`/`deserialize_with` given alongside `with` wins for that one direction).
`serialize_controlled_with` supplies the explicit borrowed serializer under cumulative
`NativeEncodeControl`; ordinary custom serializers are refused by controlled output.
`skip`
(struct fields only — omitted entirely on serialize; `Default::default()`, or `default =
"path"` alongside it, on deserialize, with no lookup against the wire object at all), `flatten`
(struct fields only — on serialize, splices the field's own object entries straight into the
parent object instead of nesting under the field's wire name; on deserialize, collects every
entry NOT claimed by a sibling field into that field's own `FromValue::from_value`). Combining
`flatten` with `deny_unknown_fields` on the same struct is a `compile_error!`, matching serde's
own restriction — the two are inherently at odds, since a flattened field's whole point is to
absorb keys the container does not itself recognize.
A missing `Option<T>` field decodes as `None` without requiring `#[value(default)]`, matching
serde for both structs and named enum-variant payloads.

`#[value(transparent)]` (container, struct-only): the struct must have exactly one field
(named or unnamed) — the whole struct forwards straight to/from that field's own
`ToValue`/`FromValue`, no object wrapper.

A single-field TUPLE struct (`struct Foo(pub u32);`, no `#[value(...)]` at all) derives as
transparent AUTOMATICALLY, with no attribute needed — `ToValue` emits exactly what the inner
field's own `ToValue::to_value` emits, `FromValue` decodes the inner type and wraps it back in
`Self`. This is the newtype-wrapper idiom (`id_newtype!`-style `pub struct FooId(pub u32)`),
distinct from `#[value(transparent)]` on a NAMED-field struct: the tuple case needs no
attribute because a one-field tuple struct has no other sensible wire representation (there is
no field name to key an object under). A tuple struct with more than one field, or a unit
struct, still hits the `named-field structs … not tuple/unit structs` error below — only the
exactly-one-field tuple shape gets this transparent treatment.

A generic struct/enum gets an AUTOMATIC `Param: ToValue` (resp. `FromValue`) bound synthesized
per own type parameter by default — mirrors `serde_derive`'s own auto-inference default, and
is correct for every generic type this derive has been applied to so far (each parameter is
always reached through a `ToValue::to_value`/`FromValue::from_value` field access). Override
with `#[value(bound = "P1: Trait1, P2: Trait2, …")]` (container) for the rare case a parameter
is unused (e.g. behind `PhantomData`, so the auto bound would be an unsatisfiable-in-practice
over-constraint) or needs a different bound shape — both the `ToValue` and `FromValue` impl
get the SAME literal predicates you write, so write one valid for both (e.g.
`"K: ToValue + FromValue"` if a field of type `K` needs both).

`deny_unknown_fields` is enforced for `Data::Struct` (unknown keys in the decoded object become
a `ValueError`) AND for every `Data::Enum` representation, with "unknown field" scoped
differently per representation to match what serde itself would reject:
- **unit-only** (bare-string) enums: not applicable — the wire form is a single string matched
  exactly against the variant names, so there is no object and no extra-key slot to smuggle
  anything into; an unrecognized string is already a hard `"unknown variant"` error regardless
  of this attribute. Setting the attribute here is accepted and does nothing extra.
- **externally tagged** (no `tag`, mixed variants): the outer object is inherently exactly one
  key (`{"VariantName": payload}` — enforced unconditionally via an `entries.len() != 1` check,
  independent of this attribute), so the only enforcement `deny_unknown_fields` adds is on a
  NAMED-field variant's own payload keys (checked against that variant's known field names). A
  single-unnamed-field variant's payload is handed whole to that field type's own `FromValue` —
  its unknown-field policy is that type's business, not this container's.
- **adjacently tagged** (`tag` + `content`): checked at two independent levels — the outer
  object's keys must be a subset of `{tag, content}` (checked once, before the tag is even
  read, since it does not depend on which variant matched), and a NAMED-field variant's
  `content` object keys must be a subset of just that variant's own field names (the tag never
  appears inside `content`, only alongside it at the outer level). A single-unnamed-field
  variant's `content` payload is again that field type's own business.
- **internally tagged** (`tag` only, fields inline beside it): checked per matched variant,
  since the allowed key set depends on which variant the tag names — a unit variant only
  allows the bare `{tag}` key; a named-field variant allows `{tag} ∪ its own field names`. A
  single-unnamed-field variant hands the entries object to that field type's own `FromValue`
  with the tag key STRIPPED first (encode never puts it there either — see the payload-facing
  `Fields::Unnamed`/`None` arm in `expand_from_value` — so a payload type carrying its own
  `deny_unknown_fields` must not see it), and no further check is added here: the payload type
  decides its own policy for everything else.

An internally tagged single-unnamed-field variant whose payload does NOT encode to an object
(a `String`, a number, an array — serde refuses this shape outright, this derive does not) is
carried as the single entry `{"value": <payload>}` beside the tag. Decoding is the inverse of
that runtime branch, in that order: the tag-stripped object is offered to the payload's
`FromValue` first, and only when that fails is a lone `value` entry unwrapped and offered
bare. The order matters — a payload type whose only field is itself named `value` produces an
indistinguishable key set, and object-first decodes both correctly.

`rename_all_fields = "…"` on an enum supplies its named variant fields' default casing.
`rename_all = "…"` on a variant overrides that default for its own fields; an explicit field
`rename` takes precedence over both. Container `rename_all` controls variant tags only, so an
enum declaring only that attribute wires its named variant fields under their Rust identifiers
verbatim. These scopes match Serde's container, variant and field attributes.

An enum variant's OWN named field (unlike a plain struct field) supports only `rename`,
`required`, `default`, `skip`, and `skip_serializing_if` — `skip` omits the field on serialize and always
falls back to `default`/`Default::default()` on deserialize (no wire lookup at all), and
`skip_serializing_if = "path"` omits the field on serialize when `path(&field)` is `true`,
exactly like their plain-struct-field counterparts. `flatten`/`with`/`serialize_with`/
`deserialize_with` on an enum variant's own named field remain Deliberately NOT supported (rare
in the survey — under 5 occurrences repo-wide) and are now a `compile_error!` naming the field
rather than a silent no-op — a crate needing one of these keeps it hand-written (`impl
ToValue`/`impl FromValue` directly) rather than deriving. Also Deliberately NOT supported: tuple
variants with more than one unnamed field.

<a id="source-notes-365"></a>

### 🦀️.rs:365

🧭️ Resolves `#[value(crate = "path::to::value_root")]` to the crate-path prefix every emitted
call site interpolates as `#value_crate::Type` — defaults to `::semio_framework_value` when
absent, so a container with no `crate` attribute emits byte-identical code to before this
attribute existed. Lets a sub-kernel crate (e.g. `semio-framework-actor`, which cannot depend on
`semio-os-kernel` without a Cargo cycle) reexport `DslValue`/`ToValue`/`FromValue`/
`ValueError` from wherever it actually gets them and point the derive there instead.

<a id="source-notes-423"></a>

### 🦀️.rs:423

🧬️ Clones `generics` and adds the `where` bound this impl needs for each of its OWN type
parameters: by default, one `Param: #trait_path` predicate per type parameter (mirrors
`serde_derive`'s own auto-inference default — every generic struct/enum this derive has seen
so far needs exactly this, an owned field access through `ToValue::to_value`/
`FromValue::from_value` on that parameter). `#[value(bound = "P1: Trait1, P2: Trait2, …")]`
overrides this entirely (both impls get the SAME literal predicates you write — see the module
docs' `bound` entry) for the rare case a parameter is unused (e.g. behind `PhantomData`) or
needs a different bound shape than the uniform default.

<a id="source-notes-631"></a>

### 🦀️.rs:631

#region 🔖️VariantFields
🎯 Rejects `flatten` on an enum variant's own named field with a `compile_error!` naming the
field, instead of the previous silent drop — module docs call `flatten` "Deliberately NOT
supported" on a variant's named fields (splicing into an already-tagged object is ambiguous),
but nothing enforced that until now. `rename`, `required`, `default`, `skip`, `skip_serializing_if`,
`serialize_with`/`deserialize_with`/`with` ARE supported on a variant's own named field (see
`variant_field_to_value_push`/`variant_field_from_value_read` below) — `default` and the
`*_with` trio already worked in practice (🏪️store's `ArtifactActorMsg::LocalMutations`/
`ArtifactEvent::RemoteMutations`/`ArtifactMutationsSaved.envelope` all rely on
`serialize_with`/`deserialize_with` on an internally-tagged variant's own field to route
`MutationEnvelope` through its hand-written bridge), `skip`/`skip_serializing_if` did not (both
fixed below — same silent-drop bug class).

<a id="source-notes-650"></a>

### 🦀️.rs:650

📤️ Emits one named enum-variant field's `ToValue` push into the accumulator `push_into`
(`content_entries` for externally/adjacently-tagged, `__out_entries` for internally-tagged),
honoring `skip` (omit unconditionally), `skip_serializing_if` (omit conditionally), and
`serialize_with`/`with` (replaces the default `ToValue::to_value` call) — mirrors
`to_value_object_entries`'s struct-field handling of the same attributes. Fixes the silent
wire-shape bug where `skip`/`skip_serializing_if` were parsed off an enum variant field and then
never consulted, so the field was always emitted via the default `ToValue::to_value` regardless
of a `serialize_with` naming a different one.
