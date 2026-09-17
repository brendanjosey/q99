use crate::props::core::{ProbabiltyCoeffs, WaterUseType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixtureType {
    Shower,
    Bath,
    Toilet,
    TapBasin,
    TapKitchen,
    TapLaundry,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fixture {
    fixture_type: FixtureType,
    water_use_type: WaterUseType,
    n: Option<usize>,
    p: Option<f64>,
    flow_rate_tw: Option<f64>,
    flow_rate_cw: Option<f64>,
    flow_rate_hw: Option<f64>,
    coeffs: ProbabiltyCoeffs,
}

impl Fixture {
    // setting functions
    pub fn set_qty(&mut self, qty: Result<usize, std::num::ParseIntError>) {
        match qty {
            Ok(n) => self.n = Some(n),
            _ => self.n = None,
        }
        // self.n = Some(qty)
    }

    pub fn set_p_value(&mut self, apts: usize, occupants: usize) {
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

    pub fn fixture_type(&self) -> &FixtureType {
        &self.fixture_type
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

    pub fn shower() -> Self {
        Self {
            fixture_type: FixtureType::Shower,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.061),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.061,
                c1: 0.908,
                c2: 0.475,
                c3: 0.020,
                c4: 1.343,
            },
        }
    }

    pub fn bath() -> Self {
        Self {
            fixture_type: FixtureType::Bath,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.006),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.006,
                c1: 1.460,
                c2: 0.411,
                c3: 0.008,
                c4: 1.768,
            },
        }
    }

    pub fn toilet() -> Self {
        Self {
            fixture_type: FixtureType::Toilet,
            water_use_type: WaterUseType::ColdWater,
            n: Some(1),
            p: Some(0.002),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.002,
                c1: 1.0,
                c2: 1.0,
                c3: 0.002,
                c4: 0.880,
            },
        }
    }

    pub fn tap_basin() -> Self {
        Self {
            fixture_type: FixtureType::TapBasin,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.009),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.009,
                c1: 1.0,
                c2: 1.0,
                c3: 0.004,
                c4: 0.880,
            },
        }
    }

    pub fn tap_kitchen() -> Self {
        Self {
            fixture_type: FixtureType::TapKitchen,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.009),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.009,
                c1: 1.0,
                c2: 1.0,
                c3: 0.004,
                c4: 0.880,
            },
        }
    }

    pub fn tap_laundry() -> Self {
        Self {
            fixture_type: FixtureType::TapLaundry,
            water_use_type: WaterUseType::ColdAndHeatedWater,
            n: Some(1),
            p: Some(0.009),
            flow_rate_tw: None,
            flow_rate_cw: None,
            flow_rate_hw: None,
            coeffs: ProbabiltyCoeffs {
                pk1: 0.009,
                c1: 1.0,
                c2: 1.0,
                c3: 0.004,
                c4: 0.880,
            },
        }
    }
}
