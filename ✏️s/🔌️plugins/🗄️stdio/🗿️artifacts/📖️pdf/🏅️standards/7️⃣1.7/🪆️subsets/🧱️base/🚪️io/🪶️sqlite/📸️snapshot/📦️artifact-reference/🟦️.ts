/** 📦️ Explicit artifact identity and declared dialect columns. */
import type { ArtifactRef } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import { parseArtifactRef } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import { PdfProjection,PdfReader } from "../🧩️entity/🟦️.ts";
export const PDF_ARTIFACT_REFERENCE_SQLITE_SCHEMA = "CREATE TABLE pdf_artifact_reference (id INTEGER PRIMARY KEY, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);";
/** 📤️ Persists exactly one independently owned artifact reference. */
export async function writePdfArtifactReference(out:PdfProjection,reference:ArtifactRef):Promise<bigint>{return out.insert("pdf_artifact_reference",[reference.artifactId,reference.dialect.artifactKind,reference.dialect.standard,reference.dialect.subset]);}
/** 📥️ Admits a canonical first-party reference reconstructed from literal scalar cells. */
export async function readPdfArtifactReference(reader:PdfReader,key:bigint):Promise<ArtifactRef>{const row=await reader.take("pdf_artifact_reference",key,5);return parseArtifactRef({artifactId:await reader.text(row,1),dialect:{artifactKind:await reader.text(row,2),standard:await reader.text(row,3),subset:await reader.text(row,4)}});}
