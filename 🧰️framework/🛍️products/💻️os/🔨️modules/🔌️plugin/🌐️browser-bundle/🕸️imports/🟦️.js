"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PREVIEW2_VENDOR_RELATIVE = void 0;
exports.rewritePreview2ShimImportSource = rewritePreview2ShimImportSource;
var PREVIEW2_SHIM_IMPORT = /(from\s+['"])(?:@bytecodealliance\/preview2-shim|(?:\.\.\/)+(?:🔌️plugin-modules\/)?🪞️vendor\/🤝️bytecode-alliance\/🪟️preview2-shim)\/([\w-]+)(?:\.js)?(['"])/g;
exports.PREVIEW2_VENDOR_RELATIVE = "🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim";
/** 🪢️ Relocates generated component imports into the selected public shim directory. */
function rewritePreview2ShimImportSource(source, prefix) {
    return source.replace(PREVIEW2_SHIM_IMPORT, function (_match, lead, subpath, trail) { return "".concat(lead).concat(prefix).concat(subpath, ".js").concat(trail); });
}
