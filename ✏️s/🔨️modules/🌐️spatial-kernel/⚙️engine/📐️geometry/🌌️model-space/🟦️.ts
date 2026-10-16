/** 🌌️ The model space: linked models, their hashes, transfer by transformation and serialization. */
import type { SpatialPreviewKernel } from "../../🗺️spatial/🟦️.ts";
import { Model, hashModelPrimitives, hashModelVertices } from "../🧊️model/🟦️.ts";
import type { TransformationSpec } from "../🔄️transformation-spec/🟦️.ts";
import { applyTransformation } from "../🔁️transformation-apply/🟦️.ts";
import type { ModelJson, GeometryPrimitiveHash, ModelPrimitiveHashes } from "../🟦️.ts";

/** 🗺️ Serializable model space (`spatial.modelspace/v1`). */
export interface ModelSpaceJson {
  readonly schema: "spatial.modelspace";
  readonly revision: number;
  readonly models: readonly { readonly id: string; readonly model: ModelJson }[];
}

/** 🌌️ Container for linked models; geometry vertices are hashed per model. */
export class ModelSpace {
  revision = 0;
  models: Record<string, Model> = {};

  /** 🔗️ Registers or replaces a linked model. */
  link(modelId: string, model: Model): void {
    this.models[modelId] = model;
    this.bump();
  }

  /** ✂️ Removes a linked model. */
  unlink(modelId: string): void {
    if (!(modelId in this.models)) return;
    delete this.models[modelId];
    this.bump();
  }

  /** 🔍️ Returns a linked model or `null`. */
  get(modelId: string): Model | null {
    return this.models[modelId] ?? null;
  }

  /** #⃣ Vertex position hashes keyed by linked model id. */
  vertexHashesByModel(): Readonly<Record<string, Readonly<Record<string, GeometryPrimitiveHash>>>> {
    const out: Record<string, Readonly<Record<string, GeometryPrimitiveHash>>> = {};
    for (const [modelId, model] of Object.entries(this.models)) out[modelId] = hashModelVertices(model);
    return out;
  }

  /** #⃣ Full primitive hashes keyed by linked model id. */
  geometryHashesByModel(): Readonly<Record<string, ModelPrimitiveHashes>> {
    const out: Record<string, ModelPrimitiveHashes> = {};
    for (const [modelId, model] of Object.entries(this.models)) out[modelId] = hashModelPrimitives(model);
    return out;
  }

  /** 🔄️ Transfers a transformation from a linked source model into a new linked target model. */
  transfer(linkedSourceId: string, linkedTargetId: string, spec: TransformationSpec, preview: SpatialPreviewKernel): Model {
    const source = this.models[linkedSourceId];
    if (!source) throw new Error(`ModelSpace: unknown source model ${linkedSourceId}`);
    const target = applyTransformation(spec, source, preview);
    this.link(linkedTargetId, target);
    return target;
  }

  /** 🧭️ Serializes linked models (stable id order). */
  toJSON(): ModelSpaceJson {
    const models = Object.keys(this.models)
      .sort()
      .map((id) => ({ id, model: this.models[id]!.toJSON() }));
    return { schema: "spatial.modelspace", revision: this.revision, models };
  }

  /** 🧭️ Hydrates from `ModelSpaceJson`. */
  static fromJSON(json: ModelSpaceJson): ModelSpace {
    const space = new ModelSpace();
    space.revision = json.revision;
    for (const row of json.models ?? []) space.models[row.id] = Model.fromJSON(row.model);
    return space;
  }

  bump(): void {
    this.revision += 1;
  }
}
