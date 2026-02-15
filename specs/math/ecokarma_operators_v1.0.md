# EcoKarma Operators and Math v1.0
`specs/math/ecokarma_operators_v1.0.md`

## 1. Mass‑Balance (CEIM‑Style) Operator

For each pollutant or stressor \(j\), within a control volume over a time interval:

\[
M_j = C_{u,j} \, (C_{j,\text{in}} - C_{j,\text{out}}) \, Q \, t.
\]

Where:

- \(M_j\): net mass of pollutant \(j\) added to (positive) or removed from (negative) the control volume [kg].
- \(C_{j,\text{in}}\), \(C_{j,\text{out}}\): inlet and outlet concentrations [kg/m³ or convertible units].
- \(Q\): volumetric flow rate [m³/s].
- \(t\): time interval [s].
- \(C_{u,j}\): unit conversion factor mapping reported units (e.g., µg/m³, mg/L, ppm) into kg/m³.

This operator is medium‑agnostic and can be applied to air, water, and other environmental compartments.

## 2. Mapping Behaviors to \(M_i\)

For each standardized behavior \(a\) (e.g., “smoke one cigarette”, “consume one standard drink”, “improperly dispose of one glass bottle”):

1. Use LCI data (Ecoinvent, CLCD, other accredited sources) to define a vector:

   \[
   M^{(a)} = (M_{\text{CO₂e}}, M_{\text{PM2.5}}, M_{\text{NOx}}, M_{\text{wastewater}}, M_{\text{plastics}}, \dots).
   \]

2. For realized events, adjust components by context:
   - Location and timing (e.g., indoor vs outdoor, sensitive receptors).
   - Local environmental conditions (background concentrations, flow).

These \(M_i\) values populate the Karma operator.

## 3. Impact Normalization \(\beta_i\)

Each \(M_i\) is mapped into a common “impact currency” via \(\beta_i\):

- For climate: \(\beta_{\text{CO₂e}} = 1\) by definition (if CO₂e is chosen as base).
- For pollutants with health endpoints (PM₂․₅, NOₓ): \(\beta_i\) derived from LCIA models (e.g., DALY/kg, damage to human health).
- For litter/plastics: \(\beta_i\) based on harm indices (ecotoxicity, persistence).

Jurisdictions SHOULD document:

- Data sources for \(\beta_i\).
- Units and interpretation of the resulting \(K_i\).

## 4. Vulnerability Weights \(\lambda_i\)

\(\lambda_i\) adjusts for contextual justice and vulnerability:

- Higher \(\lambda_i\) for emissions near:
  - Schools, hospitals, elder‑care facilities.
  - Low‑income or historically over‑burdened communities.
  - Critical habitats and protected ecosystems.

\(\lambda_i\) MUST be:

- Defined in a publicly documented grid or function over space and time.
- Justified using epidemiological and environmental‑justice evidence where possible.

## 5. Karma Operator and Aggregation

For each component \(i\):

\[
K_i = \lambda_i \beta_i M_i.
\]

Total personal Karma at time \(t\):

\[
K_{\text{person}}(t) = \sum_i K_i(t).
\]

Incremental change:

\[
\Delta K_{\text{person}} = \sum_i \lambda_i \beta_i \Delta M_i.
\]

Interpretation:

- \(K_{\text{person}} > 0\): net positive contribution relative to chosen baseline.
- \(K_{\text{person}} < 0\): net ecological debt.

## 6. Ecological Safety Polytope Construction

Let \(x \in \mathbb{R}^n\) be a vector of stressors (e.g., regional mean PM₂․₅, maximum NO₂, average heat index, litter density).

To define \(P_{\text{eco}}\):

1. Select thresholds from:
   - Regulatory limits (e.g., air‑quality standards).
   - Health‑based guidelines (WHO, national agencies).
   - Ecological resilience studies.

2. Encode each threshold as a linear inequality \(a_k^\top x \le b_k\).

3. Stack inequalities into \(A_{\text{eco}} x \le b_{\text{eco}}\), yielding:

   \[
   P_{\text{eco}} = \{ x \mid A_{\text{eco}} x \le b_{\text{eco}} \}.
   \]

Implementations SHOULD keep \(P_{\text{eco}}\) conservative (erring toward protection) and periodically revise it based on new science.

## 7. Separation of Physics and Policy

- **Physics‑anchored**: \(M_j\) computation, unit conversions, LCIA characterization factors used in \(\beta_i\).
- **Policy‑normative**: choice of \(\lambda_i\), \(K_{\max}\), and the exact shape of \(P_{\text{eco}}\).

All normative parameters MUST be clearly flagged as such in code and documentation.
