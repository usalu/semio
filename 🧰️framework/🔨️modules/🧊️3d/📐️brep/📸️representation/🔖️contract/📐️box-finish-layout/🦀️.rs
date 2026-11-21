//! 📐️ Outward face rings for stock with all twelve box edges finished.

pub const VERTICES: usize = 24;
pub const EDGES: usize = 48;
pub const FACES: usize = 26;

pub struct Face {
    pub vertices: [usize; 4],
    pub normal: [f64; 3],
    pub count: usize,
}

pub const fn sign(high: usize) -> f64 {
    if high == 0 { -1.0 } else { 1.0 }
}

pub const fn face(index: usize) -> Face {
    let mut vertices;
    let mut normal = [0.0; 3];
    let count;
    let reverse;
    if index < 6 {
        let (axis, high) = (index / 2, index % 2);
        let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
        let corner = high << axis;
        vertices = [corner, corner | (1 << u), corner | (1 << u) | (1 << v), corner | (1 << v)];
        let mut i = 0;
        while i < 4 {
            vertices[i] = 3 * vertices[i] + axis;
            i += 1;
        }
        normal[axis] = sign(high);
        count = 4;
        reverse = high == 0;
    } else if index < 18 {
        let (axis, sides) = ((index - 6) / 4, (index - 6) % 4);
        let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
        let (u_high, v_high) = (sides % 2, sides / 2);
        let low = (u_high << u) | (v_high << v);
        let high = low | (1 << axis);
        vertices = [3 * low + u, 3 * low + v, 3 * high + v, 3 * high + u];
        normal[u] = sign(u_high);
        normal[v] = sign(v_high);
        count = 4;
        reverse = u_high != v_high;
    } else {
        let corner = index - 18;
        vertices = [3 * corner, 3 * corner + 1, 3 * corner + 2, 0];
        normal = [sign(corner & 1), sign((corner >> 1) & 1), sign(corner >> 2)];
        count = 3;
        reverse = corner.count_ones() % 2 == 0;
    }
    if reverse {
        let last = vertices[count - 1];
        vertices[count - 1] = vertices[1];
        vertices[1] = last;
    }
    Face { vertices, normal, count }
}

pub struct Topology {
    pub edges: [[usize; 2]; EDGES],
    pub members: [[usize; 4]; FACES],
    pub ring_sizes: [usize; FACES],
}

pub const TOPOLOGY: Topology = topology();

const fn topology() -> Topology {
    let mut result = Topology { edges: [[0; 2]; EDGES], members: [[0; 4]; FACES], ring_sizes: [0; FACES] };
    let mut incidence = [[0u8; 2]; EDGES];
    let mut edge_count = 0;
    let mut i = 0;
    while i < FACES {
        let ring = face(i);
        result.ring_sizes[i] = ring.count;
        let mut member = 0;
        while member < ring.count {
            let a = ring.vertices[member];
            let b = ring.vertices[(member + 1) % ring.count];
            let endpoints = if a < b { [a, b] } else { [b, a] };
            let mut edge = 0;
            while edge < edge_count && (result.edges[edge][0] != endpoints[0] || result.edges[edge][1] != endpoints[1]) {
                edge += 1;
            }
            if edge == edge_count {
                result.edges[edge] = endpoints;
                edge_count += 1;
            }
            incidence[edge][(a < b) as usize] += 1;
            result.members[i][member] = edge;
            member += 1;
        }
        i += 1;
    }
    assert!(edge_count == EDGES);
    let mut edge = 0;
    while edge < EDGES {
        assert!(incidence[edge][0] == 1 && incidence[edge][1] == 1);
        edge += 1;
    }
    result
}

