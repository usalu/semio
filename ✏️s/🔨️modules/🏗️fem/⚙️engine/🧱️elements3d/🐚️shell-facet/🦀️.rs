//! 🐚️ The `ShellFacet3` flat facet shell: CST membrane, DKT bending and drilling stabilization.

use super::*;

// #region 🔖️ShellFacet3
/// 🧭️ Local in-plane axes for a flat triangular facet, built directly from 3 non-collinear 3D
/// points (no roll angle or reference-vector edge case needed — 3 points unambiguously define a
/// plane, unlike `Frame3::local_system`'s 1D-member case): local x along edge `p0->p1`, local z the
/// facet normal (`cross(p1-p0, p2-p0)`, right-hand rule), local y completing the right-handed frame.
fn shell_local_axes(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3]) -> Mat3d {
    let d1 = vec3d_sub(p1, p0);
    let d2 = vec3d_sub(p2, p0);
    let local_x = vec3d_normalize(d1);
    let local_z = vec3d_normalize(vec3d_cross(d1, d2));
    let local_y = vec3d_cross(local_z, local_x);
    Mat3d::from_axes(local_x, local_y, local_z)
}

/// 🧭️ 18x18 global<->local block-diagonal rotation `T` — six `R^T` 3x3 blocks (one per node's
/// translation triple `[Tx,Ty,Tz]`, one per node's rotation triple `[Rx,Ry,Rz]`), the same pattern
/// `Frame3::local_system` uses for its 12x12 `T`, extended to 3 nodes x 2 triples.
fn shell_transform(r: &Mat3d) -> MatD {
    let rt = r.transpose();
    let mut t = MatD::zeros(18, 18);
    for offset in [0usize, 3, 6, 9, 12, 15] {
        for row in 0..3 {
            for col in 0..3 {
                t.set(offset + row, offset + col, rt.cols[col][row]);
            }
        }
    }
    t
}

/// 🐚️ Flat facet shell — 3-node, 6-DOF-per-node (`[Tx,Ty,Tz,Rx,Ry,Rz]`) element combining an in-plane
/// `Tri3Cst`-style CST membrane, `PlateDkt`-style DKT bending, and a small artificial "drilling"
/// stiffness on the local `Rz` (in-plane rotation) DOF — flat shells have no natural stiffness
/// resisting drilling rotation, so a small diagonal stabilization avoids a singular system where
/// coplanar/near-coplanar facets meet. Membrane and bending are exactly decoupled at this (flat,
/// linear) scope, so the local 18x18 stiffness is block-diagonal by construction.
pub struct ShellFacet3 {
    pub id: String,
    pub nodes: [String; 3],
    pub e: f64,
    pub nu: f64,
    pub thickness: f64,
    pub density: f64,
}

/// 🎯️ Small dimensionless drilling-stabilization factor — standard "just enough to avoid
/// singularity, small enough not to distort real behavior" scaling on `k_drill = α·E·t·Area`.
const SHELL_DRILL_ALPHA: f64 = 1e-3;

impl ShellFacet3 {
    /// 🧭️ Local in-plane 2D triangle coordinates (`p0_local=(0,0)`, `p1_local` on the local x-axis,
    /// `p2_local` completing the triangle) plus the local-axes rotation `Mat3d` shared by
    /// `local_stiffness`'s membrane/bending quadrature and `shell_transform`'s `T`.
    fn local_coords(ctx: &ElementContext) -> ([[f64; 2]; 3], Mat3d) {
        let (p0, p1, p2) = (ctx.positions[0], ctx.positions[1], ctx.positions[2]);
        let r = shell_local_axes(p0, p1, p2);
        let (local_x, local_y) = (r.cols[0], r.cols[1]);
        let d1 = vec3d_sub(p1, p0);
        let d2 = vec3d_sub(p2, p0);
        let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let coords = [[0.0, 0.0], [dot(d1, local_x), 0.0], [dot(d2, local_x), dot(d2, local_y)]];
        (coords, r)
    }

    /// 🧮️ Local 18x18 stiffness, dof order `[Tx1,Ty1,Tz1,Rx1,Ry1,Rz1, ... x3]`: 6x6 CST membrane
    /// block at each node's `(Tx,Ty)` indices, 9x9 DKT bending block at each node's `(Tz,Rx,Ry)`
    /// indices, drilling diagonal at each node's `Rz` index — all cross-terms zero by construction.
    pub(super) fn local_stiffness(&self, coords: &[[f64; 2]; 3]) -> MatD {
        let mut k = MatD::zeros(18, 18);

        let d_mem = d_matrix_plane_stress(self.e, self.nu);
        let mut k_mem = MatD::zeros(6, 6);
        for (xi, eta, w) in gauss_tri(1) {
            let (_, dn) = shape_tri3(xi, eta);
            let (_, det_j, d_n_xy) = jacobian_2d(coords, &dn);
            let b = b_matrix_plane(&d_n_xy);
            k_mem.add_triple_product(&b, &d_mem, w * det_j * self.thickness);
        }
        let mem_idx = [0usize, 1, 6, 7, 12, 13];
        for (i, &gi) in mem_idx.iter().enumerate() {
            for (j, &gj) in mem_idx.iter().enumerate() {
                k.set(gi, gj, k_mem.get(i, j));
            }
        }

        let (_, det_j, _) = jacobian_2d(coords, &shape_tri3(0.0, 0.0).1);
        let d_bend = semio_s_fem_elements2d::d_matrix_plate(self.e, self.nu, self.thickness);
        let mut k_bend = MatD::zeros(9, 9);
        for (xi, eta, w) in gauss_tri(3) {
            let b = semio_s_fem_elements2d::dkt_b_matrix(coords, xi, eta);
            k_bend.add_triple_product(&b, &d_bend, w * det_j);
        }
        let bend_idx = [2usize, 3, 4, 8, 9, 10, 14, 15, 16];
        for (i, &gi) in bend_idx.iter().enumerate() {
            for (j, &gj) in bend_idx.iter().enumerate() {
                k.set(gi, gj, k_bend.get(i, j));
            }
        }

        let area = 0.5 * det_j;
        let k_drill = SHELL_DRILL_ALPHA * self.e * self.thickness * area;
        for i in 0..3 {
            k.set(6 * i + 5, 6 * i + 5, k_drill);
        }

        k
    }
}

impl Element for ShellFacet3 {
    fn id(&self) -> &str {
        &self.id
    }

    fn node_ids(&self) -> Vec<String> {
        self.nodes.to_vec()
    }

    fn dofs_per_node(&self) -> &[Dof] {
        const DOFS: [Dof; 6] = [Dof::Tx, Dof::Ty, Dof::Tz, Dof::Rx, Dof::Ry, Dof::Rz];
        &DOFS
    }

    fn stiffness_global(&self, ctx: &ElementContext) -> MatD {
        let (coords, r) = Self::local_coords(ctx);
        let t = shell_transform(&r);
        let k_local = self.local_stiffness(&coords);
        t.transpose().matmul(&k_local).matmul(&t)
    }

    fn recover(&self, ctx: &ElementContext, u_elem: &VecD, _udl: Option<&MemberUdl>) -> ElementResult {
        let (coords, r) = Self::local_coords(ctx);
        let t = shell_transform(&r);
        let u_loc = t.mul_vec(u_elem);

        let mem_idx = [0usize, 1, 6, 7, 12, 13];
        let u_mem = VecD::from_vec(mem_idx.iter().map(|&i| u_loc.get(i)).collect());
        let d_mem = d_matrix_plane_stress(self.e, self.nu);
        let (_, dn) = shape_tri3(1.0 / 3.0, 1.0 / 3.0);
        let (_, _, d_n_xy) = jacobian_2d(&coords, &dn);
        let b_mem = b_matrix_plane(&d_n_xy);
        let eps = b_mem.mul_vec(&u_mem);
        let sigma = d_mem.mul_vec(&eps);
        let (nxx, nyy, nxy) = (sigma.get(0) * self.thickness, sigma.get(1) * self.thickness, sigma.get(2) * self.thickness);

        let bend_idx = [2usize, 3, 4, 8, 9, 10, 14, 15, 16];
        let u_bend = VecD::from_vec(bend_idx.iter().map(|&i| u_loc.get(i)).collect());
        let d_bend = semio_s_fem_elements2d::d_matrix_plate(self.e, self.nu, self.thickness);
        let b_bend = semio_s_fem_elements2d::dkt_b_matrix(&coords, 1.0 / 3.0, 1.0 / 3.0);
        let kappa = b_bend.mul_vec(&u_bend);
        let m = d_bend.mul_vec(&kappa);
        let (mxx, myy, mxy) = (m.get(0), m.get(1), m.get(2));

        let t_th = self.thickness;
        let surface = |sign: f64| {
            let sxx = nxx / t_th + sign * 6.0 * mxx / (t_th * t_th);
            let syy = nyy / t_th + sign * 6.0 * myy / (t_th * t_th);
            let sxy = nxy / t_th + sign * 6.0 * mxy / (t_th * t_th);
            (sxx * sxx - sxx * syy + syy * syy + 3.0 * sxy * sxy).sqrt()
        };
        let von_mises_top = surface(1.0);
        let von_mises_bottom = surface(-1.0);

        ElementResult::Shell { gauss: vec![ShellState { nxx, nyy, nxy, mxx, myy, mxy, von_mises_top, von_mises_bottom }] }
    }

    /// 🏋️ Lumped translational mass `ρtA/3` on each node's `[Tx,Ty,Tz]` — diagonal and isotropic
    /// (equal in all 3 local translation directions), so it needs no local->global rotation, unlike
    /// `local_stiffness`. Zero rotational inertia, same lumping rationale as `PlateDkt::mass`.
    fn mass(&self, ctx: &ElementContext) -> Option<MatD> {
        let (coords, _) = Self::local_coords(ctx);
        let (_, det_j, _) = jacobian_2d(&coords, &shape_tri3(0.0, 0.0).1);
        let area = 0.5 * det_j;
        let share = self.density * self.thickness * area / 3.0;
        let mut m = MatD::zeros(18, 18);
        for i in 0..3 {
            for a in 0..3 {
                m.set(6 * i + a, 6 * i + a, share);
            }
        }
        Some(m)
    }

    /// 🌀️ Geometric stiffness from the facet's own (constant) CST membrane forces `Nxx,Nyy,Nxy`
    /// acting on the LINEAR CST-interpolated out-of-plane `w` gradient (the standard flat-facet
    /// simplification — the DKT bending field's rotation-driven curvature correction is neglected
    /// for this coupling, following common practice for flat shell buckling), local `Tz` dof per
    /// node using the SAME constant gradient `local_stiffness`'s membrane block computes.
    fn geometric_stiffness(&self, ctx: &ElementContext, u_element: &VecD) -> Option<MatD> {
        let (coords, r) = Self::local_coords(ctx);
        let t = shell_transform(&r);
        let u_loc = t.mul_vec(u_element);

        let mem_idx = [0usize, 1, 6, 7, 12, 13];
        let u_mem = VecD::from_vec(mem_idx.iter().map(|&i| u_loc.get(i)).collect());
        let d_mem = d_matrix_plane_stress(self.e, self.nu);
        let (_, dn) = shape_tri3(1.0 / 3.0, 1.0 / 3.0);
        let (_, det_j, d_n_xy) = jacobian_2d(&coords, &dn);
        let b_mem = b_matrix_plane(&d_n_xy);
        let eps = b_mem.mul_vec(&u_mem);
        let sigma = d_mem.mul_vec(&eps);
        let (nxx, nyy, nxy) = (sigma.get(0) * self.thickness, sigma.get(1) * self.thickness, sigma.get(2) * self.thickness);

        let area = 0.5 * det_j;
        let w_idx = [2usize, 8, 14];
        let mut kg_local = MatD::zeros(18, 18);
        for i in 0..3 {
            let (gix, giy) = (d_n_xy[i][0], d_n_xy[i][1]);
            for j in 0..3 {
                let (gjx, gjy) = (d_n_xy[j][0], d_n_xy[j][1]);
                let s = gix * nxx * gjx + gix * nxy * gjy + giy * nxy * gjx + giy * nyy * gjy;
                kg_local.add_at(w_idx[i], w_idx[j], s * area);
            }
        }
        Some(t.transpose().matmul(&kg_local).matmul(&t))
    }
}
// #endregion 🔖️ShellFacet3
