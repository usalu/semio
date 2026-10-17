//! 🍽️ The `PlateDkt` Batoz Discrete Kirchhoff Triangle thin-plate bending element.

use super::*;

// #region 🔖️PlateDkt
/// 🧮️ Per-edge Batoz DKT geometric coefficients `a,b,c,d,e` (cross-checked against Batoz, Bathe & Ho
/// (1980) via the JuliaFEM `FEMPlates.jl` reference implementation — `e_k` is a DISTINCT coefficient
/// from `b_k`, used only in `Hy`'s `βx`-columns; it is NOT the `f_k` appearing in some other DKT
/// write-ups' `Hx`, which this formulation doesn't need).
struct DktEdge {
    a: f64,
    b: f64,
    pub(super) c: f64,
    pub(super) d: f64,
    pub(super) e: f64,
}

fn dkt_edge(pi: [f64; 2], pj: [f64; 2]) -> DktEdge {
    let x_ij = pi[0] - pj[0];
    let y_ij = pi[1] - pj[1];
    let l2 = x_ij * x_ij + y_ij * y_ij;
    DktEdge { a: -x_ij / l2, b: 0.75 * x_ij * y_ij / l2, c: (0.25 * x_ij * x_ij - 0.5 * y_ij * y_ij) / l2, d: -y_ij / l2, e: (0.25 * y_ij * y_ij - 0.5 * x_ij * x_ij) / l2 }
}

/// 🧱️ Bending constitutive matrix `(E t³)/(12(1-ν²)) [[1,ν,0],[ν,1,0],[0,0,(1-ν)/2]]`, shared by
/// `PlateDkt` and (via `d_matrix_plate`) `elements3d::ShellFacet3`'s bending part.
pub fn d_matrix_plate(e: f64, nu: f64, thickness: f64) -> MatD {
    let factor = e * thickness.powi(3) / (12.0 * (1.0 - nu * nu));
    let mut d = MatD::zeros(3, 3);
    d.set(0, 0, factor);
    d.set(0, 1, factor * nu);
    d.set(1, 0, factor * nu);
    d.set(1, 1, factor);
    d.set(2, 2, factor * (1.0 - nu) / 2.0);
    d
}

/// 🧮️ Batoz DKT curvature-displacement B-matrix (3x9) at parametric `(xi, eta)` on a flat triangle
/// with physical `coords`. Dof order `[w1,Rx1,Ry1,w2,Rx2,Ry2,w3,Rx3,Ry3]`, where `Rx = ∂w/∂y` and
/// `Ry = -∂w/∂x` (the physically standard rotation-about-local-axis convention — see the module docs
/// on `PlateDkt`). Builds Batoz's `Hx`/`Hy` shape combinations over the standard mid-edge quadratic
/// "bubble" functions `N4=4L2L3, N5=4L3L1, N6=4L1L2` and the QUADRATIC Tri6 corner functions
/// `Ni=Li(2Li-1)` (cross-checked against the JuliaFEM `FEMPlates.jl` reference implementation of
/// Batoz, Bathe & Ho 1980). Batoz's own `Hx`/`Hy` columns land directly on our `(Rx,Ry)` dof slots
/// with NO permutation — empirically confirmed via the patch test below — but the curvature signs
/// `κx=-∂Hx/∂x, κy=-∂Hy/∂y, κxy=-(∂Hx/∂y+∂Hy/∂x)` carry an overall minus relative to the raw `Hx`/`Hy`
/// derivatives (this formulation's `Hx`/`Hy` represent the NEGATIVE of the physical rotation fields).
/// Shared by `PlateDkt` and `elements3d::ShellFacet3`.
pub fn dkt_b_matrix(coords: &[[f64; 2]; 3], xi: f64, eta: f64) -> MatD {
    let edge4 = dkt_edge(coords[1], coords[2]);
    let edge5 = dkt_edge(coords[2], coords[0]);
    let edge6 = dkt_edge(coords[0], coords[1]);

    let (_, dn_param) = shape_tri3(xi, eta);
    let (_, _, d_n_xy) = jacobian_2d(coords, &dn_param);
    let gx = [d_n_xy[0][0], d_n_xy[1][0], d_n_xy[2][0]];
    let gy = [d_n_xy[0][1], d_n_xy[1][1], d_n_xy[2][1]];

    let (l1, l2v, l3) = (1.0 - xi - eta, xi, eta);
    let dn4dx = 4.0 * (gx[1] * l3 + l2v * gx[2]);
    let dn4dy = 4.0 * (gy[1] * l3 + l2v * gy[2]);
    let dn5dx = 4.0 * (gx[2] * l1 + l3 * gx[0]);
    let dn5dy = 4.0 * (gy[2] * l1 + l3 * gy[0]);
    let dn6dx = 4.0 * (gx[0] * l2v + l1 * gx[1]);
    let dn6dy = 4.0 * (gy[0] * l2v + l1 * gy[1]);

    // The Hx3/Hx6/Hx9 and Hy2/Hy5/Hy8 "direct" terms use the QUADRATIC Tri6 corner shape functions
    // `Ni = Li*(2Li-1)` (matching `formulation::shape_tri6`'s convention), NOT the linear area
    // coordinates `Li` themselves — `dNi/dx = (4Li-1) * dLi/dx`.
    let dn1qdx = (4.0 * l1 - 1.0) * gx[0];
    let dn1qdy = (4.0 * l1 - 1.0) * gy[0];
    let dn2qdx = (4.0 * l2v - 1.0) * gx[1];
    let dn2qdy = (4.0 * l2v - 1.0) * gy[1];
    let dn3qdx = (4.0 * l3 - 1.0) * gx[2];
    let dn3qdy = (4.0 * l3 - 1.0) * gy[2];

    // Batoz-order (w1,βx1,βy1,w2,βx2,βy2,w3,βx3,βy3) partial derivatives of the Hx/Hy shape combinations.
    let dhx_dx = [
        1.5 * (edge6.a * dn6dx - edge5.a * dn5dx),
        edge5.b * dn5dx + edge6.b * dn6dx,
        dn1qdx - edge5.c * dn5dx - edge6.c * dn6dx,
        1.5 * (edge4.a * dn4dx - edge6.a * dn6dx),
        edge6.b * dn6dx + edge4.b * dn4dx,
        dn2qdx - edge6.c * dn6dx - edge4.c * dn4dx,
        1.5 * (edge5.a * dn5dx - edge4.a * dn4dx),
        edge4.b * dn4dx + edge5.b * dn5dx,
        dn3qdx - edge4.c * dn4dx - edge5.c * dn5dx,
    ];
    let dhx_dy = [
        1.5 * (edge6.a * dn6dy - edge5.a * dn5dy),
        edge5.b * dn5dy + edge6.b * dn6dy,
        dn1qdy - edge5.c * dn5dy - edge6.c * dn6dy,
        1.5 * (edge4.a * dn4dy - edge6.a * dn6dy),
        edge6.b * dn6dy + edge4.b * dn4dy,
        dn2qdy - edge6.c * dn6dy - edge4.c * dn4dy,
        1.5 * (edge5.a * dn5dy - edge4.a * dn4dy),
        edge4.b * dn4dy + edge5.b * dn5dy,
        dn3qdy - edge4.c * dn4dy - edge5.c * dn5dy,
    ];
    let dhy_dy = [
        1.5 * (edge6.d * dn6dy - edge5.d * dn5dy),
        -dn1qdy + edge5.e * dn5dy + edge6.e * dn6dy,
        -edge5.b * dn5dy - edge6.b * dn6dy,
        1.5 * (edge4.d * dn4dy - edge6.d * dn6dy),
        -dn2qdy + edge4.e * dn4dy + edge6.e * dn6dy,
        -edge4.b * dn4dy - edge6.b * dn6dy,
        1.5 * (edge5.d * dn5dy - edge4.d * dn4dy),
        -dn3qdy + edge4.e * dn4dy + edge5.e * dn5dy,
        -edge4.b * dn4dy - edge5.b * dn5dy,
    ];
    let dhy_dx = [
        1.5 * (edge6.d * dn6dx - edge5.d * dn5dx),
        -dn1qdx + edge5.e * dn5dx + edge6.e * dn6dx,
        -edge5.b * dn5dx - edge6.b * dn6dx,
        1.5 * (edge4.d * dn4dx - edge6.d * dn6dx),
        -dn2qdx + edge4.e * dn4dx + edge6.e * dn6dx,
        -edge4.b * dn4dx - edge6.b * dn6dx,
        1.5 * (edge5.d * dn5dx - edge4.d * dn4dx),
        -dn3qdx + edge4.e * dn4dx + edge5.e * dn5dx,
        -edge4.b * dn4dx - edge5.b * dn5dx,
    ];

    // Batoz's Hx/Hy columns land directly on our (w,Rx,Ry) triple with no permutation, but with an
    // overall sign flip (see the doc comment above): κx=-∂Hx/∂x, κy=-∂Hy/∂y, κxy=-(∂Hx/∂y+∂Hy/∂x).
    let mut b = MatD::zeros(3, 9);
    for i in 0..3 {
        let (bw, bbx, bby) = (3 * i, 3 * i + 1, 3 * i + 2);
        b.set(0, bw, -dhx_dx[bw]);
        b.set(0, bbx, -dhx_dx[bbx]);
        b.set(0, bby, -dhx_dx[bby]);
        b.set(1, bw, -dhy_dy[bw]);
        b.set(1, bbx, -dhy_dy[bbx]);
        b.set(1, bby, -dhy_dy[bby]);
        b.set(2, bw, -dhx_dy[bw] - dhy_dx[bw]);
        b.set(2, bbx, -dhx_dy[bbx] - dhy_dx[bbx]);
        b.set(2, bby, -dhx_dy[bby] - dhy_dx[bby]);
    }
    b
}

/// 🧊️ Batoz Discrete Kirchhoff Triangle (DKT) — 3-node thin-plate bending element, DOFs `[Tz,Rx,Ry]`
/// per node (`Rx = ∂w/∂y`, `Ry = -∂w/∂x`, the physically standard rotation-about-axis convention: a
/// positive rotation about the local x-axis tilts the plate normal the same way a positive `∂w/∂y`
/// slope does). 3-point Gauss-tri integration of the (non-constant, unlike CST) curvature field. See
/// Batoz, Bathe & Ho (1980) "A study of three-node triangular plate bending elements".
///
/// 🌀️ Reports NO `geometric_stiffness` (stays the trait default `None`) — a pure bending element
/// carries no membrane stress state to destabilize its own transverse deflection; plate/shell
/// buckling under in-plane compression needs the membrane-bending coupling `elements3d::ShellFacet3`
/// provides, not `PlateDkt` alone.
pub struct PlateDkt {
    pub id: String,
    pub nodes: [String; 3],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub density: f64,
}

impl PlateDkt {
    fn coords(ctx: &ElementContext) -> [[f64; 2]; 3] {
        [[ctx.positions[0][0], ctx.positions[0][1]], [ctx.positions[1][0], ctx.positions[1][1]], [ctx.positions[2][0], ctx.positions[2][1]]]
    }
}

impl Element for PlateDkt {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        &[Dof::Tz, Dof::Rx, Dof::Ry]
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let coords = Self::coords(ctx);
        let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
        let d = d_matrix_plate(self.e, self.nu, self.thickness);
        let mut ke = MatD::zeros(9, 9);
        for (xi, eta, w) in gauss_tri(3) {
            let b = dkt_b_matrix(&coords, xi, eta);
            ke.add_triple_product(&b, &d, w * det_j);
        }
        ke
    }

    fn recover(&self, ctx: &ElementContext, u_local: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let coords = Self::coords(ctx);
        let d = d_matrix_plate(self.e, self.nu, self.thickness);
        let gauss = gauss_tri(3)
            .into_iter()
            .map(|(xi, eta, _)| {
                let b = dkt_b_matrix(&coords, xi, eta);
                let kappa = b.mul_vec(u_local);
                let m = d.mul_vec(&kappa);
                PlateMoments { mx: m.get(0), my: m.get(1), mxy: m.get(2) }
            })
            .collect();
        ElementResult::Plate { gauss }
    }

    /// 🏋️ Lumped translational mass `ρtA/3` on each node's `Tz` only — zero rotary inertia. DKT has
    /// no independent transverse-displacement interpolation to derive a consistent mass from (its
    /// curvature field comes from `w`+rotations jointly), so lumping the plate's own weight evenly
    /// across its 3 corners is the standard practical simplification (Cook, Malkus, Plesha & Witt).
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let coords = Self::coords(ctx);
        let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
        let area = 0.5 * det_j;
        let share = self.density * self.thickness * area / 3.0;
        let mut m = MatD::zeros(9, 9);
        for i in 0..3 {
            m.set(3 * i, 3 * i, share);
        }
        Some(m)
    }
}
// #endregion 🔖️PlateDkt
