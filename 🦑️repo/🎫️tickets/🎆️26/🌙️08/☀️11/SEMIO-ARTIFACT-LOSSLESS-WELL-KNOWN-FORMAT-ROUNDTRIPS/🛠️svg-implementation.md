# SVG Lossless Roundtrip Implementation

## Outcome

`temp/artifacts.svg` is represented by a parsed `XmlDocument` plus persisted `ArtifactSource`. The source stores the exact imported UTF-8 bytes and a BLAKE3 fingerprint of the semantic projection. Export replays the imported bytes only while the current semantic projection still matches; an intentional structural edit emits the XML writer's deterministic normal form.

The format-local implementation covers direct native import/export, artifact pack, DSL, self-diff, no-op mutation, representative mutation plus inverse, diff absorb, diff text/binary codecs, and `SetSnapshot` operation text/binary codecs.

## Format-local files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- The SVG XML import/export converter leaves and the SVG diff/mutation grammar/protocol declarations were updated consistently with the Rust source field and codecs.

## Static validation

- `xmllint --noout temp/artifacts.svg`: success.
- SHA-256: `62a1922aad9e06ba7d4a55fe13c360f286eb720873bcf6d8e6ffd1d52e782fc9`.
- Size: `423414` bytes.
- `file`: `SVG Scalable Vector Graphics image`.
- The fixture path used by existing Rust tests resolves from `CARGO_MANIFEST_DIR` through `../../../../../temp/artifacts.svg`.

The shared Nx/Cargo compile was intentionally not run in this lane; root owns the single shared compile target.

## Shared XML boundary

The direct native, pack, DSL, analyzer, and composer paths preserve exact bytes. The semantic `SvgSnapshot -> XmlSnapshot -> SvgSnapshot` converter cannot retain lexical byte identity because the current shared `XmlSnapshot` contains only `schema` and parsed `doc`; it has no source backing. The SVG serializer already exports through `SvgSnapshot::export_utf8`, but parsing those bytes into `XmlSnapshot.doc` necessarily discards lexical choices before the reverse converter runs. Root must own any shared `XmlSnapshot` source-provenance extension and its cross-format diff/mutation implications.
