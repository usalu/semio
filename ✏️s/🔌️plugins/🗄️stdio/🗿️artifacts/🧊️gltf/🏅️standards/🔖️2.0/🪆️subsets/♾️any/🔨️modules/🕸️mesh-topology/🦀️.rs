//! 🕸️ Shared mesh-topology summary contract.

//#region 🕸️Topology
#[derive(Clone, Copy)]
pub struct Topology {
    pub components: u64,
    pub boundary_loops: u64,
    pub chi: i64,
    pub genus: Option<u64>,
    pub manifold: bool,
    pub watertight: bool,
    pub oriented: bool,
}
//#endregion 🕸️Topology
