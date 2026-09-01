//#region 🧊️GltfAnySubset
/** 🫙️ Deliberately empty. `📦️glue.rs` builds the `subsets::any` barrel inline
 * (`#[path = "."] pub mod any { … }`) and only `#[path]`s into the real leaves —
 * `🚪️io/🟦️component.ts` and `🧬️schema/🟦️component.ts` — never into this level. No stdio
 * sibling (`🟪️stl`, `🎒️zip`, `📄️pdf`, `🧿️semio`'s 16-subset standard, …) mounts code directly
 * inside a `🪆️subsets/<subset>/` folder either; declarations live one level further down. Not
 * re-exported by `📦️packages/🟦️typescript/📦️index.ts`. */
export {};
//#endregion 🧊️GltfAnySubset
