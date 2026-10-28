import{nxBuildEngineDigestV1}from"../../../../🟨️.mjs";

/** 🔏️ Prints the digest of the private Nx engine closure; the Nx hasher runs it once per client as the engine's single runtime input. */
process.stdout.write(nxBuildEngineDigestV1(process.argv[2])+"\n");
