# Official IFC4X3_ADD2 Examples

These unchanged source files are the complete 46-model corpus in [buildingSMART/IFC4.x-specification-models at ef22440ec501551ccc9689309560bbc7e26aa9d7](https://github.com/buildingSMART/IFC4.x-specification-models/tree/ef22440ec501551ccc9689309560bbc7e26aa9d7). Copyright belongs to buildingSMART International Ltd. The source repository README describes its release and license terms, including CC BY-ND 4.0 for final IFC publications.

`cases.json` retains original paths, pinned raw URLs, Git blob identities, SHA-256 checksums, byte counts, entity counts and independently observed diagnostics. `schema.json` defines that language-neutral contract. File bytes and original headers are preserved.

IfcOpenShell 0.9.0.post1 evaluated all 46 original models with EXPRESS rules enabled. Forty-five had no diagnostics. `basin-advanced-brep.ifc` has one schema error: entity #52 supplies an integer for IfcPerson.Identification, whose domain is IfcIdentifier. The regression suite expects this source error and keeps the official file unchanged.

The corpus exercises lossless STEP parse/write, central no-op diff admission and semantic population evaluation. Source examples are informative; successful checks do not certify model-view or complete geometric representation conformance.
