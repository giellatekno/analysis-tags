/// Part-Of-Speech
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pos {
    /// Adjective
    A,
    /// Adposition
    Adp,
    /// Adverb
    Adv,
    /// Conjunction
    CC,
    /// Clause boundary
    CLB,
    /// Subjunction
    CS,
    /// Det (in nob)
    Det,
    /// Interjection
    Interj,
    /// URL
    URL,
    /// Noun
    N,
    /// Numeral
    Num,
    /// Particle
    Pcle,
    /// Postposition
    Po,
    /// Preposition
    Pr,
    /// Pronoun
    Pron,
    /// Verb
    V,
    /// Unknown. Used for default impl, and stringifies to ___ for korp.
    Unknown,
}

impl Pos {
    pub fn is_known_pos(&self) -> bool {
        !matches!(self, Pos::Unknown)
    }

    pub fn as_str(&self) -> &str {
        use Pos::*;
        match self {
            A => "A",
            Adp => "Adp",
            Adv => "Adv",
            CC => "CC",
            CLB => "CLB",
            CS => "CS",
            Det => "Det",
            Interj => "Interj",
            N => "N",
            Num => "Num",
            Pcle => "Pcle",
            Po => "Po",
            Pr => "Pr",
            Pron => "Pron",
            URL => "URL",
            V => "V",
            Unknown => "UNKNOWN",
        }
    }

    /// Return the [`Pos`] as a string suitable for Korp, that is, where [`Pos::Unknown`]
    /// stringifies as `"___"` instead of `"UNKNOWN"`.
    pub fn as_korp_str(&self) -> &str {
        match self {
            Pos::Unknown => "___",
            _other => self.as_str(),
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub enum PosFromStrError {
    /// We found a TV as a "pos", probably because an "Ex/TV" was found
    TV,
    /// Not a known pos
    Unknown,
}

impl std::str::FromStr for Pos {
    type Err = PosFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Pos::A),
            "Adp" => Ok(Pos::Adp),
            "Adv" => Ok(Pos::Adv),
            "CC" => Ok(Pos::CC),
            "CLB" => Ok(Pos::CLB),
            "CS" => Ok(Pos::CS),
            "Det" => Ok(Pos::Det),
            "Interj" => Ok(Pos::Interj),
            "N" => Ok(Pos::N),
            "Num" => Ok(Pos::Num),
            "Pcle" => Ok(Pos::Pcle),
            "Po" => Ok(Pos::Po),
            "Pr" => Ok(Pos::Pr),
            "Pron" => Ok(Pos::Pron),
            "V" => Ok(Pos::V),
            "URL" => Ok(Pos::URL),
            // this isn't a "true pos", but can be found when looking at
            // the "previous" pos of a derived word, i.e. the "Ex/POS" tag.
            // It can say "Ex/TV" (which is always preceded by "Ex/V")
            "TV" => Err(PosFromStrError::TV),
            _ => Err(PosFromStrError::Unknown),
        }
    }
}

impl std::hash::Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_str().as_bytes());
    }
}

impl serde::Serialize for Pos {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}

pub fn is_pos(s: &str) -> bool {
    match s.parse::<Pos>() {
        Ok(Pos::Unknown) => false,
        Ok(_) => true,
        Err(_) => false,
    }
}
