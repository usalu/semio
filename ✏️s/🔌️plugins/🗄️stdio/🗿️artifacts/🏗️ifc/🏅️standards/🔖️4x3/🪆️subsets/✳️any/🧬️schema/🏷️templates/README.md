# IFC4X3_ADD2 Property and Quantity Templates

The unchanged `annex-a-psd.zip` is from buildingSMART International Ltd.'s [formal ADD2 release](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/annex-a-psd.zip), pinned to commit `610ee3f95db9d0380b1eb8de4dc6e17833531518`. Its SHA-256 is `0e2f4c117cc9439bba23f44600c0e1a97bf248eeffc05376fcc5e03dd6d9d8e3`. Original release ownership and license terms apply to that source.

`schema.json` defines the owned catalog contract. `🔣️.json` and `data.rs` retain 645 property sets, 115 quantity sets and 3988 entries in immutable TypeScript/Rust interfaces. Each template retains its original XML file hash. Applicability distinguishes entity inheritance, exact predefined types, missing required predefined types and unknown entities. Field-form labels require explicit English or German locale selection; original definitions are identified as English source text.

The independent IfcOpenShell 0.9.0.post1 template distribution and actual quantity declarations corroborate every template name/type/applicability string and entry form/domain/enumeration. Quantity domains are resolved from the corresponding explicit *Value attribute. `cases.json` and `tests.feature` define shared regression cases. Native tests compare every retained field against the canonical JSON when authentic native execution is available.

These interfaces expose released authoring metadata. Property/quantity creation must still pass through the artifact's admitted mutations, central diff transaction and inverse infrastructure. The metadata does not itself assert MVD conformance or create every optional source property automatically.
