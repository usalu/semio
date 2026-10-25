/** 🌍️ Differential checks of the geo and spatial kernels against the d3 packages registered as their oracles.
 *
 * @see ../🧰️probe-kit/🟦️.ts — the check shape and the d3 path recorder
 * @see ../🟦️.ts — the differential check table that assembles the topics
 */
import * as mark from "../../✒️mark/🟦️.ts";
import * as geo from "../../🌍geo/🟦️.ts";
import * as spatial from "../../📍spatial/🟦️.ts";
import { type Level, type VizKernelCheck, recordD3Path } from "../🧰️probe-kit/🟦️.ts";

const GEO_PROBES: [number, number][] = [
  [0, 0],
  [10, 45],
  [-73.5, 40.7],
  [139.7, 35.7],
  [-58.4, -34.6],
  [18.4, -33.9],
  [-0.1, 51.5],
  [151.2, -33.9],
  [37.6, 55.8],
];

const DEMO_POLYGON = {
  type: "Polygon" as const,
  coordinates: [
    [
      [-10, -5],
      [-11, 6],
      [-2, 14],
      [12, 8],
      [10, -5],
      [-10, -5],
    ] as [number, number][],
  ],
};

/** 🩺️ Differential checks for `🌍geo` against `d3-geo`. */
export async function geoChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Geo = await import("d3-geo");
  const kinds = ["equirectangular", "mercator", "transverse-mercator", "azimuthal-equal-area", "azimuthal-equidistant", "gnomonic", "orthographic", "stereographic", "conic-conformal", "conic-equal-area", "conic-equidistant", "albers", "equal-earth", "natural-earth"] as const;
  const d3Factories = {
    equirectangular: d3Geo.geoEquirectangular,
    mercator: d3Geo.geoMercator,
    "transverse-mercator": d3Geo.geoTransverseMercator,
    "azimuthal-equal-area": d3Geo.geoAzimuthalEqualArea,
    "azimuthal-equidistant": d3Geo.geoAzimuthalEquidistant,
    gnomonic: d3Geo.geoGnomonic,
    orthographic: d3Geo.geoOrthographic,
    stereographic: d3Geo.geoStereographic,
    "conic-conformal": d3Geo.geoConicConformal,
    "conic-equal-area": d3Geo.geoConicEqualArea,
    "conic-equidistant": d3Geo.geoConicEquidistant,
    albers: d3Geo.geoAlbers,
    "equal-earth": d3Geo.geoEqualEarth,
    "natural-earth": d3Geo.geoNaturalEarth1,
  };
  const probes = level === "quick" ? GEO_PROBES.slice(0, 5) : GEO_PROBES;
  return [
    ...kinds.map((kind) => ({
      module: "geo",
      name: `project-${kind}`,
      subject: () => probes.map((point) => geo.vizGeoProjection(kind)(point)),
      oracle: () => probes.map((point) => d3Factories[kind]()(point)),
      tolerance: 1e-9,
    })),
    ...kinds
      .filter((kind) => kind !== "natural-earth" && kind !== "gnomonic")
      .map((kind) => ({
        module: "geo",
        name: `invert-${kind}`,
        subject: () => probes.map((point) => geo.vizGeoProjection(kind).invert(geo.vizGeoProjection(kind)(point))),
        oracle: () => probes.map((point) => d3Factories[kind]().invert!(d3Factories[kind]()(point)!)),
        tolerance: 1e-7,
      })),
    {
      module: "geo",
      name: "project-rotated-and-scaled",
      subject: () => probes.map((point) => geo.vizGeoProjection("orthographic", { rotate: [30, -20, 15], scale: 300, translate: [100, 90] })(point)),
      oracle: () => probes.map((point) => d3Geo.geoOrthographic().rotate([30, -20, 15]).scale(300).translate([100, 90])(point)),
      tolerance: 1e-9,
    },
    {
      module: "geo",
      name: "graticule-lines",
      subject: () => {
        const grid = geo.vizGraticule();
        return { count: grid.coordinates.length, first: grid.coordinates[0], last: grid.coordinates[grid.coordinates.length - 1] };
      },
      oracle: () => {
        const grid = d3Geo.geoGraticule()();
        return { count: grid.coordinates.length, first: grid.coordinates[0], last: grid.coordinates[grid.coordinates.length - 1] };
      },
      tolerance: 1e-9,
    },
    {
      module: "geo",
      name: "path-bounds-area-centroid",
      subject: () => {
        const projection = geo.vizGeoProjection("equirectangular");
        const bounds = geo.vizGeoBounds(DEMO_POLYGON, projection);
        return { bounds: [bounds.x0, bounds.y0, bounds.x1, bounds.y1], area: geo.vizGeoArea(DEMO_POLYGON, projection), centroid: geo.vizGeoCentroid(DEMO_POLYGON, projection) };
      },
      oracle: () => {
        const path = d3Geo.geoPath(d3Geo.geoEquirectangular());
        const bounds = path.bounds(DEMO_POLYGON as never);
        return { bounds: [bounds[0][0], bounds[0][1], bounds[1][0], bounds[1][1]], area: path.area(DEMO_POLYGON as never), centroid: path.centroid(DEMO_POLYGON as never) };
      },
      tolerance: 1e-6,
    },
    {
      module: "geo",
      name: "path-commands",
      subject: () => geo.vizGeoPath(DEMO_POLYGON, geo.vizGeoProjection("equirectangular")),
      oracle: () => {
        const recorder = mark.vizPathRecorder();
        recordD3Path(d3Geo.geoPath(d3Geo.geoEquirectangular().precision(0)), recorder)(DEMO_POLYGON as never);
  
      return recorder.commands;
      },
      tolerance: 1e-9,
    },
    {
      module: "geo",
      name: "fit-extent",
      subject: () => {
        const fitted = geo.vizGeoFitExtent(
          geo.vizGeoProjection("equirectangular"),
          [
            [0, 0],
            [200, 150],
          ],
          DEMO_POLYGON,
          geo.rawEquirectangular,
        );
        return { scale: fitted.scale(), translate: fitted.translate() };
      },
      oracle: () => {
        const fitted = d3Geo.geoEquirectangular().fitExtent(
          [
            [0, 0],
            [200, 150],
          ],
          DEMO_POLYGON as never,
        );
        return { scale: fitted.scale(), translate: fitted.translate() };
      },
      tolerance: 1e-6,
    },
    {
      module: "geo",
      name: "great-circle-distance",
      subject: () => [geo.vizGeoDistance([0, 0], [0, 90]), geo.vizGeoDistance([-73.5, 40.7], [139.7, 35.7]), geo.vizGeoDistance([10, 10], [10, 10])],
      oracle: () => [d3Geo.geoDistance([0, 0], [0, 90]), d3Geo.geoDistance([-73.5, 40.7], [139.7, 35.7]), d3Geo.geoDistance([10, 10], [10, 10])],
      tolerance: 1e-9,
    },
  ];
}

function spatialPoints(level: Level): [number, number][] {
  const n = level === "quick" ? 24 : level === "long" ? 90 : 240;
  return Array.from({ length: n }, (_, i) => [50 + 40 * Math.cos(i * 2.399963229728653) * Math.sqrt(i + 1) * 0.21, 50 + 40 * Math.sin(i * 2.399963229728653) * Math.sqrt(i + 1) * 0.21] as [number, number]);
}

/** 📍 Differential checks for `📍spatial` against `d3-delaunay`, `d3-hexbin` and `d3-contour`. */
export async function spatialChecks(level: Level): Promise<VizKernelCheck[]> {
  const d3Delaunay = await import("d3-delaunay");
  const d3Hexbin = await import("d3-hexbin");
  const d3Contour = await import("d3-contour");
  const points = spatialPoints(level);
  const raster = { width: 20, height: 16 };
  const values = Array.from({ length: raster.width * raster.height }, (_, i) => {
    const x = (i % raster.width) - raster.width / 2;
    const y = ((i / raster.width) | 0) - raster.height / 2;
    return Math.sin(x / 3) * Math.cos(y / 2) * 10 + x * 0.4 - y * 0.3;
  });
  const normalize = (triangles: readonly (readonly number[])[]) =>
    triangles
      .map((triangle) => [...triangle].sort((a, b) => a - b))
      .sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]! || a[2]! - b[2]!);
  return [
    {
      module: "spatial",
      name: "delaunay-triangles",
      subject: () => normalize(spatial.vizDelaunay(points).triangles),
      oracle: () => {
        const delaunay = d3Delaunay.Delaunay.from(points);
        const out: number[][] = [];
        for (let i = 0; i < delaunay.triangles.length; i += 3) out.push([delaunay.triangles[i]!, delaunay.triangles[i + 1]!, delaunay.triangles[i + 2]!]);
        return normalize(out);
      },
    },
    {
      module: "spatial",
      name: "convex-hull",
      subject: () => {
        const hull = spatial.vizConvexHull(points);
        return [...hull].sort((a, b) => a - b);
      },
      oracle: () => [...d3Delaunay.Delaunay.from(points).hull].sort((a, b) => a - b),
    },
    {
      module: "spatial",
      name: "voronoi-cell-areas",
      subject: () => {
        const voronoi = spatial.vizVoronoi(points, [0, 0, 100, 100]);
        return voronoi.cells.map((cell) => Math.abs(spatial.vizPolygonArea(cell)));
      },
      oracle: () => {
        const voronoi = d3Delaunay.Delaunay.from(points).voronoi([0, 0, 100, 100]);
        return points.map((_, i) => {
          const cell = voronoi.cellPolygon(i);
          if (cell === null) return 0;
          return Math.abs(spatial.vizPolygonArea(cell.slice(0, -1) as never));
        });
      },
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "circumcenters",
      subject: () => {
        const triangulation = spatial.vizDelaunay(points);
        return normalize(triangulation.triangles).map((triangle) => spatial.vizCircumcenter(points[triangle[0]!]!, points[triangle[1]!]!, points[triangle[2]!]!));
      },
      oracle: () => {
        const delaunay = d3Delaunay.Delaunay.from(points);
        const out: number[][] = [];
        for (let i = 0; i < delaunay.triangles.length; i += 3) out.push([delaunay.triangles[i]!, delaunay.triangles[i + 1]!, delaunay.triangles[i + 2]!]);
        return normalize(out).map((triangle) => spatial.vizCircumcenter(points[triangle[0]!]!, points[triangle[1]!]!, points[triangle[2]!]!));
      },
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "hexbin",
      subject: () => {
        const bins = spatial.vizHexbin(points, { radius: 8 });
        return bins.map((bin) => [bin.x, bin.y, bin.values.length]).sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]!);
      },
      oracle: () => {
        const bins = d3Hexbin.hexbin().radius(8)(points as never) as unknown as ({ x: number; y: number; length: number })[];
        return bins.map((bin) => [bin.x, bin.y, bin.length]).sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]!);
      },
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "hexagon-outline",
      subject: () => spatial.vizHexagon(8),
      oracle: () =>
        Array.from({ length: 6 }, (_, i) => {
          const angle = (i * Math.PI) / 3;
          return [8 * Math.sin(angle), -8 * Math.cos(angle)];
        }),
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "contours-fixed-thresholds",
      subject: () => spatial.vizContours(values, [raster.width, raster.height], [-8, -3, 0, 3, 8]).map((contour) => ({ value: contour.value, coordinates: contour.coordinates })),
      oracle: () =>
        d3Contour
          .contours()
          .size([raster.width, raster.height])
          .thresholds([-8, -3, 0, 3, 8])(values)
          .map((contour) => ({ value: contour.value, coordinates: contour.coordinates })),
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "contours-default-thresholds",
      subject: () => spatial.vizContours(values, [raster.width, raster.height]).map((contour) => contour.value),
      oracle: () =>
        d3Contour
          .contours()
          .size([raster.width, raster.height])(values)
          .map((contour) => contour.value),
      tolerance: 1e-9,
    },
    {
      module: "spatial",
      name: "density-monotone-and-normalised",
      subject: () => {
        const inside = spatialPoints("quick").filter((point) => point[0] > 30 && point[0] < 70 && point[1] > 30 && point[1] < 70);
        const grid = spatial.vizDensity2d(inside, { extent: [0, 0, 100, 100], cellSize: 2.5, bandwidth: 6 });
        const total = grid.values.reduce((sum, value) => sum + value, 0) * grid.cellSize * grid.cellSize;
        const peak = grid.values.indexOf(Math.max(...grid.values));
        const contours = spatial.vizDensityContours(grid, 4).length;
        return { integral: Math.round(total * 100) / 100, positive: grid.values.every((value) => value >= 0), cells: grid.values.length, peakInside: peak > 0 && peak < grid.values.length - 1, contours: contours > 0 };
      },
      oracle: () => ({ integral: 1, positive: true, cells: 41 * 41, peakInside: true, contours: true }),
      tolerance: 0.02,
    },
  ];
}
