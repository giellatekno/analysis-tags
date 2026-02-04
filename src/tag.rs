//! Tag. There is an OwnedTag, too.

use crate::pos::{Pos, PosFromStrError};

#[derive(Debug, PartialEq)]
pub enum Tag<'a> {
    /// A plan POS by itself.
    Pos(Pos),
    Pl,
    Sg,
    Indef,
    /// Essive
    Ess,

    /// Cardinal
    Card,

    /// Personal pronoun
    Pers,

    /// Arab numerals (a "number" can also be a word, e.g. "one", but "1" is
    /// an arabic numeral)
    Arab,

    /// Attribute
    Attr,

    /// Ordinal
    Ord,

    /// Reciprocal
    Recipr,

    /// Illative
    Ill,

    /// Negation verb
    Neg,

    /// Locative
    Loc,

    /// Indicative
    Ind,

    /// Infintive
    Inf,

    /// Proper Noun
    Prop,

    /// Reflecsive
    Refl,

    /// Perfect participle
    PrfPrc,

    /// Present participle
    PrsPrc,

    /// Neutral (gender?)
    Neu,

    /// Gerundium
    Ger,

    /// Interrogative
    Interr,

    /// Actio
    Actio,

    /// Cmp/xxx
    Cmp(&'a str),

    /// Potential
    Pot,

    /// Conditional (mood)
    Cond,

    /// Imperative (mood)
    Imprt,

    /// Second Imperative
    ImprtII,

    /// Verbal genitive
    VGen,

    /// Verbal abessive
    VAbess,

    /// Supine
    Sup,

    /// Present tense
    Prs,

    /// Nomen Actiones
    NomAg,

    /// Past tense (preteritum)
    Prt,

    /// First person singular
    Sg1,

    /// Second person singular
    Sg2,

    /// Third person singular
    Sg3,

    /// G3 and G7
    G3,
    G7,

    /// Southern Form (dialectal. sme only?)
    South,

    /// Dual
    Du,

    /// First person dual
    Du1,

    /// Second person dual
    Du2,

    /// Third person dual
    Du3,

    /// Derivation
    Der(&'a str),

    /// First person plural
    Pl1,

    /// Second person plural
    Pl2,

    /// Third person plural
    Pl3,

    /// Abbreviation
    ABBR,

    /// Negative form of the verb
    ConNeg,

    /// Negative form of the verb (II)
    ConNegII,

    /// Transitive verb
    TV,

    /// Intransitive verb
    IV,

    /// Question particle
    Qst,

    /// ??? used in sma
    Ela,

    /// Ine used in sma
    Ine,

    /// Foc ... focal?
    Foc(&'a str),

    /// Demonstrative
    Dem,

    /// Acronym
    Acr,

    /// Accusative
    Acc,

    /// Comitative
    Com,

    /// Nominative
    Nom,

    /// Homonyms
    Hom1,
    Hom2,
    Hom3,

    /// Genitive
    Gen,

    /// Gram
    Gram(&'a str),

    /// For a derived word, indicates which POS the word "used to be",
    /// E.g. "Ex/V" - "this word is derived from a verb".
    Ex(Pos),

    /// Error, specific error inside. E.g. `Err/Orth`
    Err(&'a str),

    /// Semantic tag, e.g. "Sem/Anim". The general form is "Sem/*". Many
    /// things are possible
    Sem(&'a str),

    /// Allegro
    Allegro,

    /// Variants
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,

    /// Some other, unknown tag.
    Unknown(&'a str),

    /// Relative
    Rel,

    /// First person dual possessive suffix
    PxDu1,
    /// Second person dual possessive suffix
    PxDu2,
    /// Third person dual possessive suffix
    PxDu3,
    /// First person plural possessive suffix
    PxPl1,
    /// Second person plural possessive suffix
    PxPl2,
    /// Third person plural possessive suffix
    PxPl3,
    /// First person singular possessive suffix
    PxSg1,
    /// Second person singular possessive suffix
    PxSg2,
    /// Third person singular possessive suffix
    PxSg3,
}

impl Tag<'_> {
    pub fn pos(&self) -> Option<Pos> {
        match self {
            Tag::Pos(pos) => Some(*pos),
            _ => None,
        }
    }

    pub fn is_pos(&self) -> bool {
        matches!(self, Tag::Pos(_))
    }

    pub fn is_sem(&self) -> bool {
        matches!(self, Tag::Sem(_))
    }

    /// If it's any of the "angle bracketed" tags, shown as `"<TAG>"`
    pub fn is_angle_bracketed(&self) -> bool {
        matches!(self, Tag::Unknown(s) if s.starts_with("<"))
    }

    pub fn is_err_starts_with(&self, s: &str) -> bool {
        matches!(self, Tag::Err(sx) if sx.starts_with(s))
    }

    pub fn is_der(&self) -> bool {
        matches!(self, Tag::Der(_))
    }
}

impl From<Pos> for Tag<'_> {
    fn from(value: Pos) -> Self {
        Self::Pos(value)
    }
}

impl std::fmt::Display for Tag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tag::*;
        match self {
            Tag::Pos(pos) => write!(f, "{pos}"),
            ABBR => write!(f, "ABBR"),
            Acc => write!(f, "Acc"),
            Acr => write!(f, "Acr"),
            Actio => write!(f, "Actio"),
            Allegro => write!(f, "Allegro"),
            Attr => write!(f, "Attr"),
            Arab => write!(f, "Arab"),
            Card => write!(f, "Card"),
            Cmp(s) => write!(f, "Cmp/{s}"),
            Com => write!(f, "Com"),
            Ord => write!(f, "Ord"),
            ConNeg => write!(f, "ConNeg"),
            ConNegII => write!(f, "ConNegII"),
            Cond => write!(f, "Cond"),
            Du => write!(f, "Du"),
            Du1 => write!(f, "Du1"),
            Du2 => write!(f, "Du2"),
            Du3 => write!(f, "Du3"),
            Dem => write!(f, "Dem"),
            Refl => write!(f, "Refl"),
            G3 => write!(f, "G3"),
            G7 => write!(f, "G7"),
            Der(s) => write!(f, "Der/{s}"),
            Ind => write!(f, "Ind"),
            Prop => write!(f, "Prop"),
            Pers => write!(f, "Pers"),
            Ela => write!(f, "Ela"),
            Err(s) => write!(f, "Err/{s}"),
            Ess => write!(f, "Ess"),
            Recipr => write!(f, "Recipr"),
            Gram(s) => write!(f, "Gram/{s}"),
            Ex(pos) => write!(f, "Ex/{pos}"),
            Foc(s) => write!(f, "Foc/{s}"),
            Gen => write!(f, "Gen"),
            Rel => write!(f, "Rel"),
            Ger => write!(f, "Ger"),
            Neg => write!(f, "Neg"),
            Neu => write!(f, "Neu"),
            NomAg => write!(f, "NomAg"),
            Ill => write!(f, "Ill"),
            Imprt => write!(f, "Imprt"),
            ImprtII => write!(f, "ImprtII"),
            Indef => write!(f, "Indef"),
            Interr => write!(f, "Interr"),
            Ine => write!(f, "Ine"),
            Inf => write!(f, "Inf"),
            IV => write!(f, "IV"),
            Loc => write!(f, "Loc"),
            Nom => write!(f, "Nom"),
            Hom1 => write!(f, "Hom1"),
            Hom2 => write!(f, "Hom2"),
            Hom3 => write!(f, "Hom3"),
            Pl => write!(f, "Pl"),
            Pl1 => write!(f, "Pl1"),
            Pl2 => write!(f, "Pl2"),
            Pl3 => write!(f, "Pl3"),
            Pot => write!(f, "Pot"),
            PrfPrc => write!(f, "PrfPrc"),
            Prs => write!(f, "Prs"),
            PrsPrc => write!(f, "PrsPrc"),
            Prt => write!(f, "Prt"),
            PxDu1 => write!(f, "PxDu1"),
            PxDu2 => write!(f, "PxDu2"),
            PxDu3 => write!(f, "PxDu3"),
            PxPl1 => write!(f, "PxPl1"),
            PxPl2 => write!(f, "PxPl2"),
            PxPl3 => write!(f, "PxPl3"),
            PxSg1 => write!(f, "PxSg1"),
            PxSg2 => write!(f, "PxSg2"),
            PxSg3 => write!(f, "PxSg3"),
            Qst => write!(f, "Qst"),
            Sem(s) => write!(f, "Sem/{s}"),
            South => write!(f, "South"),
            Sg => write!(f, "Sg"),
            Sg1 => write!(f, "Sg1"),
            Sg2 => write!(f, "Sg2"),
            Sg3 => write!(f, "Sg3"),
            Sup => write!(f, "Sup"),
            TV => write!(f, "TV"),
            Unknown(s) => write!(f, "{s}"),
            V1 => write!(f, "v1"),
            V2 => write!(f, "v2"),
            V3 => write!(f, "v3"),
            V4 => write!(f, "v4"),
            V5 => write!(f, "v5"),
            V6 => write!(f, "v6"),
            V7 => write!(f, "v7"),
            V8 => write!(f, "v8"),
            V9 => write!(f, "v9"),
            V10 => write!(f, "v10"),
            V11 => write!(f, "v11"),
            V12 => write!(f, "v12"),
            V13 => write!(f, "v13"),
            V14 => write!(f, "v14"),
            V15 => write!(f, "v15"),
            V16 => write!(f, "v16"),
            V17 => write!(f, "v17"),
            V18 => write!(f, "v18"),
            V19 => write!(f, "v19"),
            V20 => write!(f, "v20"),
            VAbess => write!(f, "VAbess"),
            VGen => write!(f, "VGen"),
            //_ => write!(f, "nothin'"),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum OwnedTag {
    /// A plan POS by itself.
    Pos(Pos),
    Pl,
    Sg,
    Indef,
    /// Essive
    Ess,

    /// Arab numerals (a "number" can also be a word, e.g. "one", but "1" is
    /// an arabic numeral
    Arab,

    /// Attribute
    Attr,

    /// Ordinal
    Ord,

    /// Illative
    Ill,

    /// Locative
    Loc,

    /// Second Imperative
    ImprtII,

    /// Indicative
    Ind,

    /// Infintive
    Inf,

    /// Perfect participle
    PrfPrc,

    /// Present participle
    PrsPrc,

    /// Neutral (gender?)
    Neu,

    /// Gerundium
    Ger,

    /// Personal pronoun
    Pers,

    /// Actio
    Actio,

    /// Cmp/xxx
    Cmp(String),

    /// Potential
    Pot,

    /// Conditional (mood)
    Cond,

    /// Imperative (mood)
    Imprt,

    /// Reflecsive
    Refl,

    /// Verbal genitive
    VGen,

    /// Verbal abessive
    VAbess,

    /// Reciprocal
    Recipr,

    /// G3 and G7
    G3,
    G7,

    /// Supine
    Sup,

    /// Proper Noun
    Prop,

    /// Present tense
    Prs,

    /// Nomen Actiones
    NomAg,

    /// Interrogative
    Interr,

    /// Past tense (preteritum)
    Prt,

    /// First person singular
    Sg1,

    /// Negation verb
    Neg,

    /// Second person singular
    Sg2,

    /// Third person singular
    Sg3,

    /// Southern Form (dialectal. sme only?)
    South,

    /// Dual
    Du,

    /// First person dual
    Du1,

    /// Second person dual
    Du2,

    /// Third person dual
    Du3,

    /// Derivation
    Der(String),

    /// First person plural
    Pl1,

    /// Second person plural
    Pl2,

    /// Third person plural
    Pl3,

    /// Abbreviation
    ABBR,

    /// Negative form of the verb
    ConNeg,

    /// Negative form of the verb (II)
    ConNegII,

    /// Transitive verb
    TV,

    /// Intransitive verb
    IV,

    /// Question particle
    Qst,

    /// ??? used in sma
    Ela,

    /// Ine used in sma
    Ine,

    /// Foc ... focal?
    Foc(String),

    /// Cardinal
    Card,

    /// Demonstrative
    Dem,

    /// Acronym
    Acr,

    /// Accusative
    Acc,

    /// Comitative
    Com,

    /// Nominative
    Nom,

    /// Homonyms
    Hom1,
    Hom2,
    Hom3,

    /// Genitive
    Gen,

    /// Relative
    Rel,

    /// Gram
    Gram(String),

    /// For a derived word, indicates which POS the word "used to be",
    /// E.g. "Ex/V" - "this word is derived from a verb".
    Ex(Pos),

    /// Error, specific error inside. E.g. `Err/Orth`
    Err(String),

    /// Semantic tag, e.g. "Sem/Anim". The general form is "Sem/*". Many
    /// things are possible
    Sem(String),

    /// Allegro
    Allegro,

    /// Variants
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,

    /// Some other, unknown tag.
    Unknown(String),

    /// First person dual possessive suffix
    PxDu1,
    /// Second person dual possessive suffix
    PxDu2,
    /// Third person dual possessive suffix
    PxDu3,
    /// First person plural possessive suffix
    PxPl1,
    /// Second person plural possessive suffix
    PxPl2,
    /// Third person plural possessive suffix
    PxPl3,
    /// First person singular possessive suffix
    PxSg1,
    /// Second person singular possessive suffix
    PxSg2,
    /// Third person singular possessive suffix
    PxSg3,
}

impl<'a> std::borrow::Borrow<Tag<'a>> for OwnedTag {
    fn borrow(&self) -> &Tag<'a> {
        unimplemented!()
    }
}

impl<'a> std::convert::From<&'a str> for Tag<'a> {
    fn from(value: &'a str) -> Self {
        tag_from_str(value)
    }
}

impl ToOwned for Tag<'_> {
    type Owned = OwnedTag;

    fn to_owned(&self) -> Self::Owned {
        tag_to_owned_tag(self)
    }
}

impl std::fmt::Display for OwnedTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use OwnedTag::*;
        match self {
            OwnedTag::Pos(pos) => write!(f, "{pos}"),
            ABBR => write!(f, "ABBR"),
            Acc => write!(f, "Acc"),
            Acr => write!(f, "Acr"),
            Actio => write!(f, "Actio"),
            Allegro => write!(f, "Allegro"),
            Attr => write!(f, "Attr"),
            Arab => write!(f, "Arab"),
            Card => write!(f, "Card"),
            Cmp(s) => write!(f, "Cmp/{s}"),
            Com => write!(f, "Com"),
            Ord => write!(f, "Ord"),
            ConNeg => write!(f, "ConNeg"),
            ConNegII => write!(f, "ConNegII"),
            Cond => write!(f, "Cond"),
            Recipr => write!(f, "Recipr"),
            Du => write!(f, "Du"),
            Du1 => write!(f, "Du1"),
            Du2 => write!(f, "Du2"),
            Du3 => write!(f, "Du3"),
            Dem => write!(f, "Dem"),
            Der(s) => write!(f, "Der/{s}"),
            Ind => write!(f, "Ind"),
            Ela => write!(f, "Ela"),
            Err(s) => write!(f, "Err/{s}"),
            Ess => write!(f, "Ess"),
            Gram(s) => write!(f, "Gram/{s}"),
            Ex(pos) => write!(f, "Ex/{pos}"),
            Foc(s) => write!(f, "Foc/{s}"),
            Pers => write!(f, "Pers"),
            Gen => write!(f, "Gen"),
            Ger => write!(f, "Ger"),
            Neu => write!(f, "Neu"),
            Neg => write!(f, "Neg"),
            NomAg => write!(f, "NomAg"),
            Ill => write!(f, "Ill"),
            Imprt => write!(f, "Imprt"),
            ImprtII => write!(f, "ImprtII"),
            Interr => write!(f, "Interr"),
            Indef => write!(f, "Indef"),
            Ine => write!(f, "Ine"),
            Refl => write!(f, "Refl"),
            Rel => write!(f, "Rel"),
            Inf => write!(f, "Inf"),
            IV => write!(f, "IV"),
            Loc => write!(f, "Loc"),
            Nom => write!(f, "Nom"),
            G3 => write!(f, "G3"),
            G7 => write!(f, "G7"),
            Hom1 => write!(f, "Hom1"),
            Hom2 => write!(f, "Hom2"),
            Hom3 => write!(f, "Hom3"),
            Pl => write!(f, "Pl"),
            Pl1 => write!(f, "Pl1"),
            Pl2 => write!(f, "Pl2"),
            Pl3 => write!(f, "Pl3"),
            Pot => write!(f, "Pot"),
            PrfPrc => write!(f, "PrfPrc"),
            Prs => write!(f, "Prs"),
            PrsPrc => write!(f, "PrsPrc"),
            Prt => write!(f, "Prt"),
            Prop => write!(f, "Prop"),
            PxDu1 => write!(f, "PxDu1"),
            PxDu2 => write!(f, "PxDu2"),
            PxDu3 => write!(f, "PxDu3"),
            PxPl1 => write!(f, "PxPl1"),
            PxPl2 => write!(f, "PxPl2"),
            PxPl3 => write!(f, "PxPl3"),
            PxSg1 => write!(f, "PxSg1"),
            PxSg2 => write!(f, "PxSg2"),
            PxSg3 => write!(f, "PxSg3"),
            Qst => write!(f, "Qst"),
            Sem(s) => write!(f, "Sem/{s}"),
            South => write!(f, "South"),
            Sg => write!(f, "Sg"),
            Sg1 => write!(f, "Sg1"),
            Sg2 => write!(f, "Sg2"),
            Sg3 => write!(f, "Sg3"),
            Sup => write!(f, "Sup"),
            TV => write!(f, "TV"),
            Unknown(s) => write!(f, "{s}"),
            V1 => write!(f, "v1"),
            V2 => write!(f, "v2"),
            V3 => write!(f, "v3"),
            V4 => write!(f, "v4"),
            V5 => write!(f, "v5"),
            V6 => write!(f, "v6"),
            V7 => write!(f, "v7"),
            V8 => write!(f, "v8"),
            V9 => write!(f, "v9"),
            V10 => write!(f, "v10"),
            V11 => write!(f, "v11"),
            V12 => write!(f, "v12"),
            V13 => write!(f, "v13"),
            V14 => write!(f, "v14"),
            V15 => write!(f, "v15"),
            V16 => write!(f, "v16"),
            V17 => write!(f, "v17"),
            V18 => write!(f, "v18"),
            V19 => write!(f, "v19"),
            V20 => write!(f, "v20"),
            VAbess => write!(f, "VAbess"),
            VGen => write!(f, "VGen"),
            //_ => write!(f, "nothin'"),
        }
    }
}

impl serde::Serialize for OwnedTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        // TODO write "directly to serializer"?
        serializer.serialize_str(&self.to_string())
    }
}

fn tag_to_owned_tag(tag: &Tag) -> OwnedTag {
    match tag {
        Tag::ABBR => OwnedTag::ABBR,
        Tag::Acc => OwnedTag::Acc,
        Tag::Acr => OwnedTag::Acr,
        Tag::Actio => OwnedTag::Actio,
        Tag::Allegro => OwnedTag::Allegro,
        Tag::Arab => OwnedTag::Arab,
        Tag::Attr => OwnedTag::Attr,
        Tag::Card => OwnedTag::Card,
        Tag::Cmp(s) => OwnedTag::Cmp((*s).to_owned()),
        Tag::Com => OwnedTag::Com,
        Tag::ConNeg => OwnedTag::ConNeg,
        Tag::ConNegII => OwnedTag::ConNegII,
        Tag::Cond => OwnedTag::Cond,
        Tag::Dem => OwnedTag::Dem,
        Tag::Der(s) => OwnedTag::Der((*s).to_owned()),
        Tag::Du => OwnedTag::Du,
        Tag::Du1 => OwnedTag::Du1,
        Tag::Du2 => OwnedTag::Du2,
        Tag::Du3 => OwnedTag::Du3,
        Tag::Ela => OwnedTag::Ela,
        Tag::Err(s) => OwnedTag::Err((*s).to_owned()),
        Tag::Ess => OwnedTag::Ess,
        Tag::Ex(pos) => OwnedTag::Ex(pos.clone()),
        Tag::Foc(s) => OwnedTag::Foc((*s).to_owned()),
        Tag::G3 => OwnedTag::G3,
        Tag::G7 => OwnedTag::G7,
        Tag::Gen => OwnedTag::Gen,
        Tag::Ger => OwnedTag::Ger,
        Tag::Gram(s) => OwnedTag::Gram((*s).to_owned()),
        Tag::Hom1 => OwnedTag::Hom1,
        Tag::Hom2 => OwnedTag::Hom2,
        Tag::Hom3 => OwnedTag::Hom3,
        Tag::IV => OwnedTag::IV,
        Tag::Ill => OwnedTag::Ill,
        Tag::Imprt => OwnedTag::Imprt,
        Tag::ImprtII => OwnedTag::ImprtII,
        Tag::Ind => OwnedTag::Ind,
        Tag::Indef => OwnedTag::Indef,
        Tag::Ine => OwnedTag::Ine,
        Tag::Inf => OwnedTag::Inf,
        Tag::Interr => OwnedTag::Interr,
        Tag::Loc => OwnedTag::Loc,
        Tag::Neu => OwnedTag::Neu,
        Tag::Neg => OwnedTag::Neg,
        Tag::Nom => OwnedTag::Nom,
        Tag::NomAg => OwnedTag::NomAg,
        Tag::Ord => OwnedTag::Ord,
        Tag::Pers => OwnedTag::Pers,
        Tag::Pl => OwnedTag::Pl,
        Tag::Pl1 => OwnedTag::Pl1,
        Tag::Pl2 => OwnedTag::Pl2,
        Tag::Pl3 => OwnedTag::Pl3,
        Tag::Pos(pos) => OwnedTag::Pos(pos.clone()),
        Tag::Pot => OwnedTag::Pot,
        Tag::PrfPrc => OwnedTag::PrfPrc,
        Tag::Prop => OwnedTag::Prop,
        Tag::Prs => OwnedTag::Prs,
        Tag::PrsPrc => OwnedTag::PrsPrc,
        Tag::Prt => OwnedTag::Prt,
        Tag::PxDu1 => OwnedTag::PxDu1,
        Tag::PxDu2 => OwnedTag::PxDu2,
        Tag::PxDu3 => OwnedTag::PxDu3,
        Tag::PxPl1 => OwnedTag::PxPl1,
        Tag::PxPl2 => OwnedTag::PxPl2,
        Tag::PxPl3 => OwnedTag::PxPl3,
        Tag::PxSg1 => OwnedTag::PxSg1,
        Tag::PxSg2 => OwnedTag::PxSg2,
        Tag::PxSg3 => OwnedTag::PxSg3,
        Tag::Qst => OwnedTag::Qst,
        Tag::Recipr => OwnedTag::Recipr,
        Tag::Refl => OwnedTag::Refl,
        Tag::Rel => OwnedTag::Rel,
        Tag::Sem(s) => OwnedTag::Sem((*s).to_owned()),
        Tag::Sg => OwnedTag::Sg,
        Tag::Sg1 => OwnedTag::Sg1,
        Tag::Sg2 => OwnedTag::Sg2,
        Tag::Sg3 => OwnedTag::Sg3,
        Tag::South => OwnedTag::South,
        Tag::Sup => OwnedTag::Sup,
        Tag::TV => OwnedTag::TV,
        Tag::Unknown(s) => OwnedTag::Unknown((*s).to_owned()),
        Tag::V1 => OwnedTag::V1,
        Tag::V10 => OwnedTag::V10,
        Tag::V11 => OwnedTag::V11,
        Tag::V12 => OwnedTag::V2,
        Tag::V13 => OwnedTag::V3,
        Tag::V14 => OwnedTag::V4,
        Tag::V15 => OwnedTag::V5,
        Tag::V16 => OwnedTag::V6,
        Tag::V17 => OwnedTag::V7,
        Tag::V18 => OwnedTag::V8,
        Tag::V19 => OwnedTag::V9,
        Tag::V2 => OwnedTag::V2,
        Tag::V20 => OwnedTag::V20,
        Tag::V3 => OwnedTag::V3,
        Tag::V4 => OwnedTag::V4,
        Tag::V5 => OwnedTag::V5,
        Tag::V6 => OwnedTag::V6,
        Tag::V7 => OwnedTag::V7,
        Tag::V8 => OwnedTag::V8,
        Tag::V9 => OwnedTag::V9,
        Tag::VAbess => OwnedTag::VAbess,
        Tag::VGen => OwnedTag::VGen,
    }
}

fn tag_from_str<'a>(s: &'a str) -> Tag<'a> {
    match s.as_bytes() {
        b"A" => Tag::Pos(Pos::A),
        b"ABBR" => Tag::ABBR,
        b"Acc" => Tag::Acc,
        b"Acr" | b"ACR" => Tag::Acr,
        b"Actio" => Tag::Actio,
        b"Adp" => Tag::Pos(Pos::Adp),
        b"Adv" => Tag::Pos(Pos::Adv),
        b"Allegro" => Tag::Allegro,
        b"Arab" => Tag::Arab,
        b"Attr" => Tag::Attr,
        b"Card" => Tag::Card,
        b"CC" => Tag::Pos(Pos::CC),
        b"CLB" => Tag::Pos(Pos::CLB),
        b"CS" => Tag::Pos(Pos::CS),
        b"Com" => Tag::Com,
        b"ConNeg" => Tag::ConNeg,
        b"ConNegII" => Tag::ConNegII,
        b"Cond" => Tag::Cond,
        b"Dem" => Tag::Dem,
        b"Det" => Tag::Pos(Pos::Det),
        b"Du" => Tag::Du,
        b"Du1" => Tag::Du1,
        b"Du2" => Tag::Du2,
        b"Du3" => Tag::Du3,
        b"Ela" => Tag::Ela,
        b"Ess" => Tag::Ess,
        b"G3" => Tag::G3,
        b"G7" => Tag::G7,
        b"Gen" => Tag::Gen,
        b"Ger" => Tag::Ger,
        b"Hom1" => Tag::Hom1,
        b"Hom2" => Tag::Hom2,
        b"Hom3" => Tag::Hom3,
        b"IV" => Tag::IV,
        b"Ill" => Tag::Ill,
        b"Imprt" => Tag::Imprt,
        b"ImprtII" => Tag::ImprtII,
        b"Ind" => Tag::Ind,
        b"Indef" => Tag::Indef,
        b"Ine" => Tag::Ine,
        b"Inf" => Tag::Inf,
        b"Interj" => Tag::Pos(Pos::Interj),
        b"Interr" => Tag::Interr,
        b"Loc" => Tag::Loc,
        b"N" => Tag::Pos(Pos::N),
        b"Neu" => Tag::Neu,
        b"Neg" => Tag::Neg,
        b"Nom" => Tag::Nom,
        b"NomAg" => Tag::NomAg,
        b"Num" => Tag::Pos(Pos::Num),
        b"Ord" => Tag::Ord,
        b"Pcle" => Tag::Pos(Pos::Pcle),
        b"Pers" => Tag::Pers,
        b"Pl" => Tag::Pl,
        b"Pl1" => Tag::Pl1,
        b"Pl2" => Tag::Pl2,
        b"Pl3" => Tag::Pl3,
        b"Po" => Tag::Pos(Pos::Po),
        b"Pot" => Tag::Pot,
        b"Pr" => Tag::Pos(Pos::Pr),
        b"PrfPrc" => Tag::PrfPrc,
        b"Pron" => Tag::Pos(Pos::Pron),
        b"Prop" => Tag::Prop,
        b"Prs" => Tag::Prs,
        b"PrsPrc" => Tag::PrsPrc,
        b"Prt" => Tag::Prt,
        b"PxDu1" => Tag::PxDu1,
        b"PxDu2" => Tag::PxDu2,
        b"PxDu3" => Tag::PxDu3,
        b"PxPl1" => Tag::PxPl1,
        b"PxPl2" => Tag::PxPl2,
        b"PxPl3" => Tag::PxPl3,
        b"PxSg1" => Tag::PxSg1,
        b"PxSg2" => Tag::PxSg2,
        b"PxSg3" => Tag::PxSg3,
        b"Qst" => Tag::Qst,
        b"Recipr" => Tag::Recipr,
        b"Refl" => Tag::Refl,
        b"Rel" => Tag::Rel,
        b"Sg" => Tag::Sg,
        b"Sg1" => Tag::Sg1,
        b"Sg2" => Tag::Sg2,
        b"Sg3" => Tag::Sg3,
        b"South" => Tag::South,
        b"Sup" => Tag::Sup,
        b"TV" => Tag::TV,
        b"V" => Tag::Pos(Pos::V),
        b"VAbess" => Tag::VAbess,
        b"VGen" => Tag::VGen,
        b"v1" => Tag::V1,
        b"v10" => Tag::V10,
        b"v11" => Tag::V11,
        b"v12" => Tag::V12,
        b"v13" => Tag::V13,
        b"v14" => Tag::V14,
        b"v15" => Tag::V15,
        b"v16" => Tag::V16,
        b"v17" => Tag::V17,
        b"v18" => Tag::V18,
        b"v19" => Tag::V19,
        b"v2" => Tag::V2,
        b"v3" => Tag::V3,
        b"v4" => Tag::V4,
        b"v5" => Tag::V5,
        b"v6" => Tag::V6,
        b"v7" => Tag::V7,
        b"v8" => Tag::V8,
        b"v9" => Tag::V9,
        // Der/xx
        [b'D', b'e', b'r', b'/', xx @ ..] => {
            Tag::Der(unsafe { core::str::from_utf8_unchecked(xx) })
        }
        // Gram/
        [b'G', b'r', b'a', b'm', b'/', xx @ ..] => {
            Tag::Gram(unsafe { core::str::from_utf8_unchecked(xx) })
        }
        // Err/
        [b'E', b'r', b'r', b'/', xx @ ..] => {
            Tag::Err(unsafe { core::str::from_utf8_unchecked(xx) })
        }
        // Foc/
        [b'F', b'o', b'c', b'/', xx @ ..] => {
            Tag::Foc(unsafe { core::str::from_utf8_unchecked(xx) })
        }
        // Cmp/
        [b'C', b'm', b'p', b'/', xx @ ..] => {
            Tag::Cmp(unsafe { core::str::from_utf8_unchecked(xx) })
        }
        // Sem/
        [b'S', b'e', b'm', b'/', xx @ ..] => {
            Tag::Sem(unsafe { core::str::from_utf8_unchecked(xx) })
        }
        // Ex/
        [b'E', b'x', b'/', ex_pos @ ..] => {
            let ex_pos = unsafe { core::str::from_utf8_unchecked(ex_pos) };
            match ex_pos.parse::<Pos>() {
                Ok(ex_pos) => Tag::Ex(ex_pos),
                Err(e) => match e {
                    PosFromStrError::TV => Tag::Unknown(ex_pos),
                    PosFromStrError::Unknown => Tag::Unknown(ex_pos),
                },
            }
        }
        _ => Tag::Unknown(s),
    }
}

