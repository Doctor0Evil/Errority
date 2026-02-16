pub struct BioState {
    pub bci_star: f32,
    pub roh: f32,
    pub fatigue: f32,
    pub pain: f32,
    pub hrv_sdnn: f32,
}

pub enum ActionVerdict {
    AllowFullAction,
    DegradePrecision,
    PauseAndRest,
}

pub trait SafetyGuard {
    fn evaluate(&self, state: &BioState, proposal: &ActionProposal) -> ActionVerdict;
    fn name(&self) -> &'static str;
}

pub struct BciCeilingGuard {
    pub warn_threshold: f32, // e.g. 0.25
    pub hard_ceiling: f32,   // 0.30
}

impl SafetyGuard for BciCeilingGuard {
    fn evaluate(&self, state: &BioState, _proposal: &ActionProposal) -> ActionVerdict {
        if state.bci_star >= self.hard_ceiling || state.roh >= self.hard_ceiling {
            ActionVerdict::PauseAndRest
        } else if state.bci_star >= self.warn_threshold || state.roh >= self.warn_threshold {
            ActionVerdict::DegradePrecision
        } else {
            ActionVerdict::AllowFullAction
        }
    }

    fn name(&self) -> &'static str { "BciCeilingGuard" }
}

pub struct NeurorightsGuard {
    pub forbidden_modules: Vec<String>,
    pub forbidden_functions: Vec<String>,
}

impl NeurorightsGuard {
    pub fn check_module_manifest(&self, manifest: &ModuleManifest) -> Result<(), String> {
        if self.forbidden_modules.contains(&manifest.name) {
            return Err("forbidden module name (neurorights)".into());
        }
        for cap in &manifest.capabilities {
            if self.forbidden_functions.contains(cap) {
                return Err("forbidden functionality (neurorights)".into());
            }
        }
        Ok(())
    }
}
