# Core EcoKarma Operators — Language-Agnostic Pseudocode v1.0

## Purpose
Reference pseudocode for CEIM mass-balance, Karma computation, ecological safety predicates, and action gating. This is NOT production code; it defines algorithms and required safeguards.

## 1. Data Structures

```pseudo
struct PollutantMass {
  id: UUID
  event_id: UUID
  pollutant_code: String      // e.g., "CO2e", "PM2.5"
  mass_kg: Float             // M_j or M_i
}

struct ImpactParameters {
  impact_code: String        // e.g., "CO2e", "PM2.5", "PLASTIC"
  alpha: Float               // impact normalization factor α_i
  beta: Float                // vulnerability/justice weight β_i
}

struct KarmaDelta {
  id: UUID
  agent_id: String
  impact_code: String
  mass_kg: Float
  alpha: Float
  beta: Float
  delta_K: Float
  timestamp: DateTime
}

struct EcoPolytope {
  A: MatrixFloat             // A_eco (m x n)
  b: VectorFloat             // b_eco (m)
  stressor_labels: [String]  // x dimensions
}

struct AgentState {
  agent_id: String
  role: Enum("FullOperator", "RestrictedOperator", "Observer")
  K_person: Float
}
