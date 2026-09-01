//#region 🧊️GltfAnySubset
//! 🫙️ Deliberately empty. `📦️glue.rs` builds `pub mod subsets { pub mod any { … } }` inline
//! (`#[path = "."]`) and only `#[path]`s into the real leaves — `🚪️io/🦀️component.rs` and
//! `🧬️schema/🦀️component.rs` — never into this level. No stdio sibling (`🟪️stl`, `🎒️zip`,
//! `📄️pdf`, `🧿️semio`'s 16-subset standard, …) mounts code directly inside a
//! `🪆️subsets/<subset>/` folder either; declarations live one level further down. Not part of
//! any `mod` tree.
//#endregion 🧊️GltfAnySubset
