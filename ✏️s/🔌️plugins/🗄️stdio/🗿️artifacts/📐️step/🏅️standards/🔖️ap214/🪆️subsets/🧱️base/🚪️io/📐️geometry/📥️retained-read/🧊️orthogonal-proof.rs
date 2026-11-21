//! 🧊 Exact authoring proof for one simply shelled orthogonal cell arrangement.

use super::*;

const PLANE_SLOTS: usize = 3 * 4 * 3 * 3;

/// 🧮 Bounded evidence for the original vertices, paired edges, and face orientations.
pub(super) struct OrthogonalProof {
    axes: [[f64; 4]; 3],
    count: [usize; 3],
    vertex: [u64; 64],
    edge: [u64; 192],
    edge_use: [i8; 192],
    plane: [i8; PLANE_SLOTS],
    cells: u32,
    ray: usize,
}

impl OrthogonalProof {
    pub(super) fn new() -> Self {
        Self { axes: [[f64::NAN;4];3], count: [0;3], vertex: [0;64], edge: [0;192], edge_use: [0;192], plane: [0;PLANE_SLOTS], cells: 0, ray: 0 }
    }

    pub(super) fn coordinates(&mut self, face: &Face) -> Result<(), ValueError> {
        axis_side(face)?;
        for edge in face.edges {
            for (axis, coord) in edge.point.into_iter().enumerate() {
                let used = self.count[axis];
                if self.axes[axis][..used].contains(&coord) { continue; }
                *self.axes[axis].get_mut(used).ok_or_else(|| unsupported("STEP orthogonal shell exceeds four planes per coordinate"))? = coord;
                self.count[axis] += 1;
            }
        }
        Ok(())
    }

    pub(super) fn prepare(&mut self) -> Result<(), ValueError> {
        for (axis, count) in self.axes.iter_mut().zip(self.count) {
            if count < 2 { return Err(invalid("STEP orthogonal shell extent is degenerate")); }
            axis[..count].sort_unstable_by(f64::total_cmp);
        }
        Ok(())
    }

    pub(super) fn face(&mut self, face: &Face) -> Result<(), ValueError> {
        let (axis, positive) = axis_side(face)?;
        let (i, j) = ((axis+1)%3, (axis+2)%3);
        let mut corners = [[0;3];4];
        for (edge, corner) in face.edges.into_iter().zip(&mut corners) {
            for (a, coordinate) in corner.iter_mut().enumerate() {
                *coordinate = self.axes[a][..self.count[a]].iter().position(|value| *value == edge.point[a]).ok_or_else(incompatible)?;
            }
            let slot = address(*corner,4);
            if self.vertex[slot] != 0 && self.vertex[slot] != edge.from {
                return Err(invalid("STEP orthogonal shell has coincident unshared vertices"));
            }
            self.vertex[slot] = edge.from;
        }
        if corners.iter().enumerate().any(|(m, corner)| corners[..m].contains(corner)) {
            return Err(invalid("STEP orthogonal shell repeats a face corner"));
        }
        let [low_i, low_j] = [i,j].map(|axis| corners.iter().map(|corner| corner[axis]).min().unwrap());
        if corners.iter().map(|corner| corner[i]).max() != Some(low_i + 1)
            || corners.iter().map(|corner| corner[j]).max() != Some(low_j + 1) {
            return Err(incompatible());
        }
        let slot = plane_slot(axis,corners[0][axis],low_i,low_j);
        if self.plane[slot] != 0 { return Err(invalid("STEP orthogonal shell repeats one face")); }
        self.plane[slot] = if positive { 1 } else { -1 };
        for member in 0..4 {
            let (a,b) = (corners[member],corners[(member+1)%4]);
            let along = (0..3).find(|&index| a[index] != b[index]).ok_or_else(incompatible)?;
            if along == axis || a[along].abs_diff(b[along]) != 1 || (0..3).any(|x| x != along && a[x] != b[x]) {
                return Err(incompatible());
            }
            let (low, sign) = if a[along] < b[along] { (a,1) } else { (b,-1) };
            let slot = along*64 + address(low,4);
            let authored = face.edges[member].id;
            if self.edge[slot] == 0 {
                self.edge[slot] = authored;
                self.edge_use[slot] = sign;
            } else if self.edge[slot] == authored && self.edge_use[slot] == -sign {
                self.edge_use[slot] = 0;
            } else {
                return Err(invalid("STEP orthogonal shell has unmatched or unshared trim instances"));
            }
        }
        Ok(())
    }

    pub(super) fn ray(&mut self) -> Result<bool, ValueError> {
        let width = self.count[1]-1;
        let (y,z) = (self.ray % width, self.ray / width);
        let mut active = false;
        for x in 0..self.count[0] {
            match (self.plane[plane_slot(0,x,y,z)],active) {
                (0,_) => {},
                (-1,false) => active = true,
                (1,true) => active = false,
                _ => return Err(invalid("STEP orthogonal shell crossings are not oriented as material")),
            }
            if active && x+1 < self.count[0] { self.cells |= 1 << address([x,y,z],3); }
        }
        if active { return Err(invalid("STEP orthogonal shell does not close its material")); }
        self.ray += 1;
        Ok(self.ray == width * (self.count[2]-1))
    }

    pub(super) fn finish(&self) -> Result<StepSolid, ValueError> {
        if self.cells == 0 || self.edge_use.iter().any(|count| *count != 0) { return Err(incompatible()); }
        for axis in 0..3 {
            let (i,j) = ((axis+1)%3,(axis+2)%3);
            for along in 0..self.count[axis] {
                for a in 0..self.count[i]-1 {
                    for b in 0..self.count[j]-1 {
                        let mut cell = [0;3];
                        (cell[i], cell[j]) = (a,b);
                        let higher = along+1 < self.count[axis] && {
                            cell[axis]=along;
                            self.cells & (1 << address(cell,3)) != 0
                        };
                        let lower = along > 0 && {
                            cell[axis]=along-1;
                            self.cells & (1 << address(cell,3)) != 0
                        };
                        let expected = i8::from(lower) - i8::from(higher);
                        if self.plane[plane_slot(axis,along,a,b)] != expected { return Err(incompatible()); }
                    }
                }
            }
        }
        Ok(StepSolid::Orthogonal { axes: self.axes, cells: self.cells })
    }
}

fn incompatible() -> ValueError { unsupported("STEP orthogonal faces do not certify a closed material grid") }

fn address([x,y,z]: [usize;3], length:usize) -> usize { x + length*(y+length*z) }
fn plane_slot(axis:usize, along:usize, a:usize, b:usize) -> usize { axis*36 + along*9 + a + b*3 }
