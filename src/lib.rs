
#![allow(non_camel_case_types)]
use strum_macros::{AsRefStr, EnumString};
#[derive(Debug)]
pub struct UnknownTagError(String);
impl ::std::fmt::Display for UnknownTagError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ::std::error::Error for UnknownTagError {}
pub fn unknown_tag(s: &str) -> UnknownTagError {
    UnknownTagError(s.to_string())
}
#[derive(Debug, PartialEq, AsRefStr, EnumString)]
#[strum(
            parse_err_fn = unknown_tag,
            parse_err_ty = UnknownTagError
        )]
pub enum Tag {
    /// kpv: **@CODE@**  Final мед ог _so that I/we won't_ 2019-04-06
    /// nob: **@CODE@**  not in use??
    #[strum(serialize = "1")]
    _1,
    /// sms: **@CODE@** used in combination with +Imp+13+ConNeg
    #[strum(serialize = "13")]
    _13,
    /// ceb: (no comment)
    #[strum(serialize = "1p")]
    _1p,
    /// sms: **@CODE@** used in combination with +Imp+2+ConNeg
    #[strum(serialize = "2")]
    _2,
    /// fit: **@CODE@** = Adjective
    /// kpv: **@CODE@**:  adjective  кывберд   прилагательное
    /// vot: (no comment)
    /// fkv: **@CODE@** = Adjective
    /// liv: **@CODE@** = adjective
    /// mhr: **@CODE@** = adjectives
    /// sma: **@CODE@** = Adjective
    /// smj: **@CODE@** = Adjective
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Adjective
    /// lit: (no comment)
    /// vep: **@CODE@**  = adjective
    /// olo: **@CODE@**  adjective
    /// sme: **@CODE@** - Adjective
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Adjective
    /// vro: **@CODE@** Adjective
    #[strum(serialize = "A")]
    A,
    /// fit: **@CODE@**
    /// kpv: **@CODE@**
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Abbreviation
    /// liv: **@CODE@** containing period
    /// mhr: **@CODE@** = for abbreviations that (may) contain period
    /// sma: **@CODE@**:  Abbreviation
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Abbreviation
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// zxx: (no comment)
    /// olo: **@CODE@**
    /// sme: **@CODE@** Abbreviation, subtag for e.g. +N
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ABBR")]
    ABBR,
    /// fit: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Acronym
    /// liv: **@CODE@** acronyms, not containing period
    /// mhr: **@CODE@** = acronyms
    /// sma: **@CODE@**:  Acronym
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Acronym
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Acronym
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// sme: **@CODE@**  Acronym, subtag for +N
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ACR")]
    ACR,
    /// fit: **@CODE@**
    /// rus: (no comment)
    #[strum(serialize = "Abbr")]
    Abbr,
    /// fit: **@CODE@** = Abessive
    /// kpv: **@CODE@** PrivMod, AbeMod джуджыд анализъястӧм да обобщениеястӧм статьяяс.
    /// fkv: **@CODE@** = Abessive
    /// liv: **@CODE@** = abessive
    /// mhr: **@CODE@** = abessive
    /// smj: **@CODE@** = Abessive case
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Abessive
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = abessive
    /// olo: **@CODE@** abessive
    /// fin: (no comment)
    /// vro: **@CODE@** abessive
    #[strum(serialize = "Abe")]
    Abe,
    /// fit: **@CODE@** = Ablative
    /// kpv: **@CODE@** ablative case -лысь босьтан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Ablative
    /// liv: **@CODE@** = ablative case
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  = ablative case
    /// vep: (no comment)
    /// olo: **@CODE@** ablative case
    /// fin: (no comment)
    /// vro: **@CODE@** ablative
    #[strum(serialize = "Abl")]
    Abl,
    /// kpv: **@CODE@** Absolute = +Sg+Nom
    /// kpv: (no comment)
    #[strum(serialize = "Abs")]
    Abs,
    /// fit: **@CODE@** = Accusative, for pronouns, but is it correct?
    /// kpv: **@CODE@** accusative ZERO керан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Accusative, for pronouns, but is it correct?
    /// mhr: **@CODE@** = accusative
    /// sma: **@CODE@** = Ackusative
    /// smj: **@CODE@** = Accusative case
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Accusative
    /// lit: (no comment)
    /// vep: **@CODE@**  = accusative
    /// olo: **@CODE@** accusative
    /// sme: **@CODE@** - Accusative
    /// fin: (no comment)
    /// vro: **@CODE@** accusative
    #[strum(serialize = "Acc")]
    Acc,
    /// kpv: **@CODE@** accusative -ӧс керан
    #[strum(serialize = "Acc1")]
    Acc1,
    /// kpv: **@CODE@** accusative -сӧ керан
    #[strum(serialize = "Acc3")]
    Acc3,
    /// fin: (no comment)
    #[strum(serialize = "Acr")]
    Acr,
    /// vro: **@CODE@**
    #[strum(serialize = "Acro")]
    Acro,
    /// kpv: **@CODE@**
    #[strum(serialize = "Acron")]
    Acron,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** = Active
    /// liv: **@CODE@** = active
    /// mhr: **@CODE@** = Active
    /// sma: **@CODE@** | *-eme*, could be changed to `+Actio`
    /// sms: **@CODE@** Active voice
    /// mrj: **@CODE@** =  Prc active participle ЫшЫ
    /// vep: **@CODE@**  = active voice
    /// olo: **@CODE@** : active voice
    /// fin: (no comment)
    /// vro: **@CODE@** active
    #[strum(serialize = "Act")]
    Act,
    /// ceb: (no comment)
    #[strum(serialize = "Act1")]
    Act1,
    /// sms: **@CODE@** -men
    #[strum(serialize = "ActEss")]
    ActEss,
    /// kpv: **@CODE@**
    #[strum(serialize = "ActPrsPtc")]
    ActPrsPtc,
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// sme: **@CODE@** Action Verb Form
    #[strum(serialize = "Actio")]
    Actio,
    /// sms: **@CODE@**
    #[strum(serialize = "Actor")]
    Actor,
    /// olo: (no comment)
    #[strum(serialize = "Ad-A")]
    Ad_MINUS_A,
    /// mrj: **@CODE@** =
    /// olo: **@CODE@**  Ad-adjective
    #[strum(serialize = "AdA")]
    AdA,
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Adc")]
    Adc,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Add1")]
    Add1,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Add2")]
    Add2,
    /// fit: **@CODE@** = Adessive
    /// vot: (no comment)
    /// fkv: **@CODE@** = Adessive
    /// liv: **@CODE@** = adessive
    /// vep: **@CODE@**  = adessive
    /// vep: (no comment)
    /// vep: (no comment)
    /// olo: **@CODE@** adessive case
    /// fin: (no comment)
    /// vro: **@CODE@** adessive
    #[strum(serialize = "Ade")]
    Ade,
    /// mrj: **@CODE@** =
    #[strum(serialize = "Adn")]
    Adn,
    /// kpv: **@CODE@**:  adposition (prepositio, postposition)
    /// vot: (no comment)
    /// liv: **@CODE@** = adposition
    /// mhr: **@CODE@** = adpositions
    /// mhr: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Adposition, i.e. Post- and Prepostion
    /// lit: (no comment)
    /// vep: **@CODE@**  = adposition
    /// olo: **@CODE@**  adposition
    /// sme: **@CODE@** - Adposition, ie Post- and Prepostion, NOT IN USE
    /// fin: (no comment)
    /// vro: **@CODE@** Adpositions
    #[strum(serialize = "Adp")]
    Adp,
    /// fit: **@CODE@** = Adverb
    /// kpv: **@CODE@**:  adverb    урчитан            наречие
    /// vot: (no comment)
    /// fkv: **@CODE@** = Adverb
    /// liv: **@CODE@** = adverb
    /// mhr: **@CODE@** = adverbs
    /// mhr: (no comment)
    /// sma: **@CODE@** = Adverb
    /// smj: **@CODE@** = Adverb
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Adverb
    /// lit: (no comment)
    /// vep: **@CODE@**  = adverb
    /// olo: **@CODE@**  adverb
    /// sme: **@CODE@** - Adverb
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Adverb
    /// ces: (no comment)
    /// vro: **@CODE@** Adverb
    #[strum(serialize = "Adv")]
    Adv,
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    #[strum(serialize = "Advc")]
    Advc,
    /// fin: (no comment)
    #[strum(serialize = "AgPrc")]
    AgPrc,
    /// fit: **@CODE@** = Allative
    /// vot: (no comment)
    /// fkv: **@CODE@** = Allative
    /// liv: **@CODE@** = allative
    /// rus: (no comment)
    /// vep: **@CODE@**  = allative
    /// vep: (no comment)
    /// olo: **@CODE@** Allatiivi
    /// fin: (no comment)
    /// vro: **@CODE@** allative
    #[strum(serialize = "All")]
    All,
    /// smj: (no comment)
    /// sms: **@CODE@** never last element of compound words
    /// sme: **@CODE@** from LEXICON GOADE-IU-
    #[strum(serialize = "Allegro")]
    Allegro,
    /// sme: (no comment)
    #[strum(serialize = "AltOrth/-bergslan")]
    AltOrth_SLASH__MINUS_bergslan,
    /// sme: (no comment)
    #[strum(serialize = "AltOrth/-standard")]
    AltOrth_SLASH__MINUS_standard,
    /// sme: (no comment)
    #[strum(serialize = "AltOrth/bergslan")]
    AltOrth_SLASH_bergslan,
    /// sme: (no comment)
    #[strum(serialize = "AltOrth/standard")]
    AltOrth_SLASH_standard,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Ani")]
    Ani,
    /// yrk: **@CODE@**
    #[strum(serialize = "Aor")]
    Aor,
    /// kpv: **@CODE@**:  Approximative numeral кавто-колмо, колмошка *two or three*   NB! do not confuse with Komi case +Apr
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    #[strum(serialize = "Appr")]
    Appr,
    /// kpv: **@CODE@** approximative -лань матыстчан
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Approximative
    #[strum(serialize = "Apr")]
    Apr,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Apr1")]
    Apr1,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Apr2")]
    Apr2,
    /// kpv: **@CODE@** approximative egressive -ланьсянь матысь ылыстчан
    #[strum(serialize = "AprEgr")]
    AprEgr,
    /// kpv: **@CODE@** approximative elative -ланьысь матысь петан
    #[strum(serialize = "AprEla")]
    AprEla,
    /// kpv: **@CODE@** approximative illative -ланьӧ матӧ матыстчан
    #[strum(serialize = "AprIll")]
    AprIll,
    /// kpv: **@CODE@** approximative inessive -ланьын матыс ина
    #[strum(serialize = "AprIne")]
    AprIne,
    /// kpv: **@CODE@** approximative prolative -ланьӧд маті вуджан
    #[strum(serialize = "AprPrl")]
    AprPrl,
    /// kpv: **@CODE@** approximative terminative -ланьӧдз матіӧдз воан
    #[strum(serialize = "AprTer")]
    AprTer,
    /// kpv: **@CODE@** approximative translative -ланьті маті вуджан
    #[strum(serialize = "AprTra")]
    AprTra,
    /// fit: **@CODE@**
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** = Arabic
    /// mhr: **@CODE@** = arabic numerals
    /// sma: **@CODE@** = Arabic numeral
    /// nob: **@CODE@**
    /// smj: **@CODE@** = arabic numerals
    /// sms: **@CODE@** - Arabic numeral
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** - Arabic numeral, subtag for +Num
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Arab")]
    Arab,
    /// sma: (no comment)
    /// smj: **@CODE@** - Used in Norway only
    #[strum(serialize = "Area/NO")]
    Area_SLASH_NO,
    /// sma: (no comment)
    /// smj: **@CODE@** - Used in Sweden only
    #[strum(serialize = "Area/SE")]
    Area_SLASH_SE,
    /// kpv: **@CODE@**:  +мезть
    #[strum(serialize = "Assoc")]
    Assoc,
    /// kpv: **@CODE@**:  -ne- ; avide-
    /// mhr: **@CODE@** = Collective associative numerals with obligatory possessive suffixes -нь-
    /// sms: **@CODE@**
    #[strum(serialize = "AssocColl")]
    AssocColl,
    /// mhr: **@CODE@** =
    #[strum(serialize = "AssocPl")]
    AssocPl,
    /// rus: (no comment)
    #[strum(serialize = "Att")]
    Att,
    /// fit: **@CODE@** = Attributive form, hmm, check, for names?
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Attributive form, hmm, check, for names?
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = attributive form
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  Attribute form
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Attributive form
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**   Attributive
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Attr")]
    Attr,
    /// yrk: **@CODE@**
    #[strum(serialize = "Aud")]
    Aud,
    /// kpv: **@CODE@**
    /// liv: **@CODE@** = Auxiliary verb
    /// mhr: **@CODE@** = Auxiliary verb
    /// yrk: **@CODE@** auxilliary verb
    /// sms: **@CODE@** = Auxiliary
    /// mrj: **@CODE@** auxiliary
    /// vep: **@CODE@**  = auxiliary verb
    #[strum(serialize = "Aux")]
    Aux,
    /// olo: **@CODE@**  Bahuvrihi
    #[strum(serialize = "Bahuv")]
    Bahuv,
    /// ceb: (no comment)
    #[strum(serialize = "Ben")]
    Ben,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Build")]
    Build,
    /// fit: **@CODE@** = Conjunction
    /// kpv: **@CODE@**:  coordinating conjunction      XX   сочинительный союз
    /// vot: (no comment)
    /// fkv: **@CODE@** = Conjunction
    /// liv: **@CODE@** = coordinating conjunction
    /// mhr: **@CODE@** = conjunctions
    /// mhr: (no comment)
    /// sma: **@CODE@** = Conjunction
    /// smj: **@CODE@** = Conjunction
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Conjunction
    /// lit: (no comment)
    /// vep: **@CODE@**  = coordinating conjunction
    /// olo: **@CODE@**  coordinating conjunction
    /// sme: **@CODE@** - Conjunction
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Conjunction
    /// vro: **@CODE@** Correlating conjunction
    #[strum(serialize = "CC")]
    CC,
    /// fit: **@CODE@** = Clause boundary
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Clause boundary
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = clause and sentence boundary symbols
    /// sma: **@CODE@**:  XXX These should be documented better
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Clause border (full stop, comma..)
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// zxx: (no comment)
    /// olo: **@CODE@**
    /// sme: **@CODE@**  Clause border (full stop, comma..)
    /// ceb: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "CLB")]
    CLB,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// sma: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// nob: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// smj: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// sms: **@CODE@** = Sentence final abbreviated expression ending in full stop, the full stop is ambiguous
    /// vep: **@CODE@**
    /// olo: **@CODE@**
    /// sme: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// fin: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// vro: **@CODE@**
    #[strum(serialize = "CLBfinal")]
    CLBfinal,
    /// rus: (no comment)
    #[strum(serialize = "COMMA")]
    COMMA,
    /// fit: **@CODE@** = Subjunction
    /// kpv: **@CODE@**:  subordinating conjunction      XX   подчинительный союз
    /// vot: (no comment)
    /// fkv: **@CODE@** = Subjunction
    /// liv: **@CODE@** = subordinating conjunction
    /// mhr: **@CODE@** = subjunctions
    /// sma: **@CODE@** = Subjunction
    /// nob: **@CODE@** = Closed POS (IM = **å**)
    /// smj: **@CODE@** = Subjunction
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// yrk: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@** = Subjunction
    /// lit: (no comment)
    /// vep: **@CODE@**  = subordinating conjunction
    /// olo: **@CODE@**  subordinating conjunction
    /// sme: **@CODE@** - Subjunction
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Subjunction
    /// vro: **@CODE@** subjunction
    #[strum(serialize = "CS")]
    CS,
    /// kpv: **@CODE@** caritive -тӧг торйӧдан
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    #[strum(serialize = "Car")]
    Car,
    /// kpv: **@CODE@**:  cardinal + NCard
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = (hmm, skip+Card?)
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    /// sme: **@CODE@**   Cardinal Number Not in use
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Card")]
    Card,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// nob: **@CODE@** = the so-called "genitive s"
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**  =
    /// vro: **@CODE@**
    #[strum(serialize = "Clt")]
    Clt,
    /// kpv: **@CODE@**  This comes at the end of a word -и or after vowels (some authors use -й)
    /// kpv: (no comment)
    #[strum(serialize = "Clt/I")]
    Clt_SLASH_I,
    /// kpv: **@CODE@** adjectival phrase
    #[strum(serialize = "Clt/Kodj")]
    Clt_SLASH_Kodj,
    /// kpv: **@CODE@** adverbial clause
    #[strum(serialize = "Clt/Moz")]
    Clt_SLASH_Moz,
    /// olo: (no comment)
    #[strum(serialize = "Clt/bo")]
    Clt_SLASH_bo,
    /// olo: (no comment)
    #[strum(serialize = "Clt/gi")]
    Clt_SLASH_gi,
    /// olo: (no comment)
    #[strum(serialize = "Clt/hAi")]
    Clt_SLASH_hAi,
    /// kpv: **@CODE@** -сӧ
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Clt/soe")]
    Clt_SLASH_soe,
    /// kpv: **@CODE@** -тӧ
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Clt/toe")]
    Clt_SLASH_toe,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Clth")]
    Clth,
    /// smj: (no comment)
    #[strum(serialize = "Cltl")]
    Cltl,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// mhr: **@CODE@** = nouns
    /// sma: **@CODE@** | Dynamic compound - this tag should *always* be part of a dynamic compound. It is important for  Apertium and the speller (to give extra weights to compounds), and useful in other cases as well.
    /// nob: **@CODE@**
    /// smj: **@CODE@**  Dynamic compound - this tag should always be part of a dynamic compound.
    /// sms: **@CODE@**
    /// sms: **@CODE@** - Dynamic compound. This tag should always be part
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// olo: **@CODE@** - Dynamic compound. This tag should always be part
    /// sme: **@CODE@** - Dynamic compound. This tag should always be part
    /// fin: **@CODE@** - Dynamic compound. This tag should always be part
    /// vro: (no comment)
    #[strum(serialize = "Cmp")]
    Cmp,
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Cmp#")]
    Cmp_POUND,
    /// sma: **@CODE@** | Compounding using attribute form
    /// smj: **@CODE@**  Attribute
    /// sms: **@CODE@** - Attributive
    /// olo: **@CODE@** compounds where first part is Attr
    /// sme: **@CODE@** - Attributive
    /// fin: **@CODE@** compounds where first part is Attr
    #[strum(serialize = "Cmp/Attr")]
    Cmp_SLASH_Attr,
    /// sma: **@CODE@** | Alle Cmp som har en attr-h
    #[strum(serialize = "Cmp/AttrH")]
    Cmp_SLASH_AttrH,
    /// sms: **@CODE@** - Tags citation compounds, which can in principle    cover any word. Requires a hyphen.
    /// sme: **@CODE@** - Tags citation compounds, which can in principle
    #[strum(serialize = "Cmp/Cit")]
    Cmp_SLASH_Cit,
    /// sma: **@CODE@** | Deletion of final **e**, as in *voelem-gaaroeh*, from *voeleme*
    #[strum(serialize = "Cmp/FinEDel")]
    Cmp_SLASH_FinEDel,
    /// fit: **@CODE@** - on dynamic compounds that have a hyphen (in use?)
    /// fkv: **@CODE@** - on dynamic compounds that have a hyphen (in use?)
    /// mhr: **@CODE@** = nouns
    /// sma: **@CODE@**:  A tag to indicate that a hyphen was used when
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - on dynamic compounds that have a hyphen
    /// vep: **@CODE@**
    /// olo: **@CODE@** compounds where first part ends in -
    /// sme: **@CODE@** - on dynamic compounds that have a hyphen
    /// fin: **@CODE@** compounds where first part ends in -
    /// vro: **@CODE@**
    #[strum(serialize = "Cmp/Hyph")]
    Cmp_SLASH_Hyph,
    /// smj: **@CODE@**  Long for om SgNom when short form is default, tjåhkaNIBMEbievddegirjje and bierjjeDAHKAiehket
    #[strum(serialize = "Cmp/Long")]
    Cmp_SLASH_Long,
    /// sms: **@CODE@** - On compounds that COULD have had a hyphen (and usually have), but doesn't
    /// sme: **@CODE@** - On compounds that COULD have had a hyphen (and usually have), but doesn't
    #[strum(serialize = "Cmp/NoHyph")]
    Cmp_SLASH_NoHyph,
    /// smj: (no comment)
    #[strum(serialize = "Cmp/OblHyph")]
    Cmp_SLASH_OblHyph,
    /// sma: **@CODE@** | Compounding using genitive plural
    /// smj: **@CODE@**  Plural Genitiv
    /// sms: **@CODE@** - Plural Genitiv
    /// olo: **@CODE@** compounds where first part is Pl Gen
    /// sme: **@CODE@** - Plural Genitiv
    /// fin: **@CODE@** compounds where first part is Pl Gen
    #[strum(serialize = "Cmp/PlGen")]
    Cmp_SLASH_PlGen,
    /// smj: **@CODE@**  Plural Nominative
    /// olo: **@CODE@** compounds where first part is Pl Nom
    /// fin: **@CODE@** compounds where first part is Pl Nom
    #[strum(serialize = "Cmp/PlNom")]
    Cmp_SLASH_PlNom,
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Cmp/Serial")]
    Cmp_SLASH_Serial,
    /// sma: **@CODE@** | Compounding using an unspecified singular stem
    /// smj: **@CODE@**  Singular
    #[strum(serialize = "Cmp/Sg")]
    Cmp_SLASH_Sg,
    /// sma: **@CODE@** | Compounding using genitive singular
    /// smj: **@CODE@**  Singular Genitive
    /// sms: **@CODE@** - Singular Genitive
    /// olo: **@CODE@** compounds where first part is Sg Gen
    /// sme: **@CODE@** - Singular Genitive
    /// fin: **@CODE@** compounds where first part is Sg Gen
    #[strum(serialize = "Cmp/SgGen")]
    Cmp_SLASH_SgGen,
    /// sma: **@CODE@** | Compounding using nominative singular
    /// smj: **@CODE@**  Singular Nominative
    /// sms: **@CODE@** - Singular Nominative
    /// vep: **@CODE@**  = compound words
    /// olo: **@CODE@** compounds where first part is Sg Nom
    /// sme: **@CODE@** - Singular Nominative
    /// fin: **@CODE@** compounds where first part is Sg Nom
    #[strum(serialize = "Cmp/SgNom")]
    Cmp_SLASH_SgNom,
    /// sma: **@CODE@** | Compounding using a short stem: *–biejj–* (from *biejjie*)
    /// smj: **@CODE@**  testing ShCmp
    /// sms: **@CODE@** - testing +Cmp/Sh
    /// olo: **@CODE@** compounds where first part is a short form
    /// sme: **@CODE@** - Tag for marking short form compound stems
    /// fin: **@CODE@** compounds where first part is a short form
    #[strum(serialize = "Cmp/Sh")]
    Cmp_SLASH_Sh,
    /// sma: **@CODE@** | Compounding using a short stem + **h**: *–biejjh–* (from *biejjie*), cf *reakedsbiejjhvadtese*
    #[strum(serialize = "Cmp/ShH")]
    Cmp_SLASH_ShH,
    /// mhr: **@CODE@** = nouns
    /// sms: **@CODE@** - Tags compounds containing SOFT HYPHENS (U+00AD)
    /// sme: **@CODE@** - Tags compounds containing SOFT HYPHENS (U+00AD)
    #[strum(serialize = "Cmp/SoftHyph")]
    Cmp_SLASH_SoftHyph,
    /// sma: **@CODE@** | This is a split compound with the other part to the left, this is the oposite of the previous case
    /// smj: **@CODE@**  This is a split compound with the other part to the left
    /// sms: **@CODE@** - This is a split compound with the other part to the left
    /// sms: (no comment)
    /// sme: **@CODE@** - This is a split compound with the split part to the right
    #[strum(serialize = "Cmp/SplitL")]
    Cmp_SLASH_SplitL,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// mhr: **@CODE@** = nouns
    /// sma: **@CODE@** | This is a split compound with the other part to the right: <br/> "Arbeids- og inkluderingsdepartementet" => *Arbeids–* = **+Cmp/SplitR**
    /// smj: **@CODE@**  This is a split compound with the other part to the right:
    /// sms: **@CODE@** - This is a split compound with the other part to
    /// vep: **@CODE@**
    /// olo: **@CODE@** compounds where first part is a split compound hmm
    /// sme: **@CODE@** - This is a split compound with the split part to
    /// fin: **@CODE@** compounds where first part is a split compound hmm
    #[strum(serialize = "Cmp/SplitR")]
    Cmp_SLASH_SplitR,
    /// smj: **@CODE@**  Unassimiled as first part of Cmp, e.g. telegram- in stead for telegrámma-
    #[strum(serialize = "Cmp/Unass")]
    Cmp_SLASH_Unass,
    /// sma: **@CODE@** | Alle Cmp som ikke har en klar klassifisering
    #[strum(serialize = "Cmp/XForm")]
    Cmp_SLASH_XForm,
    /// nob: **@CODE@** declaring both awaiting cleanup
    #[strum(serialize = "Cmp/e")]
    Cmp_SLASH_e,
    /// sma: **@CODE@** | Compound stem in **–ege**, as in *gaamege-gåaroje*
    #[strum(serialize = "Cmp/ege")]
    Cmp_SLASH_ege,
    /// sma: **@CODE@** | Compound stem in **–eh**, as in *gaameh-gåaroje*, from *gaamege*
    #[strum(serialize = "Cmp/eh")]
    Cmp_SLASH_eh,
    /// nob: **@CODE@** declaring both awaiting cleanup
    #[strum(serialize = "Cmp/null")]
    Cmp_SLASH_null,
    /// nob: **@CODE@**
    #[strum(serialize = "Cmp/s")]
    Cmp_SLASH_s,
    /// smj: **@CODE@** = Attributive
    #[strum(serialize = "CmpN/Attr")]
    CmpN_SLASH_Attr,
    /// smj: **@CODE@**  Left override
    #[strum(serialize = "CmpN/Def")]
    CmpN_SLASH_Def,
    /// smj: **@CODE@**  Overrides left tag, requires PlGen form
    #[strum(serialize = "CmpN/DefPlGen")]
    CmpN_SLASH_DefPlGen,
    /// smj: **@CODE@**  Overrides left tag, requires SgGen form
    #[strum(serialize = "CmpN/DefSgGen")]
    CmpN_SLASH_DefSgGen,
    /// liv: (no comment)
    /// sma: **@CODE@** | Plural Genitive
    /// smj: **@CODE@** = Plural Genitive
    /// sms: **@CODE@**
    /// sme: **@CODE@** Plural Genitive
    /// fin: **@CODE@** Plural Genitive
    #[strum(serialize = "CmpN/PlG")]
    CmpN_SLASH_PlG,
    /// sma: **@CODE@** | Pl Gen to the left
    /// smj: (no comment)
    /// sme: **@CODE@** Plural Genitive
    /// fin: **@CODE@** Plural Genitive
    #[strum(serialize = "CmpN/PlGenLeft")]
    CmpN_SLASH_PlGenLeft,
    /// sme: **@CODE@** Plural Nominative, propers!
    /// fin: **@CODE@** Plural Nominative, propers!
    #[strum(serialize = "CmpN/PlN")]
    CmpN_SLASH_PlN,
    /// liv: (no comment)
    /// sma: **@CODE@** | Singular
    /// smj: **@CODE@** = Singular
    #[strum(serialize = "CmpN/Sg")]
    CmpN_SLASH_Sg,
    /// liv: (no comment)
    /// sma: **@CODE@** | Singular Genitive
    /// smj: **@CODE@** = Singular Genitive
    /// sms: **@CODE@**
    /// sme: **@CODE@** Singular Genitive
    /// fin: **@CODE@** Singular Genitive
    /// vro: **@CODE@** Singular Genitive
    #[strum(serialize = "CmpN/SgG")]
    CmpN_SLASH_SgG,
    /// sma: **@CODE@** | Sg Gen to the left
    /// smj: (no comment)
    /// sme: **@CODE@** Singular Genitive
    /// fin: **@CODE@** Singular Genitive
    #[strum(serialize = "CmpN/SgGenLeft")]
    CmpN_SLASH_SgGenLeft,
    /// sma: **@CODE@** | Sg to the left
    /// smj: (no comment)
    #[strum(serialize = "CmpN/SgLeft")]
    CmpN_SLASH_SgLeft,
    /// liv: (no comment)
    /// sma: **@CODE@** | Singular Nominative
    /// smj: **@CODE@** = Singular Nominative
    /// sms: **@CODE@**
    /// sme: **@CODE@** Singular Nominative
    /// fin: **@CODE@** Singular Nominative
    /// vro: **@CODE@** Singular Nominative
    #[strum(serialize = "CmpN/SgN")]
    CmpN_SLASH_SgN,
    /// sma: **@CODE@** | Sg Nom to the left
    /// smj: (no comment)
    /// sme: **@CODE@** Singular Nominative
    /// fin: **@CODE@** Singular Nominative
    #[strum(serialize = "CmpN/SgNomLeft")]
    CmpN_SLASH_SgNomLeft,
    /// liv: (no comment)
    /// sma: **@CODE@** | ... be in all positions, **default**, this tag does not have to be written
    /// nob: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    /// smj: (no comment)
    /// sme: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    /// fin: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    #[strum(serialize = "CmpNP/All")]
    CmpNP_SLASH_All,
    /// fit: **@CODE@** - ... only be first part in a compound or alone
    /// fkv: **@CODE@** - ... only be first part in a compound or alone
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be first part in a compound or alone
    /// nob: **@CODE@** - ... only be first part in a compound or alone
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** - ... only be first part in a compound or alone
    /// fin: **@CODE@** - ... only be first part in a compound or alone
    #[strum(serialize = "CmpNP/First")]
    CmpNP_SLASH_First,
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be last part in a compound or alone
    /// nob: **@CODE@** - ... only be last part in a compound or alone
    /// smj: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@** - ... only be last part in a compound or alone
    /// fin: **@CODE@** - ... only be last part in a compound or alone
    #[strum(serialize = "CmpNP/Last")]
    CmpNP_SLASH_Last,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** =
    /// liv: (no comment)
    /// sma: **@CODE@** | ... not take part in compounds
    /// nob: **@CODE@** - ... does not take part in compounds
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** - ... does not take part in compounds
    /// fin: **@CODE@** - ... does not take part in compounds
    #[strum(serialize = "CmpNP/None")]
    CmpNP_SLASH_None,
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be part of a compound, i.e. can never be used alone, but can appear in any position
    /// nob: **@CODE@** - ... only be part of a compound, i.e. can never
    /// smj: (no comment)
    /// sme: **@CODE@** - ... only be part of a compound, i.e. can never
    /// fin: **@CODE@** - ... only be part of a compound, i.e. can never
    #[strum(serialize = "CmpNP/Only")]
    CmpNP_SLASH_Only,
    /// liv: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be **first** part in a compound, NEVER alone
    /// nob: **@CODE@** - ... only **first** part in a compound, NEVER alone
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// sme: **@CODE@** - ... only **first** part in a compound, NEVER alone
    /// fin: **@CODE@** - ... only **first** part in a compound, NEVER alone
    #[strum(serialize = "CmpNP/Pref")]
    CmpNP_SLASH_Pref,
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be **last** part in a compound, NEVER alone
    /// nob: **@CODE@** - ... only **last** part in a compound, NEVER alone
    /// smj: (no comment)
    /// sme: **@CODE@** - ... only **last** part in a compound, NEVER alone
    /// fin: **@CODE@** - ... only **last** part in a compound, NEVER alone
    #[strum(serialize = "CmpNP/Suff")]
    CmpNP_SLASH_Suff,
    /// rus: (no comment)
    #[strum(serialize = "Cmpar")]
    Cmpar,
    /// kpv: **@CODE@**  Postposition complement
    #[strum(serialize = "Cmpl")]
    Cmpl,
    /// rus: (no comment)
    #[strum(serialize = "Cmpnd")]
    Cmpnd,
    /// kpv: **@CODE@** Comparative case form -ся ӧткодялан
    /// mhr: **@CODE@** = comparative case
    #[strum(serialize = "Cmpr")]
    Cmpr,
    /// fin: (no comment)
    #[strum(serialize = "Cmt")]
    Cmt,
    /// kpv: **@CODE@** consecultative -ла могман
    #[strum(serialize = "Cns")]
    Cns,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** = Collective numeral
    /// mhr: **@CODE@** = Collective numerals -ын-
    /// sma: **@CODE@** = Collective numeral
    /// nob: **@CODE@**
    /// smj: **@CODE@** = collective numerals
    /// rus: (no comment)
    /// sms: **@CODE@** - Collective numerals
    /// vep: **@CODE@**
    /// olo: **@CODE@**  collective, probably from a common file.
    /// sme: **@CODE@** - Collective numerals, subtag for +N
    /// fin: (no comment)
    #[strum(serialize = "Coll")]
    Coll,
    /// kpv: **@CODE@** used with paired nouns **collective nouns**
    #[strum(serialize = "CollN")]
    CollN,
    /// fit: **@CODE@** = Comitative
    /// kpv: **@CODE@** Comitative -кӧд ӧтвывтан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Comitative
    /// mhr: **@CODE@** = comitative
    /// sma: **@CODE@**= Comitative
    /// smj: **@CODE@** = Comitative case
    /// sms: **@CODE@** Comitative
    /// lit: (no comment)
    /// vep: **@CODE@**
    /// olo: **@CODE@** comitative
    /// sme: **@CODE@** - Comitative
    /// fin: (no comment)
    /// vro: **@CODE@** comitative
    #[strum(serialize = "Com")]
    Com,
    /// vot: (no comment)
    /// lit: (no comment)
    /// sme: **@CODE@** - Comitative Plural Hyphened Shortform (w/o -guin), ie Beatnagii-, Biillai-, Bohccui- etc.
    /// vro: **@CODE@**
    #[strum(serialize = "Com/Sh")]
    Com_SLASH_Sh,
    /// ceb: (no comment)
    #[strum(serialize = "Comm")]
    Comm,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@** джык
    /// kpv: (no comment)
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = comparative (not: not Cmp)
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    /// fin: (no comment)
    /// ceb: **@CODE@** -
    /// ceb: **@CODE@** comparative mas -- this is not a derivation but a separate word
    /// vro: **@CODE@**
    #[strum(serialize = "Comp")]
    Comp,
    /// kpv: **@CODE@**
    #[strum(serialize = "CompMod")]
    CompMod,
    /// fkv: **@CODE@** = Comparative
    #[strum(serialize = "Compar")]
    Compar,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Negation form of verb
    /// liv: **@CODE@** = connegative, main verb complement to Neg,
    /// mhr: **@CODE@** = Invariant main verb in negation expression
    /// sma: **@CODE@** | main verb complement to Neg, form identical to Imp
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Negation Form, i.e. mõõn, reäǥǥ, jueʹjj: Indicative present except indef person; imperative Sg2 and Pl2
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// olo: **@CODE@**
    /// sme: **@CODE@** Negation Form, ie Mana, Doalvvo, Juoge etc
    /// fin: (no comment)
    /// vro: **@CODE@** saa eiq 3 elements in 2 orthographic units
    #[strum(serialize = "ConNeg")]
    ConNeg,
    /// vot: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Negation Form II: mõnnu, riâkˈku, juõkˈku: Indicative present indef person; imperative Sg3, Pl1, Pl2, Pl3 (there is an overlap at +Impert+Pl2)
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// sme: **@CODE@** Alternative, Rather Declamatory Negation Form - Infrequent
    /// vro: **@CODE@** ei saaq 3 elements in 2 orthographic units
    #[strum(serialize = "ConNegII")]
    ConNegII,
    /// kpv: **@CODE@** Used for calling animals, for example брысь, баль-баль, ...
    #[strum(serialize = "Conative")]
    Conative,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Conditional
    /// liv: **@CODE@** = conditional
    /// mhr: **@CODE@** = conditional
    /// sma: **@CODE@** | Kondisjonalis, for one form:  lidtjie. To be looked at.+ lidtjim, + lidtjih
    /// smj: (no comment)
    /// sms: **@CODE@** Conditional mood
    /// lit: (no comment)
    /// vep: **@CODE@**  = conditional
    /// olo: **@CODE@** Conditional
    /// sme: **@CODE@** Conditional
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Cond")]
    Cond,
    /// yrk: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    #[strum(serialize = "Conj")]
    Conj,
    /// ceb: **@CODE@** -
    #[strum(serialize = "Cont")]
    Cont,
    /// kpv: **@CODE@**:  Coordinates, i.e. 65˚36′8,30″ in numerals.lexc
    #[strum(serialize = "Coord")]
    Coord,
    /// kpv: **@CODE@**:  this is for copula complement predicate position with pl in -ӧсь depricated Pred
    #[strum(serialize = "Cop")]
    Cop,
    /// rus: (no comment)
    #[strum(serialize = "Count")]
    Count,
    /// rus: (no comment)
    #[strum(serialize = "DASH")]
    DASH,
    /// fin: (no comment)
    #[strum(serialize = "Dash")]
    Dash,
    /// kpv: **@CODE@** dative case -лы сетан
    /// liv: **@CODE@** = dative case
    /// mhr: **@CODE@** = dative
    /// nob: **@CODE@** = for fixed expressions *i live*
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = dative case
    /// olo: **@CODE@** dative case
    #[strum(serialize = "Dat")]
    Dat,
    /// liv: **@CODE@** =
    #[strum(serialize = "Deb")]
    Deb,
    /// rus: (no comment)
    #[strum(serialize = "Def")]
    Def,
    /// kpv: **@CODE@** Degree depricate AdA
    /// olo: (no comment)
    /// vro: **@CODE@** adjective or adverb modifier. This is degree, depricate + AdA
    #[strum(serialize = "Deg")]
    Deg,
    /// fit: **@CODE@** = Demonstrative
    /// kpv: **@CODE@**:  demonstrative
    /// vot: (no comment)
    /// fkv: **@CODE@** = Demonstrative
    /// liv: **@CODE@** = demonstrative
    /// mhr: **@CODE@** = Demonstrative pronoun
    /// sma: **@CODE@** = Demonstrative
    /// smj: **@CODE@** = Demonstrative pronoun
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Demonstrative Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  demonstrative
    /// vep: **@CODE@**  = demonstrative
    /// olo: **@CODE@** demonstrative
    /// sme: **@CODE@** - Demonstrative Pronoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Dem")]
    Dem,
    /// mhr: **@CODE@** = ( pair verbs that do not occur independently get this marker.) /was +Depend, but +Dep used in fst.
    /// mrj: **@CODE@**:  dependent word requiring the presence of noun, e.g. ӹшке
    /// olo: **@CODE@** dependent word requiring the presence of another, e.g. **мень**
    #[strum(serialize = "Dep")]
    Dep,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**  In front of every derivation to make it   possible to target derivations as a class e.g. in regular expressions etc
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// fkv: **@CODE@** =
    /// liv: (no comment)
    /// mhr: **@CODE@** = This allows for Ex/... forms
    /// mhr: (no comment)
    /// sma: **@CODE@**:  Tag to precede any non-positional derivation
    /// nob: **@CODE@** = mark derivation
    /// smj: (no comment)
    /// rus: (no comment)
    /// sms: **@CODE@**
    /// sms: **@CODE@** This ordinal derivation
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Der")]
    Der,
    /// fit: **@CODE@** =
    /// kpv: (no comment)
    /// fkv: **@CODE@** = deriving adjectives from verbs
    /// liv: **@CODE@** for example present participle to adjective
    /// sma: (no comment)
    /// sms: (no comment)
    /// olo: **@CODE@** Adjective derivation
    /// olo: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@** = Adjective derivated from Noun or Verb
    #[strum(serialize = "Der/A")]
    Der_SLASH_A,
    /// nob: **@CODE@** = Adjectives are also adverbs
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/AAdv")]
    Der_SLASH_AAdv,
    /// olo: **@CODE@** A>>N derivation
    #[strum(serialize = "Der/AN")]
    Der_SLASH_AN,
    /// kpv: **@CODE@** тӧм should take +A, see also +VAbess
    #[strum(serialize = "Der/Abe")]
    Der_SLASH_Abe,
    /// kpv: (no comment)
    /// sme: **@CODE@** = Adverb derivated from Adjective
    #[strum(serialize = "Der/Adv")]
    Der_SLASH_Adv,
    /// kpv: **@CODE@**  Process Participle +AN
    #[strum(serialize = "Der/An")]
    Der_SLASH_An,
    /// kpv: **@CODE@** Process Participle +ANA, Gerund or participle according to context (with...)
    #[strum(serialize = "Der/Ana")]
    Der_SLASH_Ana,
    /// kpv: **@CODE@**  adverb derived from participle (+ANA) +ANAA
    #[strum(serialize = "Der/Anaa")]
    Der_SLASH_Anaa,
    /// sma: **@CODE@** |            |            |      | -hts, Caritive, was Der/heapmi in sme
    /// smj: (no comment)
    /// sms: **@CODE@**  N»A -tem, -teʹmes
    /// sme: (no comment)
    #[strum(serialize = "Der/Car")]
    Der_SLASH_Car,
    /// mhr: **@CODE@** = Derivation V > V: Causative
    /// smj: (no comment)
    /// sms: **@CODE@** causative V»V, was Der/ted and Der/âʹtted
    /// sme: (no comment)
    #[strum(serialize = "Der/Caus")]
    Der_SLASH_Caus,
    /// sma: **@CODE@** |            |            | AA   | adjektiv
    /// smj: (no comment)
    /// sms: **@CODE@** comparative as in other Sami languages, depricate +Comp
    /// sme: (no comment)
    #[strum(serialize = "Der/Comp")]
    Der_SLASH_Comp,
    /// yrk: (no comment)
    #[strum(serialize = "Der/Cop")]
    Der_SLASH_Cop,
    /// sma: **@CODE@** |            |            | NN   | Diminutive
    /// smj: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** diminutive derivation N»N was Der/Dim,
    /// sme: (no comment)
    #[strum(serialize = "Der/Dimin")]
    Der_SLASH_Dimin,
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Der/IA")]
    Der_SLASH_IA,
    /// kpv: **@CODE@** иг
    #[strum(serialize = "Der/Ig")]
    Der_SLASH_Ig,
    /// kpv: **@CODE@**%{иі%}гчӧж
    #[strum(serialize = "Der/IgChozh")]
    Der_SLASH_IgChozh,
    /// kpv: **@CODE@**
    #[strum(serialize = "Der/IgKezhlo")]
    Der_SLASH_IgKezhlo,
    /// kpv: **@CODE@**
    #[strum(serialize = "Der/IgKosta")]
    Der_SLASH_IgKosta,
    /// kpv: **@CODE@**
    #[strum(serialize = "Der/IgKosti")]
    Der_SLASH_IgKosti,
    /// kpv: **@CODE@** %{иі%}гмоз
    #[strum(serialize = "Der/IgMoz")]
    Der_SLASH_IgMoz,
    /// kpv: **@CODE@** %{иі%}гсор
    #[strum(serialize = "Der/IgSor")]
    Der_SLASH_IgSor,
    /// kpv: **@CODE@** %{иі%}гтыр
    #[strum(serialize = "Der/IgTyr")]
    Der_SLASH_IgTyr,
    /// kpv: **@CODE@** %{иі%}гтыръя
    #[strum(serialize = "Der/IgTyrja")]
    Der_SLASH_IgTyrja,
    /// kpv: **@CODE@** %{иі%}гтырйи
    #[strum(serialize = "Der/IgTyrji")]
    Der_SLASH_IgTyrji,
    /// kpv: **@CODE@** %{иі%}гӧнмоз
    #[strum(serialize = "Der/IgonMoz")]
    Der_SLASH_IgonMoz,
    /// kpv: **@CODE@** Ин
    #[strum(serialize = "Der/In")]
    Der_SLASH_In,
    /// sma: **@CODE@** | VV   | Inchoative
    /// smj: (no comment)
    /// sms: **@CODE@** incoative V»V -škueʹtted
    /// sme: (no comment)
    #[strum(serialize = "Der/InchL")]
    Der_SLASH_InchL,
    /// kpv: **@CODE@** Иник
    #[strum(serialize = "Der/Inik")]
    Der_SLASH_Inik,
    /// vro: **@CODE@**	NomAg
    #[strum(serialize = "Der/JA")]
    Der_SLASH_JA,
    /// kpv: **@CODE@**
    #[strum(serialize = "Der/La")]
    Der_SLASH_La,
    /// kpv: (no comment)
    #[strum(serialize = "Der/LocMod")]
    Der_SLASH_LocMod,
    /// kpv: **@CODE@**
    /// mhr: **@CODE@** = Modifier without noun (better: +A+Sg+Nom etc.)
    /// yrk: **@CODE@** modifier without noun head
    /// olo: **@CODE@** Modifier without Noun head
    #[strum(serialize = "Der/MWN")]
    Der_SLASH_MWN,
    /// vep: **@CODE@**  = V»A
    #[strum(serialize = "Der/Matoi")]
    Der_SLASH_Matoi,
    /// vep: **@CODE@**  = V»N
    #[strum(serialize = "Der/Mine")]
    Der_SLASH_Mine,
    /// kpv: (no comment)
    /// sms: (no comment)
    /// mrj: **@CODE@** =        !! Noun derivation
    #[strum(serialize = "Der/N")]
    Der_SLASH_N,
    /// sms: (no comment)
    #[strum(serialize = "Der/N2A")]
    Der_SLASH_N2A,
    /// mhr: **@CODE@** =  Derivation V > N: Nominalization
    #[strum(serialize = "Der/Nom")]
    Der_SLASH_Nom,
    /// kpv: **@CODE@**  +Event
    /// liv: **@CODE@**
    /// sma: **@CODE@** | VN   | Nomen Actionis
    /// nob: **@CODE@** = verb +ing
    /// smj: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** action V»N +Der/m
    /// sme: (no comment)
    #[strum(serialize = "Der/NomAct")]
    Der_SLASH_NomAct,
    /// kpv: **@CODE@**
    /// liv: **@CODE@**
    /// sma: **@CODE@** | VN   | Nomen Agentis
    /// smj: (no comment)
    /// sms: **@CODE@** agent V»N
    /// vep: **@CODE@**  = tehta : tegii
    /// sme: (no comment)
    #[strum(serialize = "Der/NomAg")]
    Der_SLASH_NomAg,
    /// smj: (no comment)
    #[strum(serialize = "Der/NomInstr")]
    Der_SLASH_NomInstr,
    /// mhr: **@CODE@** = Derivation V > N: Negative nominalization
    #[strum(serialize = "Der/NomNeg")]
    Der_SLASH_NomNeg,
    /// sms: **@CODE@** Num»A ordinal
    #[strum(serialize = "Der/Ord")]
    Der_SLASH_Ord,
    /// sms: **@CODE@** rupp » ruʹppi ATTR ruʹppes DEPRICATE
    #[strum(serialize = "Der/PALi")]
    Der_SLASH_PALi,
    /// smj: (no comment)
    #[strum(serialize = "Der/PassD")]
    Der_SLASH_PassD,
    /// sma: **@CODE@** | VV   | long only
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/PassL")]
    Der_SLASH_PassL,
    /// sma: (no comment)
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/PassS")]
    Der_SLASH_PassS,
    /// kpv: **@CODE@** patronymics in Russian
    #[strum(serialize = "Der/Patr")]
    Der_SLASH_Patr,
    /// mhr: **@CODE@** =   Derivation N > A: Possessive adjective, orig. genitive form without a head
    #[strum(serialize = "Der/Poss")]
    Der_SLASH_Poss,
    /// yrk: **@CODE@** this is used with predication of nominals and deverbal modalities
    #[strum(serialize = "Der/Pr")]
    Der_SLASH_Pr,
    /// mhr: **@CODE@** =  Derivation N > A: Privative adjective
    #[strum(serialize = "Der/Priv")]
    Der_SLASH_Priv,
    /// kpv: **@CODE@** = тӧм
    #[strum(serialize = "Der/PrivMod")]
    Der_SLASH_PrivMod,
    /// kpv: **@CODE@** = +Der/APrior  Denominal prioritive adjective Der/a
    #[strum(serialize = "Der/ProprietiveMod")]
    Der_SLASH_ProprietiveMod,
    /// rus: (no comment)
    #[strum(serialize = "Der/PrsAct")]
    Der_SLASH_PrsAct,
    /// rus: (no comment)
    #[strum(serialize = "Der/PrsPss")]
    Der_SLASH_PrsPss,
    /// rus: (no comment)
    #[strum(serialize = "Der/PstAct")]
    Der_SLASH_PstAct,
    /// rus: (no comment)
    #[strum(serialize = "Der/PstPss")]
    Der_SLASH_PstPss,
    /// mhr: **@CODE@** =   Derivation N > A:
    #[strum(serialize = "Der/Pur")]
    Der_SLASH_Pur,
    /// olo: **@CODE@** used in verbs for deriving reflexive conjugation
    #[strum(serialize = "Der/Rc")]
    Der_SLASH_Rc,
    /// sma: **@CODE@** |            |            | NN   | Forholdsformer
    #[strum(serialize = "Der/Rec")]
    Der_SLASH_Rec,
    /// mhr: **@CODE@** = Derivation V > V: Reflexive
    #[strum(serialize = "Der/Refl")]
    Der_SLASH_Refl,
    /// mhr: **@CODE@** =  Derivation N > A: Relational adjective
    #[strum(serialize = "Der/Rel")]
    Der_SLASH_Rel,
    /// sma: **@CODE@** |            |            | AA   | adjektiv
    /// smj: (no comment)
    /// sms: **@CODE@** superlative previously Der/mos Der/umus Der/ummus, depricate +Superl
    /// sme: (no comment)
    /// ceb: **@CODE@** superlative prefix pinaka%<
    #[strum(serialize = "Der/Superl")]
    Der_SLASH_Superl,
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Der/Ta")]
    Der_SLASH_Ta,
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Der/Te")]
    Der_SLASH_Te,
    /// kpv: **@CODE@** TempMod Der/sja но и Ф. В. Плесовскийлысь квайтымынӧд вояссяяссӧ * позьӧ аддзыны сӧмын библиотекаясысь.  Declaring spatial adverb derivations; see also spatial postpositions
    #[strum(serialize = "Der/TempMod")]
    Der_SLASH_TempMod,
    /// vep: **@CODE@**  = nime»toi N»A
    #[strum(serialize = "Der/Toi")]
    Der_SLASH_Toi,
    /// olo: **@CODE@** with numerals 11-19
    #[strum(serialize = "Der/Tostu")]
    Der_SLASH_Tostu,
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Der/Tu")]
    Der_SLASH_Tu,
    /// olo: **@CODE@** A>>N derivation (+Der/AN better?)
    /// vro: **@CODE@** A→N
    #[strum(serialize = "Der/Us")]
    Der_SLASH_Us,
    /// vep: **@CODE@**  = sur»uz' A»N
    #[strum(serialize = "Der/Uz1")]
    Der_SLASH_Uz1,
    /// vep: **@CODE@**  = V»V
    #[strum(serialize = "Der/V")]
    Der_SLASH_V,
    /// sms: (no comment)
    #[strum(serialize = "Der/V2A")]
    Der_SLASH_V2A,
    /// sms: (no comment)
    #[strum(serialize = "Der/V2Adv")]
    Der_SLASH_V2Adv,
    /// liv: **@CODE@**
    #[strum(serialize = "Der/VN")]
    Der_SLASH_VN,
    /// kpv: **@CODE@** а
    #[strum(serialize = "Der/a")]
    Der_SLASH_a,
    /// sms: **@CODE@** ordinals to nouns
    #[strum(serialize = "Der/ad")]
    Der_SLASH_ad,
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/adda")]
    Der_SLASH_adda,
    /// sma: **@CODE@** |            | VV   | Frequentative, Kontinuativ
    #[strum(serialize = "Der/adte")]
    Der_SLASH_adte,
    /// smj: (no comment)
    #[strum(serialize = "Der/ahkes")]
    Der_SLASH_ahkes,
    /// smj: (no comment)
    #[strum(serialize = "Der/ahtes")]
    Der_SLASH_ahtes,
    /// smj: (no comment)
    #[strum(serialize = "Der/ahtja")]
    Der_SLASH_ahtja,
    /// sma: **@CODE@** | VV   | Inchoative
    #[strum(serialize = "Der/ahtje")]
    Der_SLASH_ahtje,
    /// smj: (no comment)
    #[strum(serialize = "Der/ahttjá")]
    Der_SLASH_ahttjá,
    /// smj: (no comment)
    #[strum(serialize = "Der/akti")]
    Der_SLASH_akti,
    /// kpv: **@CODE@** ал
    #[strum(serialize = "Der/al")]
    Der_SLASH_al,
    /// sma: **@CODE@** |            | VV   | Frequentative
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/alla")]
    Der_SLASH_alla,
    /// sms: (no comment)
    #[strum(serialize = "Der/allash")]
    Der_SLASH_allash,
    /// sme: (no comment)
    #[strum(serialize = "Der/amoš")]
    Der_SLASH_amoš,
    /// kpv: **@CODE@** ась
    #[strum(serialize = "Der/asj")]
    Der_SLASH_asj,
    /// smj: (no comment)
    #[strum(serialize = "Der/asste")]
    Der_SLASH_asste,
    /// kpv: **@CODE@** бердса
    #[strum(serialize = "Der/berdsa")]
    Der_SLASH_berdsa,
    /// kpv: **@CODE@** бӧрса
    #[strum(serialize = "Der/bwrsa")]
    Der_SLASH_bwrsa,
    /// sms: **@CODE@** frequentative V»V for ʹCCed verbs -čed
    #[strum(serialize = "Der/ched")]
    Der_SLASH_ched,
    /// kpv: **@CODE@**  чӧж +CHOZH
    #[strum(serialize = "Der/chwzh")]
    Der_SLASH_chwzh,
    /// kpv: **@CODE@** чы This appears to be a variant of +Der/sjy; it follows plosives
    #[strum(serialize = "Der/chy")]
    Der_SLASH_chy,
    /// sma: **@CODE@** |            |            | VV   | Continuative, Konative, Frequentative, Refleksive, Momentan
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/d")]
    Der_SLASH_d,
    /// smj: (no comment)
    #[strum(serialize = "Der/dahka")]
    Der_SLASH_dahka,
    /// smj: (no comment)
    #[strum(serialize = "Der/dahtte")]
    Der_SLASH_dahtte,
    /// smj: (no comment)
    #[strum(serialize = "Der/dalla")]
    Der_SLASH_dalla,
    /// smj: (no comment)
    #[strum(serialize = "Der/dasste")]
    Der_SLASH_dasste,
    /// kpv: **@CODE@** дор
    #[strum(serialize = "Der/dor")]
    Der_SLASH_dor,
    /// kpv: **@CODE@** дорса
    #[strum(serialize = "Der/dorsa")]
    Der_SLASH_dorsa,
    /// smj: (no comment)
    #[strum(serialize = "Der/duhtte")]
    Der_SLASH_duhtte,
    /// smj: (no comment)
    #[strum(serialize = "Der/duvva")]
    Der_SLASH_duvva,
    /// sme: (no comment)
    #[strum(serialize = "Der/eamoš")]
    Der_SLASH_eamoš,
    /// sme: (no comment)
    #[strum(serialize = "Der/easti")]
    Der_SLASH_easti,
    /// sma: **@CODE@** |            | NA   | Attributt
    #[strum(serialize = "Der/eds")]
    Der_SLASH_eds,
    /// smj: (no comment)
    #[strum(serialize = "Der/ferjak")]
    Der_SLASH_ferjak,
    /// sme: (no comment)
    #[strum(serialize = "Der/geahtes")]
    Der_SLASH_geahtes,
    /// smj: (no comment)
    #[strum(serialize = "Der/gusj")]
    Der_SLASH_gusj,
    /// kpv: **@CODE@** гӧгӧрса
    #[strum(serialize = "Der/gwgwrsa")]
    Der_SLASH_gwgwrsa,
    /// sme: (no comment)
    #[strum(serialize = "Der/h")]
    Der_SLASH_h,
    /// sme: (no comment)
    #[strum(serialize = "Der/halla")]
    Der_SLASH_halla,
    /// sme: (no comment)
    #[strum(serialize = "Der/has")]
    Der_SLASH_has,
    /// sma: **@CODE@** |            |            | VV   | Causative
    #[strum(serialize = "Der/ht")]
    Der_SLASH_ht,
    /// sma: **@CODE@** |            |            | VV   | Passive, frekeventative
    #[strum(serialize = "Der/htalle")]
    Der_SLASH_htalle,
    /// sma: **@CODE@** |            |            | NN   | Dim-cont, Frequentative
    #[strum(serialize = "Der/htj")]
    Der_SLASH_htj,
    /// sme: (no comment)
    #[strum(serialize = "Der/huhtti")]
    Der_SLASH_huhtti,
    /// sme: (no comment)
    #[strum(serialize = "Der/huvva")]
    Der_SLASH_huvva,
    /// sms: **@CODE@**  N»A luõss >> luõʹssi, čääʹcc >> čäccai 'rich in' Sg.Ill
    #[strum(serialize = "Der/i")]
    Der_SLASH_i,
    /// sma: **@CODE@** |            |            | VA   | (Handlernomen- tilbøyelig til å utføre den handlingen som grunnordet angir)
    #[strum(serialize = "Der/ihks")]
    Der_SLASH_ihks,
    /// sma: **@CODE@** |            |            | NA   | Nomen agentis
    #[strum(serialize = "Der/ijes")]
    Der_SLASH_ijes,
    /// fin: (no comment)
    #[strum(serialize = "Der/inen")]
    Der_SLASH_inen,
    /// fin: (no comment)
    #[strum(serialize = "Der/ja")]
    Der_SLASH_ja,
    /// sms: **@CODE@** inchoative V»V also middle
    #[strum(serialize = "Der/jed")]
    Der_SLASH_jed,
    /// smj: (no comment)
    #[strum(serialize = "Der/k")]
    Der_SLASH_k,
    /// sme: (no comment)
    #[strum(serialize = "Der/keahtta")]
    Der_SLASH_keahtta,
    /// kpv: **@CODE@**  +KEZHLO
    #[strum(serialize = "Der/kezhlw")]
    Der_SLASH_kezhlw,
    /// kpv: **@CODE@** кодь diminishing, kind of, sort of
    #[strum(serialize = "Der/kodj")]
    Der_SLASH_kodj,
    /// kpv: **@CODE@**  +KOSTA
    #[strum(serialize = "Der/kosta")]
    Der_SLASH_kosta,
    /// kpv: **@CODE@**  +KOSTI
    #[strum(serialize = "Der/kosti")]
    Der_SLASH_kosti,
    /// kpv: **@CODE@** костса
    #[strum(serialize = "Der/kostsa")]
    Der_SLASH_kostsa,
    /// kpv: **@CODE@** л
    /// sma: **@CODE@** |            |            | VV   | Subitive
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/l")]
    Der_SLASH_l,
    /// kpv: **@CODE@** ла
    #[strum(serialize = "Der/la")]
    Der_SLASH_la,
    /// sme: (no comment)
    #[strum(serialize = "Der/laagasj")]
    Der_SLASH_laagasj,
    /// sma: **@CODE@** |            |            | AAdv | adverb
    /// sme: (no comment)
    #[strum(serialize = "Der/laakan")]
    Der_SLASH_laakan,
    /// sma: **@CODE@** |            |            | AA   | adjektiv
    #[strum(serialize = "Der/laaketje")]
    Der_SLASH_laaketje,
    /// smj: (no comment)
    #[strum(serialize = "Der/ladda")]
    Der_SLASH_ladda,
    /// kpv: **@CODE@** ладор
    #[strum(serialize = "Der/lador")]
    Der_SLASH_lador,
    /// smj: (no comment)
    #[strum(serialize = "Der/lahtte")]
    Der_SLASH_lahtte,
    /// fit: **@CODE@** =
    /// fin: (no comment)
    #[strum(serialize = "Der/lainen")]
    Der_SLASH_lainen,
    /// sme: (no comment)
    #[strum(serialize = "Der/las")]
    Der_SLASH_las,
    /// smj: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Der/lasj")]
    Der_SLASH_lasj,
    /// smj: (no comment)
    #[strum(serialize = "Der/lasste")]
    Der_SLASH_lasste,
    /// sms: (no comment)
    #[strum(serialize = "Der/lazh")]
    Der_SLASH_lazh,
    /// sma: **@CODE@** |            |            | VA   | Resultatnomen (?)
    #[strum(serialize = "Der/ldahke")]
    Der_SLASH_ldahke,
    /// sma: **@CODE@** |            |            | VA   | Attributt
    #[strum(serialize = "Der/ldh")]
    Der_SLASH_ldh,
    /// sma: **@CODE@** |            |            | VA   |
    #[strum(serialize = "Der/ldihkie")]
    Der_SLASH_ldihkie,
    /// sma: **@CODE@** |            |            | VA   | Intensive
    #[strum(serialize = "Der/les")]
    Der_SLASH_les,
    /// sma: **@CODE@** |            |            | VV   | Passive
    #[strum(serialize = "Der/lg")]
    Der_SLASH_lg,
    /// smj: (no comment)
    #[strum(serialize = "Der/lis")]
    Der_SLASH_lis,
    /// fin: (no comment)
    #[strum(serialize = "Der/llinen")]
    Der_SLASH_llinen,
    /// vro: **@CODE@** A→Adv
    #[strum(serialize = "Der/lt")]
    Der_SLASH_lt,
    /// kpv: **@CODE@**  лун adjective-to-noun
    #[strum(serialize = "Der/lun")]
    Der_SLASH_lun,
    /// kpv: **@CODE@** лы
    #[strum(serialize = "Der/ly")]
    Der_SLASH_ly,
    /// kpv: **@CODE@** лывлы
    #[strum(serialize = "Der/lyvly")]
    Der_SLASH_lyvly,
    /// olo: (no comment)
    #[strum(serialize = "Der/mA")]
    Der_SLASH_mA,
    /// mhr: **@CODE@** =
    #[strum(serialize = "Der/mO")]
    Der_SLASH_mO,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Der/ma")]
    Der_SLASH_ma,
    /// fin: (no comment)
    #[strum(serialize = "Der/maisilla")]
    Der_SLASH_maisilla,
    /// olo: **@CODE@** comparative
    #[strum(serialize = "Der/mbi")]
    Der_SLASH_mbi,
    /// sme: (no comment)
    #[strum(serialize = "Der/meahttun")]
    Der_SLASH_meahttun,
    /// kpv: **@CODE@**  мед- Superlative
    #[strum(serialize = "Der/med")]
    Der_SLASH_med,
    /// vot: (no comment)
    #[strum(serialize = "Der/min")]
    Der_SLASH_min,
    /// vro: **@CODE@** NomAct
    #[strum(serialize = "Der/minE")]
    Der_SLASH_minE,
    /// olo: (no comment)
    #[strum(serialize = "Der/mine")]
    Der_SLASH_mine,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** =
    /// fin: (no comment)
    #[strum(serialize = "Der/minen")]
    Der_SLASH_minen,
    /// kpv: **@CODE@** моз +MOZ  diminishing, kind of, sort of
    #[strum(serialize = "Der/moz")]
    Der_SLASH_moz,
    /// smj: (no comment)
    #[strum(serialize = "Der/mus")]
    Der_SLASH_mus,
    /// sms: **@CODE@** deverbal derivation V»N architype mõš and mmuš was Der/MOsh
    #[strum(serialize = "Der/musj")]
    Der_SLASH_musj,
    /// sme: (no comment)
    #[strum(serialize = "Der/muš")]
    Der_SLASH_muš,
    /// kpv: **@CODE@**  = +MON
    #[strum(serialize = "Der/mwn")]
    Der_SLASH_mwn,
    /// kpv: **@CODE@**  = Ӧнія коми кыв. 2000: 399-403 мӧнъя
    #[strum(serialize = "Der/mwnja")]
    Der_SLASH_mwnja,
    /// kpv: **@CODE@**  +MYS
    #[strum(serialize = "Der/mysj")]
    Der_SLASH_mysj,
    /// kpv: **@CODE@**  +MYST
    #[strum(serialize = "Der/mysjt")]
    Der_SLASH_mysjt,
    /// smj: (no comment)
    #[strum(serialize = "Der/n")]
    Der_SLASH_n,
    /// sms: (no comment)
    #[strum(serialize = "Der/nalla")]
    Der_SLASH_nalla,
    /// sms: **@CODE@** N»N resident of place
    #[strum(serialize = "Der/neqkh")]
    Der_SLASH_neqkh,
    /// fin: (no comment)
    #[strum(serialize = "Der/nti")]
    Der_SLASH_nti,
    /// sms: **@CODE@**  V»V -õõllâd
    #[strum(serialize = "Der/oollyd")]
    Der_SLASH_oollyd,
    /// sms: **@CODE@** reflexive reciprocal V»V -õõttâd
    #[strum(serialize = "Der/oottyd")]
    Der_SLASH_oottyd,
    /// sms: **@CODE@**  V»V kulsted: kulstõõvvâd
    #[strum(serialize = "Der/oovvyd")]
    Der_SLASH_oovvyd,
    /// kpv: **@CODE@** овт
    #[strum(serialize = "Der/ovt")]
    Der_SLASH_ovt,
    /// kpv: **@CODE@** пӧвстса
    #[strum(serialize = "Der/pwvstsa")]
    Der_SLASH_pwvstsa,
    /// smj: (no comment)
    #[strum(serialize = "Der/r")]
    Der_SLASH_r,
    /// smj: (no comment)
    #[strum(serialize = "Der/ravak")]
    Der_SLASH_ravak,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** = deriving numerals
    /// fin: (no comment)
    #[strum(serialize = "Der/s")]
    Der_SLASH_s,
    /// kpv: **@CODE@** са
    #[strum(serialize = "Der/sa")]
    Der_SLASH_sa,
    /// kpv: **@CODE@** сайса
    #[strum(serialize = "Der/sajsa")]
    Der_SLASH_sajsa,
    /// smj: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Der/sasj")]
    Der_SLASH_sasj,
    /// sms: (no comment)
    #[strum(serialize = "Der/sazh")]
    Der_SLASH_sazh,
    /// smj: (no comment)
    #[strum(serialize = "Der/segak")]
    Der_SLASH_segak,
    /// kpv: **@CODE@** сь This only occurs following a vowel in an yny-stem 2017-09-19+Der/sj
    #[strum(serialize = "Der/sj")]
    Der_SLASH_sj,
    /// kpv: **@CODE@** ся
    #[strum(serialize = "Der/sja")]
    Der_SLASH_sja,
    /// kpv: **@CODE@** -сюрӧ -кӧ !Declaring Indefinite Pronoun derivations
    #[strum(serialize = "Der/sjurw")]
    Der_SLASH_sjurw,
    /// kpv: **@CODE@** сьы 2017-09-19+Der/ch ! This appears to be a variant of +Der/sjy; it follows plosives
    #[strum(serialize = "Der/sjy")]
    Der_SLASH_sjy,
    /// kpv: **@CODE@**  = +SOR
    #[strum(serialize = "Der/sor")]
    Der_SLASH_sor,
    /// sma: **@CODE@** |            |            | VV   | Diminutive, Subitive
    /// smj: (no comment)
    /// sms: **@CODE@**  V»V was diminutive subitive Der/sted, Der/âʹstted
    /// sme: (no comment)
    #[strum(serialize = "Der/st")]
    Der_SLASH_st,
    /// smj: (no comment)
    #[strum(serialize = "Der/stahtte")]
    Der_SLASH_stahtte,
    /// smj: (no comment)
    #[strum(serialize = "Der/stalla")]
    Der_SLASH_stalla,
    /// smj: (no comment)
    #[strum(serialize = "Der/stasste")]
    Der_SLASH_stasste,
    /// fin: (no comment)
    #[strum(serialize = "Der/sti")]
    Der_SLASH_sti,
    /// sms: **@CODE@**  V»V -stõõllâd
    #[strum(serialize = "Der/stoollyd")]
    Der_SLASH_stoollyd,
    /// sms: **@CODE@**  V»V -stõõvvâd
    #[strum(serialize = "Der/stoovvyd")]
    Der_SLASH_stoovvyd,
    /// sme: (no comment)
    #[strum(serialize = "Der/stuvva")]
    Der_SLASH_stuvva,
    /// sme: (no comment)
    #[strum(serialize = "Der/supmi")]
    Der_SLASH_supmi,
    /// fit: **@CODE@** =
    #[strum(serialize = "Der/suus")]
    Der_SLASH_suus,
    /// sms: **@CODE@** carative derivation N»A for subsequent derivation
    /// sme: (no comment)
    #[strum(serialize = "Der/t")]
    Der_SLASH_t,
    /// fin: (no comment)
    #[strum(serialize = "Der/tar")]
    Der_SLASH_tar,
    /// fin: (no comment)
    #[strum(serialize = "Der/tattaa")]
    Der_SLASH_tattaa,
    /// fin: (no comment)
    #[strum(serialize = "Der/tatuttaa")]
    Der_SLASH_tatuttaa,
    /// sms: **@CODE@** carative derivation N»A, V»A -teʹm
    #[strum(serialize = "Der/teqm")]
    Der_SLASH_teqm,
    /// smj: (no comment)
    #[strum(serialize = "Der/tj")]
    Der_SLASH_tj,
    /// fin: (no comment)
    #[strum(serialize = "Der/ton")]
    Der_SLASH_ton,
    /// sms: **@CODE@** privative + translative N»V --tõõvvâd
    #[strum(serialize = "Der/toovvyd")]
    Der_SLASH_toovvyd,
    /// kpv: **@CODE@** тор
    #[strum(serialize = "Der/tor")]
    Der_SLASH_tor,
    /// fin: (no comment)
    #[strum(serialize = "Der/tse")]
    Der_SLASH_tse,
    /// fin: (no comment)
    #[strum(serialize = "Der/ttaa")]
    Der_SLASH_ttaa,
    /// fin: (no comment)
    #[strum(serialize = "Der/ttain")]
    Der_SLASH_ttain,
    /// kpv: **@CODE@** тӧг
    #[strum(serialize = "Der/twg")]
    Der_SLASH_twg,
    /// kpv: **@CODE@** тӧм used with nouns and followed by +AbeMod
    #[strum(serialize = "Der/twm")]
    Der_SLASH_twm,
    /// kpv: **@CODE@**  = +TYR
    #[strum(serialize = "Der/tyr")]
    Der_SLASH_tyr,
    /// kpv: **@CODE@**  = +TYRJA
    #[strum(serialize = "Der/tyrja")]
    Der_SLASH_tyrja,
    /// kpv: **@CODE@**  = +TYRJI
    #[strum(serialize = "Der/tyrji")]
    Der_SLASH_tyrji,
    /// fin: (no comment)
    #[strum(serialize = "Der/u")]
    Der_SLASH_u,
    /// smj: (no comment)
    #[strum(serialize = "Der/u/a/åd")]
    Der_SLASH_u_SLASH_a_SLASH_åd,
    /// sme: (no comment)
    #[strum(serialize = "Der/upmi")]
    Der_SLASH_upmi,
    /// fin: (no comment)
    #[strum(serialize = "Der/uus")]
    Der_SLASH_uus,
    /// kpv: **@CODE@** увса
    #[strum(serialize = "Der/uvsa")]
    Der_SLASH_uvsa,
    /// sme: (no comment)
    #[strum(serialize = "Der/veara")]
    Der_SLASH_veara,
    /// kpv: **@CODE@** весьтса
    #[strum(serialize = "Der/vesjtsa")]
    Der_SLASH_vesjtsa,
    /// sme: (no comment)
    #[strum(serialize = "Der/viđi")]
    Der_SLASH_viđi,
    /// sme: (no comment)
    #[strum(serialize = "Der/viđá")]
    Der_SLASH_viđá,
    /// kpv: **@CODE@** водзса
    #[strum(serialize = "Der/vodzsa")]
    Der_SLASH_vodzsa,
    /// fin: (no comment)
    #[strum(serialize = "Der/vs")]
    Der_SLASH_vs,
    /// sma: **@CODE@** |            | AN   | Noun
    /// smj: (no comment)
    /// sms: **@CODE@** A»N -vuõtt
    /// sme: (no comment)
    #[strum(serialize = "Der/vuota")]
    Der_SLASH_vuota,
    /// kpv: **@CODE@** выв
    #[strum(serialize = "Der/vyv")]
    Der_SLASH_vyv,
    /// kpv: **@CODE@** вывса
    #[strum(serialize = "Der/vyvsa")]
    Der_SLASH_vyvsa,
    /// kpv: **@CODE@**  ӧм verb-to-noun   !Declaring Indefinite Pronoun derivations the combinatory +Event preceding the NP-final noun
    #[strum(serialize = "Der/wm")]
    Der_SLASH_wm,
    /// kpv: **@CODE@**  = +OMON !Ӧнія коми кыв. 2000: 425
    #[strum(serialize = "Der/wmwn")]
    Der_SLASH_wmwn,
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// zxx: (no comment)
    /// olo: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Der/xxx")]
    Der_SLASH_xxx,
    /// kpv: **@CODE@** ышт
    #[strum(serialize = "Der/ysht")]
    Der_SLASH_ysht,
    /// kpv: **@CODE@**
    #[strum(serialize = "Der/ysj")]
    Der_SLASH_ysj,
    /// kpv: **@CODE@** ывлы
    #[strum(serialize = "Der/yvly")]
    Der_SLASH_yvly,
    /// sme: (no comment)
    #[strum(serialize = "Der/ár")]
    Der_SLASH_ár,
    /// mrj: **@CODE@** =        NegPrc
    #[strum(serialize = "Der/дЫмЫ")]
    Der_SLASH_дЫмЫ,
    /// mrj: **@CODE@** =          Pass Prc
    #[strum(serialize = "Der/мЫ")]
    Der_SLASH_мЫ,
    /// mrj: **@CODE@** =          Act Prc
    #[strum(serialize = "Der/шЫ")]
    Der_SLASH_шЫ,
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** |            |            |      | Position tag, required
    /// nob: **@CODE@** = derivation position
    /// smj: **@CODE@**:    - positional tags, preceeds the actual der tag
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Der1")]
    Der1,
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** |            |      | Position tag, required
    /// sms: **@CODE@**
    #[strum(serialize = "Der2")]
    Der2,
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** |      | Position tag, required
    /// sms: **@CODE@**
    #[strum(serialize = "Der3")]
    Der3,
    /// fkv: **@CODE@** =
    /// sms: **@CODE@**
    #[strum(serialize = "Der4")]
    Der4,
    /// fkv: **@CODE@** =
    /// sms: **@CODE@**
    #[strum(serialize = "Der5")]
    Der5,
    /// mhr: **@CODE@** = desiderative
    /// mrj: **@CODE@** =
    #[strum(serialize = "Des")]
    Des,
    /// mhr: **@CODE@** = descriptive ideophones
    #[strum(serialize = "Descr")]
    Descr,
    /// kpv: **@CODE@**:  determiner  XX   XX
    /// vot: (no comment)
    /// rus: (no comment)
    /// sms: **@CODE@** = Determiner
    /// vep: **@CODE@**  = determiner
    /// olo: (no comment)
    /// vro: **@CODE@** Determiners
    /// vro: (no comment)
    #[strum(serialize = "Det")]
    Det,
    /// fin: (no comment)
    #[strum(serialize = "Dial")]
    Dial,
    /// sme: **@CODE@** forms not in use in GG (Guovdageaidnu)
    #[strum(serialize = "Dial/-GG")]
    Dial_SLASH__MINUS_GG,
    /// sme: **@CODE@** forms not in use in GS (Gárasavvon) NOT IN USE
    #[strum(serialize = "Dial/-GS")]
    Dial_SLASH__MINUS_GS,
    /// fkv: **@CODE@** = Not Jokivarret
    #[strum(serialize = "Dial/-Jok")]
    Dial_SLASH__MINUS_Jok,
    /// sme: **@CODE@** forms not in use in KJ (Kárásjohka)
    #[strum(serialize = "Dial/-KJ")]
    Dial_SLASH__MINUS_KJ,
    /// sma: **@CODE@** | Not in the North
    #[strum(serialize = "Dial/-N")]
    Dial_SLASH__MINUS_N,
    /// sma: **@CODE@** | Words not in Norway
    #[strum(serialize = "Dial/-NOR")]
    Dial_SLASH__MINUS_NOR,
    /// fkv: **@CODE@** = Not Porsanger
    #[strum(serialize = "Dial/-Por")]
    Dial_SLASH__MINUS_Por,
    /// sma: **@CODE@** | Not in the South
    #[strum(serialize = "Dial/-S")]
    Dial_SLASH__MINUS_S,
    /// sma: **@CODE@** | Words not in Sweden
    #[strum(serialize = "Dial/-SW")]
    Dial_SLASH__MINUS_SW,
    /// fkv: **@CODE@** = Not Varanger
    #[strum(serialize = "Dial/-Var")]
    Dial_SLASH__MINUS_Var,
    /// vro: **@CODE@** dialect u stem where o expected
    #[strum(serialize = "Dial/-u-not-o")]
    Dial_SLASH__MINUS_u_MINUS_not_MINUS_o,
    /// yrk: **@CODE@** (Eastern dialects),
    #[strum(serialize = "Dial/E")]
    Dial_SLASH_E,
    /// fin: (no comment)
    #[strum(serialize = "Dial/East")]
    Dial_SLASH_East,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Eteläpohjalaiset")]
    Dial_SLASH_Eteläpohjalaiset,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Finland")]
    Dial_SLASH_Finland,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Häme")]
    Dial_SLASH_Häme,
    /// fkv: **@CODE@** Jokivarret, short for +Dial/-Por+Dial/-V
    #[strum(serialize = "Dial/Jok")]
    Dial_SLASH_Jok,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Keskipohjalaiset")]
    Dial_SLASH_Keskipohjalaiset,
    /// sma: **@CODE@** | Long forms
    #[strum(serialize = "Dial/L")]
    Dial_SLASH_L,
    /// sma: **@CODE@** | Only in the North
    /// smj: **@CODE@** Used in the northern areas.  Some might say that
    /// sms: **@CODE@** Nuortjärvi (KKS)
    #[strum(serialize = "Dial/N")]
    Dial_SLASH_N,
    /// sma: **@CODE@** | Words only in Norway
    #[strum(serialize = "Dial/NOR")]
    Dial_SLASH_NOR,
    /// fin: (no comment)
    #[strum(serialize = "Dial/North")]
    Dial_SLASH_North,
    /// sms: **@CODE@** Paatsjoki (KKS)
    #[strum(serialize = "Dial/P")]
    Dial_SLASH_P,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Peräpohjalaiset")]
    Dial_SLASH_Peräpohjalaiset,
    /// sms: **@CODE@** Petsamo
    #[strum(serialize = "Dial/Pets")]
    Dial_SLASH_Pets,
    /// fkv: **@CODE@** Porsanger, short for +Dial/-Jok+Dial/-Var
    #[strum(serialize = "Dial/Por")]
    Dial_SLASH_Por,
    /// sma: **@CODE@** | Only in the South
    /// smj: **@CODE@** Used in the southern areas
    /// sms: **@CODE@** Suonikylä (KKS)
    #[strum(serialize = "Dial/S")]
    Dial_SLASH_S,
    /// sma: **@CODE@** | Short forms
    /// smj: **@CODE@** Short forms
    #[strum(serialize = "Dial/SH")]
    Dial_SLASH_SH,
    /// sma: **@CODE@** | Words only in Sweden
    #[strum(serialize = "Dial/SW")]
    Dial_SLASH_SW,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Savo")]
    Dial_SLASH_Savo,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Southeast")]
    Dial_SLASH_Southeast,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Southwest")]
    Dial_SLASH_Southwest,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Standard")]
    Dial_SLASH_Standard,
    /// yrk: **@CODE@** (Taimyr dialect  ),
    #[strum(serialize = "Dial/T")]
    Dial_SLASH_T,
    /// fkv: **@CODE@** Varanger, short for +Dial/-Jok+Dial/-Por
    #[strum(serialize = "Dial/Var")]
    Dial_SLASH_Var,
    /// yrk: **@CODE@** (Western dialects),
    #[strum(serialize = "Dial/W")]
    Dial_SLASH_W,
    /// fin: (no comment)
    #[strum(serialize = "Dial/West")]
    Dial_SLASH_West,
    /// kpv: **@CODE@** maybe Err/Orth-l-not-v
    #[strum(serialize = "Dial/l")]
    Dial_SLASH_l,
    /// fin: (no comment)
    #[strum(serialize = "Dial/standard")]
    Dial_SLASH_standard,
    /// kpv: **@CODE@** maybe Err/Orth-v-not-l
    #[strum(serialize = "Dial/v")]
    Dial_SLASH_v,
    /// kpv: **@CODE@** diminutive for verbs -ышт- (there might be a better term)
    #[strum(serialize = "Dim")]
    Dim,
    /// kpv: **@CODE@** diminutive for nouns -тор-
    /// sms: **@CODE@** diminutive without derivation 2025-04-04 jaska
    #[strum(serialize = "Dimin")]
    Dimin,
    /// ceb: **@CODE@** -
    /// ceb: **@CODE@** - Direct
    #[strum(serialize = "Dir")]
    Dir,
    /// ceb: **@CODE@** - Distributive
    #[strum(serialize = "Dis")]
    Dis,
    /// kpv: **@CODE@**:  Distributive
    /// sms: **@CODE@** Distributive
    /// fin: (no comment)
    #[strum(serialize = "Distr")]
    Distr,
    /// kpv: **@CODE@** for pronoun.
    /// vot: (no comment)
    /// sma: **@CODE@** = Dual
    /// smj: **@CODE@** = Dual number
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Dual = kuõiʹtilååkk
    /// sme: **@CODE@** - Dual
    #[strum(serialize = "Du")]
    Du,
    /// vot: (no comment)
    /// sma: **@CODE@** | Dual    , 1.person
    /// smj: (no comment)
    /// sms: **@CODE@** first person dual
    /// lit: (no comment)
    /// sme: **@CODE@** Dual First Person
    /// ceb: (no comment)
    #[strum(serialize = "Du1")]
    Du1,
    /// vot: (no comment)
    /// sma: **@CODE@** | Dual    , 2.person
    /// smj: (no comment)
    /// sms: **@CODE@** second person dual
    /// sme: **@CODE@** Dual Second Person
    /// ceb: (no comment)
    #[strum(serialize = "Du2")]
    Du2,
    /// vot: (no comment)
    /// sma: **@CODE@** | Dual    , 3.person
    /// smj: (no comment)
    /// sms: **@CODE@** third person dual
    /// sme: **@CODE@** Dual Third Person
    /// ceb: (no comment)
    #[strum(serialize = "Du3")]
    Du3,
    /// kpv: **@CODE@**
    #[strum(serialize = "Duration")]
    Duration,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = Dynamically created acronym
    /// sma: **@CODE@** = Code dynamic acronyms
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Code dynamic acronyms
    /// sms: **@CODE@** = Dynamically generated (acronyms)
    /// olo: (no comment)
    /// sme: **@CODE@**  Dynamically generated (acronyms) +ACR+Dyn
    #[strum(serialize = "Dyn")]
    Dyn,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/BXR")]
    EOLang_SLASH_BXR,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/CHM")]
    EOLang_SLASH_CHM,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/KOI")]
    EOLang_SLASH_KOI,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/KOM")]
    EOLang_SLASH_KOM,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/KPV")]
    EOLang_SLASH_KPV,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/MDF")]
    EOLang_SLASH_MDF,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/MHR")]
    EOLang_SLASH_MHR,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/MRJ")]
    EOLang_SLASH_MRJ,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/MYV")]
    EOLang_SLASH_MYV,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/RUS")]
    EOLang_SLASH_RUS,
    /// kpv: **@CODE@**
    #[strum(serialize = "EOLang/YRK")]
    EOLang_SLASH_YRK,
    /// mhr: **@CODE@** = change to other transitivity
    #[strum(serialize = "EX/IV")]
    EX_SLASH_IV,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Edu")]
    Edu,
    /// kpv: **@CODE@** egressive -сянь ылыстчан
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Egr")]
    Egr,
    /// fit: **@CODE@** = Elative
    /// kpv: **@CODE@** elative -ысь петан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Elative
    /// liv: **@CODE@** = elative
    /// sma: **@CODE@** = Elative
    /// smj: **@CODE@** = Elative case
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = elative
    /// vep: (no comment)
    /// zxx: (no comment)
    /// olo: **@CODE@** elative
    /// fin: (no comment)
    /// vro: **@CODE@** elative
    #[strum(serialize = "Ela")]
    Ela,
    /// rus: (no comment)
    #[strum(serialize = "Elid")]
    Elid,
    /// rus: (no comment)
    #[strum(serialize = "Epenth")]
    Epenth,
    /// mrj: **@CODE@** =
    #[strum(serialize = "Epist")]
    Epist,
    /// yrk: **@CODE@**
    #[strum(serialize = "Equ")]
    Equ,
    /// sma: **@CODE@** | Substandard, unormert Attr-form av et ord
    #[strum(serialize = "Err/Attr")]
    Err_SLASH_Attr,
    /// smj: **@CODE@** = No normative compound, often wrong first part of cmp (in other case than nom or gen, or adv og pronoun)
    #[strum(serialize = "Err/Cmp")]
    Err_SLASH_Cmp,
    /// sms: **@CODE@** substandard for compounding, not in normative fst (wrong form or POS in first part)
    /// sme: **@CODE@** substandard for compounding, not in normative fst (wrong form or POS in first part)
    #[strum(serialize = "Err/CmpSub")]
    Err_SLASH_CmpSub,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused")]
    Err_SLASH_Confused,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-ASgNom")]
    Err_SLASH_Confused_MINUS_ASgNom,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-DerPassPrsSg3")]
    Err_SLASH_Confused_MINUS_DerPassPrsSg3,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-DerPassPrtSg3")]
    Err_SLASH_Confused_MINUS_DerPassPrtSg3,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-Ess")]
    Err_SLASH_Confused_MINUS_Ess,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-ImprtDu1")]
    Err_SLASH_Confused_MINUS_ImprtDu1,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-ImprtPl2")]
    Err_SLASH_Confused_MINUS_ImprtPl2,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-ImprtSg2")]
    Err_SLASH_Confused_MINUS_ImprtSg2,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-NSgPxSg1")]
    Err_SLASH_Confused_MINUS_NSgPxSg1,
    /// sme: **@CODE@** grammarcheking rela word error confusion pairs
    #[strum(serialize = "Err/Confused-NomAgIll")]
    Err_SLASH_Confused_MINUS_NomAgIll,
    /// sma: **@CODE@** | Errors in derivations
    /// smj: **@CODE@** = Lemmas that break with regular derivation rules, both morphologically and semantically
    #[strum(serialize = "Err/Der")]
    Err_SLASH_Der,
    /// sms: **@CODE@** substandard for derivation, not in normative fst, no normative lemma
    /// sme: **@CODE@** substandard for derivation, not in normative fst, no normative lemma
    #[strum(serialize = "Err/DerSub")]
    Err_SLASH_DerSub,
    /// kpv: **@CODE@** e.g. тэг instead of тӧг
    /// vro: **@CODE@** This is the initial Dialect distinction
    #[strum(serialize = "Err/Dial")]
    Err_SLASH_Dial,
    /// sms: **@CODE@**
    #[strum(serialize = "Err/GenreLeudd")]
    Err_SLASH_GenreLeudd,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | Substandard, unormert
    /// nob: **@CODE@**
    /// smj: **@CODE@** = when there is a hyphen where none should have been
    /// sms: **@CODE@** substandard for compounding, not in normative fst (should have no hyphen)
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** when there is a hyphen where none should have been
    /// fin: (no comment)
    #[strum(serialize = "Err/Hyph")]
    Err_SLASH_Hyph,
    /// smj: **@CODE@** = Lemmas that break with normative inflection rules, often used with morphological changes
    #[strum(serialize = "Err/Infl")]
    Err_SLASH_Infl,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_FV")]
    Err_SLASH_L2_FV,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_H2S")]
    Err_SLASH_L2_H2S,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_Ikn")]
    Err_SLASH_L2_Ikn,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_NoFV")]
    Err_SLASH_L2_NoFV,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_NoGem")]
    Err_SLASH_L2_NoGem,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_NoSS")]
    Err_SLASH_L2_NoSS,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_Pal")]
    Err_SLASH_L2_Pal,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_SRc")]
    Err_SLASH_L2_SRc,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_SRo")]
    Err_SLASH_L2_SRo,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_SRy")]
    Err_SLASH_L2_SRy,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_a2o")]
    Err_SLASH_L2_a2o,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_e2je")]
    Err_SLASH_L2_e2je,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_i2j")]
    Err_SLASH_L2_i2j,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_i2y")]
    Err_SLASH_L2_i2y,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_ii")]
    Err_SLASH_L2_ii,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_j2i")]
    Err_SLASH_L2_j2i,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_je2e")]
    Err_SLASH_L2_je2e,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_o2a")]
    Err_SLASH_L2_o2a,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_prijti")]
    Err_SLASH_L2_prijti,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_revIkn")]
    Err_SLASH_L2_revIkn,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_sh2shch")]
    Err_SLASH_L2_sh2shch,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_shch2sh")]
    Err_SLASH_L2_shch2sh,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_ski")]
    Err_SLASH_L2_ski,
    /// rus: (no comment)
    #[strum(serialize = "Err/L2_y2i")]
    Err_SLASH_L2_y2i,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** substandard, not in normative fst, no  normative lemma помсьыны
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | lemma med dens ordformer er utenfor normen. <br/>No normative lemma, it's grammatically correct.
    /// nob: **@CODE@**
    /// smj: **@CODE@** = No normative lemma
    /// sms: **@CODE@** substandard, not in normative fst, no normative lemma
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** substandard, not in normative fst, no normative lemma
    /// fin: (no comment)
    #[strum(serialize = "Err/Lex")]
    Err_SLASH_Lex,
    /// sma: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** substandard for compounding, not in normative fst (needs hyph)
    /// sme: **@CODE@** when there is no hyphen where it should have been
    #[strum(serialize = "Err/MissingHyph")]
    Err_SLASH_MissingHyph,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | in use in smi lexc
    /// nob: **@CODE@**
    /// smj: **@CODE@** = indicates that there is a missing space, causing an orthographic error. Used for "goadedagi", when it should be "goade dagi"
    /// sms: **@CODE@** | in use ins smi lexc
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** indicates that there is a missing space, causing an orthographic error
    /// fin: (no comment)
    #[strum(serialize = "Err/MissingSpace")]
    Err_SLASH_MissingSpace,
    /// fit: (no comment)
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@**  misspelling or nor normative form. It will be included only in desc, not in norm.
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = orthographical error (analysed, not accepted in speller)
    /// sma: **@CODE@** | Substandard, unormert form av et ord
    /// nob: **@CODE@**    For speller use
    /// smj: **@CODE@** = Substandard. An ungrammatical, non-normative form of normative lemma.
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**  substandard, not in normative fst
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@** substandard, not in normative fst
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@** substandard, not in normative fst
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth")]
    Err_SLASH_Orth,
    /// sms: **@CODE@** substandard a, not in normative fst â
    #[strum(serialize = "Err/Orth-a-y")]
    Err_SLASH_Orth_MINUS_a_MINUS_y,
    /// smj: (no comment)
    /// sme: **@CODE@** substandard, not in normative fst
    #[strum(serialize = "Err/Orth-a-á")]
    Err_SLASH_Orth_MINUS_a_MINUS_á,
    /// vro: **@CODE@** erroneous back harmony, see flags
    #[strum(serialize = "Err/Orth-back")]
    Err_SLASH_Orth_MINUS_back,
    /// kpv: **@CODE@** colloquial form for patronymic
    #[strum(serialize = "Err/Orth-colloq")]
    Err_SLASH_Orth_MINUS_colloq,
    /// vro: **@CODE@** hirnahtadaq should be hirnahtaq
    #[strum(serialize = "Err/Orth-dAq-should-be-q-inf")]
    Err_SLASH_Orth_MINUS_dAq_MINUS_should_MINUS_be_MINUS_q_MINUS_inf,
    /// kpv: **@CODE@** this covers the dialect forms, i.e., there will be an adjustment to Dial/...
    #[strum(serialize = "Err/Orth-e-not-oe")]
    Err_SLASH_Orth_MINUS_e_MINUS_not_MINUS_oe,
    /// vro: **@CODE@** should be i stem, but is e stem
    #[strum(serialize = "Err/Orth-e-stem")]
    Err_SLASH_Orth_MINUS_e_MINUS_stem,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-est")]
    Err_SLASH_Orth_MINUS_est,
    /// sms: **@CODE@** substandard extra ʹ as in aanarneʹǩ (->) aanarneǩ
    #[strum(serialize = "Err/Orth-extra-q")]
    Err_SLASH_Orth_MINUS_extra_MINUS_q,
    /// vro: **@CODE@** erroneous front harmony, see flags
    #[strum(serialize = "Err/Orth-front")]
    Err_SLASH_Orth_MINUS_front,
    /// sms: **@CODE@** substandard k, not in normative fst ǩ(, should be  kuõccjiǩ and not kuõccjik)
    #[strum(serialize = "Err/Orth-k-kh")]
    Err_SLASH_Orth_MINUS_k_MINUS_kh,
    /// kpv: **@CODE@** followed by vowel, yet v
    #[strum(serialize = "Err/Orth-l-in-v")]
    Err_SLASH_Orth_MINUS_l_MINUS_in_MINUS_v,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-l-retenstion")]
    Err_SLASH_Orth_MINUS_l_MINUS_retenstion,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-l-to-v-in-new")]
    Err_SLASH_Orth_MINUS_l_MINUS_to_MINUS_v_MINUS_in_MINUS_new,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-l-to-vowel-lengthening")]
    Err_SLASH_Orth_MINUS_l_MINUS_to_MINUS_vowel_MINUS_lengthening,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-lowered-vow")]
    Err_SLASH_Orth_MINUS_lowered_MINUS_vow,
    /// sms: **@CODE@**  substandard, missing apostrophe
    #[strum(serialize = "Err/Orth-missing-apos")]
    Err_SLASH_Orth_MINUS_missing_MINUS_apos,
    /// vep: **@CODE@**  = palatalization mark missing
    /// vro: **@CODE@** palatalization is missing
    #[strum(serialize = "Err/Orth-no-pal")]
    Err_SLASH_Orth_MINUS_no_MINUS_pal,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-no-paragogic-j")]
    Err_SLASH_Orth_MINUS_no_MINUS_paragogic_MINUS_j,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-no-paragogic-k")]
    Err_SLASH_Orth_MINUS_no_MINUS_paragogic_MINUS_k,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-no-paragogic-m")]
    Err_SLASH_Orth_MINUS_no_MINUS_paragogic_MINUS_m,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-no-paragogic-t")]
    Err_SLASH_Orth_MINUS_no_MINUS_paragogic_MINUS_t,
    /// sms: **@CODE@** substandard missing ʹ as in mâiʹd-ne (->) mâid-ne
    /// vro: **@CODE@** Q is missing
    #[strum(serialize = "Err/Orth-no-q")]
    Err_SLASH_Orth_MINUS_no_MINUS_q,
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// sme: **@CODE@** substandard, not in normative fst
    #[strum(serialize = "Err/Orth-nom-acc")]
    Err_SLASH_Orth_MINUS_nom_MINUS_acc,
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// sme: **@CODE@** substandard, not in normative fst
    #[strum(serialize = "Err/Orth-nom-gen")]
    Err_SLASH_Orth_MINUS_nom_MINUS_gen,
    /// sms: **@CODE@**  substandard, old orth look-alikes are separated by a hyphen
    #[strum(serialize = "Err/Orth-not-apos-but-hyph")]
    Err_SLASH_Orth_MINUS_not_MINUS_apos_MINUS_but_MINUS_hyph,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-o-not-oe")]
    Err_SLASH_Orth_MINUS_o_MINUS_not_MINUS_oe,
    /// vro: **@CODE@** should be u stem, but is o stem
    #[strum(serialize = "Err/Orth-o-stem")]
    Err_SLASH_Orth_MINUS_o_MINUS_stem,
    /// sms: **@CODE@** substandard o, not in normative fst uʹ (e.g. illative kuʹmppe and not komppa)
    #[strum(serialize = "Err/Orth-o-uq")]
    Err_SLASH_Orth_MINUS_o_MINUS_uq,
    /// sms: **@CODE@** substandard palatalized diphthong ieʹ should be eä or eâ; ueʹ should be uä, uâ <no soft sign>
    #[strum(serialize = "Err/Orth-pal-vow")]
    Err_SLASH_Orth_MINUS_pal_MINUS_vow,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-raised-vow")]
    Err_SLASH_Orth_MINUS_raised_MINUS_vow,
    /// sms: **@CODE@** substandard raajeeʹl (->) raajjeeʹl
    #[strum(serialize = "Err/Orth-should-be-grade-minus1")]
    Err_SLASH_Orth_MINUS_should_MINUS_be_MINUS_grade_MINUS_minus1,
    /// vro: **@CODE@** should be o stem, but is u stem
    #[strum(serialize = "Err/Orth-u-stem")]
    Err_SLASH_Orth_MINUS_u_MINUS_stem,
    /// sms: (no comment)
    #[strum(serialize = "Err/Orth-vow-not-raised")]
    Err_SLASH_Orth_MINUS_vow_MINUS_not_MINUS_raised,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-weak-grade")]
    Err_SLASH_Orth_MINUS_weak_MINUS_grade,
    /// liv: **@CODE@** is õ should be i lītõm should be lītim 2025-09-09
    #[strum(serialize = "Err/Orth-õ-i")]
    Err_SLASH_Orth_MINUS_õ_MINUS_i,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | Substandard, unormert
    /// nob: **@CODE@**
    /// smj: **@CODE@** = used for compounds written apart - only retained in the HFST Grammar Checker disambiguation analyser
    /// sms: **@CODE@** used for compounds written apart - only retained in the HFST Grammar Checker disambiguation analyser
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** used for compounds written apart - only retained in the HFST Grammar Checker disambiguation analyser
    /// fin: (no comment)
    #[strum(serialize = "Err/SpaceCmp")]
    Err_SLASH_SpaceCmp,
    /// sma: **@CODE@** | Used to tag spellrelaxed typos (tag is inserted via flag diacritics)
    /// sms: **@CODE@** used to tag spellrelaxed typos (tag is inserted via flag diacritics)
    /// sme: **@CODE@** used to tag spellrelaxed typos (tag is inserted via flag diacritics)
    #[strum(serialize = "Err/Spellrelax")]
    Err_SLASH_Spellrelax,
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    #[strum(serialize = "Err/Sub")]
    Err_SLASH_Sub,
    /// fit: **@CODE@** = Essive
    /// vot: (no comment)
    /// fkv: **@CODE@** = Essive
    /// liv: **@CODE@** = essive
    /// sma: **@CODE@** = Essive
    /// smj: **@CODE@** = Essive case
    /// sms: **@CODE@** Essive
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**  = essive
    /// zxx: (no comment)
    /// olo: **@CODE@** essive
    /// sme: **@CODE@** - Essive
    /// fin: (no comment)
    /// vro: **@CODE@** essive
    #[strum(serialize = "Ess")]
    Ess,
    /// vep: **@CODE@**  =
    #[strum(serialize = "EssInst")]
    EssInst,
    /// yrk: **@CODE@**
    #[strum(serialize = "Evas")]
    Evas,
    /// fin: (no comment)
    #[strum(serialize = "Eventv")]
    Eventv,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mhr: **@CODE@** = for derivation from A to anoter POS
    /// sms: **@CODE@** = Adjective stem before derivation
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    #[strum(serialize = "Ex/A")]
    Ex_SLASH_A,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mhr: **@CODE@** = for derivation from N to anoter POS
    /// sms: **@CODE@** = Noun stem before derivation
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    #[strum(serialize = "Ex/N")]
    Ex_SLASH_N,
    /// sms: **@CODE@** = Number stem before derivation
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    #[strum(serialize = "Ex/Num")]
    Ex_SLASH_Num,
    /// mhr: **@CODE@** = change to other transitivity
    #[strum(serialize = "Ex/TV")]
    Ex_SLASH_TV,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mhr: **@CODE@** = for derivation from V  to anoter POS
    /// nob: **@CODE@** for derivation
    /// sms: **@CODE@** = Verb stem before derivation
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    #[strum(serialize = "Ex/V")]
    Ex_SLASH_V,
    /// liv: **@CODE@** = exessive
    /// vep: **@CODE@**  = essive
    #[strum(serialize = "Exe")]
    Exe,
    /// mhr: **@CODE@** = form уло
    #[strum(serialize = "Ext")]
    Ext,
    /// rus: (no comment)
    #[strum(serialize = "Fac")]
    Fac,
    /// kpv: **@CODE@**  мог, мон, моз 'so that I won't'
    #[strum(serialize = "Final")]
    Final,
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle?
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// vro: **@CODE@**
    #[strum(serialize = "Foc")]
    Foc,
    /// sms: **@CODE@** more specifically, exactly, contrast. used with Pron and Adv
    #[strum(serialize = "Foc/AA")]
    Foc_SLASH_AA,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/Neg-ge")]
    Foc_SLASH_Neg_MINUS_ge,
    /// smj: (no comment)
    #[strum(serialize = "Foc/Neg-k")]
    Foc_SLASH_Neg_MINUS_k,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/Pos-ge")]
    Foc_SLASH_Pos_MINUS_ge,
    /// smj: (no comment)
    #[strum(serialize = "Foc/Pos-k")]
    Foc_SLASH_Pos_MINUS_k,
    /// mhr: **@CODE@** =
    #[strum(serialize = "Foc/Poss")]
    Foc_SLASH_Poss,
    /// mhr: **@CODE@** = -ak focus particle
    #[strum(serialize = "Foc/ak")]
    Foc_SLASH_ak,
    /// sms: (no comment)
    #[strum(serialize = "Foc/ala")]
    Foc_SLASH_ala,
    /// sms: **@CODE@** -âs
    #[strum(serialize = "Foc/as")]
    Foc_SLASH_as,
    /// mhr: **@CODE@** = -at focus particla
    /// sms: **@CODE@** -ät
    #[strum(serialize = "Foc/at")]
    Foc_SLASH_at,
    /// sms: **@CODE@**
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/ba")]
    Foc_SLASH_ba,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/bahal")]
    Foc_SLASH_bahal,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/bahan")]
    Foc_SLASH_bahan,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/bai")]
    Foc_SLASH_bai,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/ban")]
    Foc_SLASH_ban,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/bas")]
    Foc_SLASH_bas,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/bason")]
    Foc_SLASH_bason,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/bat")]
    Foc_SLASH_bat,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/be")]
    Foc_SLASH_be,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/behal")]
    Foc_SLASH_behal,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/behan")]
    Foc_SLASH_behan,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/beson")]
    Foc_SLASH_beson,
    /// smj: (no comment)
    /// sms: **@CODE@** -ga
    #[strum(serialize = "Foc/ga")]
    Foc_SLASH_ga,
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle
    #[strum(serialize = "Foc/gan")]
    Foc_SLASH_gan,
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle
    /// smj: (no comment)
    #[strum(serialize = "Foc/ge")]
    Foc_SLASH_ge,
    /// smj: (no comment)
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/gen")]
    Foc_SLASH_gen,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/ges")]
    Foc_SLASH_ges,
    /// sms: **@CODE@** -ǥa
    #[strum(serialize = "Foc/gha")]
    Foc_SLASH_gha,
    /// sms: **@CODE@** -ǥõs
    #[strum(serialize = "Foc/ghos")]
    Foc_SLASH_ghos,
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle
    #[strum(serialize = "Foc/gih")]
    Foc_SLASH_gih,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/gis")]
    Foc_SLASH_gis,
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle
    #[strum(serialize = "Foc/gænnah")]
    Foc_SLASH_gænnah,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    #[strum(serialize = "Foc/haan")]
    Foc_SLASH_haan,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/hal")]
    Foc_SLASH_hal,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fkv: **@CODE@** = Focusclitic -han
    /// sms: **@CODE@** This might be used with Use/NG
    /// sme: **@CODE@**	Focus clitic
    /// fin: (no comment)
    #[strum(serialize = "Foc/han")]
    Foc_SLASH_han,
    /// sms: **@CODE@** This might be used with
    /// vep: **@CODE@**  = perhaps +Addative
    #[strum(serialize = "Foc/i")]
    Foc_SLASH_i,
    /// mhr: **@CODE@** = -ja focus particle
    #[strum(serialize = "Foc/ja")]
    Foc_SLASH_ja,
    /// mhr: **@CODE@** = -jan focus particle
    #[strum(serialize = "Foc/jan")]
    Foc_SLASH_jan,
    /// sms: **@CODE@** -jõs
    #[strum(serialize = "Foc/jos")]
    Foc_SLASH_jos,
    /// fit: **@CODE@** = sjekk denne xxx
    /// fit: (no comment)
    /// sms: **@CODE@** This might be used with Use/NG ij-ka
    /// fin: (no comment)
    #[strum(serialize = "Foc/ka")]
    Foc_SLASH_ka,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fkv: **@CODE@** = Focusclitic -kaan
    /// fin: (no comment)
    #[strum(serialize = "Foc/kaan")]
    Foc_SLASH_kaan,
    /// sms: **@CODE@**
    #[strum(serialize = "Foc/kahan")]
    Foc_SLASH_kahan,
    /// fit: **@CODE@** = sjekk denne xxx
    #[strum(serialize = "Foc/kas")]
    Foc_SLASH_kas,
    /// sms: **@CODE@** This might be used with Use/NG
    /// vep: **@CODE@**  =
    #[strum(serialize = "Foc/ki")]
    Foc_SLASH_ki,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fkv: **@CODE@** = Focusclitic -kin
    /// fin: (no comment)
    #[strum(serialize = "Foc/kin")]
    Foc_SLASH_kin,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/mat")]
    Foc_SLASH_mat,
    /// sms: **@CODE@**
    #[strum(serialize = "Foc/mes")]
    Foc_SLASH_mes,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/mis")]
    Foc_SLASH_mis,
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/naj")]
    Foc_SLASH_naj,
    /// sms: **@CODE@**
    #[strum(serialize = "Foc/o")]
    Foc_SLASH_o,
    /// sms: **@CODE@** -õs
    #[strum(serialize = "Foc/os")]
    Foc_SLASH_os,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fkv: **@CODE@** = Focusclitic -pa
    /// fin: (no comment)
    #[strum(serialize = "Foc/pa")]
    Foc_SLASH_pa,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fkv: **@CODE@** = Focusclitic -pas
    #[strum(serialize = "Foc/pas")]
    Foc_SLASH_pas,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** = Focusclitic -s
    /// sme: **@CODE@**	Focus clitic
    /// fin: (no comment)
    #[strum(serialize = "Foc/s")]
    Foc_SLASH_s,
    /// sms: **@CODE@** -šât
    #[strum(serialize = "Foc/shat")]
    Foc_SLASH_shat,
    /// sms: **@CODE@**
    /// sme: **@CODE@**	Focus clitic
    #[strum(serialize = "Foc/son")]
    Foc_SLASH_son,
    /// sms: **@CODE@** -tâma
    #[strum(serialize = "Foc/tama")]
    Foc_SLASH_tama,
    /// sms: **@CODE@** -tõt
    #[strum(serialize = "Foc/tot")]
    Foc_SLASH_tot,
    /// mhr: **@CODE@** = -ys focus particle
    #[strum(serialize = "Foc/ys")]
    Foc_SLASH_ys,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Foc/žo")]
    Foc_SLASH_žo,
    /// mrj: **@CODE@** =
    #[strum(serialize = "Foc/Ат")]
    Foc_SLASH_Ат,
    /// mrj: **@CODE@** = too, also
    #[strum(serialize = "Foc/ок")]
    Foc_SLASH_ок,
    /// kpv: **@CODE@** = expressions such as аттьӧ, ало, ...
    #[strum(serialize = "Formulaic")]
    Formulaic,
    /// mhr: **@CODE@** = Future participle
    /// yrk: (no comment)
    /// lit: (no comment)
    #[strum(serialize = "Fut")]
    Fut,
    /// mrj: **@CODE@** =
    #[strum(serialize = "FutPrc")]
    FutPrc,
    /// smj: **@CODE@** Grade 2-3 for homonymies with grade 1-2, +N+G3
    /// sme: **@CODE@** Grade 2-3 for homonymies with grade 1-2, +N+G3
    #[strum(serialize = "G3")]
    G3,
    /// smj: **@CODE@** Grade 3, no consonant gradation, +N+G7
    /// sme: **@CODE@** Grade 3, no consonant gradation, +N+G7
    #[strum(serialize = "G7")]
    G7,
    /// fit: **@CODE@** = Genitive
    /// kpv: **@CODE@** genitive case -лӧн асалан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Genitive
    /// liv: **@CODE@** = genitive case
    /// mhr: **@CODE@** = genitive
    /// sma: **@CODE@** = Genitive
    /// nob: (no comment)
    /// smj: **@CODE@** = Genitive case
    /// yrk: **@CODE@** (Genitive)
    /// sms: **@CODE@** Genitive
    /// lit: (no comment)
    /// vep: **@CODE@**  = genitive case
    /// olo: **@CODE@** genitive case
    /// sme: **@CODE@** - Genitive
    /// fin: (no comment)
    /// vro: **@CODE@** genitive
    #[strum(serialize = "Gen")]
    Gen,
    /// sms: **@CODE@** Genitive attribute used as adjective?
    #[strum(serialize = "GenAttr")]
    GenAttr,
    /// kpv: **@CODE@**  Gerund This is used with derivations
    /// vot: (no comment)
    /// liv: **@CODE@** = gerund
    /// mhr: **@CODE@** = Gerund
    /// sma: **@CODE@** | Gerundium
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** gerund +Ess, +Loc +Instr
    /// lit: (no comment)
    /// mrj: **@CODE@** = gerund
    /// vep: **@CODE@**
    /// olo: **@CODE@** : Gerund
    /// sme: **@CODE@** Gerund
    /// vro: **@CODE@** ollõn
    #[strum(serialize = "Ger")]
    Ger,
    /// yrk: **@CODE@**
    #[strum(serialize = "GerFin")]
    GerFin,
    /// fit: (no comment)
    /// fkv: **@CODE@**: trisyllabic verbs
    /// smj: **@CODE@**:  trisyllabic verbs
    /// sme: **@CODE@**: trisyllabic verbs
    /// vro: **@CODE@**: trisyllabic verbs
    #[strum(serialize = "Gram/3syll")]
    Gram_SLASH_3syll,
    /// fit: (no comment)
    /// sma: **@CODE@**
    /// smj: **@CODE@** = Inherent comp, lexicalized derivation
    /// sme: **@CODE@**   Comparative, adverbs
    /// vro: (no comment)
    #[strum(serialize = "Gram/Comp")]
    Gram_SLASH_Comp,
    /// smj: **@CODE@** = Inherent diminutive, lexicalized derivation
    #[strum(serialize = "Gram/Dimin")]
    Gram_SLASH_Dimin,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// sma: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// nob: **@CODE@**
    /// smj: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// olo: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// sme: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// fin: (no comment)
    /// vro: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    #[strum(serialize = "Gram/IAbbr")]
    Gram_SLASH_IAbbr,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Intransitive abbreviations that are homonymous with more frequent words. They should only be considered abbreviations in the middle of a sentence.
    /// smj: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// olo: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// sme: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// vro: **@CODE@**:  Intransitive abbreviations that are homonymous
    #[strum(serialize = "Gram/NoAbbr")]
    Gram_SLASH_NoAbbr,
    /// smj: **@CODE@** = Inherent Actio Noun From Verb - Nomen Actionis, lexicalized derivation
    #[strum(serialize = "Gram/NomAct")]
    Gram_SLASH_NomAct,
    /// smj: **@CODE@** = Inherent Actor Noun From Verb - Nomen Agentis, lexicalized derivation
    #[strum(serialize = "Gram/NomAg")]
    Gram_SLASH_NomAg,
    /// smj: **@CODE@** = Inherent Intsrumental noun From Verb, Nomen instrumentalis,lexicalized derivation
    #[strum(serialize = "Gram/NomInstr")]
    Gram_SLASH_NomInstr,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Transitive abbreviations for which numerals are complements and normal words. The abbreviation usage is less common and thus only the occurences in the middle of the sentence can be considered as true cases.
    /// sma: **@CODE@**:  Transitive abbreviations for which numerals
    /// nob: **@CODE@**
    /// smj: **@CODE@**:  Transitive abbreviations for which numerals
    /// olo: **@CODE@**:  Transitive abbreviations for which numerals
    /// sme: **@CODE@**:  Transitive abbreviations for which numerals
    /// fin: (no comment)
    /// vro: **@CODE@**:  Transitive abbreviations for which numerals
    #[strum(serialize = "Gram/NumNoAbbr")]
    Gram_SLASH_NumNoAbbr,
    /// fit: (no comment)
    /// sma: **@CODE@**
    /// smj: **@CODE@** = Inherent superl, lexicalized derivation
    /// sme: **@CODE@**   Superlative, adverbs
    /// vro: (no comment)
    #[strum(serialize = "Gram/Superl")]
    Gram_SLASH_Superl,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// sma: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// nob: **@CODE@**
    /// smj: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// olo: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// sme: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// fin: (no comment)
    /// vro: **@CODE@**:  Transitive abbreviation (it needs an argument)
    #[strum(serialize = "Gram/TAbbr")]
    Gram_SLASH_TAbbr,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Both transitive and intransitive abbreviation
    /// sma: **@CODE@**:  Both transitive and intransitive abbreviation
    /// smj: **@CODE@**:  Both transitive and intransitive abbreviation
    /// olo: **@CODE@**:  Both transitive and intransitive abbreviation
    /// sme: **@CODE@**:  Both transitive and intransitive abbreviation
    /// vro: **@CODE@**:  Both transitive and intransitive abbreviation
    #[strum(serialize = "Gram/TIAbbr")]
    Gram_SLASH_TIAbbr,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Transitive abbreviation if the following constituent is numeric
    /// sma: **@CODE@**:  Transitive abbreviation if the following
    /// nob: **@CODE@**
    /// smj: **@CODE@**:  Transitive abbreviation if the following
    /// olo: **@CODE@**:  Transitive abbreviation if the following
    /// sme: **@CODE@**:  Transitive abbreviation if the following
    /// fin: (no comment)
    /// vro: **@CODE@**:  Transitive abbreviation if the following
    #[strum(serialize = "Gram/TNumAbbr")]
    Gram_SLASH_TNumAbbr,
    /// smj: **@CODE@** = Inherent -r derivation. guollit-guollár
    #[strum(serialize = "Gram/r")]
    Gram_SLASH_r,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Group")]
    Group,
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  for the name guesser ?
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** = Non-dictionary words (probably not in  use
    /// vep: **@CODE@**  =
    /// zxx: (no comment)
    /// olo: **@CODE@**
    /// sme: **@CODE@** for the name guesser
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Guess")]
    Guess,
    /// fin: (no comment)
    #[strum(serialize = "Gyr")]
    Gyr,
    /// fit: **@CODE@** = Hyphenation mark
    /// fkv: **@CODE@** = Hyphenation mark
    /// sms: **@CODE@**
    #[strum(serialize = "HYPH")]
    HYPH,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  Homonymy
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = First pattern (let us say -ам)
    /// sma: **@CODE@**:  Homonymy
    /// smj: **@CODE@**:  Homonymy
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** : Homonymy толам used in l element and lexc
    /// vep: **@CODE@**
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom1")]
    Hom1,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  Homonymy
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = Second pattern (let us say -ем)
    /// sma: **@CODE@**:  Homonymy
    /// smj: **@CODE@**:  Homonymy
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** : Homonymy колем used in l element and lexc
    /// vep: **@CODE@**
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom2")]
    Hom2,
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = Third pattern (if it should exist + even more?)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom3")]
    Hom3,
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom4")]
    Hom4,
    /// mhr: **@CODE@** =
    /// fin: (no comment)
    #[strum(serialize = "Hom5")]
    Hom5,
    /// mhr: **@CODE@** =
    /// fin: (no comment)
    #[strum(serialize = "Hom6")]
    Hom6,
    /// fin: (no comment)
    #[strum(serialize = "Hom7")]
    Hom7,
    /// fin: (no comment)
    #[strum(serialize = "Hom8")]
    Hom8,
    /// fin: (no comment)
    #[strum(serialize = "Hom9")]
    Hom9,
    /// yrk: **@CODE@**
    #[strum(serialize = "Hort")]
    Hort,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Hum")]
    Hum,
    /// nob: **@CODE@** = ing-derivation
    #[strum(serialize = "ING")]
    ING,
    /// sme: (no comment)
    #[strum(serialize = "ITRAB")]
    ITRAB,
    /// fit: **@CODE@** intransitive
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Intansitive verb
    /// liv: **@CODE@** = Intransitive verb
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  Intansitive verb
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Intransitive
    /// lit: (no comment)
    /// vep: **@CODE@**  = intransitive verbs
    /// sme: **@CODE@** Intransitive Verb, +V+IV
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "IV")]
    IV,
    /// kpv: **@CODE@** These are ideophonic descriptors used to modify the verb --  вырк ливтясь "**flit** and it flew off"
    #[strum(serialize = "Ideoph")]
    Ideoph,
    /// ceb: **@CODE@** - Indirect
    #[strum(serialize = "Idr")]
    Idr,
    /// fit: **@CODE@** = Illative
    /// kpv: **@CODE@** illative -ӧ пыран
    /// vot: (no comment)
    /// fkv: **@CODE@** = Illative
    /// liv: **@CODE@** = illative
    /// mhr: **@CODE@** = illative
    /// sma: **@CODE@**= Illative
    /// nob: (no comment)
    /// smj: **@CODE@** = Illative case
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Illative
    /// vep: **@CODE@**  = illative
    /// vep: (no comment)
    /// vep: (no comment)
    /// olo: **@CODE@** illative
    /// sme: **@CODE@** - Illative
    /// fin: (no comment)
    /// vro: **@CODE@** illative
    #[strum(serialize = "Ill")]
    Ill,
    /// ceb: **@CODE@** -
    #[strum(serialize = "Imp")]
    Imp,
    /// rus: (no comment)
    #[strum(serialize = "Impf")]
    Impf,
    /// mhr: **@CODE@** = Imperfective (?) -- XXX check this
    #[strum(serialize = "Imprf")]
    Imprf,
    /// rus: (no comment)
    #[strum(serialize = "Imprs")]
    Imprs,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Imperative
    /// liv: **@CODE@** = imperative
    /// mhr: **@CODE@** = imperative
    /// sma: **@CODE@** | Imperative
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Imperative mood
    /// lit: (no comment)
    /// vep: **@CODE@**  = imperative
    /// olo: **@CODE@** Imperative
    /// sme: **@CODE@** Imperative
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Imprt")]
    Imprt,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Imperative, for Neg:  ollem ollh ...
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =  ыма
    /// vep: **@CODE@**  =
    /// sme: **@CODE@** Alternative, Rather Declamatory Imperative Form - Infrequent not in use
    /// vro: **@CODE@**
    #[strum(serialize = "ImprtII")]
    ImprtII,
    /// rus: (no comment)
    #[strum(serialize = "Inan")]
    Inan,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@** tense
    /// vot: (no comment)
    /// fkv: **@CODE@** = Indicative
    /// liv: **@CODE@** = indicative
    /// mhr: **@CODE@** = indicative
    /// sma: **@CODE@** | Indicative
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Indicative mood
    /// lit: (no comment)
    /// mrj: **@CODE@** =  Verb moods
    /// vep: **@CODE@**  = indicative
    /// zxx: (no comment)
    /// olo: **@CODE@** Indicative
    /// sme: **@CODE@** Indicative
    /// fin: (no comment)
    /// ceb: **@CODE@** - Indicative
    /// ces: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Ind")]
    Ind,
    /// fit: **@CODE@** = Indefinitue
    /// kpv: **@CODE@**:  indefinite
    /// vot: (no comment)
    /// fkv: **@CODE@** = Indefinitue
    /// liv: **@CODE@** = indefinite
    /// mhr: **@CODE@** = Indefinite pronoun
    /// sma: **@CODE@** = Indefinite
    /// nob: **@CODE@** =
    /// smj: **@CODE@** = indefinite pronoun
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Indefinitive Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  indefinite
    /// vep: **@CODE@**  = indefinite
    /// olo: **@CODE@** indefinite
    /// sme: **@CODE@** - Indefinitive Pronoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Indef")]
    Indef,
    /// mhr: **@CODE@** = forms огым, огыт, ите
    #[strum(serialize = "Indep")]
    Indep,
    /// fit: **@CODE@** = Inessive
    /// kpv: **@CODE@** inessive -ын ина
    /// vot: (no comment)
    /// fkv: **@CODE@** = Inessive
    /// liv: **@CODE@** = inessive
    /// mhr: **@CODE@** = inessive
    /// sma: **@CODE@** = Inesive
    /// smj: **@CODE@** = Inesive case
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = inessive
    /// vep: (no comment)
    /// vep: (no comment)
    /// olo: **@CODE@** inessive
    /// fin: (no comment)
    /// vro: **@CODE@** inessive
    #[strum(serialize = "Ine")]
    Ine,
    /// fit: **@CODE@** = tA Infinitive
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Infinitive
    /// liv: **@CODE@** = infinitive
    /// mhr: **@CODE@** = Infinitive
    /// sma: **@CODE@** | Infinitive and participles
    /// nob: **@CODE@** = for infinite verbs
    /// smj: (no comment)
    /// rus: (no comment)
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** = infinitive
    /// vep: **@CODE@**
    /// zxx: (no comment)
    /// olo: **@CODE@** : Infinitive
    /// sme: **@CODE@** Infinitive
    /// ceb: **@CODE@** -
    /// ces: (no comment)
    /// vro: **@CODE@** sõimadaq, elädäq
    #[strum(serialize = "Inf")]
    Inf,
    /// vro: **@CODE@** sõimama, elämä
    #[strum(serialize = "Inf/mA")]
    Inf_SLASH_mA,
    /// fkv: **@CODE@** = Infinite 3
    #[strum(serialize = "Inf3")]
    Inf3,
    /// fin: (no comment)
    #[strum(serialize = "InfA")]
    InfA,
    /// fit: **@CODE@** = e Infinite
    /// fin: (no comment)
    #[strum(serialize = "InfE")]
    InfE,
    /// yrk: **@CODE@**
    #[strum(serialize = "InfImprf")]
    InfImprf,
    /// fit: **@CODE@** = mA Infinite
    /// fin: (no comment)
    #[strum(serialize = "InfMa")]
    InfMa,
    /// yrk: **@CODE@**
    #[strum(serialize = "InfPrf")]
    InfPrf,
    /// fit: **@CODE@** = Instructive
    /// kpv: **@CODE@** instrumental -ӧн керанторъя
    /// fkv: **@CODE@** = Instructive
    /// liv: **@CODE@** = instrumental -KÕKS
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  = instrumental
    /// olo: **@CODE@** instrumental
    /// fin: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Ins")]
    Ins,
    /// lit: (no comment)
    #[strum(serialize = "Inst")]
    Inst,
    /// kpv: **@CODE@**
    /// liv: **@CODE@** = instructive -IN
    /// mhr: **@CODE@** =
    /// sms: **@CODE@** -eeʹl
    /// vep: **@CODE@**  = instructive -IN
    #[strum(serialize = "Instr")]
    Instr,
    /// kpv: **@CODE@**
    /// nob: **@CODE@** = **hmm, what is this...**
    #[strum(serialize = "Intens")]
    Intens,
    /// mrj: **@CODE@** =
    #[strum(serialize = "Intensive")]
    Intensive,
    /// fit: **@CODE@** = Interjection
    /// kpv: **@CODE@**:  interjection   междометтьӧ   междометие
    /// vot: (no comment)
    /// fkv: **@CODE@** = Interjection
    /// liv: **@CODE@** = interjection
    /// liv: (no comment)
    /// mhr: **@CODE@** = interjections
    /// mhr: (no comment)
    /// mhr: (no comment)
    /// sma: **@CODE@** = Interjection
    /// smj: **@CODE@** = Interjection
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Interjection
    /// lit: (no comment)
    /// vep: **@CODE@**  = interjection
    /// olo: **@CODE@**  interjection
    /// sme: **@CODE@** - Interjection
    /// fin: (no comment)
    /// vro: **@CODE@** Interjections
    /// vro: (no comment)
    #[strum(serialize = "Interj")]
    Interj,
    /// fit: **@CODE@** = Interrogative
    /// kpv: **@CODE@**:  interrogative
    /// vot: (no comment)
    /// fkv: **@CODE@** = Interrogative
    /// liv: **@CODE@** = interrogative
    /// mhr: **@CODE@** = Interrogative pronoun
    /// sma: **@CODE@** = Interrogative
    /// smj: **@CODE@** = Interrogative pronoun
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Interrogative Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  interrogative
    /// vep: **@CODE@**  = interrogative
    /// olo: **@CODE@** interrogative
    /// sme: **@CODE@** - Interrogative Pronoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Interr")]
    Interr,
    /// kpv: **@CODE@**  Iterative form expressing number of times
    /// kpv: (no comment)
    /// olo: **@CODE@** Iterative form expressing number of times; myv: `кавксть`, kpv: `кыкысь`
    #[strum(serialize = "Iter")]
    Iter,
    /// liv: **@CODE@** = jussitive
    /// vro: **@CODE@**
    #[strum(serialize = "Jus")]
    Jus,
    /// sms: **@CODE@** Kin term This may need to be removed, let Sem/Hum-kin suffice
    #[strum(serialize = "Kin")]
    Kin,
    /// sme: **@CODE@** - man (different from maid): mii+Pron+Rel+Sg+Acc+Known
    #[strum(serialize = "Known")]
    Known,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = left parenth
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = paired symbols
    /// sma: **@CODE@**:  XXX These should be documented better
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = left paranthesis
    /// vep: **@CODE@**
    /// olo: **@CODE@**
    /// sme: **@CODE@**  left paranthesis
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "LEFT")]
    LEFT,
    /// sma: **@CODE@**:  ad hoc tag for development purposes ?
    #[strum(serialize = "LOAN")]
    LOAN,
    /// rus: (no comment)
    #[strum(serialize = "LPAR")]
    LPAR,
    /// rus: (no comment)
    #[strum(serialize = "LQUOT")]
    LQUOT,
    /// sms: **@CODE@** largo variant instead of allegro saaǥǥstõõllâd vs saǥstõõllâd
    #[strum(serialize = "Largo")]
    Largo,
    /// fkv: **@CODE@** = lative (the infinitive, used in Apertium)
    /// liv: **@CODE@** = Lative
    /// mhr: **@CODE@** = lative
    /// sms: **@CODE@** Lative lääinas, säämas, toimmpââjas
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = Lative
    /// vep: (no comment)
    /// olo: **@CODE@** Lative
    /// fin: (no comment)
    #[strum(serialize = "Lat")]
    Lat,
    /// rus: (no comment)
    #[strum(serialize = "Leng")]
    Leng,
    /// ceb: **@CODE@** -  Ligature
    /// ceb: (no comment)
    #[strum(serialize = "Lig")]
    Lig,
    /// yrk: **@CODE@** limitative
    #[strum(serialize = "Lim")]
    Lim,
    /// kpv: **@CODE@** LocMod, IneMod Быд во шедӧдӧны бур успеваемость Воркута да Инта каръясса, Прилузскӧй да Княжпогостскӧй районъясса школаяс.
    /// vot: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** = Locative case
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Locative
    /// lit: (no comment)
    /// vep: **@CODE@**  = Locative
    /// olo: **@CODE@** Locative
    /// sme: **@CODE@** - Locative = Inessive and Ellative
    /// ceb: (no comment)
    /// vro: **@CODE@** locative
    #[strum(serialize = "Loc")]
    Loc,
    /// rus: (no comment)
    #[strum(serialize = "Loc2")]
    Loc2,
    /// kpv: **@CODE@** move to Loc
    #[strum(serialize = "LocMod")]
    LocMod,
    /// mhr: **@CODE@** = location, better witho LocusPl to avoid Loc case?
    #[strum(serialize = "LocPl")]
    LocPl,
    /// sma: **@CODE@** - ?
    #[strum(serialize = "Logo")]
    Logo,
    /// smj: **@CODE@**  Long form
    #[strum(serialize = "Long")]
    Long,
    /// rus: (no comment)
    #[strum(serialize = "Lxc")]
    Lxc,
    /// kpv: **@CODE@** abessive modifier -тӧм
    #[strum(serialize = "MAbe")]
    MAbe,
    /// kpv: **@CODE@** habeo modifier а -
    #[strum(serialize = "MHab")]
    MHab,
    /// sme: **@CODE@**  in-word punctuation, typically hyphen, used to indicate a measurement span of some sort
    #[strum(serialize = "MIDDLE")]
    MIDDLE,
    /// kpv: **@CODE@** locative modifier са -
    #[strum(serialize = "MLoc")]
    MLoc,
    /// kpv: **@CODE@** temporal modifier ся -
    #[strum(serialize = "MTmp")]
    MTmp,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** = multiword expression, for tokenisation
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@**  multi word expressions, goes to abbr
    /// sms: **@CODE@** never last element of compound words
    /// vep: **@CODE@**
    /// olo: **@CODE@** (both?)
    /// sme: **@CODE@** - Multi-word expressions treated as such in the preprocessor. To be added as first tag after the lemma
    /// fin: (no comment)
    #[strum(serialize = "MWE")]
    MWE,
    /// mhr: **@CODE@** Split point for MWE
    /// nob: **@CODE@** Split point for MWE
    /// sme: **@CODE@** Split point for MWE
    #[strum(serialize = "MWESplit")]
    MWESplit,
    /// kpv: **@CODE@**  check! used once, should it be +Der/MWN?, Well, yes.
    #[strum(serialize = "MWN")]
    MWN,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Mal")]
    Mal,
    /// kpv: **@CODE@** with reference to type of adverb
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// vep: (no comment)
    /// olo: (no comment)
    #[strum(serialize = "Manner")]
    Manner,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Measr")]
    Measr,
    /// yrk: **@CODE@** approximative imperfective
    #[strum(serialize = "Mod/appr")]
    Mod_SLASH_appr,
    /// yrk: **@CODE@** desiderative
    #[strum(serialize = "Mod/des")]
    Mod_SLASH_des,
    /// yrk: **@CODE@** approximative futuritive
    #[strum(serialize = "Mod/futappr")]
    Mod_SLASH_futappr,
    /// yrk: **@CODE@** hyperprobablitative
    #[strum(serialize = "Mod/hyp")]
    Mod_SLASH_hyp,
    /// yrk: **@CODE@** interrogative
    #[strum(serialize = "Mod/int")]
    Mod_SLASH_int,
    /// yrk: **@CODE@** narrative
    #[strum(serialize = "Mod/narr")]
    Mod_SLASH_narr,
    /// yrk: **@CODE@** necessitativee
    #[strum(serialize = "Mod/nec")]
    Mod_SLASH_nec,
    /// yrk: **@CODE@** obligative
    #[strum(serialize = "Mod/obl")]
    Mod_SLASH_obl,
    /// yrk: **@CODE@** approximative perfective
    #[strum(serialize = "Mod/perfappr")]
    Mod_SLASH_perfappr,
    /// yrk: **@CODE@** perfective probabilitative
    #[strum(serialize = "Mod/perfprob")]
    Mod_SLASH_perfprob,
    /// yrk: **@CODE@** imperfective probabilitative
    #[strum(serialize = "Mod/prob")]
    Mod_SLASH_prob,
    /// yrk: **@CODE@** reputative
    #[strum(serialize = "Mod/rep")]
    Mod_SLASH_rep,
    /// yrk: **@CODE@** superprobabilitative
    #[strum(serialize = "Mod/sup")]
    Mod_SLASH_sup,
    /// nob: **@CODE@** = Gender. MF = Masc or Fem (used for adjs, not nouns)
    /// rus: (no comment)
    /// lit: (no comment)
    /// ces: (no comment)
    #[strum(serialize = "Msc")]
    Msc,
    /// kpv: **@CODE@** multiplicative, i.e. iterations
    /// kpv: (no comment)
    #[strum(serialize = "Mult")]
    Mult,
    /// kpv: **@CODE@** Special multiword units are analysed with:
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  Multiword phrase tag ?
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** = !! multiword units
    /// vep: **@CODE@**  =
    /// zxx: (no comment)
    /// olo: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Multi")]
    Multi,
    /// fit: **@CODE@** = Noun
    /// kpv: **@CODE@**:  noun      эмакыв    - существительное
    /// vot: (no comment)
    /// fkv: **@CODE@** = Noun
    /// liv: **@CODE@** = noun
    /// mhr: **@CODE@** = nouns
    /// mhr: (no comment)
    /// sma: **@CODE@** = Noun
    /// nob: **@CODE@** = Open parts of speech
    /// smj: **@CODE@** = Noun
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Noun
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**  = noun
    /// zxx: (no comment)
    /// olo: **@CODE@**  noun
    /// sme: **@CODE@** - Noun
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Noun
    /// ces: (no comment)
    /// vro: **@CODE@** Noun
    #[strum(serialize = "N")]
    N,
    /// liv: (no comment)
    #[strum(serialize = "N_100_pan")]
    N_100_pan,
    /// liv: (no comment)
    #[strum(serialize = "N_101_täm")]
    N_101_täm,
    /// liv: (no comment)
    #[strum(serialize = "N_102_nirjtj")]
    N_102_nirjtj,
    /// liv: (no comment)
    #[strum(serialize = "N_103_kulj")]
    N_103_kulj,
    /// liv: (no comment)
    #[strum(serialize = "N_104_panj")]
    N_104_panj,
    /// liv: (no comment)
    #[strum(serialize = "N_105_danjtsh")]
    N_105_danjtsh,
    /// liv: (no comment)
    #[strum(serialize = "N_106_koush")]
    N_106_koush,
    /// liv: (no comment)
    #[strum(serialize = "N_107_spleish")]
    N_107_spleish,
    /// liv: (no comment)
    #[strum(serialize = "N_108_veis")]
    N_108_veis,
    /// liv: (no comment)
    #[strum(serialize = "N_109_pois")]
    N_109_pois,
    /// liv: (no comment)
    #[strum(serialize = "N_112_taeuzh")]
    N_112_taeuzh,
    /// liv: (no comment)
    #[strum(serialize = "N_113_varjzh")]
    N_113_varjzh,
    /// liv: (no comment)
    #[strum(serialize = "N_114_pierz")]
    N_114_pierz,
    /// liv: (no comment)
    #[strum(serialize = "N_115_vorgo")]
    N_115_vorgo,
    /// liv: (no comment)
    #[strum(serialize = "N_116_jougo")]
    N_116_jougo,
    /// liv: (no comment)
    #[strum(serialize = "N_117_vooidag")]
    N_117_vooidag,
    /// liv: (no comment)
    #[strum(serialize = "N_118_sieldo")]
    N_118_sieldo,
    /// liv: (no comment)
    #[strum(serialize = "N_119_nooqgo")]
    N_119_nooqgo,
    /// liv: (no comment)
    #[strum(serialize = "N_120_ashsho")]
    N_120_ashsho,
    /// liv: (no comment)
    #[strum(serialize = "N_121_druuqosho")]
    N_121_druuqosho,
    /// liv: (no comment)
    #[strum(serialize = "N_122_soqv")]
    N_122_soqv,
    /// liv: (no comment)
    #[strum(serialize = "N_123_kooj")]
    N_123_kooj,
    /// liv: (no comment)
    #[strum(serialize = "N_124_ooqj")]
    N_124_ooqj,
    /// liv: (no comment)
    #[strum(serialize = "N_125_irm")]
    N_125_irm,
    /// liv: (no comment)
    #[strum(serialize = "N_126_kim")]
    N_126_kim,
    /// liv: (no comment)
    #[strum(serialize = "N_127_vonj")]
    N_127_vonj,
    /// liv: (no comment)
    #[strum(serialize = "N_128_vaqit")]
    N_128_vaqit,
    /// liv: (no comment)
    #[strum(serialize = "N_129_amaat")]
    N_129_amaat,
    /// liv: (no comment)
    #[strum(serialize = "N_12_puu")]
    N_12_puu,
    /// liv: (no comment)
    #[strum(serialize = "N_130_kultuur")]
    N_130_kultuur,
    /// liv: (no comment)
    #[strum(serialize = "N_131_saqgdit")]
    N_131_saqgdit,
    /// liv: (no comment)
    #[strum(serialize = "N_132_viljtj")]
    N_132_viljtj,
    /// liv: (no comment)
    #[strum(serialize = "N_133_elj")]
    N_133_elj,
    /// liv: (no comment)
    #[strum(serialize = "N_134_bleeqdj")]
    N_134_bleeqdj,
    /// liv: (no comment)
    #[strum(serialize = "N_135_fakt")]
    N_135_fakt,
    /// liv: (no comment)
    #[strum(serialize = "N_136_tas")]
    N_136_tas,
    /// liv: (no comment)
    #[strum(serialize = "N_137_neitst")]
    N_137_neitst,
    /// liv: (no comment)
    #[strum(serialize = "N_138_siiend")]
    N_138_siiend,
    /// liv: (no comment)
    #[strum(serialize = "N_139_laeaeqnd")]
    N_139_laeaeqnd,
    /// liv: (no comment)
    #[strum(serialize = "N_13_rooq")]
    N_13_rooq,
    /// liv: (no comment)
    #[strum(serialize = "N_140_aaigast")]
    N_140_aaigast,
    /// liv: (no comment)
    #[strum(serialize = "N_141_analiiz")]
    N_141_analiiz,
    /// liv: (no comment)
    #[strum(serialize = "N_142_niiqem")]
    N_142_niiqem,
    /// liv: (no comment)
    #[strum(serialize = "N_143_jaljksh")]
    N_143_jaljksh,
    /// liv: (no comment)
    #[strum(serialize = "N_144_vish")]
    N_144_vish,
    /// liv: (no comment)
    #[strum(serialize = "N_145_ruuqtsh")]
    N_145_ruuqtsh,
    /// liv: (no comment)
    #[strum(serialize = "N_146_teeqdj")]
    N_146_teeqdj,
    /// liv: (no comment)
    #[strum(serialize = "N_147_leeqdj")]
    N_147_leeqdj,
    /// liv: (no comment)
    #[strum(serialize = "N_148_kiintsh")]
    N_148_kiintsh,
    /// liv: (no comment)
    #[strum(serialize = "N_149_koonjtsh")]
    N_149_koonjtsh,
    /// liv: (no comment)
    #[strum(serialize = "N_14_paeae")]
    N_14_paeae,
    /// liv: (no comment)
    #[strum(serialize = "N_150_leenjtsh")]
    N_150_leenjtsh,
    /// liv: (no comment)
    #[strum(serialize = "N_151_liiem")]
    N_151_liiem,
    /// liv: (no comment)
    #[strum(serialize = "N_152_aqm")]
    N_152_aqm,
    /// liv: (no comment)
    #[strum(serialize = "N_153_azuum")]
    N_153_azuum,
    /// liv: (no comment)
    #[strum(serialize = "N_154_votjiim")]
    N_154_votjiim,
    /// liv: (no comment)
    #[strum(serialize = "N_155_koonjim")]
    N_155_koonjim,
    /// liv: (no comment)
    #[strum(serialize = "N_156_eetam")]
    N_156_eetam,
    /// liv: (no comment)
    #[strum(serialize = "N_157_sidaam")]
    N_157_sidaam,
    /// liv: (no comment)
    #[strum(serialize = "N_158_armtob")]
    N_158_armtob,
    /// liv: (no comment)
    #[strum(serialize = "N_159_koorand")]
    N_159_koorand,
    /// liv: (no comment)
    #[strum(serialize = "N_15_tiie")]
    N_15_tiie,
    /// liv: (no comment)
    #[strum(serialize = "N_160_ooqdog")]
    N_160_ooqdog,
    /// liv: (no comment)
    #[strum(serialize = "N_161_")]
    N_161_,
    /// liv: (no comment)
    #[strum(serialize = "N_162_")]
    N_162_,
    /// liv: (no comment)
    #[strum(serialize = "N_163_")]
    N_163_,
    /// liv: (no comment)
    #[strum(serialize = "N_164_")]
    N_164_,
    /// liv: (no comment)
    #[strum(serialize = "N_165_")]
    N_165_,
    /// liv: (no comment)
    #[strum(serialize = "N_166_")]
    N_166_,
    /// liv: (no comment)
    #[strum(serialize = "N_167_")]
    N_167_,
    /// liv: (no comment)
    #[strum(serialize = "N_168_")]
    N_168_,
    /// liv: (no comment)
    #[strum(serialize = "N_169_")]
    N_169_,
    /// liv: (no comment)
    #[strum(serialize = "N_16_brii")]
    N_16_brii,
    /// liv: (no comment)
    #[strum(serialize = "N_170_")]
    N_170_,
    /// liv: (no comment)
    #[strum(serialize = "N_171_")]
    N_171_,
    /// liv: (no comment)
    #[strum(serialize = "N_172_")]
    N_172_,
    /// liv: (no comment)
    #[strum(serialize = "N_173_")]
    N_173_,
    /// liv: (no comment)
    #[strum(serialize = "N_174_")]
    N_174_,
    /// liv: (no comment)
    #[strum(serialize = "N_175_")]
    N_175_,
    /// liv: (no comment)
    #[strum(serialize = "N_176_")]
    N_176_,
    /// liv: (no comment)
    #[strum(serialize = "N_177_")]
    N_177_,
    /// liv: (no comment)
    #[strum(serialize = "N_178_")]
    N_178_,
    /// liv: (no comment)
    #[strum(serialize = "N_179_")]
    N_179_,
    /// liv: (no comment)
    #[strum(serialize = "N_17_tei")]
    N_17_tei,
    /// liv: (no comment)
    #[strum(serialize = "N_180_")]
    N_180_,
    /// liv: (no comment)
    #[strum(serialize = "N_181_")]
    N_181_,
    /// liv: (no comment)
    #[strum(serialize = "N_182_")]
    N_182_,
    /// liv: (no comment)
    #[strum(serialize = "N_183_")]
    N_183_,
    /// liv: (no comment)
    #[strum(serialize = "N_184_")]
    N_184_,
    /// liv: (no comment)
    #[strum(serialize = "N_185_")]
    N_185_,
    /// liv: (no comment)
    #[strum(serialize = "N_186_")]
    N_186_,
    /// liv: (no comment)
    #[strum(serialize = "N_187_")]
    N_187_,
    /// liv: (no comment)
    #[strum(serialize = "N_188_")]
    N_188_,
    /// liv: (no comment)
    #[strum(serialize = "N_189_")]
    N_189_,
    /// liv: (no comment)
    #[strum(serialize = "N_18_kalaa")]
    N_18_kalaa,
    /// liv: (no comment)
    #[strum(serialize = "N_18a_iree")]
    N_18a_iree,
    /// liv: (no comment)
    #[strum(serialize = "N_190_")]
    N_190_,
    /// liv: (no comment)
    #[strum(serialize = "N_191_")]
    N_191_,
    /// liv: (no comment)
    #[strum(serialize = "N_192_")]
    N_192_,
    /// liv: (no comment)
    #[strum(serialize = "N_193_")]
    N_193_,
    /// liv: (no comment)
    #[strum(serialize = "N_194_")]
    N_194_,
    /// liv: (no comment)
    #[strum(serialize = "N_195_")]
    N_195_,
    /// liv: (no comment)
    #[strum(serialize = "N_196_")]
    N_196_,
    /// liv: (no comment)
    #[strum(serialize = "N_197_")]
    N_197_,
    /// liv: (no comment)
    #[strum(serialize = "N_198_")]
    N_198_,
    /// liv: (no comment)
    #[strum(serialize = "N_199_")]
    N_199_,
    /// liv: (no comment)
    #[strum(serialize = "N_19_tubaa")]
    N_19_tubaa,
    /// liv: (no comment)
    #[strum(serialize = "N_19a_amaa")]
    N_19a_amaa,
    /// liv: (no comment)
    #[strum(serialize = "N_200_")]
    N_200_,
    /// liv: (no comment)
    #[strum(serialize = "N_201_")]
    N_201_,
    /// liv: (no comment)
    #[strum(serialize = "N_202_")]
    N_202_,
    /// liv: (no comment)
    #[strum(serialize = "N_203_")]
    N_203_,
    /// liv: (no comment)
    #[strum(serialize = "N_204_")]
    N_204_,
    /// liv: (no comment)
    #[strum(serialize = "N_205_")]
    N_205_,
    /// liv: (no comment)
    #[strum(serialize = "N_206_")]
    N_206_,
    /// liv: (no comment)
    #[strum(serialize = "N_207_")]
    N_207_,
    /// liv: (no comment)
    #[strum(serialize = "N_208_")]
    N_208_,
    /// liv: (no comment)
    #[strum(serialize = "N_209_")]
    N_209_,
    /// liv: (no comment)
    #[strum(serialize = "N_20_aigaa")]
    N_20_aigaa,
    /// liv: (no comment)
    #[strum(serialize = "N_210_")]
    N_210_,
    /// liv: (no comment)
    #[strum(serialize = "N_211_")]
    N_211_,
    /// liv: (no comment)
    #[strum(serialize = "N_212_")]
    N_212_,
    /// liv: (no comment)
    #[strum(serialize = "N_213_")]
    N_213_,
    /// liv: (no comment)
    #[strum(serialize = "N_214_")]
    N_214_,
    /// liv: (no comment)
    #[strum(serialize = "N_215_")]
    N_215_,
    /// liv: (no comment)
    #[strum(serialize = "N_216_")]
    N_216_,
    /// liv: (no comment)
    #[strum(serialize = "N_217_")]
    N_217_,
    /// liv: (no comment)
    #[strum(serialize = "N_218_")]
    N_218_,
    /// liv: (no comment)
    #[strum(serialize = "N_219_")]
    N_219_,
    /// liv: (no comment)
    #[strum(serialize = "N_21_leeba")]
    N_21_leeba,
    /// liv: (no comment)
    #[strum(serialize = "N_220_")]
    N_220_,
    /// liv: (no comment)
    #[strum(serialize = "N_221_")]
    N_221_,
    /// liv: (no comment)
    #[strum(serialize = "N_222_")]
    N_222_,
    /// liv: (no comment)
    #[strum(serialize = "N_223_")]
    N_223_,
    /// liv: (no comment)
    #[strum(serialize = "N_224_")]
    N_224_,
    /// liv: (no comment)
    #[strum(serialize = "N_225_")]
    N_225_,
    /// liv: (no comment)
    #[strum(serialize = "N_226_")]
    N_226_,
    /// liv: (no comment)
    #[strum(serialize = "N_227_")]
    N_227_,
    /// liv: (no comment)
    #[strum(serialize = "N_228_")]
    N_228_,
    /// liv: (no comment)
    #[strum(serialize = "N_229_")]
    N_229_,
    /// liv: (no comment)
    #[strum(serialize = "N_230_")]
    N_230_,
    /// liv: (no comment)
    #[strum(serialize = "N_231_")]
    N_231_,
    /// liv: (no comment)
    #[strum(serialize = "N_232_")]
    N_232_,
    /// liv: (no comment)
    #[strum(serialize = "N_233_")]
    N_233_,
    /// liv: (no comment)
    #[strum(serialize = "N_234_")]
    N_234_,
    /// liv: (no comment)
    #[strum(serialize = "N_235_")]
    N_235_,
    /// liv: (no comment)
    #[strum(serialize = "N_236_")]
    N_236_,
    /// liv: (no comment)
    #[strum(serialize = "N_237_")]
    N_237_,
    /// liv: (no comment)
    #[strum(serialize = "N_238_")]
    N_238_,
    /// liv: (no comment)
    #[strum(serialize = "N_239_")]
    N_239_,
    /// liv: (no comment)
    #[strum(serialize = "N_23_looja")]
    N_23_looja,
    /// liv: (no comment)
    #[strum(serialize = "N_240_")]
    N_240_,
    /// liv: (no comment)
    #[strum(serialize = "N_241_")]
    N_241_,
    /// liv: (no comment)
    #[strum(serialize = "N_242_")]
    N_242_,
    /// liv: (no comment)
    #[strum(serialize = "N_243_")]
    N_243_,
    /// liv: (no comment)
    #[strum(serialize = "N_244_")]
    N_244_,
    /// liv: (no comment)
    #[strum(serialize = "N_245_")]
    N_245_,
    /// liv: (no comment)
    #[strum(serialize = "N_246_")]
    N_246_,
    /// liv: (no comment)
    #[strum(serialize = "N_247_")]
    N_247_,
    /// liv: (no comment)
    #[strum(serialize = "N_248_")]
    N_248_,
    /// liv: (no comment)
    #[strum(serialize = "N_249_")]
    N_249_,
    /// liv: (no comment)
    #[strum(serialize = "N_24_paeaeva")]
    N_24_paeaeva,
    /// liv: (no comment)
    #[strum(serialize = "N_25_izaa")]
    N_25_izaa,
    /// liv: (no comment)
    #[strum(serialize = "N_25a_piezaa")]
    N_25a_piezaa,
    /// liv: (no comment)
    #[strum(serialize = "N_26_azhaa")]
    N_26_azhaa,
    /// liv: (no comment)
    #[strum(serialize = "N_27_aaldzha")]
    N_27_aaldzha,
    /// liv: (no comment)
    #[strum(serialize = "N_28_sukaa")]
    N_28_sukaa,
    /// liv: (no comment)
    #[strum(serialize = "N_29_liepaa")]
    N_29_liepaa,
    /// liv: (no comment)
    #[strum(serialize = "N_30_oksaa")]
    N_30_oksaa,
    /// liv: (no comment)
    #[strum(serialize = "N_31_voontsa")]
    N_31_voontsa,
    /// liv: (no comment)
    #[strum(serialize = "N_32_liestaa")]
    N_32_liestaa,
    /// liv: (no comment)
    #[strum(serialize = "N_33_aaiga")]
    N_33_aaiga,
    /// liv: (no comment)
    #[strum(serialize = "N_33a_laanga")]
    N_33a_laanga,
    /// liv: (no comment)
    #[strum(serialize = "N_34_siilma")]
    N_34_siilma,
    /// liv: (no comment)
    #[strum(serialize = "N_35_kaeaenga")]
    N_35_kaeaenga,
    /// liv: (no comment)
    #[strum(serialize = "N_36_aaina")]
    N_36_aaina,
    /// liv: (no comment)
    #[strum(serialize = "N_37_veena")]
    N_37_veena,
    /// liv: (no comment)
    #[strum(serialize = "N_38_rooda")]
    N_38_rooda,
    /// liv: (no comment)
    #[strum(serialize = "N_39_padaa")]
    N_39_padaa,
    /// liv: (no comment)
    #[strum(serialize = "N_40_sobraa")]
    N_40_sobraa,
    /// liv: (no comment)
    #[strum(serialize = "N_41_kaepaa")]
    N_41_kaepaa,
    /// liv: (no comment)
    #[strum(serialize = "N_42_maksaa")]
    N_42_maksaa,
    /// liv: (no comment)
    #[strum(serialize = "N_43_keera")]
    N_43_keera,
    /// liv: (no comment)
    #[strum(serialize = "N_44_joora")]
    N_44_joora,
    /// liv: (no comment)
    #[strum(serialize = "N_45_puuola")]
    N_45_puuola,
    /// liv: (no comment)
    #[strum(serialize = "N_46_aaita")]
    N_46_aaita,
    /// liv: (no comment)
    #[strum(serialize = "N_47_uushka")]
    N_47_uushka,
    /// liv: (no comment)
    #[strum(serialize = "N_48_mooka")]
    N_48_mooka,
    /// liv: (no comment)
    #[strum(serialize = "N_49_dadzhaa")]
    N_49_dadzhaa,
    /// liv: (no comment)
    #[strum(serialize = "N_50_sushaa")]
    N_50_sushaa,
    /// liv: (no comment)
    #[strum(serialize = "N_51_kupshaa")]
    N_51_kupshaa,
    /// liv: (no comment)
    #[strum(serialize = "N_52_tshuurtsha")]
    N_52_tshuurtsha,
    /// liv: (no comment)
    #[strum(serialize = "N_53_seemdja")]
    N_53_seemdja,
    /// liv: (no comment)
    #[strum(serialize = "N_54_tiiera")]
    N_54_tiiera,
    /// liv: (no comment)
    #[strum(serialize = "N_55_krooipa")]
    N_55_krooipa,
    /// liv: (no comment)
    #[strum(serialize = "N_56_lilla")]
    N_56_lilla,
    /// liv: (no comment)
    #[strum(serialize = "N_57_kutjaa")]
    N_57_kutjaa,
    /// liv: (no comment)
    #[strum(serialize = "N_58_neetsha")]
    N_58_neetsha,
    /// liv: (no comment)
    #[strum(serialize = "N_59_kiqv")]
    N_59_kiqv,
    /// liv: (no comment)
    #[strum(serialize = "N_60_kuoqig")]
    N_60_kuoqig,
    /// liv: (no comment)
    #[strum(serialize = "N_61_kep")]
    N_61_kep,
    /// liv: (no comment)
    #[strum(serialize = "N_62_krutjk")]
    N_62_krutjk,
    /// liv: (no comment)
    #[strum(serialize = "N_63_kraemp")]
    N_63_kraemp,
    /// liv: (no comment)
    #[strum(serialize = "N_64_piqnj")]
    N_64_piqnj,
    /// liv: (no comment)
    #[strum(serialize = "N_65_kuotj")]
    N_65_kuotj,
    /// liv: (no comment)
    #[strum(serialize = "N_66_vaktj")]
    N_66_vaktj,
    /// liv: (no comment)
    #[strum(serialize = "N_67_renj")]
    N_67_renj,
    /// liv: (no comment)
    #[strum(serialize = "N_68_oksh")]
    N_68_oksh,
    /// liv: (no comment)
    #[strum(serialize = "N_69_kash")]
    N_69_kash,
    /// liv: (no comment)
    #[strum(serialize = "N_70_dunjtsh")]
    N_70_dunjtsh,
    /// liv: (no comment)
    #[strum(serialize = "N_71_torii")]
    N_71_torii,
    /// liv: (no comment)
    #[strum(serialize = "N_72_vaqg")]
    N_72_vaqg,
    /// liv: (no comment)
    #[strum(serialize = "N_73_koql")]
    N_73_koql,
    /// liv: (no comment)
    #[strum(serialize = "N_74_suqg")]
    N_74_suqg,
    /// liv: (no comment)
    #[strum(serialize = "N_75_joqug")]
    N_75_joqug,
    /// liv: (no comment)
    #[strum(serialize = "N_76_niqm")]
    N_76_niqm,
    /// liv: (no comment)
    #[strum(serialize = "N_77_usk")]
    N_77_usk,
    /// liv: (no comment)
    #[strum(serialize = "N_78_vashk")]
    N_78_vashk,
    /// liv: (no comment)
    #[strum(serialize = "N_79_tup")]
    N_79_tup,
    /// liv: (no comment)
    #[strum(serialize = "N_80_maks")]
    N_80_maks,
    /// liv: (no comment)
    #[strum(serialize = "N_81_luqm")]
    N_81_luqm,
    /// liv: (no comment)
    #[strum(serialize = "N_82_mieqr")]
    N_82_mieqr,
    /// liv: (no comment)
    #[strum(serialize = "N_83_meqr")]
    N_83_meqr,
    /// liv: (no comment)
    #[strum(serialize = "N_84_muqr")]
    N_84_muqr,
    /// liv: (no comment)
    #[strum(serialize = "N_86_suqzh")]
    N_86_suqzh,
    /// liv: (no comment)
    #[strum(serialize = "N_87_keqzh")]
    N_87_keqzh,
    /// liv: (no comment)
    #[strum(serialize = "N_88_veqzh")]
    N_88_veqzh,
    /// liv: (no comment)
    #[strum(serialize = "N_89_kuqzh")]
    N_89_kuqzh,
    /// liv: (no comment)
    #[strum(serialize = "N_90_ruzuu")]
    N_90_ruzuu,
    /// liv: (no comment)
    #[strum(serialize = "N_91_radio")]
    N_91_radio,
    /// liv: (no comment)
    #[strum(serialize = "N_92_ang")]
    N_92_ang,
    /// liv: (no comment)
    #[strum(serialize = "N_93_kouv")]
    N_93_kouv,
    /// liv: (no comment)
    #[strum(serialize = "N_94_kand")]
    N_94_kand,
    /// liv: (no comment)
    #[strum(serialize = "N_95_port")]
    N_95_port,
    /// liv: (no comment)
    #[strum(serialize = "N_96_nurjm")]
    N_96_nurjm,
    /// liv: (no comment)
    #[strum(serialize = "N_97_laint")]
    N_97_laint,
    /// liv: (no comment)
    #[strum(serialize = "N_98_loul")]
    N_98_loul,
    /// liv: (no comment)
    #[strum(serialize = "N_99_ul")]
    N_99_ul,
    /// mhr: **@CODE@** = Necessive infinitive
    #[strum(serialize = "Nec")]
    Nec,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Negation verb ei
    /// liv: **@CODE@** = verb of negation эзь, аволь, иля
    /// mhr: **@CODE@** = Negation verb
    /// mhr: **@CODE@** = Negative participle
    /// sma: **@CODE@** | negation verb ij
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// sms: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// sme: **@CODE@** Negation Verb, Ii and its forms, ie Ale, Alli, Allot, Ehpet, Eat etc.
    /// fin: (no comment)
    /// vro: **@CODE@** saa-aiq 3 elements in 1 orthographic unit
    #[strum(serialize = "Neg")]
    Neg,
    /// mrj: **@CODE@** =  negative passive participle ЫдЫмЫ
    /// fin: (no comment)
    #[strum(serialize = "NegPrc")]
    NegPrc,
    /// sms: **@CODE@** No translation available
    #[strum(serialize = "NoTransl")]
    NoTransl,
    /// sma: **@CODE@**:  A tag to indicate the lack of realised or potential Umlaut
    #[strum(serialize = "NoUml")]
    NoUml,
    /// fit: **@CODE@** = Nominative
    /// kpv: **@CODE@** nominative case нимтан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Nominative
    /// liv: **@CODE@** = nominative case
    /// mhr: **@CODE@** = nominative
    /// sma: **@CODE@** = Nominative
    /// smj: **@CODE@** = Nominative case
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Nominative = nominativ
    /// lit: (no comment)
    /// vep: **@CODE@**  = nominative case
    /// olo: **@CODE@** nominative case
    /// sme: **@CODE@** - Nominative
    /// fin: (no comment)
    /// ces: (no comment)
    /// vro: **@CODE@** nominative
    #[strum(serialize = "Nom")]
    Nom,
    /// kpv: **@CODE@**
    /// liv: **@CODE@**
    #[strum(serialize = "NomAct")]
    NomAct,
    /// liv: **@CODE@**
    /// smj: (no comment)
    /// sms: **@CODE@** CHECK ME this is not the same as Der/NomAg, which is a derivation
    /// sme: **@CODE@** Actor Noun From Verb - Nomen Agentis, +N+NomAg
    #[strum(serialize = "NomAg")]
    NomAg,
    /// kpv: **@CODE@** look at this and place somewhere
    #[strum(serialize = "NonHum")]
    NonHum,
    /// fit: **@CODE@** = Numerals
    /// kpv: **@CODE@**:  numeral      лыдакыв   числительное
    /// vot: (no comment)
    /// fkv: **@CODE@** = Numerals
    /// liv: **@CODE@** = numeral
    /// mhr: **@CODE@** = numerals
    /// sma: **@CODE@** = Numerals
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Numeral
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Numeral
    /// lit: (no comment)
    /// vep: **@CODE@**  = numeral
    /// olo: **@CODE@**  numeral
    /// sme: **@CODE@** - Numeral
    /// fin: (no comment)
    /// ceb: **@CODE@** - Numeral
    /// vro: **@CODE@** Numerals
    #[strum(serialize = "Num")]
    Num,
    /// nob: **@CODE@**  For dictionary use., Nynorsk only
    #[strum(serialize = "Nynorsk")]
    Nynorsk,
    /// mhr: **@CODE@** =
    #[strum(serialize = "N→A")]
    N_A,
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | English
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - English
    /// vep: **@CODE@**
    /// sme: **@CODE@** = English
    /// fin: (no comment)
    #[strum(serialize = "OLang/ENG")]
    OLang_SLASH_ENG,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Finnish
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Finnish
    /// vep: **@CODE@**
    /// olo: **@CODE@** olang tags
    /// sme: **@CODE@** = Finnish
    /// fin: (no comment)
    #[strum(serialize = "OLang/FIN")]
    OLang_SLASH_FIN,
    /// sma: **@CODE@** | Hungarian
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Hungarian
    /// sme: **@CODE@** = Hungarian
    #[strum(serialize = "OLang/HUN")]
    OLang_SLASH_HUN,
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Norw. nynorsk
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Norw. nynorsk
    /// vep: **@CODE@**
    /// sme: **@CODE@** = Norw. nynorsk
    /// fin: (no comment)
    #[strum(serialize = "OLang/NNO")]
    OLang_SLASH_NNO,
    /// fit: **@CODE@** = language code for names from common name source
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Norw. bokmål
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Norw. bokmål
    /// vep: **@CODE@**
    /// olo: **@CODE@** olang tags
    /// sme: **@CODE@** = Norw. bokmål
    /// fin: (no comment)
    #[strum(serialize = "OLang/NOB")]
    OLang_SLASH_NOB,
    /// sma: **@CODE@** | parallelle navn, navnet skal ikke overføres til andre samisk språk
    /// smj: (no comment)
    /// sms: **@CODE@** - parallelle navn, navnet skal ikke overføres til andre samisk språk
    /// sme: **@CODE@** = parallelle navn, navnet skal ikke overføres til andre samisk språk
    #[strum(serialize = "OLang/PARA")]
    OLang_SLASH_PARA,
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Russian
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Russian
    /// vep: **@CODE@**
    /// sme: **@CODE@** = Russian
    /// fin: (no comment)
    #[strum(serialize = "OLang/RUS")]
    OLang_SLASH_RUS,
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | South Sámi
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - South Sámi
    /// vep: **@CODE@**
    /// sme: **@CODE@** = South Sámi
    /// fin: (no comment)
    #[strum(serialize = "OLang/SMA")]
    OLang_SLASH_SMA,
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | North Sámi
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - North Sámi
    /// vep: **@CODE@**
    /// sme: **@CODE@** = North Sámi
    /// fin: (no comment)
    #[strum(serialize = "OLang/SME")]
    OLang_SLASH_SME,
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Lule Sámi
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Lule Sámi
    /// sme: **@CODE@** = Lule Sámi
    #[strum(serialize = "OLang/SMJ")]
    OLang_SLASH_SMJ,
    /// sms: **@CODE@** - Inari Sámi
    #[strum(serialize = "OLang/SMN")]
    OLang_SLASH_SMN,
    /// smj: (no comment)
    /// sms: **@CODE@** - Skolt Sámi
    #[strum(serialize = "OLang/SMS")]
    OLang_SLASH_SMS,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Swedish
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Swedish
    /// vep: **@CODE@**
    /// olo: **@CODE@** olang tags
    /// sme: **@CODE@** = Swedish
    /// fin: (no comment)
    #[strum(serialize = "OLang/SWE")]
    OLang_SLASH_SWE,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Undefined
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Undefined
    /// vep: **@CODE@**
    /// olo: **@CODE@** olang tags
    /// sme: **@CODE@** = Undefined
    /// fin: (no comment)
    #[strum(serialize = "OLang/UND")]
    OLang_SLASH_UND,
    /// fin: (no comment)
    #[strum(serialize = "OLang/eng")]
    OLang_SLASH_eng,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Obj")]
    Obj,
    /// ceb: (no comment)
    #[strum(serialize = "Obj1")]
    Obj1,
    /// ceb: **@CODE@** - Oblique
    #[strum(serialize = "Obl")]
    Obl,
    /// yrk: **@CODE@**
    #[strum(serialize = "OcDu3")]
    OcDu3,
    /// yrk: **@CODE@**
    #[strum(serialize = "OcPl3")]
    OcPl3,
    /// yrk: **@CODE@**
    #[strum(serialize = "OcSg3")]
    OcSg3,
    /// yrk: **@CODE@**
    /// fin: (no comment)
    #[strum(serialize = "Opt")]
    Opt,
    /// fit: **@CODE@** = Ordinals
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Ordinal
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  Ordinal number
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// yrk: (no comment)
    /// sms: **@CODE@** -
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**   Ordinal Number
    /// fin: (no comment)
    /// ceb: **@CODE@** - Ordinal (etymologically these are numerals, but syntactically adjective might better suit them. I have even contemplated determiner, but no one else seems to. Jaska 2024-08-04
    /// vro: **@CODE@**
    #[strum(serialize = "Ord")]
    Ord,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Org")]
    Org,
    /// mrj: **@CODE@**
    #[strum(serialize = "Orth/1933")]
    Orth_SLASH_1933,
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    #[strum(serialize = "Orth/Colloq")]
    Orth_SLASH_Colloq,
    /// sme: (no comment)
    #[strum(serialize = "Orth/IPA")]
    Orth_SLASH_IPA,
    /// sme: (no comment)
    #[strum(serialize = "Orth/Strd")]
    Orth_SLASH_Strd,
    /// rus: (no comment)
    #[strum(serialize = "PObj")]
    PObj,
    /// fit: **@CODE@** = Punctuation mark
    /// vot: (no comment)
    /// fkv: **@CODE@** = Punctuation mark
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = other punctuation marks
    /// sma: **@CODE@**:  XXX These should be documented better
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = punctuation
    /// vep: **@CODE@**
    /// olo: **@CODE@**
    /// sme: **@CODE@**  punctuation
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "PUNCT")]
    PUNCT,
    /// fit: **@CODE@** = Partitive
    /// vot: (no comment)
    /// fkv: **@CODE@** = Partitive
    /// liv: **@CODE@** = partitive
    /// smj: **@CODE@** = Partitive case
    /// sms: **@CODE@** Partative
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// vep: **@CODE@**  = partitive
    /// olo: **@CODE@** partitive
    /// fin: (no comment)
    /// vro: **@CODE@** partitive
    #[strum(serialize = "Par")]
    Par,
    /// rus: (no comment)
    #[strum(serialize = "Paren")]
    Paren,
    /// kpv: **@CODE@** parenthetical phrase
    /// sms: **@CODE@**
    #[strum(serialize = "Parenthetic")]
    Parenthetic,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** = Passive
    /// mhr: **@CODE@** = Passive
    /// nob: **@CODE@** = for verb voice, mood, tense
    /// rus: (no comment)
    /// sms: **@CODE@** - hallat/haddat not in use
    /// sms: **@CODE@** Passive voice
    /// mrj: **@CODE@** =  passive participle ЫмЫ
    /// sme: **@CODE@** - hallat/haddat not in use
    #[strum(serialize = "Pass")]
    Pass,
    /// kpv: **@CODE@**
    #[strum(serialize = "PastPtc")]
    PastPtc,
    /// mhr: **@CODE@** = patronym, look at this in other cyr fsts.
    /// olo: (no comment)
    #[strum(serialize = "Patr")]
    Patr,
    /// vro: **@CODE@**
    #[strum(serialize = "Pc")]
    Pc,
    /// vro: **@CODE@**
    #[strum(serialize = "PcPl1")]
    PcPl1,
    /// vro: **@CODE@**
    #[strum(serialize = "PcPl2")]
    PcPl2,
    /// vro: **@CODE@**
    #[strum(serialize = "PcPl3")]
    PcPl3,
    /// vro: **@CODE@**
    #[strum(serialize = "PcSg1")]
    PcSg1,
    /// vro: **@CODE@**
    #[strum(serialize = "PcSg2")]
    PcSg2,
    /// vro: **@CODE@**
    #[strum(serialize = "PcSg3")]
    PcSg3,
    /// fit: **@CODE@** = Participle?
    /// kpv: **@CODE@**:  particle      кывтор   частица
    /// vot: (no comment)
    /// fkv: **@CODE@** = Participle?
    /// liv: **@CODE@** = particle
    /// mhr: **@CODE@** = particles
    /// sma: **@CODE@** = Particle
    /// smj: **@CODE@** = Particle
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Particle
    /// lit: (no comment)
    /// vep: **@CODE@**  = particle
    /// olo: **@CODE@**  particle
    /// sme: **@CODE@** - Particle
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Particle
    /// vro: **@CODE@**
    #[strum(serialize = "Pcle")]
    Pcle,
    /// fin: (no comment)
    #[strum(serialize = "Pe4")]
    Pe4,
    /// lit: (no comment)
    #[strum(serialize = "PerFreq")]
    PerFreq,
    /// fit: **@CODE@** = Personal
    /// kpv: **@CODE@**:  personal
    /// vot: (no comment)
    /// fkv: **@CODE@** = Personal
    /// liv: **@CODE@** = personal
    /// mhr: **@CODE@** = Personal pronoun
    /// sma: **@CODE@** = Personal
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Personal pronoun
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Personal Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  personal
    /// vep: **@CODE@**  = personal
    /// olo: **@CODE@** personal
    /// sme: **@CODE@** - Personal Pronoun
    /// fin: (no comment)
    /// ceb: **@CODE@** - Personal
    /// vro: **@CODE@**
    #[strum(serialize = "Pers")]
    Pers,
    /// fit: **@CODE@** = Plural
    /// kpv: **@CODE@**  plural
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural
    /// liv: **@CODE@** = plural
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** = Plural
    /// smj: **@CODE@** = Plural number
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Plural = määŋgailååkk
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// vep: **@CODE@**  = plural
    /// olo: **@CODE@** plural
    /// sme: **@CODE@** - Plural
    /// fin: (no comment)
    /// vro: **@CODE@** Plural
    #[strum(serialize = "Pl")]
    Pl,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural 1
    /// liv: **@CODE@** = first person plural conjugation
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** | Plural  , 1.person
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** first person plural
    /// lit: (no comment)
    /// vep: **@CODE@** Plural First Person
    /// sme: **@CODE@** Plural First Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pl1")]
    Pl1,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural 2
    /// liv: **@CODE@** = second person plural conjugation
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** | Plural  , 2.person
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** second person plural
    /// vep: **@CODE@** Plural Second Person
    /// sme: **@CODE@** Plural Second Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pl2")]
    Pl2,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural 3
    /// liv: **@CODE@** = third person plural conjugation
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** | Plural  , 3.person
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** third  person plural
    /// vep: **@CODE@** Plural Third Person
    /// sme: **@CODE@** Plural Third Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pl3")]
    Pl3,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Plant")]
    Plant,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Plc")]
    Plc,
    /// fit: **@CODE@** = Postposition
    /// kpv: **@CODE@**:  postposition   кывбӧр   послелог
    /// vot: (no comment)
    /// fkv: **@CODE@** = Postposition
    /// liv: **@CODE@** = postposition
    /// mhr: **@CODE@** = postpositions
    /// sma: **@CODE@** = Postposition
    /// smj: **@CODE@** = Postposition
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Postposition
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = postposition
    /// olo: **@CODE@**  postposition
    /// sme: **@CODE@** - Postpostion
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Postposition, postposed
    /// vro: **@CODE@** Postposition
    #[strum(serialize = "Po")]
    Po,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// nob: **@CODE@** = For adjectives
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mrj: **@CODE@** =  The comparative
    /// vep: **@CODE@**  =
    /// zxx: (no comment)
    /// olo: (no comment)
    #[strum(serialize = "Pos")]
    Pos,
    /// kpv: **@CODE@**:  possessive
    #[strum(serialize = "Poss")]
    Poss,
    /// fit: **@CODE@** = Potential
    /// vot: (no comment)
    /// fkv: **@CODE@** = Potential
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Potential mood
    /// lit: (no comment)
    /// vep: **@CODE@**  = potential linne-
    /// olo: **@CODE@** Potential
    /// sme: **@CODE@** Potential
    /// fin: (no comment)
    /// ceb: **@CODE@** - Potential
    /// vro: **@CODE@**
    #[strum(serialize = "Pot")]
    Pot,
    /// fit: **@CODE@** = Preposition
    /// kpv: **@CODE@**:  preposition   XX   предлог
    /// vot: (no comment)
    /// fkv: **@CODE@** = Preposition
    /// liv: **@CODE@** = preposition
    /// mhr: **@CODE@** = prepositons
    /// sma: **@CODE@** = Preposition
    /// smj: **@CODE@** = Preposition
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Preposition
    /// sms: (no comment)
    /// sms: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = preposition
    /// olo: **@CODE@**  preposition
    /// sme: **@CODE@** - Preposition
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Preposition
    /// vro: **@CODE@** Preposition
    #[strum(serialize = "Pr")]
    Pr,
    /// rus: (no comment)
    #[strum(serialize = "Prb")]
    Prb,
    /// liv: **@CODE@** = participle CHECK! how is this used ?
    /// mhr: **@CODE@** = Participle
    #[strum(serialize = "Prc")]
    Prc,
    /// yrk: **@CODE@**
    #[strum(serialize = "PrcFut")]
    PrcFut,
    /// yrk: **@CODE@**
    #[strum(serialize = "PrcImprf")]
    PrcImprf,
    /// yrk: **@CODE@**
    #[strum(serialize = "PrcNeg")]
    PrcNeg,
    /// yrk: **@CODE@**
    #[strum(serialize = "PrcPrf")]
    PrcPrf,
    /// rus: (no comment)
    #[strum(serialize = "Prcnt")]
    Prcnt,
    /// nob: **@CODE@**
    #[strum(serialize = "Prdt")]
    Prdt,
    /// rus: (no comment)
    /// yrk: **@CODE@** = predestinative
    /// sms: **@CODE@**
    #[strum(serialize = "Pred")]
    Pred,
    /// lit: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Pref")]
    Pref,
    /// fin: (no comment)
    #[strum(serialize = "Pref-")]
    Pref_MINUS_,
    /// liv: **@CODE@**
    /// sms: **@CODE@** = prefix
    /// sms: (no comment)
    #[strum(serialize = "Prefix")]
    Prefix,
    /// fit: **@CODE@**
    #[strum(serialize = "Prel")]
    Prel,
    /// kpv: **@CODE@**
    #[strum(serialize = "Presentational")]
    Presentational,
    /// mhr: **@CODE@** = perfective
    /// vep: **@CODE@**
    #[strum(serialize = "Prf")]
    Prf,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Preteritum Particip
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Infinitive and participles
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  = nu
    /// olo: **@CODE@** : past participle
    /// sme: **@CODE@** Perfect Participe
    /// fin: (no comment)
    /// vro: **@CODE@**
    /// vro: **@CODE@** elet
    #[strum(serialize = "PrfPrc")]
    PrfPrc,
    /// olo: **@CODE@**
    #[strum(serialize = "PrfPrcPl3")]
    PrfPrcPl3,
    /// vep: **@CODE@**
    #[strum(serialize = "PrfPrs")]
    PrfPrs,
    /// lit: (no comment)
    #[strum(serialize = "Priv")]
    Priv,
    /// kpv: **@CODE@** move to Abe
    #[strum(serialize = "PrivMod")]
    PrivMod,
    /// kpv: **@CODE@** prolative -ӧд вуджан
    /// liv: **@CODE@** = prolative
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Prolative
    /// vep: **@CODE@**  = prolative
    /// olo: **@CODE@** prolative
    /// fin: (no comment)
    #[strum(serialize = "Prl")]
    Prl,
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// fin: (no comment)
    #[strum(serialize = "Pro")]
    Pro,
    /// ceb: **@CODE@** -
    #[strum(serialize = "Prog")]
    Prog,
    /// fit: **@CODE@** = Pronomen
    /// kpv: **@CODE@**:  pronoun   нимвежтас   местоимение
    /// vot: (no comment)
    /// fkv: **@CODE@** = Pronomen
    /// liv: **@CODE@** = pronoun
    /// mhr: **@CODE@** = pronouns
    /// sma: **@CODE@** = Pronomen
    /// nob: **@CODE@** =
    /// smj: **@CODE@** = Pronouns
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**  = pronoun
    /// zxx: (no comment)
    /// olo: **@CODE@**  pronoun
    /// sme: **@CODE@** - Pronoun
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Pronoun
    /// vro: **@CODE@** Pronoun
    #[strum(serialize = "Pron")]
    Pron,
    /// fit: **@CODE@** = Propernoun
    /// kpv: **@CODE@** proper
    /// vot: (no comment)
    /// fkv: **@CODE@** = Propernoun
    /// liv: **@CODE@** = proper nouns
    /// mhr: **@CODE@** = Proper noun
    /// sma: **@CODE@** =
    /// nob: **@CODE@** = Propernouns are tagged +N+Prop
    /// smj: **@CODE@** = Propernouns
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Proper noun
    /// lit: (no comment)
    /// mrj: **@CODE@** is a noun subtag
    /// vep: **@CODE@**  = proper
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@** - Propernoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Prop")]
    Prop,
    /// kpv: **@CODE@** move to Prp
    #[strum(serialize = "ProprietiveMod")]
    ProprietiveMod,
    /// vro: **@CODE@**
    #[strum(serialize = "Prop→A")]
    Prop_A,
    /// yrk: **@CODE@** (Prosecutive)
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Pros")]
    Pros,
    /// kpv: **@CODE@** ProprietiveMod, HabObjMod Весиг киясыс тӧдсаӧсь, найӧ мугов рӧмаӧсь, кузь чорыд чуньясаӧсь.
    #[strum(serialize = "Prp")]
    Prp,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Presens
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = present
    /// sma: **@CODE@** | Presens
    /// smj: (no comment)
    /// sms: **@CODE@** Present
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** | Present, non-past Tense
    /// sme: **@CODE@** Present Tense
    /// fin: (no comment)
    /// ces: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Prs")]
    Prs,
    /// rus: (no comment)
    #[strum(serialize = "PrsAct")]
    PrsAct,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Presence Particip
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Infinitive and participles
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**
    /// olo: **@CODE@** : present participle
    /// sme: **@CODE@** Present Participe
    /// fin: (no comment)
    /// vro: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "PrsPrc")]
    PrsPrc,
    /// kpv: **@CODE@**
    #[strum(serialize = "PrsPtc")]
    PrsPtc,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Preteritum
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Preteritum
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Preterite
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** | Preterite Tense
    /// sme: **@CODE@** Past Tense, Preterite
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Prt")]
    Prt,
    /// mhr: **@CODE@** = 1st preterite, direct observation
    /// yrk: **@CODE@**
    #[strum(serialize = "Prt1")]
    Prt1,
    /// mhr: **@CODE@** = 2nd preterite, indirect narrative, conclusion
    /// yrk: **@CODE@**
    #[strum(serialize = "Prt2")]
    Prt2,
    /// liv: **@CODE@** = passive
    /// vep: **@CODE@**  = passive voice
    /// olo: **@CODE@** : passive voice
    /// fin: (no comment)
    /// vro: **@CODE@** passive
    #[strum(serialize = "Pss")]
    Pss,
    /// rus: (no comment)
    #[strum(serialize = "Pst")]
    Pst,
    /// rus: (no comment)
    #[strum(serialize = "PstAct")]
    PstAct,
    /// vep: **@CODE@**  =
    #[strum(serialize = "PstI")]
    PstI,
    /// vep: **@CODE@**  =
    #[strum(serialize = "PstII")]
    PstII,
    /// fin: (no comment)
    #[strum(serialize = "Punct")]
    Punct,
    /// kpv: **@CODE@**
    #[strum(serialize = "Px1")]
    Px1,
    /// fin: (no comment)
    #[strum(serialize = "Px3")]
    Px3,
    /// sme: **@CODE@** used in pronoun-sme-morph.txt
    #[strum(serialize = "PxCPlComRecipr")]
    PxCPlComRecipr,
    /// vot: (no comment)
    /// sma: **@CODE@** =  Possessives Dual
    /// smj: **@CODE@** possessive suffix dual first person
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**    Dual First Person
    #[strum(serialize = "PxDu1")]
    PxDu1,
    /// vot: (no comment)
    /// sma: **@CODE@** =  Possessives Dual
    /// smj: **@CODE@** possessive suffix dual second person
    /// yrk: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@**    Dual Second Person
    #[strum(serialize = "PxDu2")]
    PxDu2,
    /// vot: (no comment)
    /// sma: **@CODE@** =  Possessives Dual
    /// smj: **@CODE@** possessive suffix dual third person
    /// yrk: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@**    Dual Third Person
    #[strum(serialize = "PxDu3")]
    PxDu3,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Plural 1
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** =  Possessives Plural
    /// smj: **@CODE@** possessive suffix plural first person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  = -moi
    /// olo: (no comment)
    /// sme: **@CODE@**    Plural First Person
    /// fin: (no comment)
    #[strum(serialize = "PxPl1")]
    PxPl1,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Plural 2
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** =  Possessives Plural
    /// smj: **@CODE@** possessive suffix plural second person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// vep: **@CODE@**  = -toi
    /// olo: (no comment)
    /// sme: **@CODE@**    Plural Second Person
    /// fin: (no comment)
    #[strum(serialize = "PxPl2")]
    PxPl2,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Plural 3
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** =  Possessives Plural
    /// smj: **@CODE@** possessive suffix plural plural person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// vep: **@CODE@**  = -ze
    /// olo: (no comment)
    /// sme: **@CODE@**    Plural Third Person
    /// fin: (no comment)
    #[strum(serialize = "PxPl3")]
    PxPl3,
    /// olo: (no comment)
    #[strum(serialize = "PxSP3")]
    PxSP3,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Singular 1
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** =  Possessives Singular
    /// smj: **@CODE@** possessive suffix singular first person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: (no comment)
    /// vep: **@CODE@**  = -in
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**    Singular First Person
    /// fin: (no comment)
    #[strum(serialize = "PxSg1")]
    PxSg1,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Singular 2
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** =  Possessives Singular
    /// smj: **@CODE@** possessive suffix singular second person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// vep: **@CODE@**  =  -iž
    /// olo: (no comment)
    /// sme: **@CODE@**    Singular Second Person
    /// fin: (no comment)
    #[strum(serialize = "PxSg2")]
    PxSg2,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Singular 3
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** =  Possessives Singular
    /// smj: **@CODE@** possessive suffix singular third person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// vep: **@CODE@**  = -ze
    /// olo: (no comment)
    /// sme: **@CODE@**    Singular Third Person
    /// fin: (no comment)
    #[strum(serialize = "PxSg3")]
    PxSg3,
    /// vot: (no comment)
    #[strum(serialize = "QNT")]
    QNT,
    /// rus: (no comment)
    #[strum(serialize = "QUOT")]
    QUOT,
    /// kpv: **@CODE@**:  Quantifier   ХХ   XX
    /// vot: (no comment)
    /// liv: **@CODE@** = quantifier
    /// mhr: **@CODE@** = quantifiers
    /// nob: **@CODE@** = quantifier noen, begge
    /// yrk: (no comment)
    /// sms: **@CODE@** - SHOULD THIS be here or a Sem/Qnt
    /// vep: **@CODE@**  = quantifier
    /// olo: **@CODE@**  quantifier
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Qnt")]
    Qnt,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Focusclitic question -ko
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Question particle -a
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**   Question Particle: +Pcle+Qst
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Qst")]
    Qst,
    /// kpv: **@CODE@** a
    /// kpv: (no comment)
    #[strum(serialize = "Qst/A")]
    Qst_SLASH_A,
    /// kpv: **@CODE@** -ӧ
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Qst/Oe")]
    Qst_SLASH_Oe,
    /// sms: **@CODE@** Question particle -a
    #[strum(serialize = "Qst/a")]
    Qst_SLASH_a,
    /// sms: **@CODE@** -ǥo
    #[strum(serialize = "Qst/gho")]
    Qst_SLASH_gho,
    /// sms: **@CODE@** -go
    #[strum(serialize = "Qst/go")]
    Qst_SLASH_go,
    /// sms: **@CODE@** Question particle -ko might be used with Use/NG
    #[strum(serialize = "Qst/ko")]
    Qst_SLASH_ko,
    /// fit: **@CODE@** = Hmm, Question?? Interr? Check this.
    /// fkv: **@CODE@** = Quantity
    /// fin: (no comment)
    #[strum(serialize = "Qu")]
    Qu,
    /// liv: **@CODE@** = quotative, quoted speech
    /// vro: **@CODE@** = quotative, quoted speech
    #[strum(serialize = "Quo")]
    Quo,
    /// fin: (no comment)
    #[strum(serialize = "Quote")]
    Quote,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = right parenth
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = paired symbols
    /// sma: **@CODE@**:  XXX These should be documented better
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = right paranthesis
    /// vep: **@CODE@**  =
    /// olo: **@CODE@**
    /// sme: **@CODE@**  right paranthesis
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "RIGHT")]
    RIGHT,
    /// rus: (no comment)
    #[strum(serialize = "RPAR")]
    RPAR,
    /// rus: (no comment)
    #[strum(serialize = "RQUOT")]
    RQUOT,
    /// nob: **@CODE@**  For dictionary testing, Radical Bokmål
    #[strum(serialize = "Radical")]
    Radical,
    /// olo: (no comment)
    #[strum(serialize = "Rc")]
    Rc,
    /// vep: **@CODE@**
    /// olo: (no comment)
    #[strum(serialize = "RcPl")]
    RcPl,
    /// vep: **@CODE@**
    /// olo: (no comment)
    #[strum(serialize = "RcPl1")]
    RcPl1,
    /// vep: **@CODE@**
    #[strum(serialize = "RcPl2")]
    RcPl2,
    /// vep: **@CODE@**  =
    #[strum(serialize = "RcPl3")]
    RcPl3,
    /// olo: **@CODE@** : reflexive past participle
    #[strum(serialize = "RcPrfPrc")]
    RcPrfPrc,
    /// vep: **@CODE@**
    /// olo: (no comment)
    #[strum(serialize = "RcSg")]
    RcSg,
    /// vep: **@CODE@**
    /// olo: (no comment)
    #[strum(serialize = "RcSg1")]
    RcSg1,
    /// vep: **@CODE@**
    #[strum(serialize = "RcSg2")]
    RcSg2,
    /// vep: **@CODE@**
    #[strum(serialize = "RcSg3")]
    RcSg3,
    /// ceb: **@CODE@** -
    #[strum(serialize = "Rcmp")]
    Rcmp,
    /// ceb: (no comment)
    #[strum(serialize = "Rcp")]
    Rcp,
    /// rus: (no comment)
    #[strum(serialize = "Recip")]
    Recip,
    /// fit: **@CODE@** = Reciprocal
    /// kpv: **@CODE@**:  reciprocal
    /// vot: (no comment)
    /// fkv: **@CODE@** = Reciprocal
    /// liv: **@CODE@** = reciprocal
    /// mhr: **@CODE@** = Reciprocal pronoun
    /// sma: **@CODE@** = Reciprocal
    /// smj: **@CODE@** = reciprocal pronoun
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Reciprocal Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  reciprocal
    /// vep: **@CODE@**  = reciprocal
    /// olo: **@CODE@** reciprocal
    /// sme: **@CODE@** - Reciprocal Pronoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Recipr")]
    Recipr,
    /// fit: **@CODE@** = Reflexive
    /// kpv: **@CODE@**:  reflexive
    /// vot: (no comment)
    /// fkv: **@CODE@** = Reflexive
    /// liv: **@CODE@** = reflexive
    /// mhr: **@CODE@** = Reflexive pronoun
    /// sma: **@CODE@** = Reflexive
    /// smj: **@CODE@** = reflexive pronoun
    /// rus: (no comment)
    /// yrk: **@CODE@** reflexive
    /// sms: **@CODE@** - Reflexive Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  reflexive
    /// vep: **@CODE@**  = reflexive
    /// olo: **@CODE@** reflexive
    /// sme: **@CODE@** - Reflexive Pronoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Refl")]
    Refl,
    /// yrk: **@CODE@** referential adverbs
    /// yrk: **@CODE@** (referential),
    #[strum(serialize = "Refr")]
    Refr,
    /// fit: **@CODE@** = Relative
    /// kpv: **@CODE@**:  relative
    /// vot: (no comment)
    /// fkv: **@CODE@** = Relative
    /// liv: **@CODE@** = relative
    /// mhr: **@CODE@** = Relative pronoun
    /// sma: **@CODE@** = Relative
    /// smj: **@CODE@** = relative pronoun
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** - Relative Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  relative
    /// vep: **@CODE@**  = relative
    /// olo: **@CODE@** relative
    /// sme: **@CODE@** - Relative Pronoun
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Rel")]
    Rel,
    /// kpv: **@CODE@** relational noun: выв, ув
    #[strum(serialize = "Relat")]
    Relat,
    /// ceb: **@CODE@** - Remote deixis
    /// ceb: (no comment)
    #[strum(serialize = "Remt")]
    Remt,
    /// olo: **@CODE@** : This is a work around for olo passive. Olo has a passive conjugation,
    #[strum(serialize = "Rfl")]
    Rfl,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = Roman numerals I, II, ...
    /// mhr: **@CODE@** = roman numerals
    /// sma: **@CODE@** = Roman numeral
    /// nob: **@CODE@**
    /// smj: **@CODE@** = remertall
    /// sms: **@CODE@** - Roman numeral
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** - Roman numeral, subtag for +Num
    /// fin: (no comment)
    #[strum(serialize = "Rom")]
    Rom,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Route")]
    Route,
    /// ceb: (no comment)
    #[strum(serialize = "Rsn")]
    Rsn,
    /// yrk: **@CODE@** (100% Russian homograph)
    #[strum(serialize = "Rus")]
    Rus,
    /// rus: (no comment)
    #[strum(serialize = "SENT")]
    SENT,
    /// vro: **@CODE@** Singular and Plural, used for CG and Apertium
    #[strum(serialize = "SP")]
    SP,
    /// vot: (no comment)
    #[strum(serialize = "SPAT")]
    SPAT,
    /// vot: (no comment)
    #[strum(serialize = "STATE")]
    STATE,
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sc")]
    Sc,
    /// yrk: **@CODE@**
    #[strum(serialize = "ScDu1")]
    ScDu1,
    /// yrk: **@CODE@**
    #[strum(serialize = "ScDu2")]
    ScDu2,
    /// yrk: **@CODE@**
    #[strum(serialize = "ScDu3")]
    ScDu3,
    /// vep: **@CODE@**  =
    #[strum(serialize = "ScPl")]
    ScPl,
    /// yrk: **@CODE@**
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScPl1")]
    ScPl1,
    /// yrk: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "ScPl2")]
    ScPl2,
    /// yrk: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "ScPl3")]
    ScPl3,
    /// vep: **@CODE@**
    #[strum(serialize = "ScSg")]
    ScSg,
    /// yrk: **@CODE@**
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScSg1")]
    ScSg1,
    /// yrk: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "ScSg2")]
    ScSg2,
    /// yrk: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "ScSg3")]
    ScSg3,
    /// kpv: **@CODE@** Activity
    /// liv: **@CODE@** Activity
    /// mhr: **@CODE@** = Activity
    /// sma: (no comment)
    /// smj: **@CODE@** = Activity; cleaning, work, occupation, project, photosynthesis
    /// yrk: **@CODE@** Activity
    /// sms: **@CODE@** = Activity
    /// mrj: **@CODE@** Activity
    /// olo: **@CODE@** Activity
    /// sme: (no comment)
    /// vro: **@CODE@** Activity
    #[strum(serialize = "Sem/Act")]
    Sem_SLASH_Act,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Clth")]
    Sem_SLASH_Act_Clth,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Domain")]
    Sem_SLASH_Act_Domain,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Domain_Measr")]
    Sem_SLASH_Act_Domain_Measr,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Event")]
    Sem_SLASH_Act_Event,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Feat")]
    Sem_SLASH_Act_Feat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Feat-psych")]
    Sem_SLASH_Act_Feat_MINUS_psych,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Act_Fruit")]
    Sem_SLASH_Act_Fruit,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@** Activity and Group
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Act_Group")]
    Sem_SLASH_Act_Group,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Act_Hum")]
    Sem_SLASH_Act_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Hum_Obj")]
    Sem_SLASH_Act_Hum_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Money")]
    Sem_SLASH_Act_Money,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Obj")]
    Sem_SLASH_Act_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Obj-play")]
    Sem_SLASH_Act_Obj_MINUS_play,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Org")]
    Sem_SLASH_Act_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Perc-emo")]
    Sem_SLASH_Act_Perc_MINUS_emo,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** = A persons job is an activity, and a place as well
    /// yrk: (no comment)
    /// sms: **@CODE@**   A persons job is an activity, and a place as well
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Act_Plc")]
    Sem_SLASH_Act_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Prod-audio")]
    Sem_SLASH_Act_Prod_MINUS_audio,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Prod-vis")]
    Sem_SLASH_Act_Prod_MINUS_vis,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** = Activity and Route, ie johtolat
    /// yrk: (no comment)
    /// sms: **@CODE@** Activity and Route, ie johtolat
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Act_Route")]
    Sem_SLASH_Act_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Semcon")]
    Sem_SLASH_Act_Semcon,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_State")]
    Sem_SLASH_Act_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Time")]
    Sem_SLASH_Act_Time,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Act_Tool-it")]
    Sem_SLASH_Act_Tool_MINUS_it,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Txt")]
    Sem_SLASH_Act_Txt,
    /// sma: (no comment)
    /// smj: **@CODE@** = Webadr
    /// sms: **@CODE@** = Webadr
    /// sme: (no comment)
    #[strum(serialize = "Sem/Adr")]
    Sem_SLASH_Adr,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Amount
    /// fkv: (no comment)
    /// liv: **@CODE@** Amount
    /// mhr: **@CODE@** = Amount
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Amount; pile, amount of lynx, amount of food, one fifteenth, ten percent
    /// yrk: **@CODE@** Amount
    /// sms: **@CODE@** = Amount
    /// mrj: **@CODE@** Amount
    /// vep: **@CODE@**
    /// olo: **@CODE@** Amount
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Amount
    #[strum(serialize = "Sem/Amount")]
    Sem_SLASH_Amount,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**   Amount and Building
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: **@CODE@**   Amount and Building
    /// vro: (no comment)
    #[strum(serialize = "Sem/Amount_Build")]
    Sem_SLASH_Amount_Build,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Amount_Semcon")]
    Sem_SLASH_Amount_Semcon,
    /// fit: **@CODE@** = Animal names
    /// kpv: **@CODE@** Animate
    /// vot: (no comment)
    /// fkv: **@CODE@** = Animal names
    /// liv: **@CODE@** Animate
    /// mhr: **@CODE@** = Animate
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Animate; dog, reindeer, teddy bear,ndragon, lice
    /// yrk: **@CODE@** Animate
    /// sms: **@CODE@** Animate       (names)
    /// mrj: **@CODE@** Animate
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Animate
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Animate
    #[strum(serialize = "Sem/Ani")]
    Sem_SLASH_Ani,
    /// sma: (no comment)
    /// smj: **@CODE@** = Fish
    /// sms: **@CODE@** Animate
    /// sme: (no comment)
    #[strum(serialize = "Sem/Ani-fish")]
    Sem_SLASH_Ani_MINUS_fish,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Body")]
    Sem_SLASH_Ani_Body,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Body-abstr_Hum")]
    Sem_SLASH_Ani_Body_MINUS_abstr_Hum,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Build")]
    Sem_SLASH_Ani_Build,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Build-part")]
    Sem_SLASH_Ani_Build_MINUS_part,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Ani_Build_Hum_Obj-clo_Txt")]
    Sem_SLASH_Ani_Build_Hum_Obj_MINUS_clo_Txt,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Build_Hum_Txt")]
    Sem_SLASH_Ani_Build_Hum_Txt,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Ani_Buildpart")]
    Sem_SLASH_Ani_Buildpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Cat")]
    Sem_SLASH_Ani_Cat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Clth")]
    Sem_SLASH_Ani_Clth,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Feat_Hum")]
    Sem_SLASH_Ani_Feat_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Feat_Plant")]
    Sem_SLASH_Ani_Feat_Plant,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Food")]
    Sem_SLASH_Ani_Food,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Group")]
    Sem_SLASH_Ani_Group,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Group_Hum")]
    Sem_SLASH_Ani_Group_Hum,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Ani_Group_Prod-vis")]
    Sem_SLASH_Ani_Group_Prod_MINUS_vis,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Hum")]
    Sem_SLASH_Ani_Hum,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Hum_Plc")]
    Sem_SLASH_Ani_Hum_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Hum_Time")]
    Sem_SLASH_Ani_Hum_Time,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Obj")]
    Sem_SLASH_Ani_Obj,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Ani_Obj-el")]
    Sem_SLASH_Ani_Obj_MINUS_el,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Org")]
    Sem_SLASH_Ani_Org,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Plc")]
    Sem_SLASH_Ani_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Plc_Txt")]
    Sem_SLASH_Ani_Plc_Txt,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_State")]
    Sem_SLASH_Ani_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Substnc")]
    Sem_SLASH_Ani_Substnc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Time")]
    Sem_SLASH_Ani_Time,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Ani_Tool")]
    Sem_SLASH_Ani_Tool,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Veh")]
    Sem_SLASH_Ani_Veh,
    /// kpv: **@CODE@** Animal Product
    /// liv: **@CODE@** Animal Product
    /// mhr: **@CODE@** = Animal Product
    /// sma: (no comment)
    /// smj: **@CODE@** = Animal Product; sweat, reindeer skin, lice egg, blood for making dumplings, pee
    /// yrk: **@CODE@** Animal Product
    /// sms: **@CODE@** = Animal Product
    /// mrj: **@CODE@** Animal Product
    /// olo: **@CODE@** Animal Product
    /// sme: (no comment)
    /// vro: **@CODE@** Animal Product
    #[strum(serialize = "Sem/Aniprod")]
    Sem_SLASH_Aniprod,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Hum")]
    Sem_SLASH_Aniprod_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Mat")]
    Sem_SLASH_Aniprod_Mat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Obj")]
    Sem_SLASH_Aniprod_Obj,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Obj-clo")]
    Sem_SLASH_Aniprod_Obj_MINUS_clo,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Perc-phys")]
    Sem_SLASH_Aniprod_Perc_MINUS_phys,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Plant")]
    Sem_SLASH_Aniprod_Plant,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Plc")]
    Sem_SLASH_Aniprod_Plc,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Aniprod_Plc_Route")]
    Sem_SLASH_Aniprod_Plc_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Substnc")]
    Sem_SLASH_Aniprod_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Substnc_Wthr")]
    Sem_SLASH_Aniprod_Substnc_Wthr,
    /// kpv: **@CODE@**  Anthroponym
    /// rus: (no comment)
    /// sms: **@CODE@** Anthroponym  (names)
    /// olo: (no comment)
    #[strum(serialize = "Sem/Ant")]
    Sem_SLASH_Ant,
    /// kpv: **@CODE@**  Anthroponym female
    /// olo: (no comment)
    #[strum(serialize = "Sem/Ant-Fem")]
    Sem_SLASH_Ant_MINUS_Fem,
    /// kpv: **@CODE@**  Anthroponym male
    /// olo: (no comment)
    #[strum(serialize = "Sem/Ant-Mal")]
    Sem_SLASH_Ant_MINUS_Mal,
    /// kpv: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Ant_Fem")]
    Sem_SLASH_Ant_Fem,
    /// kpv: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Ant_Mal")]
    Sem_SLASH_Ant_Mal,
    /// nob: **@CODE@**
    #[strum(serialize = "Sem/Atr")]
    Sem_SLASH_Atr,
    /// kpv: **@CODE@** Bodypart
    /// liv: **@CODE@** Bodypart
    /// mhr: **@CODE@** = Bodypart
    /// sma: (no comment)
    /// smj: **@CODE@** = Bodypart; ear, bone, ear canal, artificial leg, mustache, nervous system
    /// yrk: **@CODE@** Bodypart
    /// sms: **@CODE@** = Bodypart
    /// mrj: **@CODE@** Bodypart
    /// olo: **@CODE@** Bodypart
    /// sme: (no comment)
    /// vro: **@CODE@** Bodypart
    #[strum(serialize = "Sem/Body")]
    Sem_SLASH_Body,
    /// kpv: **@CODE@** siellu, vuoig?a, jierbmi
    /// liv: **@CODE@** siellu, vuoig?a, jierbmi
    /// mhr: **@CODE@** = siellu, vuoig?a, jierbmi
    /// sma: (no comment)
    /// smj: **@CODE@** = Non-physical body part; Sjel, ånd, reason, soul, voice, eyesight, conscience
    /// yrk: **@CODE@** siellu, vuoig?a, jierbmi
    /// sms: **@CODE@** = siellu, vuoig?a, jierbmi
    /// mrj: **@CODE@** siellu, vuoig?a, jierbmi
    /// olo: **@CODE@** siellu, vuoig?a, jierbmi
    /// sme: (no comment)
    /// vro: **@CODE@** siellu, vuoig?a, jierbmi
    #[strum(serialize = "Sem/Body-abstr")]
    Sem_SLASH_Body_MINUS_abstr,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Body-abstr_Feat-cogn")]
    Sem_SLASH_Body_MINUS_abstr_Feat_MINUS_cogn,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Body-abstr_Feat-psych")]
    Sem_SLASH_Body_MINUS_abstr_Feat_MINUS_psych,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body-abstr_Prod-audio_Semcon")]
    Sem_SLASH_Body_MINUS_abstr_Prod_MINUS_audio_Semcon,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Body-abstr")]
    Sem_SLASH_Body_Body_MINUS_abstr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Buildpart")]
    Sem_SLASH_Body_Buildpart,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Clth")]
    Sem_SLASH_Body_Clth,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Clthpart")]
    Sem_SLASH_Body_Clthpart,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Food")]
    Sem_SLASH_Body_Food,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Fruit")]
    Sem_SLASH_Body_Fruit,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Group_Hum")]
    Sem_SLASH_Body_Group_Hum,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Body_Group_Hum_Time")]
    Sem_SLASH_Body_Group_Hum_Time,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Hum")]
    Sem_SLASH_Body_Hum,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Mat")]
    Sem_SLASH_Body_Mat,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Measr")]
    Sem_SLASH_Body_Measr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Obj")]
    Sem_SLASH_Body_Obj,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Obj_Tool-catch")]
    Sem_SLASH_Body_Obj_Tool_MINUS_catch,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Org")]
    Sem_SLASH_Body_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Part")]
    Sem_SLASH_Body_Part,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Body_Plant")]
    Sem_SLASH_Body_Plant,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Plc")]
    Sem_SLASH_Body_Plc,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Body_Plc-elevate")]
    Sem_SLASH_Body_Plc_MINUS_elevate,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Plc_State")]
    Sem_SLASH_Body_Plc_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_State")]
    Sem_SLASH_Body_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Time")]
    Sem_SLASH_Body_Time,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Building
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Building
    /// mhr: **@CODE@** = Building
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Building; house, museum, Sámi tent, nest, sandcastle
    /// yrk: **@CODE@** Building
    /// sms: **@CODE@** = Building
    /// mrj: **@CODE@** Building
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Building
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Building
    #[strum(serialize = "Sem/Build")]
    Sem_SLASH_Build,
    /// kpv: **@CODE@** Part of Bulding, like the closet
    /// liv: **@CODE@** Part of Bulding, like the closet
    /// mhr: **@CODE@** = Part of Bulding, like the closet
    /// yrk: **@CODE@** Part of Bulding, like the closet
    /// sms: **@CODE@** = Part of Bulding, like the closet
    /// mrj: **@CODE@** Part of Bulding, like the closet
    /// olo: **@CODE@** Part of Bulding, like the closet
    /// vro: **@CODE@** Part of Bulding, like the closet
    #[strum(serialize = "Sem/Build-part")]
    Sem_SLASH_Build_MINUS_part,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Build-part_Cat")]
    Sem_SLASH_Build_MINUS_part_Cat,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Build-part_Cat_Ctain")]
    Sem_SLASH_Build_MINUS_part_Cat_Ctain,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Build-part_Cat_Ctain_Mat")]
    Sem_SLASH_Build_MINUS_part_Cat_Ctain_Mat,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Build-part_Ctain")]
    Sem_SLASH_Build_MINUS_part_Ctain,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Build-part_Ctain_Mat")]
    Sem_SLASH_Build_MINUS_part_Ctain_Mat,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Build-part_Ctain_Obj")]
    Sem_SLASH_Build_MINUS_part_Ctain_Obj,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build-part_Plc")]
    Sem_SLASH_Build_MINUS_part_Plc,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Room in a building, typically place to be
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Build-room")]
    Sem_SLASH_Build_MINUS_room,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Build-room_Cat_Ctain_Mat")]
    Sem_SLASH_Build_MINUS_room_Cat_Ctain_Mat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build-room_Furn")]
    Sem_SLASH_Build_MINUS_room_Furn,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Build-room_Org")]
    Sem_SLASH_Build_MINUS_room_Org,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Build-part")]
    Sem_SLASH_Build_Build_MINUS_part,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build_Build-room")]
    Sem_SLASH_Build_Build_MINUS_room,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build_Buildpart")]
    Sem_SLASH_Build_Buildpart,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Clth-part")]
    Sem_SLASH_Build_Clth_MINUS_part,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Build_Clthpart")]
    Sem_SLASH_Build_Clthpart,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Edu_Org")]
    Sem_SLASH_Build_Edu_Org,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Event_Org")]
    Sem_SLASH_Build_Event_Org,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Build_Obj")]
    Sem_SLASH_Build_Obj,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Org")]
    Sem_SLASH_Build_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build_Plc")]
    Sem_SLASH_Build_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Route")]
    Sem_SLASH_Build_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build_Tool")]
    Sem_SLASH_Build_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build_Veh")]
    Sem_SLASH_Build_Veh,
    /// sma: (no comment)
    /// smj: **@CODE@** = Part of building; closet, room, door, balcony, pool, office
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart")]
    Sem_SLASH_Buildpart,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Cat")]
    Sem_SLASH_Buildpart_Cat,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Cat_Ctain")]
    Sem_SLASH_Buildpart_Cat_Ctain,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Cat_Ctain_Mat")]
    Sem_SLASH_Buildpart_Cat_Ctain_Mat,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Ctain")]
    Sem_SLASH_Buildpart_Ctain,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Ctain_Mat")]
    Sem_SLASH_Buildpart_Ctain_Mat,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Ctain_Obj")]
    Sem_SLASH_Buildpart_Ctain_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Buildpart_Obj")]
    Sem_SLASH_Buildpart_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Buildpart_Obj_Plc")]
    Sem_SLASH_Buildpart_Obj_Plc,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Org")]
    Sem_SLASH_Buildpart_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Buildpart_Part")]
    Sem_SLASH_Buildpart_Part,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Buildpart_Plc")]
    Sem_SLASH_Buildpart_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Buildpart_Prod-audio")]
    Sem_SLASH_Buildpart_Prod_MINUS_audio,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Category
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Category
    /// mhr: **@CODE@** = Category
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Category; name, subjunction, suffix, password, existential sentence
    /// yrk: **@CODE@** Category
    /// sms: **@CODE@** = Category
    /// mrj: **@CODE@** Category
    /// vep: **@CODE@**
    /// olo: **@CODE@** Category
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Category
    #[strum(serialize = "Sem/Cat")]
    Sem_SLASH_Cat,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Cat_Edu")]
    Sem_SLASH_Cat_Edu,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Feat")]
    Sem_SLASH_Cat_Feat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Feat-psych_Plc")]
    Sem_SLASH_Cat_Feat_MINUS_psych_Plc,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Cat_Group_Hum")]
    Sem_SLASH_Cat_Group_Hum,
    /// sma: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Cat_Group_Hum_Plc")]
    Sem_SLASH_Cat_Group_Hum_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Hum")]
    Sem_SLASH_Cat_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Measr")]
    Sem_SLASH_Cat_Measr,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Cat_Obj")]
    Sem_SLASH_Cat_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Plantpart")]
    Sem_SLASH_Cat_Plantpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Plantpart_Semcon")]
    Sem_SLASH_Cat_Plantpart_Semcon,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Cat_Txt")]
    Sem_SLASH_Cat_Txt,
    /// kpv: **@CODE@** Clothes
    /// vot: (no comment)
    /// liv: **@CODE@** Clothes
    /// mhr: **@CODE@** = Clothes
    /// sma: (no comment)
    /// smj: **@CODE@** = Clothes/Clothing; shirt, hat, theater costume, shawl, seatbelt, diaper
    /// yrk: **@CODE@** Clothes
    /// sms: **@CODE@** = Clothes
    /// mrj: **@CODE@** Clothes
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Clothes
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Clothes
    #[strum(serialize = "Sem/Clth")]
    Sem_SLASH_Clth,
    /// kpv: **@CODE@** Jewelery
    /// liv: **@CODE@** Jewelery
    /// mhr: **@CODE@** = Jewelery
    /// sma: (no comment)
    /// smj: **@CODE@** = Jewelery and similar; watch, sunglasses, ring, necklace, crown
    /// yrk: **@CODE@** Jewelery
    /// sms: **@CODE@** = Jewelery
    /// mrj: **@CODE@** Jewelery
    /// olo: **@CODE@** Jewelery
    /// sme: (no comment)
    /// vro: **@CODE@** Jewelery
    #[strum(serialize = "Sem/Clth-jewl")]
    Sem_SLASH_Clth_MINUS_jewl,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Curr")]
    Sem_SLASH_Clth_MINUS_jewl_Curr,
    /// sma: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Curr_Obj")]
    Sem_SLASH_Clth_MINUS_jewl_Curr_Obj,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Curr_Obj_Org")]
    Sem_SLASH_Clth_MINUS_jewl_Curr_Obj_Org,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Fruit")]
    Sem_SLASH_Clth_MINUS_jewl_Fruit,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Money")]
    Sem_SLASH_Clth_MINUS_jewl_Money,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clth-jewl_Obj")]
    Sem_SLASH_Clth_MINUS_jewl_Obj,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Org")]
    Sem_SLASH_Clth_MINUS_jewl_Org,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Plant")]
    Sem_SLASH_Clth_MINUS_jewl_Plant,
    /// kpv: **@CODE@** part of clothes, boallu, sávdnji...
    /// liv: **@CODE@** part of clothes, boallu, sávdnji...
    /// mhr: **@CODE@** = part of clothes, boallu, sávdnji...
    /// yrk: **@CODE@** part of clothes, boallu, sávdnji...
    /// sms: **@CODE@** = part of clothes, boallu, sávdnji...
    /// mrj: **@CODE@** part of clothes, boallu, sávdnji...
    /// olo: **@CODE@** part of clothes, boallu, sávdnji...
    /// vro: **@CODE@** part of clothes, boallu, sávdnji...
    #[strum(serialize = "Sem/Clth-part")]
    Sem_SLASH_Clth_MINUS_part,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth_Hum")]
    Sem_SLASH_Clth_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clth_Obj")]
    Sem_SLASH_Clth_Obj,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Clth_Obj-clo")]
    Sem_SLASH_Clth_Obj_MINUS_clo,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clth_Part")]
    Sem_SLASH_Clth_Part,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clth_Sur")]
    Sem_SLASH_Clth_Sur,
    /// sma: (no comment)
    /// smj: **@CODE@** = Part of clothes; knapp, søm, pocket (OBS! Ctain-abstr), bottom part of an article of clothing, upper part of trousers, seam, button
    /// sme: (no comment)
    #[strum(serialize = "Sem/Clthpart")]
    Sem_SLASH_Clthpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clthpart_Plc")]
    Sem_SLASH_Clthpart_Plc,
    /// olo: (no comment)
    #[strum(serialize = "Sem/Color")]
    Sem_SLASH_Color,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Constellation")]
    Sem_SLASH_Constellation,
    /// kpv: **@CODE@** Container
    /// liv: **@CODE@** Container
    /// mhr: **@CODE@** = Container
    /// sma: (no comment)
    /// smj: **@CODE@** = Container; suitcase, terrarium, closet, container, gas tank
    /// yrk: **@CODE@** Container
    /// sms: **@CODE@** = Container
    /// mrj: **@CODE@** Container
    /// olo: **@CODE@** Container
    /// sme: (no comment)
    /// vro: **@CODE@** Container
    #[strum(serialize = "Sem/Ctain")]
    Sem_SLASH_Ctain,
    /// kpv: **@CODE@** Abstract container like bank account
    /// liv: **@CODE@** Abstract container like bank account
    /// mhr: **@CODE@** = Abstract container like bank account
    /// sma: (no comment)
    /// smj: **@CODE@** = Abstract container; bank account, fund, account, loan fund, pot, bank account
    /// yrk: **@CODE@** Abstract container like bank account
    /// sms: **@CODE@** = Abstract container like bank account
    /// mrj: **@CODE@** Abstract container like bank account
    /// olo: **@CODE@** Abstract container like bank account
    /// sme: (no comment)
    /// vro: **@CODE@** Abstract container like bank account
    #[strum(serialize = "Sem/Ctain-abstr")]
    Sem_SLASH_Ctain_MINUS_abstr,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain-abstr_Org")]
    Sem_SLASH_Ctain_MINUS_abstr_Org,
    /// kpv: **@CODE@**
    /// liv: **@CODE@**
    /// mhr: **@CODE@**
    /// sma: (no comment)
    /// smj: **@CODE@** = Eks; lomme/pocket
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Soft container, like a rucksack
    /// mrj: **@CODE@**
    /// olo: **@CODE@**
    /// sme: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Ctain-clth")]
    Sem_SLASH_Ctain_MINUS_clth,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain-clth_Plant")]
    Sem_SLASH_Ctain_MINUS_clth_Plant,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain-clth_Veh")]
    Sem_SLASH_Ctain_MINUS_clth_Veh,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Feat-phys")]
    Sem_SLASH_Ctain_Feat_MINUS_phys,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Furn")]
    Sem_SLASH_Ctain_Furn,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ctain_Obj-surfc")]
    Sem_SLASH_Ctain_Obj_MINUS_surfc,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Ctain_Plc")]
    Sem_SLASH_Ctain_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Tool")]
    Sem_SLASH_Ctain_Tool,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Tool-measr")]
    Sem_SLASH_Ctain_Tool_MINUS_measr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ctain_Txt")]
    Sem_SLASH_Ctain_Txt,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Currency like dollár, Not Money
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Currency like dollár, Not Money
    /// mhr: **@CODE@** = Currency like dollár, Not Money
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Currency; not money, euro, US dollar, denar, Danish crown, currency
    /// yrk: **@CODE@** Currency like dollár, Not Money
    /// sms: **@CODE@** = Currency like dollár, Not Money
    /// mrj: **@CODE@** Currency like dollár, Not Money
    /// vep: **@CODE@**
    /// olo: **@CODE@** Currency like dollár, Not Money
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Currency like dollár, Not Money
    #[strum(serialize = "Sem/Curr")]
    Sem_SLASH_Curr,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Curr_Obj")]
    Sem_SLASH_Curr_Obj,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Curr_Org")]
    Sem_SLASH_Curr_Org,
    /// kpv: **@CODE@** Dance
    /// liv: **@CODE@** Dance
    /// mhr: **@CODE@** = Dance
    /// sma: (no comment)
    /// smj: **@CODE@** = Dance; swing, rumba, ballet, belly dance, war dance
    /// yrk: **@CODE@** Dance
    /// sms: **@CODE@** = Dance
    /// mrj: **@CODE@** Dance
    /// olo: **@CODE@** Dance
    /// sme: (no comment)
    /// vro: **@CODE@** Dance
    #[strum(serialize = "Sem/Dance")]
    Sem_SLASH_Dance,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Dance_Org")]
    Sem_SLASH_Dance_Org,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Dance_Prod-audio")]
    Sem_SLASH_Dance_Prod_MINUS_audio,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Date
    /// sms: **@CODE@** = Number as date
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Date")]
    Sem_SLASH_Date,
    /// kpv: **@CODE@** Direction like GPS-kursa
    /// liv: **@CODE@** Direction like GPS-kursa
    /// mhr: **@CODE@** = Direction like GPS-kursa
    /// sma: (no comment)
    /// smj: **@CODE@** = Direction; GPS course, stock exchange price, graph, tendency, starboard
    /// yrk: **@CODE@** Direction like GPS-kursa
    /// sms: **@CODE@** = Direction like GPS-kursa
    /// mrj: **@CODE@** Direction like GPS-kursa
    /// olo: **@CODE@** Direction like GPS-kursa
    /// sme: (no comment)
    /// vro: **@CODE@** Direction like GPS-kursa
    #[strum(serialize = "Sem/Dir")]
    Sem_SLASH_Dir,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Dir_Geom")]
    Sem_SLASH_Dir_Geom,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// mhr: **@CODE@** = Domain like politics, reindeerherding (a system of actions)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Domain like politics, reindeerherding (a system of actions), anthropology, punk rock, biology, linguistics, medicine
    /// yrk: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// sms: **@CODE@** = Domain like politics, reindeerherding (a system of actions)
    /// mrj: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// vep: **@CODE@**
    /// olo: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    #[strum(serialize = "Sem/Domain")]
    Sem_SLASH_Domain,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Edu")]
    Sem_SLASH_Domain_Edu,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Feat")]
    Sem_SLASH_Domain_Feat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Feat-phys")]
    Sem_SLASH_Domain_Feat_MINUS_phys,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Domain_Food-med")]
    Sem_SLASH_Domain_Food_MINUS_med,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Domain_Hum")]
    Sem_SLASH_Domain_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Ideol")]
    Sem_SLASH_Domain_Ideol,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Domain_Money")]
    Sem_SLASH_Domain_Money,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Obj")]
    Sem_SLASH_Domain_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Org")]
    Sem_SLASH_Domain_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Org_Plc-abstr")]
    Sem_SLASH_Domain_Org_Plc_MINUS_abstr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Perc-emo")]
    Sem_SLASH_Domain_Perc_MINUS_emo,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Domain_Prod-audio")]
    Sem_SLASH_Domain_Prod_MINUS_audio,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_State-sick")]
    Sem_SLASH_Domain_State_MINUS_sick,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Domain_Txt")]
    Sem_SLASH_Domain_Txt,
    /// kpv: **@CODE@** Drink
    /// liv: **@CODE@** Drink
    /// mhr: **@CODE@** = Drink
    /// sma: (no comment)
    /// smj: **@CODE@** = Drink; tea, beer, UHT milk, soda, communion wine
    /// yrk: **@CODE@** Drink
    /// sms: **@CODE@** = Drink
    /// mrj: **@CODE@** Drink
    /// olo: **@CODE@** Drink
    /// sme: (no comment)
    /// vro: **@CODE@** Drink
    #[strum(serialize = "Sem/Drink")]
    Sem_SLASH_Drink,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Drink_Plant")]
    Sem_SLASH_Drink_Plant,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Drink_Plc")]
    Sem_SLASH_Drink_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Drink_Plc_Substnc")]
    Sem_SLASH_Drink_Plc_Substnc,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Dummytag
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Dummytag
    /// mhr: **@CODE@** = Dummytag
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Default tag for uncategorized nouns
    /// yrk: **@CODE@** Dummytag
    /// sms: **@CODE@** = Dummytag
    /// mrj: **@CODE@** Dummytag
    /// vep: **@CODE@**
    /// olo: **@CODE@** Dummytag
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Dummytag
    #[strum(serialize = "Sem/Dummytag")]
    Sem_SLASH_Dummytag,
    /// fit: **@CODE@** = Education institution
    /// kpv: **@CODE@** Educational event
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Educational event
    /// mhr: **@CODE@** = Educational event
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Educational event; skiing high school skiing academy, course, music lesson, lesson, master
    /// yrk: **@CODE@** Educational event
    /// sms: **@CODE@** = Educational event
    /// mrj: **@CODE@** Educational event
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Educational event
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Educational event
    #[strum(serialize = "Sem/Edu")]
    Sem_SLASH_Edu,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Event")]
    Sem_SLASH_Edu_Event,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Edu_Geom")]
    Sem_SLASH_Edu_Geom,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Edu_Geom_Plc-line")]
    Sem_SLASH_Edu_Geom_Plc_MINUS_line,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Group_Hum")]
    Sem_SLASH_Edu_Group_Hum,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Edu_Hum")]
    Sem_SLASH_Edu_Hum,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Mat")]
    Sem_SLASH_Edu_Mat,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Org")]
    Sem_SLASH_Edu_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Edu_Txt")]
    Sem_SLASH_Edu_Txt,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Event
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Event
    /// mhr: **@CODE@** = Event
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Event; wedding, meeting, competition, election, festival
    /// yrk: **@CODE@** Event
    /// sms: **@CODE@** = Event
    /// mrj: **@CODE@** Event
    /// vep: **@CODE@**
    /// olo: **@CODE@** Event
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Event
    #[strum(serialize = "Sem/Event")]
    Sem_SLASH_Event,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Food")]
    Sem_SLASH_Event_Food,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Hum")]
    Sem_SLASH_Event_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Event_Org")]
    Sem_SLASH_Event_Org,
    /// kpv: **@CODE@**  сёянін
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Plc")]
    Sem_SLASH_Event_Plc,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Event_Plc-elevate")]
    Sem_SLASH_Event_Plc_MINUS_elevate,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Time")]
    Sem_SLASH_Event_Time,
    /// kpv: **@CODE@** Feature, like Árvu
    /// liv: **@CODE@** Feature, like Árvu
    /// mhr: **@CODE@** = Feature, like Árvu
    /// sma: (no comment)
    /// smj: **@CODE@** = Feature; Árvu, age difference, homosexuality, femininity, identity, congruence
    /// yrk: **@CODE@** Feature, like Árvu
    /// sms: **@CODE@** = Feature, like Árvu
    /// mrj: **@CODE@** Feature, like Árvu
    /// olo: **@CODE@** Feature, like Árvu
    /// sme: (no comment)
    /// vro: **@CODE@** Feature, like Árvu
    #[strum(serialize = "Sem/Feat")]
    Sem_SLASH_Feat,
    /// kpv: **@CODE@** Psychological feauture
    /// liv: **@CODE@** Psychological feauture
    /// mhr: **@CODE@** = Psychological feauture
    /// sma: (no comment)
    /// smj: **@CODE@** = Measurable feature; radius, diameter, volume, circumference, perimeter, frequency
    /// yrk: **@CODE@** Psychological feauture
    /// sms: **@CODE@** = Psychological feauture
    /// mrj: **@CODE@** Psychological feauture
    /// olo: **@CODE@** Psychological feauture
    /// sme: (no comment)
    /// vro: **@CODE@** Psychological feauture
    #[strum(serialize = "Sem/Feat-measr")]
    Sem_SLASH_Feat_MINUS_measr,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Feat-measr_Plc")]
    Sem_SLASH_Feat_MINUS_measr_Plc,
    /// kpv: **@CODE@** Physiological feature, ivdni, fárda
    /// liv: **@CODE@** Physiological feature, ivdni, fárda
    /// mhr: **@CODE@** = Physiological feature, ivdni, fárda
    /// sma: (no comment)
    /// smj: **@CODE@** = Physiological feature; size, color, height, shape, weight, horsepower
    /// yrk: **@CODE@** Physiological feature, ivdni, fárda
    /// sms: **@CODE@** = Physiological feature, ivdni, fárda
    /// mrj: **@CODE@** Physiological feature, ivdni, fárda
    /// olo: **@CODE@** Physiological feature, ivdni, fárda
    /// sme: (no comment)
    /// vro: **@CODE@** Physiological feature, ivdni, fárda
    #[strum(serialize = "Sem/Feat-phys")]
    Sem_SLASH_Feat_MINUS_phys,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Feat-phys_Food_State")]
    Sem_SLASH_Feat_MINUS_phys_Food_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Feat-phys_Hum")]
    Sem_SLASH_Feat_MINUS_phys_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Feat-phys_Obj")]
    Sem_SLASH_Feat_MINUS_phys_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Feat-phys_Plc-line")]
    Sem_SLASH_Feat_MINUS_phys_Plc_MINUS_line,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-phys_Tool-write")]
    Sem_SLASH_Feat_MINUS_phys_Tool_MINUS_write,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-phys_Veh")]
    Sem_SLASH_Feat_MINUS_phys_Veh,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-phys_Wthr")]
    Sem_SLASH_Feat_MINUS_phys_Wthr,
    /// kpv: **@CODE@** Psychological feauture
    /// liv: **@CODE@** Psychological feauture
    /// mhr: **@CODE@** = Psychological feauture
    /// sma: (no comment)
    /// smj: **@CODE@** = Psychological feature; authority, nature, childishness, creativity, arrogance
    /// yrk: **@CODE@** Psychological feauture
    /// sms: **@CODE@** = Psychological feauture
    /// mrj: **@CODE@** Psychological feauture
    /// olo: **@CODE@** Psychological feauture
    /// sme: (no comment)
    /// vro: **@CODE@** Psychological feauture
    #[strum(serialize = "Sem/Feat-psych")]
    Sem_SLASH_Feat_MINUS_psych,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-psych_Hum")]
    Sem_SLASH_Feat_MINUS_psych_Hum,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Feat-psych_Plc")]
    Sem_SLASH_Feat_MINUS_psych_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Feat_Hum")]
    Sem_SLASH_Feat_Hum,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat_Plant")]
    Sem_SLASH_Feat_Plant,
    /// fit: **@CODE@** = Female names
    /// kpv: **@CODE@** Female name
    /// vot: (no comment)
    /// fkv: **@CODE@** = Female names
    /// liv: **@CODE@** Female name
    /// mhr: **@CODE@** = Female name
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Female name
    /// yrk: **@CODE@** Female name
    /// sms: **@CODE@** = Female name
    /// mrj: **@CODE@** Female name
    /// vep: **@CODE@**
    /// olo: **@CODE@** Female name
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Female name
    #[strum(serialize = "Sem/Fem")]
    Sem_SLASH_Fem,
    /// kpv: **@CODE@** Food
    /// liv: **@CODE@** Food
    /// mhr: **@CODE@** = Food
    /// sma: (no comment)
    /// smj: **@CODE@** = Food; bread, vegetarian food, flour, tobacco, salt
    /// yrk: **@CODE@** Food
    /// sms: **@CODE@** = Food
    /// mrj: **@CODE@** Food
    /// olo: **@CODE@** Food
    /// sme: (no comment)
    /// vro: **@CODE@** Food
    #[strum(serialize = "Sem/Food")]
    Sem_SLASH_Food,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Medicine
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Medicine
    /// mhr: **@CODE@** = Medicine
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Medicine; birth-control pill, asthma medicine, medicine, penicillin, vaccine
    /// yrk: **@CODE@** Medicine
    /// sms: **@CODE@** = Medicine
    /// mrj: **@CODE@** Medicine
    /// vep: **@CODE@**
    /// olo: **@CODE@** Medicine
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Medicine
    #[strum(serialize = "Sem/Food-med")]
    Sem_SLASH_Food_MINUS_med,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Food_Obj-surfc")]
    Sem_SLASH_Food_Obj_MINUS_surfc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Food_Part")]
    Sem_SLASH_Food_Part,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Food_Perc-phys")]
    Sem_SLASH_Food_Perc_MINUS_phys,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Food_Plant")]
    Sem_SLASH_Food_Plant,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Food_Sign")]
    Sem_SLASH_Food_Sign,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Food_Substnc")]
    Sem_SLASH_Food_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Food_Time")]
    Sem_SLASH_Food_Time,
    /// sma: (no comment)
    /// smj: **@CODE@** = Fruit and fruit-like edibles
    /// sme: (no comment)
    #[strum(serialize = "Sem/Fruit")]
    Sem_SLASH_Fruit,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Fruit_Hum")]
    Sem_SLASH_Fruit_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Fruit_Sport")]
    Sem_SLASH_Fruit_Sport,
    /// kpv: **@CODE@** Furniture
    /// liv: **@CODE@** Furniture
    /// mhr: **@CODE@** = Furniture
    /// sma: (no comment)
    /// smj: **@CODE@** = Furniture; throne, chair, table, altar, trampoline
    /// yrk: **@CODE@** Furniture
    /// sms: **@CODE@** = Furniture
    /// mrj: **@CODE@** Furniture
    /// olo: **@CODE@** Furniture
    /// sme: (no comment)
    /// vro: **@CODE@** Furniture
    #[strum(serialize = "Sem/Furn")]
    Sem_SLASH_Furn,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Furn_Ctain-abstr")]
    Sem_SLASH_Furn_Ctain_MINUS_abstr,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Furn_Time")]
    Sem_SLASH_Furn_Time,
    /// kpv: **@CODE@** Game
    /// liv: **@CODE@** Game
    /// mhr: **@CODE@** = Game
    /// sma: (no comment)
    /// smj: **@CODE@** = Game; biŋgo bingo, TV game, flipper, paintball, chess
    /// yrk: **@CODE@** Game
    /// sms: **@CODE@** = Game
    /// mrj: **@CODE@** Game
    /// olo: **@CODE@** Game
    /// sme: (no comment)
    /// vro: **@CODE@** Game
    #[strum(serialize = "Sem/Game")]
    Sem_SLASH_Game,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Game_Obj-play")]
    Sem_SLASH_Game_Obj_MINUS_play,
    /// fin: (no comment)
    #[strum(serialize = "Sem/Geo")]
    Sem_SLASH_Geo,
    /// kpv: **@CODE@** Geometrical object
    /// liv: **@CODE@** Geometrical object
    /// mhr: **@CODE@** = Geometrical object
    /// sma: (no comment)
    /// smj: **@CODE@** = Geometrical object; triangle, triangle, tetrahedron, asymptote, star
    /// yrk: **@CODE@** Geometrical object
    /// sms: **@CODE@** = Geometrical object
    /// mrj: **@CODE@** Geometrical object
    /// olo: **@CODE@** Geometrical object
    /// sme: (no comment)
    /// vro: **@CODE@** Geometrical object
    #[strum(serialize = "Sem/Geom")]
    Sem_SLASH_Geom,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Geom_Hum_Plc")]
    Sem_SLASH_Geom_Hum_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Geom_Obj")]
    Sem_SLASH_Geom_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Geom_Plc")]
    Sem_SLASH_Geom_Plc,
    /// kpv: **@CODE@** Animal or Human Group
    /// vot: (no comment)
    /// liv: **@CODE@** Animal or Human Group
    /// mhr: **@CODE@** = Animal or Human Group
    /// sma: (no comment)
    /// smj: **@CODE@** = Animal or Human Group; family, herd, group, indigenous people, delegation
    /// yrk: **@CODE@** Animal or Human Group
    /// sms: **@CODE@** = Animal or Human Group
    /// mrj: **@CODE@** Animal or Human Group
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Animal or Human Group
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Animal or Human Group
    #[strum(serialize = "Sem/Group")]
    Sem_SLASH_Group,
    /// fit: **@CODE@**
    /// kpv: (no comment)
    /// fkv: **@CODE@** =
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum")]
    Sem_SLASH_Group_Hum,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Org")]
    Sem_SLASH_Group_Hum_Org,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Plc")]
    Sem_SLASH_Group_Hum_Plc,
    /// sma: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Plc-abstr")]
    Sem_SLASH_Group_Hum_Plc_MINUS_abstr,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Prod-vis")]
    Sem_SLASH_Group_Hum_Prod_MINUS_vis,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Time")]
    Sem_SLASH_Group_Hum_Time,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Org")]
    Sem_SLASH_Group_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Group_Plc")]
    Sem_SLASH_Group_Plc,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Group_Prod-vis")]
    Sem_SLASH_Group_Prod_MINUS_vis,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Group_Prod-vis_Txt_Veh")]
    Sem_SLASH_Group_Prod_MINUS_vis_Txt_Veh,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Sign")]
    Sem_SLASH_Group_Sign,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Group_State")]
    Sem_SLASH_Group_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Txt")]
    Sem_SLASH_Group_Txt,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Holiday")]
    Sem_SLASH_Holiday,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Human
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Human
    /// mhr: **@CODE@** = Human
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Human
    /// yrk: **@CODE@** Human
    /// sms: **@CODE@** = Human
    /// mrj: **@CODE@** Human
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Human
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Human
    #[strum(serialize = "Sem/Hum")]
    Sem_SLASH_Hum,
    /// kpv: **@CODE@** Human abstract
    /// liv: **@CODE@** Human abstract
    /// mhr: **@CODE@** = Human abstract
    /// smj: **@CODE@** = Human abstract
    /// smj: **@CODE@** =
    /// yrk: **@CODE@** Human abstract
    /// sms: **@CODE@** = Human abstract
    /// mrj: **@CODE@** Human abstract
    /// olo: **@CODE@** Human abstract
    /// sme: (no comment)
    /// vro: **@CODE@** Human abstract
    #[strum(serialize = "Sem/Hum-abstr")]
    Sem_SLASH_Hum_MINUS_abstr,
    /// sms: **@CODE@** =
    #[strum(serialize = "Sem/Hum-kin")]
    Sem_SLASH_Hum_MINUS_kin,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum-prof")]
    Sem_SLASH_Hum_MINUS_prof,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Lang")]
    Sem_SLASH_Hum_Lang,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Lang_Plc")]
    Sem_SLASH_Hum_Lang_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Lang_Time")]
    Sem_SLASH_Hum_Lang_Time,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Mat_Tool")]
    Sem_SLASH_Hum_Mat_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Money")]
    Sem_SLASH_Hum_Money,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Obj")]
    Sem_SLASH_Hum_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Obj_Plc")]
    Sem_SLASH_Hum_Obj_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Org")]
    Sem_SLASH_Hum_Org,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Org_Pos")]
    Sem_SLASH_Hum_Org_Pos,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Part")]
    Sem_SLASH_Hum_Part,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Plant")]
    Sem_SLASH_Hum_Plant,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Plc")]
    Sem_SLASH_Hum_Plc,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Pos")]
    Sem_SLASH_Hum_Pos,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Prod-vis")]
    Sem_SLASH_Hum_Prod_MINUS_vis,
    /// kpv: **@CODE@**  profession, capacity doctor, tractor driver
    #[strum(serialize = "Sem/Hum_Prof")]
    Sem_SLASH_Hum_Prof,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Rule")]
    Sem_SLASH_Hum_Rule,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Sign")]
    Sem_SLASH_Hum_Sign,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_State")]
    Sem_SLASH_Hum_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Tool")]
    Sem_SLASH_Hum_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Tool-catch")]
    Sem_SLASH_Hum_Tool_MINUS_catch,
    /// sms: **@CODE@** = Human
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Tool-it")]
    Sem_SLASH_Hum_Tool_MINUS_it,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Veh")]
    Sem_SLASH_Hum_Veh,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Wthr")]
    Sem_SLASH_Hum_Wthr,
    /// fin: (no comment)
    #[strum(serialize = "Sem/Human")]
    Sem_SLASH_Human,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = ID
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = ID
    /// sms: **@CODE@** = number as identity tag, not as amount
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** used with numerals 2023_04_04
    #[strum(serialize = "Sem/ID")]
    Sem_SLASH_ID,
    /// kpv: **@CODE@** Ideology
    /// liv: **@CODE@** Ideology
    /// mhr: **@CODE@** = Ideology
    /// sma: (no comment)
    /// smj: **@CODE@** = Ideology; nomadism, buddhism, feminism, christianity, fanaticism
    /// yrk: **@CODE@** Ideology
    /// sms: **@CODE@** = Ideology
    /// mrj: **@CODE@** Ideology
    /// olo: **@CODE@** Ideology
    /// sme: (no comment)
    /// vro: **@CODE@** Ideology
    #[strum(serialize = "Sem/Ideol")]
    Sem_SLASH_Ideol,
    /// sms: **@CODE@** for deprication to Sem/Hum-kin
    #[strum(serialize = "Sem/Kin")]
    Sem_SLASH_Kin,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Language
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Language
    /// mhr: **@CODE@** = Language
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Language; South Sámi, mother tongue, Maori, Yiddish, second language
    /// yrk: **@CODE@** Language
    /// sms: **@CODE@** = Language
    /// mrj: **@CODE@** Language
    /// vep: **@CODE@**
    /// olo: **@CODE@** Language
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Language
    #[strum(serialize = "Sem/Lang")]
    Sem_SLASH_Lang,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Lang_Tool")]
    Sem_SLASH_Lang_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Lang_Tool-catch")]
    Sem_SLASH_Lang_Tool_MINUS_catch,
    /// fit: **@CODE@** = Male names
    /// kpv: **@CODE@** Male name
    /// vot: (no comment)
    /// fkv: **@CODE@** = Male names
    /// liv: **@CODE@** Male name
    /// mhr: **@CODE@** = Male name
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Male name
    /// yrk: **@CODE@** Male name
    /// sms: **@CODE@** = Male name
    /// mrj: **@CODE@** Male name
    /// vep: **@CODE@**
    /// olo: **@CODE@** Male name
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Male name
    #[strum(serialize = "Sem/Mal")]
    Sem_SLASH_Mal,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Manner")]
    Sem_SLASH_Manner,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Material for producing things
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Material for producing things
    /// mhr: **@CODE@** = Material for producing things
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Material for producing things; paper, steel, wood, leather, wool
    /// yrk: **@CODE@** Material for producing things
    /// sms: **@CODE@** = Material for producing things
    /// mrj: **@CODE@** Material for producing things
    /// vep: **@CODE@**
    /// olo: **@CODE@** Material for producing things
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Material for producing things
    #[strum(serialize = "Sem/Mat")]
    Sem_SLASH_Mat,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Mat_Obj")]
    Sem_SLASH_Mat_Obj,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Mat_Obj-clo")]
    Sem_SLASH_Mat_Obj_MINUS_clo,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Mat_Part")]
    Sem_SLASH_Mat_Part,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Mat_Plant")]
    Sem_SLASH_Mat_Plant,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Mat_Plantpart")]
    Sem_SLASH_Mat_Plantpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Mat_Plc")]
    Sem_SLASH_Mat_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Mat_Tool")]
    Sem_SLASH_Mat_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Mat_Tool-catch")]
    Sem_SLASH_Mat_Tool_MINUS_catch,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Mat_Txt")]
    Sem_SLASH_Mat_Txt,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Measure
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Measure
    /// mhr: **@CODE@** = Measure
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Measure; unit of volume, quarter meter, hour, minus degree, wátta watt
    /// yrk: **@CODE@** Measure
    /// sms: **@CODE@** = Measure
    /// mrj: **@CODE@** Measure
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Measure
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Measure
    #[strum(serialize = "Sem/Measr")]
    Sem_SLASH_Measr,
    /// sma: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "Sem/Measr_Obj_Time")]
    Sem_SLASH_Measr_Obj_Time,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Measr_Plc_Time")]
    Sem_SLASH_Measr_Plc_Time,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@** = Sign (e.g. numbers, punctuation)
    /// sme: (no comment)
    #[strum(serialize = "Sem/Measr_Sign")]
    Sem_SLASH_Measr_Sign,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Measr_Time")]
    Sem_SLASH_Measr_Time,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// mhr: **@CODE@** = Has to do with money, like wages, not Curr(ency)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Has to do with money; wages, not Curr(ency), treasure, belongings, debt, food price, bill, pension
    /// yrk: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// sms: **@CODE@** = Has to do with money, like wages, not Curr(ency)
    /// mrj: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// vep: **@CODE@**
    /// olo: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Has to do with money, like wages, not Curr(ency)
    #[strum(serialize = "Sem/Money")]
    Sem_SLASH_Money,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Money_Obj")]
    Sem_SLASH_Money_Obj,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Money_Org")]
    Sem_SLASH_Money_Org,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Money_Part")]
    Sem_SLASH_Money_Part,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Money_Plc")]
    Sem_SLASH_Money_Plc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Money_Txt")]
    Sem_SLASH_Money_Txt,
    /// fit: **@CODE@** = Names of objects
    /// kpv: **@CODE@** Object
    /// vot: (no comment)
    /// fkv: **@CODE@** = Names of objects
    /// liv: **@CODE@** Object
    /// mhr: **@CODE@** = Object
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Object; thing, cup, thing, toy, painting
    /// yrk: **@CODE@** Object
    /// sms: **@CODE@**              (names)
    /// mrj: **@CODE@** Object
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Object
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Object
    #[strum(serialize = "Sem/Obj")]
    Sem_SLASH_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj-catch")]
    Sem_SLASH_Obj_MINUS_catch,
    /// kpv: **@CODE@** Cloth
    /// liv: **@CODE@** Cloth
    /// mhr: **@CODE@** = Cloth
    /// sma: (no comment)
    /// smj: **@CODE@** = Cloth; carpet, flag, curtain, silk sheets, napkin
    /// yrk: **@CODE@** Cloth
    /// sms: **@CODE@** = Cloth
    /// mrj: **@CODE@** Cloth
    /// olo: **@CODE@** Cloth
    /// sme: (no comment)
    /// vro: **@CODE@** Cloth
    #[strum(serialize = "Sem/Obj-clo")]
    Sem_SLASH_Obj_MINUS_clo,
    /// kpv: **@CODE@** Cloth
    /// liv: **@CODE@** Cloth
    /// mhr: **@CODE@** = Cloth
    /// smj: **@CODE@** =
    /// yrk: **@CODE@** Cloth
    /// sms: **@CODE@** = Cloth
    /// mrj: **@CODE@** Cloth
    /// olo: **@CODE@** Cloth
    /// sme: (no comment)
    /// vro: **@CODE@** Cloth
    #[strum(serialize = "Sem/Obj-cogn")]
    Sem_SLASH_Obj_MINUS_cogn,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** (Electrical) machine or apparatus
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** (Electrical) machine or apparatus
    /// mhr: **@CODE@** = (Electrical) machine or apparatus
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = (Electrical) machine or apparatus; player, lamp, TV, radio, oven
    /// yrk: **@CODE@** (Electrical) machine or apparatus
    /// sms: **@CODE@** = (Electrical) machine or apparatus
    /// mrj: **@CODE@** (Electrical) machine or apparatus
    /// vep: **@CODE@**
    /// olo: **@CODE@** (Electrical) machine or apparatus
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** (Electrical) machine or apparatus
    #[strum(serialize = "Sem/Obj-el")]
    Sem_SLASH_Obj_MINUS_el,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Object with something written on it
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Object with something written on it
    /// mhr: **@CODE@** = Object with something written on it
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Object with something written on it
    /// yrk: **@CODE@** Object with something written on it
    /// sms: **@CODE@** = Object with something written on it
    /// mrj: **@CODE@** Object with something written on it
    /// vep: **@CODE@**
    /// olo: **@CODE@** Object with something written on it
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Object with something written on it
    #[strum(serialize = "Sem/Obj-ling")]
    Sem_SLASH_Obj_MINUS_ling,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Obj-ling_Obj-surfc")]
    Sem_SLASH_Obj_MINUS_ling_Obj_MINUS_surfc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** = Play object
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj-play")]
    Sem_SLASH_Obj_MINUS_play,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj-play_Sport")]
    Sem_SLASH_Obj_MINUS_play_Sport,
    /// kpv: **@CODE@** flexible ropelike object
    /// liv: **@CODE@** flexible ropelike object
    /// mhr: **@CODE@** = flexible ropelike object
    /// sma: (no comment)
    /// smj: **@CODE@** = flexible ropelike object; barbed wire, thread, rope, cable, dental floss
    /// yrk: **@CODE@** flexible ropelike object
    /// sms: **@CODE@** = flexible ropelike object
    /// mrj: **@CODE@** flexible ropelike object
    /// olo: **@CODE@** flexible ropelike object
    /// sme: (no comment)
    /// vro: **@CODE@** flexible ropelike object
    #[strum(serialize = "Sem/Obj-rope")]
    Sem_SLASH_Obj_MINUS_rope,
    /// kpv: **@CODE@** Surface object
    /// liv: **@CODE@** Surface object
    /// mhr: **@CODE@** = Surface object
    /// sma: (no comment)
    /// smj: **@CODE@** = Surface object; blackboard, drawing paper, canvas, board (for playing board games), (money) bill
    /// yrk: **@CODE@** Surface object
    /// sms: **@CODE@** = Surface object
    /// mrj: **@CODE@** Surface object
    /// olo: **@CODE@** Surface object
    /// sme: (no comment)
    /// vro: **@CODE@** Surface object
    #[strum(serialize = "Sem/Obj-surfc")]
    Sem_SLASH_Obj_MINUS_surfc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Part_Sign")]
    Sem_SLASH_Obj_Part_Sign,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Perc-emo")]
    Sem_SLASH_Obj_Perc_MINUS_emo,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Plantpart")]
    Sem_SLASH_Obj_Plantpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Plc")]
    Sem_SLASH_Obj_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Plc-abstr")]
    Sem_SLASH_Obj_Plc_MINUS_abstr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Plc_Semcon")]
    Sem_SLASH_Obj_Plc_Semcon,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Process")]
    Sem_SLASH_Obj_Process,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Prod-audio")]
    Sem_SLASH_Obj_Prod_MINUS_audio,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj_Semcon")]
    Sem_SLASH_Obj_Semcon,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Obj_Sign")]
    Sem_SLASH_Obj_Sign,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj_State")]
    Sem_SLASH_Obj_State,
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Obj_Symbol")]
    Sem_SLASH_Obj_Symbol,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Tool-music")]
    Sem_SLASH_Obj_Tool_MINUS_music,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Tool-write")]
    Sem_SLASH_Obj_Tool_MINUS_write,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj_Txt")]
    Sem_SLASH_Obj_Txt,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Obj_Veh")]
    Sem_SLASH_Obj_Veh,
    /// sms: **@CODE@** (not used so far (?)), note oeu-
    #[strum(serialize = "Sem/Oeuvre")]
    Sem_SLASH_Oeuvre,
    /// sms: **@CODE@** (so far only Biblija, changed to Sem/Txt)
    #[strum(serialize = "Sem/Oeuvre-txt")]
    Sem_SLASH_Oeuvre_MINUS_txt,
    /// fit: **@CODE@** = Names of organisations
    /// kpv: **@CODE@** Organisation
    /// vot: (no comment)
    /// fkv: **@CODE@** = Names of organisations
    /// liv: **@CODE@** Organisation
    /// mhr: **@CODE@** = Organisation
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Organisation; newspaper, supreme court, company, museum, administration
    /// yrk: **@CODE@** Organisation
    /// sms: **@CODE@** Organization (names)
    /// mrj: **@CODE@** Organisation
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Organisation
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Organisation
    #[strum(serialize = "Sem/Org")]
    Sem_SLASH_Org,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Org_Buildpart")]
    Sem_SLASH_Org_Buildpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Org_Play")]
    Sem_SLASH_Org_Play,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Org_Plc")]
    Sem_SLASH_Org_Plc,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Org_Prod-audio")]
    Sem_SLASH_Org_Prod_MINUS_audio,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Org_Prod-cogn")]
    Sem_SLASH_Org_Prod_MINUS_cogn,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Org_Prod-vis")]
    Sem_SLASH_Org_Prod_MINUS_vis,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Org_Rule")]
    Sem_SLASH_Org_Rule,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Org_State")]
    Sem_SLASH_Org_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Org_Txt")]
    Sem_SLASH_Org_Txt,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Org_Veh")]
    Sem_SLASH_Org_Veh,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Feature, oassi, bealli
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Feature, oassi, bealli
    /// mhr: **@CODE@** = Feature, oassi, bealli
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Feature, part of something; half, part, percent, rest, tenth
    /// yrk: **@CODE@** Feature, oassi, bealli
    /// sms: **@CODE@** = Feature, oassi, bealli
    /// mrj: **@CODE@** Feature, oassi, bealli
    /// vep: **@CODE@**
    /// olo: **@CODE@** Feature, oassi, bealli
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Feature, oassi, bealli
    #[strum(serialize = "Sem/Part")]
    Sem_SLASH_Part,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Part_Plc")]
    Sem_SLASH_Part_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Part_Plc_Prod-audio")]
    Sem_SLASH_Part_Plc_Prod_MINUS_audio,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Part_Prod-cogn")]
    Sem_SLASH_Part_Prod_MINUS_cogn,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Part_Substnc")]
    Sem_SLASH_Part_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Part_Txt")]
    Sem_SLASH_Part_Txt,
    /// kpv: **@CODE@**  Patronym
    /// mrj: **@CODE@** =
    #[strum(serialize = "Sem/Patr")]
    Sem_SLASH_Patr,
    /// kpv: **@CODE@**  Patronym female
    /// olo: (no comment)
    #[strum(serialize = "Sem/Patr-Fem")]
    Sem_SLASH_Patr_MINUS_Fem,
    /// kpv: **@CODE@**  Patronym male
    /// olo: (no comment)
    #[strum(serialize = "Sem/Patr-Mal")]
    Sem_SLASH_Patr_MINUS_Mal,
    /// kpv: **@CODE@** Cognative perception
    /// liv: **@CODE@** Cognative perception
    /// mhr: **@CODE@** = Cognative perception
    /// smj: **@CODE@** = Cloth
    /// yrk: **@CODE@** Cognative perception
    /// sms: **@CODE@** = Cloth
    /// mrj: **@CODE@** Cognative perception
    /// olo: **@CODE@** Cognative perception
    /// sme: (no comment)
    /// vro: **@CODE@** Cognative perception
    #[strum(serialize = "Sem/Perc-cogn")]
    Sem_SLASH_Perc_MINUS_cogn,
    /// kpv: **@CODE@** Emotional perception
    /// liv: **@CODE@** Emotional perception
    /// mhr: **@CODE@** = Emotional perception
    /// sma: (no comment)
    /// smj: **@CODE@** = Emotional perception; fear, feeling of identity, empathy, sadness, working motivation
    /// yrk: **@CODE@** Emotional perception
    /// sms: **@CODE@** = Emotional perception
    /// mrj: **@CODE@** Emotional perception
    /// olo: **@CODE@** Emotional perception
    /// sme: (no comment)
    /// vro: **@CODE@** Emotional perception
    #[strum(serialize = "Sem/Perc-emo")]
    Sem_SLASH_Perc_MINUS_emo,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Perc-emo_Plc")]
    Sem_SLASH_Perc_MINUS_emo_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Perc-emo_State")]
    Sem_SLASH_Perc_MINUS_emo_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Perc-emo_Wthr")]
    Sem_SLASH_Perc_MINUS_emo_Wthr,
    /// kpv: **@CODE@** Physical perception
    /// liv: **@CODE@** Physical perception
    /// mhr: **@CODE@** = Physical perception
    /// sma: (no comment)
    /// smj: **@CODE@** = Physical perception; shoulder pain, gass smell, sleep (during the night), need of sleep, hunger
    /// yrk: **@CODE@** Physical perception
    /// sms: **@CODE@** = Physical perception
    /// mrj: **@CODE@** Physical perception
    /// olo: **@CODE@** Physical perception
    /// sme: (no comment)
    /// vro: **@CODE@** Physical perception
    #[strum(serialize = "Sem/Perc-phys")]
    Sem_SLASH_Perc_MINUS_phys,
    /// kpv: **@CODE@** Physical perception
    /// liv: **@CODE@** Physical perception
    /// mhr: **@CODE@** = Physical perception
    /// smj: **@CODE@** = Psychic perception
    /// yrk: **@CODE@** Physical perception
    /// sms: **@CODE@** = Psychical perception
    /// mrj: **@CODE@** Physical perception
    /// olo: **@CODE@** Physical perception
    /// sme: (no comment)
    /// vro: **@CODE@** Physical perception
    #[strum(serialize = "Sem/Perc-psych")]
    Sem_SLASH_Perc_MINUS_psych,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Phonenr")]
    Sem_SLASH_Phonenr,
    /// kpv: **@CODE@** Plant
    /// vot: (no comment)
    /// liv: **@CODE@** Plant
    /// mhr: **@CODE@** = Plant
    /// sma: (no comment)
    /// smj: **@CODE@** = Plant; lichen, plant, cucumber, bluebell, poisonous mushroom
    /// yrk: **@CODE@** Plant
    /// sms: **@CODE@** = Plant
    /// mrj: **@CODE@** Plant
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Plant
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Plant
    #[strum(serialize = "Sem/Plant")]
    Sem_SLASH_Plant,
    /// kpv: **@CODE@** Plant part
    /// liv: **@CODE@** Plant part
    /// mhr: **@CODE@** = Plant part
    /// yrk: **@CODE@** Plant part
    /// sms: **@CODE@** = Plant part
    /// mrj: **@CODE@** Plant part
    /// olo: **@CODE@** Plant part
    /// vro: **@CODE@** Plant part
    #[strum(serialize = "Sem/Plant-part")]
    Sem_SLASH_Plant_MINUS_part,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plant_Plant-part")]
    Sem_SLASH_Plant_Plant_MINUS_part,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Plant_Plantpart")]
    Sem_SLASH_Plant_Plantpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plant_Plc")]
    Sem_SLASH_Plant_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plant_Time_Wthr")]
    Sem_SLASH_Plant_Time_Wthr,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plant_Tool")]
    Sem_SLASH_Plant_Tool,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plant_Tool-measr")]
    Sem_SLASH_Plant_Tool_MINUS_measr,
    /// sma: (no comment)
    /// smj: **@CODE@** = Plant part; twig, leaf, root, seed, pine trunk
    /// sme: (no comment)
    #[strum(serialize = "Sem/Plantpart")]
    Sem_SLASH_Plantpart,
    /// smj: **@CODE@** = Play
    #[strum(serialize = "Sem/Play")]
    Sem_SLASH_Play,
    /// fit: **@CODE@** = Place names
    /// kpv: **@CODE@** Place
    /// vot: (no comment)
    /// fkv: **@CODE@** = Place names
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Place; world, nature, airport, workplace, fireplace
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** Place name   (names)
    /// mrj: **@CODE@** Place
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Place
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc")]
    Sem_SLASH_Plc,
    /// kpv: **@CODE@** Abstract place
    /// liv: **@CODE@** Abstract place
    /// mhr: **@CODE@** = Abstract place
    /// sma: (no comment)
    /// smj: **@CODE@** = Abstract place; bachelor level, job market, third place, Troms website, address
    /// yrk: **@CODE@** Abstract place
    /// sms: **@CODE@** = Abstract place
    /// mrj: **@CODE@** Abstract place
    /// olo: **@CODE@** Abstract place
    /// sme: (no comment)
    /// vro: **@CODE@** Abstract place
    #[strum(serialize = "Sem/Plc-abstr")]
    Sem_SLASH_Plc_MINUS_abstr,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc-abstr_Rel_State")]
    Sem_SLASH_Plc_MINUS_abstr_Rel_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc-abstr_Route")]
    Sem_SLASH_Plc_MINUS_abstr_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plc-abstr_Rule")]
    Sem_SLASH_Plc_MINUS_abstr_Rule,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plc-abstr_State")]
    Sem_SLASH_Plc_MINUS_abstr_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plc-abstr_Txt")]
    Sem_SLASH_Plc_MINUS_abstr_Txt,
    /// kpv: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// sma: (no comment)
    /// smj: **@CODE@** = Elevated place; mountain, peak, mountain top, scree, volcano
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** = Place
    /// mrj: **@CODE@** Place
    /// olo: **@CODE@** Place
    /// sme: (no comment)
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc-elevate")]
    Sem_SLASH_Plc_MINUS_elevate,
    /// kpv: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// sma: (no comment)
    /// smj: **@CODE@** = Place limitations; national border, border, finish line, stop line, equator
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** = Place
    /// mrj: **@CODE@** Place
    /// olo: **@CODE@** Place
    /// sme: (no comment)
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc-line")]
    Sem_SLASH_Plc_MINUS_line,
    /// kpv: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// sma: (no comment)
    /// smj: **@CODE@** = Water; river, lake, polar sea, sea, well
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** = Place
    /// mrj: **@CODE@** Place
    /// olo: **@CODE@** Place
    /// sme: (no comment)
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc-water")]
    Sem_SLASH_Plc_MINUS_water,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Pos")]
    Sem_SLASH_Plc_Pos,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Route")]
    Sem_SLASH_Plc_Route,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Plc_Semcon")]
    Sem_SLASH_Plc_Semcon,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Plc_State")]
    Sem_SLASH_Plc_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Substnc")]
    Sem_SLASH_Plc_Substnc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Substnc_Wthr")]
    Sem_SLASH_Plc_Substnc_Wthr,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Time")]
    Sem_SLASH_Plc_Time,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/Plc_Time_Wthr")]
    Sem_SLASH_Plc_Time_Wthr,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Tool-catch")]
    Sem_SLASH_Plc_Tool_MINUS_catch,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    #[strum(serialize = "Sem/Plc_Txt")]
    Sem_SLASH_Plc_Txt,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Wthr")]
    Sem_SLASH_Plc_Wthr,
    /// kpv: **@CODE@** Position (as in social position job)
    /// liv: **@CODE@** Position (as in social position job)
    /// mhr: **@CODE@** = Position (as in social position job)
    /// sma: (no comment)
    /// smj: **@CODE@** = Position (as in social position job); 50% position, presidency, fixed position, kindergarten place, role in society
    /// yrk: **@CODE@** Position (as in social position job)
    /// sms: **@CODE@** = Position (as in social position job)
    /// mrj: **@CODE@** Position (as in social position job)
    /// olo: **@CODE@** Position (as in social position job)
    /// sme: (no comment)
    /// vro: **@CODE@** Position (as in social position job)
    #[strum(serialize = "Sem/Pos")]
    Sem_SLASH_Pos,
    /// kpv: **@CODE@** Process
    /// liv: **@CODE@** Process
    /// mhr: **@CODE@** = Process
    /// sma: (no comment)
    /// smj: **@CODE@** = Process
    /// yrk: **@CODE@** Process
    /// sms: **@CODE@** = Process
    /// mrj: **@CODE@** Process
    /// olo: **@CODE@** Process
    /// sme: (no comment)
    /// vro: **@CODE@** Process
    #[strum(serialize = "Sem/Process")]
    Sem_SLASH_Process,
    /// kpv: **@CODE@** Product
    /// liv: **@CODE@** Product
    /// mhr: **@CODE@** = Product
    /// sma: (no comment)
    /// smj: **@CODE@** = Product
    /// yrk: **@CODE@** Product
    /// sms: **@CODE@** = Product
    /// mrj: **@CODE@** Product
    /// olo: **@CODE@** Product
    /// sme: (no comment)
    /// vro: **@CODE@** Product
    #[strum(serialize = "Sem/Prod")]
    Sem_SLASH_Prod,
    /// kpv: **@CODE@** Audio product
    /// liv: **@CODE@** Audio product
    /// mhr: **@CODE@** = Audio product
    /// sma: (no comment)
    /// smj: **@CODE@** = Audio product; yoik, roar, Beatles song, Bible psalm, blues
    /// yrk: **@CODE@** Audio product
    /// sms: **@CODE@** = Audio product
    /// mrj: **@CODE@** Audio product
    /// olo: **@CODE@** Audio product
    /// sme: (no comment)
    /// vro: **@CODE@** Audio product
    #[strum(serialize = "Sem/Prod-audio")]
    Sem_SLASH_Prod_MINUS_audio,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Prod-audio_Prod-vis")]
    Sem_SLASH_Prod_MINUS_audio_Prod_MINUS_vis,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Prod-audio_Rule")]
    Sem_SLASH_Prod_MINUS_audio_Rule,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Prod-audio_Substnc")]
    Sem_SLASH_Prod_MINUS_audio_Substnc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Prod-audio_Txt")]
    Sem_SLASH_Prod_MINUS_audio_Txt,
    /// kpv: **@CODE@** Cognition product
    /// liv: **@CODE@** Cognition product
    /// mhr: **@CODE@** = Cognition product
    /// sma: (no comment)
    /// smj: **@CODE@** = Cognition product; thought, decision, knowledge, lack of understanding, requirement
    /// yrk: **@CODE@** Cognition product
    /// sms: **@CODE@** = Cognition product
    /// mrj: **@CODE@** Cognition product
    /// olo: **@CODE@** Cognition product
    /// sme: (no comment)
    /// vro: **@CODE@** Cognition product
    #[strum(serialize = "Sem/Prod-cogn")]
    Sem_SLASH_Prod_MINUS_cogn,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Prod-cogn_Txt")]
    Sem_SLASH_Prod_MINUS_cogn_Txt,
    /// kpv: **@CODE@** Linguistic product
    /// liv: **@CODE@** Linguistic product
    /// mhr: **@CODE@** = Linguistic product
    /// sma: (no comment)
    /// smj: **@CODE@** = Linguistic product; message, question, agreement, translation, criticism
    /// yrk: **@CODE@** Linguistic product
    /// sms: **@CODE@** = Linguistic product
    /// mrj: **@CODE@** Linguistic product
    /// olo: **@CODE@** Linguistic product
    /// sme: (no comment)
    /// vro: **@CODE@** Linguistic product
    #[strum(serialize = "Sem/Prod-ling")]
    Sem_SLASH_Prod_MINUS_ling,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Visual product
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Visual product
    /// mhr: **@CODE@** = Visual product
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Visual product; picture, film, TV series, documentary, art
    /// yrk: **@CODE@** Visual product
    /// sms: **@CODE@** = Visual product
    /// mrj: **@CODE@** Visual product
    /// vep: **@CODE@**
    /// olo: **@CODE@** Visual product
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Visual product
    #[strum(serialize = "Sem/Prod-vis")]
    Sem_SLASH_Prod_MINUS_vis,
    /// kpv: **@CODE@** Relation
    /// liv: **@CODE@** Relation
    /// mhr: **@CODE@** = Relation
    /// sma: (no comment)
    /// smj: **@CODE@** = Relation; relation, dependency, subordination, analogy, equivalence
    /// yrk: **@CODE@** Relation
    /// sms: **@CODE@** = Relation
    /// mrj: **@CODE@** Relation
    /// olo: **@CODE@** Relation
    /// sme: (no comment)
    /// vro: **@CODE@** Relation
    #[strum(serialize = "Sem/Rel")]
    Sem_SLASH_Rel,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Name of a Route
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Name of a Route
    /// mhr: **@CODE@** = Name of a Route
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Route-like place; street, path, corridor, bridge, winter path
    /// yrk: **@CODE@** Name of a Route
    /// sms: **@CODE@** = Route
    /// mrj: **@CODE@** Name of a Route
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Name of a Route
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Name of a Route
    #[strum(serialize = "Sem/Route")]
    Sem_SLASH_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Route_State")]
    Sem_SLASH_Route_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Route_Txt")]
    Sem_SLASH_Route_Txt,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Rule or convention
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Rule or convention
    /// mhr: **@CODE@** = Rule or convention
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Rule or convention; cultural tradition, abortion law, EU rule, law of cosines, fair play
    /// yrk: **@CODE@** Rule or convention
    /// sms: **@CODE@** = Rule or convention
    /// mrj: **@CODE@** Rule or convention
    /// vep: **@CODE@**
    /// olo: **@CODE@** Rule or convention
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Rule or convention
    #[strum(serialize = "Sem/Rule")]
    Sem_SLASH_Rule,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Rule_Txt")]
    Sem_SLASH_Rule_Txt,
    /// kpv: **@CODE@** Semantic concept
    /// liv: **@CODE@** Semantic concept
    /// mhr: **@CODE@** = Semantic concept
    /// sma: (no comment)
    /// smj: **@CODE@** = Abstract semantic concept; result, objective, reason, alternative, case
    /// yrk: **@CODE@** Semantic concept
    /// sms: **@CODE@** = Semantic concept
    /// mrj: **@CODE@** Semantic concept
    /// olo: **@CODE@** Semantic concept
    /// sme: (no comment)
    /// vro: **@CODE@** Semantic concept
    #[strum(serialize = "Sem/Semcon")]
    Sem_SLASH_Semcon,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Semcon_State")]
    Sem_SLASH_Semcon_State,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Semcon_Txt")]
    Sem_SLASH_Semcon_Txt,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Semcon_Wthr")]
    Sem_SLASH_Semcon_Wthr,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Sign (e.g. numbers, punctuation)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Sign (e.g. numbers, punctuation)
    /// mhr: **@CODE@** = Sign (e.g. numbers, punctuation)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Sign (e.g. numbers, punctuation); ID number, ČSV letters, quotation mark, hieroglyph, symbol
    /// yrk: **@CODE@** Sign (e.g. numbers, punctuation)
    /// sms: **@CODE@** = Sign (e.g. numbers, punctuation)
    /// mrj: **@CODE@** Sign (e.g. numbers, punctuation)
    /// vep: **@CODE@**
    /// olo: **@CODE@** Sign (e.g. numbers, punctuation)
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Sign (e.g. numbers, punctuation)
    #[strum(serialize = "Sem/Sign")]
    Sem_SLASH_Sign,
    /// kpv: **@CODE@** Sport
    /// liv: **@CODE@** Sport
    /// mhr: **@CODE@** = Sport
    /// sma: (no comment)
    /// smj: **@CODE@** = Sport; table tennis, judo, motor cross, ice hockey, floorball
    /// yrk: **@CODE@** Sport
    /// sms: **@CODE@** = Sport
    /// mrj: **@CODE@** Sport
    /// olo: **@CODE@** Sport
    /// sme: (no comment)
    /// vro: **@CODE@** Sport
    #[strum(serialize = "Sem/Sport")]
    Sem_SLASH_Sport,
    /// fit: **@CODE@**
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = State; hurry, captivity, anarchy, biodiversity, chaos
    /// yrk: **@CODE@**
    /// sms: **@CODE@** =
    /// mrj: **@CODE@**
    /// vep: **@CODE@**
    /// olo: **@CODE@**
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/State")]
    Sem_SLASH_State,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Illness
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Illness
    /// mhr: **@CODE@** = Illness
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Illness; allergy, cold, autism, dementia, somnambulism
    /// yrk: **@CODE@** Illness
    /// sms: **@CODE@** = Illness
    /// mrj: **@CODE@** Illness
    /// vep: **@CODE@**
    /// olo: **@CODE@** Illness
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Illness
    #[strum(serialize = "Sem/State-sick")]
    Sem_SLASH_State_MINUS_sick,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    #[strum(serialize = "Sem/State-sick_Substnc")]
    Sem_SLASH_State_MINUS_sick_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/State_Veh")]
    Sem_SLASH_State_Veh,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Substance, like Air and Water
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Substance, like Air and Water
    /// mhr: **@CODE@** = Substance, like Air and Water
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Substance; Water, sand, air, smoke, carbohydrate, vitamin, dust
    /// yrk: **@CODE@** Substance, like Air and Water
    /// sms: **@CODE@** = Substance, like Air and Water
    /// mrj: **@CODE@** Substance, like Air and Water
    /// vep: **@CODE@**
    /// olo: **@CODE@** Substance, like Air and Water
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Substance, like Air and Water
    #[strum(serialize = "Sem/Substnc")]
    Sem_SLASH_Substnc,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Substnc_Wthr")]
    Sem_SLASH_Substnc_Wthr,
    /// fit: **@CODE@** = Surnames
    /// kpv: **@CODE@** Surname
    /// vot: (no comment)
    /// fkv: **@CODE@** = Surnames
    /// liv: **@CODE@** Surname
    /// mhr: **@CODE@** = Surname
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Surname
    /// rus: (no comment)
    /// yrk: **@CODE@** Surname
    /// sms: **@CODE@** Surname       (names)
    /// mrj: **@CODE@** Surname
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Surname
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Surname
    #[strum(serialize = "Sem/Sur")]
    Sem_SLASH_Sur,
    /// kpv: **@CODE@** Surname female
    /// olo: **@CODE@** Female Surname
    #[strum(serialize = "Sem/Sur-Fem")]
    Sem_SLASH_Sur_MINUS_Fem,
    /// kpv: **@CODE@** Surname male
    /// olo: **@CODE@** Male Surname
    #[strum(serialize = "Sem/Sur-Mal")]
    Sem_SLASH_Sur_MINUS_Mal,
    /// kpv: **@CODE@** Symbol
    /// liv: **@CODE@** Symbol
    /// mhr: **@CODE@** = Symbol
    /// smj: **@CODE@** = Symbol
    /// yrk: **@CODE@** Symbol
    /// sms: **@CODE@** = Symbol
    /// mrj: **@CODE@** Symbol
    /// olo: **@CODE@** Symbol
    /// sme: (no comment)
    /// vro: **@CODE@** Symbol
    #[strum(serialize = "Sem/Symbol")]
    Sem_SLASH_Symbol,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Time
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Time
    /// mhr: **@CODE@** = Time
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Time; áigi time, April, half an hour, Tuesday, deadline
    /// yrk: **@CODE@** Time
    /// sms: **@CODE@** = Time
    /// sms: (no comment)
    /// mrj: **@CODE@** Time
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Time
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Time
    #[strum(serialize = "Sem/Time")]
    Sem_SLASH_Time,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Time
    /// sms: **@CODE@** = Time
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Sem/Time-clock")]
    Sem_SLASH_Time_MINUS_clock,
    /// kpv: (no comment)
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// olo: (no comment)
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Time_Wthr")]
    Sem_SLASH_Time_Wthr,
    /// kpv: **@CODE@** Prototypical tool for repairing things
    /// liv: **@CODE@** Prototypical tool for repairing things
    /// mhr: **@CODE@** = Prototypical tool for repairing things
    /// sma: (no comment)
    /// smj: **@CODE@** = Prototypical tool for repairing things; axe, knife, fire striker, plastic hammer, wrench
    /// yrk: **@CODE@** Prototypical tool for repairing things
    /// sms: **@CODE@** = Prototypical tool for repairing things
    /// mrj: **@CODE@** Prototypical tool for repairing things
    /// olo: **@CODE@** Prototypical tool for repairing things
    /// sme: (no comment)
    /// vro: **@CODE@** Prototypical tool for repairing things
    #[strum(serialize = "Sem/Tool")]
    Sem_SLASH_Tool,
    /// kpv: **@CODE@** Tool used for catching (e.g. fish)
    /// liv: **@CODE@** Tool used for catching (e.g. fish)
    /// mhr: **@CODE@** = Tool used for catching (e.g. fish)
    /// sma: (no comment)
    /// smj: **@CODE@** = Tool used for catching; artificial fly, fishing net for cod, fishing rod, lasso used in wintertime, mouse trap
    /// yrk: **@CODE@** Tool used for catching (e.g. fish)
    /// sms: **@CODE@** = Tool used for catching (e.g. fish)
    /// mrj: **@CODE@** Tool used for catching (e.g. fish)
    /// olo: **@CODE@** Tool used for catching (e.g. fish)
    /// sme: (no comment)
    /// vro: **@CODE@** Tool used for catching (e.g. fish)
    #[strum(serialize = "Sem/Tool-catch")]
    Sem_SLASH_Tool_MINUS_catch,
    /// kpv: **@CODE@** Tool used for cleaning
    /// liv: **@CODE@** Tool used for cleaning
    /// mhr: **@CODE@** = Tool used for cleaning
    /// sma: (no comment)
    /// smj: **@CODE@** = Tool used for cleaning; broom, vegetable brush, toothbrush, cleaning equipment
    /// yrk: **@CODE@** Tool used for cleaning
    /// sms: **@CODE@** = Tool used for cleaning
    /// mrj: **@CODE@** Tool used for cleaning
    /// olo: **@CODE@** Tool used for cleaning
    /// sme: (no comment)
    /// vro: **@CODE@** Tool used for cleaning
    #[strum(serialize = "Sem/Tool-clean")]
    Sem_SLASH_Tool_MINUS_clean,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Tool used in IT
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Tool used in IT
    /// mhr: **@CODE@** = Tool used in IT
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Tool used in IT/tool within IT; IT infrastructure, analyzer, searching function, computer program, browser
    /// yrk: **@CODE@** Tool used in IT
    /// sms: **@CODE@** = Tool used in IT
    /// mrj: **@CODE@** Tool used in IT
    /// vep: **@CODE@**
    /// olo: **@CODE@** Tool used in IT
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** Tool used in IT
    #[strum(serialize = "Sem/Tool-it")]
    Sem_SLASH_Tool_MINUS_it,
    /// kpv: **@CODE@** Tool used for measuring
    /// liv: **@CODE@** Tool used for measuring
    /// mhr: **@CODE@** = Tool used for measuring
    /// smj: **@CODE@** = Tool used for measuring; barometer, hourglass, ruler, spirit level, scale
    /// yrk: **@CODE@** Tool used for measuring
    /// sms: **@CODE@** = Tool used for measuring
    /// mrj: **@CODE@** Tool used for measuring
    /// olo: **@CODE@** Tool used for measuring
    /// sme: (no comment)
    /// vro: **@CODE@** Tool used for measuring
    #[strum(serialize = "Sem/Tool-measr")]
    Sem_SLASH_Tool_MINUS_measr,
    /// kpv: **@CODE@** Music instrument
    /// liv: **@CODE@** Music instrument
    /// mhr: **@CODE@** = Music instrument
    /// sma: (no comment)
    /// smj: **@CODE@** = Musical instrument; shaman drum, guitar, violin, musical instrument, jaw harp
    /// smj: **@CODE@** =
    /// yrk: **@CODE@** Music instrument
    /// sms: **@CODE@** = Music instrument
    /// mrj: **@CODE@** Music instrument
    /// olo: **@CODE@** Music instrument
    /// sme: (no comment)
    /// vro: **@CODE@** Music instrument
    #[strum(serialize = "Sem/Tool-music")]
    Sem_SLASH_Tool_MINUS_music,
    /// kpv: **@CODE@** Writing tool
    /// liv: **@CODE@** Writing tool
    /// mhr: **@CODE@** = Writing tool
    /// sma: (no comment)
    /// smj: **@CODE@** = Writing tool; colored pen, pencil, chalk, paintbrush, paint
    /// yrk: **@CODE@** Writing tool
    /// sms: **@CODE@** = Writing tool
    /// mrj: **@CODE@** Writing tool
    /// olo: **@CODE@** Writing tool
    /// sme: (no comment)
    /// vro: **@CODE@** Writing tool
    #[strum(serialize = "Sem/Tool-write")]
    Sem_SLASH_Tool_MINUS_write,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Text (girji, lávlla...)
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Text (girji, lávlla...)
    /// mhr: **@CODE@** = Text (girji, lávlla...)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Written document; paper, book, letter, e-mail, song
    /// yrk: **@CODE@** Text (girji, lávlla...)
    /// sms: **@CODE@** = Text (girji, lávlla...)
    /// mrj: **@CODE@** Text (girji, lávlla...)
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Text (girji, lávlla...)
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Text (girji, lávlla...)
    #[strum(serialize = "Sem/Txt")]
    Sem_SLASH_Txt,
    /// fit: **@CODE@**
    /// kpv: **@CODE@** Vehicle
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// liv: **@CODE@** Vehicle
    /// mhr: **@CODE@** = Vehicle
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Vehicle; car, boat, sled, vehicle, draft reindeer, bicycle
    /// yrk: **@CODE@** Vehicle
    /// sms: **@CODE@** = Vehicle
    /// mrj: **@CODE@** Vehicle
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Vehicle
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** Vehicle
    #[strum(serialize = "Sem/Veh")]
    Sem_SLASH_Veh,
    /// kpv: **@CODE@** Weapon
    /// liv: **@CODE@** Weapon
    /// mhr: **@CODE@** = Weapon
    /// sma: (no comment)
    /// smj: **@CODE@** = Weapon; rifle, bow, sword, arrow, war axe
    /// yrk: **@CODE@** Weapon
    /// sms: **@CODE@** = Weapon
    /// mrj: **@CODE@** Weapon
    /// olo: **@CODE@** Weapon
    /// sme: (no comment)
    /// vro: **@CODE@** Weapon
    #[strum(serialize = "Sem/Wpn")]
    Sem_SLASH_Wpn,
    /// kpv: **@CODE@** The Weather or the state of ground
    /// vot: (no comment)
    /// liv: **@CODE@** The Weather or the state of ground
    /// mhr: **@CODE@** = The Weather or the state of ground
    /// sma: (no comment)
    /// smj: **@CODE@** = The Weather or the state of ground; cloudy weather, wind, driving conditions, night sunlight, rain shower
    /// yrk: **@CODE@** The Weather or the state of ground
    /// sms: **@CODE@** = The Weather or the state of ground
    /// mrj: **@CODE@** The Weather or the state of ground
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** The Weather or the state of ground
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@** The Weather or the state of ground
    #[strum(serialize = "Sem/Wthr")]
    Sem_SLASH_Wthr,
    /// fit: **@CODE@**
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Year
    /// sms: **@CODE@** = Year
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Year")]
    Sem_SLASH_Year,
    /// smj: (no comment)
    #[strum(serialize = "SentInit")]
    SentInit,
    /// fit: **@CODE@** = Singular
    /// kpv: **@CODE@**  singular
    /// vot: (no comment)
    /// fkv: **@CODE@** = Singular
    /// liv: **@CODE@** = singular
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** = Singular
    /// nob: **@CODE@** =
    /// smj: **@CODE@** = Singular number
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Singular = õõutilååkk
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**  = singular
    /// zxx: (no comment)
    /// olo: **@CODE@**  singular
    /// sme: **@CODE@** - Singular
    /// fin: (no comment)
    /// ceb: (no comment)
    /// ces: (no comment)
    /// vro: **@CODE@** Singular
    #[strum(serialize = "Sg")]
    Sg,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@** person тэа-меа
    /// vot: (no comment)
    /// fkv: **@CODE@** = Singular 1
    /// liv: **@CODE@** = first person singular conjugation
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** | Singular, 1.person
    /// nob: **@CODE@** =
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** first person singular
    /// lit: (no comment)
    /// mrj: **@CODE@** = Verb personal forms
    /// vep: **@CODE@** Singular First Person
    /// zxx: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@** Singular First Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// ces: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sg1")]
    Sg1,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Singular 2
    /// liv: **@CODE@** = second person singular conjugation
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** | Singular, 2.person
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** second person singular
    /// vep: **@CODE@** Singular Second Person
    /// sme: **@CODE@** Singular Second Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sg2")]
    Sg2,
    /// fit: **@CODE@** =
    /// vot: (no comment)
    /// fkv: **@CODE@** = Singular 3
    /// liv: **@CODE@** = third person singular conjugation
    /// mhr: **@CODE@** =
    /// sma: **@CODE@** | Singular, 3.person
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** third person singular
    /// vep: **@CODE@** Singular Third Person
    /// sme: **@CODE@** Singular Third Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sg3")]
    Sg3,
    /// sms: **@CODE@** impersonal fourth person, in MT a Pl4 might be needed
    #[strum(serialize = "Sg4")]
    Sg4,
    /// smj: **@CODE@**  Short form
    #[strum(serialize = "Sh")]
    Sh,
    /// rus: (no comment)
    #[strum(serialize = "Sint")]
    Sint,
    /// kpv: **@CODE@** segment ordering: case, person
    /// mhr: **@CODE@** = Suffix ordering: Case + Possessive Person marking
    /// mrj: **@CODE@** = Suffix ordering: Case + Possessive Person marking
    #[strum(serialize = "So/CP")]
    So_SLASH_CP,
    /// mhr: **@CODE@** = Suffix ordering: Number + Case + Possessive Person marking
    /// mrj: **@CODE@** = Suffix ordering: Number + Case + Possessive Person marking
    #[strum(serialize = "So/NCP")]
    So_SLASH_NCP,
    /// mhr: **@CODE@** = Suffix ordering: Number + Possessive Person marking
    /// mrj: **@CODE@** = Suffix ordering: Number + Possessive Person marking
    #[strum(serialize = "So/NP")]
    So_SLASH_NP,
    /// mhr: **@CODE@** = Suffix ordering: Number + Possessive Person + Case marking
    /// mrj: **@CODE@** = Suffix ordering: Number + Possessive Person + Case marking
    #[strum(serialize = "So/NPC")]
    So_SLASH_NPC,
    /// kpv: **@CODE@** segment ordering: person, case
    /// mhr: **@CODE@** = Suffix ordering: Possessive Person + Case marking
    /// mrj: **@CODE@** = Suffix ordering: Possessive Person + Case marking
    #[strum(serialize = "So/PC")]
    So_SLASH_PC,
    /// mhr: **@CODE@** = Suffix ordering: Possessive Person + Number marking
    /// mrj: **@CODE@** = Suffix ordering: Possessive Person + Number marking
    #[strum(serialize = "So/PN")]
    So_SLASH_PN,
    /// mhr: **@CODE@** = Suffix ordering: Possessive Person + Number + Case marking
    /// mrj: **@CODE@** = Suffix ordering: Possessive Person + Number + Case marking
    #[strum(serialize = "So/PNC")]
    So_SLASH_PNC,
    /// ceb: **@CODE@** - Social
    #[strum(serialize = "Soc")]
    Soc,
    /// sms: **@CODE@**
    /// sme: **@CODE@** foreløpig lagt til Sg Loc -n, som er en sub-form
    #[strum(serialize = "South")]
    South,
    /// fit: **@CODE@** = ?
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    /// smj: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    #[strum(serialize = "Span")]
    Span,
    /// kpv: **@CODE@** spatial
    /// vot: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    #[strum(serialize = "Spat")]
    Spat,
    /// fin: (no comment)
    #[strum(serialize = "Spell/NoSugg")]
    Spell_SLASH_NoSugg,
    /// kpv: **@CODE@** foreign source apparently 2015-09-08
    #[strum(serialize = "Src/F")]
    Src_SLASH_F,
    /// vot: (no comment)
    #[strum(serialize = "State")]
    State,
    /// yrk: **@CODE@**
    #[strum(serialize = "Subord")]
    Subord,
    /// sma: **@CODE@** - used for adverbs
    /// sme: **@CODE@**   Embedded Question Particle: +Adv+Subqst
    #[strum(serialize = "Subqst")]
    Subqst,
    /// fin: (no comment)
    #[strum(serialize = "Suff")]
    Suff,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// olo: **@CODE@**
    /// sme: **@CODE@** Supine
    /// vro: **@CODE@** olõman, olõmaldaq; oldama
    #[strum(serialize = "Sup")]
    Sup,
    /// fit: **@CODE@** =
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Superlative
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = superlative
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Superl")]
    Superl,
    /// fit: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// kpv: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// vot: (no comment)
    /// fkv: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// liv: (no comment)
    /// mhr: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// sma: **@CODE@**:  Symbols in the text stream, like £, €, ©
    /// nob: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// smj: (no comment)
    /// rus: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// lit: (no comment)
    /// mrj: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// vep: **@CODE@**  = independent symbols in the text stream, like £, €, ©
    /// zxx: (no comment)
    /// olo: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// sme: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// fin: (no comment)
    /// ceb: (no comment)
    /// ces: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Symbol")]
    Symbol,
    /// smj: (no comment)
    #[strum(serialize = "TEL")]
    TEL,
    /// vot: (no comment)
    #[strum(serialize = "TEMP")]
    TEMP,
    /// mhr: **@CODE@** =
    #[strum(serialize = "TEST")]
    TEST,
    /// sma: **@CODE@** = Code for items that have not been modeled yet
    /// smj: **@CODE@** = Code for items that have not been modeled yet
    #[strum(serialize = "TODO")]
    TODO,
    /// sme: (no comment)
    #[strum(serialize = "TRAB")]
    TRAB,
    /// sme: (no comment)
    #[strum(serialize = "TRNUM")]
    TRNUM,
    /// fit: **@CODE@** transitive
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Transitive verb
    /// liv: **@CODE@** = Transitive verb
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  Transitive verb
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Transitive
    /// lit: (no comment)
    /// mrj: **@CODE@** =  to transitivity:
    /// vep: **@CODE@** = transitive and
    /// zxx: (no comment)
    /// olo: **@CODE@**
    /// sme: **@CODE@**    Transitive Verb, +V+TV
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "TV")]
    TV,
    /// vep: **@CODE@**
    /// olo: **@CODE@** This alerts workers of work to be done
    #[strum(serialize = "TYÄ")]
    TYÄ,
    /// kpv: **@CODE@** temporal
    /// vot: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// vep: (no comment)
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Temp")]
    Temp,
    /// fin: (no comment)
    #[strum(serialize = "Tempr")]
    Tempr,
    /// kpv: **@CODE@** Terminative -ӧдз матыстчан
    /// vot: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Terminative
    /// vro: **@CODE@** terminative
    #[strum(serialize = "Ter")]
    Ter,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Ter1")]
    Ter1,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Ter2")]
    Ter2,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Ter3")]
    Ter3,
    /// sma: **@CODE@**:  form uses consonant cluster gk
    #[strum(serialize = "Test/GK")]
    Test_SLASH_GK,
    /// sma: **@CODE@**:  form uses consonant cluster hk
    #[strum(serialize = "Test/HK")]
    Test_SLASH_HK,
    /// sma: **@CODE@**:  form uses consonant cluster jk
    #[strum(serialize = "Test/JK")]
    Test_SLASH_JK,
    /// sma: **@CODE@**:  form uses i
    #[strum(serialize = "Test/LysI")]
    Test_SLASH_LysI,
    /// sma: **@CODE@**:  form uses ï
    #[strum(serialize = "Test/MørkI")]
    Test_SLASH_MørkI,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Time")]
    Time,
    /// kpv: **@CODE@**
    #[strum(serialize = "Tot")]
    Tot,
    /// fit: **@CODE@** = Translaive
    /// kpv: **@CODE@** translative -ті вуджан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Translaive
    /// liv: **@CODE@** = translative
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  = translative
    /// olo: **@CODE@** translative
    /// fin: (no comment)
    /// vro: **@CODE@** translative
    #[strum(serialize = "Tra")]
    Tra,
    /// fin: (no comment)
    #[strum(serialize = "TruncPrefix")]
    TruncPrefix,
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Txt")]
    Txt,
    /// kpv: (no comment)
    #[strum(serialize = "URL")]
    URL,
    /// sma: **@CODE@**:  A tag to indicate realised or potential Umlaut
    #[strum(serialize = "Uml")]
    Uml,
    /// fit: **@CODE@** = ?
    /// kpv: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// fkv: **@CODE@** never retained in the HFST Grammar Checker disambiguation analyser
    /// sma: **@CODE@** | **never** retained in the HFST Grammar Checker disambiguation analyser
    /// nob: **@CODE@** never retained in the HFST Grammar Checker disambiguation analyser
    /// smj: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// sms: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// vep: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// olo: **@CODE@**  (?)
    /// sme: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    #[strum(serialize = "Use/-GC")]
    Use_SLASH__MINUS_GC,
    /// fkv: **@CODE@** =
    /// mhr: **@CODE@** Excluded in PLX-speller
    /// sma: **@CODE@** | Excluded in PLX speller
    /// nob: **@CODE@**
    /// smj: **@CODE@** – Excluded from PLX speller
    /// vep: **@CODE@** Excluded in PLX-speller
    /// sme: **@CODE@** Excluded in PLX-speller
    #[strum(serialize = "Use/-PLX")]
    Use_SLASH__MINUS_PLX,
    /// fit: (no comment)
    /// kpv: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// fkv: **@CODE@** = for tokenisation
    /// mhr: **@CODE@** Do not include in fsts made for hfst-pmatch
    /// sma: **@CODE@** | Do not include in fst's made for hfst-pmatch
    /// nob: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// nob: **@CODE@**
    /// smj: **@CODE@** – Do not include in fst's made for hfst-pmatch
    /// sms: **@CODE@** -
    /// vep: **@CODE@**
    /// vep: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// olo: (no comment)
    /// sme: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// fin: (no comment)
    #[strum(serialize = "Use/-PMatch")]
    Use_SLASH__MINUS_PMatch,
    /// fit: **@CODE@** = Excluded in speller
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Excluded in speller
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = accepted in normative FST but not in speller
    /// sma: **@CODE@** | Excluded from speller
    /// nob: **@CODE@**
    /// smj: **@CODE@** – Excluded from speller
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Excluded in speller
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@** Orthographically correct, typically perifer words, excluded in speller because they cause trouble for frequent words
    /// zxx: (no comment)
    /// olo: **@CODE@** Excluded in speller
    /// sme: **@CODE@** Orthographically correct, typically perifer words, excluded in speller because they cause trouble for frequent words
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Use/-Spell")]
    Use_SLASH__MINUS_Spell,
    /// fit: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fkv: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mhr: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sma: **@CODE@** | **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// nob: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// smj: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sms: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vep: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// olo: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sme: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fin: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vro: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    #[strum(serialize = "Use/-TTS")]
    Use_SLASH__MINUS_TTS,
    /// nob: **@CODE@**
    #[strum(serialize = "Use/-TTS-")]
    Use_SLASH__MINUS_TTS_MINUS_,
    /// nob: **@CODE@**
    #[strum(serialize = "Use/-TTS–")]
    Use_SLASH__MINUS_TTS_EMDASH_,
    /// rus: (no comment)
    #[strum(serialize = "Use/Ant")]
    Use_SLASH_Ant,
    /// fin: (no comment)
    #[strum(serialize = "Use/Arch")]
    Use_SLASH_Arch,
    /// fit: (no comment)
    /// fkv: **@CODE@** for numerals, copied from sme
    /// mhr: **@CODE@** circular paths (old ^C^)
    /// sma: **@CODE@** | Circular path
    /// nob: **@CODE@** = circular string
    /// smj: **@CODE@** – Circular path
    /// sms: **@CODE@** circular paths (old ^C^)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// vep: **@CODE@** circular paths (old ^C^)
    /// olo: **@CODE@** circular paths (old ^C^)
    /// sme: **@CODE@** circular paths (old ^C^)
    /// fin: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Use/Circ")]
    Use_SLASH_Circ,
    /// vro: **@CODE@**
    #[strum(serialize = "Use/Circ100")]
    Use_SLASH_Circ100,
    /// mhr: **@CODE@** circular paths for the numerals (old ^N^)
    /// sma: **@CODE@** | Circular number path?
    /// sma: (no comment)
    /// smj: **@CODE@** – Circular number path
    /// smj: (no comment)
    /// sms: **@CODE@** circular paths for the numerals (old ^N^)
    /// vep: **@CODE@** circular paths for the numerals (old ^N^)
    /// olo: **@CODE@** circular paths for the numerals (old ^N^)
    /// sme: **@CODE@** circular paths for the numerals (old ^N^)
    #[strum(serialize = "Use/CircN")]
    Use_SLASH_CircN,
    /// sma: **@CODE@** | For words without formal normalization. Divvun suggest that this should be normative. Included in speller. Based on 2010 normative decision & Ove Lorentz' suggestions for the norm.
    /// smj: **@CODE@** – For words without formal normalization.  Divvun suggest that this should be normative. Included in speller.
    /// sme: (no comment)
    #[strum(serialize = "Use/DNorm")]
    Use_SLASH_DNorm,
    /// smj: **@CODE@** – Derrogatory word. Recognized, but not suggested in speller, same as SpellNoSugg
    #[strum(serialize = "Use/Derrog")]
    Use_SLASH_Derrog,
    /// mhr: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    #[strum(serialize = "Use/Disamb")]
    Use_SLASH_Disamb,
    /// fit: (no comment)
    /// kpv: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// fkv: **@CODE@** only retained in the HFST Grammar Checker disambiguation analyser
    /// mhr: **@CODE@** only retained in the HFST Grammar Checker disambiguation analyser
    /// sma: **@CODE@** | **only** retained in the HFST Grammar Checker disambiguation analyser
    /// nob: **@CODE@** only retained in the HFST Grammar Checker disambiguation analyser
    /// smj: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// sms: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// vep: **@CODE@** ??? typo?, occurs once.
    /// vep: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// olo: (no comment)
    /// sme: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// sme: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Use/GC")]
    Use_SLASH_GC,
    /// fin: (no comment)
    #[strum(serialize = "Use/Hyphen")]
    Use_SLASH_Hyphen,
    /// sma: (no comment)
    /// smj: (no comment)
    /// sme: **@CODE@** only for LIA-analyser
    #[strum(serialize = "Use/LIA")]
    Use_SLASH_LIA,
    /// mhr: **@CODE@** Generate for MT only, for restricting analyses needed
    /// sma: **@CODE@** | Generate for apertium only
    /// smj: **@CODE@** – Generate for MT only, for restricting analyses needed
    /// vep: **@CODE@** Generate for MT only, for restricting analyses needed
    /// sme: **@CODE@** Generate for MT only, for restricting analyses needed
    #[strum(serialize = "Use/MT")]
    Use_SLASH_MT,
    /// mhr: **@CODE@** marginal
    /// sma: **@CODE@** | Marginal, korrekte, eksisterende former, men som er sjeldne. vi kan fjerne disse ordene f.eks fra speller, fordi de er så sjeldne og lite i bruk at de lemma som ligger nært kan bli forvekslet.
    /// smj: **@CODE@** – Marginal, but normative lemmas. Not in speller.
    /// sms: **@CODE@** marginal (?)
    /// olo: **@CODE@** marginal (?)
    /// fin: (no comment)
    #[strum(serialize = "Use/Marg")]
    Use_SLASH_Marg,
    /// fin: (no comment)
    #[strum(serialize = "Use/N")]
    Use_SLASH_N,
    /// fit: (no comment)
    /// kpv: **@CODE@** This is used for minimizing excess generation in apertium kpv-koi
    /// vot: (no comment)
    /// fkv: **@CODE@** only accept, not generate (for MT and Oahpa use)
    /// liv: **@CODE@** no generation
    /// mhr: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// sma: **@CODE@** | Do not generate <br/> for isme-ped.fst and apertium
    /// nob: **@CODE@**
    /// nob: **@CODE@** not-generate, for ped generation isme-ped.fst and MT
    /// smj: **@CODE@** – Do not generate, only for Oahpa and MT. In speller.
    /// rus: (no comment)
    /// sms: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// mrj: **@CODE@** =
    /// vep: **@CODE@** do not generate
    /// vep: **@CODE@** not-generate, for ped generation isme-ped.fst and MT
    /// olo: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// sme: **@CODE@** not-generate, for ped generation isme-ped.fst and MT
    /// fin: (no comment)
    /// vro: **@CODE@** No generation
    /// vro: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Use/NG")]
    Use_SLASH_NG,
    /// mhr: **@CODE@** Not for miniparadigm in VD dicts
    /// smj: **@CODE@** – Not for miniparadigm in VD dicts
    /// sms: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// vep: **@CODE@** Not for miniparadigm in NDS dicts
    /// sme: **@CODE@** Not for miniparadigm in NDS dicts
    #[strum(serialize = "Use/NGminip")]
    Use_SLASH_NGminip,
    /// fin: (no comment)
    #[strum(serialize = "Use/NoHyphens")]
    Use_SLASH_NoHyphens,
    /// sma: **@CODE@** | For (spellings of) words that do not follow the orthographic principles of sma. Divvun suggest that this shouldn't be normative, even though they are decided upon by GG. Included in speller.
    /// smj: **@CODE@** – For words without formal normalization. Divvun suggest that this shouldn't be normative.
    /// sme: (no comment)
    #[strum(serialize = "Use/NotDNorm")]
    Use_SLASH_NotDNorm,
    /// rus: (no comment)
    #[strum(serialize = "Use/Obs")]
    Use_SLASH_Obs,
    /// fit: (no comment)
    /// kpv: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// fkv: **@CODE@** = for tokenisation
    /// sma: **@CODE@** | Do *only* include in fst's for hfst-pmatch
    /// nob: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// nob: **@CODE@**
    /// smj: **@CODE@** – Only use in fst's targeted for `hfst-pmatch`
    /// sms: **@CODE@** -
    /// vep: **@CODE@**
    /// vep: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// olo: (no comment)
    /// sme: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// fin: **@CODE@** means that the following is only used in the analyser feeding the disambiguator. This is missing.
    /// vro: **@CODE@**
    #[strum(serialize = "Use/PMatch")]
    Use_SLASH_PMatch,
    /// sma: **@CODE@** | Remove from pedagogical speller
    /// smj: **@CODE@** – Remove from pedagogical speller
    /// sms: **@CODE@** for use with pedagogical work
    #[strum(serialize = "Use/Ped")]
    Use_SLASH_Ped,
    /// fin: (no comment)
    #[strum(serialize = "Use/Rare")]
    Use_SLASH_Rare,
    /// fit: **@CODE@** = recognized but not suggested in speller
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = recognized but not suggested in speller
    /// mhr: **@CODE@** recognized but not suggested in speller
    /// sma: **@CODE@** | Recognized but not suggested in speller
    /// nob: **@CODE@**
    /// smj: **@CODE@** – Recognized, but not suggested in speller
    /// smj: (no comment)
    /// yrk: **@CODE@** recognized but not suggested in speller
    /// sms: **@CODE@** recognized but not suggested in speller
    /// vep: **@CODE@**
    /// vep: **@CODE@** recognized but not suggested in speller
    /// olo: **@CODE@** recognized but not suggested in speller
    /// sme: **@CODE@** recognized but not suggested in speller
    /// sme: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@** not suggested in speller
    #[strum(serialize = "Use/SpellNoSugg")]
    Use_SLASH_SpellNoSugg,
    /// fit: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fkv: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mhr: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sma: **@CODE@** | **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// nob: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// smj: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sms: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vep: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vep: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// olo: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sme: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fin: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fin: (no comment)
    /// vro: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    #[strum(serialize = "Use/TTS")]
    Use_SLASH_TTS,
    /// mhr: **@CODE@** =  Dealing with lative form 2012-10-27 аваеш, пашаш
    #[strum(serialize = "Use/Test")]
    Use_SLASH_Test,
    /// fin: (no comment)
    #[strum(serialize = "Use/sub")]
    Use_SLASH_sub,
    /// fit: **@CODE@** = Verb
    /// kpv: **@CODE@**:  verb      кадакыв   глагол
    /// vot: (no comment)
    /// fkv: **@CODE@** = Verb
    /// liv: **@CODE@** = verb
    /// mhr: **@CODE@** = verbs
    /// sma: **@CODE@** = Verb
    /// smj: **@CODE@** = Verb
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Verb
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**  = verb
    /// olo: **@CODE@**  verb
    /// sme: **@CODE@** - Verb
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Verb
    /// vro: **@CODE@** Verb
    #[strum(serialize = "V")]
    V,
    /// kpv: **@CODE@** тӧм Participle verbal adjective, see also Der/Abe
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Verb Abessive
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// sme: **@CODE@** VerbAbbesive
    /// vro: **@CODE@**
    #[strum(serialize = "VAbess")]
    VAbess,
    /// kpv: **@CODE@** тӧг Gerund
    #[strum(serialize = "VCar")]
    VCar,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Verbgenitive
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**
    /// sme: **@CODE@** VerbGenitive
    /// vro: **@CODE@**
    #[strum(serialize = "VGen")]
    VGen,
    /// kpv: **@CODE@** тӧдз Gerund
    #[strum(serialize = "VTer")]
    VTer,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Veh")]
    Veh,
    /// kpv: **@CODE@** Vocative ??
    /// liv: **@CODE@** = Vocative
    /// mhr: **@CODE@** = vocative
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = Vocative
    #[strum(serialize = "Voc")]
    Voc,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "V→A")]
    V_A,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// zxx: (no comment)
    /// olo: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "V→N")]
    V_N,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// yrk: **@CODE@**
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "V→V")]
    V_V,
    /// kpv: **@CODE@** to mark intermediate solutions
    /// mhr: **@CODE@** = nouns
    /// yrk: **@CODE@** WORK HAS TO BE DONE Do not remove, replaces +TYÄ
    /// mrj: **@CODE@** = Error?
    /// vro: **@CODE@** (eng) work needed
    #[strum(serialize = "WORK")]
    WORK,
    /// mhr: **@CODE@** = weak (?) form
    #[strum(serialize = "Weak")]
    Weak,
    /// lit: (no comment)
    /// zxx: (no comment)
    #[strum(serialize = "Wthr")]
    Wthr,
    /// nob: **@CODE@**  denoting not-checked.
    #[strum(serialize = "X")]
    X,
    /// kpv: **@CODE@**:  Zero collective кодныс
    #[strum(serialize = "ZeroColl")]
    ZeroColl,
    /// fin: (no comment)
    #[strum(serialize = "s")]
    s,
    /// fit: **@CODE@**
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// fkv: **@CODE@**:  variant 1
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 1
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: **@CODE@** version tags
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v1")]
    v1,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v10")]
    v10,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v11")]
    v11,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v12")]
    v12,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v13")]
    v13,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v14")]
    v14,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v15")]
    v15,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v16")]
    v16,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v17")]
    v17,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v18")]
    v18,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v19")]
    v19,
    /// fit: **@CODE@**
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// fkv: **@CODE@**:  variant 2
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 2
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: **@CODE@** version tags
    /// sme: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v2")]
    v2,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v20")]
    v20,
    /// kpv: **@CODE@**
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v21")]
    v21,
    /// kpv: **@CODE@**
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v22")]
    v22,
    /// kpv: **@CODE@**
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v23")]
    v23,
    /// kpv: **@CODE@**
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v24")]
    v24,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 3
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 3
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: **@CODE@** version tags
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v3")]
    v3,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 4
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 4
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// olo: **@CODE@** version tags
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v4")]
    v4,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 5
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 5
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// olo: **@CODE@** version tags
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v5")]
    v5,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 6
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 6
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v6")]
    v6,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 7
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 7
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v7")]
    v7,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v8")]
    v8,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v9")]
    v9,
}

impl Tag {
    pub fn is_pos(&self) -> bool {
        use Tag::{A, ABBR, Adp, CC, CLB, CS, Det, Interj, N, Num, Pcle, Po, Pr, Pron, URL, V};
        matches!(
            self,
            A | ABBR
                | Adp
                | CC
                | CLB
                | CS
                | Det
                | Interj
                | URL
                | N
                | Num
                | Pcle
                | Po
                | Pr
                | Pron
                | V
        )
    }

    /// Get the `&str` representation of this tag
    pub fn as_str(&self) -> &str {
        // strum's AsRefStr gives us this
        self.as_ref()
    }

    pub fn is_sem(&self) -> bool {
        self.as_str().starts_with("Sem/")
    }

    pub fn is_err(&self) -> bool {
        self.as_str().starts_with("Err/")
    }

    pub fn is_der(&self) -> bool {
        self.as_str().starts_with("Der/")
    }

    /// If the tag is `Err/xxx`, get a `&str` to the `xxx` part, or `None` if it is not
    /// an `Err/` tag.
    pub fn err(&self) -> Option<&str> {
        self.as_str().strip_prefix("Err/")
    }

    /// Helper function to see check if `self` is `Err/xxx`, and if so, if the `xxx`
    /// starts with the given `prefix`. Returns `None` if `self` is not `Err/xxx`.
    #[deprecated(
        since = "0.2.0",
        note = "use self.err().map(|error| error.starts_with(prefix)) instead"
    )]
    pub fn is_err_starts_with(&self, prefix: &str) -> Option<bool> {
        self.err().map(|err| err.starts_with(prefix))
    }

    /// If the tag is `Sem/xxx`, get a `&str` to the `xxx` part, or `None` if it is not
    /// a `Sem/` tag.
    pub fn sem(&self) -> Option<&str> {
        self.as_str().strip_prefix("Sem/")
    }

    pub fn is_subclass(&self) -> bool {
        use Tag::{Dem, G3, G7, Indef, Interr, Neg, NomAg, Ord, Pers, Prop, Qu, Recipr, Refl, Rel};
        matches!(
            self,
            Neg | Prop
                | G3
                | G7
                | NomAg
                | Ord
                | Pers
                | Rel
                | Interr
                | Dem
                | Indef
                | Refl
                | Recipr
                | Qu
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(s: &str, tag: Tag) {
        assert!(Tag::try_from(s).is_ok_and(|t| t == tag));
    }

    #[test]
    fn test_poses() {
        assert!(Tag::try_from("N").is_ok_and(|tag| tag == Tag::N));
    }

    /// Tests all the previously known tags, the tags that were available in version 0.1.x.
    #[test]
    fn test_previously_known_tags() {
        // note: "Unknown" is not a tag in this new version, because all are known.
        // note: the "variant" tags (v1, v2, v3, ...) are lower cased as of 0.2
        t("Abe", Tag::Abe);
        t("Abl", Tag::Abl);
        t("Acc", Tag::Acc);
        t("Acr", Tag::Acr);
        t("Actio", Tag::Actio);
        t("Ade", Tag::Ade);
        t("All", Tag::All);
        t("Allegro", Tag::Allegro);
        t("Arab", Tag::Arab);
        t("Attr", Tag::Attr);
        t("Card", Tag::Card);
        t("Cmp", Tag::Cmp);
        t("Cmp#", Tag::Cmp_POUND);
        t("Cmp/Attr", Tag::Cmp_SLASH_Attr);
        t("Com", Tag::Com);
        t("ConNeg", Tag::ConNeg);
        t("ConNegII", Tag::ConNegII);
        t("Cond", Tag::Cond);
        t("Dem", Tag::Dem);
        t("Der/A", Tag::Der_SLASH_A);
        t("Du", Tag::Du);
        t("Du1", Tag::Du1);
        t("Du2", Tag::Du2);
        t("Du3", Tag::Du3);
        t("Ela", Tag::Ela);
        t("Err/Hyph", Tag::Err_SLASH_Hyph);
        t("Ess", Tag::Ess);
        t("Ex/A", Tag::Ex_SLASH_A);
        t("Foc", Tag::Foc);
        t("Foc/AA", Tag::Foc_SLASH_AA);
        t("G3", Tag::G3);
        t("G7", Tag::G7);
        t("Gen", Tag::Gen);
        t("Ger", Tag::Ger);
        t("Gram/3syll", Tag::Gram_SLASH_3syll);
        t("Hom1", Tag::Hom1);
        t("Hom2", Tag::Hom2);
        t("Hom3", Tag::Hom3);
        t("IV", Tag::IV);
        t("Ill", Tag::Ill);
        t("Imprt", Tag::Imprt);
        t("ImprtII", Tag::ImprtII);
        t("Ind", Tag::Ind);
        t("Indef", Tag::Indef);
        t("Ine", Tag::Ine);
        t("Inf", Tag::Inf);
        t("Interr", Tag::Interr);
        t("Loc", Tag::Loc);
        t("Neg", Tag::Neg);

        // NOT FOUND
        //t("Neu", Tag::Neu);
        t("Nom", Tag::Nom);
        t("NomAg", Tag::NomAg);
        t("Ord", Tag::Ord);
        t("Par", Tag::Par);
        t("Pers", Tag::Pers);
        t("Pl", Tag::Pl);
        t("Pl1", Tag::Pl1);
        t("Pl2", Tag::Pl2);
        t("Pl3", Tag::Pl3);
        t("Pot", Tag::Pot);
        t("PrfPrc", Tag::PrfPrc);
        t("Prop", Tag::Prop);
        t("Prs", Tag::Prs);
        t("PrsPrc", Tag::PrsPrc);
        t("Prt", Tag::Prt);
        t("PxDu1", Tag::PxDu1);
        t("PxDu2", Tag::PxDu2);
        t("PxDu3", Tag::PxDu3);
        t("PxPl1", Tag::PxPl1);
        t("PxPl2", Tag::PxPl2);
        t("PxPl3", Tag::PxPl3);
        t("PxSg1", Tag::PxSg1);
        t("PxSg2", Tag::PxSg2);
        t("PxSg3", Tag::PxSg3);
        t("Qst", Tag::Qst);
        t("Qu", Tag::Qu);
        t("Recipr", Tag::Recipr);
        t("Refl", Tag::Refl);
        t("Rel", Tag::Rel);
        t("Sem/Act_Hum", Tag::Sem_SLASH_Act_Hum);
        t("Sg", Tag::Sg);
        t("Sg1", Tag::Sg1);
        t("Sg2", Tag::Sg2);
        t("Sg3", Tag::Sg3);
        t("South", Tag::South);
        t("Sup", Tag::Sup);
        t("TV", Tag::TV);
        t("v1", Tag::v1);
        t("v2", Tag::v2);
        t("v3", Tag::v3);
        t("v4", Tag::v4);
        t("v5", Tag::v5);
        t("v6", Tag::v6);
        t("v7", Tag::v7);
        t("v8", Tag::v8);
        t("v9", Tag::v9);
        t("v10", Tag::v10);
        t("v11", Tag::v11);
        t("v12", Tag::v12);
        t("v13", Tag::v13);
        t("v14", Tag::v14);
        t("v15", Tag::v15);
        t("v16", Tag::v16);
        t("v17", Tag::v17);
        t("v18", Tag::v18);
        t("v19", Tag::v19);
        t("v20", Tag::v20);
        t("VAbess", Tag::VAbess);
        t("VGen", Tag::VGen);
    }
}
