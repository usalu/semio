//! ⚙️ Layout options of the board schema: the force graph, hierarchical tree and redraw layout options with their defaults. See [`super`].

/// ⚙️ Force-directed layout parameters for normal undirected node-id graphs.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct ForceGraphLayoutOptions {
    #[value(default = "default_iterations")]
    pub iterations: u32,
    #[value(default = "default_ideal_edge_length")]
    pub ideal_edge_length: f64,
    #[value(default = "default_repulsion_strength")]
    pub repulsion_strength: f64,
    #[value(default = "default_spring_strength")]
    pub spring_strength: f64,
    #[value(default = "default_gravity")]
    pub gravity: f64,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
    #[value(default = "default_time_step")]
    pub time_step: f64,
    #[value(default = "default_velocity_damping")]
    pub velocity_damping: f64,
    #[value(default = "default_max_speed")]
    pub max_speed: f64,
    #[value(default = "default_random_seed")]
    pub random_seed: u64,
    #[value(default = "default_barnes_hut_theta")]
    pub barnes_hut_theta: f64,
    #[value(default = "default_pairwise_repulsion_max_bodies")]
    pub pairwise_repulsion_max_bodies: u32,
    #[value(default)]
    pub locked_node_ids: Vec<String>,
}

fn default_iterations() -> u32 {
    420
}

fn default_ideal_edge_length() -> f64 {
    140.0
}

fn default_repulsion_strength() -> f64 {
    6500.0
}

fn default_spring_strength() -> f64 {
    0.028
}

fn default_gravity() -> f64 {
    0.0
}

fn default_time_step() -> f64 {
    0.85
}

fn default_velocity_damping() -> f64 {
    0.88
}

fn default_max_speed() -> f64 {
    48.0
}

fn default_random_seed() -> u64 {
    0x5eedfaced0
}

fn default_barnes_hut_theta() -> f64 {
    0.78
}

fn default_pairwise_repulsion_max_bodies() -> u32 {
    56
}

impl Default for ForceGraphLayoutOptions {
    fn default() -> Self {
        Self {
            iterations: default_iterations(),
            ideal_edge_length: default_ideal_edge_length(),
            repulsion_strength: default_repulsion_strength(),
            spring_strength: default_spring_strength(),
            gravity: default_gravity(),
            center_x: None,
            center_y: None,
            time_step: default_time_step(),
            velocity_damping: default_velocity_damping(),
            max_speed: default_max_speed(),
            random_seed: default_random_seed(),
            barnes_hut_theta: default_barnes_hut_theta(),
            pairwise_repulsion_max_bodies: default_pairwise_repulsion_max_bodies(),
            locked_node_ids: Vec::new(),
        }
    }
}

/// 🌳️ Buchheim tidy-tree knobs: rank gap, sibling breadth, growth-axis string, optional world anchor for the laid subtree.
#[derive(Clone, Debug, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct HierarchicalTreeLayoutOptions {
    #[value(default = "default_tree_layer_spacing")]
    pub layer_spacing: f64,
    #[value(default = "default_tree_sibling_gap")]
    pub sibling_gap: f64,
    #[value(default = "default_direction")]
    pub direction: String,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
    /// 📌️ Node ids whose incoming snapshot centers are kept; Buchheim still runs for placement of unlocked nodes.
    #[value(default)]
    pub locked_node_ids: Vec<String>,
}

pub(crate) fn default_tree_layer_spacing() -> f64 {
    120.0
}

pub(crate) fn default_tree_sibling_gap() -> f64 {
    28.0
}

pub(crate) fn default_direction() -> String {
    "downwards".into()
}

impl Default for HierarchicalTreeLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_tree_layer_spacing(), sibling_gap: default_tree_sibling_gap(), direction: default_direction(), center_x: None, center_y: None, locked_node_ids: Vec::new() }
    }
}

#[derive(Debug, Clone, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase",deny_unknown_fields)]
pub struct RedrawLayoutOptions {
    pub mode: String,
    #[value(default)]
    pub center_x: Option<f64>,
    #[value(default)]
    pub center_y: Option<f64>,
    #[value(default)]
    pub random_seed: Option<u64>,
    #[value(default)]
    pub redraw_handles_after: bool,
    #[value(default)]
    pub locked_node_ids: Vec<String>,
    #[value(default)]
    pub force_graph: Option<ForceGraphLayoutOptions>,
    #[value(default)]
    pub hierarchical_tree: Option<HierarchicalTreeLayoutOptions>,
}
