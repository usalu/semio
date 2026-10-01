/** 🧊️ Schema-first GLTF semantic SQLite projection and complete typed reconstruction. */
import {artifactSqliteCheckpoint,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactDialect} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type {GltfSnapshot} from "../🟦️.ts";
import {Write,Read} from "./🧩️control/🟦️.ts";
import * as document from "./📄️document/🟦️.ts";
import * as node from "./🌳️node/🟦️.ts";
import * as mesh from "./🏔️mesh/🟦️.ts";
import * as buffer from "./📦️buffer/🟦️.ts";
import * as material from "./🖌️material/🟦️.ts";
import * as texture from "./🖼️texture/🟦️.ts";
import * as skin from "./🦴️skin/🟦️.ts";
import * as animation from "./🎬️animation/🟦️.ts";
import * as camera from "./🎥️camera/🟦️.ts";
export {GLTF_SQLITE_SCHEMA} from "./🗄️.ts";
export async function projectGltfSnapshotSqlite(snapshot:GltfSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{const write=await Write.create(options);await document.project(write,snapshot);await node.project(write,snapshot.document.nodes);await mesh.project(write,snapshot.document.meshes);await buffer.project(write,snapshot.document);await material.project(write,snapshot.document.materials);await texture.project(write,snapshot.document);await skin.project(write,snapshot.document.skins);await animation.project(write,snapshot.document.animations);await camera.project(write,snapshot.document.cameras);return write.finish()}
export async function reconstructGltfSnapshotSqlite(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<GltfSnapshot>{const read=await Read.create(database,options),snapshot=await document.reconstruct(read);snapshot.document.nodes=await node.reconstruct(read);snapshot.document.meshes=await mesh.reconstruct(read);await buffer.reconstruct(read,snapshot.document);snapshot.document.materials=await material.reconstruct(read);await texture.reconstruct(read,snapshot.document);snapshot.document.skins=await skin.reconstruct(read);snapshot.document.animations=await animation.reconstruct(read);snapshot.document.cameras=await camera.reconstruct(read);await read.finish();return snapshot}
export async function validateGltfSnapshotSqliteDialect(snapshot:GltfSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.stdio.gltf"||dialect.standard!=="2.0"||dialect.subset!=="*")throw Error("GLTF owned SQLite dialect differs");const table=database.tables.find(table=>table.name.toLowerCase()==="gltf_document");if(table?.rows.length!==1||table.rows[0]!.rowid!==1n||table.rows[0]!.values[0]!==1n||table.rows[0]!.values[1]!==snapshot.schema)throw Error("GLTF document identity differs from semantic projection");return[]}
