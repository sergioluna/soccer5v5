#[derive(Clone, Debug, PartialEq)]
pub enum PlayerPosition {
    GK,
    CD,
    DF,
    LM,
    CM,
    RM,
    FW,
}

impl std::fmt::Display for PlayerPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlayerPosition::GK => write!(f, "GK")?,
            PlayerPosition::CD => write!(f, "CD")?,
            PlayerPosition::DF => write!(f, "DF")?,
            PlayerPosition::LM => write!(f, "LM")?,
            PlayerPosition::CM => write!(f, "CM")?,
            PlayerPosition::RM => write!(f, "RM")?,
            PlayerPosition::FW => write!(f, "FW")?,
        }

        Ok(())
    }
}
