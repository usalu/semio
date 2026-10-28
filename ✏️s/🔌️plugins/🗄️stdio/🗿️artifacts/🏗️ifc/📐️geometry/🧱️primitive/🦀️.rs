//! 🧱️ Bounded primitive coordinates shared by immediate and retained projection.
pub const BLOCK_TRIANGLES:[[u32;3];12]=[[0,2,1],[0,3,2],[4,5,6],[4,6,7],[0,1,5],[0,5,4],[1,2,6],[1,6,5],[2,3,7],[2,7,6],[3,0,4],[3,4,7]];
pub fn block_point(lengths:[f64;3],index:usize)->[f64;3]{[if (index^(index>>1))&1==0{0.}else{lengths[0]},if index&2==0{0.}else{lengths[1]},if index&4==0{0.}else{lengths[2]}]}
pub fn rectangle_point(x:f64,y:f64,index:usize)->[f64;3]{[(if (index^(index>>1))&1==0{-0.5}else{0.5})*x,(if index&2==0{-0.5}else{0.5})*y,0.]}
