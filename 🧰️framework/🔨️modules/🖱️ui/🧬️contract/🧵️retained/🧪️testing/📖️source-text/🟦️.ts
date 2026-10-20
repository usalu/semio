/** 📖️ Public entry `@semio-tech/ui-contract/testing/source-text/retained-fixtures`: typed loaders of the retained resident and owned scene JSON fixture texts for the ui fixture corpus. */
/// <reference types="vite/client" />
export const loadRetainedOwnedSceneFixtureText = (): Promise<string> => import("../../🧪️tests/🧫️fixtures/🎬️owned-scene/🔣️.json?raw").then(module => module.default);
export const loadRetainedResidentReaderFixtureText = (): Promise<string> => import("../../💾️resident/📖️reader/🧪️tests/🧫️fixtures/🔣️.json?raw").then(module => module.default);
export const loadRetainedResidentPageFixtureText = (): Promise<string> => import("../../💾️resident/📃️page/🧪️tests/🧫️fixtures/🔣️.json?raw").then(module => module.default);
