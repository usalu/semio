//! 📐️ Bounded placement recipe shared by native solid pattern publication.

use semio_framework_3d_brep_vector::{Vec3,matrix::Affine3};
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_value::{ValueError,ValueRefusalKind};

const MAX_MEMBERS: usize = 4096;

#[derive(Clone, Copy)]
enum Layout {
    Grid { row: Vec3, column: Vec3, columns: usize },
    Circular { axis: Vec3 },
}

/// 🧭️ A bounded procedural pattern in authored member order.
#[derive(Clone, Copy)]
pub struct SolidPatternRecipe {
    layout: Layout,
    count: usize,
}

impl SolidPatternRecipe {
    /// 📏️ Translates by successive multiples of the supplied direction and spacing.
    pub fn linear(direction: [f64; 3], spacing: f64, count: usize) -> Result<Self, ValueError> {
        Self::grid(direction, [0.0; 3], spacing, 0.0, count, 1)
    }

    /// 🔄️ Rotates each successive instance around the supplied world axis.
    pub fn circular(axis: [f64; 3], count: usize) -> Result<Self, ValueError> {
        let norm = axis[0].hypot(axis[1]).hypot(axis[2]);
        if !norm.is_finite() || norm <= Tol::DEFAULT.value() { return Err(invalid()); }
        Self::checked(Layout::Circular { axis: Vec3::from_array(axis.map(|v| v / norm)) }, count.max(1))
    }

    /// ▦️ Translates in row-major order along two independent authored directions.
    pub fn grid(x: [f64; 3], y: [f64; 3], sx: f64, sy: f64, nx: usize, ny: usize) -> Result<Self, ValueError> {
        let row = Vec3::from_array(x) * sx;
        let column = Vec3::from_array(y) * sy;
        if !row.to_array().into_iter().chain(column.to_array()).all(f64::is_finite) { return Err(invalid()); }
        let columns = ny.max(1);
        Self::checked(Layout::Grid { row, column, columns }, nx.max(1).saturating_mul(columns))
    }

    fn checked(layout: Layout, count: usize) -> Result<Self, ValueError> {
        if count > MAX_MEMBERS { return Err(invalid()); }
        Ok(Self { layout, count })
    }

    pub fn count(self)->usize { self.count }

    /// 🧭️ Borrows the exact affine placement of a noninitial member.
    pub fn transform(self,index:usize)->Result<Affine3,ValueError>{
        let map = match self.layout {
            Layout::Grid { row, column, columns } => {
                let offset = row * (index / columns) as f64 + column * (index % columns) as f64;
                if !offset.to_array().into_iter().all(f64::is_finite) { return Err(invalid()); }
                Affine3::translation(offset)
            }
            Layout::Circular { axis } => Affine3::rotation_axis_angle(axis, std::f64::consts::TAU * index as f64 / self.count as f64),
        };
        Ok(map)
    }
}

fn invalid()->ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "pattern requires finite parameters and at most 4096 instances")
}
