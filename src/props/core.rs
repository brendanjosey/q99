use crate::props::fixtures::{Fixture, FixtureType};

pub fn calc_diversified_flow_rate(
    flow_rate: Option<f64>,
    target_temp: Option<f64>,
    cw_temp: Option<f64>,
    hw_temp: Option<f64>,
    calc_cw: bool,
) -> f64 {
    // return zero flow rate if target temp is None
    if target_temp == None {
        return 0.0;
    };

    match (cw_temp, hw_temp, target_temp, flow_rate) {
        (Some(cw), Some(hw), Some(tt), Some(q)) => {
            let tt = tt.clamp(cw, hw);
            let dt = hw - cw;

            if calc_cw {
                q * (hw - tt) / dt
            } else {
                q * (tt - cw) / dt
            }
        }
        _ => 0.0,
    }
}

#[derive(Debug, Default)]
pub struct DemandPanelResults {
    pub q_total: Option<f64>,
    pub q_tw: Option<f64>,
    pub q_hw: Option<f64>,
    pub q_cw: Option<f64>,
    pub p0: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaterUseType {
    ColdAndHeatedWater,
    ColdWater,
    HeatedWater,
}

impl std::fmt::Display for WaterUseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ColdAndHeatedWater => "C&HW",
            Self::HeatedWater => "HW",
            Self::ColdWater => "CW",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowType {
    TotalWaterDemand,
    HeatedWaterDemand,
    ColdWaterDemand,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TempProps {
    pub cold_water_temp: Option<f64>,
    pub cold_water_temp_str: String,
    pub heated_water_temp: Option<f64>,
    pub heated_water_temp_str: String,
}

impl Default for TempProps {
    fn default() -> Self {
        Self {
            cold_water_temp: Some(20.0),
            cold_water_temp_str: String::from("20.0"),
            heated_water_temp: Some(65.0),
            heated_water_temp_str: String::from("65.0"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProbabiltyCoeffs {
    pub pk1: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
    pub c4: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixtureProps {
    pub fixture_type: FixtureType,
    pub water_use_type: Option<WaterUseType>,
    pub flow_rate: Option<f64>,
    pub flow_rate_str: String,
    pub temperature: Option<f64>,
    pub temperature_str: String,
}

impl FixtureProps {
    // setters
    pub fn set_flow_rate(&mut self, value: String) {
        let flow_rate = &value.trim().parse::<f64>();
        self.flow_rate_str = value;

        match flow_rate {
            Ok(q) => self.flow_rate = Some(*q),
            _ => self.flow_rate = None,
        }
    }

    pub fn set_temperature(&mut self, value: String, cw_temp: Option<f64>, hw_temp: Option<f64>) {
        let temp = &value.trim().parse::<f64>();
        self.temperature_str = value;

        match temp {
            Ok(t) => {
                let t = t.clamp(cw_temp.unwrap(), hw_temp.unwrap());
                self.temperature = Some(t);
            }
            _ => self.temperature = None,
        }
    }
    //getters
    pub fn flow_rate_tw(&self) -> f64 {
        match self.flow_rate {
            Some(q) => q,
            None => 0.0,
        }
    }

    pub fn flow_rate_hw<'a>(
        &self,
        // fixture_props: &'a FixtureProps,
        temp_props: &'a TempProps,
    ) -> f64 {
        // get fixture full fixture flow_rate
        let flow_rate = match self.flow_rate {
            Some(q) => q,
            None => 0.0,
        };

        match self.fixture_type {
            FixtureType::WashingMachine => match self.water_use_type {
                Some(WaterUseType::ColdWater) => 0.0,
                _ => flow_rate,
            },
            FixtureType::Dishwasher => match self.water_use_type {
                Some(WaterUseType::ColdWater) => 0.0,
                _ => flow_rate,
            },
            _ => calc_diversified_flow_rate(
                Some(flow_rate),
                self.temperature,
                temp_props.cold_water_temp,
                temp_props.heated_water_temp,
                false,
            ),
        }
    }

    pub fn flow_rate_cw<'a>(
        &self,
        // fixture_props: &'a FixtureProps,
        temp_props: &'a TempProps,
    ) -> f64 {
        // get fixture full fixture flow_rate
        let flow_rate = match self.flow_rate {
            Some(q) => q,
            None => 0.0,
        };

        match self.fixture_type {
            FixtureType::WashingMachine => match self.water_use_type {
                Some(WaterUseType::HeatedWater) => 0.0,
                _ => flow_rate,
            },
            FixtureType::Dishwasher => match self.water_use_type {
                Some(WaterUseType::HeatedWater) => 0.0,
                _ => flow_rate,
            },
            _ => calc_diversified_flow_rate(
                Some(flow_rate),
                self.temperature,
                temp_props.cold_water_temp,
                temp_props.heated_water_temp,
                true,
            ),
        }
    }

    // intitalisers
    pub fn all() -> [Self; 8] {
        [
            Self::shower(),
            Self::bath(),
            Self::toilet(),
            Self::tap_basin(),
            Self::tap_kitchen(),
            Self::tap_laundry(),
            Self::washing_machine(),
            Self::dishwasher(),
        ]
    }

    pub fn shower() -> Self {
        Self {
            fixture_type: FixtureType::Shower,
            water_use_type: Some(WaterUseType::ColdAndHeatedWater),
            flow_rate: Some(0.15),
            flow_rate_str: 0.15.to_string(),
            temperature: Some(42.0),
            temperature_str: 42.0.to_string(),
        }
    }

    pub fn bath() -> Self {
        Self {
            fixture_type: FixtureType::Bath,
            water_use_type: Some(WaterUseType::ColdAndHeatedWater),
            flow_rate: Some(0.30),
            flow_rate_str: 0.30.to_string(),
            temperature: Some(42.0),
            temperature_str: 42.0.to_string(),
        }
    }

    pub fn toilet() -> Self {
        Self {
            fixture_type: FixtureType::Toilet,
            water_use_type: Some(WaterUseType::ColdWater),
            flow_rate: Some(0.19),
            flow_rate_str: 0.19.to_string(),
            temperature: Some(20.0),
            temperature_str: 20.0.to_string(),
        }
    }

    pub fn tap_basin() -> Self {
        Self {
            fixture_type: FixtureType::TapBasin,
            water_use_type: Some(WaterUseType::ColdAndHeatedWater),
            flow_rate: Some(0.08),
            flow_rate_str: 0.08.to_string(),
            temperature: Some(42.0),
            temperature_str: 42.0.to_string(),
        }
    }

    pub fn tap_kitchen() -> Self {
        Self {
            fixture_type: FixtureType::TapKitchen,
            water_use_type: Some(WaterUseType::ColdAndHeatedWater),
            flow_rate: Some(0.09),
            flow_rate_str: 0.09.to_string(),
            temperature: Some(42.0),
            temperature_str: 42.0.to_string(),
        }
    }

    pub fn tap_laundry() -> Self {
        Self {
            fixture_type: FixtureType::TapLaundry,
            water_use_type: Some(WaterUseType::ColdAndHeatedWater),
            flow_rate: Some(0.12),
            flow_rate_str: 0.12.to_string(),
            temperature: Some(42.0),
            temperature_str: 42.0.to_string(),
        }
    }

    pub fn washing_machine() -> Self {
        Self {
            fixture_type: FixtureType::WashingMachine,
            water_use_type: Some(WaterUseType::ColdWater),
            flow_rate: Some(0.22),
            flow_rate_str: 0.22.to_string(),
            temperature: Some(20.0),
            temperature_str: 20.0.to_string(),
        }
    }

    pub fn dishwasher() -> Self {
        Self {
            fixture_type: FixtureType::Dishwasher,
            water_use_type: Some(WaterUseType::ColdWater),
            flow_rate: Some(0.08),
            flow_rate_str: 0.08.to_string(),
            temperature: Some(20.0),
            temperature_str: 20.0.to_string(),
        }
    }
}
