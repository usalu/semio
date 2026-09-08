const PREVIEW2_SHIM_IMPORT = /(from\s+['"])(?:@bytecodealliance\/preview2-shim|(?:\.\.\/)+(?:🔌️plugin-modules\/)?🪞️vendor\/🤝️bytecode-alliance\/🪟️preview2-shim)\/([\w-]+)(?:\.js)?(['"])/g;

/** 🪢️ Relocates generated component imports into the selected public shim directory. */
export function rewritePreview2ShimImportSource(source: string, prefix: string): string {
  return source.replace(PREVIEW2_SHIM_IMPORT, (_match, lead, subpath, trail) => `${lead}${prefix}${subpath}.js${trail}`);
}
