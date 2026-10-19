//! 🎲️ Discrete structural causal model: conditional probability tables, factor arithmetic, exact inference and interventional queries. See [`super`].

use super::*;

// #region 🔖️ScmDiscrete
/// 🎲️ Conditional probability table for one node: `probs` is flat, indexed `[node value fastest,
/// then parents[0], parents[1], ...]` — the same mixed-radix layout [`Factor`] uses internally, so
/// [`Factor::from_cpt`] can copy it directly.
#[derive(Clone, Debug)]
pub struct Cpt {
    pub node: usize,
    pub parents: Vec<usize>,
    pub cardinality: usize,
    pub probs: Vec<f64>,
}

/// 🎲️ A CPT-based Bayesian network treated as a discrete SCM: interventions replace a node's CPT
/// with a point mass. Counterfactuals are out of scope for v1 — CPTs fix only the joint
/// distribution, not the functional/twin-network structure a counterfactual query needs.
#[derive(Clone, Debug)]
pub struct DiscreteScm {
    pub dag: CausalDag,
    pub cardinalities: Vec<usize>,
    pub cpts: Vec<Cpt>,
}

impl DiscreteScm {
    /// 📊️ MLE CPTs from categorical columns with additive (Laplace) smoothing `pseudocount` (`0.0` = pure MLE).
    pub fn fit(dag: &CausalDag, data: &semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, pseudocount: f64) -> Result<Self, CausalError> {
        let n = dag.n();
        let columns: Vec<&semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::CategoricalColumn> = (0..n).map(|i| data.categorical(i)).collect::<Result<_, _>>()?;
        let cardinalities: Vec<usize> = (0..n).map(|v| columns[v].n_levels()).collect();
        let mut cpts = Vec::with_capacity(n);
        for v in 0..n {
            let parents = dag.parents(v).to_vec();
            let n_configs = parents.iter().map(|&p| cardinalities[p]).product::<usize>().max(1);
            let mut counts = vec![pseudocount; n_configs * cardinalities[v]];
            for row in 0..data.n_rows() {
                let own = columns[v].codes()[row];
                if own == semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::MISSING_CODE {
                    continue;
                }
                let mut config = 0usize;
                let mut stride = 1usize;
                let mut missing_parent = false;
                for &p in &parents {
                    let code = columns[p].codes()[row];
                    if code == semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::MISSING_CODE {
                        missing_parent = true;
                        break;
                    }
                    config += code as usize * stride;
                    stride *= cardinalities[p];
                }
                if missing_parent {
                    continue;
                }
                counts[config * cardinalities[v] + own as usize] += 1.0;
            }
            let mut probs = vec![0.0; counts.len()];
            for config in 0..n_configs {
                let start = config * cardinalities[v];
                let total: f64 = counts[start..start + cardinalities[v]].iter().sum();
                let total = if total > 0.0 { total } else { 1.0 };
                for k in 0..cardinalities[v] {
                    probs[start + k] = counts[start + k] / total;
                }
            }
            cpts.push(Cpt { node: v, parents, cardinality: cardinalities[v], probs });
        }
        Ok(Self { dag: dag.clone(), cardinalities, cpts })
    }

    /// 🎲️ Ancestral sampling in topological order.
    #[allow(clippy::needless_range_loop, reason = "row indexes a per-variable column selected by the inner topological-order loop, not a single Vec — no single iterator covers both loop levels")]
    pub fn simulate(&self, n_samples: usize, rng: &mut semio_framework_geometry::random::Rng) -> Result<semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table, CausalError> {
        let n = self.dag.n();
        let mut codes = vec![vec![0u32; n_samples]; n];
        for row in 0..n_samples {
            for &v in self.dag.topological_order() {
                let cpt = &self.cpts[v];
                let mut config = 0usize;
                let mut stride = 1usize;
                for &p in &cpt.parents {
                    config += codes[p][row] as usize * stride;
                    stride *= self.cardinalities[p];
                }
                let start = config * cpt.cardinality;
                let probs = &cpt.probs[start..start + cpt.cardinality];
                let mut draw = rng.next_f64();
                let mut chosen = cpt.cardinality - 1;
                for (k, &p) in probs.iter().enumerate() {
                    if draw < p {
                        chosen = k;
                        break;
                    }
                    draw -= p;
                }
                codes[v][row] = chosen as u32;
            }
        }
        let names = self.dag.names().to_vec();
        let columns: Vec<(Vec<u32>, Vec<String>)> = (0..n).map(|v| (codes[v].clone(), (0..self.cardinalities[v]).map(|k| format!("l{k}")).collect())).collect();
        Ok(semio_s_artifact_stdio_semio_table_tabular::schema::tabular_internals::Table::from_categorical_columns(names, columns)?)
    }

    /// ✂️ `do(v := value)` for each entry: replaces `v`'s CPT with a point mass and drops its parents.
    pub fn intervened(&self, interventions: &[(usize, u32)]) -> Result<Self, CausalError> {
        let intervened_map: HashMap<usize, u32> = interventions.iter().copied().collect();
        let remaining_edges: Vec<(usize, usize)> = self.dag.edges().into_iter().filter(|&(_, child)| !intervened_map.contains_key(&child)).collect();
        let new_dag = CausalDag::new(self.dag.names().to_vec(), &remaining_edges)?;
        let mut cpts = self.cpts.clone();
        for (&v, &value) in &intervened_map {
            let card = self.cardinalities[v];
            let mut probs = vec![0.0; card];
            probs[value as usize] = 1.0;
            cpts[v] = Cpt { node: v, parents: Vec::new(), cardinality: card, probs };
        }
        Ok(Self { dag: new_dag, cardinalities: self.cardinalities.clone(), cpts })
    }

    /// 🔢️ `P(target | evidence)` by variable elimination (Zhang & Poole 1994) over a
    /// min-remaining-factors ordering, guarded against blow-up.
    pub fn posterior(&self, target: usize, evidence: &[(usize, u32)]) -> Result<Vec<f64>, CausalError> {
        const MAX_FACTOR_ENTRIES: usize = 1_000_000;
        let n = self.dag.n();
        let evidence_map: HashMap<usize, u32> = evidence.iter().copied().collect();
        let mut factors: Vec<Factor> = self.cpts.iter().map(|cpt| Factor::from_cpt(cpt, &self.cardinalities)).collect();
        for (&v, &value) in &evidence_map {
            for factor in factors.iter_mut() {
                *factor = factor.restrict(v, value);
            }
        }
        let mut eliminate: Vec<usize> = (0..n).filter(|v| *v != target && !evidence_map.contains_key(v)).collect();
        while !eliminate.is_empty() {
            let (pos, &var) = eliminate.iter().enumerate().min_by_key(|&(_, &v)| factors.iter().filter(|f| f.vars.contains(&v)).count()).expect("eliminate is non-empty");
            eliminate.remove(pos);
            let (involved, mut rest): (Vec<Factor>, Vec<Factor>) = factors.into_iter().partition(|f| f.vars.contains(&var));
            if involved.is_empty() {
                factors = rest;
                continue;
            }
            let mut product = involved[0].clone();
            for f in &involved[1..] {
                product = product.multiply(f);
                if product.values.len() > MAX_FACTOR_ENTRIES {
                    return Err(CausalError::InferenceTooLarge(product.values.len(), MAX_FACTOR_ENTRIES));
                }
            }
            rest.push(product.marginalize(var));
            factors = rest;
        }
        let mut result = factors[0].clone();
        for f in &factors[1..] {
            result = result.multiply(f);
        }
        let total: f64 = result.values.iter().sum();
        if total <= 0.0 {
            return Err(CausalError::InvalidQuery("evidence has zero probability".to_string()));
        }
        Ok(result.values.iter().map(|v| v / total).collect())
    }

    /// 🔮️ `P(target | do(interventions), evidence)`: posterior on the intervened network.
    pub fn interventional_distribution(&self, target: usize, interventions: &[(usize, u32)], evidence: &[(usize, u32)]) -> Result<Vec<f64>, CausalError> {
        self.intervened(interventions)?.posterior(target, evidence)
    }
}

/// 🧮️ A variable-elimination factor: a flat table over `vars`, mixed-radix with `vars[0]` fastest-varying.
#[derive(Clone, Debug)]
struct Factor {
    vars: Vec<usize>,
    cards: Vec<usize>,
    pub(super) values: Vec<f64>,
}

impl Factor {
    fn from_cpt(cpt: &Cpt, cardinalities: &[usize]) -> Self {
        let mut vars = vec![cpt.node];
        vars.extend_from_slice(&cpt.parents);
        let cards: Vec<usize> = vars.iter().map(|&v| cardinalities[v]).collect();
        Self { vars, cards, values: cpt.probs.clone() }
    }

    fn encode(&self, assignment: &HashMap<usize, u32>) -> usize {
        let mut idx = 0usize;
        let mut stride = 1usize;
        for (i, &v) in self.vars.iter().enumerate() {
            idx += assignment[&v] as usize * stride;
            stride *= self.cards[i];
        }
        idx
    }

    fn restrict(&self, var: usize, value: u32) -> Self {
        let Some(pos) = self.vars.iter().position(|&v| v == var) else {
            return self.clone();
        };
        let new_vars: Vec<usize> = self.vars.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, v)| v).collect();
        let new_cards: Vec<usize> = self.cards.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, c)| c).collect();
        let total_new = new_cards.iter().product::<usize>().max(1);
        let mut values = Vec::with_capacity(total_new);
        for idx in 0..total_new {
            let mut assignment = decode(&new_vars, &new_cards, idx);
            assignment.insert(var, value);
            values.push(self.values[self.encode(&assignment)]);
        }
        Factor { vars: new_vars, cards: new_cards, values }
    }

    fn multiply(&self, other: &Factor) -> Factor {
        let mut vars = self.vars.clone();
        let mut cards = self.cards.clone();
        for (i, &v) in other.vars.iter().enumerate() {
            if !vars.contains(&v) {
                vars.push(v);
                cards.push(other.cards[i]);
            }
        }
        let total = cards.iter().product::<usize>().max(1);
        let mut values = Vec::with_capacity(total);
        for idx in 0..total {
            let assignment = decode(&vars, &cards, idx);
            values.push(self.values[self.encode(&assignment)] * other.values[other.encode(&assignment)]);
        }
        Factor { vars, cards, values }
    }

    fn marginalize(&self, var: usize) -> Factor {
        let Some(pos) = self.vars.iter().position(|&v| v == var) else {
            return self.clone();
        };
        let new_vars: Vec<usize> = self.vars.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, v)| v).collect();
        let new_cards: Vec<usize> = self.cards.iter().copied().enumerate().filter(|&(i, _)| i != pos).map(|(_, c)| c).collect();
        let total_new = new_cards.iter().product::<usize>().max(1);
        let mut values = vec![0.0; total_new];
        for (new_idx, slot) in values.iter_mut().enumerate() {
            let mut assignment = decode(&new_vars, &new_cards, new_idx);
            let mut sum = 0.0;
            for k in 0..self.cards[pos] {
                assignment.insert(var, k as u32);
                sum += self.values[self.encode(&assignment)];
            }
            *slot = sum;
        }
        Factor { vars: new_vars, cards: new_cards, values }
    }
}

/// 🧮️ Decodes a flat mixed-radix index (`vars[0]` fastest-varying) into a var -> value assignment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode(vars: &[usize], cards: &[usize], mut idx: usize) -> HashMap<usize, u32> {
    let mut out = HashMap::with_capacity(vars.len());
    for (i, &v) in vars.iter().enumerate() {
        let c = cards[i];
        out.insert(v, (idx % c) as u32);
        idx /= c;
    }
    out
}
// #endregion 🔖️ScmDiscrete
