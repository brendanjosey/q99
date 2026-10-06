#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum PipeMaterial {
    #[default]
    Copper,
    AusPex,
}

impl std::fmt::Display for PipeMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Copper => "Copper",
            Self::AusPex => "Pex - AusPex",
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
        };
        f.write_str(name)
    }
}

// impl std::fmt::Display for CopperPipeDia {}

impl CopperPipeDia {
    pub const ALL: [Self; 7] = [
        Self::DN15,
        Self::DN18,
        Self::DN20,
        Self::DN25,
        Self::DN32,
        Self::DN40,
        Self::DN50,
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AusPexPipeDia {
    DN16,
    DN20,
    DN25,
    DN32,
}

impl std::fmt::Display for AusPexPipeDia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::DN16 => "DN16",
            Self::DN20 => "DN20",
            Self::DN25 => "DN25",
            Self::DN32 => "DN32",
        };
        f.write_str(name)
    }
}
impl AusPexPipeDia {
    pub const ALL: [Self; 4] = [Self::DN16, Self::DN20, Self::DN25, Self::DN32];

    pub fn pipe_dia(&self) -> f64 {
        match self {
            Self::DN16 => 11.9,
            Self::DN20 => 15.2,
            Self::DN25 => 19.2,
            Self::DN32 => 24.5,
        }
    }
}
