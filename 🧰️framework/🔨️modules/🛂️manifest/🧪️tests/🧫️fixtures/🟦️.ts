/** 🧫 The manifest module's public fixture entry: the language-neutral corpora its consumers read, exposed by name under one import so no consumer spells the owner's internal folders. */
export { default as manifestPublicInvocationSchema } from "../../🎛️public-invocation/🧬️schema/🔣️.json" with { type: "json" };
export { default as manifestActionSemanticsFixture } from "./⚖️action-semantics.json" with { type: "json" };
export { default as manifestExamplePickerFixture } from "./📚️example-picker.json" with { type: "json" };
export const loadPagedTextCarrierFixture = () => import("./🔬️paged-text-carrier/🔣️.json").then(module => module.default);
export const loadUiRefreshSectionFixture = () => import("./🔬️ui-refresh-section/🔣️.json").then(module => module.default);
export { default as manifestDialogChoicesFixture } from "./🧫️dialog-choices/🔣️.json" with { type: "json" };
export { default as manifestFaultNoticesFixture } from "./🧫️fault-notices/🔣️.json" with { type: "json" };
export { default as manifestNumberFacetsFixture } from "./🧫️number-facets/🔣️.json" with { type: "json" };
export { default as manifestSchema } from "../../🧬️schema/🔣️.json" with { type: "json" };
export { default as manifestUtilitiesInitialFixture } from "../../🪛️utilities/🌅️initial/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
export { default as contextIntegerCarriersFixture } from "../../🪟️view-context/🧪️tests/🧫️fixtures/🔢️integer-carriers/🔣️.json" with { type: "json" };
export const loadContextExtensionInputFixture = () => import("../../🪟️view-context/🧪️tests/🧫️fixtures/🧩️extension-input/🔣️.json").then(module => module.default);
export { default as manifestViewContextSchema } from "../../🪟️view-context/🧬️schema/🔣️.json" with { type: "json" };
export const loadManifestViewContextSchema = () => import("../../🪟️view-context/🧬️schema/🔣️.json").then(module => module.default);
