/** 🗄️ The compiled core modules of one realm: a page holds the modules its shard workers compiled and seeds them into every later activation, and each shard worker compiles a served core module once for all of its activations.
 * @see ./🧬️schema/🔣️.json
 * @see ../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🔌️actor-loading-and-replies/🟦️.ts */

/** 🌱️ A compiled core module handed from one realm to another (structured clone of a `WebAssembly.Module` shares its compiled code): the served URL it was compiled from and the module. */
export type ShardCoreModuleSeed<Module = WebAssembly.Module> = readonly [url: string, module: Module];

/** 🧊️ Core modules keyed by their served URL, the identity the browser's module map gives the component glue that asks for them: the first holder compiles (or adopts a seed), every further holder shares the same module, and the entry is dropped with its last holder so the module, its code and every instance closure that referenced it become collectable.
 * Self-contained on purpose: the generated shard worker embeds this class's source text. */
export class ShardCoreModules<Module = WebAssembly.Module> {
  readonly compile: (url: string) => Promise<Module>;
  readonly entries = new Map<string, { readonly module: Promise<Module>; ready: Module | undefined; readonly holders: Set<string> }>();
  compilations = 0;

  constructor(compile: (url: string) => Promise<Module>) {
    this.compile = compile;
  }

  /** 🧲️ Holds `url` for `holder`: an existing entry is shared, a seed is adopted without compiling, otherwise the module is compiled once. A failed compilation leaves no entry behind. */
  acquire(holder: string, url: string, seed?: Module): Promise<Module> {
    let entry = this.entries.get(url);
    if (entry === undefined) {
      if (seed === undefined) this.compilations += 1;
      const created = { module: seed === undefined ? this.compile(url) : Promise.resolve(seed), ready: seed, holders: new Set<string>() };
      created.module.then((module) => { created.ready = module; }, () => { if (this.entries.get(url) === created) this.entries.delete(url); });
      this.entries.set(url, created);
      entry = created;
    }
    entry.holders.add(holder);
    return entry.module;
  }

  /** 🧹️ Drops every hold of `holder` and answers the URLs whose last holder it was. */
  release(holder: string): string[] {
    const released: string[] = [];
    for (const [url, entry] of this.entries) {
      if (!entry.holders.delete(holder) || entry.holders.size > 0) continue;
      this.entries.delete(url);
      released.push(url);
    }
    return released;
  }

  /** 🌾️ The compiled modules held right now, as seeds for another realm. */
  seeds(): ShardCoreModuleSeed<Module>[] {
    const seeds: ShardCoreModuleSeed<Module>[] = [];
    for (const [url, entry] of this.entries) if (entry.ready !== undefined) seeds.push([url, entry.ready]);
    return seeds;
  }

  /** 🔢️ How many holders share `url`. */
  holders(url: string): number {
    return this.entries.get(url)?.holders.size ?? 0;
  }
}
