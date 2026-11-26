/** ♨️ Keeps the TypeScript programs of a long-lived deriver between questions: every parsed source file is kept by its text and parse options, so an unchanged file is never parsed again, and the previous program of a configuration is handed to `createProgram` as `oldProgram` only while the deriver has armed the store, i.e. after it proved that every probe the previous program's module resolution read (existence, directories, real paths, configuration and package files) still answers the same; TypeScript's own structure reuse then re-resolves exactly the files whose text changed.
 * @see https://github.com/microsoft/TypeScript/wiki/Using-the-Compiler-API#incremental-build-support-using-the-language-services */

let active;

/** 🏭️ A fresh reuse store: programs by configuration, parsed files by configuration and the compiler options they were parsed under (a parsed file carries what its options decided, e.g. the module indicator of `moduleDetection`/`jsx` and the synthesized `importHelpers`/`jsxImportSource` imports, so another option set starts from an empty table), the files of the programs of the running question, and the arm switch. */
export function createTypeScriptProgramReuseV1() {
  const programs = new Map(), parsed = new Map();
  let armed = false, sources = new Set(), used = new Map(), reused = [];
  return Object.freeze({
    begin(arm) { armed = arm === true; sources = new Set(); used = new Map(); reused = []; },
    end() { armed = false; for (const [config, keys] of used) { const files = parsed.get(config)?.files; if (files) for (const key of [...files.keys()]) if (!keys.has(key)) files.delete(key); } const done = { sources, reused }; sources = new Set(); used = new Map(); reused = []; return done; },
    sourceFile(config, original, readFile, settings = "") {
      let entry = parsed.get(config); if (!entry || entry.settings !== settings) parsed.set(config, entry = { settings, files: new Map() });
      const files = entry.files;
      let keys = used.get(config); if (!keys) used.set(config, keys = new Set());
      return (fileName, languageVersionOrOptions, onError, shouldCreateNewSourceFile) => {
        const text = readFile(fileName);
        if (text === undefined) return original(fileName, languageVersionOrOptions, onError, shouldCreateNewSourceFile);
        const options = typeof languageVersionOrOptions === "object" && languageVersionOrOptions !== null ? languageVersionOrOptions : { languageVersion: languageVersionOrOptions };
        const key = `${fileName}\0${options.languageVersion}\0${options.impliedNodeFormat ?? ""}\0${options.jsDocParsingMode ?? ""}`, known = files.get(key);
        keys.add(key);
        if (known && known.text === text && !shouldCreateNewSourceFile) return known.file;
        const file = original(fileName, languageVersionOrOptions, onError, shouldCreateNewSourceFile);
        if (file && file.text === text) files.set(key, { text, file }); else files.delete(key);
        return file;
      };
    },
    textBytes() { let sum = 0; for (const entry of parsed.values()) for (const known of entry.files.values()) sum += known.text.length; return sum; },
    oldProgram(config) { return armed ? programs.get(config) : undefined; },
    keep(config, program) { programs.set(config, program); for (const file of program.getSourceFiles()) sources.add(file.fileName); if (armed) reused.push(program.structureIsReused); },
  });
}

/** 🤝️ The parsed source files of the programs of every owner of this process, shared by the compiler options they were parsed under: an owner whose tsconfig reaches the same declaration files as an earlier owner (`lib.*.d.ts`, the typings of the installed packages) never parses them again; a file is reused only when its text is equal to the text the compiler reads now, so a changed file is parsed again and the programs that reach it see the change. Bounded by `maximumFiles` distinct files (then dropped as a whole), dropped by `clear()` when the owner state is released under memory pressure; it keeps no program and hands none to `createProgram`. */
export function createSharedSourceFilesV1({ maximumFiles = 8000 } = {}) {
  const stores = new Map(), seen = new Set();
  return Object.freeze({
    shared: true,
    sourceFile(_config, original, readFile, settings = "") {
      let store = stores.get(settings); if (!store) stores.set(settings, store = createTypeScriptProgramReuseV1());
      const inner = store.sourceFile("shared", original, readFile, settings);
      return (fileName, ...rest) => { if (!seen.has(settings + "\0" + fileName)) { if (seen.size >= maximumFiles) { stores.clear(); seen.clear(); stores.set(settings, store = createTypeScriptProgramReuseV1()); return store.sourceFile("shared", original, readFile, settings)(fileName, ...rest); } seen.add(settings + "\0" + fileName); } return inner(fileName, ...rest); };
    },
    oldProgram() { return undefined; },
    keep() {},
    clear() { stores.clear(); seen.clear(); },
    size() { return seen.size; },
    textBytes() { let sum = 0; for (const store of stores.values()) sum += store.textBytes(); return sum; },
  });
}

/** 🤝️ The shared parsed files of this process. */
export const sharedSourceFilesV1 = createSharedSourceFilesV1();

/** 🔌️ Installs the store the program questions of this process use (`undefined` uninstalls it); only a long-lived deriver installs one. */
export function useTypeScriptProgramReuseV1(store) { active = store; }

/** 🔎️ The installed store, if any. */
export function activeTypeScriptProgramReuseV1() { return active; }
