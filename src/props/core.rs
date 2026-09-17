use crate::props::appliances::{Appliance, ApplianceType};
use crate::props::fixtures::{Fixture, FixtureType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaterUseType {
    ColdAndHeatedWater,
    ColdWater,
    HeatedWater,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndUseType {
    Fixture(FixtureType),
    Appliance(ApplianceType),
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
pub struct EndUseProps {
    pub end_use_type: EndUseType,
    pub water_use_type: WaterUseType,
    pub flow_rate: Option<f64>,
    pub temperature: Option<f64>,
}

impl EndUseProps {
    // setters

    //getters

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
            end_use_type: EndUseType::Fixture(FixtureType::Shower),
            water_use_type: WaterUseType::ColdAndHeatedWater,
            flow_rate: Some(0.15),
            temperature: Some(42.0),
        }
    }

    pub fn bath() -> Self {
        Self {
            end_use_type: EndUseType::Fixture(FixtureType::Bath),
            water_use_type: WaterUseType::ColdAndHeatedWater,
            flow_rate: Some(0.30),
            temperature: Some(42.0),
        }
    }

    pub fn toilet() -> Self {
        Self {
            end_use_type: EndUseType::Fixture(FixtureType::Toilet),
            water_use_type: WaterUseType::ColdWater,
            flow_rate: Some(0.19),
            temperature: None,
        }
    }

    pub fn tap_basin() -> Self {
        Self {
            end_use_type: EndUseType::Fixture(FixtureType::TapBasin),
            water_use_type: WaterUseType::ColdAndHeatedWater,
            flow_rate: Some(0.08),
            temperature: Some(42.0),
        }
    }

    pub fn tap_kitchen() -> Self {
        Self {
            end_use_type: EndUseType::Fixture(FixtureType::TapKitchen),
            water_use_type: WaterUseType::ColdAndHeatedWater,
            flow_rate: Some(0.09),
            temperature: Some(42.0),
        }
    }

    pub fn tap_laundry() -> Self {
        Self {
            end_use_type: EndUseType::Fixture(FixtureType::TapLaundry),
            water_use_type: WaterUseType::ColdAndHeatedWater,
            flow_rate: Some(0.12),
            temperature: Some(42.0),
        }
    }

    pub fn washing_machine() -> Self {
        Self {
            end_use_type: EndUseType::Appliance(ApplianceType::WashingMachine),
            water_use_type: WaterUseType::ColdWater,
            flow_rate: Some(0.22),
            temperature: None,
        }
    }

    pub fn dishwasher() -> Self {
        Self {
            end_use_type: EndUseType::Appliance(ApplianceType::Dishwasher),
            water_use_type: WaterUseType::ColdWater,
            flow_rate: Some(0.08),
            temperature: None,
        }
    }
}
