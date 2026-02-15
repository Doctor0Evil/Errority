CREATE TABLE Agent (
    agent_id        VARCHAR(128) PRIMARY KEY,
    display_name    VARCHAR(256),
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Action_Event (
    event_id        UUID PRIMARY KEY,
    agent_id        VARCHAR(128) NOT NULL REFERENCES Agent(agent_id),
    timestamp       TIMESTAMP NOT NULL,
    action_type     VARCHAR(64) NOT NULL, -- e.g., 'Car_Trip', 'Walk_Smoke', 'Alcohol_Consume', 'Litter_Disposal'
    location_geom   TEXT,                 -- WKT or GeoJSON
    metadata_json   JSONB                 -- arbitrary key/value metadata
);

CREATE TABLE Pollutant_Mass (
    mass_id         UUID PRIMARY KEY,
    event_id        UUID NOT NULL REFERENCES Action_Event(event_id),
    pollutant_code  VARCHAR(32) NOT NULL, -- e.g., 'CO2e', 'PM2.5', 'NOx', 'Plastic'
    mass_kg         DOUBLE PRECISION NOT NULL,
    unit            VARCHAR(16) NOT NULL, -- e.g., 'kg'
    source_ref      TEXT,                 -- LCA source or sensor reference
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Governing_Parameter (
    param_id        UUID PRIMARY KEY,
    jurisdiction_id VARCHAR(128) NOT NULL, -- e.g., 'City_X_District_Y'
    pollutant_code  VARCHAR(32) NOT NULL,
    hazard_weight   DOUBLE PRECISION NOT NULL, -- lambda_i
    scaling_factor  DOUBLE PRECISION NOT NULL, -- beta_i
    effective_date  DATE NOT NULL,
    UNIQUE (jurisdiction_id, pollutant_code, effective_date)
);

CREATE TABLE Karma_Delta (
    karma_id        UUID PRIMARY KEY,
    mass_id         UUID NOT NULL REFERENCES Pollutant_Mass(mass_id),
    agent_id        VARCHAR(128) NOT NULL REFERENCES Agent(agent_id),
    hazard_weight   DOUBLE PRECISION NOT NULL,
    scaling_factor  DOUBLE PRECISION NOT NULL,
    karma_delta     DOUBLE PRECISION NOT NULL, -- K_i = lambda_i * beta_i * M_i
    timestamp       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Agent_Role (
    agent_id        VARCHAR(128) PRIMARY KEY REFERENCES Agent(agent_id),
    role_level      VARCHAR(32) NOT NULL, -- 'FullOperator', 'RestrictedOperator', 'Observer'
    k_current       DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    last_update     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status          VARCHAR(32) NOT NULL DEFAULT 'Active'
);

-- Optional: table to store ecological polytope parameters per jurisdiction
CREATE TABLE Eco_Polytope (
    poly_id         UUID PRIMARY KEY,
    jurisdiction_id VARCHAR(128) NOT NULL,
    description     TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Eco_Polytope_Row (
    row_id          UUID PRIMARY KEY,
    poly_id         UUID NOT NULL REFERENCES Eco_Polytope(poly_id),
    coeffs_json     JSONB NOT NULL, -- array of coefficients for one inequality row a_k
    bound_value     DOUBLE PRECISION NOT NULL -- b_k
);
