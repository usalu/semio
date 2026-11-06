# Design Notes

## Preserved Source Rationale

<a id="source-notes-3"></a>

### 🦀️.rs:3

#region 🔁️Value
🌱️ First-party replacement for the `#[cfg(test)]`-only `Serialize`/`Deserialize` above — see
that region's doc. Written against `super::ToValue`/`FromValue` (a RELATIVE path) rather than
`#[derive(ToValue, FromValue)]`, because this component is mounted by BOTH `os-kernel` (where
the derive's hard-literal `::semio_os_kernel::…` path would resolve) and
`replication` (where it would not — `replication` sits below `os-kernel` in the DAG); a
relative path resolves correctly under either mount point.
