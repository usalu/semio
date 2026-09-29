import Mathlib

/-!
👴 Real parts of the powers `z ^ n` of the reduced quaternion `z = x + i y + j t`.

Source: `../📯️notes/📐️.tex`, paper: `../🏆️proof/📐️.tex`. Every statement of the paper is checked here:

* Theorem 1, `Re (z ^ n) = ∑ c_k x ^ (n - 2 k) (y² + t²) ^ k` with `c_k = (-1) ^ k * n.choose (2 k)`: `re_pow`, `re_pow_even`,
* Corollary 2, `c_0 = 1`, `(2 k + 2) (2 k + 1) c_{k+1} = -(n - 2 k) (n - 2 k - 1) c_k`: `c_zero`, `c_succ`, `a_zero`, `a_succ`,
* Corollary 3, `a_m = (-1) ^ m` and `a_{m-k} = (-1) ^ m a_k` for every `k ≤ m`: `a_self`, `a_sub`,
* Lemma 5, `t Δu - ∂ₜu = t (P_xx + 4 r P_rr + 2 P_r)` for every polynomial `P`: `laplaceBeltrami_defect_general`, `laplaceBeltrami_defect`,
* Theorem 6, `t Δu = ∂ₜu` iff the recursion holds, existence and uniqueness: `laplaceBeltrami_iff`, `laplaceBeltrami_re_pow`, `eq_c_of_recursion`,
* Theorem 8, `z ^ n = Re (z ^ n) + (i y + j t) S_n` with `s_k = (-1) ^ k * n.choose (2 k + 1)`: `imI_pow`, `imJ_pow`, `imK_pow`,
* Corollary 9, `s_0 = n`, `(2 k + 3) (2 k + 2) s_{k+1} = -(n - 2 k - 1) (n - 2 k - 2) s_k`, `s_{m-1-k} = (-1) ^ (m-1) s_k`: `s_zero`, `s_succ`, `s_sub`,
* Lemma 10, defects `t y (P_xx + 4 r P_rr + 6 P_r)` and `t³ (P_xx + 4 r P_rr + 6 P_r)`: `defect_VI`, `defect_VJ`,
* Theorem 11, `t Δv = ∂ₜv` and `t² Δw - t ∂ₜw + w = 0` iff the recursion holds, existence and uniqueness:
  `laplaceBeltrami_imI_iff`, `hyperbolic_imJ_iff`, `laplaceBeltrami_imI_pow`, `hyperbolic_imJ_pow`, `eq_s_of_recursion`,
* Theorem 13, `(x + w) ^ n = R_n + S_n w` whenever `w² = -r`, in particular for paravectors of `Cl_{0,μ}`:
  `pow_eq_of_sq`, `z_pow_eq`, `clifford_pow`, `clifford_pow_components`,
* Lemma 14, defects in dimension `μ + 1` for every polynomial `P`: `defectN_re`, `defectN_im`, `defectN_last`,
* Theorem 15, `v_l Δu = (μ - 1) ∂_l u` and `v_l² Δw - (μ - 1) v_l ∂_l w + (μ - 1) w = 0` iff the recursions hold:
  `hyperbolic_re_iff`, `hyperbolic_im_iff`, `hyperbolic_last_iff`, `hyperbolic_scalar_pow`, `hyperbolic_vector_pow`, `hyperbolic_last_pow`,
* Examples 17 and 19: `Rpoly_examples`, `Spoly_examples`, `a_four`, `a_five`, `s_six`, `z_cube_example`,
* Theorem 22, coefficients in the `xy`-plane force circles of roots, no isolated root outside the plane:
  `evalPoly_z`, `polyA_polyB_eq_zero`, `circle_of_roots`, `root_not_isolated`,
* Corollary 23, roots of `z² + b z + c` with real `b`, `c`: `real_quadratic_root_iff`,
* Theorem 24, `q² - q (1 + j) + j` has exactly the roots `1` and `j`: `jPoly_root_iff`, `jPoly_root_outside_plane`,
* Theorem 26, real coefficients of any degree: `real_poly_root_iff`; Corollary 27, degree three: `real_cubic_root_iff`, `real_cubic_circle`,
* Theorem 28, coefficients in `ℂ_j` on all of `ℍ`, spheres of roots: `quat_pow_eq`, `evalPoly_quat`, `polyA_polyB_eq_zero_J`, `sphere_of_roots_J`,
* Theorem 29, `(q - 1)(q - j)(q - 2 j)` has exactly the roots `1`, `j`, `2 j`: `jCubic_root_iff`,
* Theorem 30, `qⁿ = j`: roots exist, lie in `ℂ_j` outside the `xy`-plane and are finitely many: `exists_pow_eq_j`, `pow_eq_j`, `pow_eq_j_finite`.
-/

open Finset Quaternion

namespace RealPartsOfPowers

/-- 🔢 Coefficient `c_k` of `x ^ (n - 2 k) (y² + t²) ^ k` in `Re (z ^ n)`. -/
def c (n k : ℕ) : ℤ := (-1) ^ k * (n.choose (2 * k) : ℤ)

/-- 🔢 Coefficient `a_k` of `x ^ (2 (m - k)) (y² + t²) ^ k` in `Re (z ^ (2 m))`. -/
def a (m k : ℕ) : ℤ := c (2 * m) k

/-- 🥇 `c_0 = 1`. -/
theorem c_zero (n : ℕ) : c n 0 = 1 := by simp [c]

/-- 🕳️ `c_k = 0` beyond the top coefficient. -/
theorem c_eq_zero {n k : ℕ} (h : n < 2 * k) : c n k = 0 := by
  simp [c, Nat.choose_eq_zero_of_lt h]

/-- 🔁 Explicit two-term recursion of the coefficients, for every exponent `n` and every `k`. -/
theorem c_succ (n k : ℕ) :
    ((2 * k + 2 : ℤ) * (2 * k + 1)) * c n (k + 1)
      = -(((n : ℤ) - 2 * k) * ((n : ℤ) - 2 * k - 1)) * c n k := by
  by_cases h : 2 * k + 2 ≤ n
  · have h1 := Nat.choose_succ_right_eq n (2 * k)
    have h2 := Nat.choose_succ_right_eq n (2 * k + 1)
    zify [show 2 * k ≤ n by omega, show 2 * k + 1 ≤ n by omega] at h1 h2
    simp only [c, show 2 * (k + 1) = 2 * k + 1 + 1 by ring, pow_succ]
    linear_combination (-1 : ℤ) ^ k * (-(2 * k + 1 : ℤ)) * h2
      + (-1 : ℤ) ^ k * (-((n : ℤ) - 2 * k - 1)) * h1
  · rw [c_eq_zero (show n < 2 * (k + 1) by omega)]
    rcases (show n = 2 * k + 1 ∨ n = 2 * k ∨ n < 2 * k by omega) with h1 | h1 | h1
    · subst h1
      push_cast
      ring
    · subst h1
      push_cast
      ring
    · rw [c_eq_zero h1]
      ring

/-- 🥇 `a_0 = 1`. -/
theorem a_zero (m : ℕ) : a m 0 = 1 := c_zero _

/-- 🏁 `a_m = (-1) ^ m`. -/
theorem a_self (m : ℕ) : a m m = (-1) ^ m := by simp [a, c]

/-- 🕳️ `a_k = 0` beyond the top coefficient. -/
theorem a_eq_zero {m k : ℕ} (h : m < k) : a m k = 0 := c_eq_zero (by omega)

/-- 🔁 Explicit two-term recursion of the even coefficients. -/
theorem a_succ (m k : ℕ) :
    ((2 * k + 2 : ℤ) * (2 * k + 1)) * a m (k + 1)
      = -((2 * m - 2 * k : ℤ) * (2 * m - 2 * k - 1)) * a m k := by
  have h := c_succ (2 * m) k
  push_cast at h
  exact h

/-- 🪞 Symmetry `a_{m-k} = (-1) ^ m a_k`, which contains every conjectured relation of the source. -/
theorem a_sub {m k : ℕ} (h : k ≤ m) : a m (m - k) = (-1) ^ m * a m k := by
  have hs : (-1 : ℤ) ^ (m - k) = (-1) ^ m * (-1) ^ k := by
    have h1 : (-1 : ℤ) ^ m = (-1) ^ (m - k) * (-1) ^ k := by rw [← pow_add, Nat.sub_add_cancel h]
    have h2 : ((-1 : ℤ) ^ k) * (-1) ^ k = 1 := by rw [← mul_pow]; simp
    rw [h1, mul_assoc, h2, mul_one]
  have hc : (2 * m).choose (2 * (m - k)) = (2 * m).choose (2 * k) := by
    rw [show 2 * (m - k) = 2 * m - 2 * k by omega]
    exact Nat.choose_symm (by omega)
  simp only [a, c, hs, hc]
  ring

/-- 🧭 The reduced quaternion `x + i y + j t`. -/
def z (x y t : ℝ) : ℍ[ℝ] := ⟨x, y, t, 0⟩

/-- 🧲 The vector part `i y + j t`. -/
def v (y t : ℝ) : ℍ[ℝ] := ⟨0, y, t, 0⟩

theorem z_eq (x y t : ℝ) : z x y t = v y t + (x : ℍ[ℝ]) := by
  ext <;> simp [z, v]

theorem v_mul_self (y t : ℝ) : v y t * v y t = ((-(y ^ 2 + t ^ 2) : ℝ) : ℍ[ℝ]) := by
  ext <;> simp [v, -Quaternion.coe_pow] <;> ring

theorem v_pow_even (y t : ℝ) (k : ℕ) :
    v y t ^ (2 * k) = (((-(y ^ 2 + t ^ 2)) ^ k : ℝ) : ℍ[ℝ]) := by
  rw [pow_mul, pow_two, v_mul_self, Quaternion.coe_pow]

theorem v_pow_odd (y t : ℝ) (k : ℕ) :
    v y t ^ (2 * k + 1) = (((-(y ^ 2 + t ^ 2)) ^ k : ℝ) : ℍ[ℝ]) * v y t := by
  rw [pow_succ, v_pow_even]

theorem re_mul_coe (p : ℍ[ℝ]) (s : ℝ) : (p * (s : ℍ[ℝ])).re = p.re * s := by simp

theorem re_coe_mul (p : ℍ[ℝ]) (s : ℝ) : ((s : ℍ[ℝ]) * p).re = s * p.re := by simp

theorem re_sum (s : Finset ℕ) (f : ℕ → ℍ[ℝ]) : (∑ i ∈ s, f i).re = ∑ i ∈ s, (f i).re := by
  classical
  induction s using Finset.induction_on with
  | empty => simp
  | insert i s hi ih => rw [sum_insert hi, sum_insert hi, Quaternion.re_add, ih]

/-- ✂️ Parity split of a sum over `range (2 m + 1)`. -/
theorem sum_range_parity {M : Type*} [AddCommMonoid M] (f : ℕ → M) (m : ℕ) :
    ∑ i ∈ range (2 * m + 1), f i
      = ∑ k ∈ range (m + 1), f (2 * k) + ∑ k ∈ range m, f (2 * k + 1) := by
  induction m with
  | zero => simp
  | succ m ih =>
    rw [show 2 * (m + 1) + 1 = 2 * m + 1 + 1 + 1 by ring, sum_range_succ, sum_range_succ, ih,
      sum_range_succ (fun k => f (2 * k)) (m + 1), sum_range_succ (fun k => f (2 * k + 1)) m,
      show 2 * (m + 1) = 2 * m + 1 + 1 by ring]
    abel

/-- ✂️ Parity split of a sum over `range (2 m + 2)`. -/
theorem sum_range_parity_odd {M : Type*} [AddCommMonoid M] (f : ℕ → M) (m : ℕ) :
    ∑ i ∈ range (2 * m + 1 + 1), f i
      = ∑ k ∈ range (m + 1), f (2 * k) + ∑ k ∈ range (m + 1), f (2 * k + 1) := by
  rw [sum_range_succ, sum_range_parity, sum_range_succ (fun k => f (2 * k + 1)) m]
  abel

/-- 🧮 Binomial expansion of `z ^ n`. -/
theorem z_pow (x y t : ℝ) (n : ℕ) :
    z x y t ^ n = ∑ i ∈ range (n + 1),
      v y t ^ i * (((x ^ (n - i) * (n.choose i : ℝ) : ℝ)) : ℍ[ℝ]) := by
  have hc : Commute (v y t) (x : ℍ[ℝ]) := (Quaternion.coe_commutes x (v y t)).symm
  rw [z_eq, hc.add_pow]
  refine sum_congr rfl fun i _ => ?_
  rw [mul_assoc]
  congr 1
  push_cast
  rfl

theorem re_term_odd (x y t : ℝ) (n k : ℕ) :
    (v y t ^ (2 * k + 1)
      * (((x ^ (n - (2 * k + 1)) * (n.choose (2 * k + 1) : ℝ) : ℝ)) : ℍ[ℝ])).re = 0 := by
  rw [v_pow_odd, re_mul_coe, re_coe_mul]
  simp [v]

theorem re_term_even (x y t : ℝ) (n k : ℕ) :
    (v y t ^ (2 * k) * (((x ^ (n - 2 * k) * (n.choose (2 * k) : ℝ) : ℝ)) : ℍ[ℝ])).re
      = (c n k : ℝ) * x ^ (n - 2 * k) * (y ^ 2 + t ^ 2) ^ k := by
  rw [v_pow_even, re_mul_coe, Quaternion.re_coe, neg_pow]
  simp only [c]
  push_cast
  ring

/-- 👑 Theorem 1: closed form of `Re (z ^ n)` for every exponent. -/
theorem re_pow (x y t : ℝ) (n : ℕ) :
    (z x y t ^ n).re
      = ∑ k ∈ range (n / 2 + 1), (c n k : ℝ) * x ^ (n - 2 * k) * (y ^ 2 + t ^ 2) ^ k := by
  rw [z_pow]
  rcases Nat.even_or_odd' n with ⟨m, rfl | rfl⟩
  · rw [sum_range_parity, Quaternion.re_add, re_sum, re_sum,
      sum_congr rfl fun k _ => re_term_even x y t (2 * m) k,
      sum_congr rfl fun k _ => re_term_odd x y t (2 * m) k,
      show 2 * m / 2 = m by omega]
    simp
  · rw [sum_range_parity_odd, Quaternion.re_add, re_sum, re_sum,
      sum_congr rfl fun k _ => re_term_even x y t (2 * m + 1) k,
      sum_congr rfl fun k _ => re_term_odd x y t (2 * m + 1) k,
      show (2 * m + 1) / 2 = m by omega]
    simp

/-- 👑 Closed form of `Re (z ^ (2 m))` with the coefficients `a m k`. -/
theorem re_pow_even (x y t : ℝ) (m : ℕ) :
    (z x y t ^ (2 * m)).re
      = ∑ k ∈ range (m + 1), (a m k : ℝ) * x ^ (2 * (m - k)) * (y ^ 2 + t ^ 2) ^ k := by
  rw [re_pow, show 2 * m / 2 = m by omega]
  refine sum_congr rfl fun k _ => ?_
  rw [show 2 * m - 2 * k = 2 * (m - k) by omega]
  rfl

/-! ### The Laplace–Beltrami equation -/

/-- 🧱 A general polynomial `P (x, y² + t²)`, given as a finite sum of monomials `d_i x ^ p_i (y² + t²) ^ k_i`. -/
def V {ι : Type*} (s : Finset ι) (d : ι → ℝ) (p k : ι → ℕ) (x y t : ℝ) : ℝ :=
  ∑ i ∈ s, d i * x ^ p i * (y ^ 2 + t ^ 2) ^ k i

/-- 🧱 The polynomial `∑ d_k x ^ (n - 2 k) (y² + t²) ^ k`. -/
def U (n : ℕ) (d : ℕ → ℝ) : ℝ → ℝ → ℝ → ℝ :=
  V (range (n / 2 + 1)) d (fun k => n - 2 * k) id

/-- ∂ Partial derivative in `t`. -/
noncomputable def dt (u : ℝ → ℝ → ℝ → ℝ) (x y t : ℝ) : ℝ := deriv (fun s => u x y s) t

/-- Δ Euclidean Laplacian on `ℝ³`. -/
noncomputable def laplacian (u : ℝ → ℝ → ℝ → ℝ) (x y t : ℝ) : ℝ :=
  deriv (fun s => deriv (fun s' => u s' y t) s) x
    + deriv (fun s => deriv (fun s' => u x s' t) s) y
    + deriv (fun s => deriv (fun s' => u x y s') s) t

/-- 🧾 `P_xx + 4 r P_rr + 2 P_r` for `P = ∑ d_k x ^ (n - 2 k) r ^ k`, coefficientwise in `r`. -/
def b (n : ℕ) (d : ℕ → ℝ) (k : ℕ) : ℝ :=
  ((n - 2 * k : ℕ) : ℝ) * ((n - 2 * k - 1 : ℕ) : ℝ) * d k + (2 * k + 2) * (2 * k + 1) * d (k + 1)

section General

variable {ι : Type*} (s : Finset ι) (d : ι → ℝ) (p k : ι → ℕ)

def D1 (x y t : ℝ) : ℝ :=
  ∑ i ∈ s, d i * x ^ p i * ((k i : ℝ) * (y ^ 2 + t ^ 2) ^ (k i - 1) * (2 * t))

def D2 (x y t : ℝ) : ℝ :=
  ∑ i ∈ s, d i * x ^ p i
    * ((k i : ℝ) * (((k i - 1 : ℕ) : ℝ) * (y ^ 2 + t ^ 2) ^ (k i - 1 - 1) * (2 * t)) * (2 * t)
      + (k i : ℝ) * (y ^ 2 + t ^ 2) ^ (k i - 1) * 2)

def E1 (x y t : ℝ) : ℝ :=
  ∑ i ∈ s, d i * ((p i : ℝ) * x ^ (p i - 1)) * (y ^ 2 + t ^ 2) ^ k i

def E2 (x y t : ℝ) : ℝ :=
  ∑ i ∈ s, d i * ((p i : ℝ) * (((p i - 1 : ℕ) : ℝ) * x ^ (p i - 1 - 1))) * (y ^ 2 + t ^ 2) ^ k i

theorem hasDerivAt_radial (q : ℝ) (j : ℕ) (r : ℝ) :
    HasDerivAt (fun r => (q + r ^ 2) ^ j) ((j : ℝ) * (q + r ^ 2) ^ (j - 1) * (2 * r)) r := by
  have h := ((hasDerivAt_pow 2 r).const_add q).pow j
  convert h using 1
  simp

theorem hasDerivAt_radial' (q : ℝ) (j : ℕ) (r : ℝ) :
    HasDerivAt (fun r => (j : ℝ) * (q + r ^ 2) ^ (j - 1) * (2 * r))
      ((j : ℝ) * (((j - 1 : ℕ) : ℝ) * (q + r ^ 2) ^ (j - 1 - 1) * (2 * r)) * (2 * r)
        + (j : ℝ) * (q + r ^ 2) ^ (j - 1) * 2) r := by
  have h := ((hasDerivAt_radial q (j - 1) r).const_mul (j : ℝ)).mul ((hasDerivAt_id r).const_mul 2)
  convert h using 1
  · ext y
    simp
  · simp

theorem V_symm (x y t : ℝ) : V s d p k x y t = V s d p k x t y := by
  simp only [V, add_comm]

theorem hasDerivAt_V_t (x y t : ℝ) :
    HasDerivAt (fun r => V s d p k x y r) (D1 s d p k x y t) t := by
  unfold V D1
  exact HasDerivAt.fun_sum fun i _ => (hasDerivAt_radial (y ^ 2) (k i) t).const_mul _

theorem hasDerivAt_D1_t (x y t : ℝ) :
    HasDerivAt (fun r => D1 s d p k x y r) (D2 s d p k x y t) t := by
  unfold D1 D2
  exact HasDerivAt.fun_sum fun i _ => (hasDerivAt_radial' (y ^ 2) (k i) t).const_mul _

theorem hasDerivAt_V_x (x y t : ℝ) :
    HasDerivAt (fun r => V s d p k r y t) (E1 s d p k x y t) x := by
  unfold V E1
  exact HasDerivAt.fun_sum fun i _ => ((hasDerivAt_pow (p i) x).const_mul (d i)).mul_const _

theorem hasDerivAt_E1_x (x y t : ℝ) :
    HasDerivAt (fun r => E1 s d p k r y t) (E2 s d p k x y t) x := by
  unfold E1 E2
  exact HasDerivAt.fun_sum fun i _ =>
    (((hasDerivAt_pow (p i - 1) x).const_mul _).const_mul (d i)).mul_const _

theorem dt_V (x y t : ℝ) : dt (V s d p k) x y t = D1 s d p k x y t :=
  (hasDerivAt_V_t s d p k x y t).deriv

theorem laplacian_V (x y t : ℝ) :
    laplacian (V s d p k) x y t = E2 s d p k x y t + D2 s d p k x t y + D2 s d p k x y t := by
  have hx : (fun r => deriv (fun r' => V s d p k r' y t) r) = fun r => E1 s d p k r y t :=
    funext fun r => (hasDerivAt_V_x s d p k r y t).deriv
  have ht : (fun r => deriv (fun r' => V s d p k x y r') r) = fun r => D1 s d p k x y r :=
    funext fun r => (hasDerivAt_V_t s d p k x y r).deriv
  have hy : (fun r => deriv (fun r' => V s d p k x r' t) r) = fun r => D1 s d p k x t r := by
    funext r
    rw [show (fun r' => V s d p k x r' t) = fun r' => V s d p k x t r' from
      funext fun r' => V_symm s d p k x r' t]
    exact (hasDerivAt_V_t s d p k x t r).deriv
  unfold laplacian
  rw [hx, ht, hy, (hasDerivAt_E1_x s d p k x y t).deriv, (hasDerivAt_D1_t s d p k x y t).deriv,
    (hasDerivAt_D1_t s d p k x t y).deriv]

theorem radial_term (j : ℕ) (y t : ℝ) :
    t * (((j : ℝ) * (((j - 1 : ℕ) : ℝ) * (t ^ 2 + y ^ 2) ^ (j - 1 - 1) * (2 * y)) * (2 * y)
        + (j : ℝ) * (t ^ 2 + y ^ 2) ^ (j - 1) * 2)
      + ((j : ℝ) * (((j - 1 : ℕ) : ℝ) * (y ^ 2 + t ^ 2) ^ (j - 1 - 1) * (2 * t)) * (2 * t)
        + (j : ℝ) * (y ^ 2 + t ^ 2) ^ (j - 1) * 2))
      - (j : ℝ) * (y ^ 2 + t ^ 2) ^ (j - 1) * (2 * t)
      = t * (2 * (j : ℝ) * (2 * j - 1) * (y ^ 2 + t ^ 2) ^ (j - 1)) := by
  rcases j with _ | _ | j
  · simp
  · simp
    ring
  · simp only [Nat.add_sub_cancel, show j + 1 + 1 - 1 = j + 1 by omega]
    push_cast
    ring

/-- 📐 Lemma 5 for an arbitrary polynomial `P = ∑ d_i x ^ p_i r ^ k_i`: the defect `t Δu - ∂ₜu` of
`u = P (x, y² + t²)` is `t (P_xx + 4 r P_rr + 2 P_r)`, where monomialwise
`P_xx = p (p - 1) x ^ (p - 2) r ^ k` and `4 r P_rr + 2 P_r = 2 k (2 k - 1) x ^ p r ^ (k - 1)`. -/
theorem laplaceBeltrami_defect_general (x y t : ℝ) :
    t * laplacian (V s d p k) x y t - dt (V s d p k) x y t
      = t * ∑ i ∈ s, d i * ((p i : ℝ) * ((p i - 1 : ℕ) : ℝ) * x ^ (p i - 1 - 1) * (y ^ 2 + t ^ 2) ^ k i
          + 2 * (k i : ℝ) * (2 * k i - 1) * x ^ p i * (y ^ 2 + t ^ 2) ^ (k i - 1)) := by
  rw [laplacian_V, dt_V]
  unfold E2 D2 D1
  simp only [mul_add, mul_sum, ← sum_add_distrib, ← sum_sub_distrib]
  refine sum_congr rfl fun i _ => ?_
  linear_combination d i * x ^ p i * radial_term (k i) y t

end General

/-- 📐 Lemma 5 for `P = ∑ d_k x ^ (n - 2 k) r ^ k`, collected by powers of `r`. -/
theorem laplaceBeltrami_defect (n : ℕ) (d : ℕ → ℝ) (hd : d (n / 2 + 1) = 0) (x y t : ℝ) :
    t * laplacian (U n d) x y t - dt (U n d) x y t
      = t * ∑ k ∈ range (n / 2 + 1), b n d k * x ^ (n - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k := by
  have hshift : ∑ k ∈ range (n / 2 + 1),
        2 * (k : ℝ) * (2 * k - 1) * d k * x ^ (n - 2 * k) * (y ^ 2 + t ^ 2) ^ (k - 1)
      = ∑ k ∈ range (n / 2 + 1),
        (2 * (k : ℝ) + 2) * (2 * k + 1) * d (k + 1) * x ^ (n - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k := by
    rw [sum_range_succ', sum_range_succ _ (n / 2), hd]
    simp only [Nat.cast_zero, mul_zero, zero_mul, add_zero, Nat.add_sub_cancel]
    refine sum_congr rfl fun k _ => ?_
    rw [show n - 2 * (k + 1) = n - 2 * k - 1 - 1 by omega]
    push_cast
    ring
  unfold U
  rw [laplaceBeltrami_defect_general]
  simp only [id]
  calc _ = ∑ k ∈ range (n / 2 + 1),
        (((n - 2 * k : ℕ) : ℝ) * ((n - 2 * k - 1 : ℕ) : ℝ) * d k * x ^ (n - 2 * k - 1 - 1)
            * (y ^ 2 + t ^ 2) ^ k
          + 2 * (k : ℝ) * (2 * k - 1) * d k * x ^ (n - 2 * k) * (y ^ 2 + t ^ 2) ^ (k - 1)) * t := by
        rw [mul_comm, sum_mul]
        refine sum_congr rfl fun k _ => ?_
        ring
    _ = _ := by
        rw [← sum_mul, sum_add_distrib, hshift, ← sum_add_distrib, mul_comm]
        congr 1
        refine sum_congr rfl fun k _ => ?_
        unfold b
        ring

theorem b_eq_zero_iff (n : ℕ) (d : ℕ → ℝ) (hd : ∀ k, n / 2 < k → d k = 0) :
    (∀ k ∈ range (n / 2 + 1), b n d k = 0)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 2) * (2 * k + 1) * d (k + 1)
          = -(((n : ℝ) - 2 * k) * ((n : ℝ) - 2 * k - 1)) * d k := by
  have key : ∀ k, 2 * k ≤ n → ((n - 2 * k : ℕ) : ℝ) * ((n - 2 * k - 1 : ℕ) : ℝ)
      = ((n : ℝ) - 2 * k) * ((n : ℝ) - 2 * k - 1) := by
    intro k hk
    rcases Nat.eq_zero_or_pos (n - 2 * k) with h0 | h0
    · have : (n : ℝ) = 2 * k := by exact_mod_cast (show n = 2 * k by omega)
      rw [h0, this]
      simp
    · rw [Nat.cast_sub (show 1 ≤ n - 2 * k from h0), Nat.cast_sub hk]
      push_cast
      ring
  constructor
  · intro h k
    by_cases hk : k < n / 2 + 1
    · have := h k (mem_range.mpr hk)
      unfold b at this
      rw [key k (by omega)] at this
      linear_combination this
    · rw [hd k (by omega), hd (k + 1) (by omega)]
      ring
  · intro h k hk
    have hk' := mem_range.mp hk
    unfold b
    rw [key k (by omega)]
    linear_combination h k

/-- 🏛️ Theorem 6: `t Δu = ∂ₜu` holds identically iff the coefficients satisfy the two-term recursion. -/
theorem laplaceBeltrami_iff (n : ℕ) (d : ℕ → ℝ) (hd : ∀ k, n / 2 < k → d k = 0) :
    (∀ x y t : ℝ, t * laplacian (U n d) x y t = dt (U n d) x y t)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 2) * (2 * k + 1) * d (k + 1)
          = -(((n : ℝ) - 2 * k) * ((n : ℝ) - 2 * k - 1)) * d k := by
  rw [← b_eq_zero_iff n d hd]
  have hdefect := laplaceBeltrami_defect n d (hd _ (Nat.lt_succ_self _))
  constructor
  · intro h
    let p : Polynomial ℝ := ∑ k ∈ range (n / 2 + 1), Polynomial.C (b n d k) * Polynomial.X ^ k
    have hroot : ∀ r : ℝ, 0 < r → p.IsRoot r := by
      intro r hr
      have h1 := hdefect 1 0 (Real.sqrt r)
      rw [h 1 0 (Real.sqrt r), sub_self] at h1
      have hs : Real.sqrt r ≠ 0 := (Real.sqrt_pos.mpr hr).ne'
      have h2 := (mul_eq_zero.mp h1.symm).resolve_left hs
      have hsq : (0 : ℝ) ^ 2 + Real.sqrt r ^ 2 = r := by rw [Real.sq_sqrt hr.le]; ring
      simp only [one_pow, mul_one, hsq] at h2
      simp only [Polynomial.IsRoot, p, Polynomial.eval_finsetSum, Polynomial.eval_mul,
        Polynomial.eval_C, Polynomial.eval_pow, Polynomial.eval_X]
      exact h2
    have hp : p = 0 := Polynomial.eq_zero_of_infinite_isRoot p
      (Set.Infinite.mono (fun r hr => hroot r hr) (Set.Ioi_infinite (0 : ℝ)))
    intro k hk
    have hcoeff : p.coeff k = b n d k := by
      simp only [p, Polynomial.finsetSum_coeff, Polynomial.coeff_C_mul_X_pow]
      rw [sum_eq_single k (fun j _ hj => by simp [Ne.symm hj]) (fun hk' => absurd hk hk')]
      simp
    rw [← hcoeff, hp, Polynomial.coeff_zero]
  · intro h x y t
    have h1 := hdefect x y t
    rw [sum_congr rfl fun k hk => by rw [h k hk, zero_mul, zero_mul]] at h1
    simp only [sum_const_zero, mul_zero] at h1
    linarith

/-- 🎯 The recursion with `d_0 = 1` determines the coefficients: they are those of `Re (z ^ n)`. -/
theorem eq_c_of_recursion (n : ℕ) (d : ℕ → ℝ) (h0 : d 0 = 1)
    (h : ∀ k : ℕ, (2 * (k : ℝ) + 2) * (2 * k + 1) * d (k + 1)
      = -(((n : ℝ) - 2 * k) * ((n : ℝ) - 2 * k - 1)) * d k) (k : ℕ) : d k = (c n k : ℝ) := by
  induction k with
  | zero => simp [h0, c_zero]
  | succ k ih =>
    have hc : (2 * (k : ℝ) + 2) * (2 * k + 1) * (c n (k + 1) : ℝ)
        = -(((n : ℝ) - 2 * k) * ((n : ℝ) - 2 * k - 1)) * (c n k : ℝ) := by
      exact_mod_cast c_succ n k
    have hpos : (2 * (k : ℝ) + 2) * (2 * k + 1) ≠ 0 := by positivity
    apply mul_left_cancel₀ hpos
    rw [h k, hc, ih]

/-- 🌊 `Re (z ^ n)` solves `t Δu = ∂ₜu` for every exponent `n`. -/
theorem laplaceBeltrami_re_pow (n : ℕ) (x y t : ℝ) :
    t * laplacian (fun x y t => (z x y t ^ n).re) x y t
      = dt (fun x y t => (z x y t ^ n).re) x y t := by
  have hU : (fun x y t => (z x y t ^ n).re) = U n (fun k => (c n k : ℝ)) := by
    funext x y t
    exact re_pow x y t n
  rw [hU]
  refine (laplaceBeltrami_iff n _ fun k hk => ?_).mpr (fun k => ?_) x y t
  · exact_mod_cast c_eq_zero (show n < 2 * k by omega)
  · exact_mod_cast c_succ n k

/-! ### The components of `i` and `j` -/

/-- 🔢 Coefficient `s_k` of `x ^ (n - 1 - 2 k) (y² + t²) ^ k` in the common factor `S_n` of the `i` and `j` components. -/
def s (n k : ℕ) : ℤ := (-1) ^ k * (n.choose (2 * k + 1) : ℤ)

/-- 🥇 `s_0 = n`. -/
theorem s_zero (n : ℕ) : s n 0 = n := by simp [s]

/-- 🕳️ `s_k = 0` beyond the top coefficient. -/
theorem s_eq_zero {n k : ℕ} (h : n < 2 * k + 1) : s n k = 0 := by
  simp [s, Nat.choose_eq_zero_of_lt h]

/-- 🔁 Explicit two-term recursion of the coefficients of `S_n`. -/
theorem s_succ (n k : ℕ) :
    ((2 * k + 3 : ℤ) * (2 * k + 2)) * s n (k + 1)
      = -(((n : ℤ) - 2 * k - 1) * ((n : ℤ) - 2 * k - 2)) * s n k := by
  by_cases h : 2 * k + 3 ≤ n
  · have h1 := Nat.choose_succ_right_eq n (2 * k + 1)
    have h2 := Nat.choose_succ_right_eq n (2 * k + 1 + 1)
    zify [show 2 * k + 1 ≤ n by omega, show 2 * k + 1 + 1 ≤ n by omega] at h1 h2
    simp only [s, show 2 * (k + 1) + 1 = 2 * k + 1 + 1 + 1 by ring, pow_succ]
    linear_combination (-1 : ℤ) ^ k * (-(2 * k + 2 : ℤ)) * h2
      + (-1 : ℤ) ^ k * (-((n : ℤ) - 2 * k - 2)) * h1
  · rw [s_eq_zero (show n < 2 * (k + 1) + 1 by omega)]
    rcases (show n = 2 * k + 2 ∨ n = 2 * k + 1 ∨ n < 2 * k + 1 by omega) with h1 | h1 | h1
    · subst h1
      push_cast
      ring
    · subst h1
      push_cast
      ring
    · rw [s_eq_zero h1]
      ring

/-- 🪞 Symmetry `s_{m-1-k} = (-1) ^ (m-1) s_k` for the even exponent `2 m`. -/
theorem s_sub {m k : ℕ} (h : k < m) : s (2 * m) (m - 1 - k) = (-1) ^ (m - 1) * s (2 * m) k := by
  have hs : (-1 : ℤ) ^ (m - 1 - k) = (-1) ^ (m - 1) * (-1) ^ k := by
    have h1 : (-1 : ℤ) ^ (m - 1) = (-1) ^ (m - 1 - k) * (-1) ^ k := by
      rw [← pow_add, Nat.sub_add_cancel (by omega)]
    have h2 : ((-1 : ℤ) ^ k) * (-1) ^ k = 1 := by rw [← mul_pow]; simp
    rw [h1, mul_assoc, h2, mul_one]
  have hc : (2 * m).choose (2 * (m - 1 - k) + 1) = (2 * m).choose (2 * k + 1) := by
    rw [show 2 * (m - 1 - k) + 1 = 2 * m - (2 * k + 1) by omega]
    exact Nat.choose_symm (by omega)
  simp only [s, hs, hc]
  ring

theorem imI_sum (σ : Finset ℕ) (f : ℕ → ℍ[ℝ]) : (∑ i ∈ σ, f i).imI = ∑ i ∈ σ, (f i).imI := by
  classical
  induction σ using Finset.induction_on with
  | empty => simp
  | insert i σ hi ih => rw [sum_insert hi, sum_insert hi, Quaternion.imI_add, ih]

theorem imJ_sum (σ : Finset ℕ) (f : ℕ → ℍ[ℝ]) : (∑ i ∈ σ, f i).imJ = ∑ i ∈ σ, (f i).imJ := by
  classical
  induction σ using Finset.induction_on with
  | empty => simp
  | insert i σ hi ih => rw [sum_insert hi, sum_insert hi, Quaternion.imJ_add, ih]

theorem imK_sum (σ : Finset ℕ) (f : ℕ → ℍ[ℝ]) : (∑ i ∈ σ, f i).imK = ∑ i ∈ σ, (f i).imK := by
  classical
  induction σ using Finset.induction_on with
  | empty => simp
  | insert i σ hi ih => rw [sum_insert hi, sum_insert hi, Quaternion.imK_add, ih]

theorem im_term_even (y t w : ℝ) (k : ℕ) :
    (v y t ^ (2 * k) * (w : ℍ[ℝ])).imI = 0 ∧ (v y t ^ (2 * k) * (w : ℍ[ℝ])).imJ = 0
      ∧ (v y t ^ (2 * k) * (w : ℍ[ℝ])).imK = 0 := by
  rw [v_pow_even, ← Quaternion.coe_mul]
  exact ⟨rfl, rfl, rfl⟩

theorem im_term_odd (y t w : ℝ) (k : ℕ) :
    (v y t ^ (2 * k + 1) * (w : ℍ[ℝ])).imI = (-(y ^ 2 + t ^ 2)) ^ k * y * w
      ∧ (v y t ^ (2 * k + 1) * (w : ℍ[ℝ])).imJ = (-(y ^ 2 + t ^ 2)) ^ k * t * w
      ∧ (v y t ^ (2 * k + 1) * (w : ℍ[ℝ])).imK = 0 := by
  rw [v_pow_odd]
  generalize (-(y ^ 2 + t ^ 2)) ^ k = q
  simp [v]

theorem s_term (x y t q : ℝ) (n k : ℕ) :
    (-(y ^ 2 + t ^ 2)) ^ k * q * (x ^ (n - (2 * k + 1)) * (n.choose (2 * k + 1) : ℝ))
      = q * ((s n k : ℝ) * x ^ (n - 1 - 2 * k) * (y ^ 2 + t ^ 2) ^ k) := by
  rw [show n - (2 * k + 1) = n - 1 - 2 * k by omega, neg_pow]
  simp only [s]
  push_cast
  ring

/-- 👑 The `i` component: `Im_i (z ^ n) = y S_n`. -/
theorem imI_pow (x y t : ℝ) (n : ℕ) :
    (z x y t ^ n).imI
      = y * ∑ k ∈ range ((n + 1) / 2), (s n k : ℝ) * x ^ (n - 1 - 2 * k) * (y ^ 2 + t ^ 2) ^ k := by
  rw [z_pow, mul_sum]
  rcases Nat.even_or_odd' n with ⟨m, rfl | rfl⟩
  · rw [sum_range_parity, Quaternion.imI_add, imI_sum, imI_sum,
      sum_congr rfl fun k _ => (im_term_even y t _ k).1,
      sum_congr rfl fun k _ => (im_term_odd y t _ k).1,
      sum_congr rfl fun k _ => s_term x y t y (2 * m) k,
      show (2 * m + 1) / 2 = m by omega]
    simp
  · rw [sum_range_parity_odd, Quaternion.imI_add, imI_sum, imI_sum,
      sum_congr rfl fun k _ => (im_term_even y t _ k).1,
      sum_congr rfl fun k _ => (im_term_odd y t _ k).1,
      sum_congr rfl fun k _ => s_term x y t y (2 * m + 1) k,
      show (2 * m + 1 + 1) / 2 = m + 1 by omega]
    simp

/-- 👑 The `j` component: `Im_j (z ^ n) = t S_n`. -/
theorem imJ_pow (x y t : ℝ) (n : ℕ) :
    (z x y t ^ n).imJ
      = t * ∑ k ∈ range ((n + 1) / 2), (s n k : ℝ) * x ^ (n - 1 - 2 * k) * (y ^ 2 + t ^ 2) ^ k := by
  rw [z_pow, mul_sum]
  rcases Nat.even_or_odd' n with ⟨m, rfl | rfl⟩
  · rw [sum_range_parity, Quaternion.imJ_add, imJ_sum, imJ_sum,
      sum_congr rfl fun k _ => (im_term_even y t _ k).2.1,
      sum_congr rfl fun k _ => (im_term_odd y t _ k).2.1,
      sum_congr rfl fun k _ => s_term x y t t (2 * m) k,
      show (2 * m + 1) / 2 = m by omega]
    simp
  · rw [sum_range_parity_odd, Quaternion.imJ_add, imJ_sum, imJ_sum,
      sum_congr rfl fun k _ => (im_term_even y t _ k).2.1,
      sum_congr rfl fun k _ => (im_term_odd y t _ k).2.1,
      sum_congr rfl fun k _ => s_term x y t t (2 * m + 1) k,
      show (2 * m + 1 + 1) / 2 = m + 1 by omega]
    simp

/-- 👑 The `k` component vanishes: the powers stay in the reduced quaternions. -/
theorem imK_pow (x y t : ℝ) (n : ℕ) : (z x y t ^ n).imK = 0 := by
  rw [z_pow, imK_sum]
  refine sum_eq_zero fun i _ => ?_
  rcases Nat.even_or_odd' i with ⟨k, rfl | rfl⟩
  · exact (im_term_even y t _ k).2.2
  · exact (im_term_odd y t _ k).2.2

section Components

variable {ι : Type*} (σ : Finset ι) (d : ι → ℝ) (p k : ι → ℕ)

/-- 🧱 `y P (x, y² + t²)`. -/
def VI (x y t : ℝ) : ℝ := y * V σ d p k x y t

/-- 🧱 `t P (x, y² + t²)`. -/
def VJ (x y t : ℝ) : ℝ := t * V σ d p k x y t

/-- 🧾 `P_xx + 4 r P_rr + 6 P_r`, monomialwise. -/
def N (x y t : ℝ) : ℝ :=
  ∑ i ∈ σ, d i * ((p i : ℝ) * ((p i - 1 : ℕ) : ℝ) * x ^ (p i - 1 - 1) * (y ^ 2 + t ^ 2) ^ k i
    + 2 * (k i : ℝ) * (2 * k i + 1) * x ^ p i * (y ^ 2 + t ^ 2) ^ (k i - 1))

theorem hasDerivAt_id_mul {f : ℝ → ℝ} {f' q : ℝ} (h : HasDerivAt f f' q) :
    HasDerivAt (fun r => r * f r) (f q + q * f') q := by
  have h2 := (hasDerivAt_id q).mul h
  have e : 1 * f q + id q * f' = f q + q * f' := by simp
  rw [e] at h2
  exact h2

theorem D1_eq (x y t : ℝ) :
    D1 σ d p k x y t
      = 2 * t * ∑ i ∈ σ, d i * x ^ p i * (k i : ℝ) * (y ^ 2 + t ^ 2) ^ (k i - 1) := by
  unfold D1
  rw [mul_sum]
  refine sum_congr rfl fun i _ => ?_
  ring

theorem D1_eq' (x y t : ℝ) :
    D1 σ d p k x t y
      = 2 * y * ∑ i ∈ σ, d i * x ^ p i * (k i : ℝ) * (y ^ 2 + t ^ 2) ^ (k i - 1) := by
  unfold D1
  rw [mul_sum]
  refine sum_congr rfl fun i _ => ?_
  ring

theorem N_eq (x y t : ℝ) :
    N σ d p k x y t
      = (∑ i ∈ σ, d i * ((p i : ℝ) * ((p i - 1 : ℕ) : ℝ) * x ^ (p i - 1 - 1) * (y ^ 2 + t ^ 2) ^ k i
          + 2 * (k i : ℝ) * (2 * k i - 1) * x ^ p i * (y ^ 2 + t ^ 2) ^ (k i - 1)))
        + 4 * ∑ i ∈ σ, d i * x ^ p i * (k i : ℝ) * (y ^ 2 + t ^ 2) ^ (k i - 1) := by
  unfold N
  rw [mul_sum, ← sum_add_distrib]
  refine sum_congr rfl fun i _ => ?_
  ring

theorem dt_VJ (x y t : ℝ) :
    dt (VJ σ d p k) x y t = V σ d p k x y t + t * D1 σ d p k x y t :=
  (hasDerivAt_id_mul (hasDerivAt_V_t σ d p k x y t)).deriv

theorem dt_VI (x y t : ℝ) : dt (VI σ d p k) x y t = y * D1 σ d p k x y t :=
  ((hasDerivAt_V_t σ d p k x y t).const_mul y).deriv

theorem laplacian_VJ (x y t : ℝ) :
    laplacian (VJ σ d p k) x y t
      = t * laplacian (V σ d p k) x y t + 2 * dt (V σ d p k) x y t := by
  have hx : (fun r => deriv (fun r' => VJ σ d p k r' y t) r) = fun r => t * E1 σ d p k r y t :=
    funext fun r => ((hasDerivAt_V_x σ d p k r y t).const_mul t).deriv
  have hy : (fun r => deriv (fun r' => VJ σ d p k x r' t) r) = fun r => t * D1 σ d p k x t r := by
    funext r
    rw [show (fun r' => VJ σ d p k x r' t) = fun r' => t * V σ d p k x t r' from
      funext fun r' => by rw [VJ, V_symm]]
    exact ((hasDerivAt_V_t σ d p k x t r).const_mul t).deriv
  have ht : (fun r => deriv (fun r' => VJ σ d p k x y r') r)
      = fun r => V σ d p k x y r + r * D1 σ d p k x y r :=
    funext fun r => (hasDerivAt_id_mul (hasDerivAt_V_t σ d p k x y r)).deriv
  have ht2 : HasDerivAt (fun r => V σ d p k x y r + r * D1 σ d p k x y r)
      (D1 σ d p k x y t + (D1 σ d p k x y t + t * D2 σ d p k x y t)) t :=
    (hasDerivAt_V_t σ d p k x y t).add (hasDerivAt_id_mul (hasDerivAt_D1_t σ d p k x y t))
  rw [laplacian_V, dt_V]
  unfold laplacian
  rw [hx, hy, ht, ((hasDerivAt_E1_x σ d p k x y t).const_mul t).deriv,
    ((hasDerivAt_D1_t σ d p k x t y).const_mul t).deriv, ht2.deriv]
  ring

theorem laplacian_VI (x y t : ℝ) :
    laplacian (VI σ d p k) x y t
      = y * laplacian (V σ d p k) x y t + 2 * D1 σ d p k x t y := by
  have hx : (fun r => deriv (fun r' => VI σ d p k r' y t) r) = fun r => y * E1 σ d p k r y t :=
    funext fun r => ((hasDerivAt_V_x σ d p k r y t).const_mul y).deriv
  have ht : (fun r => deriv (fun r' => VI σ d p k x y r') r) = fun r => y * D1 σ d p k x y r :=
    funext fun r => ((hasDerivAt_V_t σ d p k x y r).const_mul y).deriv
  have hy : (fun r => deriv (fun r' => VI σ d p k x r' t) r)
      = fun r => V σ d p k x t r + r * D1 σ d p k x t r := by
    funext r
    rw [show (fun r' => VI σ d p k x r' t) = fun r' => r' * V σ d p k x t r' from
      funext fun r' => by rw [VI, V_symm]]
    exact (hasDerivAt_id_mul (hasDerivAt_V_t σ d p k x t r)).deriv
  have hy2 : HasDerivAt (fun r => V σ d p k x t r + r * D1 σ d p k x t r)
      (D1 σ d p k x t y + (D1 σ d p k x t y + y * D2 σ d p k x t y)) y :=
    (hasDerivAt_V_t σ d p k x t y).add (hasDerivAt_id_mul (hasDerivAt_D1_t σ d p k x t y))
  rw [laplacian_V]
  unfold laplacian
  rw [hx, hy, ht, ((hasDerivAt_E1_x σ d p k x y t).const_mul y).deriv, hy2.deriv,
    ((hasDerivAt_D1_t σ d p k x y t).const_mul y).deriv]
  ring

/-- 📐 Defect of `v = y P (x, y² + t²)` in `t Δv = ∂ₜv`: it is `t y (P_xx + 4 r P_rr + 6 P_r)`. -/
theorem defect_VI (x y t : ℝ) :
    t * laplacian (VI σ d p k) x y t - dt (VI σ d p k) x y t = t * y * N σ d p k x y t := by
  rw [laplacian_VI, dt_VI]
  linear_combination y * laplaceBeltrami_defect_general σ d p k x y t + y * dt_V σ d p k x y t
    + 2 * t * D1_eq' σ d p k x y t - t * y * N_eq σ d p k x y t

/-- 📐 Defect of `w = t P (x, y² + t²)` in `t² Δw - t ∂ₜw + w = 0`: it is `t³ (P_xx + 4 r P_rr + 6 P_r)`. -/
theorem defect_VJ (x y t : ℝ) :
    t ^ 2 * laplacian (VJ σ d p k) x y t - t * dt (VJ σ d p k) x y t + VJ σ d p k x y t
      = t ^ 3 * N σ d p k x y t := by
  rw [laplacian_VJ, dt_VJ, VJ]
  linear_combination t ^ 2 * laplaceBeltrami_defect_general σ d p k x y t
    + 3 * t ^ 2 * dt_V σ d p k x y t + 2 * t ^ 2 * D1_eq σ d p k x y t
    - t ^ 3 * N_eq σ d p k x y t

end Components

/-- 🧱 The `i` component polynomial `y ∑ e_k x ^ (n - 1 - 2 k) (y² + t²) ^ k`. -/
def UI (n : ℕ) (e : ℕ → ℝ) : ℝ → ℝ → ℝ → ℝ :=
  VI (range ((n + 1) / 2)) e (fun k => n - 1 - 2 * k) id

/-- 🧱 The `j` component polynomial `t ∑ e_k x ^ (n - 1 - 2 k) (y² + t²) ^ k`. -/
def UJ (n : ℕ) (e : ℕ → ℝ) : ℝ → ℝ → ℝ → ℝ :=
  VJ (range ((n + 1) / 2)) e (fun k => n - 1 - 2 * k) id

/-- 🧾 `S_xx + 4 r S_rr + 6 S_r` for `S = ∑ e_k x ^ (n - 1 - 2 k) r ^ k`, coefficientwise in `r`. -/
def bS (n : ℕ) (e : ℕ → ℝ) (k : ℕ) : ℝ :=
  ((n - 1 - 2 * k : ℕ) : ℝ) * ((n - 1 - 2 * k - 1 : ℕ) : ℝ) * e k
    + (2 * k + 2) * (2 * k + 3) * e (k + 1)

theorem N_collect (n : ℕ) (e : ℕ → ℝ) (he : e ((n + 1) / 2) = 0) (x y t : ℝ) :
    N (range ((n + 1) / 2)) e (fun k => n - 1 - 2 * k) id x y t
      = ∑ k ∈ range ((n + 1) / 2), bS n e k * x ^ (n - 1 - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k := by
  unfold N bS
  simp only [id]
  obtain ⟨M, hM⟩ : ∃ M, (n + 1) / 2 = M := ⟨_, rfl⟩
  rw [hM] at he ⊢
  have hshift : ∑ k ∈ range M,
        2 * (k : ℝ) * (2 * k + 1) * e k * x ^ (n - 1 - 2 * k) * (y ^ 2 + t ^ 2) ^ (k - 1)
      = ∑ k ∈ range M, (2 * (k : ℝ) + 2) * (2 * k + 3) * e (k + 1)
          * x ^ (n - 1 - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k := by
    rcases M with _ | M
    · simp
    · rw [sum_range_succ', sum_range_succ _ M, he]
      simp only [Nat.cast_zero, mul_zero, zero_mul, add_zero, Nat.add_sub_cancel]
      refine sum_congr rfl fun k _ => ?_
      rw [show n - 1 - 2 * (k + 1) = n - 1 - 2 * k - 1 - 1 by omega]
      push_cast
      ring
  calc _ = ∑ k ∈ range M,
        (((n - 1 - 2 * k : ℕ) : ℝ) * ((n - 1 - 2 * k - 1 : ℕ) : ℝ) * e k
            * x ^ (n - 1 - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k
          + 2 * (k : ℝ) * (2 * k + 1) * e k * x ^ (n - 1 - 2 * k) * (y ^ 2 + t ^ 2) ^ (k - 1)) := by
        refine sum_congr rfl fun k _ => ?_
        ring
    _ = _ := by
        rw [sum_add_distrib, hshift, ← sum_add_distrib]
        refine sum_congr rfl fun k _ => ?_
        ring

/-- 🧹 A real polynomial expression vanishing for all `r > 1` has vanishing coefficients. -/
theorem coeff_eq_zero_of_forall (M : ℕ) (β : ℕ → ℝ)
    (h : ∀ r : ℝ, 1 < r → ∑ k ∈ range M, β k * r ^ k = 0) : ∀ k ∈ range M, β k = 0 := by
  let P : Polynomial ℝ := ∑ k ∈ range M, Polynomial.C (β k) * Polynomial.X ^ k
  have hroot : ∀ r : ℝ, 1 < r → P.IsRoot r := by
    intro r hr
    simp only [Polynomial.IsRoot, P, Polynomial.eval_finsetSum, Polynomial.eval_mul,
      Polynomial.eval_C, Polynomial.eval_pow, Polynomial.eval_X]
    exact h r hr
  have hP : P = 0 := Polynomial.eq_zero_of_infinite_isRoot P
    (Set.Infinite.mono (fun r hr => hroot r hr) (Set.Ioi_infinite (1 : ℝ)))
  intro k hk
  have hcoeff : P.coeff k = β k := by
    simp only [P, Polynomial.finsetSum_coeff, Polynomial.coeff_C_mul_X_pow]
    rw [sum_eq_single k (fun j _ hj => by simp [Ne.symm hj]) (fun hk' => absurd hk hk')]
    simp
  rw [← hcoeff, hP, Polynomial.coeff_zero]

theorem bS_eq_zero_iff (n : ℕ) (e : ℕ → ℝ) (he : ∀ k, (n + 1) / 2 ≤ k → e k = 0) :
    (∀ k ∈ range ((n + 1) / 2), bS n e k = 0)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 3) * (2 * k + 2) * e (k + 1)
          = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * e k := by
  have key : ∀ k, 2 * k + 1 ≤ n → ((n - 1 - 2 * k : ℕ) : ℝ) * ((n - 1 - 2 * k - 1 : ℕ) : ℝ)
      = ((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2) := by
    intro k hk
    rcases Nat.eq_zero_or_pos (n - 1 - 2 * k) with h0 | h0
    · have : (n : ℝ) = 2 * k + 1 := by exact_mod_cast (show n = 2 * k + 1 by omega)
      rw [h0, this]
      simp
    · rw [Nat.cast_sub (show 1 ≤ n - 1 - 2 * k from h0),
        Nat.cast_sub (show 2 * k ≤ n - 1 by omega), Nat.cast_sub (show 1 ≤ n by omega)]
      push_cast
      ring
  constructor
  · intro h k
    by_cases hk : k < (n + 1) / 2
    · have := h k (mem_range.mpr hk)
      unfold bS at this
      rw [key k (by omega)] at this
      linear_combination this
    · rw [he k (by omega), he (k + 1) (by omega)]
      ring
  · intro h k hk
    have hk' := mem_range.mp hk
    unfold bS
    rw [key k (by omega)]
    linear_combination h k

/-- 🏛️ `v = y S` solves `t Δv = ∂ₜv` identically iff the coefficients of `S` satisfy the recursion. -/
theorem laplaceBeltrami_imI_iff (n : ℕ) (e : ℕ → ℝ) (he : ∀ k, (n + 1) / 2 ≤ k → e k = 0) :
    (∀ x y t : ℝ, t * laplacian (UI n e) x y t = dt (UI n e) x y t)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 3) * (2 * k + 2) * e (k + 1)
          = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * e k := by
  rw [← bS_eq_zero_iff n e he]
  have hdefect : ∀ x y t : ℝ, t * laplacian (UI n e) x y t - dt (UI n e) x y t
      = t * y * ∑ k ∈ range ((n + 1) / 2),
          bS n e k * x ^ (n - 1 - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k := by
    intro x y t
    unfold UI
    rw [defect_VI, N_collect n e (he _ le_rfl)]
  constructor
  · intro h
    refine coeff_eq_zero_of_forall _ _ fun r hr => ?_
    have h1 := hdefect 1 1 (Real.sqrt (r - 1))
    rw [h 1 1 (Real.sqrt (r - 1)), sub_self] at h1
    have hs : Real.sqrt (r - 1) * 1 ≠ 0 := by
      rw [mul_one]
      exact (Real.sqrt_pos.mpr (by linarith)).ne'
    have h2 := (mul_eq_zero.mp h1.symm).resolve_left hs
    have hsq : 1 + Real.sqrt (r - 1) ^ 2 = r := by
      rw [Real.sq_sqrt (by linarith)]
      ring
    simp only [one_pow, mul_one, hsq] at h2
    exact h2
  · intro h x y t
    have h1 := hdefect x y t
    rw [sum_congr rfl fun k hk => by rw [h k hk, zero_mul, zero_mul]] at h1
    simp only [sum_const_zero, mul_zero] at h1
    linarith

/-- 🏛️ `w = t S` solves `t² Δw - t ∂ₜw + w = 0` identically iff the coefficients of `S` satisfy the recursion. -/
theorem hyperbolic_imJ_iff (n : ℕ) (e : ℕ → ℝ) (he : ∀ k, (n + 1) / 2 ≤ k → e k = 0) :
    (∀ x y t : ℝ, t ^ 2 * laplacian (UJ n e) x y t - t * dt (UJ n e) x y t + UJ n e x y t = 0)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 3) * (2 * k + 2) * e (k + 1)
          = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * e k := by
  rw [← bS_eq_zero_iff n e he]
  have hdefect : ∀ x y t : ℝ,
      t ^ 2 * laplacian (UJ n e) x y t - t * dt (UJ n e) x y t + UJ n e x y t
        = t ^ 3 * ∑ k ∈ range ((n + 1) / 2),
            bS n e k * x ^ (n - 1 - 2 * k - 1 - 1) * (y ^ 2 + t ^ 2) ^ k := by
    intro x y t
    unfold UJ
    rw [defect_VJ, N_collect n e (he _ le_rfl)]
  constructor
  · intro h
    refine coeff_eq_zero_of_forall _ _ fun r hr => ?_
    have h1 := hdefect 1 0 (Real.sqrt r)
    rw [h 1 0 (Real.sqrt r)] at h1
    have hs : Real.sqrt r ^ 3 ≠ 0 := pow_ne_zero 3 (Real.sqrt_pos.mpr (by linarith)).ne'
    have h2 := (mul_eq_zero.mp h1.symm).resolve_left hs
    have hsq : (0 : ℝ) ^ 2 + Real.sqrt r ^ 2 = r := by
      rw [Real.sq_sqrt (by linarith)]
      ring
    simp only [one_pow, mul_one, hsq] at h2
    exact h2
  · intro h x y t
    rw [hdefect x y t, sum_congr rfl fun k hk => by rw [h k hk, zero_mul, zero_mul]]
    simp

/-- 🎯 The recursion with `e_0 = n` determines the coefficients: they are those of `S_n`. -/
theorem eq_s_of_recursion (n : ℕ) (e : ℕ → ℝ) (h0 : e 0 = n)
    (h : ∀ k : ℕ, (2 * (k : ℝ) + 3) * (2 * k + 2) * e (k + 1)
      = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * e k) (k : ℕ) : e k = (s n k : ℝ) := by
  induction k with
  | zero => simp [h0, s_zero]
  | succ k ih =>
    have hc : (2 * (k : ℝ) + 3) * (2 * k + 2) * (s n (k + 1) : ℝ)
        = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * (s n k : ℝ) := by
      exact_mod_cast s_succ n k
    have hpos : (2 * (k : ℝ) + 3) * (2 * k + 2) ≠ 0 := by positivity
    apply mul_left_cancel₀ hpos
    rw [h k, hc, ih]

/-- 🌊 The `i` component of `z ^ n` solves `t Δv = ∂ₜv` for every exponent `n`. -/
theorem laplaceBeltrami_imI_pow (n : ℕ) (x y t : ℝ) :
    t * laplacian (fun x y t => (z x y t ^ n).imI) x y t
      = dt (fun x y t => (z x y t ^ n).imI) x y t := by
  have hU : (fun x y t => (z x y t ^ n).imI) = UI n (fun k => (s n k : ℝ)) := by
    funext x y t
    exact imI_pow x y t n
  rw [hU]
  refine (laplaceBeltrami_imI_iff n _ fun k hk => ?_).mpr (fun k => ?_) x y t
  · exact_mod_cast s_eq_zero (show n < 2 * k + 1 by omega)
  · exact_mod_cast s_succ n k

/-- 🌊 The `j` component of `z ^ n` solves `t² Δw - t ∂ₜw + w = 0` for every exponent `n`. -/
theorem hyperbolic_imJ_pow (n : ℕ) (x y t : ℝ) :
    t ^ 2 * laplacian (fun x y t => (z x y t ^ n).imJ) x y t
      - t * dt (fun x y t => (z x y t ^ n).imJ) x y t + (z x y t ^ n).imJ = 0 := by
  have hU : (fun x y t => (z x y t ^ n).imJ) = UJ n (fun k => (s n k : ℝ)) := by
    funext x y t
    exact imJ_pow x y t n
  have h := (hyperbolic_imJ_iff n (fun k => (s n k : ℝ)) fun k hk => by
    exact_mod_cast s_eq_zero (show n < 2 * k + 1 by omega)).mpr
      (fun k => by exact_mod_cast s_succ n k) x y t
  rw [hU, imJ_pow]
  exact h

/-! ### Clifford algebras -/

/-- 🧮 `R_n (x, r) = ∑ c_k x ^ (n - 2 k) r ^ k`. -/
def Rpoly (n : ℕ) (x r : ℝ) : ℝ :=
  ∑ k ∈ range (n / 2 + 1), (c n k : ℝ) * x ^ (n - 2 * k) * r ^ k

/-- 🧮 `S_n (x, r) = ∑ s_k x ^ (n - 1 - 2 k) r ^ k`. -/
def Spoly (n : ℕ) (x r : ℝ) : ℝ :=
  ∑ k ∈ range ((n + 1) / 2), (s n k : ℝ) * x ^ (n - 1 - 2 * k) * r ^ k

/-- 👑 In any real algebra, `w² = -r` implies `(x + w) ^ n = R_n (x, r) + S_n (x, r) w`. -/
theorem pow_eq_of_sq {A : Type*} [Ring A] [Algebra ℝ A] (w : A) (r : ℝ)
    (hw : w * w = algebraMap ℝ A (-r)) (x : ℝ) (n : ℕ) :
    (algebraMap ℝ A x + w) ^ n
      = algebraMap ℝ A (Rpoly n x r) + algebraMap ℝ A (Spoly n x r) * w := by
  have hc : Commute w (algebraMap ℝ A x) := (Algebra.commutes x w).symm
  have hev : ∀ (k : ℕ) (q : ℝ),
      w ^ (2 * k) * algebraMap ℝ A q = algebraMap ℝ A ((-r) ^ k * q) := by
    intro k q
    rw [pow_mul, pow_two, hw, ← map_pow, ← map_mul]
  have hodd : ∀ (k : ℕ) (q : ℝ),
      w ^ (2 * k + 1) * algebraMap ℝ A q = algebraMap ℝ A ((-r) ^ k * q) * w := by
    intro k q
    rw [pow_succ, pow_mul, pow_two, hw, ← map_pow, mul_assoc, ← Algebra.commutes q w, ← mul_assoc,
      ← map_mul]
  have hterm : ∀ i, w ^ i * algebraMap ℝ A x ^ (n - i) * (n.choose i : A)
      = w ^ i * algebraMap ℝ A (x ^ (n - i) * (n.choose i : ℝ)) := by
    intro i
    rw [mul_assoc, map_mul, map_pow, map_natCast]
  have hR : ∀ M, ∑ k ∈ range M, (-r) ^ k * (x ^ (n - 2 * k) * (n.choose (2 * k) : ℝ))
      = ∑ k ∈ range M, (c n k : ℝ) * x ^ (n - 2 * k) * r ^ k := by
    intro M
    refine sum_congr rfl fun k _ => ?_
    rw [neg_pow]
    simp only [c]
    push_cast
    ring
  have hS : ∀ M, ∑ k ∈ range M, (-r) ^ k * (x ^ (n - (2 * k + 1)) * (n.choose (2 * k + 1) : ℝ))
      = ∑ k ∈ range M, (s n k : ℝ) * x ^ (n - 1 - 2 * k) * r ^ k := by
    intro M
    refine sum_congr rfl fun k _ => ?_
    rw [show n - (2 * k + 1) = n - 1 - 2 * k by omega, neg_pow]
    simp only [s]
    push_cast
    ring
  rw [add_comm, hc.add_pow, sum_congr rfl fun i _ => hterm i]
  unfold Rpoly Spoly
  rcases Nat.even_or_odd' n with ⟨m, rfl | rfl⟩
  · rw [sum_range_parity, sum_congr rfl fun k _ => hev k _, sum_congr rfl fun k _ => hodd k _,
      ← map_sum, ← sum_mul, ← map_sum, hR, hS, show 2 * m / 2 = m by omega,
      show (2 * m + 1) / 2 = m by omega]
  · rw [sum_range_parity_odd, sum_congr rfl fun k _ => hev k _, sum_congr rfl fun k _ => hodd k _,
      ← map_sum, ← sum_mul, ← map_sum, hR, hS, show (2 * m + 1) / 2 = m by omega,
      show (2 * m + 1 + 1) / 2 = m + 1 by omega]

/-- 🧭 The quaternionic case as an instance: `z ^ n = R_n + S_n (i y + j t)`. -/
theorem z_pow_eq (x y t : ℝ) (n : ℕ) :
    z x y t ^ n = ((Rpoly n x (y ^ 2 + t ^ 2) : ℝ) : ℍ[ℝ])
      + ((Spoly n x (y ^ 2 + t ^ 2) : ℝ) : ℍ[ℝ]) * v y t := by
  rw [z_eq, add_comm]
  exact pow_eq_of_sq (v y t) (y ^ 2 + t ^ 2) (v_mul_self y t) x n

/-- 🟥 The negative definite form `-(v₁² + … + v_μ²)` whose Clifford algebra is `Cl_{0,μ}`. -/
noncomputable def Q (μ : ℕ) : QuadraticForm ℝ (Fin μ → ℝ) :=
  QuadraticMap.weightedSumSquares ℝ fun _ => (-1 : ℝ)

/-- 📏 `r = v₁² + … + v_μ²`. -/
def rr {μ : ℕ} (v : Fin μ → ℝ) : ℝ := ∑ j, v j ^ 2

theorem Q_apply {μ : ℕ} (v : Fin μ → ℝ) : Q μ v = -rr v := by
  simp [Q, rr, QuadraticMap.weightedSumSquares_apply, sq, Finset.sum_neg_distrib]

/-- 👑 Powers of the paravector `x + v` in `Cl_{0,μ}`. -/
theorem clifford_pow {μ : ℕ} (x : ℝ) (v : Fin μ → ℝ) (n : ℕ) :
    (algebraMap ℝ (CliffordAlgebra (Q μ)) x + CliffordAlgebra.ι (Q μ) v) ^ n
      = algebraMap ℝ _ (Rpoly n x (rr v))
        + algebraMap ℝ _ (Spoly n x (rr v)) * CliffordAlgebra.ι (Q μ) v :=
  pow_eq_of_sq (CliffordAlgebra.ι (Q μ) v) (rr v)
    (by rw [CliffordAlgebra.ι_sq_scalar, Q_apply]) x n

/-- 👑 Components: the coefficient of the generator `e_j` in `(x + v) ^ n` is `v_j S_n`. -/
theorem clifford_pow_components {μ : ℕ} (x : ℝ) (v : Fin μ → ℝ) (n : ℕ) :
    (algebraMap ℝ (CliffordAlgebra (Q μ)) x + CliffordAlgebra.ι (Q μ) v) ^ n
      = algebraMap ℝ _ (Rpoly n x (rr v))
        + ∑ j, (v j * Spoly n x (rr v)) • CliffordAlgebra.ι (Q μ) (Pi.single j (1 : ℝ)) := by
  have hv : v = ∑ j, v j • Pi.single j (1 : ℝ) := by
    ext l
    simp [Finset.sum_apply, Pi.single_apply]
  have hι : CliffordAlgebra.ι (Q μ) v
      = ∑ j, v j • CliffordAlgebra.ι (Q μ) (Pi.single j (1 : ℝ)) := by
    conv_lhs => rw [hv]
    rw [map_sum]
    exact sum_congr rfl fun j _ => map_smul _ _ _
  rw [clifford_pow]
  conv_lhs => rw [hι]
  rw [mul_sum]
  congr 1
  refine sum_congr rfl fun j _ => ?_
  rw [← Algebra.smul_def, smul_smul, mul_comm]

section Hyperbolic

variable {μ : ℕ} {τ : Type*} (σ : Finset τ) (d : τ → ℝ) (p k : τ → ℕ)

theorem rr_nonneg (v : Fin μ → ℝ) : 0 ≤ rr v := sum_nonneg fun _ _ => sq_nonneg _

theorem rr_eq_add (v : Fin μ → ℝ) (i : Fin μ) :
    rr v = v i ^ 2 + ∑ j ∈ univ.erase i, v j ^ 2 :=
  (Finset.add_sum_erase univ (fun j => v j ^ 2) (mem_univ i)).symm

theorem rr_update (v : Fin μ → ℝ) (i : Fin μ) (r : ℝ) :
    rr (Function.update v i r) = (rr v - v i ^ 2) + r ^ 2 := by
  have h : ∑ j ∈ univ.erase i, Function.update v i r j ^ 2 = ∑ j ∈ univ.erase i, v j ^ 2 :=
    sum_congr rfl fun j hj => by rw [Function.update_of_ne (ne_of_mem_erase hj)]
  rw [rr_eq_add (Function.update v i r) i, rr_eq_add v i, Function.update_self, h]
  ring

theorem rr_sub_nonneg (v : Fin μ → ℝ) (i : Fin μ) : 0 ≤ rr v - v i ^ 2 := by
  rw [rr_eq_add v i, add_sub_cancel_left]
  exact sum_nonneg fun j _ => sq_nonneg _

/-- 📐 The distance of `v` from the `i`-th coordinate axis. -/
noncomputable def qq (v : Fin μ → ℝ) (i : Fin μ) : ℝ := Real.sqrt (rr v - v i ^ 2)

theorem qq_sq (v : Fin μ → ℝ) (i : Fin μ) : qq v i ^ 2 + v i ^ 2 = rr v := by
  rw [qq, Real.sq_sqrt (rr_sub_nonneg v i)]
  ring

/-- 🧱 A general polynomial `P (x, v₁² + … + v_μ²)` on `ℝ × ℝ^μ`. -/
def F (x : ℝ) (v : Fin μ → ℝ) : ℝ := ∑ i ∈ σ, d i * x ^ p i * rr v ^ k i

/-- 🧱 `v_i P (x, |v|²)`. -/
def G (i : Fin μ) (x : ℝ) (v : Fin μ → ℝ) : ℝ := v i * F σ d p k x v

theorem F_update (x : ℝ) (v : Fin μ → ℝ) (i : Fin μ) (r : ℝ) :
    F σ d p k x (Function.update v i r) = V σ d p k x (qq v i) r := by
  unfold F V
  rw [rr_update, qq, Real.sq_sqrt (rr_sub_nonneg v i)]

theorem F_eq_zero_sqrt (x : ℝ) (v : Fin μ → ℝ) :
    F σ d p k x v = V σ d p k x 0 (Real.sqrt (rr v)) := by
  unfold F V
  rw [Real.sq_sqrt (rr_nonneg v)]
  simp

/-- ∂ Partial derivative in the coordinate `v_i`. -/
noncomputable def pd (u : ℝ → (Fin μ → ℝ) → ℝ) (i : Fin μ) (x : ℝ) (v : Fin μ → ℝ) : ℝ :=
  deriv (fun r => u x (Function.update v i r)) (v i)

/-- ∂² Second partial derivative in the coordinate `v_i`. -/
noncomputable def pd2 (u : ℝ → (Fin μ → ℝ) → ℝ) (i : Fin μ) (x : ℝ) (v : Fin μ → ℝ) : ℝ :=
  deriv (fun r => deriv (fun r' => u x (Function.update v i r')) r) (v i)

/-- ∂² Second partial derivative in `x`. -/
noncomputable def pdx2 (u : ℝ → (Fin μ → ℝ) → ℝ) (x : ℝ) (v : Fin μ → ℝ) : ℝ :=
  deriv (fun r => deriv (fun r' => u r' v) r) x

/-- Δ Euclidean Laplacian on `ℝ × ℝ^μ`. -/
noncomputable def lapN (u : ℝ → (Fin μ → ℝ) → ℝ) (x : ℝ) (v : Fin μ → ℝ) : ℝ :=
  pdx2 u x v + ∑ i, pd2 u i x v

/-- 🧾 `∑ d k x ^ p ρ ^ (k - 1) = P_r`. -/
def Kp (x ρ : ℝ) : ℝ := ∑ i ∈ σ, d i * x ^ p i * (k i : ℝ) * ρ ^ (k i - 1)

/-- 🧾 `P_xx + 4 r P_rr + (4 + 2 α) P_r`, monomialwise; `α = -1` and `α = 1` are the two brackets. -/
def Br (α x ρ : ℝ) : ℝ :=
  ∑ i ∈ σ, d i * ((p i : ℝ) * ((p i - 1 : ℕ) : ℝ) * x ^ (p i - 1 - 1) * ρ ^ k i
    + 2 * (k i : ℝ) * (2 * k i + α) * x ^ p i * ρ ^ (k i - 1))

theorem Br_eq (α x ρ : ℝ) :
    Br σ d p k α x ρ = Br σ d p k (-1) x ρ + (2 * α + 2) * Kp σ d p k x ρ := by
  unfold Br Kp
  rw [mul_sum, ← sum_add_distrib]
  refine sum_congr rfl fun i _ => ?_
  ring

theorem pd_F_eq_D1 (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    pd (F σ d p k) l x v = D1 σ d p k x (qq v l) (v l) := by
  unfold pd
  rw [show (fun r => F σ d p k x (Function.update v l r)) = fun r => V σ d p k x (qq v l) r from
    funext fun r => F_update σ d p k x v l r, (hasDerivAt_V_t σ d p k x (qq v l) (v l)).deriv]

theorem pd_F (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    pd (F σ d p k) l x v = 2 * v l * Kp σ d p k x (rr v) := by
  rw [pd_F_eq_D1, D1_eq, qq_sq]
  rfl

theorem pd2_F (i : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    pd2 (F σ d p k) i x v = D2 σ d p k x (qq v i) (v i) := by
  unfold pd2
  rw [show (fun r => deriv (fun r' => F σ d p k x (Function.update v i r')) r)
      = fun r => D1 σ d p k x (qq v i) r from funext fun r => by
    rw [show (fun r' => F σ d p k x (Function.update v i r')) = fun r' => V σ d p k x (qq v i) r'
      from funext fun r' => F_update σ d p k x v i r']
    exact (hasDerivAt_V_t σ d p k x (qq v i) r).deriv]
  exact (hasDerivAt_D1_t σ d p k x (qq v i) (v i)).deriv

theorem pdx2_F (x : ℝ) (v : Fin μ → ℝ) :
    pdx2 (F σ d p k) x v = E2 σ d p k x 0 (Real.sqrt (rr v)) := by
  unfold pdx2
  rw [show (fun r => deriv (fun r' => F σ d p k r' v) r)
      = fun r => E1 σ d p k r 0 (Real.sqrt (rr v)) from funext fun r => by
    rw [show (fun r' => F σ d p k r' v) = fun r' => V σ d p k r' 0 (Real.sqrt (rr v))
      from funext fun r' => F_eq_zero_sqrt σ d p k r' v]
    exact (hasDerivAt_V_x σ d p k r 0 (Real.sqrt (rr v))).deriv]
  exact (hasDerivAt_E1_x σ d p k x 0 (Real.sqrt (rr v))).deriv

theorem mono_term (j : ℕ) (ρ m : ℝ) :
    4 * (j : ℝ) * ((j - 1 : ℕ) : ℝ) * ρ ^ (j - 1 - 1) * ρ + 2 * m * (j : ℝ) * ρ ^ (j - 1)
      = (2 * (j : ℝ) * (2 * j + -1) + (2 * m - 2) * j) * ρ ^ (j - 1) := by
  rcases j with _ | _ | j
  · simp
  · simp
    ring
  · simp only [Nat.add_sub_cancel, show j + 1 + 1 - 1 = j + 1 by omega]
    push_cast
    ring

/-- Δ The Laplacian of `P (x, |v|²)` is `P_xx + 4 r P_rr + 2 μ P_r`. -/
theorem lapN_F (x : ℝ) (v : Fin μ → ℝ) :
    lapN (F σ d p k) x v
      = Br σ d p k (-1) x (rr v) + (2 * (μ : ℝ) - 2) * Kp σ d p k x (rr v) := by
  have hD2 : ∀ i : Fin μ, D2 σ d p k x (qq v i) (v i)
      = ∑ m ∈ σ, d m * x ^ p m
        * ((k m : ℝ) * (((k m - 1 : ℕ) : ℝ) * rr v ^ (k m - 1 - 1) * (2 * v i)) * (2 * v i)
          + (k m : ℝ) * rr v ^ (k m - 1) * 2) := by
    intro i
    unfold D2
    rw [qq_sq]
  have hmono : ∀ m ∈ σ, ∑ i : Fin μ, d m * x ^ p m
        * ((k m : ℝ) * (((k m - 1 : ℕ) : ℝ) * rr v ^ (k m - 1 - 1) * (2 * v i)) * (2 * v i)
          + (k m : ℝ) * rr v ^ (k m - 1) * 2)
      = d m * x ^ p m * (4 * (k m : ℝ) * ((k m - 1 : ℕ) : ℝ) * rr v ^ (k m - 1 - 1) * rr v
          + 2 * (μ : ℝ) * (k m : ℝ) * rr v ^ (k m - 1)) := by
    intro m _
    have e : ∑ i : Fin μ,
          (k m : ℝ) * (((k m - 1 : ℕ) : ℝ) * rr v ^ (k m - 1 - 1) * (2 * v i)) * (2 * v i)
        = 4 * (k m : ℝ) * ((k m - 1 : ℕ) : ℝ) * rr v ^ (k m - 1 - 1) * rr v := by
      unfold rr
      rw [mul_sum]
      exact sum_congr rfl fun i _ => by ring
    rw [← mul_sum, sum_add_distrib, e, sum_const, card_univ, Fintype.card_fin, nsmul_eq_mul]
    ring
  unfold lapN
  rw [pdx2_F, sum_congr rfl fun i _ => (pd2_F σ d p k i x v).trans (hD2 i), sum_comm,
    sum_congr rfl hmono]
  unfold E2 Br Kp
  rw [show (0 : ℝ) ^ 2 + Real.sqrt (rr v) ^ 2 = rr v by
    rw [Real.sq_sqrt (rr_nonneg v)]
    simp]
  rw [mul_sum, ← sum_add_distrib, ← sum_add_distrib]
  refine sum_congr rfl fun m _ => ?_
  linear_combination d m * x ^ p m * mono_term (k m) (rr v) (μ : ℝ)

theorem pd_G_of_ne {i l : Fin μ} (h : i ≠ l) (x : ℝ) (v : Fin μ → ℝ) :
    pd (G σ d p k i) l x v = v i * pd (F σ d p k) l x v := by
  unfold pd
  rw [show (fun r => G σ d p k i x (Function.update v l r))
      = fun r => v i * V σ d p k x (qq v l) r from funext fun r => by
    rw [G, Function.update_of_ne h, F_update],
    show (fun r => F σ d p k x (Function.update v l r)) = fun r => V σ d p k x (qq v l) r from
      funext fun r => F_update σ d p k x v l r,
    ((hasDerivAt_V_t σ d p k x (qq v l) (v l)).const_mul (v i)).deriv,
    (hasDerivAt_V_t σ d p k x (qq v l) (v l)).deriv]

theorem pd_G_self (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    pd (G σ d p k l) l x v = F σ d p k x v + v l * pd (F σ d p k) l x v := by
  have hF : F σ d p k x v = V σ d p k x (qq v l) (v l) := by
    rw [← F_update, Function.update_eq_self]
  unfold pd
  rw [show (fun r => G σ d p k l x (Function.update v l r))
      = fun r => r * V σ d p k x (qq v l) r from funext fun r => by
    rw [G, Function.update_self, F_update],
    show (fun r => F σ d p k x (Function.update v l r)) = fun r => V σ d p k x (qq v l) r from
      funext fun r => F_update σ d p k x v l r,
    (hasDerivAt_id_mul (hasDerivAt_V_t σ d p k x (qq v l) (v l))).deriv,
    (hasDerivAt_V_t σ d p k x (qq v l) (v l)).deriv, hF]

theorem pdx2_G (i : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    pdx2 (G σ d p k i) x v = v i * pdx2 (F σ d p k) x v := by
  rw [pdx2_F]
  unfold pdx2
  rw [show (fun r => deriv (fun r' => G σ d p k i r' v) r)
      = fun r => v i * E1 σ d p k r 0 (Real.sqrt (rr v)) from funext fun r => by
    rw [show (fun r' => G σ d p k i r' v) = fun r' => v i * V σ d p k r' 0 (Real.sqrt (rr v))
      from funext fun r' => by rw [G, F_eq_zero_sqrt]]
    exact ((hasDerivAt_V_x σ d p k r 0 (Real.sqrt (rr v))).const_mul (v i)).deriv]
  exact ((hasDerivAt_E1_x σ d p k x 0 (Real.sqrt (rr v))).const_mul (v i)).deriv

theorem pd2_G (i j : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    pd2 (G σ d p k i) j x v
      = v i * pd2 (F σ d p k) j x v + if j = i then 2 * pd (F σ d p k) i x v else 0 := by
  rw [pd2_F]
  by_cases h : j = i
  · subst h
    simp only [↓reduceIte]
    rw [pd_F_eq_D1]
    unfold pd2
    rw [show (fun r => deriv (fun r' => G σ d p k j x (Function.update v j r')) r)
        = fun r => V σ d p k x (qq v j) r + r * D1 σ d p k x (qq v j) r from funext fun r => by
      rw [show (fun r' => G σ d p k j x (Function.update v j r'))
          = fun r' => r' * V σ d p k x (qq v j) r' from funext fun r' => by
        rw [G, Function.update_self, F_update]]
      exact (hasDerivAt_id_mul (hasDerivAt_V_t σ d p k x (qq v j) r)).deriv]
    have h2 : HasDerivAt (fun r => V σ d p k x (qq v j) r + r * D1 σ d p k x (qq v j) r)
        (D1 σ d p k x (qq v j) (v j)
          + (D1 σ d p k x (qq v j) (v j) + v j * D2 σ d p k x (qq v j) (v j))) (v j) :=
      (hasDerivAt_V_t σ d p k x (qq v j) (v j)).add
        (hasDerivAt_id_mul (hasDerivAt_D1_t σ d p k x (qq v j) (v j)))
    rw [h2.deriv]
    ring
  · simp only [h, ↓reduceIte, add_zero]
    unfold pd2
    rw [show (fun r => deriv (fun r' => G σ d p k i x (Function.update v j r')) r)
        = fun r => v i * D1 σ d p k x (qq v j) r from funext fun r => by
      rw [show (fun r' => G σ d p k i x (Function.update v j r'))
          = fun r' => v i * V σ d p k x (qq v j) r' from funext fun r' => by
        rw [G, Function.update_of_ne (Ne.symm h), F_update]]
      exact ((hasDerivAt_V_t σ d p k x (qq v j) r).const_mul (v i)).deriv]
    exact ((hasDerivAt_D1_t σ d p k x (qq v j) (v j)).const_mul (v i)).deriv

theorem lapN_G (i : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    lapN (G σ d p k i) x v = v i * lapN (F σ d p k) x v + 2 * pd (F σ d p k) i x v := by
  unfold lapN
  rw [pdx2_G, sum_congr rfl fun j _ => pd2_G σ d p k i j x v, sum_add_distrib, ← mul_sum,
    sum_ite_eq' univ i]
  simp only [mem_univ, ↓reduceIte]
  ring

/-- 📐 Lemma: defect of `u = P (x, |v|²)` in `v_l Δu = (μ - 1) ∂_l u`. -/
theorem defectN_re (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    v l * lapN (F σ d p k) x v - ((μ : ℝ) - 1) * pd (F σ d p k) l x v
      = v l * Br σ d p k (-1) x (rr v) := by
  rw [lapN_F, pd_F]
  ring

/-- 📐 Lemma: defect of `v_i P`, `i ≠ l`, in the same equation. -/
theorem defectN_im {i l : Fin μ} (h : i ≠ l) (x : ℝ) (v : Fin μ → ℝ) :
    v l * lapN (G σ d p k i) x v - ((μ : ℝ) - 1) * pd (G σ d p k i) l x v
      = v l * v i * Br σ d p k 1 x (rr v) := by
  rw [lapN_G, pd_G_of_ne σ d p k h, lapN_F, pd_F, pd_F, Br_eq σ d p k 1]
  ring

/-- 📐 Lemma: defect of `v_l P` in `v_l² Δw - (μ - 1) v_l ∂_l w + (μ - 1) w = 0`. -/
theorem defectN_last (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    v l ^ 2 * lapN (G σ d p k l) x v - ((μ : ℝ) - 1) * v l * pd (G σ d p k l) l x v
        + ((μ : ℝ) - 1) * G σ d p k l x v
      = v l ^ 3 * Br σ d p k 1 x (rr v) := by
  rw [lapN_G, pd_G_self, lapN_F, pd_F, Br_eq σ d p k 1, G]
  ring

end Hyperbolic

/-- 🧹 Collecting `Br` by powers of `ρ` for `P = ∑ e_k x ^ (q - 2 k) ρ ^ k`. -/
theorem Br_collect (q M : ℕ) (e : ℕ → ℝ) (α : ℝ) (he : e M = 0) (x ρ : ℝ) :
    Br (range M) e (fun k => q - 2 * k) id α x ρ
      = ∑ k ∈ range M, (((q - 2 * k : ℕ) : ℝ) * ((q - 2 * k - 1 : ℕ) : ℝ) * e k
          + (2 * (k : ℝ) + 2) * (2 * k + 2 + α) * e (k + 1)) * x ^ (q - 2 * k - 1 - 1) * ρ ^ k := by
  unfold Br
  simp only [id]
  have hshift : ∑ k ∈ range M,
        2 * (k : ℝ) * (2 * k + α) * e k * x ^ (q - 2 * k) * ρ ^ (k - 1)
      = ∑ k ∈ range M, (2 * (k : ℝ) + 2) * (2 * k + 2 + α) * e (k + 1)
          * x ^ (q - 2 * k - 1 - 1) * ρ ^ k := by
    rcases M with _ | M
    · simp
    · rw [sum_range_succ', sum_range_succ _ M, he]
      simp only [Nat.cast_zero, mul_zero, zero_mul, add_zero, Nat.add_sub_cancel]
      refine sum_congr rfl fun k _ => ?_
      rw [show q - 2 * (k + 1) = q - 2 * k - 1 - 1 by omega]
      push_cast
      ring
  calc _ = ∑ k ∈ range M,
        (((q - 2 * k : ℕ) : ℝ) * ((q - 2 * k - 1 : ℕ) : ℝ) * e k * x ^ (q - 2 * k - 1 - 1) * ρ ^ k
          + 2 * (k : ℝ) * (2 * k + α) * e k * x ^ (q - 2 * k) * ρ ^ (k - 1)) := by
        refine sum_congr rfl fun k _ => ?_
        ring
    _ = _ := by
        rw [sum_add_distrib, hshift, ← sum_add_distrib]
        refine sum_congr rfl fun k _ => ?_
        ring

section HyperbolicPowers

variable {μ : ℕ}

/-- 🧱 The scalar part polynomial on `ℝ × ℝ^μ`. -/
def FR (n : ℕ) (e : ℕ → ℝ) : ℝ → (Fin μ → ℝ) → ℝ :=
  F (range (n / 2 + 1)) e (fun k => n - 2 * k) id

/-- 🧱 The `e_i` component polynomial on `ℝ × ℝ^μ`. -/
def GS (n : ℕ) (e : ℕ → ℝ) (i : Fin μ) : ℝ → (Fin μ → ℝ) → ℝ :=
  G (range ((n + 1) / 2)) e (fun k => n - 1 - 2 * k) id i

theorem rr_axis (l : Fin μ) (r : ℝ) (hr : 0 ≤ r) :
    rr (Function.update (0 : Fin μ → ℝ) l (Real.sqrt r)) = r := by
  rw [rr_update, Real.sq_sqrt hr]
  simp [rr]

theorem rr_plane {i l : Fin μ} (h : i ≠ l) (r : ℝ) (hr : 1 ≤ r) :
    rr (Function.update (Function.update (0 : Fin μ → ℝ) i 1) l (Real.sqrt (r - 1))) = r := by
  rw [rr_update, rr_update, Function.update_of_ne (Ne.symm h), Real.sq_sqrt (by linarith)]
  simp [rr]

theorem FR_defect (n : ℕ) (e : ℕ → ℝ) (he : e (n / 2 + 1) = 0) (l : Fin μ) (x : ℝ)
    (v : Fin μ → ℝ) :
    v l * lapN (FR n e) x v - ((μ : ℝ) - 1) * pd (FR n e) l x v
      = v l * ∑ k ∈ range (n / 2 + 1), b n e k * x ^ (n - 2 * k - 1 - 1) * rr v ^ k := by
  unfold FR
  rw [defectN_re, Br_collect n _ e (-1) he]
  congr 1
  refine sum_congr rfl fun k _ => ?_
  unfold b
  ring

theorem GS_collect (n : ℕ) (e : ℕ → ℝ) (he : e ((n + 1) / 2) = 0) (x ρ : ℝ) :
    Br (range ((n + 1) / 2)) e (fun k => n - 1 - 2 * k) id 1 x ρ
      = ∑ k ∈ range ((n + 1) / 2), bS n e k * x ^ (n - 1 - 2 * k - 1 - 1) * ρ ^ k := by
  rw [Br_collect (n - 1) _ e 1 he]
  refine sum_congr rfl fun k _ => ?_
  unfold bS
  ring

/-- 🏛️ In dimension `μ + 1`: `u = ∑ e_k x ^ (n - 2 k) |v|^(2 k)` solves `v_l Δu = (μ - 1) ∂_l u` iff the recursion holds. -/
theorem hyperbolic_re_iff (n : ℕ) (e : ℕ → ℝ) (he : ∀ k, n / 2 < k → e k = 0) (l : Fin μ) :
    (∀ (x : ℝ) (v : Fin μ → ℝ), v l * lapN (FR n e) x v = ((μ : ℝ) - 1) * pd (FR n e) l x v)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 2) * (2 * k + 1) * e (k + 1)
          = -(((n : ℝ) - 2 * k) * ((n : ℝ) - 2 * k - 1)) * e k := by
  rw [← b_eq_zero_iff n e he]
  have hdefect := FR_defect (μ := μ) n e (he _ (Nat.lt_succ_self _)) l
  constructor
  · intro h
    refine coeff_eq_zero_of_forall _ _ fun r hr => ?_
    have h1 := hdefect 1 (Function.update 0 l (Real.sqrt r))
    rw [h 1 _, sub_self, Function.update_self, rr_axis l r (by linarith)] at h1
    have hs : Real.sqrt r ≠ 0 := (Real.sqrt_pos.mpr (by linarith)).ne'
    have h2 := (mul_eq_zero.mp h1.symm).resolve_left hs
    simp only [one_pow, mul_one] at h2
    exact h2
  · intro h x v
    have h1 := hdefect x v
    rw [sum_congr rfl fun k hk => by rw [h k hk, zero_mul, zero_mul]] at h1
    simp only [sum_const_zero, mul_zero] at h1
    linarith

/-- 🏛️ In dimension `μ + 1`: `v_i S`, `i ≠ l`, solves `v_l Δu = (μ - 1) ∂_l u` iff the recursion holds. -/
theorem hyperbolic_im_iff (n : ℕ) (e : ℕ → ℝ) (he : ∀ k, (n + 1) / 2 ≤ k → e k = 0)
    {i l : Fin μ} (hil : i ≠ l) :
    (∀ (x : ℝ) (v : Fin μ → ℝ),
        v l * lapN (GS n e i) x v = ((μ : ℝ) - 1) * pd (GS n e i) l x v)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 3) * (2 * k + 2) * e (k + 1)
          = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * e k := by
  rw [← bS_eq_zero_iff n e he]
  have hdefect : ∀ (x : ℝ) (v : Fin μ → ℝ),
      v l * lapN (GS n e i) x v - ((μ : ℝ) - 1) * pd (GS n e i) l x v
        = v l * v i * ∑ k ∈ range ((n + 1) / 2),
            bS n e k * x ^ (n - 1 - 2 * k - 1 - 1) * rr v ^ k := by
    intro x v
    unfold GS
    rw [defectN_im _ _ _ _ hil, GS_collect n e (he _ le_rfl)]
  constructor
  · intro h
    refine coeff_eq_zero_of_forall _ _ fun r hr => ?_
    have h1 := hdefect 1 (Function.update (Function.update 0 i 1) l (Real.sqrt (r - 1)))
    rw [h 1 _, sub_self, Function.update_self, Function.update_of_ne hil, Function.update_self,
      rr_plane hil r hr.le] at h1
    have hs : Real.sqrt (r - 1) * 1 ≠ 0 := by
      rw [mul_one]
      exact (Real.sqrt_pos.mpr (by linarith)).ne'
    have h2 := (mul_eq_zero.mp h1.symm).resolve_left hs
    simp only [one_pow, mul_one] at h2
    exact h2
  · intro h x v
    have h1 := hdefect x v
    rw [sum_congr rfl fun k hk => by rw [h k hk, zero_mul, zero_mul]] at h1
    simp only [sum_const_zero, mul_zero] at h1
    linarith

/-- 🏛️ In dimension `μ + 1`: `w = v_l S` solves `v_l² Δw - (μ - 1) v_l ∂_l w + (μ - 1) w = 0` iff the recursion holds. -/
theorem hyperbolic_last_iff (n : ℕ) (e : ℕ → ℝ) (he : ∀ k, (n + 1) / 2 ≤ k → e k = 0)
    (l : Fin μ) :
    (∀ (x : ℝ) (v : Fin μ → ℝ),
        v l ^ 2 * lapN (GS n e l) x v - ((μ : ℝ) - 1) * v l * pd (GS n e l) l x v
          + ((μ : ℝ) - 1) * GS n e l x v = 0)
      ↔ ∀ k : ℕ, (2 * (k : ℝ) + 3) * (2 * k + 2) * e (k + 1)
          = -(((n : ℝ) - 2 * k - 1) * ((n : ℝ) - 2 * k - 2)) * e k := by
  rw [← bS_eq_zero_iff n e he]
  have hdefect : ∀ (x : ℝ) (v : Fin μ → ℝ),
      v l ^ 2 * lapN (GS n e l) x v - ((μ : ℝ) - 1) * v l * pd (GS n e l) l x v
          + ((μ : ℝ) - 1) * GS n e l x v
        = v l ^ 3 * ∑ k ∈ range ((n + 1) / 2),
            bS n e k * x ^ (n - 1 - 2 * k - 1 - 1) * rr v ^ k := by
    intro x v
    unfold GS
    rw [defectN_last, GS_collect n e (he _ le_rfl)]
  constructor
  · intro h
    refine coeff_eq_zero_of_forall _ _ fun r hr => ?_
    have h1 := hdefect 1 (Function.update 0 l (Real.sqrt r))
    rw [h 1 _, Function.update_self, rr_axis l r (by linarith)] at h1
    have hs : Real.sqrt r ^ 3 ≠ 0 := pow_ne_zero 3 (Real.sqrt_pos.mpr (by linarith)).ne'
    have h2 := (mul_eq_zero.mp h1.symm).resolve_left hs
    simp only [one_pow, mul_one] at h2
    exact h2
  · intro h x v
    rw [hdefect x v, sum_congr rfl fun k hk => by rw [h k hk, zero_mul, zero_mul]]
    simp

/-- 🌊 The scalar part `R_n (x, |v|²)` of `(x + v) ^ n` in `Cl_{0,μ}` solves `v_l Δu = (μ - 1) ∂_l u`. -/
theorem hyperbolic_scalar_pow (n : ℕ) (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    v l * lapN (fun x v => Rpoly n x (rr v)) x v
      = ((μ : ℝ) - 1) * pd (fun x (v : Fin μ → ℝ) => Rpoly n x (rr v)) l x v := by
  have hU : (fun x (v : Fin μ → ℝ) => Rpoly n x (rr v)) = FR n (fun k => (c n k : ℝ)) := rfl
  rw [hU]
  refine (hyperbolic_re_iff n _ (fun k hk => ?_) l).mpr (fun k => ?_) x v
  · exact_mod_cast c_eq_zero (show n < 2 * k by omega)
  · exact_mod_cast c_succ n k

/-- 🌊 The coefficient `v_i S_n` of `e_i`, `i ≠ l`, solves the same equation. -/
theorem hyperbolic_vector_pow (n : ℕ) {i l : Fin μ} (hil : i ≠ l) (x : ℝ) (v : Fin μ → ℝ) :
    v l * lapN (fun x v => v i * Spoly n x (rr v)) x v
      = ((μ : ℝ) - 1) * pd (fun x (v : Fin μ → ℝ) => v i * Spoly n x (rr v)) l x v := by
  have hU : (fun x (v : Fin μ → ℝ) => v i * Spoly n x (rr v)) = GS n (fun k => (s n k : ℝ)) i :=
    rfl
  rw [hU]
  refine (hyperbolic_im_iff n _ (fun k hk => ?_) hil).mpr (fun k => ?_) x v
  · exact_mod_cast s_eq_zero (show n < 2 * k + 1 by omega)
  · exact_mod_cast s_succ n k

/-- 🌊 The coefficient `w = v_l S_n` of `e_l` solves `v_l² Δw - (μ - 1) v_l ∂_l w + (μ - 1) w = 0`. -/
theorem hyperbolic_last_pow (n : ℕ) (l : Fin μ) (x : ℝ) (v : Fin μ → ℝ) :
    v l ^ 2 * lapN (fun x v => v l * Spoly n x (rr v)) x v
        - ((μ : ℝ) - 1) * v l * pd (fun x (v : Fin μ → ℝ) => v l * Spoly n x (rr v)) l x v
        + ((μ : ℝ) - 1) * (v l * Spoly n x (rr v)) = 0 := by
  have hU : (fun x (v : Fin μ → ℝ) => v l * Spoly n x (rr v)) = GS n (fun k => (s n k : ℝ)) l :=
    rfl
  have h := (hyperbolic_last_iff (μ := μ) n (fun k => (s n k : ℝ)) (fun k hk => by
    exact_mod_cast s_eq_zero (show n < 2 * k + 1 by omega)) l).mpr
      (fun k => by exact_mod_cast s_succ n k) x v
  rw [hU]
  exact h

end HyperbolicPowers

/-! ### Examples -/

/-- 🧪 The coefficients of `Re (z ^ 10)` from the source notes. -/
theorem a_five : (List.range 6).map (a 5) = [1, -45, 210, -210, 45, -1] := by decide

/-- 🧪 The coefficients of `Re (z ^ 8)` from the source notes. -/
theorem a_four : (List.range 5).map (a 4) = [1, -28, 70, -28, 1] := by decide

/-- 🧪 The coefficients of `S_6`. -/
theorem s_six : (List.range 3).map (s 6) = [6, -20, 6] := by decide

/-- 🧪 `R_1, …, R_6`. -/
theorem Rpoly_examples (x r : ℝ) :
    Rpoly 1 x r = x ∧ Rpoly 2 x r = x ^ 2 - r ∧ Rpoly 3 x r = x ^ 3 - 3 * x * r
      ∧ Rpoly 4 x r = x ^ 4 - 6 * x ^ 2 * r + r ^ 2
      ∧ Rpoly 5 x r = x ^ 5 - 10 * x ^ 3 * r + 5 * x * r ^ 2
      ∧ Rpoly 6 x r = x ^ 6 - 15 * x ^ 4 * r + 15 * x ^ 2 * r ^ 2 - r ^ 3 := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩ <;>
    norm_num [Rpoly, c, Finset.sum_range_succ, Nat.choose] <;> ring

/-- 🧪 `S_1, …, S_6`. -/
theorem Spoly_examples (x r : ℝ) :
    Spoly 1 x r = 1 ∧ Spoly 2 x r = 2 * x ∧ Spoly 3 x r = 3 * x ^ 2 - r
      ∧ Spoly 4 x r = 4 * x ^ 3 - 4 * x * r
      ∧ Spoly 5 x r = 5 * x ^ 4 - 10 * x ^ 2 * r + r ^ 2
      ∧ Spoly 6 x r = 6 * x ^ 5 - 20 * x ^ 3 * r + 6 * x * r ^ 2 := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩ <;>
    norm_num [Spoly, s, Finset.sum_range_succ, Nat.choose] <;> ring

/-- 🧪 `(1 + 2 i + 3 j) ^ 3 = -38 - 20 i - 30 j`. -/
theorem z_cube_example : z 1 2 3 ^ 3 = ⟨-38, -20, -30, 0⟩ := by
  rw [z_pow_eq, show (2 : ℝ) ^ 2 + 3 ^ 2 = 13 by norm_num, (Rpoly_examples 1 13).2.2.1,
    (Spoly_examples 1 13).2.2.1, show (1 : ℝ) ^ 3 - 3 * 1 * 13 = -38 by norm_num,
    show 3 * (1 : ℝ) ^ 2 - 13 = -10 by norm_num]
  ext <;> simp [v] <;> norm_num

/-! ### Roots of polynomials -/

/-- 🧮 The polynomial `∑ q ^ n a_n` with quaternionic coefficients on the right. -/
def evalPoly (a : ℕ → ℍ[ℝ]) (N : ℕ) (q : ℍ[ℝ]) : ℍ[ℝ] := ∑ n ∈ range (N + 1), q ^ n * a n

/-- 🧮 `∑ R_n (x, r) a_n`. -/
def polyA (a : ℕ → ℍ[ℝ]) (N : ℕ) (x r : ℝ) : ℍ[ℝ] :=
  ∑ n ∈ range (N + 1), ((Rpoly n x r : ℝ) : ℍ[ℝ]) * a n

/-- 🧮 `∑ S_n (x, r) a_n`. -/
def polyB (a : ℕ → ℍ[ℝ]) (N : ℕ) (x r : ℝ) : ℍ[ℝ] :=
  ∑ n ∈ range (N + 1), ((Spoly n x r : ℝ) : ℍ[ℝ]) * a n

/-- 🗺️ The `xy`-plane `ℂ_i` inside the quaternions. -/
def InPlane (q : ℍ[ℝ]) : Prop := q.imJ = 0 ∧ q.imK = 0

/-- ✂️ `p (x + v) = A (x, |v|²) + v B (x, |v|²)`. -/
theorem evalPoly_z (a : ℕ → ℍ[ℝ]) (N : ℕ) (x y t : ℝ) :
    evalPoly a N (z x y t)
      = polyA a N x (y ^ 2 + t ^ 2) + v y t * polyB a N x (y ^ 2 + t ^ 2) := by
  unfold evalPoly polyA polyB
  rw [mul_sum, ← sum_add_distrib]
  refine sum_congr rfl fun n _ => ?_
  rw [z_pow_eq, add_mul, Quaternion.coe_commutes (Spoly n x (y ^ 2 + t ^ 2)) (v y t), mul_assoc]

theorem polyA_inPlane {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlane (a n)) (N : ℕ) (x r : ℝ) :
    InPlane (polyA a N x r) := by
  unfold polyA
  refine ⟨?_, ?_⟩
  · rw [imJ_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).1]
  · rw [imK_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).2]

theorem polyB_inPlane {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlane (a n)) (N : ℕ) (x r : ℝ) :
    InPlane (polyB a N x r) := by
  unfold polyB
  refine ⟨?_, ?_⟩
  · rw [imJ_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).1]
  · rw [imK_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).2]

/-- 🔑 A root outside the `xy`-plane of a polynomial with coefficients in the `xy`-plane kills `A` and `B`. -/
theorem polyA_polyB_eq_zero {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlane (a n)) {N : ℕ} {x y t : ℝ}
    (h : evalPoly a N (z x y t) = 0) (ht : t ≠ 0) :
    polyA a N x (y ^ 2 + t ^ 2) = 0 ∧ polyB a N x (y ^ 2 + t ^ 2) = 0 := by
  rw [evalPoly_z] at h
  obtain ⟨hAJ, hAK⟩ := polyA_inPlane ha N x (y ^ 2 + t ^ 2)
  obtain ⟨hBJ, hBK⟩ := polyB_inPlane ha N x (y ^ 2 + t ^ 2)
  generalize polyA a N x (y ^ 2 + t ^ 2) = A at h hAJ hAK ⊢
  generalize polyB a N x (y ^ 2 + t ^ 2) = B at h hBJ hBK ⊢
  have hJ : t * B.re = 0 := by
    have := congrArg (fun q : ℍ[ℝ] => q.imJ) h
    simpa [v, hAJ, hBJ, hBK] using this
  have hK : t * B.imI = 0 := by
    have := congrArg (fun q : ℍ[ℝ] => q.imK) h
    simpa [v, hAK, hBJ, hBK] using this
  have hB : B = 0 := by
    ext
    · simpa using (mul_eq_zero.mp hJ).resolve_left ht
    · simpa using (mul_eq_zero.mp hK).resolve_left ht
    · simpa using hBJ
    · simpa using hBK
  rw [hB, mul_zero, add_zero] at h
  exact ⟨h, hB⟩

/-- 🏛️ Coefficients in the `xy`-plane: every root outside the plane comes with its whole circle `y² + t² = const`. -/
theorem circle_of_roots {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlane (a n)) {N : ℕ} {x y t : ℝ}
    (h : evalPoly a N (z x y t) = 0) (ht : t ≠ 0) {y' t' : ℝ}
    (hr : y' ^ 2 + t' ^ 2 = y ^ 2 + t ^ 2) : evalPoly a N (z x y' t') = 0 := by
  obtain ⟨hA, hB⟩ := polyA_polyB_eq_zero ha h ht
  rw [evalPoly_z, hr, hA, hB, mul_zero, add_zero]

/-- 🏛️ Coefficients in the `xy`-plane: no root outside the plane is isolated. -/
theorem root_not_isolated {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlane (a n)) {N : ℕ} {x y t : ℝ}
    (h : evalPoly a N (z x y t) = 0) (ht : t ≠ 0) {ε : ℝ} (hε : 0 < ε) :
    ∃ y' t' : ℝ, (y', t') ≠ (y, t) ∧ (y' - y) ^ 2 + (t' - t) ^ 2 < ε ^ 2
      ∧ evalPoly a N (z x y' t') = 0 := by
  have hρ : 0 < y ^ 2 + t ^ 2 := by positivity
  set M : ℝ := |y| + |t| + 1 with hM
  have hMpos : 0 < M := by positivity
  have hρM : y ^ 2 + t ^ 2 < M ^ 2 := by
    have h1 : y ^ 2 = |y| ^ 2 := (sq_abs y).symm
    have h2 : t ^ 2 = |t| ^ 2 := (sq_abs t).symm
    rw [hM, h1, h2]
    nlinarith [abs_nonneg y, abs_nonneg t]
  set θ : ℝ := min 1 (ε / M) with hθ
  have hθpos : 0 < θ := lt_min one_pos (div_pos hε hMpos)
  have hθ1 : θ ≤ 1 := min_le_left _ _
  have hθε : θ ≤ ε / M := min_le_right _ _
  have hcs := Real.cos_sq_add_sin_sq θ
  have hcos1 : Real.cos θ ≠ 1 := by
    intro hc
    have := (Real.cos_eq_one_iff_of_lt_of_lt (by linarith [Real.pi_gt_three])
      (by linarith [Real.pi_gt_three])).mp hc
    linarith
  have hcosle : Real.cos θ ≤ 1 := Real.cos_le_one θ
  have hcoslt : Real.cos θ < 1 := lt_of_le_of_ne hcosle hcos1
  have hbound : 1 - Real.cos θ ≤ θ ^ 2 / 2 := by
    have := Real.one_sub_sq_div_two_le_cos (x := θ)
    linarith
  have hdist : (y * Real.cos θ - t * Real.sin θ - y) ^ 2 + (y * Real.sin θ + t * Real.cos θ - t) ^ 2
      = 2 * (y ^ 2 + t ^ 2) * (1 - Real.cos θ) := by
    linear_combination (y ^ 2 + t ^ 2) * hcs
  refine ⟨y * Real.cos θ - t * Real.sin θ, y * Real.sin θ + t * Real.cos θ, ?_, ?_, ?_⟩
  · intro heq
    have h1 := (Prod.mk.injEq _ _ _ _ ▸ heq : _ ∧ _).1
    have h2 := (Prod.mk.injEq _ _ _ _ ▸ heq : _ ∧ _).2
    have hzero : 2 * (y ^ 2 + t ^ 2) * (1 - Real.cos θ) = 0 := by
      rw [← hdist, h1, h2]
      ring
    have : 0 < 2 * (y ^ 2 + t ^ 2) * (1 - Real.cos θ) := by
      have : 0 < 1 - Real.cos θ := by linarith
      positivity
    linarith
  · rw [hdist]
    have hθM : θ * M ≤ ε := by
      have := mul_le_mul_of_nonneg_right hθε hMpos.le
      rwa [div_mul_cancel₀ _ hMpos.ne'] at this
    have h3 : (θ * M) ^ 2 ≤ ε ^ 2 := pow_le_pow_left₀ (by positivity) hθM 2
    have h4 : (y ^ 2 + t ^ 2) * θ ^ 2 < M ^ 2 * θ ^ 2 :=
      mul_lt_mul_of_pos_right hρM (by positivity)
    nlinarith
  · refine circle_of_roots ha h ht ?_
    linear_combination (y ^ 2 + t ^ 2) * hcs

/-- 🏛️ Real coefficients, degree two: the roots are the real roots and whole circles. -/
theorem real_quadratic_root_iff (b c x y t : ℝ) :
    z x y t ^ 2 + (b : ℍ[ℝ]) * z x y t + (c : ℍ[ℝ]) = 0
      ↔ x ^ 2 - (y ^ 2 + t ^ 2) + b * x + c = 0 ∧ (2 * x + b) * y = 0 ∧ (2 * x + b) * t = 0 := by
  have h2 : z x y t ^ 2 = ⟨x ^ 2 - y ^ 2 - t ^ 2, 2 * x * y, 2 * x * t, 0⟩ := by
    rw [pow_two]
    ext <;> simp [z] <;> ring
  rw [h2, Quaternion.ext_iff]
  simp only [z, Quaternion.re_add, Quaternion.imI_add, Quaternion.imJ_add, Quaternion.imK_add,
    re_coe_mul, Quaternion.re_coe, Quaternion.imI_coe, Quaternion.imJ_coe, Quaternion.imK_coe,
    Quaternion.re_zero, Quaternion.imI_zero, Quaternion.imJ_zero, Quaternion.imK_zero]
  simp
  have e : ∀ w : ℝ, 2 * x * w + b * w = 0 ↔ 2 * x + b = 0 ∨ w = 0 := fun w => by
    rw [← mul_eq_zero]
    constructor <;> intro h <;> linear_combination h
  constructor
  · rintro ⟨h1, h2, h3⟩
    exact ⟨by linarith, (e y).mp h2, (e t).mp h3⟩
  · rintro ⟨h1, h2, h3⟩
    exact ⟨by linarith, (e y).mpr h2, (e t).mpr h3⟩

/-- 🧪 A quadratic polynomial with a coefficient outside the `xy`-plane: `q² - q (1 + j) + j`. -/
def jPoly (q : ℍ[ℝ]) : ℍ[ℝ] := q ^ 2 - q * ⟨1, 0, 1, 0⟩ + ⟨0, 0, 1, 0⟩

/-- 🏛️ Its only roots in all of `ℍ` are `1` and `j`, so `j`, which lies outside the `xy`-plane, is an isolated root. -/
theorem jPoly_root_iff (q : ℍ[ℝ]) : jPoly q = 0 ↔ q = 1 ∨ q = ⟨0, 0, 1, 0⟩ := by
  constructor
  · intro h
    obtain ⟨a, b, c, d⟩ := q
    unfold jPoly at h
    rw [pow_two] at h
    have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
    have h2 := congrArg (fun q : ℍ[ℝ] => q.imI) h
    have h3 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
    have h4 := congrArg (fun q : ℍ[ℝ] => q.imK) h
    simp at h1 h2 h3 h4
    have hb : b = 0 := by
      have hb' : b * ((2 * a - 1) ^ 2 + 1) = 0 := by linear_combination (2 * a - 1) * h2 - h4
      exact (mul_eq_zero.mp hb').resolve_right (by positivity)
    have hd : d = 0 := by
      have hd' : d * ((2 * a - 1) ^ 2 + 1) = 0 := by linear_combination (2 * a - 1) * h4 + h2
      exact (mul_eq_zero.mp hd').resolve_right (by positivity)
    subst hb hd
    have hac : (a - c) * (a + c - 1) = 0 := by linear_combination h1
    rcases mul_eq_zero.mp hac with hac | hac
    · have : c = a := by linarith
      subst this
      nlinarith [sq_nonneg (2 * c - 1)]
    · have hc : c = 1 - a := by linarith
      subst hc
      have ha : a * (a - 1) = 0 := by linear_combination (-1 / 2 : ℝ) * h3
      rcases mul_eq_zero.mp ha with ha | ha
      · right
        subst ha
        ext <;> simp
      · left
        have : a = 1 := by linarith
        subst this
        ext <;> simp
  · rintro (rfl | rfl)
    · unfold jPoly
      ext <;> simp
    · unfold jPoly
      rw [pow_two]
      ext <;> simp

/-- 🧪 The isolated root `j = z 0 0 1` has `t = 1 ≠ 0`. -/
theorem jPoly_root_outside_plane : jPoly (z 0 0 1) = 0 ∧ ¬ InPlane (z 0 0 1) := by
  refine ⟨(jPoly_root_iff _).mpr (Or.inr rfl), ?_⟩
  unfold InPlane z
  simp

/-! ### Roots: degree three and general degree -/

/-- 🧲 A purely imaginary quaternion `b i + c j + d ij`. -/
def w3 (b c d : ℝ) : ℍ[ℝ] := ⟨0, b, c, d⟩

theorem w3_mul_self (b c d : ℝ) :
    w3 b c d * w3 b c d = ((-(b ^ 2 + c ^ 2 + d ^ 2) : ℝ) : ℍ[ℝ]) := by
  ext <;> simp [w3, -Quaternion.coe_pow] <;> ring

theorem quat_eq (a b c d : ℝ) : (⟨a, b, c, d⟩ : ℍ[ℝ]) = (a : ℍ[ℝ]) + w3 b c d := by
  ext <;> simp [w3]

/-- 👑 Every quaternion power: `(x + w) ^ n = R_n (x, |w|²) + S_n (x, |w|²) w`. -/
theorem quat_pow_eq (x b c d : ℝ) (n : ℕ) :
    ((x : ℍ[ℝ]) + w3 b c d) ^ n
      = ((Rpoly n x (b ^ 2 + c ^ 2 + d ^ 2) : ℝ) : ℍ[ℝ])
        + ((Spoly n x (b ^ 2 + c ^ 2 + d ^ 2) : ℝ) : ℍ[ℝ]) * w3 b c d :=
  pow_eq_of_sq (w3 b c d) (b ^ 2 + c ^ 2 + d ^ 2) (w3_mul_self b c d) x n

/-- ✂️ `p (x + w) = A (x, |w|²) + w B (x, |w|²)` on all of `ℍ`. -/
theorem evalPoly_quat (a : ℕ → ℍ[ℝ]) (N : ℕ) (x b c d : ℝ) :
    evalPoly a N ((x : ℍ[ℝ]) + w3 b c d)
      = polyA a N x (b ^ 2 + c ^ 2 + d ^ 2) + w3 b c d * polyB a N x (b ^ 2 + c ^ 2 + d ^ 2) := by
  unfold evalPoly polyA polyB
  rw [mul_sum, ← sum_add_distrib]
  refine sum_congr rfl fun n _ => ?_
  rw [quat_pow_eq, add_mul,
    Quaternion.coe_commutes (Spoly n x (b ^ 2 + c ^ 2 + d ^ 2)) (w3 b c d), mul_assoc]

/-- 🗺️ The plane `ℂ_j = {a + c j}` inside the quaternions. -/
def InPlaneJ (q : ℍ[ℝ]) : Prop := q.imI = 0 ∧ q.imK = 0

theorem polyA_inPlaneJ {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlaneJ (a n)) (N : ℕ) (x r : ℝ) :
    InPlaneJ (polyA a N x r) := by
  unfold polyA
  refine ⟨?_, ?_⟩
  · rw [imI_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).1]
  · rw [imK_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).2]

theorem polyB_inPlaneJ {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlaneJ (a n)) (N : ℕ) (x r : ℝ) :
    InPlaneJ (polyB a N x r) := by
  unfold polyB
  refine ⟨?_, ?_⟩
  · rw [imI_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).1]
  · rw [imK_sum]
    exact sum_eq_zero fun n _ => by simp [(ha n).2]

/-- 🏛️ Coefficients in `ℂ_j`, any degree, all of `ℍ`: a root outside `ℂ_j` kills `A` and `B`. -/
theorem polyA_polyB_eq_zero_J {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlaneJ (a n)) {N : ℕ} {x b c d : ℝ}
    (h : evalPoly a N ((x : ℍ[ℝ]) + w3 b c d) = 0) (hbd : b ≠ 0 ∨ d ≠ 0) :
    polyA a N x (b ^ 2 + c ^ 2 + d ^ 2) = 0 ∧ polyB a N x (b ^ 2 + c ^ 2 + d ^ 2) = 0 := by
  rw [evalPoly_quat] at h
  obtain ⟨hAI, hAK⟩ := polyA_inPlaneJ ha N x (b ^ 2 + c ^ 2 + d ^ 2)
  obtain ⟨hBI, hBK⟩ := polyB_inPlaneJ ha N x (b ^ 2 + c ^ 2 + d ^ 2)
  generalize polyA a N x (b ^ 2 + c ^ 2 + d ^ 2) = A at h hAI hAK ⊢
  generalize polyB a N x (b ^ 2 + c ^ 2 + d ^ 2) = B at h hBI hBK ⊢
  have hI : b * B.re - d * B.imJ = 0 := by
    have := congrArg (fun q : ℍ[ℝ] => q.imI) h
    simp [w3, hAI, hBI, hBK] at this
    linarith
  have hK : b * B.imJ + d * B.re = 0 := by
    have := congrArg (fun q : ℍ[ℝ] => q.imK) h
    simp [w3, hAK, hBI, hBK] at this
    linarith
  have hpos : 0 < b ^ 2 + d ^ 2 := by
    rcases hbd with hb | hd
    · positivity
    · positivity
  have hre : B.re = 0 := by
    have : B.re * (b ^ 2 + d ^ 2) = 0 := by linear_combination b * hI + d * hK
    exact (mul_eq_zero.mp this).resolve_right hpos.ne'
  have him : B.imJ = 0 := by
    have : B.imJ * (b ^ 2 + d ^ 2) = 0 := by linear_combination b * hK - d * hI
    exact (mul_eq_zero.mp this).resolve_right hpos.ne'
  have hB : B = 0 := by
    ext
    · simpa using hre
    · simpa using hBI
    · simpa using him
    · simpa using hBK
  rw [hB, mul_zero, add_zero] at h
  exact ⟨h, hB⟩

/-- 🏛️ … and then the whole 2-sphere `x + |w| 𝕊` consists of roots. -/
theorem sphere_of_roots_J {a : ℕ → ℍ[ℝ]} (ha : ∀ n, InPlaneJ (a n)) {N : ℕ} {x b c d : ℝ}
    (h : evalPoly a N ((x : ℍ[ℝ]) + w3 b c d) = 0) (hbd : b ≠ 0 ∨ d ≠ 0) {b' c' d' : ℝ}
    (hr : b' ^ 2 + c' ^ 2 + d' ^ 2 = b ^ 2 + c ^ 2 + d ^ 2) :
    evalPoly a N ((x : ℍ[ℝ]) + w3 b' c' d') = 0 := by
  obtain ⟨hA, hB⟩ := polyA_polyB_eq_zero_J ha h hbd
  rw [evalPoly_quat, hr, hA, hB, mul_zero, add_zero]

theorem coe_sum (σ : Finset ℕ) (f : ℕ → ℝ) :
    ((∑ n ∈ σ, f n : ℝ) : ℍ[ℝ]) = ∑ n ∈ σ, ((f n : ℝ) : ℍ[ℝ]) := by
  classical
  induction σ using Finset.induction_on with
  | empty => simp
  | insert i σ hi ih => rw [sum_insert hi, sum_insert hi, Quaternion.coe_add, ih]

/-- 🏛️ Real coefficients, any degree: `p (z) = 0` iff `A = 0` and `B y = B t = 0` with real `A`, `B`. -/
theorem real_poly_root_iff (α : ℕ → ℝ) (N : ℕ) (x y t : ℝ) :
    evalPoly (fun n => ((α n : ℝ) : ℍ[ℝ])) N (z x y t) = 0
      ↔ (∑ n ∈ range (N + 1), Rpoly n x (y ^ 2 + t ^ 2) * α n = 0)
        ∧ (∑ n ∈ range (N + 1), Spoly n x (y ^ 2 + t ^ 2) * α n) * y = 0
        ∧ (∑ n ∈ range (N + 1), Spoly n x (y ^ 2 + t ^ 2) * α n) * t = 0 := by
  have hA : polyA (fun n => ((α n : ℝ) : ℍ[ℝ])) N x (y ^ 2 + t ^ 2)
      = ((∑ n ∈ range (N + 1), Rpoly n x (y ^ 2 + t ^ 2) * α n : ℝ) : ℍ[ℝ]) := by
    unfold polyA
    rw [coe_sum]
    exact sum_congr rfl fun n _ => by rw [Quaternion.coe_mul]
  have hB : polyB (fun n => ((α n : ℝ) : ℍ[ℝ])) N x (y ^ 2 + t ^ 2)
      = ((∑ n ∈ range (N + 1), Spoly n x (y ^ 2 + t ^ 2) * α n : ℝ) : ℍ[ℝ]) := by
    unfold polyB
    rw [coe_sum]
    exact sum_congr rfl fun n _ => by rw [Quaternion.coe_mul]
  rw [evalPoly_z, hA, hB]
  generalize (∑ n ∈ range (N + 1), Rpoly n x (y ^ 2 + t ^ 2) * α n) = A
  generalize (∑ n ∈ range (N + 1), Spoly n x (y ^ 2 + t ^ 2) * α n) = B
  constructor
  · intro h
    have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
    have h2 := congrArg (fun q : ℍ[ℝ] => q.imI) h
    have h3 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
    simp [v] at h1 h2 h3
    refine ⟨h1, ?_, ?_⟩
    · rcases h2 with h2 | h2 <;> simp [h2]
    · rcases h3 with h3 | h3 <;> simp [h3]
  · rintro ⟨h1, h2, h3⟩
    ext
    · simpa [v] using h1
    · simpa [v, mul_comm] using h2
    · simpa [v, mul_comm] using h3
    · simp [v]

theorem z_sq (x y t : ℝ) :
    z x y t ^ 2 = ⟨x ^ 2 - (y ^ 2 + t ^ 2), 2 * x * y, 2 * x * t, 0⟩ := by
  rw [pow_two]
  ext <;> simp [z] <;> ring

theorem z_cube (x y t : ℝ) :
    z x y t ^ 3 = ⟨x ^ 3 - 3 * x * (y ^ 2 + t ^ 2), (3 * x ^ 2 - (y ^ 2 + t ^ 2)) * y,
      (3 * x ^ 2 - (y ^ 2 + t ^ 2)) * t, 0⟩ := by
  rw [pow_succ, z_sq]
  ext <;> simp [z] <;> ring

/-- 🏛️ Real coefficients, degree three. -/
theorem real_cubic_root_iff (b c d x y t : ℝ) :
    z x y t ^ 3 + (b : ℍ[ℝ]) * z x y t ^ 2 + (c : ℍ[ℝ]) * z x y t + (d : ℍ[ℝ]) = 0
      ↔ x ^ 3 - 3 * x * (y ^ 2 + t ^ 2) + b * (x ^ 2 - (y ^ 2 + t ^ 2)) + c * x + d = 0
        ∧ (3 * x ^ 2 - (y ^ 2 + t ^ 2) + 2 * b * x + c) * y = 0
        ∧ (3 * x ^ 2 - (y ^ 2 + t ^ 2) + 2 * b * x + c) * t = 0 := by
  rw [z_cube, z_sq]
  constructor
  · intro h
    have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
    have h2 := congrArg (fun q : ℍ[ℝ] => q.imI) h
    have h3 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
    simp [z] at h1 h2 h3
    exact ⟨by linarith, by linarith, by linarith⟩
  · rintro ⟨h1, h2, h3⟩
    ext
    · simp [z]
      linarith
    · simp [z]
      linarith
    · simp [z]
      linarith
    · simp [z]

/-- 🏛️ A non-real root of a real cubic lies on the circle `r = p' (x)` over a root of the resolvent cubic. -/
theorem real_cubic_circle (b c d x y t : ℝ) (hyt : y ≠ 0 ∨ t ≠ 0)
    (h : z x y t ^ 3 + (b : ℍ[ℝ]) * z x y t ^ 2 + (c : ℍ[ℝ]) * z x y t + (d : ℍ[ℝ]) = 0) :
    y ^ 2 + t ^ 2 = 3 * x ^ 2 + 2 * b * x + c
      ∧ 8 * x ^ 3 + 8 * b * x ^ 2 + 2 * (b ^ 2 + c) * x + b * c - d = 0 := by
  obtain ⟨h1, h2, h3⟩ := (real_cubic_root_iff b c d x y t).mp h
  have hr : 3 * x ^ 2 - (y ^ 2 + t ^ 2) + 2 * b * x + c = 0 := by
    rcases hyt with hy | ht
    · exact (mul_eq_zero.mp h2).resolve_right hy
    · exact (mul_eq_zero.mp h3).resolve_right ht
  exact ⟨by linarith, by linear_combination (3 * x + b) * hr - h1⟩

/-- 🧪 The cubic `q³ - q² (1 + 3 j) + q (-2 + 3 j) + 2`, which is `(q - 1)(q - j)(q - 2 j)` on `ℂ_j`. -/
def jCubic (q : ℍ[ℝ]) : ℍ[ℝ] :=
  q ^ 3 - q ^ 2 * ⟨1, 0, 3, 0⟩ + q * ⟨-2, 0, 3, 0⟩ + ⟨2, 0, 0, 0⟩

/-- 🧮 Its coefficients, all in `ℂ_j`. -/
def jCubicCoeff : ℕ → ℍ[ℝ]
  | 0 => ⟨2, 0, 0, 0⟩
  | 1 => ⟨-2, 0, 3, 0⟩
  | 2 => -⟨1, 0, 3, 0⟩
  | 3 => 1
  | _ => 0

theorem jCubicCoeff_inPlaneJ (n : ℕ) : InPlaneJ (jCubicCoeff n) := by
  rcases n with _ | _ | _ | _ | n <;> simp [jCubicCoeff, InPlaneJ]

theorem jCubic_eq_evalPoly (q : ℍ[ℝ]) : jCubic q = evalPoly jCubicCoeff 3 q := by
  unfold jCubic evalPoly
  simp only [sum_range_succ, sum_range_zero, jCubicCoeff, zero_add, pow_zero, one_mul, pow_one,
    mul_one, mul_neg]
  abel

theorem jCubic_plane (a c : ℝ) :
    jCubic ⟨a, 0, c, 0⟩
      = ((⟨a, 0, c, 0⟩ : ℍ[ℝ]) - ⟨1, 0, 0, 0⟩) * ((⟨a, 0, c, 0⟩ : ℍ[ℝ]) - ⟨0, 0, 1, 0⟩)
        * ((⟨a, 0, c, 0⟩ : ℍ[ℝ]) - ⟨0, 0, 2, 0⟩) := by
  unfold jCubic
  ext <;> simp [pow_succ] <;> ring

theorem jCubic_plane_root {a c : ℝ} (h : jCubic ⟨a, 0, c, 0⟩ = 0) :
    (a = 1 ∧ c = 0) ∨ (a = 0 ∧ c = 1) ∨ (a = 0 ∧ c = 2) := by
  rw [jCubic_plane] at h
  rcases mul_eq_zero.mp h with h | h
  · rcases mul_eq_zero.mp h with h | h
    · left
      have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
      have h2 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
      simp at h1 h2
      exact ⟨by linarith, by linarith⟩
    · right; left
      have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
      have h2 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
      simp at h1 h2
      exact ⟨by linarith, by linarith⟩
  · right; right
    have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
    have h2 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
    simp at h1 h2
    exact ⟨by linarith, by linarith⟩

/-- 🏛️ In all of `ℍ` the cubic has exactly the three isolated roots `1`, `j`, `2 j`; two of them lie outside the `xy`-plane. -/
theorem jCubic_root_iff (q : ℍ[ℝ]) :
    jCubic q = 0 ↔ q = ⟨1, 0, 0, 0⟩ ∨ q = ⟨0, 0, 1, 0⟩ ∨ q = ⟨0, 0, 2, 0⟩ := by
  constructor
  · intro h
    obtain ⟨a, b, c, d⟩ := q
    by_cases hbd : b = 0 ∧ d = 0
    · obtain ⟨rfl, rfl⟩ := hbd
      rcases jCubic_plane_root h with ⟨rfl, rfl⟩ | ⟨rfl, rfl⟩ | ⟨rfl, rfl⟩
      · exact Or.inl rfl
      · exact Or.inr (Or.inl rfl)
      · exact Or.inr (Or.inr rfl)
    · exfalso
      have hbd' : b ≠ 0 ∨ d ≠ 0 := by
        by_cases hb : b = 0
        · exact Or.inr fun hd => hbd ⟨hb, hd⟩
        · exact Or.inl hb
      have hρ : 0 < b ^ 2 + c ^ 2 + d ^ 2 := by
        rcases hbd' with hb | hd
        · positivity
        · positivity
      set ρ := b ^ 2 + c ^ 2 + d ^ 2 with hρdef
      have hs : 0 < Real.sqrt ρ := Real.sqrt_pos.mpr hρ
      have hroot : evalPoly jCubicCoeff 3 ((a : ℍ[ℝ]) + w3 b c d) = 0 := by
        rw [← quat_eq, ← jCubic_eq_evalPoly]
        exact h
      have key : ∀ c' : ℝ, c' ^ 2 = ρ → jCubic ⟨a, 0, c', 0⟩ = 0 := by
        intro c' hc'
        rw [jCubic_eq_evalPoly, quat_eq]
        refine sphere_of_roots_J jCubicCoeff_inPlaneJ hroot hbd' ?_
        rw [← hρdef, ← hc']
        ring
      have hplus := jCubic_plane_root (key (Real.sqrt ρ) (Real.sq_sqrt hρ.le))
      have hminus := jCubic_plane_root (key (-Real.sqrt ρ) (by rw [neg_sq, Real.sq_sqrt hρ.le]))
      rcases hminus with ⟨_, h0⟩ | ⟨_, h0⟩ | ⟨_, h0⟩ <;> linarith
  · rintro (rfl | rfl | rfl)
    · rw [jCubic_plane]
      simp
    · rw [jCubic_plane]
      simp
    · rw [jCubic_plane]
      simp

/-- 🧮 The embedding `ℂ → ℂ_j ⊂ ℍ`, `a + c i ↦ a + c j`. -/
def jEmb (w : ℂ) : ℍ[ℝ] := ⟨w.re, 0, w.im, 0⟩

theorem jEmb_mul (w₁ w₂ : ℂ) : jEmb (w₁ * w₂) = jEmb w₁ * jEmb w₂ := by
  ext
  · simp [jEmb]
    ring
  · simp [jEmb]
  · simp [jEmb]
  · simp [jEmb]

theorem jEmb_pow (w : ℂ) (n : ℕ) : jEmb (w ^ n) = jEmb w ^ n := by
  induction n with
  | zero => ext <;> simp [jEmb]
  | succ n ih => rw [pow_succ, pow_succ, jEmb_mul, ih]

theorem jEmb_injective : Function.Injective jEmb := by
  intro w₁ w₂ h
  have h1 := congrArg (fun q : ℍ[ℝ] => q.re) h
  have h2 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
  exact Complex.ext h1 h2

/-- 🏛️ Every root of `qⁿ = j` lies in `ℂ_j` and outside the `xy`-plane. -/
theorem pow_eq_j {q : ℍ[ℝ]} {n : ℕ} (h : q ^ n = ⟨0, 0, 1, 0⟩) :
    q.imI = 0 ∧ q.imK = 0 ∧ q.imJ ≠ 0 := by
  obtain ⟨a, b, c, d⟩ := q
  rw [quat_eq, quat_pow_eq] at h
  generalize Rpoly n a (b ^ 2 + c ^ 2 + d ^ 2) = R at h
  generalize Spoly n a (b ^ 2 + c ^ 2 + d ^ 2) = S at h
  have h2 := congrArg (fun q : ℍ[ℝ] => q.imI) h
  have h3 := congrArg (fun q : ℍ[ℝ] => q.imJ) h
  have h4 := congrArg (fun q : ℍ[ℝ] => q.imK) h
  simp [w3] at h2 h3 h4
  have hS : S ≠ 0 := by
    rintro rfl
    simp at h3
  have hc : c ≠ 0 := by
    rintro rfl
    simp at h3
  exact ⟨h2.resolve_left hS, h4.resolve_left hS, hc⟩

/-- 🏛️ `qⁿ = j` has a root for every `n ≥ 1`. -/
theorem exists_pow_eq_j {n : ℕ} (hn : 0 < n) : ∃ q : ℍ[ℝ], q ^ n = ⟨0, 0, 1, 0⟩ := by
  refine ⟨jEmb (Complex.exp ((Real.pi / (2 * n) : ℝ) * Complex.I)), ?_⟩
  have hn' : (n : ℂ) ≠ 0 := by exact_mod_cast hn.ne'
  rw [← jEmb_pow, ← Complex.exp_nat_mul]
  have : (n : ℂ) * (((Real.pi / (2 * n) : ℝ) : ℂ) * Complex.I)
      = ((Real.pi / 2 : ℝ) : ℂ) * Complex.I := by
    push_cast
    field_simp
  rw [this, Complex.exp_mul_I, ← Complex.ofReal_cos, ← Complex.ofReal_sin, Real.cos_pi_div_two,
    Real.sin_pi_div_two]
  ext <;> simp [jEmb]

/-- 🏛️ `qⁿ = j` has only finitely many roots in `ℍ`, so every root is isolated. -/
theorem pow_eq_j_finite {n : ℕ} (hn : 0 < n) :
    {q : ℍ[ℝ] | q ^ n = ⟨0, 0, 1, 0⟩}.Finite := by
  have hfin : {w : ℂ | w ^ n = Complex.I}.Finite := by
    have hp : (Polynomial.X ^ n - Polynomial.C Complex.I : Polynomial ℂ) ≠ 0 :=
      Polynomial.X_pow_sub_C_ne_zero hn _
    refine Set.Finite.subset
      (Polynomial.X ^ n - Polynomial.C Complex.I : Polynomial ℂ).roots.toFinset.finite_toSet ?_
    intro w hw
    have hw' : w ^ n = Complex.I := hw
    simp [Polynomial.mem_roots hp, Polynomial.IsRoot, hw']
  refine (hfin.image jEmb).subset ?_
  intro q hq
  obtain ⟨hI, hK, _⟩ := pow_eq_j hq
  refine ⟨⟨q.re, q.imJ⟩, ?_, ?_⟩
  · have hq' : jEmb ⟨q.re, q.imJ⟩ = q := by
      ext <;> simp [jEmb, hI, hK]
    apply jEmb_injective
    rw [jEmb_pow, hq']
    have hq'' : q ^ n = ⟨0, 0, 1, 0⟩ := hq
    rw [hq'']
    ext <;> simp [jEmb]
  · ext <;> simp [jEmb, hI, hK]

end RealPartsOfPowers
