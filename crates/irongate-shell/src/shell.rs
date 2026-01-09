use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum Shell {
    Sh,
    PowerShell,
}

impl Shell {
    pub fn current() -> Self {
        if cfg!(windows) {
            Self::PowerShell
        } else {
            Self::Sh
        }
    }

    pub fn binary(&self) -> &str {
        match self {
            Self::Sh => "sh",
            Self::PowerShell => "powershell.exe",
        }
    }

    pub fn args(&self) -> Vec<&str> {
        match self {
            Self::Sh => vec!["-s"],
            Self::PowerShell => vec!["-ExecutionPolicy", "Bypass", "-Command", "-"],
        }
    }
}
