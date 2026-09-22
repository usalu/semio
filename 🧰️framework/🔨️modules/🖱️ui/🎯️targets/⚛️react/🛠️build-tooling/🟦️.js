"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.uiTailwindBuildPlugins = uiTailwindBuildPlugins;
exports.uiReactBuildPlugin = uiReactBuildPlugin;
exports.defineOwnedTestConfig = defineOwnedTestConfig;
exports.defineOwnedBuildConfig = defineOwnedBuildConfig;
exports.defineOwnedBuildConfigFactory = defineOwnedBuildConfigFactory;
//#region 🔌️Adapters
var vite_1 = require("@tailwindcss/vite");
var plugin_react_1 = require("@vitejs/plugin-react");
var config_1 = require("vitest/config");
//#endregion 🔖️OwnedBuildContract
//#region 🏭️Factories
/** @emoji 🎨️ Tailwind's temporary build adapter behind the UI package that declares it. */
function uiTailwindBuildPlugins() {
    return (0, vite_1.default)();
}
/** @emoji ⚛️ React's temporary build adapter behind the UI package that declares it. */
function uiReactBuildPlugin() {
    return (0, plugin_react_1.default)();
}
/** @emoji 🧪️ Defines a test/build config without exporting Vitest or Vite types. */
function defineOwnedTestConfig(config) {
    return (0, config_1.defineConfig)(config);
}
/** @emoji 🏗️ Identity helper for owned build configs that need no implementation runtime. */
function defineOwnedBuildConfig(config) {
    return config;
}
/** @emoji 🏭️ Identity helper for an owned build config a build tool resolves per command, the form a
 * configuration whose shape depends on `serve` versus `build` must take. */
function defineOwnedBuildConfigFactory(factory) {
    return factory;
}
//#endregion 🏭️Factories
