//??????? mogoce delava kje drugje (Bevy)


use std::fmt;

use crate::{Nadstropje, Okus, Preliv, Topping, Torta};

impl fmt::Display for Okus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Okus::Cokolada => write!(f, "Čokolada"),
            Okus::Vanilija => write!(f, "Vanilija"),
            Okus::Jagoda => write!(f, "Jagoda"),
            Okus::Drugo(s) => write!(f, "{s}"),
        }
    }
}

impl Preliv {
    fn narisi(&self, sirina: usize) -> String {
        let znak = match self {
            Preliv::Cokoladni => "~",
            Preliv::Sadni => "^",
            Preliv::Karamelni => "=",
        };

        znak.repeat(sirina)
    }
}

impl Topping {
    fn narisi(&self) -> &str {
        match self {
            Topping::Svecka => "🕯",
            Topping::Cesnja => "🍒",
            Topping::Sadje => "🍓",
        }
    }
}

/* 
impl fmt::Display for Nadstropje {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let okus = self.okus();
        let notranja_sirina = okus.len().max(10);

        let topping = match self.topping() {
            Some(t) => t.narisi(),
            None => " ",
        };

        writeln!(
            f,
            "  {:^width$}",
            topping,
            width = notranja_sirina
        )?;

        writeln!(
            f,
            "  {}",
            self.preliv().narisi(notranja_sirina + 4)
        )?;

        writeln!(
            f,
            " | {:^width$} |",
            okus,
            width = notranja_sirina
        )?;

        writeln!(
            f,
            " |{}|",
            "_".repeat(notranja_sirina + 2)
        )
    }
}

impl fmt::Display for Torta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for n in self.ostala().iter().rev() {
            writeln!(f, "{n}")?;
        }

        writeln!(f, "{}", self.spodnje())
    }
} */