# Neuro‑EcoKarma Governance Specification v1.0
`specs/governance/neuro_ecokarma_governance_spec_v1.0.md`

## 1. Scope and First Principles

This specification defines the governance layer for a neurorights‑compliant ecological accountability system (“Neuro‑EcoKarma”). It sits above a physics‑based impact engine and NanoKarma‑style operators, and below local law and policy.

The system MUST satisfy three non‑negotiable principles:

1. **Cognitive liberty and mental privacy are non‑derogable.** No rule, measurement, or sanction may intrude on, profile, or engineer an individual’s inner mental states.

2. **Ecological accountability is a legitimate basis for conditioning external permissions.** Permissions to deploy devices, alter infrastructures, and influence shared environments MAY be conditioned on physically measured environmental impact and historical ecological performance.

3. **Only physical and contextual data feed governance operators.** All predicates and operators defined herein MUST be computed solely from:
   - Mass and energy flows (e.g., kg CO₂e, g PM₂.₅, L wastewater).
   - Exposure fields (ambient pollutant concentrations, heat index, noise).
   - Contextual vulnerability indicators (e.g., school zones, critical habitats).
   They MUST NOT use neural telemetry, brain‑data, or inferred mental states.

## 2. Core Definitions

Let:

- \( i \) index an impact component (e.g., CO₂e, PM₂․₅, plastics).
- \( j \) index a pollutant or stressor.
- \( t \) index time.
- \( M_i(t) \): physically measured or LCA‑derived mass/flow for component \(i\) at time \(t\), in standardized units (kg, g, L).
- \( \beta_i \): impact normalization factor mapping \(M_i\) into a common “impact currency” (e.g., CO₂e‑equivalent or dimensionless harm score).
- \( \lambda_i \): contextual justice weight capturing vulnerability and exposure inequality at the location and time of the impact.
- \( K_i(t) \): impact‑specific Karma contribution at time \(t\).
- \( K_{\text{person}}(t) \): cumulative ecological Karma for a given agent at time \(t\).

### 2.1 Karma Operator

For each component \(i\),

\[
K_i(t) = \lambda_i \, \beta_i \, M_i(t).
\]

The personal cumulative Karma is

\[
K_{\text{person}}(t) = \sum_i K_i(t) = \sum_i \lambda_i \beta_i M_i(t).
\]

The incremental change over an interval \([t_0, t_1]\) is

\[
\Delta K_{\text{person}} = \sum_i \lambda_i \beta_i \Delta M_i,
\]

where \(\Delta M_i = M_i(t_1) - M_i(t_0)\).

## 3. Ecological Safety Polytope

Let \(x \in \mathbb{R}^n\) be a vector of ecological stressors for a region over a forecast horizon (e.g., ambient PM₂․₅, NOₓ, O₃, heat index, litter index, biodiversity indicators).

Define the ecological safety polytope:

\[
P_{\text{eco}} = \{ x \in \mathbb{R}^n \mid A_{\text{eco}} x \le b_{\text{eco}} \},
\]

where:

- \( A_{\text{eco}} \in \mathbb{R}^{m \times n} \),
- \( b_{\text{eco}} \in \mathbb{R}^m \),

are derived from regulatory standards, health‑based thresholds, and ecological resilience studies for the jurisdiction.

## 4. Governance Predicates

### 4.1 EcoAdmissible

Given a projected stressor trajectory \(x_{\text{proj}}\),

\[
\text{EcoAdmissible}(x_{\text{proj}}) \iff A_{\text{eco}} x_{\text{proj}} \le b_{\text{eco}}.
\]

This predicate MUST be evaluated using only physically measured or modelled environmental stressors. It MUST NOT depend on any neural, psychological, or personality data.

### 4.2 KarmaAdmissible

Let \(K_{\text{person}}^{\text{proj}}\) denote the projected cumulative Karma for an agent after a proposed action.

Let \(K_{\max} > 0\) denote the maximum allowable ecological debt (a jurisdiction‑specific parameter).

\[
\text{KarmaAdmissible}(K_{\text{person}}^{\text{proj}}) \iff K_{\text{person}}^{\text{proj}} \ge -K_{\max}.
\]

If this condition fails, the agent is in a state of **KarmaInadmissible** for that action class until sufficient positive \(\Delta K\) is earned through restorative or low‑impact behaviors.

### 4.3 ActionAllowed

Given a proposed action, with:

- Projected stressor vector \(x_{\text{proj}}\),
- Projected cumulative Karma \(K_{\text{person}}^{\text{proj}}\),

define:

\[
\text{ActionAllowed} \iff \text{EcoAdmissible}(x_{\text{proj}}) \land \text{KarmaAdmissible}(K_{\text{person}}^{\text{proj}}).
\]

Implementations MUST refuse or block execution of externally impactful actions when `ActionAllowed` evaluates to false.

## 5. Roles, Downgrades, and Restorative Credits

### 5.1 Role Levels

At minimum, the system SHOULD support:

- `FullOperator` – can perform high‑impact actions, subject to predicates.
- `RestrictedOperator` – limited to low‑impact actions or supervised operations.
- `Observer` – read‑only access, no direct actuation rights.

Each agent is associated with:

- Current role level.
- Current \(K_{\text{person}}\).
- Historical ledger of \(\Delta K\) and decisions.

### 5.2 Role Downgrade Rules

Implementations SHOULD define threshold‑based rules, for example:

- If \(K_{\text{person}} < -\alpha_1 K_{\max}\): downgrade from `FullOperator` to `RestrictedOperator`.
- If \(K_{\text{person}} < -\alpha_2 K_{\max}\) (with \(\alpha_2 > \alpha_1\)): downgrade from `RestrictedOperator` to `Observer` for specified high‑impact domains.

Downgrades MUST:

- Be logged with timestamp, reason, and parameters.
- Be appealable under due‑process procedures.

### 5.3 Restorative Credit Rules

To avoid purely punitive dynamics, implementations MUST support positive \(\Delta K\) from restorative actions, e.g.:

- Verified cleanup activities (waste removal, litter collection).
- Habitat restoration (tree planting, wetland rehabilitation).
- Sponsorship or operation of environmental monitoring and mitigation systems.

Each restorative action type MUST have:

- A defined mapping to \(\Delta M_i\) and hence \(\Delta K\).
- Verification requirements (e.g., sensor confirmation, third‑party audit).

## 6. Neurorights Non‑Interference Clauses

The following are mandatory:

1. **Data Scope Restriction**

   - No predicate, operator, or role decision MAY incorporate:
     - Brain imaging data (EEG, fMRI, MEG, etc.).
     - Brain‑computer interface logs.
     - Direct neural recordings.
     - Algorithmic inferences about beliefs, intentions, or attitudes from behavioral traces.

2. **Sanction Scope Restriction**

   - No sanction MAY:
     - Force neural monitoring.
     - Enforce targeted neuromodulation, stimulation, or pharmacological intervention on the brain.
     - Condition restoration of external permissions on disclosure of mental content.

Any violation of these clauses MUST be treated as a protocol breach and reported to appropriate legal and regulatory bodies.

## 7. Consent, Due Process, and Audit

- Participation MUST be based on informed, revocable consent, except where impact accounting is mandated by general environmental law (in which case personal‑level scoring MAY be optional).
- Agents MUST have access to their own \(K_{\text{person}}\), underlying \(M_i\), and parameter sets (\(\lambda_i, \beta_i, K_{\max}, A_{\text{eco}}, b_{\text{eco}}\)).
- There MUST be:
  - A well‑defined appeals process.
  - Independent audit of data quality, λ‑choices, and distributional impacts.
  - Public reporting (aggregated and anonymized) on system performance and fairness indicators.
