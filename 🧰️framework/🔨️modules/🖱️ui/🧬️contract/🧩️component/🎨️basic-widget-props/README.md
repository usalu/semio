# Design Notes

## Preserved Source Rationale

<a id="source-notes-84"></a>

### 🦀️.rs:84

🖼️ Icon key. The old `IconName` is generated per-consuming-crate via a `#[path]` mount (see
`🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`), not a publishable
dependency this crate's `Cargo.toml` — which this packet is forbidden from editing — could
take on. A plain `String` icon key is the only viable choice here; flagged as a
registrar-request in `📓️terra-contract-doc-report.md` in case a shared icon crate should
exist instead.
