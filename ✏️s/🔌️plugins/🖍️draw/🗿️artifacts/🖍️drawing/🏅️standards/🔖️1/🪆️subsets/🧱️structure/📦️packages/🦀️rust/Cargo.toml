[package]
workspace = "../../../../../.."
name = "semio-s-artifact-draw-drawing-structure"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "The `structure` subset of the drawing artifact: the layer-tree mutation leaves (create, duplicate, delete, reorder, image assets)"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
semio-framework-dsl-record = { workspace = true }
semio-framework-dsl-record-derive = { workspace = true }
semio-framework-os-kernel = { workspace = true }
semio-framework-pack-json = { workspace = true }
semio-framework-schema = { workspace = true }
semio-framework-ui-locale = { workspace = true }
semio-framework-value = { workspace = true }
semio-framework-value-derive = { workspace = true }
semio-s-artifact-draw-drawing-diff = { workspace = true }
semio-s-artifact-draw-drawing-model = { workspace = true }

[dev-dependencies]
semio-framework = { workspace = true }
semio-framework-async-macros = { workspace = true }
semio-framework-diagnostic = { workspace = true }
semio-framework-os-kernel = { workspace = true, features = ["mutation-testing", "protocol-laws"] }
serde = { workspace = true }
serde_json = { workspace = true }
