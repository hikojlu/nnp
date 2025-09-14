use Nanpa::*;
use std::fmt::Display;

pub struct Nnp(Vec<Nanpa>);
#[derive(Clone, Copy, PartialEq)]
enum Nanpa {
    Ala,
    Wan,
    Tu,
    Luka,
    Mute,
    Ale,
}

impl From<usize> for Nnp {
    fn from(mut value: usize) -> Self {
        if value == 0 {
            return Self(vec![Ala]);
        };
        let ale = value / 100;
        value %= 100;
        let mute = value / 20;
        value %= 20;
        let luka = value / 5;
        value %= 5;
        let tu = value / 2;
        value %= 2;

        let mut out = vec![];
        if ale > 0 {
            out.extend(Self::from(ale).0);
            out.push(Ale);
        };
        out.extend([Mute].repeat(mute));
        out.extend([Luka].repeat(luka));
        out.extend([Tu].repeat(tu));
        if value != 0 {
            out.push(Wan);
        };
        Self(out)
    }
}
impl Nnp {
    pub fn try_from_long(value: &str) -> Result<Self, String> {
        let out = value
            .split_whitespace()
            .map(|n| match n {
                "ale" => Ok(Ale),
                "mute" => Ok(Mute),
                "luka" => Ok(Luka),
                "tu" => Ok(Tu),
                "wan" => Ok(Wan),
                "ala" => Ok(Ala),
                _ => Err(format!("`{}` li nasin nanpa pona ala", value)),
            })
            .collect::<Result<Vec<Nanpa>, _>>()?;
        if out.contains(&Ala) && out.len() > 1 {
            Err("o pana e nanpa `ala` pi taso taso".into())
        } else {
            Ok(Self(out))
        }
    }
    pub fn try_from_short(value: &str) -> Result<Self, String> {
        let out = value
            .chars()
            .map(|n| match n {
                'A' => Ok(Ale),
                'M' => Ok(Mute),
                'L' => Ok(Luka),
                'T' => Ok(Tu),
                'W' => Ok(Wan),
                'X' => Ok(Ala),
                _ => Err(format!("`{}` li NNP ala", value)),
            })
            .collect::<Result<Vec<Nanpa>, _>>()?;
        if out.contains(&Ala) && out.len() > 1 {
            Err("o pana e nanpa `X` pi taso taso".into())
        } else {
            Ok(Self(out))
        }
    }
}
impl TryFrom<&str> for Nnp {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.chars().next().unwrap().is_lowercase() {
            Self::try_from_long(value)
        } else {
            Self::try_from_short(value)
        }
    }
}

impl From<Nnp> for usize {
    fn from(value: Nnp) -> Self {
        value.0.iter().fold(0, |i, n| match n {
            Ale => 1.max(i) * 100,
            Mute => i + 20,
            Luka => i + 5,
            Tu => i + 2,
            Wan => i + 1,
            Ala => i,
        })
    }
}
impl Nnp {
    pub fn long(&self) -> String {
        self.0
            .iter()
            .map(|n| match n {
                Ale => "ale",
                Mute => "mute",
                Luka => "luka",
                Tu => "tu",
                Wan => "wan",
                Ala => "ala",
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    pub fn short(&self) -> String {
        self.0
            .iter()
            .map(|n| match n {
                Ale => "A",
                Mute => "M",
                Luka => "L",
                Tu => "T",
                Wan => "W",
                Ala => "X",
            })
            .collect::<String>()
    }
}
impl Display for Nnp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.long())
    }
}
