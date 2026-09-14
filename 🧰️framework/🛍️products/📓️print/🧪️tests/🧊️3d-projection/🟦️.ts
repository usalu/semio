// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { glMatrix, mat4, vec3 } from "gl-matrix";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";

glMatrix.setMatrixArrayType(Array);
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "3d-projection";
const FIXTURE = "local://3d-projection.tex";
const DECIMALS = 5;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the comparison uses. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

type Camera = Readonly<{ projection: string; azimuth: number; elevation: number; distance: number; zoom: number; obliqueAngle?: number; obliqueScale?: number }>;

/** 🧱️ Column-major basis change from the rotated world frame `(depth, screenX, screenY)` onto the page. */
const SCREEN_BASIS = mat4.fromValues(0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1);

/** 🎥️ The camera as one `gl-matrix` `mat4`: the screen basis, the elevation about the view axis, the azimuth about z. */
function cameraMatrix(camera: Camera): mat4 {
  const matrix = mat4.clone(SCREEN_BASIS);
  mat4.rotateY(matrix, matrix, (camera.elevation * Math.PI) / 180);
  mat4.rotateZ(matrix, matrix, (-camera.azimuth * Math.PI) / 180);
  return matrix;
}

/** 🎥️ The cabinet shear as one `gl-matrix` `mat4`, the oblique camera's whole geometry. */
function obliqueMatrix(camera: Camera): mat4 {
  const angle = ((camera.obliqueAngle ?? 45) * Math.PI) / 180;
  const scale = camera.obliqueScale ?? 0.5;
  return mat4.fromValues(1, 0, 0, 0, 0, 1, 0, 0, scale * Math.cos(angle), scale * Math.sin(angle), 1, 0, 0, 0, 0, 1);
}

/** 🔮️ One world point through the oracle's matrices: screen x, screen y and the camera depth. */
function project(camera: Camera, point: readonly [number, number, number]): [number, number, number] {
  const transformed = vec3.create();
  const source = vec3.fromValues(point[0], point[1], point[2]);
  if (camera.projection === "oblique") {
    vec3.transformMat4(transformed, source, obliqueMatrix(camera));
    return [camera.zoom * transformed[0]!, camera.zoom * transformed[1]!, point[2]];
  }
  vec3.transformMat4(transformed, source, cameraMatrix(camera));
  const depth = transformed[2]!;
  const factor = camera.projection === "perspective" ? camera.distance / Math.max(0.2, camera.distance - depth) : 1;
  return [camera.zoom * transformed[0]! * factor, camera.zoom * transformed[1]! * factor, depth];
}

const ISOMETRIC: Camera = { projection: "isometric", azimuth: 45, elevation: 35.26438968275465, distance: 60, zoom: 1 };
const AXONOMETRIC: Camera = { projection: "axonometric", azimuth: 30, elevation: 20, distance: 60, zoom: 2 };
const ORTHOGRAPHIC: Camera = { projection: "orthographic", azimuth: 0, elevation: 0, distance: 60, zoom: 1 };
const PERSPECTIVE: Camera = { projection: "perspective", azimuth: 45, elevation: 30, distance: 10, zoom: 1 };
const OBLIQUE: Camera = { projection: "oblique", azimuth: 0, elevation: 0, distance: 60, zoom: 1, obliqueAngle: 45, obliqueScale: 0.5 };

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    isometric: {
      /** 🔮️ The isometric camera applied to the unit axes and two further points. */
      oracle: () => ({
        projection: {
          "project/isometric": grid(
            ([[1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 1], [-1, 2, 3]] as Array<[number, number, number]>).flatMap((point) =>
              project(ISOMETRIC, point),
            ),
          ),
        },
      }),
      subject,
    },
    axonometric: {
      /** 🔮️ A general axonometric camera and the orthographic preset. */
      oracle: () => ({
        projection: {
          "project/axonometric": grid(
            ([[1, 0, 0], [0, 1, 0], [0, 0, 1], [2, -1, 0.5]] as Array<[number, number, number]>).flatMap((point) =>
              project(AXONOMETRIC, point),
            ),
          ),
          "project/orthographic": grid(
            ([[1, 2, 3], [-2, 0.5, 1]] as Array<[number, number, number]>).flatMap((point) => project(ORTHOGRAPHIC, point)),
          ),
        },
      }),
      subject,
    },
    "perspective-and-oblique": {
      /** 🔮️ The perspective divide and the cabinet shear. */
      oracle: () => ({
        projection: {
          "project/perspective": grid(
            ([[1, 1, 1], [-1, 2, 0], [0, 0, 2]] as Array<[number, number, number]>).flatMap((point) => project(PERSPECTIVE, point)),
          ),
          "project/oblique": grid(
            ([[1, 2, 3], [-1, 0, 2]] as Array<[number, number, number]>).flatMap((point) => project(OBLIQUE, point)),
          ),
        },
      }),
      subject,
    },
  },
});
// #endregion 🧭️Adapter
