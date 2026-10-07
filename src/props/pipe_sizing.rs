#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum PipeMaterial {
    #[default]
    Copper,
    Pex,
}

impl std::fmt::Display for PipeMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Copper => "Copper",
            Self::Pex => "PEX",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopperPipeDia {
    DN15,
    DN18,
    DN20,
    DN25,
    DN32,
    DN40,
    DN50,
    DN65,
    DN80,
    DN90,
    DN100,
    DN125,
    DN150,
    DN200,
}

impl std::fmt::Display for CopperPipeDia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::DN15 => "DN15",
            Self::DN18 => "DN18",
            Self::DN20 => "DN20",
            Self::DN25 => "DN25",
            Self::DN32 => "DN32",
            Self::DN40 => "DN40",
            Self::DN50 => "DN50",
            Self::DN65 => "DN65",
            Self::DN80 => "DN80",
            Self::DN90 => "DN90",
            Self::DN100 => "DN100",
            Self::DN125 => "DN125",
            Self::DN150 => "DN150",
            Self::DN200 => "DN200",
        };
        f.write_str(name)
    }
}

// impl std::fmt::Display for CopperPipeDia {}

impl CopperPipeDia {
    pub const ALL: [Self; 14] = [
        Self::DN15,
        Self::DN18,
        Self::DN20,
        Self::DN25,
        Self::DN32,
        Self::DN40,
        Self::DN50,
        Self::DN65,
        Self::DN80,
        Self::DN90,
        Self::DN100,
        Self::DN125,
        Self::DN150,
        Self::DN200,
    ];

    pub fn pipe_dia(&self) -> f64 {
        match self {
            Self::DN15 => 10.9,
            Self::DN18 => 13.8,
            Self::DN20 => 17.0,
            Self::DN25 => 23.0,
            Self::DN32 => 29.3,
            Self::DN40 => 35.7,
            Self::DN50 => 48.4,
            Self::DN65 => 61.1,
            Self::DN80 => 72.9,
            Self::DN90 => 85.6,
            Self::DN100 => 98.3,
            Self::DN125 => 123.7,
            Self::DN150 => 148.3,
            Self::DN200 => 199.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PexPipeDia {
    DN16,
    DN20,
    DN25,
    DN32,
    DN40,
    DN50,
    DN63,
    DN75,
    DN90,
    DN110,
    DN125,
}

impl std::fmt::Display for PexPipeDia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::DN16 => "DN16",
            Self::DN20 => "DN20",
            Self::DN25 => "DN25",
            Self::DN32 => "DN32",
            Self::DN40 => "DN40",
            Self::DN50 => "DN50",
            Self::DN63 => "DN63",
            Self::DN75 => "DN75",
            Self::DN90 => "DN90",
            Self::DN110 => "DN110",
            Self::DN125 => "DN125",
        };
        f.write_str(name)
    }
}
impl PexPipeDia {
    pub const ALL: [Self; 11] = [
        Self::DN16,
        Self::DN20,
        Self::DN25,
        Self::DN32,
        Self::DN40,
        Self::DN50,
        Self::DN63,
        Self::DN75,
        Self::DN90,
        Self::DN110,
        Self::DN125,
    ];

    pub fn pipe_dia(&self) -> f64 {
        match self {
            Self::DN16 => 12.7,
            Self::DN20 => 15.7,
            Self::DN25 => 19.7,
            Self::DN32 => 25.1,
            Self::DN40 => 31.4,
            Self::DN50 => 39.3,
            Self::DN63 => 49.4,
            Self::DN75 => 58.6,
            Self::DN90 => 70.7,
            Self::DN110 => 86.4,
            Self::DN125 => 98.2,
        }
    }
}
