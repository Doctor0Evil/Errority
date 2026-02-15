# City District EcoKarma Pilot — Evaluation Plan v1.0

## 1. Scope

Pilot domain:

- Geography: One urban district.
- Behaviors: Tobacco use, alcohol consumption (containers), litter and unmanaged waste, small-scale outdoor fires.
- Timeframe: 24–36 months.

## 2. Objectives

1. Reduce physical environmental burdens (PM2.5, litter, unmanaged waste, fire incidents).
2. Preserve neurorights (no neural data; no cognitive scoring).
3. Assess fairness and distributional impacts (no disproportionate harms to vulnerable groups).

## 3. Metrics

### 3.1 Environmental Metrics (Before/After)

- Air:
  - Annual mean PM2.5, NO2, O3.
  - Frequency of exceedances vs. standards.
- Litter and Waste:
  - Mismanaged plastic per km².
  - Glass/container litter counts per survey transect.
- Fire:
  - Number of small-scale wildfires,
  - Burned area, estimated biomass burned.

### 3.2 Health and Exposure

- Respiratory admissions (asthma, COPD) per 10,000 residents.
- Self-reported exposure to smoke and litter.

### 3.3 Governance Outcomes

- Distribution of role levels (FullOperator, RestrictedOperator, Observer) by:
  - Income,
  - Race/ethnicity (where lawfully collectable),
  - Neighborhood.
- Frequency and success of appeals/restorative pathways.

## 4. Data Collection

- Baseline (Year 0):
  - Historical environmental monitoring data.
  - Historical incident logs (fires, dumping).
- Pilot (Years 1–3):
  - Continuous air quality monitors.
  - Waste and litter surveys (quarterly).
  - Event detection (fires, illegal dumping).
  - EcoKarma ledger data (anonymized for research; per-DID with consent).

All datasets MUST exclude neural or psychological measurements.

## 5. Analytic Methods

- Pre/post statistical comparison for environmental metrics (e.g., difference-in-differences).
- Distributional analysis of:
  - Environmental benefits (exposure reductions),
  - Restrictions (frequency of blocked actions, role downgrades).
- Qualitative surveys:
  - Perceived fairness,
  - Perceived intrusiveness,
  - Understanding of neurorights guarantees.

## 6. Success Criteria

The pilot is considered successful if:

- Environmental metrics show statistically significant improvement vs. baseline.
- No evidence of systematic bias against vulnerable groups in:
  - Role downgrades,
  - ActionAllowed decisions.
- Participants report:
  - High understanding of neurorights protections,
  - Acceptable levels of intrusiveness.

## 7. Iteration Triggers

- If any group experiences disproportionate restrictions:
  - Recalibrate `β_i` and thresholds.
  - Adjust restorative pathways.
- If environmental outcomes do not improve:
  - Reassess mapping from behaviors to `M_i`,
  - Update `α_i` from latest LCIA,
  - Strengthen or retarget polytopes (`A_eco`, `b_eco`).

