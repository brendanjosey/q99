use crate::props::core::{ProbabiltyCoeffs, WaterUseType};

#[derive(Debug, Clone, PartialEq)]
pub enum ApplianceType {
    WashingMachine,
    Dishwasher,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Appliance {
    appliance_type: ApplianceType,
    water_use_type: WaterUseType,
    n: Option<usize>,
    p: Option<f64>,
    flow_rate_tw: Option<f64>,
    flow_rate_cw: Option<f64>,
    flow_rate_hw: Option<f64>,
    coeffs: ProbabiltyCoeffs,
}

impl Appliance {
    // setting functions
    pub fn set_qty(&mut self, qty: usize) {
        self.n = Some(qty)
    }

    pub fn set_p(&mut self, apts: usize, occupants: usize) {
        self.p = match self.calc_p_value(apts, occupants) {
            Ok(p) => Some(p),
            _ => None,
        }
    }

    // getters
    pub fn qty(&self) -> usize {
        match self.n {
            Some(qty) => qty,
            _ => 0,
        }
    }

    pub fn p_value(&self) -> f64 {
        match self.p {
            Some(p) => p,
            _ => 0.0,
        }
    }

    pub fn appliance_type(&self) -> &ApplianceType {
        &self.appliance_type
    }

    pub fn calc_p_value(&self, apts: usize, occupants: usize) -> Result<f64, String> {
        let apts = apts.clamp(1, 20) as f64;
        let occupants = occupants as f64;

        let pk = match apts {
            1.0 => self.coeffs.pk1,
            _ => {
                let pkb = self.coeffs.c1 * self.coeffs.pk1 * (apts).powf(-self.coeffs.c2);
                let mkb = self.coeffs.c3 * (apts).powf(-self.coeffs.c4);
                let fob = mkb * (occupants - apts);

                pkb + fob
            }
        };

        Ok(pk)
    }

    pub fn washing_machine() -> Self {
        Self {
            appliance_type: ApplianceType::WashingMachine,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.031),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.031,
                c1: 0.976,
                c2: 0.515,
                c3: 0.005,
                c4: 1.349,
            },
        }
    }

    pub fn dishwasher() -> Self {
        Self {
            appliance_type: ApplianceType::Dishwasher,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.001),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.001,
                c1: 1.0,
                c2: 1.0,
                c3: 0.0005,
                c4: 0.880,
            },
        }
    }
}
