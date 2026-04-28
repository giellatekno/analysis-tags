#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

/// Error type returned from `Tag::try_from` if no tag is matched.
#[derive(Debug)]
pub struct UnknownTagError(String);

impl ::std::fmt::Display for UnknownTagError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ::std::error::Error for UnknownTagError {}

/// Initializer for the `UnknownTagError` error, which sets the offending string.
pub fn unknown_tag(s: &str) -> UnknownTagError {
    UnknownTagError(s.to_string())
}

/// An fst Tag. Every single possible tag in our infrastructure is its own
/// variant.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, AsRefStr, EnumString, Serialize, Deserialize)]
#[strum(
    parse_err_fn = unknown_tag,
    parse_err_ty = UnknownTagError
)]
pub enum Tag {
    /// cwd: (no comment)
    #[strum(serialize = "0Sg/Pl")]
    _0Sg_SLASH_Pl,
    /// kpv: **@CODE@**  Final мед ог _so that I/we won't_ 2019-04-06
    /// swe: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// nob: **@CODE@**  not in use??
    /// oji: (no comment)
    /// qya: (no comment)
    /// otw: (no comment)
    /// nso: (no comment)
    /// mya: (no comment)
    /// non: (no comment)
    #[strum(serialize = "1")]
    _1,
    /// xak: (no comment)
    #[strum(serialize = "123PlObj")]
    _123PlObj,
    /// xak: (no comment)
    #[strum(serialize = "123PlSubj")]
    _123PlSubj,
    /// xak: (no comment)
    #[strum(serialize = "12DlObj")]
    _12DlObj,
    /// xak: (no comment)
    #[strum(serialize = "12DlSubj")]
    _12DlSubj,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "12Pl")]
    _12Pl,
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "12PlO")]
    _12PlO,
    /// xak: (no comment)
    #[strum(serialize = "12PlObj")]
    _12PlObj,
    /// xak: (no comment)
    #[strum(serialize = "12PlSubj")]
    _12PlSubj,
    /// sms: **@CODE@** used in combination with +Imp+13+ConNeg
    #[strum(serialize = "13")]
    _13,
    /// xak: (no comment)
    #[strum(serialize = "13DlObj")]
    _13DlObj,
    /// xak: (no comment)
    #[strum(serialize = "13DlSubj")]
    _13DlSubj,
    /// xak: (no comment)
    #[strum(serialize = "13PlObj")]
    _13PlObj,
    /// xak: (no comment)
    #[strum(serialize = "13PlSubj")]
    _13PlSubj,
    /// kal: **@CODE@** = Subject 1.person dual
    /// ess: (no comment)
    /// non: (no comment)
    /// ipk: (no comment)
    /// mpj: **@CODE@**
    #[strum(serialize = "1Du")]
    _1Du,
    /// ess: (no comment)
    /// ipk: (no comment)
    #[strum(serialize = "1DuO")]
    _1DuO,
    /// gle: **@CODE@** = first person inflection
    /// amh: (no comment)
    /// amh: (no comment)
    #[strum(serialize = "1P")]
    _1P,
    /// cho: (no comment)
    #[strum(serialize = "1Pauc")]
    _1Pauc,
    /// kal: **@CODE@** = Subject 1.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// ess: (no comment)
    /// som: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// ipk: (no comment)
    /// mpj: **@CODE@**
    #[strum(serialize = "1Pl")]
    _1Pl,
    /// kal: **@CODE@** = Object 1.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// ess: (no comment)
    /// kio: (no comment)
    /// ipk: (no comment)
    #[strum(serialize = "1PlO")]
    _1PlO,
    /// kal: **@CODE@** = Possessor 1.person plural
    #[strum(serialize = "1PlPoss")]
    _1PlPoss,
    /// kal: **@CODE@** = Subject 1.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// crj: (no comment)
    /// cor: (no comment)
    /// tku: (no comment)
    /// ess: (no comment)
    /// som: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// ipk: (no comment)
    /// mpj: **@CODE@**
    /// iku: (no comment)
    #[strum(serialize = "1Sg")]
    _1Sg,
    /// kal: **@CODE@** = Object 1.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// ess: (no comment)
    /// kio: (no comment)
    /// ipk: (no comment)
    /// iku: (no comment)
    #[strum(serialize = "1SgO")]
    _1SgO,
    /// xak: (no comment)
    #[strum(serialize = "1SgObj")]
    _1SgObj,
    /// som: (no comment)
    #[strum(serialize = "1SgObj/i")]
    _1SgObj_SLASH_i,
    /// kal: **@CODE@** = Possessor 1.person singular
    /// ciw: (no comment)
    #[strum(serialize = "1SgPoss")]
    _1SgPoss,
    /// xak: (no comment)
    #[strum(serialize = "1SgSubj")]
    _1SgSubj,
    /// tgl: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "1p")]
    _1p,
    /// fro: (no comment)
    #[strum(serialize = "1s")]
    _1s,
    /// hdn: (no comment)
    #[strum(serialize = "1sg")]
    _1sg,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// sms: **@CODE@** used in combination with +Imp+2+ConNeg
    /// mya: (no comment)
    /// non: (no comment)
    #[strum(serialize = "2")]
    _2,
    /// bla: (no comment)
    #[strum(serialize = "21Pl")]
    _21Pl,
    /// bla: (no comment)
    #[strum(serialize = "21PlO")]
    _21PlO,
    /// non: (no comment)
    #[strum(serialize = "2Du")]
    _2Du,
    /// gle: **@CODE@** = second person inflection
    #[strum(serialize = "2P")]
    _2P,
    /// kal: **@CODE@** = Subject 2.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// som: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "2Pl")]
    _2Pl,
    /// kal: **@CODE@** = Object 2.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "2PlO")]
    _2PlO,
    /// xak: (no comment)
    #[strum(serialize = "2PlObj")]
    _2PlObj,
    /// kal: **@CODE@** = Possessor 2.person plural
    #[strum(serialize = "2PlPoss")]
    _2PlPoss,
    /// xak: (no comment)
    #[strum(serialize = "2PlSubj")]
    _2PlSubj,
    /// kal: **@CODE@** = Subject 2.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// som: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "2Sg")]
    _2Sg,
    /// kio: (no comment)
    #[strum(serialize = "2Sg/Pl")]
    _2Sg_SLASH_Pl,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "2Sg/PlO")]
    _2Sg_SLASH_PlO,
    /// kal: **@CODE@** = Object 2.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "2SgO")]
    _2SgO,
    /// xak: (no comment)
    #[strum(serialize = "2SgObj")]
    _2SgObj,
    /// som: (no comment)
    #[strum(serialize = "2SgObj/ku")]
    _2SgObj_SLASH_ku,
    /// kal: **@CODE@** = Possessor 2.person singular
    #[strum(serialize = "2SgPoss")]
    _2SgPoss,
    /// xak: (no comment)
    #[strum(serialize = "2SgSubj")]
    _2SgSubj,
    /// hdn: (no comment)
    #[strum(serialize = "2dl")]
    _2dl,
    /// hdn: (no comment)
    #[strum(serialize = "2pl")]
    _2pl,
    /// hdn: (no comment)
    #[strum(serialize = "2sg")]
    _2sg,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// mya: (no comment)
    /// non: (no comment)
    #[strum(serialize = "3")]
    _3,
    /// bla: (no comment)
    /// bla: (no comment)
    #[strum(serialize = "3/")]
    _3_SLASH_,
    /// gle: **@CODE@** = third person inflection
    /// amh: (no comment)
    #[strum(serialize = "3P")]
    _3P,
    /// kal: **@CODE@** = Subject 3.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// som: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "3Pl")]
    _3Pl,
    /// kal: **@CODE@** = Object 3.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "3PlO")]
    _3PlO,
    /// xak: (no comment)
    #[strum(serialize = "3PlObj")]
    _3PlObj,
    /// ciw: (no comment)
    #[strum(serialize = "3PlObvHead")]
    _3PlObvHead,
    /// kal: **@CODE@** = Possessor 3.person plural
    #[strum(serialize = "3PlPoss")]
    _3PlPoss,
    /// ciw: (no comment)
    #[strum(serialize = "3PlProxObj")]
    _3PlProxObj,
    /// xak: (no comment)
    #[strum(serialize = "3PlSubj")]
    _3PlSubj,
    /// kal: **@CODE@** = Subject 3.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// som: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "3Sg")]
    _3Sg,
    /// bla: (no comment)
    #[strum(serialize = "3Sg/Pl")]
    _3Sg_SLASH_Pl,
    /// som: (no comment)
    #[strum(serialize = "3SgF")]
    _3SgF,
    /// som: (no comment)
    #[strum(serialize = "3SgM")]
    _3SgM,
    /// kal: **@CODE@** = Object 3.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "3SgO")]
    _3SgO,
    /// xak: (no comment)
    #[strum(serialize = "3SgObj")]
    _3SgObj,
    /// ciw: (no comment)
    #[strum(serialize = "3SgObvSubj")]
    _3SgObvSubj,
    /// kal: **@CODE@** = Possessor 3.person singular
    #[strum(serialize = "3SgPoss")]
    _3SgPoss,
    /// xak: (no comment)
    #[strum(serialize = "3SgSubj")]
    _3SgSubj,
    /// tku: (no comment)
    #[strum(serialize = "3iPl")]
    _3iPl,
    /// tku: (no comment)
    #[strum(serialize = "3iPlO")]
    _3iPlO,
    /// tku: (no comment)
    #[strum(serialize = "3iSg")]
    _3iSg,
    /// tku: (no comment)
    #[strum(serialize = "3iSgO")]
    _3iSgO,
    /// tku: (no comment)
    #[strum(serialize = "3oPl")]
    _3oPl,
    /// tku: (no comment)
    #[strum(serialize = "3oPlO")]
    _3oPlO,
    /// tku: (no comment)
    #[strum(serialize = "3oSg")]
    _3oSg,
    /// tku: (no comment)
    #[strum(serialize = "3oSgO")]
    _3oSgO,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "3pl")]
    _3pl,
    /// hdn: (no comment)
    #[strum(serialize = "3sg")]
    _3sg,
    /// kal: **@CODE@** = Subject 4.person plural
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "4Pl")]
    _4Pl,
    /// kal: **@CODE@** = Object 4.person plural
    /// bla: (no comment)
    /// crj: (no comment)
    #[strum(serialize = "4PlO")]
    _4PlO,
    /// kal: **@CODE@** = Possessor 4.person plural
    #[strum(serialize = "4PlPoss")]
    _4PlPoss,
    /// kal: **@CODE@** = Subject 4.person singular
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "4Sg")]
    _4Sg,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "4Sg/Pl")]
    _4Sg_SLASH_Pl,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "4Sg/PlO")]
    _4Sg_SLASH_PlO,
    /// kal: **@CODE@** = Object 4.person singular
    /// bla: (no comment)
    /// crj: (no comment)
    #[strum(serialize = "4SgO")]
    _4SgO,
    /// kal: **@CODE@** = Possessor 4.person singular
    #[strum(serialize = "4SgPoss")]
    _4SgPoss,
    /// crj: (no comment)
    #[strum(serialize = "5Sg")]
    _5Sg,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "5Sg/Pl")]
    _5Sg_SLASH_Pl,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "5Sg/PlO")]
    _5Sg_SLASH_PlO,
    /// fit: **@CODE@** = Adjective
    /// sje: (no comment)
    /// gle: **@CODE@** = XXX check
    /// kal: **@CODE@**
    /// kpv: **@CODE@**:  adjective  кывберд   прилагательное
    /// vot: (no comment)
    /// bla: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Adjective
    /// liv: **@CODE@** = adjective
    /// crk: (no comment)
    /// mhr: **@CODE@** = adjectives
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Adjective
    /// tgl: **@CODE@** -  Adjective
    /// hdn: (no comment)
    /// smj: **@CODE@** = Adjective
    /// udm: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Adjective
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Adjective
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = adjective
    /// cho: (no comment)
    /// cho: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  adjective
    /// koi: **@CODE@**:  adjective
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Adjective
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  adjective
    /// ceb: **@CODE@** -  Adjective
    /// khk: (no comment)
    /// vro: **@CODE@** Adjective
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// kca: (no comment)
    /// kca: (no comment)
    #[strum(serialize = "A")]
    A,
    /// kal: **@CODE@**
    #[strum(serialize = "AA")]
    AA,
    /// kal: **@CODE@**
    #[strum(serialize = "AASIIT")]
    AASIIT,
    /// kal: **@CODE@**
    #[strum(serialize = "AASIT")]
    AASIT,
    /// kal: **@CODE@**
    #[strum(serialize = "AAT")]
    AAT,
    /// fit: **@CODE@**
    /// sje: (no comment)
    /// gle: **@CODE@** = Abbreviation
    /// kpv: **@CODE@**
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Abbreviation
    /// izh: **@CODE@**
    /// izh: (no comment)
    /// liv: **@CODE@** containing period
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = for abbreviations that (may) contain period
    /// krl: **@CODE@**
    /// xal: (no comment)
    /// sma: **@CODE@**:  Abbreviation
    /// lut: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@**
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Abbreviation
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@**
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// koi: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Abbreviation, subtag for e.g. +N
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** abbreviations
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "ABBR")]
    ABBR,
    /// myv: (no comment)
    #[strum(serialize = "ACC")]
    ACC,
    /// fit: **@CODE@**
    /// gle: **@CODE@** = Acronym
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Acronym
    /// liv: **@CODE@** acronyms, not containing period
    /// mhr: **@CODE@** = acronyms
    /// sma: **@CODE@**:  Acronym
    /// nob: **@CODE@**
    /// hdn: (no comment)
    /// smj: **@CODE@** = Acronym
    /// smj: (no comment)
    /// udm: (no comment)
    /// chr: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Acronym
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// myv: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@**  Acronym, subtag for +N
    /// smn: **@CODE@** - Acronym
    /// fin: (no comment)
    /// mdf: (no comment)
    /// rmf: **@CODE@** acronyms
    /// vro: **@CODE@**
    #[strum(serialize = "ACR")]
    ACR,
    /// ciw: (no comment)
    #[strum(serialize = "ADVConj")]
    ADVConj,
    /// ciw: (no comment)
    #[strum(serialize = "ADVPred")]
    ADVPred,
    /// tgl: (no comment)
    #[strum(serialize = "AF")]
    AF,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "AI")]
    AI,
    /// kal: **@CODE@**
    #[strum(serialize = "ALAAQ")]
    ALAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "ALAAR")]
    ALAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "ALLAK")]
    ALLAK,
    /// kal: **@CODE@**
    #[strum(serialize = "ALLAP")]
    ALLAP,
    /// kal: **@CODE@**
    #[strum(serialize = "ALUK")]
    ALUK,
    /// kal: **@CODE@**
    #[strum(serialize = "ALUP")]
    ALUP,
    /// crj: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "AN")]
    AN,
    /// koi: (no comment)
    #[strum(serialize = "ANA")]
    ANA,
    /// koi: (no comment)
    #[strum(serialize = "ANAA")]
    ANAA,
    /// kal: **@CODE@**
    #[strum(serialize = "AR")]
    AR,
    /// kal: **@CODE@**
    #[strum(serialize = "ARAQ")]
    ARAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "ARSUK")]
    ARSUK,
    /// kal: **@CODE@**
    #[strum(serialize = "ARSUP")]
    ARSUP,
    /// kal: **@CODE@**
    #[strum(serialize = "ATAAR")]
    ATAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "ATSIAQ")]
    ATSIAQ,
    /// fit: **@CODE@**
    /// rue: (no comment)
    /// fao: (no comment)
    /// rus: (no comment)
    /// som: (no comment)
    /// eus: (no comment)
    /// khk: (no comment)
    /// iku: (no comment)
    #[strum(serialize = "Abbr")]
    Abbr,
    /// fit: **@CODE@** = Abessive
    /// sje: (no comment)
    /// kpv: **@CODE@** PrivMod, AbeMod джуджыд анализъястӧм да обобщениеястӧм статьяяс.
    /// fkv: **@CODE@** = Abessive
    /// liv: **@CODE@** = abessive
    /// mhr: **@CODE@** = abessive
    /// smj: **@CODE@** = Abessive case
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Abessive
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = abessive
    /// myv: (no comment)
    /// olo: **@CODE@** abessive
    /// fin: (no comment)
    /// mdf: **@CODE@**:  abessive
    /// vro: **@CODE@** abessive
    #[strum(serialize = "Abe")]
    Abe,
    /// fit: **@CODE@** = Ablative
    /// kal: **@CODE@** = Ablative
    /// kpv: **@CODE@** ablative case -лысь босьтан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Ablative
    /// liv: **@CODE@** = ablative case
    /// evn: (no comment)
    /// udm: (no comment)
    /// rmy: (no comment)
    /// skf: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@** ныл
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**  = ablative case
    /// vep: (no comment)
    /// myv: (no comment)
    /// sqi: (no comment)
    /// olo: **@CODE@** ablative case
    /// koi: (no comment)
    /// xak: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  ablative
    /// rmf: **@CODE@** Ablative
    /// khk: (no comment)
    /// vro: **@CODE@** ablative
    /// mpj: **@CODE@**
    /// tqn: (no comment)
    #[strum(serialize = "Abl")]
    Abl,
    /// gle: **@CODE@** = "Abbreviation"
    #[strum(serialize = "Abr")]
    Abr,
    /// kal: **@CODE@** = Absolutive
    /// kpv: **@CODE@** Absolute = +Sg+Nom
    /// kpv: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// udm: (no comment)
    /// ess: (no comment)
    /// som: (no comment)
    /// eus: (no comment)
    /// ipk: (no comment)
    /// mpj: **@CODE@**
    /// iku: (no comment)
    #[strum(serialize = "Abs")]
    Abs,
    /// eus: (no comment)
    #[strum(serialize = "Abs/Pl1")]
    Abs_SLASH_Pl1,
    /// eus: (no comment)
    #[strum(serialize = "Abs/Pl2")]
    Abs_SLASH_Pl2,
    /// eus: (no comment)
    #[strum(serialize = "Abs/Pl3")]
    Abs_SLASH_Pl3,
    /// eus: (no comment)
    #[strum(serialize = "Abs/Sg1")]
    Abs_SLASH_Sg1,
    /// eus: (no comment)
    #[strum(serialize = "Abs/Sg2")]
    Abs_SLASH_Sg2,
    /// eus: (no comment)
    #[strum(serialize = "Abs/Sg3")]
    Abs_SLASH_Sg3,
    /// som: (no comment)
    #[strum(serialize = "Abstr")]
    Abstr,
    /// fit: **@CODE@** = Accusative, for pronouns, but is it correct?
    /// sje: (no comment)
    /// gle: **@CODE@** = Accusative case
    /// kpv: **@CODE@** accusative ZERO керан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Accusative, for pronouns, but is it correct?
    /// mhr: **@CODE@** = accusative
    /// evn: (no comment)
    /// sma: **@CODE@** = Ackusative
    /// smj: **@CODE@** = Accusative case
    /// udm: (no comment)
    /// rmy: (no comment)
    /// epo: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** Accusative
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = accusative
    /// rmg: **@CODE@**  needed ?
    /// myv: (no comment)
    /// sqi: (no comment)
    /// olo: **@CODE@** accusative
    /// koi: (no comment)
    /// sme: **@CODE@** - Accusative
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  accusative Not really necessary
    /// mdf: (no comment)
    /// rmf: **@CODE@** Accusative
    /// khk: (no comment)
    /// vro: **@CODE@** accusative
    #[strum(serialize = "Acc")]
    Acc,
    /// kpv: **@CODE@** accusative -ӧс керан
    #[strum(serialize = "Acc1")]
    Acc1,
    /// kpv: **@CODE@** accusative -сӧ керан
    #[strum(serialize = "Acc3")]
    Acc3,
    /// bla: (no comment)
    #[strum(serialize = "Accomp")]
    Accomp,
    /// som: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Acr")]
    Acr,
    /// vro: **@CODE@**
    #[strum(serialize = "Acro")]
    Acro,
    /// kpv: **@CODE@**
    #[strum(serialize = "Acron")]
    Acron,
    /// mpj: **@CODE@**  Acs = Accessory = locative or allative
    #[strum(serialize = "Acs")]
    Acs,
    /// fit: **@CODE@** =
    /// fkv: **@CODE@** = Active
    /// liv: **@CODE@** = active
    /// mhr: **@CODE@** = Active
    /// krl: (no comment)
    /// sma: **@CODE@** | *-eme*, could be changed to `+Actio`
    /// got: (no comment)
    /// sms: **@CODE@** Active voice
    /// mrj: **@CODE@** =  Prc active participle ЫшЫ
    /// vep: **@CODE@**  = active voice
    /// myv: (no comment)
    /// olo: **@CODE@** : active voice
    /// fin: (no comment)
    /// mdf: **@CODE@**:  active
    /// vro: **@CODE@** active
    /// mpj: **@CODE@** -ti	nominal -> IT verb. changes position/stance meaning to action.
    #[strum(serialize = "Act")]
    Act,
    /// tgl: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Act1")]
    Act1,
    /// myv: (no comment)
    #[strum(serialize = "ActDemPrc")]
    ActDemPrc,
    /// sms: **@CODE@** -men
    #[strum(serialize = "ActEss")]
    ActEss,
    /// myv: (no comment)
    #[strum(serialize = "ActPrcLong")]
    ActPrcLong,
    /// myv: (no comment)
    #[strum(serialize = "ActPrcShort")]
    ActPrcShort,
    /// izh: **@CODE@**
    #[strum(serialize = "ActPrsPrc")]
    ActPrsPrc,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "ActPrsPtc")]
    ActPrsPtc,
    /// smj: (no comment)
    /// mns: (no comment)
    /// sms: **@CODE@**
    /// sme: **@CODE@** Action Verb Form
    /// smn: **@CODE@**
    #[strum(serialize = "Actio")]
    Actio,
    /// sms: **@CODE@**
    #[strum(serialize = "Actor")]
    Actor,
    /// gle: **@CODE@** = Adverbial particle: go
    #[strum(serialize = "Ad")]
    Ad,
    /// olo: (no comment)
    #[strum(serialize = "Ad-A")]
    Ad_MINUS_A,
    /// mrj: **@CODE@** =
    /// olo: **@CODE@**  Ad-adjective
    /// koi: (no comment)
    #[strum(serialize = "AdA")]
    AdA,
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// ron: (no comment)
    /// grn: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
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
    /// hun: (no comment)
    /// vep: **@CODE@**  = adessive
    /// vep: (no comment)
    /// vep: (no comment)
    /// olo: **@CODE@** adessive case
    /// fin: (no comment)
    /// vro: **@CODE@** adessive
    #[strum(serialize = "Ade")]
    Ade,
    /// gle: **@CODE@** = Adjective
    /// bla: (no comment)
    /// epo: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Adj")]
    Adj,
    /// mpj: **@CODE@**  = Admonitive
    #[strum(serialize = "Admon")]
    Admon,
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Adn")]
    Adn,
    /// kpv: **@CODE@**:  adposition (prepositio, postposition)
    /// vot: (no comment)
    /// liv: **@CODE@** = adposition
    /// mhr: **@CODE@** = adpositions
    /// mhr: (no comment)
    /// hdn: (no comment)
    /// udm: (no comment)
    /// epo: (no comment)
    /// apu: **@CODE@** Adposition, i.e. Post- and Prepostion, NOT IN USE/ NÃO USADO
    /// chr: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Adposition, i.e. Post- and Prepostion
    /// lit: (no comment)
    /// vep: **@CODE@**  = adposition
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  adposition
    /// koi: **@CODE@**:  Adposition
    /// sme: **@CODE@** - Adposition, ie Post- and Prepostion, NOT IN USE
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@** Adpositions
    /// mpj: **@CODE@**
    #[strum(serialize = "Adp")]
    Adp,
    /// som: (no comment)
    #[strum(serialize = "Adp/ka")]
    Adp_SLASH_ka,
    /// som: (no comment)
    #[strum(serialize = "Adp/ku")]
    Adp_SLASH_ku,
    /// som: (no comment)
    #[strum(serialize = "Adp/la")]
    Adp_SLASH_la,
    /// som: (no comment)
    #[strum(serialize = "Adp/u")]
    Adp_SLASH_u,
    /// fit: **@CODE@** = Adverb
    /// sje: (no comment)
    /// gle: **@CODE@** = Adverb
    /// kal: **@CODE@** = Adverb
    /// kpv: **@CODE@**:  adverb    урчитан            наречие
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Adverb
    /// liv: **@CODE@** = adverb
    /// crk: (no comment)
    /// mhr: **@CODE@** = adverbs
    /// mhr: (no comment)
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@** = Adverb
    /// tgl: **@CODE@** -  Adverb
    /// hdn: (no comment)
    /// smj: **@CODE@** = Adverb
    /// udm: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Adverb
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Adverb
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = adverb
    /// tat: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// grn: (no comment)
    /// olo: **@CODE@**  adverb
    /// koi: **@CODE@**:  adverb
    /// deu: (no comment)
    /// sme: **@CODE@** - Adverb
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  adverb
    /// ceb: **@CODE@** -  Adverb
    /// khk: (no comment)
    /// ces: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@** Adverb
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// kca: (no comment)
    #[strum(serialize = "Adv")]
    Adv,
    /// koi: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Adv-Ideoph")]
    Adv_MINUS_Ideoph,
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    #[strum(serialize = "Advc")]
    Advc,
    /// udm: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Advl")]
    Advl,
    /// kal: **@CODE@** = Aequalis
    #[strum(serialize = "Aeq")]
    Aeq,
    /// apu: **@CODE@** affected by action of verb %>nhi
    #[strum(serialize = "Aff")]
    Aff,
    /// bxr: (no comment)
    #[strum(serialize = "AgConstPrc")]
    AgConstPrc,
    /// fit: **@CODE@** = agent participe
    /// bxr: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "AgPrc")]
    AgPrc,
    /// fro: (no comment)
    #[strum(serialize = "Agent")]
    Agent,
    /// moh: (no comment)
    #[strum(serialize = "AgentDu2")]
    AgentDu2,
    /// moh: (no comment)
    #[strum(serialize = "AgentDu21")]
    AgentDu21,
    /// moh: (no comment)
    #[strum(serialize = "AgentDu3")]
    AgentDu3,
    /// moh: (no comment)
    #[strum(serialize = "AgentDu31")]
    AgentDu31,
    /// moh: (no comment)
    #[strum(serialize = "AgentDu3Fem")]
    AgentDu3Fem,
    /// moh: (no comment)
    #[strum(serialize = "AgentPl2")]
    AgentPl2,
    /// moh: (no comment)
    #[strum(serialize = "AgentPl2Sg1")]
    AgentPl2Sg1,
    /// moh: (no comment)
    #[strum(serialize = "AgentPl3")]
    AgentPl3,
    /// moh: (no comment)
    #[strum(serialize = "AgentPl3Fem")]
    AgentPl3Fem,
    /// moh: (no comment)
    #[strum(serialize = "AgentPl3Sg1")]
    AgentPl3Sg1,
    /// moh: (no comment)
    #[strum(serialize = "AgentSg1")]
    AgentSg1,
    /// moh: (no comment)
    #[strum(serialize = "AgentSg2")]
    AgentSg2,
    /// moh: (no comment)
    #[strum(serialize = "AgentSg3Fem")]
    AgentSg3Fem,
    /// moh: (no comment)
    #[strum(serialize = "AgentSg3Mal")]
    AgentSg3Mal,
    /// moh: (no comment)
    #[strum(serialize = "AgentSg3Neuter")]
    AgentSg3Neuter,
    /// kal: **@CODE@** = Accusative
    #[strum(serialize = "Akk")]
    Akk,
    /// khk: (no comment)
    #[strum(serialize = "Al")]
    Al,
    /// fit: **@CODE@** = Allative
    /// vot: (no comment)
    /// fkv: **@CODE@** = Allative
    /// liv: **@CODE@** = allative
    /// evn: (no comment)
    /// rue: (no comment)
    /// apu: **@CODE@** -mukary (allative)
    /// rus: (no comment)
    /// hun: (no comment)
    /// vep: **@CODE@**  = allative
    /// vep: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** Allatiivi
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@** allative
    /// mpj: **@CODE@**
    #[strum(serialize = "All")]
    All,
    /// smj: (no comment)
    /// sms: **@CODE@** never last element of compound words
    /// sme: **@CODE@** from LEXICON GOADE-IU-
    /// smn: **@CODE@**
    #[strum(serialize = "Allegro")]
    Allegro,
    /// hdn: (no comment)
    /// apu: (no comment)
    #[strum(serialize = "Almost")]
    Almost,
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
    /// epo: (no comment)
    #[strum(serialize = "Amt")]
    Amt,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "AnIn")]
    AnIn,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Anew")]
    Anew,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Ani")]
    Ani,
    /// qya: (no comment)
    /// dag: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Anim")]
    Anim,
    /// gle: **@CODE@** = Anonymisation in transcribed speech
    #[strum(serialize = "Anon")]
    Anon,
    /// khk: (no comment)
    #[strum(serialize = "Ant")]
    Ant,
    /// yrk: **@CODE@**
    #[strum(serialize = "Aor")]
    Aor,
    /// khk: (no comment)
    #[strum(serialize = "Aori")]
    Aori,
    /// khk: (no comment)
    #[strum(serialize = "Apost")]
    Apost,
    /// som: (no comment)
    #[strum(serialize = "Appos")]
    Appos,
    /// kpv: **@CODE@**:  Approximative numeral кавто-колмо, колмошка *two or three*   NB! do not confuse with Komi case +Apr
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// myv: (no comment)
    /// koi: **@CODE@**:  Approximative numeral кавто-колмо, колмошка *two or three*
    /// mdf: **@CODE@**:  Approximative numeral кафта-колма, колмошка "two or three" requires split?
    #[strum(serialize = "Appr")]
    Appr,
    /// kpv: **@CODE@** approximative -лань матыстчан
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// som: (no comment)
    /// olo: **@CODE@** Approximative
    /// koi: (no comment)
    #[strum(serialize = "Apr")]
    Apr,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Apr1")]
    Apr1,
    /// vep: **@CODE@**  =
    #[strum(serialize = "Apr2")]
    Apr2,
    /// kpv: **@CODE@** approximative egressive -ланьсянь матысь ылыстчан
    /// koi: (no comment)
    #[strum(serialize = "AprEgr")]
    AprEgr,
    /// kpv: **@CODE@** approximative elative -ланьысь матысь петан
    /// koi: (no comment)
    #[strum(serialize = "AprEla")]
    AprEla,
    /// kpv: **@CODE@** approximative illative -ланьӧ матӧ матыстчан
    /// koi: (no comment)
    #[strum(serialize = "AprIll")]
    AprIll,
    /// kpv: **@CODE@** approximative inessive -ланьын матыс ина
    /// koi: (no comment)
    #[strum(serialize = "AprIne")]
    AprIne,
    /// kpv: **@CODE@** approximative prolative -ланьӧд маті вуджан
    /// koi: (no comment)
    #[strum(serialize = "AprPrl")]
    AprPrl,
    /// kpv: **@CODE@** approximative terminative -ланьӧдз матіӧдз воан
    #[strum(serialize = "AprTer")]
    AprTer,
    /// koi: (no comment)
    #[strum(serialize = "AprTerI")]
    AprTerI,
    /// koi: (no comment)
    #[strum(serialize = "AprTerII")]
    AprTerII,
    /// kpv: **@CODE@** approximative translative -ланьті маті вуджан
    #[strum(serialize = "AprTra")]
    AprTra,
    /// fit: **@CODE@**
    /// gle: **@CODE@** = Arabic numerals (1, 2, ...)
    /// kal: **@CODE@** = pga- brug i ~/langtech/shared-mul/src/fst/stems/telephone.lexc
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Arabic
    /// crk: (no comment)
    /// mhr: **@CODE@** = arabic numerals
    /// sma: **@CODE@** = Arabic numeral
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = arabic numerals
    /// apu: **@CODE@** Arabic numeral, subtag for +Num
    /// mns: **@CODE@** - Arabic numeral
    /// sms: **@CODE@** - Arabic numeral
    /// rmn: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Arabic numeral, subtag for +Num
    /// smn: **@CODE@** - Arabic numeral, subtag for +Num
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  arabic numeral
    /// vro: **@CODE@**
    /// bak: (no comment)
    #[strum(serialize = "Arab")]
    Arab,
    /// ess: (no comment)
    /// ipk: (no comment)
    #[strum(serialize = "Arch")]
    Arch,
    /// sma: (no comment)
    /// smj: **@CODE@** - Used in Norway only
    #[strum(serialize = "Area/NO")]
    Area_SLASH_NO,
    /// sma: (no comment)
    /// smj: **@CODE@** - Used in Sweden only
    #[strum(serialize = "Area/SE")]
    Area_SLASH_SE,
    /// gle: **@CODE@** = Article determiner (an/na)
    /// epo: (no comment)
    /// eus: (no comment)
    /// non: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Art")]
    Art,
    /// skf: (no comment)
    #[strum(serialize = "Asc")]
    Asc,
    /// chp: (no comment)
    #[strum(serialize = "Asp")]
    Asp,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Asp/Cont")]
    Asp_SLASH_Cont,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Asp/Pfv")]
    Asp_SLASH_Pfv,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Asp/Prog")]
    Asp_SLASH_Prog,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Asp/ProgStat")]
    Asp_SLASH_ProgStat,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Asp/Stat")]
    Asp_SLASH_Stat,
    /// kpv: **@CODE@**:  +мезть
    /// epo: (no comment)
    /// myv: (no comment)
    /// koi: **@CODE@**:  +мезть
    #[strum(serialize = "Assoc")]
    Assoc,
    /// kpv: **@CODE@**:  -ne- ; avide-
    /// mhr: **@CODE@** = Collective associative numerals with obligatory possessive suffixes -нь-
    /// udm: (no comment)
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: **@CODE@**:  -ne- ; avide-
    /// mdf: **@CODE@**:  -ne- ; avide-; -месть
    #[strum(serialize = "AssocColl")]
    AssocColl,
    /// mhr: **@CODE@** =
    #[strum(serialize = "AssocPl")]
    AssocPl,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Att")]
    Att,
    /// fit: **@CODE@** = Attributive form, hmm, check, for names?
    /// sje: (no comment)
    /// gle: **@CODE@** = Attribute, element preceeding head
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Attributive form, hmm, check, for names?
    /// izh: **@CODE@**
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** = attributive form
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@**:  Attribute form
    /// lut: (no comment)
    /// sjt: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@**
    /// got: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** Attributive form
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@**   Attributive
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  attribute, premodifier
    /// ale: (no comment)
    /// rmf: **@CODE@** attribute
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Attr")]
    Attr,
    /// yrk: **@CODE@**
    #[strum(serialize = "Aud")]
    Aud,
    /// koi: **@CODE@**:  augmentative, intensive -ӧв 'too X' with reference to quality ыджытӧв 'too big'
    /// tlh: (no comment)
    #[strum(serialize = "Aug")]
    Aug,
    /// apu: (no comment)
    #[strum(serialize = "Augm")]
    Augm,
    /// gle: **@CODE@** = Autonomous verb form
    #[strum(serialize = "Auto")]
    Auto,
    /// kpv: **@CODE@**
    /// liv: **@CODE@** = Auxiliary verb
    /// mhr: **@CODE@** = Auxiliary verb
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: **@CODE@** Auxiliary
    /// yrk: **@CODE@** auxilliary verb
    /// mns: **@CODE@** Auxiliary
    /// sms: **@CODE@** = Auxiliary
    /// mrj: **@CODE@** auxiliary
    /// vep: **@CODE@**  = auxiliary verb
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// tkl: (no comment)
    /// koi: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Aux")]
    Aux,
    /// mns: (no comment)
    #[strum(serialize = "Aux/vos")]
    Aux_SLASH_vos,
    /// mpj: **@CODE@**
    /// mpj: **@CODE@**  Avoidance
    #[strum(serialize = "Avoid")]
    Avoid,
    /// tgl: (no comment)
    #[strum(serialize = "BF")]
    BF,
    /// pad: **@CODE@** bitransitive / bitransitivo
    #[strum(serialize = "BV")]
    BV,
    /// olo: **@CODE@**  Bahuvrihi
    #[strum(serialize = "Bahuv")]
    Bahuv,
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    #[strum(serialize = "Bahuvrihi")]
    Bahuvrihi,
    /// gle: **@CODE@** = hyphen, underscore, dash etc.
    #[strum(serialize = "Bar")]
    Bar,
    /// gle: **@CODE@** = bare number form used after number particle "a"
    #[strum(serialize = "Bare")]
    Bare,
    /// gle: **@CODE@** = Base form of adjective (changed from +Pos to +Base 10/09/03)
    #[strum(serialize = "Base")]
    Base,
    /// tgl: (no comment)
    /// slh: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Ben")]
    Ben,
    /// hdn: (no comment)
    #[strum(serialize = "Bias")]
    Bias,
    /// tlh: (no comment)
    #[strum(serialize = "Bnf")]
    Bnf,
    /// gle: **@CODE@** = round, square and curly brackets
    #[strum(serialize = "Brack")]
    Brack,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Build")]
    Build,
    /// krl: **@CODE@** TYÖÖÖÖ! this need depricating :-) 2023-11-08 Jaska
    #[strum(serialize = "C")]
    C,
    /// fit: **@CODE@** = Conjunction
    /// sje: (no comment)
    /// gle: **@CODE@** = Canúint Chonnachta, Connaught dialect
    /// kpv: **@CODE@**:  coordinating conjunction      XX   сочинительный союз
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Conjunction
    /// izh: (no comment)
    /// liv: **@CODE@** = coordinating conjunction
    /// crk: (no comment)
    /// mhr: **@CODE@** = conjunctions
    /// mhr: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Conjunction
    /// tgl: **@CODE@** -  Conjunction
    /// hdn: (no comment)
    /// fao: (no comment)
    /// smj: **@CODE@** = Conjunction
    /// udm: (no comment)
    /// epo: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Conjunction
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Conjunction
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = coordinating conjunction
    /// tat: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  coordinating conjunction
    /// koi: **@CODE@**:  coordinating conjunction
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Conjunction
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  coordinating conjunction
    /// ceb: **@CODE@** -  Conjunction
    /// khk: (no comment)
    /// vro: **@CODE@** Correlating conjunction
    /// mpj: **@CODE@**
    #[strum(serialize = "CC")]
    CC,
    /// qya: (no comment)
    #[strum(serialize = "CCONJ")]
    CCONJ,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Color")]
    CL_SLASH_Color,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Descriptive")]
    CL_SLASH_Descriptive,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Human")]
    CL_SLASH_Human,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Human_Female")]
    CL_SLASH_Human_Female,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Human_Male")]
    CL_SLASH_Human_Male,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Manner")]
    CL_SLASH_Manner,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Shape")]
    CL_SLASH_Shape,
    /// hdn: (no comment)
    #[strum(serialize = "CL/Sound")]
    CL_SLASH_Sound,
    /// hdn: (no comment)
    #[strum(serialize = "CL/cha")]
    CL_SLASH_cha,
    /// hdn: (no comment)
    #[strum(serialize = "CL/chab")]
    CL_SLASH_chab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/cháam")]
    CL_SLASH_cháam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/dab")]
    CL_SLASH_dab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/dla")]
    CL_SLASH_dla,
    /// hdn: (no comment)
    #[strum(serialize = "CL/dláam")]
    CL_SLASH_dláam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/dlál")]
    CL_SLASH_dlál,
    /// hdn: (no comment)
    #[strum(serialize = "CL/dám")]
    CL_SLASH_dám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/ga")]
    CL_SLASH_ga,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gab")]
    CL_SLASH_gab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gi")]
    CL_SLASH_gi,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/gu")]
    CL_SLASH_gu,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gyáam")]
    CL_SLASH_gyáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gáam")]
    CL_SLASH_gáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gám")]
    CL_SLASH_gám,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/gáng")]
    CL_SLASH_gáng,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gáw")]
    CL_SLASH_gáw,
    /// hdn: (no comment)
    #[strum(serialize = "CL/gúl")]
    CL_SLASH_gúl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/ĝám")]
    CL_SLASH_ĝám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/g̲a")]
    CL_SLASH_g̲a,
    /// hdn: (no comment)
    #[strum(serialize = "CL/g̲áam")]
    CL_SLASH_g̲áam,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlga")]
    CL_SLASH_hlga,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlgi")]
    CL_SLASH_hlgi,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlgáam")]
    CL_SLASH_hlgáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlgám")]
    CL_SLASH_hlgám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlĝám")]
    CL_SLASH_hlĝám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlg̲a")]
    CL_SLASH_hlg̲a,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlg̲áam")]
    CL_SLASH_hlg̲áam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlg̲áy")]
    CL_SLASH_hlg̲áy,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk")]
    CL_SLASH_hlk,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlku")]
    CL_SLASH_hlku,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlkuhl")]
    CL_SLASH_hlkuhl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlkwáahl")]
    CL_SLASH_hlkwáahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlkʼib")]
    CL_SLASH_hlkʼib,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlkʼu")]
    CL_SLASH_hlkʼu,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲")]
    CL_SLASH_hlk̲,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲áa")]
    CL_SLASH_hlk̲áa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲ám")]
    CL_SLASH_hlk̲ám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲íl")]
    CL_SLASH_hlk̲íl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲ʼuhl")]
    CL_SLASH_hlk̲ʼuhl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲ʼwáahl")]
    CL_SLASH_hlk̲ʼwáahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hlk̲ʼáam")]
    CL_SLASH_hlk̲ʼáam,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltab")]
    CL_SLASH_hltab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltáam")]
    CL_SLASH_hltáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltám")]
    CL_SLASH_hltám,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltʼab")]
    CL_SLASH_hltʼab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltʼah")]
    CL_SLASH_hltʼah,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltʼahl")]
    CL_SLASH_hltʼahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hltʼáam")]
    CL_SLASH_hltʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/hám")]
    CL_SLASH_hám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/id")]
    CL_SLASH_id,
    /// hdn: (no comment)
    #[strum(serialize = "CL/is")]
    CL_SLASH_is,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/ja")]
    CL_SLASH_ja,
    /// hdn: (no comment)
    #[strum(serialize = "CL/jah")]
    CL_SLASH_jah,
    /// hdn: (no comment)
    #[strum(serialize = "CL/jám")]
    CL_SLASH_jám,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/jíi")]
    CL_SLASH_jíi,
    /// hdn: (no comment)
    #[strum(serialize = "CL/jíihl")]
    CL_SLASH_jíihl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k")]
    CL_SLASH_k,
    /// hdn: (no comment)
    #[strum(serialize = "CL/ki")]
    CL_SLASH_ki,
    /// hdn: (no comment)
    #[strum(serialize = "CL/káa")]
    CL_SLASH_káa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/kál")]
    CL_SLASH_kál,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/kám")]
    CL_SLASH_kám,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/kún")]
    CL_SLASH_kún,
    /// hdn: (no comment)
    #[strum(serialize = "CL/kʼu")]
    CL_SLASH_kʼu,
    /// hdn: (no comment)
    #[strum(serialize = "CL/kʼwáa")]
    CL_SLASH_kʼwáa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/kʼwáahl")]
    CL_SLASH_kʼwáahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/kʼúl")]
    CL_SLASH_kʼúl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ab")]
    CL_SLASH_k̲ab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ám")]
    CL_SLASH_k̲ám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲áw")]
    CL_SLASH_k̲áw,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼa")]
    CL_SLASH_k̲ʼa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼah")]
    CL_SLASH_k̲ʼah,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼuhl")]
    CL_SLASH_k̲ʼuhl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼwáahl")]
    CL_SLASH_k̲ʼwáahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼáam")]
    CL_SLASH_k̲ʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼéem")]
    CL_SLASH_k̲ʼéem,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼíi")]
    CL_SLASH_k̲ʼíi,
    /// hdn: (no comment)
    #[strum(serialize = "CL/k̲ʼún")]
    CL_SLASH_k̲ʼún,
    /// hdn: (no comment)
    #[strum(serialize = "CL/mál")]
    CL_SLASH_mál,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sda")]
    CL_SLASH_sda,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sdah")]
    CL_SLASH_sdah,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sdáam")]
    CL_SLASH_sdáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sdúu")]
    CL_SLASH_sdúu,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/sga")]
    CL_SLASH_sga,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sgab")]
    CL_SLASH_sgab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sgáam")]
    CL_SLASH_sgáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sgám")]
    CL_SLASH_sgám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sgíl")]
    CL_SLASH_sgíl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sgún")]
    CL_SLASH_sgún,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sg̲a")]
    CL_SLASH_sg̲a,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sg̲áam")]
    CL_SLASH_sg̲áam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sk")]
    CL_SLASH_sk,
    /// hdn: (no comment)
    #[strum(serialize = "CL/skáa")]
    CL_SLASH_skáa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/skám")]
    CL_SLASH_skám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/skáy")]
    CL_SLASH_skáy,
    /// hdn: (no comment)
    #[strum(serialize = "CL/skʼáam")]
    CL_SLASH_skʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/skʼál")]
    CL_SLASH_skʼál,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sk̲")]
    CL_SLASH_sk̲,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sk̲ʼihl")]
    CL_SLASH_sk̲ʼihl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sk̲ʼáam")]
    CL_SLASH_sk̲ʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sk̲ʼéehl")]
    CL_SLASH_sk̲ʼéehl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/smál")]
    CL_SLASH_smál,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stad")]
    CL_SLASH_stad,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stl")]
    CL_SLASH_stl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stlab")]
    CL_SLASH_stlab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stláam")]
    CL_SLASH_stláam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stlúu")]
    CL_SLASH_stlúu,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stlʼáam")]
    CL_SLASH_stlʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/stlʼúu")]
    CL_SLASH_stlʼúu,
    /// hdn: (no comment)
    #[strum(serialize = "CL/síi")]
    CL_SLASH_síi,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sʼab")]
    CL_SLASH_sʼab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sʼahl")]
    CL_SLASH_sʼahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/sʼyúu")]
    CL_SLASH_sʼyúu,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/t")]
    CL_SLASH_t,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tíi")]
    CL_SLASH_tíi,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tl")]
    CL_SLASH_tl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tláa")]
    CL_SLASH_tláa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlúu")]
    CL_SLASH_tlúu,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼab")]
    CL_SLASH_tlʼab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼad")]
    CL_SLASH_tlʼad,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼáahl")]
    CL_SLASH_tlʼáahl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼáam")]
    CL_SLASH_tlʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼál")]
    CL_SLASH_tlʼál,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼán")]
    CL_SLASH_tlʼán,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tlʼúu")]
    CL_SLASH_tlʼúu,
    /// hdn: (no comment)
    #[strum(serialize = "CL/ts")]
    CL_SLASH_ts,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tsʼám")]
    CL_SLASH_tsʼám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tsʼúu")]
    CL_SLASH_tsʼúu,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/tʼab")]
    CL_SLASH_tʼab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tʼáam")]
    CL_SLASH_tʼáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/tʼám")]
    CL_SLASH_tʼám,
    /// hdn: (no comment)
    #[strum(serialize = "CL/xa")]
    CL_SLASH_xa,
    /// hdn: (no comment)
    #[strum(serialize = "CL/xab")]
    CL_SLASH_xab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/xwáad")]
    CL_SLASH_xwáad,
    /// hdn: (no comment)
    #[strum(serialize = "CL/xáam")]
    CL_SLASH_xáam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/xáw")]
    CL_SLASH_xáw,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "CL/x̲a")]
    CL_SLASH_x̲a,
    /// hdn: (no comment)
    #[strum(serialize = "CL/x̲ab")]
    CL_SLASH_x̲ab,
    /// hdn: (no comment)
    #[strum(serialize = "CL/x̲aw")]
    CL_SLASH_x̲aw,
    /// hdn: (no comment)
    #[strum(serialize = "CL/x̲áam")]
    CL_SLASH_x̲áam,
    /// hdn: (no comment)
    #[strum(serialize = "CL/x̲úl")]
    CL_SLASH_x̲úl,
    /// hdn: (no comment)
    #[strum(serialize = "CL/x̲ún")]
    CL_SLASH_x̲ún,
    /// nso: (no comment)
    #[strum(serialize = "CL1")]
    CL1,
    /// fit: **@CODE@** = Clause boundary
    /// sje: (no comment)
    /// gle: **@CODE@** = Clause boundary
    /// kal: **@CODE@**
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Clause boundary
    /// izh: **@CODE@**  Special symbols
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = clause and sentence boundary symbols
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@**:  XXX These should be documented better
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@**
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// otw: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// rus: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Clause border (full stop, comma..)
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@**
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// xak: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// sme: **@CODE@**  Clause border (full stop, comma..)
    /// smn: **@CODE@**
    /// srs: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** clause boundary for period and comma
    /// ceb: (no comment)
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "CLB")]
    CLB,
    /// fit: **@CODE@** =
    /// gle: **@CODE@** = Final clause boundary
    /// swe: (no comment)
    /// fkv: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// sma: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// nob: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// fao: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// smj: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// mns: **@CODE@** Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// sms: **@CODE@** = Sentence final abbreviated expression ending in full stop, the full stop is ambiguous
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@**
    /// deu: (no comment)
    /// sme: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// smn: **@CODE@**  Sentence final abbreviated expression ending in full stop, the full stop is ambiguous
    /// fin: **@CODE@**  Sentence final abbreviated expression ending in full stop, so that the full stop is ambiguous
    /// mdf: (no comment)
    /// vro: **@CODE@**
    /// bak: (no comment)
    #[strum(serialize = "CLBfinal")]
    CLBfinal,
    /// gle: **@CODE@** = Canúint na Mumhan, Munster dialect
    #[strum(serialize = "CM")]
    CM,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "COMMA")]
    COMMA,
    /// srs: (no comment)
    #[strum(serialize = "CR")]
    CR,
    /// fit: **@CODE@** = Subjunction
    /// sje: (no comment)
    /// kpv: **@CODE@**:  subordinating conjunction      XX   подчинительный союз
    /// swe: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Subjunction
    /// izh: (no comment)
    /// liv: **@CODE@** = subordinating conjunction
    /// crk: (no comment)
    /// mhr: **@CODE@** = subjunctions
    /// rue: (no comment)
    /// sma: **@CODE@** = Subjunction
    /// tgl: **@CODE@** -  Subjunction
    /// nob: **@CODE@** = Closed POS (IM = **å**)
    /// hdn: (no comment)
    /// smj: **@CODE@** = Subjunction
    /// udm: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Subjunction
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// yrk: (no comment)
    /// yrk: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Subjunction
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = subordinating conjunction
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  subordinating conjunction
    /// koi: **@CODE@**:  subordinating conjunction
    /// deu: (no comment)
    /// sme: **@CODE@** - Subjunction
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  subordinating conjunction
    /// ceb: **@CODE@** -  Subjunction
    /// khk: (no comment)
    /// vro: **@CODE@** subjunction
    /// mpj: **@CODE@**
    /// kca: (no comment)
    #[strum(serialize = "CS")]
    CS,
    /// gle: **@CODE@** = Canúint Uladh, Ulster dialect
    #[strum(serialize = "CU")]
    CU,
    /// apu: **@CODE@** 0 N>V capacity
    #[strum(serialize = "Cap")]
    Cap,
    /// kpv: **@CODE@** caritive -тӧг торйӧдан
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// koi: (no comment)
    #[strum(serialize = "Car")]
    Car,
    /// sje: (no comment)
    /// gle: **@CODE@** = Cardinal number(one two three ...)
    /// kpv: **@CODE@**:  cardinal + NCard
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = (hmm, skip+Card?)
    /// sjd: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**  Numerals are classified under:
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@**:  cardinal + NCard
    /// sme: **@CODE@**   Cardinal Number Not in use
    /// fin: (no comment)
    /// mdf: **@CODE@**:  cardinal
    /// rmf: **@CODE@** Cardinal and ordinal numerals
    /// vro: **@CODE@**
    #[strum(serialize = "Card")]
    Card,
    /// kal: **@CODE@** = Causative
    /// slh: (no comment)
    /// apu: **@CODE@**	-xika 'because of'
    /// hun: (no comment)
    /// mdf: **@CODE@**:  causatative
    #[strum(serialize = "Cau")]
    Cau,
    /// apu: (no comment)
    /// apu: (no comment)
    /// cho: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Caus")]
    Caus,
    /// mpj: **@CODE@**  -ma	nominal -> T verb. (particularly for attributes)
    #[strum(serialize = "Caus/Make")]
    Caus_SLASH_Make,
    /// mpj: **@CODE@**  -ju	nominal -> T verb.
    #[strum(serialize = "Caus/PutTo")]
    Caus_SLASH_PutTo,
    /// hdn: (no comment)
    /// cho: (no comment)
    #[strum(serialize = "Cert")]
    Cert,
    /// mpj: **@CODE@**  = Characteristic (*payi may behave differently - nominalisation?)
    #[strum(serialize = "Char")]
    Char,
    /// hdn: (no comment)
    #[strum(serialize = "Circum")]
    Circum,
    /// som: (no comment)
    #[strum(serialize = "Clit/CS")]
    Clit_SLASH_CS,
    /// som: (no comment)
    #[strum(serialize = "Clit/Without")]
    Clit_SLASH_Without,
    /// som: (no comment)
    #[strum(serialize = "Clit/ba")]
    Clit_SLASH_ba,
    /// som: (no comment)
    #[strum(serialize = "Clit/na")]
    Clit_SLASH_na,
    /// som: (no comment)
    #[strum(serialize = "Clit/oo")]
    Clit_SLASH_oo,
    /// som: (no comment)
    #[strum(serialize = "Clit/se")]
    Clit_SLASH_se,
    /// som: (no comment)
    #[strum(serialize = "Close")]
    Close,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** =
    /// nob: **@CODE@** = the so-called "genitive s"
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// skf: (no comment)
    /// skf: (no comment)
    /// skf: (no comment)
    /// nio: (no comment)
    /// sms: **@CODE@**
    /// vep: **@CODE@**  =
    /// koi: (no comment)
    /// koi: (no comment)
    /// koi: (no comment)
    /// rmf: **@CODE@** clitic
    /// rmf: (no comment)
    /// vro: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Clt")]
    Clt,
    /// koi: (no comment)
    #[strum(serialize = "Clt/-to")]
    Clt_SLASH__MINUS_to,
    /// koi: (no comment)
    #[strum(serialize = "Clt/-tu")]
    Clt_SLASH__MINUS_tu,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Clt/Add")]
    Clt_SLASH_Add,
    /// mdf: (no comment)
    #[strum(serialize = "Clt/AddGA")]
    Clt_SLASH_AddGA,
    /// myv: (no comment)
    #[strum(serialize = "Clt/AddGak")]
    Clt_SLASH_AddGak,
    /// mdf: (no comment)
    #[strum(serialize = "Clt/AddJok")]
    Clt_SLASH_AddJok,
    /// mdf: (no comment)
    #[strum(serialize = "Clt/AddNgA")]
    Clt_SLASH_AddNgA,
    /// mdf: (no comment)
    #[strum(serialize = "Clt/AddVok")]
    Clt_SLASH_AddVok,
    /// mdf: (no comment)
    #[strum(serialize = "Clt/Aram")]
    Clt_SLASH_Aram,
    /// mpj: **@CODE@**  =  ngulyu
    #[strum(serialize = "Clt/Cert")]
    Clt_SLASH_Cert,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Cond")]
    Clt_SLASH_Cond,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Clt/Cop")]
    Clt_SLASH_Cop,
    /// mpj: **@CODE@**  =  pa
    #[strum(serialize = "Clt/Dub")]
    Clt_SLASH_Dub,
    /// mpj: **@CODE@**  =  kaja, rtuka, rtu
    #[strum(serialize = "Clt/Emph")]
    Clt_SLASH_Emph,
    /// mpj: **@CODE@** =
    #[strum(serialize = "Clt/Foc")]
    Clt_SLASH_Foc,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Ga")]
    Clt_SLASH_Ga,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gaj")]
    Clt_SLASH_Gaj,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gaja")]
    Clt_SLASH_Gaja,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gajatj")]
    Clt_SLASH_Gajatj,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gajatja")]
    Clt_SLASH_Gajatja,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gaka")]
    Clt_SLASH_Gaka,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gakaja")]
    Clt_SLASH_Gakaja,
    /// myv: (no comment)
    #[strum(serialize = "Clt/Gatja")]
    Clt_SLASH_Gatja,
    /// krl: (no comment)
    #[strum(serialize = "Clt/Han")]
    Clt_SLASH_Han,
    /// kpv: **@CODE@**  This comes at the end of a word -и or after vowels (some authors use -й)
    /// kpv: (no comment)
    #[strum(serialize = "Clt/I")]
    Clt_SLASH_I,
    /// krl: (no comment)
    /// mns: **@CODE@** With Pron+Prs+Nom
    #[strum(serialize = "Clt/Ki")]
    Clt_SLASH_Ki,
    /// kpv: **@CODE@** adjectival phrase
    #[strum(serialize = "Clt/Kodj")]
    Clt_SLASH_Kodj,
    /// kpv: **@CODE@** adverbial clause
    #[strum(serialize = "Clt/Moz")]
    Clt_SLASH_Moz,
    /// lut: (no comment)
    /// lut: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Clt/Now")]
    Clt_SLASH_Now,
    /// krl: (no comment)
    #[strum(serialize = "Clt/Pa")]
    Clt_SLASH_Pa,
    /// mpj: **@CODE@** = kirli
    #[strum(serialize = "Clt/Prob")]
    Clt_SLASH_Prob,
    /// mpj: **@CODE@**  =  nyu
    #[strum(serialize = "Clt/Rep")]
    Clt_SLASH_Rep,
    /// mns: **@CODE@** With Pron+Prs+Nom
    #[strum(serialize = "Clt/Ti")]
    Clt_SLASH_Ti,
    /// olo: (no comment)
    #[strum(serialize = "Clt/bo")]
    Clt_SLASH_bo,
    /// mpj: **@CODE@**  lka
    #[strum(serialize = "Clt/contrary_to_expectation")]
    Clt_SLASH_contrary_to_expectation,
    /// olo: (no comment)
    #[strum(serialize = "Clt/gi")]
    Clt_SLASH_gi,
    /// olo: (no comment)
    #[strum(serialize = "Clt/hAi")]
    Clt_SLASH_hAi,
    /// izh: **@CODE@**
    #[strum(serialize = "Clt/kAA")]
    Clt_SLASH_kAA,
    /// mpj: **@CODE@**  =
    #[strum(serialize = "Clt/really")]
    Clt_SLASH_really,
    /// kpv: **@CODE@** -сӧ
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Clt/soe")]
    Clt_SLASH_soe,
    /// mpj: **@CODE@**  =  yila, lta
    #[strum(serialize = "Clt/then")]
    Clt_SLASH_then,
    /// kpv: **@CODE@** -тӧ
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// kpv: (no comment)
    #[strum(serialize = "Clt/toe")]
    Clt_SLASH_toe,
    /// mpj: **@CODE@**  =  la
    #[strum(serialize = "Clt/when")]
    Clt_SLASH_when,
    /// mpj: **@CODE@**  =  kaji
    #[strum(serialize = "Clt/while")]
    Clt_SLASH_while,
    /// koi: (no comment)
    #[strum(serialize = "Clt/а")]
    Clt_SLASH__CYRILLIC_a,
    /// koi: (no comment)
    #[strum(serialize = "Clt/ӧ")]
    Clt_SLASH_ӧ,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Clth")]
    Clth,
    /// smj: (no comment)
    #[strum(serialize = "Cltl")]
    Cltl,
    /// khk: (no comment)
    #[strum(serialize = "Cm")]
    Cm,
    /// gle: **@CODE@** =  Communicator (yeah, y'know) in transcribed speech
    #[strum(serialize = "Cmc")]
    Cmc,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// sje: **@CODE@**  Dynamic compound - this tag should always be part of a
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// pma: (no comment)
    /// mhr: **@CODE@** = nouns
    /// krl: (no comment)
    /// sma: **@CODE@** | Dynamic compound - this tag should *always* be part of a dynamic compound. It is important for  Apertium and the speller (to give extra weights to compounds), and useful in other cases as well.
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@**  Dynamic compound - this tag should always be part of a dynamic compound.
    /// mns: **@CODE@** compounding
    /// sms: **@CODE@**
    /// sms: **@CODE@** - Dynamic compound. This tag should always be part
    /// rmn: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** - Dynamic compound. This tag should always be part
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Dynamic compound. This tag should always be part
    /// smn: **@CODE@** This tag to mark there is a compound. It comes after the Cmp/xxx tag.
    /// non: (no comment)
    /// fin: **@CODE@** - Dynamic compound. This tag should always be part
    /// mdf: (no comment)
    /// vro: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Cmp")]
    Cmp,
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// pma: (no comment)
    #[strum(serialize = "Cmp#")]
    Cmp_POUND,
    /// sje: **@CODE@**  Attribute
    /// sma: **@CODE@** | Compounding using attribute form
    /// smj: **@CODE@**  Attribute
    /// sms: **@CODE@** - Attributive
    /// olo: **@CODE@** compounds where first part is Attr
    /// sme: **@CODE@** - Attributive
    /// smn: **@CODE@** compounds where first part is Attr
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
    /// mns: **@CODE@** collective noun with inflection in two stems а̄гирищит-пы̄грищит
    #[strum(serialize = "Cmp/Coll")]
    Cmp_SLASH_Coll,
    /// sma: **@CODE@** | Deletion of final **e**, as in *voelem-gaaroeh*, from *voeleme*
    #[strum(serialize = "Cmp/FinEDel")]
    Cmp_SLASH_FinEDel,
    /// fit: **@CODE@** - on dynamic compounds that have a hyphen (in use?)
    /// sje: (no comment)
    /// fkv: **@CODE@** - on dynamic compounds that have a hyphen (in use?)
    /// mhr: **@CODE@** = nouns
    /// sma: **@CODE@**:  A tag to indicate that a hyphen was used when
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - on dynamic compounds that have a hyphen
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// myv: (no comment)
    /// olo: **@CODE@** compounds where first part ends in -
    /// sme: **@CODE@** - on dynamic compounds that have a hyphen
    /// smn: **@CODE@** compounds where first part ends in -
    /// fin: **@CODE@** compounds where first part ends in -
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Cmp/Hyph")]
    Cmp_SLASH_Hyph,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Cmp/Hyph-Coll")]
    Cmp_SLASH_Hyph_MINUS_Coll,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Cmp/Hyph-Redup")]
    Cmp_SLASH_Hyph_MINUS_Redup,
    /// myv: (no comment)
    /// mdf: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Cmp/Hyph-Serial")]
    Cmp_SLASH_Hyph_MINUS_Serial,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Cmp/Hyph-Synonym")]
    Cmp_SLASH_Hyph_MINUS_Synonym,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Cmp/Hyph-tejems")]
    Cmp_SLASH_Hyph_MINUS_tejems,
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
    /// deu: (no comment)
    #[strum(serialize = "Cmp/Pl")]
    Cmp_SLASH_Pl,
    /// sje: **@CODE@**  Plural Genitiv
    /// sma: **@CODE@** | Compounding using genitive plural
    /// smj: **@CODE@**  Plural Genitiv
    /// sms: **@CODE@** - Plural Genitiv
    /// olo: **@CODE@** compounds where first part is Pl Gen
    /// sme: **@CODE@** - Plural Genitiv
    /// smn: **@CODE@** compounds where first part is Pl Gen
    /// fin: **@CODE@** compounds where first part is Pl Gen
    #[strum(serialize = "Cmp/PlGen")]
    Cmp_SLASH_PlGen,
    /// sje: **@CODE@**  Plural Nominative
    /// smj: **@CODE@**  Plural Nominative
    /// olo: **@CODE@** compounds where first part is Pl Nom
    /// smn: **@CODE@** compounds where first part is Pl Nom
    /// fin: **@CODE@** compounds where first part is Pl Nom
    #[strum(serialize = "Cmp/PlNom")]
    Cmp_SLASH_PlNom,
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// kpv: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Cmp/Serial")]
    Cmp_SLASH_Serial,
    /// sje: **@CODE@**  Singular
    /// sma: **@CODE@** | Compounding using an unspecified singular stem
    /// smj: **@CODE@**  Singular
    /// deu: (no comment)
    #[strum(serialize = "Cmp/Sg")]
    Cmp_SLASH_Sg,
    /// sje: (no comment)
    /// sje: **@CODE@**  Singular Genitive
    /// sma: **@CODE@** | Compounding using genitive singular
    /// smj: **@CODE@**  Singular Genitive
    /// sms: **@CODE@** - Singular Genitive
    /// olo: **@CODE@** compounds where first part is Sg Gen
    /// sme: **@CODE@** - Singular Genitive
    /// smn: **@CODE@** compounds where first part is Sg Gen
    /// fin: **@CODE@** compounds where first part is Sg Gen
    #[strum(serialize = "Cmp/SgGen")]
    Cmp_SLASH_SgGen,
    /// sje: (no comment)
    /// sje: **@CODE@**  Singular Nominative
    /// sma: **@CODE@** | Compounding using nominative singular
    /// smj: **@CODE@**  Singular Nominative
    /// sms: **@CODE@** - Singular Nominative
    /// vep: **@CODE@**  = compound words
    /// olo: **@CODE@** compounds where first part is Sg Nom
    /// sme: **@CODE@** - Singular Nominative
    /// smn: **@CODE@** compounds where first part is Sg Nom
    /// fin: **@CODE@** compounds where first part is Sg Nom
    #[strum(serialize = "Cmp/SgNom")]
    Cmp_SLASH_SgNom,
    /// sje: **@CODE@**  testing ShCmp
    /// sma: **@CODE@** | Compounding using a short stem: *–biejj–* (from *biejjie*)
    /// smj: **@CODE@**  testing ShCmp
    /// sms: **@CODE@** - testing +Cmp/Sh
    /// olo: **@CODE@** compounds where first part is a short form
    /// sme: **@CODE@** - Tag for marking short form compound stems
    /// smn: **@CODE@** compounds where first part is a short form
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
    /// sje: **@CODE@**  This is a split compound with the other part to the left
    /// sma: **@CODE@** | This is a split compound with the other part to the left, this is the oposite of the previous case
    /// smj: **@CODE@**  This is a split compound with the other part to the left
    /// sms: **@CODE@** - This is a split compound with the other part to the left
    /// sms: (no comment)
    /// sme: **@CODE@** - This is a split compound with the split part to the right
    #[strum(serialize = "Cmp/SplitL")]
    Cmp_SLASH_SplitL,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// sje: **@CODE@**  This is a split compound with the other part to the right:
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// mhr: **@CODE@** = nouns
    /// sma: **@CODE@** | This is a split compound with the other part to the right: <br/> "Arbeids- og inkluderingsdepartementet" => *Arbeids–* = **+Cmp/SplitR**
    /// smj: **@CODE@**  This is a split compound with the other part to the right:
    /// sms: **@CODE@** - This is a split compound with the other part to
    /// vep: **@CODE@**
    /// tkl: (no comment)
    /// olo: **@CODE@** compounds where first part is a split compound hmm
    /// sme: **@CODE@** - This is a split compound with the split part to
    /// smn: **@CODE@** compounds where first part is a split compound hmm
    /// fin: **@CODE@** compounds where first part is a split compound hmm
    /// mdf: (no comment)
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
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | Plural Genitive
    /// smj: **@CODE@** = Plural Genitive
    /// sms: **@CODE@**
    /// sme: **@CODE@** Plural Genitive
    /// smn: **@CODE@** Plural Genitive
    /// fin: **@CODE@** Plural Genitive
    #[strum(serialize = "CmpN/PlG")]
    CmpN_SLASH_PlG,
    /// sje: (no comment)
    /// sma: **@CODE@** | Pl Gen to the left
    /// smj: (no comment)
    /// sme: **@CODE@** Plural Genitive
    /// smn: **@CODE@** Plural Genitive
    /// fin: **@CODE@** Plural Genitive
    #[strum(serialize = "CmpN/PlGenLeft")]
    CmpN_SLASH_PlGenLeft,
    /// sme: **@CODE@** Plural Nominative, propers!
    /// fin: **@CODE@** Plural Nominative, propers!
    #[strum(serialize = "CmpN/PlN")]
    CmpN_SLASH_PlN,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | Singular
    /// smj: **@CODE@** = Singular
    #[strum(serialize = "CmpN/Sg")]
    CmpN_SLASH_Sg,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | Singular Genitive
    /// smj: **@CODE@** = Singular Genitive
    /// sms: **@CODE@**
    /// sme: **@CODE@** Singular Genitive
    /// smn: **@CODE@** Singular Genitive
    /// fin: **@CODE@** Singular Genitive
    /// vro: **@CODE@** Singular Genitive
    #[strum(serialize = "CmpN/SgG")]
    CmpN_SLASH_SgG,
    /// sje: (no comment)
    /// sma: **@CODE@** | Sg Gen to the left
    /// smj: (no comment)
    /// sme: **@CODE@** Singular Genitive
    /// smn: **@CODE@** Singular Genitive
    /// fin: **@CODE@** Singular Genitive
    #[strum(serialize = "CmpN/SgGenLeft")]
    CmpN_SLASH_SgGenLeft,
    /// sje: (no comment)
    /// sma: **@CODE@** | Sg to the left
    /// smj: (no comment)
    #[strum(serialize = "CmpN/SgLeft")]
    CmpN_SLASH_SgLeft,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | Singular Nominative
    /// smj: **@CODE@** = Singular Nominative
    /// sms: **@CODE@**
    /// sme: **@CODE@** Singular Nominative
    /// smn: **@CODE@** Singular Nominative
    /// fin: **@CODE@** Singular Nominative
    /// vro: **@CODE@** Singular Nominative
    #[strum(serialize = "CmpN/SgN")]
    CmpN_SLASH_SgN,
    /// sje: (no comment)
    /// sma: **@CODE@** | Sg Nom to the left
    /// smj: (no comment)
    /// sme: **@CODE@** Singular Nominative
    /// smn: **@CODE@** Singular Nominative
    /// fin: **@CODE@** Singular Nominative
    #[strum(serialize = "CmpN/SgNomLeft")]
    CmpN_SLASH_SgNomLeft,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | ... be in all positions, **default**, this tag does not have to be written
    /// nob: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    /// fao: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    /// smj: (no comment)
    /// sme: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    /// smn: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    /// fin: **@CODE@** - ... in all positions, **default**, this tag does not have to be written
    #[strum(serialize = "CmpNP/All")]
    CmpNP_SLASH_All,
    /// fit: **@CODE@** - ... only be first part in a compound or alone
    /// sje: (no comment)
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** - ... only be first part in a compound or alone
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be first part in a compound or alone
    /// nob: **@CODE@** - ... only be first part in a compound or alone
    /// fao: **@CODE@** - ... only be first part in a compound or alone
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - ... only be first part in a compound or alone
    /// smn: **@CODE@** - ... only be first part in a compound or alone
    /// fin: **@CODE@** - ... only be first part in a compound or alone
    /// bak: (no comment)
    #[strum(serialize = "CmpNP/First")]
    CmpNP_SLASH_First,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be last part in a compound or alone
    /// nob: **@CODE@** - ... only be last part in a compound or alone
    /// fao: **@CODE@** - ... only be last part in a compound or alone
    /// smj: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@** - ... only be last part in a compound or alone
    /// smn: **@CODE@** - ... only be last part in a compound or alone
    /// fin: **@CODE@** - ... only be last part in a compound or alone
    #[strum(serialize = "CmpNP/Last")]
    CmpNP_SLASH_Last,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// liv: (no comment)
    /// sma: **@CODE@** | ... not take part in compounds
    /// nob: **@CODE@** - ... does not take part in compounds
    /// fao: **@CODE@** - ... does not take part in compounds
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - ... does not take part in compounds
    /// smn: **@CODE@** - ... does not take part in compounds
    /// fin: **@CODE@** - ... does not take part in compounds
    /// bak: (no comment)
    #[strum(serialize = "CmpNP/None")]
    CmpNP_SLASH_None,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be part of a compound, i.e. can never be used alone, but can appear in any position
    /// nob: **@CODE@** - ... only be part of a compound, i.e. can never
    /// fao: **@CODE@** - ... only be part of a compound, i.e. can never
    /// smj: (no comment)
    /// sme: **@CODE@** - ... only be part of a compound, i.e. can never
    /// smn: **@CODE@** - ... only be part of a compound, i.e. can never be used alone, but can appear in any position
    /// fin: **@CODE@** - ... only be part of a compound, i.e. can never
    #[strum(serialize = "CmpNP/Only")]
    CmpNP_SLASH_Only,
    /// sje: (no comment)
    /// liv: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be **first** part in a compound, NEVER alone
    /// nob: **@CODE@** - ... only **first** part in a compound, NEVER alone
    /// fao: **@CODE@** - ... only **first** part in a compound, NEVER alone
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// sme: **@CODE@** - ... only **first** part in a compound, NEVER alone
    /// smn: **@CODE@** - ... only **first** part in a compound, NEVER alone
    /// fin: **@CODE@** - ... only **first** part in a compound, NEVER alone
    #[strum(serialize = "CmpNP/Pref")]
    CmpNP_SLASH_Pref,
    /// sje: (no comment)
    /// liv: (no comment)
    /// sma: **@CODE@** | ... only be **last** part in a compound, NEVER alone
    /// nob: **@CODE@** - ... only **last** part in a compound, NEVER alone
    /// fao: **@CODE@** - ... only **last** part in a compound, NEVER alone
    /// smj: (no comment)
    /// sme: **@CODE@** - ... only **last** part in a compound, NEVER alone
    /// smn: **@CODE@** - ... only **last** part in a compound, NEVER alone
    /// fin: **@CODE@** - ... only **last** part in a compound, NEVER alone
    #[strum(serialize = "CmpNP/Suff")]
    CmpNP_SLASH_Suff,
    /// swe: (no comment)
    #[strum(serialize = "CmpS")]
    CmpS,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "CmpTest")]
    CmpTest,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Cmpar")]
    Cmpar,
    /// gle: **@CODE@** = Compound prepostion
    #[strum(serialize = "Cmpd")]
    Cmpd,
    /// gle: **@CODE@** = Compound prepostion which does not require genitive case on object NP
    #[strum(serialize = "CmpdNoGen")]
    CmpdNoGen,
    /// gle: **@CODE@** = Complementizer: go, gur, nach, nár
    /// kpv: **@CODE@**  Postposition complement
    /// udm: (no comment)
    /// koi: **@CODE@**  Postposition complement
    /// koi: (no comment)
    #[strum(serialize = "Cmpl")]
    Cmpl,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Cmpnd")]
    Cmpnd,
    /// mdf: **@CODE@**:  compositional numeral 14, 24, 34, 44 нилие,
    #[strum(serialize = "Cmpos")]
    Cmpos,
    /// kpv: **@CODE@** Comparative case form -ся ӧткодялан
    /// mhr: **@CODE@** = comparative case
    /// myv: (no comment)
    /// koi: (no comment)
    /// mdf: **@CODE@**:  comparative -шка
    #[strum(serialize = "Cmpr")]
    Cmpr,
    /// fin: (no comment)
    #[strum(serialize = "Cmt")]
    Cmt,
    /// khk: (no comment)
    #[strum(serialize = "Cna_Cnd")]
    Cna_Cnd,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Cnj")]
    Cnj,
    /// kpv: **@CODE@** consecultative -ла могман
    /// som: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Cns")]
    Cns,
    /// khk: (no comment)
    #[strum(serialize = "Cog")]
    Cog,
    /// apu: (no comment)
    #[strum(serialize = "ColTV")]
    ColTV,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// fkv: **@CODE@** = Collective numeral
    /// mhr: **@CODE@** = Collective numerals -ын-
    /// rue: (no comment)
    /// sma: **@CODE@** = Collective numeral
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = collective numerals
    /// skf: (no comment)
    /// apu: **@CODE@** Collective numerals, subtag for +N
    /// rus: (no comment)
    /// mns: **@CODE@** - Collective numeral
    /// sms: **@CODE@** - Collective numerals
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@**  collective, probably from a common file.
    /// koi: **@CODE@**:  collective
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Collective numerals, subtag for +N
    /// smn: **@CODE@** - Collective numerals, subtag for +N
    /// fin: (no comment)
    /// mdf: **@CODE@**:  Collective
    /// khk: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Coll")]
    Coll,
    /// kpv: **@CODE@** used with paired nouns **collective nouns**
    /// koi: (no comment)
    /// mdf: **@CODE@**:  used with paired nouns **collective nouns**
    #[strum(serialize = "CollN")]
    CollN,
    /// fit: **@CODE@** = Comitative
    /// sje: (no comment)
    /// gle: **@CODE@** = Common case (nominative/accusative/dative case)
    /// kpv: **@CODE@** Comitative -кӧд ӧтвывтан
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Comitative
    /// mhr: **@CODE@** = comitative
    /// evn: (no comment)
    /// sma: **@CODE@**= Comitative
    /// fao: (no comment)
    /// smj: **@CODE@** = Comitative case
    /// udm: (no comment)
    /// skf: (no comment)
    /// apu: **@CODE@** Comitative -kata  (Assoc)
    /// sms: **@CODE@** Comitative
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// grn: (no comment)
    /// olo: **@CODE@** comitative
    /// koi: (no comment)
    /// xak: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Comitative
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: **@CODE@**:  comitative -нек
    /// khk: (no comment)
    /// vro: **@CODE@** comitative
    #[strum(serialize = "Com")]
    Com,
    /// vot: (no comment)
    /// lit: (no comment)
    /// sme: **@CODE@** - Comitative Plural Hyphened Shortform (w/o -guin), ie Beatnagii-, Biillai-, Bohccui- etc.
    /// vro: **@CODE@**
    #[strum(serialize = "Com/Sh")]
    Com_SLASH_Sh,
    /// tgl: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Comm")]
    Comm,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// gle: **@CODE@** = Comparative adjective (c)
    /// kpv: **@CODE@** джык
    /// kpv: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: **@CODE@** comparative grade
    /// izh: **@CODE@** comparative tags
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** = comparative (not: not Cmp)
    /// xal: (no comment)
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: **@CODE@** -
    /// got: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** нув in "арге̄н" Adv+Comp аргеннув
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  =
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: **@CODE@**:  comparative
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  comparative as opposed to superlative
    /// ale: (no comment)
    /// rmf: **@CODE@** Comparative, superlative
    /// ceb: **@CODE@** -
    /// ceb: **@CODE@** comparative mas -- this is not a derivation but a separate word
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Comp")]
    Comp,
    /// kpv: **@CODE@**
    #[strum(serialize = "CompMod")]
    CompMod,
    /// fkv: **@CODE@** = Comparative
    #[strum(serialize = "Compar")]
    Compar,
    /// mpj: **@CODE@** = completed action
    #[strum(serialize = "Compl")]
    Compl,
    /// mpj: **@CODE@** = kati
    #[strum(serialize = "Compound/carry")]
    Compound_SLASH_carry,
    /// mpj: **@CODE@** = pakala
    #[strum(serialize = "Compound/get_up")]
    Compound_SLASH_get_up,
    /// mpj: **@CODE@** = yarra
    #[strum(serialize = "Compound/go")]
    Compound_SLASH_go,
    /// mpj: **@CODE@** = kanyila
    #[strum(serialize = "Compound/have")]
    Compound_SLASH_have,
    /// mpj: **@CODE@** = puwa
    #[strum(serialize = "Compound/hit")]
    Compound_SLASH_hit,
    /// mpj: **@CODE@** = ngarrin
    #[strum(serialize = "Compound/lie")]
    Compound_SLASH_lie,
    /// mpj: **@CODE@** = jarra
    #[strum(serialize = "Compound/mouth_action")]
    Compound_SLASH_mouth_action,
    /// mpj: **@CODE@** = jurra
    #[strum(serialize = "Compound/put")]
    Compound_SLASH_put,
    /// kal: **@CODE@** = Conditional
    /// oji: (no comment)
    /// otw: (no comment)
    /// cho: (no comment)
    #[strum(serialize = "Con")]
    Con,
    /// bxr: (no comment)
    #[strum(serialize = "ConAbtmp")]
    ConAbtmp,
    /// bxr: (no comment)
    #[strum(serialize = "ConCmp")]
    ConCmp,
    /// bxr: (no comment)
    #[strum(serialize = "ConCntmp")]
    ConCntmp,
    /// bxr: (no comment)
    #[strum(serialize = "ConConc")]
    ConConc,
    /// bxr: (no comment)
    #[strum(serialize = "ConCond")]
    ConCond,
    /// bxr: (no comment)
    #[strum(serialize = "ConFin")]
    ConFin,
    /// bxr: (no comment)
    #[strum(serialize = "ConImpf")]
    ConImpf,
    /// bxr: (no comment)
    #[strum(serialize = "ConIntnt")]
    ConIntnt,
    /// bxr: (no comment)
    #[strum(serialize = "ConMod")]
    ConMod,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Negation form of verb
    /// liv: **@CODE@** = connegative, main verb complement to Neg,
    /// mhr: **@CODE@** = Invariant main verb in negation expression
    /// sma: **@CODE@** | main verb complement to Neg, form identical to Imp
    /// smj: (no comment)
    /// udm: (no comment)
    /// crj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Negation Form, i.e. mõõn, reäǥǥ, jueʹjj: Indicative present except indef person; imperative Sg2 and Pl2
    /// tku: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**
    /// koi: (no comment)
    /// sme: **@CODE@** Negation Form, ie Mana, Doalvvo, Juoge etc
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@** saa eiq 3 elements in 2 orthographic units
    #[strum(serialize = "ConNeg")]
    ConNeg,
    /// vot: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Negation Form II: mõnnu, riâkˈku, juõkˈku: Indicative present indef person; imperative Sg3, Pl1, Pl2, Pl3 (there is an overlap at +Impert+Pl2)
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// myv: (no comment)
    /// sme: **@CODE@** Alternative, Rather Declamatory Negation Form - Infrequent
    /// mdf: (no comment)
    /// vro: **@CODE@** ei saaq 3 elements in 2 orthographic units
    #[strum(serialize = "ConNegII")]
    ConNegII,
    /// bxr: (no comment)
    #[strum(serialize = "ConPrf")]
    ConPrf,
    /// bxr: (no comment)
    #[strum(serialize = "ConSucc")]
    ConSucc,
    /// bxr: (no comment)
    #[strum(serialize = "ConTerm")]
    ConTerm,
    /// kpv: **@CODE@** Used for calling animals, for example брысь, баль-баль, ...
    /// koi: (no comment)
    #[strum(serialize = "Conative")]
    Conative,
    /// fit: **@CODE@** =
    /// gle: **@CODE@** = Conditional mood
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Conditional
    /// liv: **@CODE@** = conditional
    /// crk: (no comment)
    /// mhr: **@CODE@** = conditional
    /// sma: **@CODE@** | Kondisjonalis, for one form:  lidtjie. To be looked at.+ lidtjim, + lidtjih
    /// smj: (no comment)
    /// udm: (no comment)
    /// epo: (no comment)
    /// crj: (no comment)
    /// sms: **@CODE@** Conditional mood
    /// bxr: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = conditional
    /// myv: (no comment)
    /// olo: **@CODE@** Conditional
    /// koi: (no comment)
    /// xak: (no comment)
    /// eus: (no comment)
    /// sme: **@CODE@** Conditional
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  conditional ндяря- protasis
    /// vro: **@CODE@**
    #[strum(serialize = "Cond")]
    Cond,
    /// hdn: (no comment)
    #[strum(serialize = "Cond-Aux1")]
    Cond_MINUS_Aux1,
    /// hdn: (no comment)
    #[strum(serialize = "Cond-Aux2")]
    Cond_MINUS_Aux2,
    /// eus: (no comment)
    #[strum(serialize = "Condfin")]
    Condfin,
    /// bxr: (no comment)
    #[strum(serialize = "Conf")]
    Conf,
    /// gle: **@CODE@** = Conjunction
    /// kal: **@CODE@** = Conjunction
    /// udm: (no comment)
    /// otw: (no comment)
    /// yrk: **@CODE@**
    /// mrj: **@CODE@** =
    /// tat: (no comment)
    /// tat: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// mdf: **@CODE@**:  conjunctional "Оль"
    /// fro: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "Conj")]
    Conj,
    /// koi: (no comment)
    #[strum(serialize = "Cons")]
    Cons,
    /// kal: **@CODE@** = Infinitive
    /// tgl: **@CODE@** - Continuative
    /// hdn: (no comment)
    /// ceb: **@CODE@** -
    /// tlh: (no comment)
    #[strum(serialize = "Cont")]
    Cont,
    /// kal: **@CODE@** = Negated Infinitive
    #[strum(serialize = "ContNeg")]
    ContNeg,
    /// apu: (no comment)
    /// mpj: **@CODE@**  Contradictive
    #[strum(serialize = "Contr")]
    Contr,
    /// myv: (no comment)
    #[strum(serialize = "ConvPrc")]
    ConvPrc,
    /// khk: (no comment)
    #[strum(serialize = "Coop")]
    Coop,
    /// gle: **@CODE@** = Coordinating conjunction
    /// kpv: **@CODE@**:  Coordinates, i.e. 65˚36′8,30″ in numerals.lexc
    /// koi: **@CODE@**:  Coordinates, i.e. 65˚36′8,30″ in numerals.lexc
    /// fro: (no comment)
    #[strum(serialize = "Coord")]
    Coord,
    /// gle: **@CODE@** = Copula
    /// kpv: **@CODE@**:  this is for copula complement predicate position with pl in -ӧсь depricated Pred
    /// skf: (no comment)
    /// koi: **@CODE@**:  this is for copula complement position with pl in -ӧсь Pred depricated
    /// khk: (no comment)
    #[strum(serialize = "Cop")]
    Cop,
    /// gle: **@CODE@** = correction in Learner corpus
    #[strum(serialize = "Corr")]
    Corr,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Count")]
    Count,
    /// gle: **@CODE@** = ?
    #[strum(serialize = "Cp")]
    Cp,
    /// hdn: (no comment)
    #[strum(serialize = "Ctfact")]
    Ctfact,
    /// gle: **@CODE@** = Currency symbols
    #[strum(serialize = "Curr")]
    Curr,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "D")]
    D,
    /// bla: (no comment)
    #[strum(serialize = "D3Sg")]
    D3Sg,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "DASH")]
    DASH,
    /// khk: (no comment)
    #[strum(serialize = "DH")]
    DH,
    /// som: (no comment)
    #[strum(serialize = "DV")]
    DV,
    /// fin: (no comment)
    #[strum(serialize = "Dash")]
    Dash,
    /// gle: **@CODE@** = Dative case (e.g. chois) fossilised forms
    /// kpv: **@CODE@** dative case -лы сетан
    /// swe: (no comment)
    /// liv: **@CODE@** = dative case
    /// mhr: **@CODE@** = dative
    /// evn: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@** = for fixed expressions *i live*
    /// got: (no comment)
    /// udm: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: **@CODE@** Dative -munhi (allative)
    /// yrk: **@CODE@**
    /// mns: **@CODE@** Dative
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = dative case
    /// cho: (no comment)
    /// myv: (no comment)
    /// sqi: (no comment)
    /// olo: **@CODE@** dative case
    /// koi: (no comment)
    /// xak: (no comment)
    /// non: (no comment)
    /// mdf: **@CODE@**:  dative
    /// rmf: **@CODE@** Dative
    /// khk: (no comment)
    /// mpj: **@CODE@**
    /// bak: (no comment)
    #[strum(serialize = "Dat")]
    Dat,
    /// eus: (no comment)
    #[strum(serialize = "Dat/Pl1")]
    Dat_SLASH_Pl1,
    /// eus: (no comment)
    #[strum(serialize = "Dat/Pl2")]
    Dat_SLASH_Pl2,
    /// eus: (no comment)
    #[strum(serialize = "Dat/Pl3")]
    Dat_SLASH_Pl3,
    /// eus: (no comment)
    #[strum(serialize = "Dat/Sg1")]
    Dat_SLASH_Sg1,
    /// eus: (no comment)
    #[strum(serialize = "Dat/Sg2")]
    Dat_SLASH_Sg2,
    /// eus: (no comment)
    #[strum(serialize = "Dat/Sg3")]
    Dat_SLASH_Sg3,
    /// gle: **@CODE@** = Adjectives drived from proper nouns, e.g. Albanach (Scottish adjective), not the same as Albanais (Scottish language noun)
    #[strum(serialize = "DeNom")]
    DeNom,
    /// liv: **@CODE@** =
    #[strum(serialize = "Deb")]
    Deb,
    /// som: (no comment)
    #[strum(serialize = "Decl/1")]
    Decl_SLASH_1,
    /// som: (no comment)
    #[strum(serialize = "Decl/2")]
    Decl_SLASH_2,
    /// som: (no comment)
    #[strum(serialize = "Decl/2A")]
    Decl_SLASH_2A,
    /// som: (no comment)
    #[strum(serialize = "Decl/2B")]
    Decl_SLASH_2B,
    /// som: (no comment)
    #[strum(serialize = "Decl/3")]
    Decl_SLASH_3,
    /// som: (no comment)
    #[strum(serialize = "Decl/4")]
    Decl_SLASH_4,
    /// som: (no comment)
    #[strum(serialize = "Decl/5")]
    Decl_SLASH_5,
    /// som: (no comment)
    #[strum(serialize = "Decl/6")]
    Decl_SLASH_6,
    /// som: (no comment)
    #[strum(serialize = "Decl/7")]
    Decl_SLASH_7,
    /// gle: **@CODE@** = Definite article
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// evn: (no comment)
    /// rue: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// kio: (no comment)
    /// mdf: **@CODE@**:  Definite
    /// fro: (no comment)
    #[strum(serialize = "Def")]
    Def,
    /// gle: **@CODE@** = noun/number form that follows a definite article (an)
    #[strum(serialize = "DefArt")]
    DefArt,
    /// non: (no comment)
    #[strum(serialize = "Defkyn")]
    Defkyn,
    /// gle: **@CODE@** = degree particle with Adj/Abstract Noun (so loud, so sharp etc..
    /// kpv: **@CODE@** Degree depricate AdA
    /// lut: (no comment)
    /// slh: (no comment)
    /// mns: **@CODE@** degree мощща
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** Degree depricate AdA
    /// mdf: (no comment)
    /// vro: **@CODE@** adjective or adverb modifier. This is degree, depricate + AdA
    #[strum(serialize = "Deg")]
    Deg,
    /// mdf: (no comment)
    #[strum(serialize = "Degree")]
    Degree,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// crj: (no comment)
    /// hun: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Del")]
    Del,
    /// myv: (no comment)
    #[strum(serialize = "Delib")]
    Delib,
    /// fit: **@CODE@** = Demonstrative
    /// sje: (no comment)
    /// gle: **@CODE@** = Demonstrative determiner (also combined with copula, e.g. Seo
    /// kpv: **@CODE@**:  demonstrative
    /// chp: (no comment)
    /// vot: (no comment)
    /// bla: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Demonstrative
    /// liv: **@CODE@** = demonstrative
    /// crk: (no comment)
    /// mhr: **@CODE@** = Demonstrative pronoun
    /// rue: (no comment)
    /// sma: **@CODE@** = Demonstrative
    /// hdn: (no comment)
    /// smj: **@CODE@** = Demonstrative pronoun
    /// udm: (no comment)
    /// apu: **@CODE@** Demonstrative Pronoun
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Demonstrative Pronoun
    /// bxr: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@**:  demonstrative
    /// vep: **@CODE@**  = demonstrative
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// olo: **@CODE@** demonstrative
    /// koi: **@CODE@**:  demonstrative
    /// deu: (no comment)
    /// sme: **@CODE@** - Demonstrative Pronoun
    /// smn: **@CODE@** - Demonstrative Pronoun
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  demonstrative
    /// khk: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Dem")]
    Dem,
    /// tkl: (no comment)
    #[strum(serialize = "Dem-Dist")]
    Dem_MINUS_Dist,
    /// mpj: **@CODE@**  only with palunya
    #[strum(serialize = "Dem/janulu")]
    Dem_SLASH_janulu,
    /// mpj: **@CODE@** in book +Foc +Abl
    #[strum(serialize = "Dem/na")]
    Dem_SLASH_na,
    /// mpj: **@CODE@** only with yangka. In book +Rel +Pa
    #[strum(serialize = "Dem/ngula")]
    Dem_SLASH_ngula,
    /// myv: (no comment)
    #[strum(serialize = "DemPrc")]
    DemPrc,
    /// gle: **@CODE@** = Dependant forms of verbs
    /// mhr: **@CODE@** = ( pair verbs that do not occur independently get this marker.) /was +Depend, but +Dep used in fst.
    /// mrj: **@CODE@**:  dependent word requiring the presence of noun, e.g. ӹшке
    /// myv: (no comment)
    /// olo: **@CODE@** dependent word requiring the presence of another, e.g. **мень**
    /// mdf: **@CODE@**:  dependent word requiring the presence of another, e.g. **мень**
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
    /// krl: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@**:  Tag to precede any non-positional derivation
    /// lut: (no comment)
    /// nob: **@CODE@** = mark derivation
    /// smj: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// rus: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// sms: **@CODE@** This ordinal derivation
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** is the tag to put before the other der tags.
    /// mdf: **@CODE@**  In front of every derivation to make it
    /// vro: **@CODE@**
    /// tqn: (no comment)
    #[strum(serialize = "Der")]
    Der,
    /// fit: **@CODE@** =
    /// kpv: (no comment)
    /// fkv: **@CODE@** = deriving adjectives from verbs
    /// liv: **@CODE@** for example present participle to adjective
    /// sma: (no comment)
    /// fao: (no comment)
    /// sms: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** Adjective derivation
    /// olo: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// sme: **@CODE@** = Adjective derivated from Noun or Verb
    #[strum(serialize = "Der/A")]
    Der_SLASH_A,
    /// nob: **@CODE@** = Adjectives are also adverbs
    /// smj: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |      |    | adverb pyeremusávt pyeremusâht
    #[strum(serialize = "Der/AAdv")]
    Der_SLASH_AAdv,
    /// olo: **@CODE@** A>>N derivation
    #[strum(serialize = "Der/AN")]
    Der_SLASH_AN,
    /// kpv: **@CODE@** тӧм should take +A, see also +VAbess
    /// myv: (no comment)
    #[strum(serialize = "Der/Abe")]
    Der_SLASH_Abe,
    /// myv: (no comment)
    #[strum(serialize = "Der/AbeAttr")]
    Der_SLASH_AbeAttr,
    /// izh: **@CODE@** = for derivation of adjectives without specification
    #[strum(serialize = "Der/Adj")]
    Der_SLASH_Adj,
    /// kpv: (no comment)
    /// fao: (no comment)
    /// rmg: **@CODE@**
    /// sme: **@CODE@** = Adverb derivated from Adjective
    /// rmf: **@CODE@** transitivity tags
    #[strum(serialize = "Der/Adv")]
    Der_SLASH_Adv,
    /// koi: **@CODE@** Adverbial comparative modifier in моз Der/моз +MOZ  diminishing, kind of, sort of
    #[strum(serialize = "Der/AdvCompMod")]
    Der_SLASH_AdvCompMod,
    /// koi: (no comment)
    #[strum(serialize = "Der/Al")]
    Der_SLASH_Al,
    /// kpv: **@CODE@**  Process Participle +AN
    /// koi: (no comment)
    #[strum(serialize = "Der/An")]
    Der_SLASH_An,
    /// kpv: **@CODE@** Process Participle +ANA, Gerund or participle according to context (with...)
    /// koi: (no comment)
    #[strum(serialize = "Der/Ana")]
    Der_SLASH_Ana,
    /// kpv: **@CODE@**  adverb derived from participle (+ANA) +ANAA
    #[strum(serialize = "Der/Anaa")]
    Der_SLASH_Anaa,
    /// mns: **@CODE@** V» ап Deverbal noun, not regular semantic correlation
    #[strum(serialize = "Der/Ap")]
    Der_SLASH_Ap,
    /// koi: (no comment)
    #[strum(serialize = "Der/Asj")]
    Der_SLASH_Asj,
    /// koi: (no comment)
    #[strum(serialize = "Der/AssocColl")]
    Der_SLASH_AssocColl,
    /// hun: (no comment)
    #[strum(serialize = "Der/Assoc_N")]
    Der_SLASH_Assoc_N,
    /// mdf: (no comment)
    #[strum(serialize = "Der/Bachk")]
    Der_SLASH_Bachk,
    /// bla: (no comment)
    #[strum(serialize = "Der/Bnf")]
    Der_SLASH_Bnf,
    /// sma: **@CODE@** |            |            |      | -hts, Caritive, was Der/heapmi in sme
    /// smj: (no comment)
    /// sms: **@CODE@**  N»A -tem, -teʹmes
    /// sme: (no comment)
    /// smn: **@CODE@** |            |            |      | NA | * +Der1+Der2 - only combine with Der3 caritive: peljittem
    #[strum(serialize = "Der/Car")]
    Der_SLASH_Car,
    /// pur: (no comment)
    /// bla: (no comment)
    /// mhr: **@CODE@** = Derivation V > V: Causative
    /// smj: (no comment)
    /// sms: **@CODE@** causative V»V, was Der/ted and Der/âʹtted
    /// eus: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |            |      | VV | - 3-syll causatives
    #[strum(serialize = "Der/Caus")]
    Der_SLASH_Caus,
    /// bla: (no comment)
    #[strum(serialize = "Der/Caus2")]
    Der_SLASH_Caus2,
    /// myv: (no comment)
    #[strum(serialize = "Der/Cmpr")]
    Der_SLASH_Cmpr,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Der/Com")]
    Der_SLASH_Com,
    /// sma: **@CODE@** |            |            | AA   | adjektiv
    /// smj: (no comment)
    /// mns: **@CODE@** Comparitive -нув-
    /// sms: **@CODE@** comparative as in other Sami languages, depricate +Comp
    /// koi: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@**
    #[strum(serialize = "Der/Comp")]
    Der_SLASH_Comp,
    /// koi: **@CODE@** comparative modifier in кодь Der/кодь ! diminishing, kind of, sort of
    /// koi: (no comment)
    #[strum(serialize = "Der/CompMod")]
    Der_SLASH_CompMod,
    /// myv: (no comment)
    #[strum(serialize = "Der/Cond")]
    Der_SLASH_Cond,
    /// yrk: (no comment)
    /// myv: (no comment)
    #[strum(serialize = "Der/Cop")]
    Der_SLASH_Cop,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dat")]
    Der_SLASH_Dat,
    /// myv: (no comment)
    #[strum(serialize = "Der/Dem")]
    Der_SLASH_Dem,
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Der/Dim")]
    Der_SLASH_Dim,
    /// sje: (no comment)
    /// sma: **@CODE@** |            |            | NN   | Diminutive
    /// lut: (no comment)
    /// smj: (no comment)
    /// smj: (no comment)
    /// slh: (no comment)
    /// mns: **@CODE@** Diminutives in -кве (older)/-ке (~ Sosva)/-те (~ Ljapin)
    /// sms: **@CODE@** diminutive derivation N»N was Der/Dim,
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |      | NN | (was: Der/aš & Der/š)
    /// mdf: (no comment)
    #[strum(serialize = "Der/Dimin")]
    Der_SLASH_Dimin,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_12123")]
    Der_SLASH_Dimin_12123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1213")]
    Der_SLASH_Dimin_1213,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_121e3")]
    Der_SLASH_Dimin_121e3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_12q123")]
    Der_SLASH_Dimin_12q123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_12q13")]
    Der_SLASH_Dimin_12q13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_12q1e3")]
    Der_SLASH_Dimin_12q1e3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1i123")]
    Der_SLASH_Dimin_1i123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1i13")]
    Der_SLASH_Dimin_1i13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1i1e3")]
    Der_SLASH_Dimin_1i1e3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1i1i13")]
    Der_SLASH_Dimin_1i1i13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1iq123")]
    Der_SLASH_Dimin_1iq123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1iq13")]
    Der_SLASH_Dimin_1iq13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Dimin_1iq1e3")]
    Der_SLASH_Dimin_1iq1e3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Distr")]
    Der_SLASH_Distr,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Distr_123123")]
    Der_SLASH_Distr_123123,
    /// mpj: **@CODE@** = derivational tags
    #[strum(serialize = "Der/Foc")]
    Der_SLASH_Foc,
    /// myv: (no comment)
    #[strum(serialize = "Der/Gak")]
    Der_SLASH_Gak,
    /// myv: (no comment)
    #[strum(serialize = "Der/GenAttr")]
    Der_SLASH_GenAttr,
    /// mpj: (no comment)
    #[strum(serialize = "Der/Hav")]
    Der_SLASH_Hav,
    /// mdf: (no comment)
    #[strum(serialize = "Der/I")]
    Der_SLASH_I,
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
    /// koi: (no comment)
    #[strum(serialize = "Der/Ik")]
    Der_SLASH_Ik,
    /// kpv: **@CODE@** Ин
    #[strum(serialize = "Der/In")]
    Der_SLASH_In,
    /// sma: **@CODE@** | VV   | Inchoative
    /// smj: (no comment)
    /// sms: **@CODE@** incoative V»V -škueʹtted
    /// sme: (no comment)
    /// smn: **@CODE@** | VV |
    #[strum(serialize = "Der/InchL")]
    Der_SLASH_InchL,
    /// kpv: **@CODE@** Иник
    #[strum(serialize = "Der/Inik")]
    Der_SLASH_Inik,
    /// mdf: **@CODE@**  весть, кафксть, колмоксть...
    #[strum(serialize = "Der/Iter")]
    Der_SLASH_Iter,
    /// vro: **@CODE@**	NomAg
    #[strum(serialize = "Der/JA")]
    Der_SLASH_JA,
    /// kpv: **@CODE@**
    #[strum(serialize = "Der/La")]
    Der_SLASH_La,
    /// kpv: (no comment)
    /// koi: **@CODE@** locative modifier са - IneMod
    #[strum(serialize = "Der/LocMod")]
    Der_SLASH_LocMod,
    /// kpv: **@CODE@**
    /// mhr: **@CODE@** = Modifier without noun (better: +A+Sg+Nom etc.)
    /// yrk: **@CODE@** modifier without noun head
    /// myv: (no comment)
    /// olo: **@CODE@** Modifier without Noun head
    /// koi: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/MWN")]
    Der_SLASH_MWN,
    /// vep: **@CODE@**  = V»A
    #[strum(serialize = "Der/Matoi")]
    Der_SLASH_Matoi,
    /// vep: **@CODE@**  = V»N
    #[strum(serialize = "Der/Mine")]
    Der_SLASH_Mine,
    /// kpv: (no comment)
    /// cwd: (no comment)
    /// pma: (no comment)
    /// crk: (no comment)
    /// sms: (no comment)
    /// mrj: **@CODE@** =        !! Noun derivation
    /// som: (no comment)
    #[strum(serialize = "Der/N")]
    Der_SLASH_N,
    /// sms: (no comment)
    /// myv: (no comment)
    #[strum(serialize = "Der/N2A")]
    Der_SLASH_N2A,
    /// apu: (no comment)
    #[strum(serialize = "Der/NIku")]
    Der_SLASH_NIku,
    /// mdf: (no comment)
    #[strum(serialize = "Der/NJ")]
    Der_SLASH_NJ,
    /// myv: (no comment)
    #[strum(serialize = "Der/NJE")]
    Der_SLASH_NJE,
    /// bla: (no comment)
    /// mhr: **@CODE@** =  Derivation V > N: Nominalization
    #[strum(serialize = "Der/Nom")]
    Der_SLASH_Nom,
    /// bla: (no comment)
    #[strum(serialize = "Der/Nom2")]
    Der_SLASH_Nom2,
    /// bla: (no comment)
    #[strum(serialize = "Der/Nom3")]
    Der_SLASH_Nom3,
    /// bla: (no comment)
    #[strum(serialize = "Der/Nom4")]
    Der_SLASH_Nom4,
    /// bla: (no comment)
    #[strum(serialize = "Der/Nom5")]
    Der_SLASH_Nom5,
    /// kpv: **@CODE@**  +Event
    /// liv: **@CODE@**
    /// krl: (no comment)
    /// sma: **@CODE@** | VN   | Nomen Actionis
    /// nob: **@CODE@** = verb +ing
    /// smj: (no comment)
    /// smj: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@** м
    /// sms: **@CODE@** action V»N +Der/m
    /// myv: (no comment)
    /// wac: **@CODE@** action V»N
    /// sme: (no comment)
    /// smn: **@CODE@** |            |      | VN | Der/NomAct har to realisasjonar, med ulike restriksjonar,
    /// mdf: (no comment)
    /// tqn: **@CODE@** action V»N
    #[strum(serialize = "Der/NomAct")]
    Der_SLASH_NomAct,
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// liv: **@CODE@**
    /// krl: (no comment)
    /// sma: **@CODE@** | VN   | Nomen Agentis
    /// smj: (no comment)
    /// skf: (no comment)
    /// sms: **@CODE@** agent V»N
    /// vep: **@CODE@**  = tehta : tegii
    /// myv: (no comment)
    /// wac: **@CODE@** agent V»N
    /// sme: (no comment)
    /// smn: **@CODE@** |            |      |    |
    /// mdf: (no comment)
    /// tqn: **@CODE@** agent V»N
    #[strum(serialize = "Der/NomAg")]
    Der_SLASH_NomAg,
    /// skf: (no comment)
    #[strum(serialize = "Der/NomIns")]
    Der_SLASH_NomIns,
    /// smj: (no comment)
    #[strum(serialize = "Der/NomInstr")]
    Der_SLASH_NomInstr,
    /// mhr: **@CODE@** = Derivation V > N: Negative nominalization
    #[strum(serialize = "Der/NomNeg")]
    Der_SLASH_NomNeg,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/OkshnOms")]
    Der_SLASH_OkshnOms,
    /// myv: (no comment)
    #[strum(serialize = "Der/OmA")]
    Der_SLASH_OmA,
    /// myv: (no comment)
    #[strum(serialize = "Der/OmO")]
    Der_SLASH_OmO,
    /// myv: (no comment)
    #[strum(serialize = "Der/Oma")]
    Der_SLASH_Oma,
    /// myv: (no comment)
    #[strum(serialize = "Der/OmdO")]
    Der_SLASH_OmdO,
    /// myv: (no comment)
    #[strum(serialize = "Der/Omga")]
    Der_SLASH_Omga,
    /// myv: (no comment)
    #[strum(serialize = "Der/Omka")]
    Der_SLASH_Omka,
    /// koi: (no comment)
    #[strum(serialize = "Der/Omon")]
    Der_SLASH_Omon,
    /// myv: (no comment)
    #[strum(serialize = "Der/Oms")]
    Der_SLASH_Oms,
    /// myv: (no comment)
    #[strum(serialize = "Der/OmsO")]
    Der_SLASH_OmsO,
    /// myv: (no comment)
    #[strum(serialize = "Der/OmstO")]
    Der_SLASH_OmstO,
    /// myv: (no comment)
    #[strum(serialize = "Der/Oncje")]
    Der_SLASH_Oncje,
    /// mdf: (no comment)
    #[strum(serialize = "Der/Onj")]
    Der_SLASH_Onj,
    /// mns: **@CODE@** is this the best analysis?
    /// sms: **@CODE@** Num»A ordinal
    /// koi: (no comment)
    /// mdf: **@CODE@**
    #[strum(serialize = "Der/Ord")]
    Der_SLASH_Ord,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/OvOms")]
    Der_SLASH_OvOms,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/OvkshnOms")]
    Der_SLASH_OvkshnOms,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ovt")]
    Der_SLASH_Ovt,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/OvtOms")]
    Der_SLASH_OvtOms,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ovtnjems")]
    Der_SLASH_Ovtnjems,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/Ozevems")]
    Der_SLASH_Ozevems,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ozevkshnems")]
    Der_SLASH_Ozevkshnems,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ozevtems")]
    Der_SLASH_Ozevtems,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ozevtnjems")]
    Der_SLASH_Ozevtnjems,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/Ozj")]
    Der_SLASH_Ozj,
    /// sms: **@CODE@** rupp » ruʹppi ATTR ruʹppes DEPRICATE
    #[strum(serialize = "Der/PALi")]
    Der_SLASH_PALi,
    /// smn: **@CODE@** |            |            |      | VV | -  passive
    #[strum(serialize = "Der/Pass")]
    Der_SLASH_Pass,
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
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/PatrFem")]
    Der_SLASH_PatrFem,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/PatrMal")]
    Der_SLASH_PatrMal,
    /// mhr: **@CODE@** =   Derivation N > A: Possessive adjective, orig. genitive form without a head
    /// mdf: **@CODE@**  possessive noun ава » аванне
    #[strum(serialize = "Der/Poss")]
    Der_SLASH_Poss,
    /// hun: (no comment)
    #[strum(serialize = "Der/Poss_N")]
    Der_SLASH_Poss_N,
    /// yrk: **@CODE@** this is used with predication of nominals and deverbal modalities
    /// myv: (no comment)
    #[strum(serialize = "Der/Pr")]
    Der_SLASH_Pr,
    /// mhr: **@CODE@** =  Derivation N > A: Privative adjective
    #[strum(serialize = "Der/Priv")]
    Der_SLASH_Priv,
    /// kpv: **@CODE@** = тӧм
    /// koi: **@CODE@** privative or abessive modifier -тӧм AbeMod
    #[strum(serialize = "Der/PrivMod")]
    Der_SLASH_PrivMod,
    /// koi: (no comment)
    #[strum(serialize = "Der/Pron")]
    Der_SLASH_Pron,
    /// kpv: **@CODE@** = +Der/APrior  Denominal prioritive adjective Der/a
    /// koi: **@CODE@** habeo modifier HabObjMod Der/а
    #[strum(serialize = "Der/ProprietiveMod")]
    Der_SLASH_ProprietiveMod,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Der/PrsAct")]
    Der_SLASH_PrsAct,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Der/PrsPss")]
    Der_SLASH_PrsPss,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Der/PstAct")]
    Der_SLASH_PstAct,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Der/PstPss")]
    Der_SLASH_PstPss,
    /// mhr: **@CODE@** =   Derivation N > A:
    #[strum(serialize = "Der/Pur")]
    Der_SLASH_Pur,
    /// kal: (no comment)
    #[strum(serialize = "Der/Quote")]
    Der_SLASH_Quote,
    /// olo: **@CODE@** used in verbs for deriving reflexive conjugation
    #[strum(serialize = "Der/Rc")]
    Der_SLASH_Rc,
    /// bla: (no comment)
    #[strum(serialize = "Der/Rcpr")]
    Der_SLASH_Rcpr,
    /// sma: **@CODE@** |            |            | NN   | Forholdsformer
    #[strum(serialize = "Der/Rec")]
    Der_SLASH_Rec,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_12123")]
    Der_SLASH_Red1_12123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1213")]
    Der_SLASH_Red1_1213,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_121ə3")]
    Der_SLASH_Red1_121ə3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_12ʔ123")]
    Der_SLASH_Red1_12ʔ123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_12ʔ13")]
    Der_SLASH_Red1_12ʔ13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_12ʔ1ə3")]
    Der_SLASH_Red1_12ʔ1ə3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1i123")]
    Der_SLASH_Red1_1i123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1i13")]
    Der_SLASH_Red1_1i13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1i13_Distr_1212")]
    Der_SLASH_Red1_1i13_Distr_1212,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1i1ə3")]
    Der_SLASH_Red1_1i1ə3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1iʔ123")]
    Der_SLASH_Red1_1iʔ123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1iʔ13")]
    Der_SLASH_Red1_1iʔ13,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red1_1iʔ1ə3")]
    Der_SLASH_Red1_1iʔ1ə3,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red2_3L")]
    Der_SLASH_Red2_3L,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red3_12323")]
    Der_SLASH_Red3_12323,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red5_12323")]
    Der_SLASH_Red5_12323,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red6_122123")]
    Der_SLASH_Red6_122123,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Der/Red6_1aa1ə3")]
    Der_SLASH_Red6_1aa1ə3,
    /// mhr: **@CODE@** = Derivation V > V: Reflexive
    #[strum(serialize = "Der/Refl")]
    Der_SLASH_Refl,
    /// mhr: **@CODE@** =  Derivation N > A: Relational adjective
    #[strum(serialize = "Der/Rel")]
    Der_SLASH_Rel,
    /// bla: (no comment)
    #[strum(serialize = "Der/Rflx")]
    Der_SLASH_Rflx,
    /// mdf: (no comment)
    #[strum(serialize = "Der/Shka")]
    Der_SLASH_Shka,
    /// mdf: (no comment)
    #[strum(serialize = "Der/Shovor")]
    Der_SLASH_Shovor,
    /// mpj: **@CODE@**
    #[strum(serialize = "Der/SpatAbl")]
    Der_SLASH_SpatAbl,
    /// mpj: **@CODE@**
    #[strum(serialize = "Der/SpatAll")]
    Der_SLASH_SpatAll,
    /// sje: (no comment)
    #[strum(serialize = "Der/State")]
    Der_SLASH_State,
    /// sma: **@CODE@** |            |            | AA   | adjektiv
    /// smj: (no comment)
    /// sms: **@CODE@** superlative previously Der/mos Der/umus Der/ummus, depricate +Superl
    /// sme: (no comment)
    /// smn: **@CODE@**
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
    /// mpj: **@CODE@**
    #[strum(serialize = "Der/TempLoc")]
    Der_SLASH_TempLoc,
    /// kpv: **@CODE@** TempMod Der/sja но и Ф. В. Плесовскийлысь квайтымынӧд вояссяяссӧ * позьӧ аддзыны сӧмын библиотекаясысь.  Declaring spatial adverb derivations; see also spatial postpositions
    /// koi: **@CODE@** temporal modifier ся -
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
    /// kal: **@CODE@** = Ukendt morfem !Ukendte morfemer i ellers analyserede stammer for at få rigtig lemmaanalyse, som orsoq+NNAP+nv+UNK+vn:orsunnattaaq
    #[strum(serialize = "Der/Unk")]
    Der_SLASH_Unk,
    /// olo: **@CODE@** A>>N derivation (+Der/AN better?)
    /// vro: **@CODE@** A→N
    #[strum(serialize = "Der/Us")]
    Der_SLASH_Us,
    /// vep: **@CODE@**  = sur»uz' A»N
    #[strum(serialize = "Der/Uz1")]
    Der_SLASH_Uz1,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// vep: **@CODE@**  = V»V
    /// som: (no comment)
    #[strum(serialize = "Der/V")]
    Der_SLASH_V,
    /// sms: (no comment)
    /// myv: (no comment)
    #[strum(serialize = "Der/V2A")]
    Der_SLASH_V2A,
    /// sms: (no comment)
    #[strum(serialize = "Der/V2Adv")]
    Der_SLASH_V2Adv,
    /// bla: (no comment)
    #[strum(serialize = "Der/VAI")]
    Der_SLASH_VAI,
    /// sje: (no comment)
    #[strum(serialize = "Der/VAdv")]
    Der_SLASH_VAdv,
    /// bla: (no comment)
    #[strum(serialize = "Der/VII")]
    Der_SLASH_VII,
    /// liv: **@CODE@**
    #[strum(serialize = "Der/VN")]
    Der_SLASH_VN,
    /// mns: **@CODE@** V»V - ялуӈкве 'сходить, съездить' 'to go, to travel' -> яласаӈкве 'ходить, ездить' 'to go, to travel (regularly)'
    #[strum(serialize = "Der/Vasa")]
    Der_SLASH_Vasa,
    /// mns: **@CODE@** V»V - паргуӈкве 'сыпаться' 'fall' -> паргалтаӈкве 'посеять' 'sprinkle'
    #[strum(serialize = "Der/Vlt")]
    Der_SLASH_Vlt,
    /// mns: **@CODE@** V»V - о̄луӈкве 'жить' 'to be, to live, to reside' -> о̄лантаӈкве 'существовать' 'to live, to reside'
    #[strum(serialize = "Der/Vnt")]
    Der_SLASH_Vnt,
    /// kal: **@CODE@** = Vocative
    #[strum(serialize = "Der/Vok")]
    Der_SLASH_Vok,
    /// myv: (no comment)
    #[strum(serialize = "Der/VtOmO")]
    Der_SLASH_VtOmO,
    /// mns: **@CODE@** V»V - ликмуӈкве 'попасть в ловушку' -> ликмыгла̄луӈкве попадать в ловушку
    #[strum(serialize = "Der/Vyglal")]
    Der_SLASH_Vyglal,
    /// mns: **@CODE@** V»V - алуӈкве 'убить, добить' 'to kill, to hunt down' -> алыщлаӈкве 'ловить, охотиться' 'to kill, to fish, to hunt'
    #[strum(serialize = "Der/Vyschl")]
    Der_SLASH_Vyschl,
    /// mdf: **@CODE@**  Added to male names, surnames, patronymics
    #[strum(serialize = "Der/Wife")]
    Der_SLASH_Wife,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Der/X")]
    Der_SLASH_X,
    /// myv: (no comment)
    #[strum(serialize = "Der/Y")]
    Der_SLASH_Y,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ycja")]
    Der_SLASH_Ycja,
    /// myv: (no comment)
    #[strum(serialize = "Der/Yj")]
    Der_SLASH_Yj,
    /// myv: (no comment)
    #[strum(serialize = "Der/Yja")]
    Der_SLASH_Yja,
    /// myv: (no comment)
    #[strum(serialize = "Der/Yks")]
    Der_SLASH_Yks,
    /// mns: **@CODE@** N» ыӈ Proprietive as in kpv -а
    #[strum(serialize = "Der/Yng")]
    Der_SLASH_Yng,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ynje")]
    Der_SLASH_Ynje,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ynjka")]
    Der_SLASH_Ynjka,
    /// myv: (no comment)
    #[strum(serialize = "Der/Ynjkinje")]
    Der_SLASH_Ynjkinje,
    /// mns: **@CODE@** A» ыщ
    #[strum(serialize = "Der/Yshch")]
    Der_SLASH_Yshch,
    /// kpv: **@CODE@** а
    #[strum(serialize = "Der/a")]
    Der_SLASH_a,
    /// sms: **@CODE@** ordinals to nouns
    #[strum(serialize = "Der/ad")]
    Der_SLASH_ad,
    /// smj: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |      | VV |
    #[strum(serialize = "Der/adda")]
    Der_SLASH_adda,
    /// sma: **@CODE@** |            | VV   | Frequentative, Kontinuativ
    #[strum(serialize = "Der/adte")]
    Der_SLASH_adte,
    /// smn: **@CODE@** neeljičievâg neeljijienâg kuulmâloonjâg neeljičievâg neeljijienâg
    #[strum(serialize = "Der/ag")]
    Der_SLASH_ag,
    /// smn: **@CODE@** 85-ahasâš škovlâahasâš
    #[strum(serialize = "Der/ahasas")]
    Der_SLASH_ahasas,
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
    /// myv: (no comment)
    #[strum(serialize = "Der/aj")]
    Der_SLASH_aj,
    /// smj: (no comment)
    #[strum(serialize = "Der/akti")]
    Der_SLASH_akti,
    /// kpv: **@CODE@** ал
    #[strum(serialize = "Der/al")]
    Der_SLASH_al,
    /// sma: **@CODE@** |            | VV   | Frequentative
    /// smj: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |      | VV |
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
    /// myv: (no comment)
    #[strum(serialize = "Der/buti")]
    Der_SLASH_buti,
    /// kpv: **@CODE@** бӧрса
    #[strum(serialize = "Der/bwrsa")]
    Der_SLASH_bwrsa,
    /// sms: **@CODE@** frequentative V»V for ʹCCed verbs -čed
    #[strum(serialize = "Der/ched")]
    Der_SLASH_ched,
    /// myv: (no comment)
    #[strum(serialize = "Der/chi")]
    Der_SLASH_chi,
    /// kpv: **@CODE@**  чӧж +CHOZH
    #[strum(serialize = "Der/chwzh")]
    Der_SLASH_chwzh,
    /// kpv: **@CODE@** чы This appears to be a variant of +Der/sjy; it follows plosives
    #[strum(serialize = "Der/chy")]
    Der_SLASH_chy,
    /// myv: (no comment)
    #[strum(serialize = "Der/cje")]
    Der_SLASH_cje,
    /// sma: **@CODE@** |            |            | VV   | Continuative, Konative, Frequentative, Refleksive, Momentan
    /// smj: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |            |      | VV |
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
    /// mdf: (no comment)
    #[strum(serialize = "Der/f")]
    Der_SLASH_f,
    /// smj: (no comment)
    #[strum(serialize = "Der/ferjak")]
    Der_SLASH_ferjak,
    /// mdf: (no comment)
    #[strum(serialize = "Der/fks")]
    Der_SLASH_fks,
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
    /// fao: (no comment)
    #[strum(serialize = "Der/heit")]
    Der_SLASH_heit,
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
    /// lav: (no comment)
    #[strum(serialize = "Der/iens")]
    Der_SLASH_iens,
    /// sma: **@CODE@** |            |            | VA   | (Handlernomen- tilbøyelig til å utføre den handlingen som grunnordet angir)
    #[strum(serialize = "Der/ihks")]
    Der_SLASH_ihks,
    /// sma: **@CODE@** |            |            | NA   | Nomen agentis
    #[strum(serialize = "Der/ijes")]
    Der_SLASH_ijes,
    /// fin: (no comment)
    #[strum(serialize = "Der/inen")]
    Der_SLASH_inen,
    /// smn: **@CODE@**
    #[strum(serialize = "Der/ivvaas")]
    Der_SLASH_ivvaas,
    /// izh: **@CODE@** = actor name !!2012-10-30
    #[strum(serialize = "Der/jA")]
    Der_SLASH_jA,
    /// fin: (no comment)
    #[strum(serialize = "Der/ja")]
    Der_SLASH_ja,
    /// sms: **@CODE@** inchoative V»V also middle
    #[strum(serialize = "Der/jed")]
    Der_SLASH_jed,
    /// smj: (no comment)
    #[strum(serialize = "Der/k")]
    Der_SLASH_k,
    /// myv: (no comment)
    #[strum(serialize = "Der/ka")]
    Der_SLASH_ka,
    /// myv: (no comment)
    #[strum(serialize = "Der/kaj")]
    Der_SLASH_kaj,
    /// myv: (no comment)
    #[strum(serialize = "Der/ke")]
    Der_SLASH_ke,
    /// sme: (no comment)
    #[strum(serialize = "Der/keahtta")]
    Der_SLASH_keahtta,
    /// kpv: **@CODE@**  +KEZHLO
    #[strum(serialize = "Der/kezhlw")]
    Der_SLASH_kezhlw,
    /// myv: (no comment)
    #[strum(serialize = "Der/kinje")]
    Der_SLASH_kinje,
    /// kpv: **@CODE@** кодь diminishing, kind of, sort of
    #[strum(serialize = "Der/kodj")]
    Der_SLASH_kodj,
    /// myv: (no comment)
    #[strum(serialize = "Der/koj")]
    Der_SLASH_koj,
    /// kpv: **@CODE@**  +KOSTA
    #[strum(serialize = "Der/kosta")]
    Der_SLASH_kosta,
    /// kpv: **@CODE@**  +KOSTI
    #[strum(serialize = "Der/kosti")]
    Der_SLASH_kosti,
    /// kpv: **@CODE@** костса
    #[strum(serialize = "Der/kostsa")]
    Der_SLASH_kostsa,
    /// myv: (no comment)
    #[strum(serialize = "Der/ks")]
    Der_SLASH_ks,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Der/kshnO")]
    Der_SLASH_kshnO,
    /// kpv: **@CODE@** л
    /// sma: **@CODE@** |            |            | VV   | Subitive
    /// smj: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@** |            |            |      | VV |
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
    /// smn: **@CODE@** |            |            |      | AA | * +Der1+Der2 - only combine with Der3
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
    /// smn: **@CODE@** |            |            |      | NA |
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
    /// smn: **@CODE@** | VN |
    #[strum(serialize = "Der/mas")]
    Der_SLASH_mas,
    /// olo: **@CODE@** comparative
    #[strum(serialize = "Der/mbi")]
    Der_SLASH_mbi,
    /// sme: (no comment)
    #[strum(serialize = "Der/meahttun")]
    Der_SLASH_meahttun,
    /// kpv: **@CODE@**  мед- Superlative
    #[strum(serialize = "Der/med")]
    Der_SLASH_med,
    /// izh: **@CODE@** = Deverbal nouns
    #[strum(serialize = "Der/miin")]
    Der_SLASH_miin,
    /// vot: (no comment)
    /// izh: **@CODE@** = Deverbal nouns
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
    /// myv: (no comment)
    #[strum(serialize = "Der/ms")]
    Der_SLASH_ms,
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
    /// myv: (no comment)
    #[strum(serialize = "Der/nJE")]
    Der_SLASH_nJE,
    /// sms: (no comment)
    #[strum(serialize = "Der/nalla")]
    Der_SLASH_nalla,
    /// sms: **@CODE@** N»N resident of place
    #[strum(serialize = "Der/neqkh")]
    Der_SLASH_neqkh,
    /// myv: (no comment)
    #[strum(serialize = "Der/nje")]
    Der_SLASH_nje,
    /// myv: (no comment)
    #[strum(serialize = "Der/njems")]
    Der_SLASH_njems,
    /// kal: **@CODE@** = noun to noun
    #[strum(serialize = "Der/nn")]
    Der_SLASH_nn,
    /// fin: (no comment)
    #[strum(serialize = "Der/nti")]
    Der_SLASH_nti,
    /// kal: **@CODE@** = noun to verb
    #[strum(serialize = "Der/nv")]
    Der_SLASH_nv,
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
    /// myv: (no comment)
    #[strum(serialize = "Der/pelj")]
    Der_SLASH_pelj,
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
    /// lut: (no comment)
    /// slh: (no comment)
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
    /// smn: **@CODE@** |            |      | NA |
    #[strum(serialize = "Der/sasj")]
    Der_SLASH_sasj,
    /// sms: (no comment)
    #[strum(serialize = "Der/sazh")]
    Der_SLASH_sazh,
    /// smj: (no comment)
    #[strum(serialize = "Der/segak")]
    Der_SLASH_segak,
    /// myv: (no comment)
    #[strum(serialize = "Der/shka")]
    Der_SLASH_shka,
    /// som: (no comment)
    #[strum(serialize = "Der/sho")]
    Der_SLASH_sho,
    /// kpv: **@CODE@** сь This only occurs following a vowel in an yny-stem 2017-09-19+Der/sj
    #[strum(serialize = "Der/sj")]
    Der_SLASH_sj,
    /// kpv: **@CODE@** ся
    #[strum(serialize = "Der/sja")]
    Der_SLASH_sja,
    /// myv: (no comment)
    #[strum(serialize = "Der/sje")]
    Der_SLASH_sje,
    /// kpv: **@CODE@** -сюрӧ -кӧ !Declaring Indefinite Pronoun derivations
    #[strum(serialize = "Der/sjurw")]
    Der_SLASH_sjurw,
    /// kpv: **@CODE@** сьы 2017-09-19+Der/ch ! This appears to be a variant of +Der/sjy; it follows plosives
    #[strum(serialize = "Der/sjy")]
    Der_SLASH_sjy,
    /// kpv: **@CODE@**  = +SOR
    #[strum(serialize = "Der/sor")]
    Der_SLASH_sor,
    /// izh: **@CODE@** = for derivation of manner adverbs
    /// sma: **@CODE@** |            |            | VV   | Diminutive, Subitive
    /// smj: (no comment)
    /// sms: **@CODE@**  V»V was diminutive subitive Der/sted, Der/âʹstted
    /// sme: (no comment)
    /// smn: **@CODE@** |            |            |      | VV | čälistiđ
    #[strum(serialize = "Der/st")]
    Der_SLASH_st,
    /// myv: (no comment)
    #[strum(serialize = "Der/stO")]
    Der_SLASH_stO,
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
    /// izh: **@CODE@** = Deverbal _arvaamaton_ and Denominal adjectives
    #[strum(serialize = "Der/tOin")]
    Der_SLASH_tOin,
    /// myv: (no comment)
    #[strum(serialize = "Der/ta")]
    Der_SLASH_ta,
    /// smn: **@CODE@** |            |      |    | adverb pyeremustáá !This is not the best tag?
    #[strum(serialize = "Der/taa")]
    Der_SLASH_taa,
    /// myv: (no comment)
    #[strum(serialize = "Der/tago")]
    Der_SLASH_tago,
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
    /// myv: (no comment)
    #[strum(serialize = "Der/tjks")]
    Der_SLASH_tjks,
    /// izh: **@CODE@** = ykstoist (11), kakstoist (12)
    #[strum(serialize = "Der/toist")]
    Der_SLASH_toist,
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
    /// smn: **@CODE@** |            |            |      | VV | - Causative čälittiđ
    #[strum(serialize = "Der/tt")]
    Der_SLASH_tt,
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
    /// deu: (no comment)
    #[strum(serialize = "Der/ung")]
    Der_SLASH_ung,
    /// sme: (no comment)
    /// smn: **@CODE@** | VN |
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
    /// kal: **@CODE@** = verb to noun
    #[strum(serialize = "Der/vn")]
    Der_SLASH_vn,
    /// kpv: **@CODE@** водзса
    #[strum(serialize = "Der/vodzsa")]
    Der_SLASH_vodzsa,
    /// fin: (no comment)
    #[strum(serialize = "Der/vs")]
    Der_SLASH_vs,
    /// smn: **@CODE@** tutkâmvuálásâš
    #[strum(serialize = "Der/vualasas")]
    Der_SLASH_vualasas,
    /// sma: **@CODE@** |            | AN   | Noun
    /// smj: (no comment)
    /// sms: **@CODE@** A»N -vuõtt
    /// sme: (no comment)
    /// smn: **@CODE@** |      | AN |
    #[strum(serialize = "Der/vuota")]
    Der_SLASH_vuota,
    /// kal: **@CODE@** = verb to verb
    #[strum(serialize = "Der/vv")]
    Der_SLASH_vv,
    /// kpv: **@CODE@** выв
    #[strum(serialize = "Der/vyv")]
    Der_SLASH_vyv,
    /// kpv: **@CODE@** вывса
    #[strum(serialize = "Der/vyvsa")]
    Der_SLASH_vyvsa,
    /// tlh: (no comment)
    #[strum(serialize = "Der/wi")]
    Der_SLASH_wi,
    /// kpv: **@CODE@**  ӧм verb-to-noun   !Declaring Indefinite Pronoun derivations the combinatory +Event preceding the NP-final noun
    #[strum(serialize = "Der/wm")]
    Der_SLASH_wm,
    /// kpv: **@CODE@**  = +OMON !Ӧнія коми кыв. 2000: 425
    #[strum(serialize = "Der/wmwn")]
    Der_SLASH_wmwn,
    /// kal: **@CODE@** = Follows the enclitic suffix TUQ to be able to distinguish TUQ+xx from the derivational morpheme TUQ+vn in CG: qanortoq
    #[strum(serialize = "Der/xx")]
    Der_SLASH_xx,
    /// kpv: **@CODE@**
    /// pur: (no comment)
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// xal: (no comment)
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: (no comment)
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
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
    /// lav: (no comment)
    #[strum(serialize = "Der/šana")]
    Der_SLASH_šana,
    /// koi: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Der/И")]
    Der_SLASH_И,
    /// koi: (no comment)
    #[strum(serialize = "Der/Ок")]
    Der_SLASH_Ок,
    /// udm: (no comment)
    #[strum(serialize = "Der/Он")]
    Der_SLASH_Он,
    /// udm: (no comment)
    #[strum(serialize = "Der/Оно")]
    Der_SLASH_Оно,
    /// udm: (no comment)
    #[strum(serialize = "Der/Эм")]
    Der_SLASH_Эм,
    /// udm: (no comment)
    #[strum(serialize = "Der/Эмтэ")]
    Der_SLASH_Эмтэ,
    /// udm: (no comment)
    /// udm: (no comment)
    #[strum(serialize = "Der/Этӥ")]
    Der_SLASH_Этӥ,
    /// koi: (no comment)
    #[strum(serialize = "Der/а")]
    Der_SLASH__CYRILLIC_a,
    /// koi: (no comment)
    #[strum(serialize = "Der/ал")]
    Der_SLASH__CYRILLIC_aл,
    /// koi: (no comment)
    #[strum(serialize = "Der/ась")]
    Der_SLASH__CYRILLIC_aсь,
    /// koi: (no comment)
    #[strum(serialize = "Der/выв")]
    Der_SLASH_выв,
    /// mrj: **@CODE@** =        NegPrc
    #[strum(serialize = "Der/дЫмЫ")]
    Der_SLASH_дЫмЫ,
    /// koi: (no comment)
    #[strum(serialize = "Der/дор")]
    Der_SLASH_дор,
    /// koi: (no comment)
    #[strum(serialize = "Der/иг")]
    Der_SLASH_иг,
    /// koi: (no comment)
    #[strum(serialize = "Der/йӧ")]
    Der_SLASH_йӧ,
    /// koi: (no comment)
    #[strum(serialize = "Der/кежлӧ")]
    Der_SLASH_кежлӧ,
    /// koi: (no comment)
    #[strum(serialize = "Der/коста")]
    Der_SLASH_кост_CYRILLIC_a,
    /// koi: (no comment)
    #[strum(serialize = "Der/кості")]
    Der_SLASH_кості,
    /// udm: (no comment)
    #[strum(serialize = "Der/ку")]
    Der_SLASH_ку,
    /// koi: (no comment)
    #[strum(serialize = "Der/л")]
    Der_SLASH_л,
    /// koi: (no comment)
    #[strum(serialize = "Der/ла")]
    Der_SLASH_л_CYRILLIC_a,
    /// koi: (no comment)
    #[strum(serialize = "Der/ладор")]
    Der_SLASH_л_CYRILLIC_aдор,
    /// koi: (no comment)
    #[strum(serialize = "Der/лун")]
    Der_SLASH_лун,
    /// koi: (no comment)
    #[strum(serialize = "Der/лы")]
    Der_SLASH_лы,
    /// koi: (no comment)
    #[strum(serialize = "Der/лывлы")]
    Der_SLASH_лывлы,
    /// mrj: **@CODE@** =          Pass Prc
    #[strum(serialize = "Der/мЫ")]
    Der_SLASH_мЫ,
    /// koi: (no comment)
    #[strum(serialize = "Der/мед")]
    Der_SLASH_мед,
    /// koi: (no comment)
    #[strum(serialize = "Der/моз")]
    Der_SLASH_моз,
    /// koi: (no comment)
    #[strum(serialize = "Der/мысь")]
    Der_SLASH_мысь,
    /// koi: (no comment)
    #[strum(serialize = "Der/мысьт")]
    Der_SLASH_мысьт,
    /// koi: (no comment)
    #[strum(serialize = "Der/мӧн")]
    Der_SLASH_мӧн,
    /// koi: (no comment)
    #[strum(serialize = "Der/мӧнъя")]
    Der_SLASH_мӧнъя,
    /// udm: (no comment)
    #[strum(serialize = "Der/ны")]
    Der_SLASH_ны,
    /// koi: (no comment)
    #[strum(serialize = "Der/овт")]
    Der_SLASH_овт,
    /// koi: (no comment)
    #[strum(serialize = "Der/сор")]
    Der_SLASH_сор,
    /// koi: (no comment)
    #[strum(serialize = "Der/сь")]
    Der_SLASH_сь,
    /// koi: (no comment)
    #[strum(serialize = "Der/сьы")]
    Der_SLASH_сьы,
    /// koi: (no comment)
    #[strum(serialize = "Der/сюрӧ")]
    Der_SLASH_сюрӧ,
    /// udm: (no comment)
    #[strum(serialize = "Der/тозь")]
    Der_SLASH_тозь,
    /// koi: (no comment)
    #[strum(serialize = "Der/тыр")]
    Der_SLASH_тыр,
    /// koi: (no comment)
    #[strum(serialize = "Der/тырйи")]
    Der_SLASH_тырйи,
    /// koi: (no comment)
    #[strum(serialize = "Der/тыръя")]
    Der_SLASH_тыръя,
    /// udm: (no comment)
    #[strum(serialize = "Der/тэм")]
    Der_SLASH_тэм,
    /// koi: (no comment)
    #[strum(serialize = "Der/тӧг")]
    Der_SLASH_тӧг,
    /// koi: (no comment)
    #[strum(serialize = "Der/тӧдз")]
    Der_SLASH_тӧдз,
    /// koi: (no comment)
    #[strum(serialize = "Der/тӧм")]
    Der_SLASH_тӧм,
    /// koi: (no comment)
    #[strum(serialize = "Der/тӧн")]
    Der_SLASH_тӧн,
    /// koi: (no comment)
    #[strum(serialize = "Der/чы")]
    Der_SLASH_чы,
    /// koi: (no comment)
    #[strum(serialize = "Der/чӧж")]
    Der_SLASH_чӧж,
    /// mrj: **@CODE@** =          Act Prc
    #[strum(serialize = "Der/шЫ")]
    Der_SLASH_шЫ,
    /// koi: (no comment)
    #[strum(serialize = "Der/ывлы")]
    Der_SLASH_ывлы,
    /// koi: (no comment)
    #[strum(serialize = "Der/ысь")]
    Der_SLASH_ысь,
    /// koi: (no comment)
    #[strum(serialize = "Der/ышт")]
    Der_SLASH_ышт,
    /// koi: **@CODE@** Der/ысь
    #[strum(serialize = "Der/ісь")]
    Der_SLASH_ісь,
    /// koi: (no comment)
    #[strum(serialize = "Der/ӧм")]
    Der_SLASH_ӧм,
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** |            |            |      | Position tag, required
    /// nob: **@CODE@** = derivation position
    /// smj: **@CODE@**:    - positional tags, preceeds the actual der tag
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: **@CODE@** |            |            |      |    | Position tag, required
    #[strum(serialize = "Der1")]
    Der1,
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** |            |      | Position tag, required
    /// sms: **@CODE@**
    /// smn: **@CODE@** |            |      |    | Position tag, required
    #[strum(serialize = "Der2")]
    Der2,
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** |      | Position tag, required
    /// sms: **@CODE@**
    /// smn: **@CODE@** |      |    | Position tag, required
    #[strum(serialize = "Der3")]
    Der3,
    /// fkv: **@CODE@** =
    /// sms: **@CODE@**
    /// smn: **@CODE@** |    | Position tag, required
    #[strum(serialize = "Der4")]
    Der4,
    /// fkv: **@CODE@** =
    /// sms: **@CODE@**
    #[strum(serialize = "Der5")]
    Der5,
    /// bxr: (no comment)
    #[strum(serialize = "Derm")]
    Derm,
    /// mya: (no comment)
    #[strum(serialize = "Derogatory")]
    Derogatory,
    /// mhr: **@CODE@** = desiderative
    /// apu: (no comment)
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// mdf: **@CODE@**:  desiderative ксоль "was about to; wanted to"
    #[strum(serialize = "Des")]
    Des,
    /// mhr: **@CODE@** = descriptive ideophones
    /// udm: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// koi: (no comment)
    /// koi: (no comment)
    /// koi: (no comment)
    /// mdf: **@CODE@**:  descriptive
    /// mdf: (no comment)
    #[strum(serialize = "Descr")]
    Descr,
    /// sje: (no comment)
    /// gle: **@CODE@** = Determiner, e.g. possessive determiner: mo, do
    /// kpv: **@CODE@**:  determiner  XX   XX
    /// vot: (no comment)
    /// pma: (no comment)
    /// evn: (no comment)
    /// rue: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// rus: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Determiner
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**  = determiner
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// non: (no comment)
    /// mdf: **@CODE@**:  Determiner
    /// khk: (no comment)
    /// vro: **@CODE@** Determiners
    /// vro: (no comment)
    #[strum(serialize = "Det")]
    Det,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// myv: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Dial")]
    Dial,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Dial/-C")]
    Dial_SLASH__MINUS_C,
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
    /// lut: **@CODE@** not North Lushootseed
    /// slh: **@CODE@** not North Lushootseed
    #[strum(serialize = "Dial/-NL")]
    Dial_SLASH__MINUS_NL,
    /// sma: **@CODE@** | Words not in Norway
    #[strum(serialize = "Dial/-NOR")]
    Dial_SLASH__MINUS_NOR,
    /// fkv: **@CODE@** = Not Porsanger
    #[strum(serialize = "Dial/-Por")]
    Dial_SLASH__MINUS_Por,
    /// sma: **@CODE@** | Not in the South
    #[strum(serialize = "Dial/-S")]
    Dial_SLASH__MINUS_S,
    /// lut: **@CODE@** not South Lushootseed
    /// slh: **@CODE@** not South Lushootseed
    #[strum(serialize = "Dial/-SL")]
    Dial_SLASH__MINUS_SL,
    /// mns: **@CODE@** forms not in use in SOSV (Sosva)
    #[strum(serialize = "Dial/-SOSV")]
    Dial_SLASH__MINUS_SOSV,
    /// sma: **@CODE@** | Words not in Sweden
    #[strum(serialize = "Dial/-SW")]
    Dial_SLASH__MINUS_SW,
    /// fkv: **@CODE@** = Not Varanger
    #[strum(serialize = "Dial/-Var")]
    Dial_SLASH__MINUS_Var,
    /// vro: **@CODE@** dialect u stem where o expected
    #[strum(serialize = "Dial/-u-not-o")]
    Dial_SLASH__MINUS_u_MINUS_not_MINUS_o,
    /// myv: (no comment)
    #[strum(serialize = "Dial/C")]
    Dial_SLASH_C,
    /// yrk: **@CODE@** (Eastern dialects),
    #[strum(serialize = "Dial/E")]
    Dial_SLASH_E,
    /// cwd: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Dial/East")]
    Dial_SLASH_East,
    /// kal: **@CODE@** = Eastern dialect
    #[strum(serialize = "Dial/Egr")]
    Dial_SLASH_Egr,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Eteläpohjalaiset")]
    Dial_SLASH_Eteläpohjalaiset,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Finland")]
    Dial_SLASH_Finland,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Häme")]
    Dial_SLASH_Häme,
    /// koi: (no comment)
    #[strum(serialize = "Dial/Injv")]
    Dial_SLASH_Injv,
    /// fkv: **@CODE@** Jokivarret, short for +Dial/-Por+Dial/-V
    #[strum(serialize = "Dial/Jok")]
    Dial_SLASH_Jok,
    /// kca: (no comment)
    #[strum(serialize = "Dial/Kaz")]
    Dial_SLASH_Kaz,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Keskipohjalaiset")]
    Dial_SLASH_Keskipohjalaiset,
    /// sma: **@CODE@** | Long forms
    /// myv: (no comment)
    #[strum(serialize = "Dial/L")]
    Dial_SLASH_L,
    /// izh: **@CODE@**  Dialect tag. Peculiar to Laukaa
    #[strum(serialize = "Dial/Lauk")]
    Dial_SLASH_Lauk,
    /// myv: (no comment)
    #[strum(serialize = "Dial/M")]
    Dial_SLASH_M,
    /// sma: **@CODE@** | Only in the North
    /// smj: **@CODE@** Used in the northern areas.  Some might say that
    /// sms: **@CODE@** Nuortjärvi (KKS)
    #[strum(serialize = "Dial/N")]
    Dial_SLASH_N,
    /// sma: **@CODE@** | Words only in Norway
    #[strum(serialize = "Dial/NOR")]
    Dial_SLASH_NOR,
    /// hdn: (no comment)
    #[strum(serialize = "Dial/NOT-A")]
    Dial_SLASH_NOT_MINUS_A,
    /// hdn: (no comment)
    #[strum(serialize = "Dial/NOT-M")]
    Dial_SLASH_NOT_MINUS_M,
    /// myv: (no comment)
    #[strum(serialize = "Dial/NW")]
    Dial_SLASH_NW,
    /// kal: **@CODE@** = Northern dialect (such as ng for g; predictable dialectal changes might be dealt with in the postprosessor in stead)
    #[strum(serialize = "Dial/Ngr")]
    Dial_SLASH_Ngr,
    /// udm: (no comment)
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
    /// myv: (no comment)
    #[strum(serialize = "Dial/SE")]
    Dial_SLASH_SE,
    /// sma: **@CODE@** | Short forms
    /// smj: **@CODE@** Short forms
    /// myv: (no comment)
    #[strum(serialize = "Dial/SH")]
    Dial_SLASH_SH,
    /// sma: **@CODE@** | Words only in Sweden
    #[strum(serialize = "Dial/SW")]
    Dial_SLASH_SW,
    /// fin: (no comment)
    #[strum(serialize = "Dial/Savo")]
    Dial_SLASH_Savo,
    /// kal: **@CODE@** = Southern dialect (such as VVC for VVCC), aneeqavoq (and not aneeqqavoq)
    #[strum(serialize = "Dial/Sgr")]
    Dial_SLASH_Sgr,
    /// kca: (no comment)
    #[strum(serialize = "Dial/Shur")]
    Dial_SLASH_Shur,
    /// izh: **@CODE@**  Dialects tag. Peculiar to Soikkola
    #[strum(serialize = "Dial/Soik")]
    Dial_SLASH_Soik,
    /// udm: (no comment)
    #[strum(serialize = "Dial/South")]
    Dial_SLASH_South,
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
    /// kal: **@CODE@** = Western dialect (such as rng for rn)
    #[strum(serialize = "Dial/Vgr")]
    Dial_SLASH_Vgr,
    /// yrk: **@CODE@** (Western dialects),
    /// myv: (no comment)
    #[strum(serialize = "Dial/W")]
    Dial_SLASH_W,
    /// myv: (no comment)
    #[strum(serialize = "Dial/W-NW")]
    Dial_SLASH_W_MINUS_NW,
    /// myv: (no comment)
    #[strum(serialize = "Dial/W-SW")]
    Dial_SLASH_W_MINUS_SW,
    /// cwd: (no comment)
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
    /// koi: (no comment)
    #[strum(serialize = "Dial/сев.чаз.")]
    Dial_SLASH_сев_DOT_ч_CYRILLIC_aз_DOT_,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Dig1")]
    Dig1,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Dig2")]
    Dig2,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Dig3")]
    Dig3,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Dig4")]
    Dig4,
    /// kpv: **@CODE@** diminutive for verbs -ышт- (there might be a better term)
    /// chp: (no comment)
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// crj: (no comment)
    /// bxr: (no comment)
    /// eus: (no comment)
    /// kio: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "Dim")]
    Dim,
    /// crj: (no comment)
    #[strum(serialize = "Dim/Der")]
    Dim_SLASH_Der,
    /// kpv: **@CODE@** diminutive for nouns -тор-
    /// lut: (no comment)
    /// slh: (no comment)
    /// sms: **@CODE@** diminutive without derivation 2025-04-04 jaska
    /// myv: (no comment)
    /// koi: **@CODE@** diminutive,  words that are already -ok dimin
    /// mdf: (no comment)
    #[strum(serialize = "Dimin")]
    Dimin,
    /// gle: **@CODE@** = Directional adverb
    /// evn: (no comment)
    /// tgl: **@CODE@** -
    /// tgl: **@CODE@** - Direct
    /// hdn: (no comment)
    /// tkl: (no comment)
    /// ceb: **@CODE@** -
    /// ceb: **@CODE@** - Direct
    /// khk: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Dir")]
    Dir,
    /// gle: **@CODE@** = Direct relative particle
    #[strum(serialize = "Direct")]
    Direct,
    /// mpj: **@CODE@**  ni (suffix / infix)
    #[strum(serialize = "Directional/towards")]
    Directional_SLASH_towards,
    /// tgl: **@CODE@** - Distributive
    /// ceb: **@CODE@** - Distributive
    #[strum(serialize = "Dis")]
    Dis,
    /// mns: (no comment)
    #[strum(serialize = "Disc/ke")]
    Disc_SLASH_ke,
    /// mns: (no comment)
    #[strum(serialize = "Disc/ta")]
    Disc_SLASH_ta,
    /// mns: (no comment)
    #[strum(serialize = "Disc/te")]
    Disc_SLASH_te,
    /// mns: (no comment)
    #[strum(serialize = "Disc/ty")]
    Disc_SLASH_ty,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// dag: (no comment)
    /// crj: (no comment)
    /// bxr: (no comment)
    /// som: (no comment)
    /// koi: (no comment)
    /// kio: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Dist")]
    Dist,
    /// xak: (no comment)
    #[strum(serialize = "DistFut")]
    DistFut,
    /// xak: (no comment)
    #[strum(serialize = "DistPst")]
    DistPst,
    /// pma: (no comment)
    #[strum(serialize = "Distal")]
    Distal,
    /// kpv: **@CODE@**:  Distributive
    /// cwd: (no comment)
    /// crk: (no comment)
    /// apu: (no comment)
    /// sms: **@CODE@** Distributive
    /// myv: (no comment)
    /// koi: **@CODE@**:  Distributive
    /// koi: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  Distributive
    #[strum(serialize = "Distr")]
    Distr,
    /// hdn: (no comment)
    #[strum(serialize = "Distrib")]
    Distrib,
    /// kal: **@CODE@** = Dualis
    /// kpv: **@CODE@** for pronoun.
    /// chp: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** = Dual
    /// smj: **@CODE@** = Dual number
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Dual = kuõiʹtilååkk
    /// koi: (no comment)
    /// sme: **@CODE@** - Dual
    #[strum(serialize = "Du")]
    Du,
    /// sje: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** | Dual    , 1.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// sms: **@CODE@** first person dual
    /// lit: (no comment)
    /// tkl: (no comment)
    /// sme: **@CODE@** Dual First Person
    /// smn: **@CODE@**
    /// ceb: (no comment)
    #[strum(serialize = "Du1")]
    Du1,
    /// sje: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** | Dual    , 2.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// sms: **@CODE@** second person dual
    /// tkl: (no comment)
    /// sme: **@CODE@** Dual Second Person
    /// ceb: (no comment)
    #[strum(serialize = "Du2")]
    Du2,
    /// sje: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** | Dual    , 3.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// sms: **@CODE@** third person dual
    /// tkl: (no comment)
    /// sme: **@CODE@** Dual Third Person
    /// ceb: (no comment)
    #[strum(serialize = "Du3")]
    Du3,
    /// bxr: (no comment)
    #[strum(serialize = "DualPrc")]
    DualPrc,
    /// bxr: (no comment)
    /// tku: (no comment)
    #[strum(serialize = "Dur")]
    Dur,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "Duration")]
    Duration,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** = Dynamically created acronym
    /// sma: **@CODE@** = Code dynamic acronyms
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Code dynamic acronyms
    /// sms: **@CODE@** = Dynamically generated (acronyms)
    /// rmn: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**  Dynamically generated (acronyms) +ACR+Dyn
    /// smn: **@CODE@** - Dynamic Acronym
    #[strum(serialize = "Dyn")]
    Dyn,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/BXR")]
    EOLang_SLASH_BXR,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/CHM")]
    EOLang_SLASH_CHM,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/KOI")]
    EOLang_SLASH_KOI,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/KOM")]
    EOLang_SLASH_KOM,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/KPV")]
    EOLang_SLASH_KPV,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/MDF")]
    EOLang_SLASH_MDF,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/MHR")]
    EOLang_SLASH_MHR,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/MRJ")]
    EOLang_SLASH_MRJ,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/MYV")]
    EOLang_SLASH_MYV,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/RUS")]
    EOLang_SLASH_RUS,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "EOLang/YRK")]
    EOLang_SLASH_YRK,
    /// mhr: **@CODE@** = change to other transitivity
    #[strum(serialize = "EX/IV")]
    EX_SLASH_IV,
    /// gle: **@CODE@** = Eclipsis (+Urú) initial mutation, e.g. ar an gcat
    #[strum(serialize = "Ecl")]
    Ecl,
    /// hdn: (no comment)
    #[strum(serialize = "Edl")]
    Edl,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Edu")]
    Edu,
    /// eus: (no comment)
    #[strum(serialize = "Egi")]
    Egi,
    /// kpv: **@CODE@** egressive -сянь ылыстчан
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Egr")]
    Egr,
    /// xak: (no comment)
    #[strum(serialize = "El")]
    El,
    /// fit: **@CODE@** = Elative
    /// sje: (no comment)
    /// kpv: **@CODE@** elative -ысь петан
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Elative
    /// liv: **@CODE@** = elative
    /// evn: (no comment)
    /// sma: **@CODE@** = Elative
    /// fao: (no comment)
    /// smj: **@CODE@** = Elative case
    /// udm: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@** Elative
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = elative
    /// vep: (no comment)
    /// cho: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** elative
    /// koi: (no comment)
    /// deu: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  elative
    /// vro: **@CODE@** elative
    /// bak: (no comment)
    #[strum(serialize = "Ela")]
    Ela,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Elid")]
    Elid,
    /// gle: **@CODE@** = Emphatic (Contrastive) form of personal pronoun e.g. ár dteachsa, do theachsa, a teachsa
    /// apu: **@CODE@** = pyty: Ithu-pyty-ma-ry, Iãu-pyty-ka-ry, ithu-pyty-ry
    /// mns: **@CODE@** This might overlap with Pron+Refl
    /// tkl: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Emph")]
    Emph,
    /// myv: (no comment)
    /// mdf: **@CODE@**:  used with negation particles афи,
    #[strum(serialize = "Emphatic")]
    Emphatic,
    /// gle: **@CODE@** = end bracket, quote etc
    /// gle: **@CODE@** = end bracket, quote etc
    #[strum(serialize = "End")]
    End,
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "Eng")]
    Eng,
    /// gle: **@CODE@** = English language words
    #[strum(serialize = "English")]
    English,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Epenth")]
    Epenth,
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// myv: (no comment)
    /// mdf: **@CODE@**:  epistemic
    #[strum(serialize = "Epist")]
    Epist,
    /// hdn: (no comment)
    #[strum(serialize = "Epl")]
    Epl,
    /// yrk: **@CODE@**
    #[strum(serialize = "Equ")]
    Equ,
    /// koi: (no comment)
    /// pad: (no comment)
    /// mpj: **@CODE@**
    #[strum(serialize = "Erg")]
    Erg,
    /// qya: (no comment)
    #[strum(serialize = "Err/-Norm")]
    Err_SLASH__MINUS_Norm,
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
    /// mdf: (no comment)
    /// vro: **@CODE@** This is the initial Dialect distinction
    #[strum(serialize = "Err/Dial")]
    Err_SLASH_Dial,
    /// cwd: (no comment)
    #[strum(serialize = "Err/Dim")]
    Err_SLASH_Dim,
    /// cwd: (no comment)
    #[strum(serialize = "Err/Dummy")]
    Err_SLASH_Dummy,
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "Err/Frag")]
    Err_SLASH_Frag,
    /// sms: **@CODE@**
    #[strum(serialize = "Err/GenreLeudd")]
    Err_SLASH_GenreLeudd,
    /// fao: (no comment)
    #[strum(serialize = "Err/Guess")]
    Err_SLASH_Guess,
    /// fit: **@CODE@** =
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | Substandard, unormert
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = when there is a hyphen where none should have been
    /// sms: **@CODE@** substandard for compounding, not in normative fst (should have no hyphen)
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** when there is a hyphen where none should have been
    /// smn: **@CODE@** - substandard, not in normative fst
    /// fin: (no comment)
    /// bak: (no comment)
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
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** substandard, not in normative fst, no  normative lemma помсьыны
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | lemma med dens ordformer er utenfor normen. <br/>No normative lemma, it's grammatically correct.
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = No normative lemma
    /// sms: **@CODE@** substandard, not in normative fst, no normative lemma
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** substandard, not in normative fst, no normative lemma
    /// smn: **@CODE@** - substandard, not in normative fst, no normative lemma
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Err/Lex")]
    Err_SLASH_Lex,
    /// sma: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** substandard for compounding, not in normative fst (needs hyph)
    /// sme: **@CODE@** when there is no hyphen where it should have been
    #[strum(serialize = "Err/MissingHyph")]
    Err_SLASH_MissingHyph,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | in use in smi lexc
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = indicates that there is a missing space, causing an orthographic error. Used for "goadedagi", when it should be "goade dagi"
    /// sms: **@CODE@** | in use ins smi lexc
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** indicates that there is a missing space, causing an orthographic error
    /// smn: **@CODE@** - in use in smi lexc
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Err/MissingSpace")]
    Err_SLASH_MissingSpace,
    /// cwd: (no comment)
    #[strum(serialize = "Err/Morph")]
    Err_SLASH_Morph,
    /// fro: (no comment)
    #[strum(serialize = "Err/Obsc")]
    Err_SLASH_Obsc,
    /// fit: (no comment)
    /// sje: (no comment)
    /// gle: **@CODE@** = Orthografical error
    /// kal: **@CODE@**  tag, vi ikke bruger, men som optræder i de delte filer
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// pur: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@**  misspelling or nor normative form. It will be included only in desc, not in norm.
    /// izh: **@CODE@** error forms
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = orthographical error (analysed, not accepted in speller)
    /// krl: **@CODE@**
    /// rue: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** | Substandard, unormert form av et ord
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@**    For speller use
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Substandard. An ungrammatical, non-normative form of normative lemma.
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**  do not accept, but recognize
    /// sms: **@CODE@**  substandard, not in normative fst
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@**
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@** substandard, not in normative fst
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// mya: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// xak: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** substandard, not in normative fst
    /// smn: **@CODE@** - substandard, not in normative fst
    /// srs: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  substandard / outside the written norm
    /// ale: (no comment)
    /// rmf: **@CODE@** tagging non-standard forms
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Err/Orth")]
    Err_SLASH_Orth,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-0-not-pal")]
    Err_SLASH_Orth_MINUS_0_MINUS_not_MINUS_pal,
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
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-back-should-be-hard-front")]
    Err_SLASH_Orth_MINUS_back_MINUS_should_MINUS_be_MINUS_hard_MINUS_front,
    /// kpv: **@CODE@** colloquial form for patronymic
    /// udm: (no comment)
    /// mns: (no comment)
    /// bxr: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-colloq")]
    Err_SLASH_Orth_MINUS_colloq,
    /// myv: (no comment)
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-cons-stem")]
    Err_SLASH_Orth_MINUS_cons_MINUS_stem,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-cons-stem-front")]
    Err_SLASH_Orth_MINUS_cons_MINUS_stem_MINUS_front,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-d")]
    Err_SLASH_Orth_MINUS_d,
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
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-f")]
    Err_SLASH_Orth_MINUS_f,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-freq-le")]
    Err_SLASH_Orth_MINUS_freq_MINUS_le,
    /// vro: **@CODE@** erroneous front harmony, see flags
    #[strum(serialize = "Err/Orth-front")]
    Err_SLASH_Orth_MINUS_front,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-front-linking-vowel")]
    Err_SLASH_Orth_MINUS_front_MINUS_linking_MINUS_vowel,
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-glide-final")]
    Err_SLASH_Orth_MINUS_glide_MINUS_final,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-high-linking-vowel")]
    Err_SLASH_Orth_MINUS_high_MINUS_linking_MINUS_vowel,
    /// apu: **@CODE@**
    #[strum(serialize = "Err/Orth-i-not-y")]
    Err_SLASH_Orth_MINUS_i_MINUS_not_MINUS_y,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-je-for-jo")]
    Err_SLASH_Orth_MINUS_je_MINUS_for_MINUS_jo,
    /// sms: **@CODE@** substandard k, not in normative fst ǩ(, should be  kuõccjiǩ and not kuõccjik)
    #[strum(serialize = "Err/Orth-k-kh")]
    Err_SLASH_Orth_MINUS_k_MINUS_kh,
    /// kpv: **@CODE@** followed by vowel, yet v
    #[strum(serialize = "Err/Orth-l-in-v")]
    Err_SLASH_Orth_MINUS_l_MINUS_in_MINUS_v,
    /// koi: (no comment)
    #[strum(serialize = "Err/Orth-l-retension")]
    Err_SLASH_Orth_MINUS_l_MINUS_retension,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-l-retenstion")]
    Err_SLASH_Orth_MINUS_l_MINUS_retenstion,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-l-to-v-in-new")]
    Err_SLASH_Orth_MINUS_l_MINUS_to_MINUS_v_MINUS_in_MINUS_new,
    /// kpv: **@CODE@**
    #[strum(serialize = "Err/Orth-l-to-vowel-lengthening")]
    Err_SLASH_Orth_MINUS_l_MINUS_to_MINUS_vowel_MINUS_lengthening,
    /// mns: **@CODE@**  do not accept, but recognize vowel is long
    #[strum(serialize = "Err/Orth-long-v")]
    Err_SLASH_Orth_MINUS_long_MINUS_v,
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-lowered-final-e-2-ja")]
    Err_SLASH_Orth_MINUS_lowered_MINUS_final_MINUS_e_MINUS_2_MINUS_ja,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-lowered-vow")]
    Err_SLASH_Orth_MINUS_lowered_MINUS_vow,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-mid-linking-vowel-should-be-high")]
    Err_SLASH_Orth_MINUS_mid_MINUS_linking_MINUS_vowel_MINUS_should_MINUS_be_MINUS_high,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-mid-onset-default-missing")]
    Err_SLASH_Orth_MINUS_mid_MINUS_onset_MINUS_default_MINUS_missing,
    /// sms: **@CODE@**  substandard, missing apostrophe
    #[strum(serialize = "Err/Orth-missing-apos")]
    Err_SLASH_Orth_MINUS_missing_MINUS_apos,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-missing-soft-in-stem")]
    Err_SLASH_Orth_MINUS_missing_MINUS_soft_MINUS_in_MINUS_stem,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-missing-t-in-def-pl")]
    Err_SLASH_Orth_MINUS_missing_MINUS_t_MINUS_in_MINUS_def_MINUS_pl,
    /// skf: (no comment)
    #[strum(serialize = "Err/Orth-no-glot")]
    Err_SLASH_Orth_MINUS_no_MINUS_glot,
    /// mns: **@CODE@**  do not accept, preverb merged without hyphen
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-no-hyphen")]
    Err_SLASH_Orth_MINUS_no_MINUS_hyphen,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-no-linking-vowel")]
    Err_SLASH_Orth_MINUS_no_MINUS_linking_MINUS_vowel,
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
    /// lut: (no comment)
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
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-old1")]
    Err_SLASH_Orth_MINUS_old1,
    /// sms: **@CODE@** substandard palatalized diphthong ieʹ should be eä or eâ; ueʹ should be uä, uâ <no soft sign>
    #[strum(serialize = "Err/Orth-pal-vow")]
    Err_SLASH_Orth_MINUS_pal_MINUS_vow,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-pre1880")]
    Err_SLASH_Orth_MINUS_pre1880,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-pre1978")]
    Err_SLASH_Orth_MINUS_pre1978,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-pre2012")]
    Err_SLASH_Orth_MINUS_pre2012,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-raised-vow")]
    Err_SLASH_Orth_MINUS_raised_MINUS_vow,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-s")]
    Err_SLASH_Orth_MINUS_s,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-s-to-j")]
    Err_SLASH_Orth_MINUS_s_MINUS_to_MINUS_j,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-shib-hard")]
    Err_SLASH_Orth_MINUS_shib_MINUS_hard,
    /// sms: **@CODE@** substandard raajeeʹl (->) raajjeeʹl
    #[strum(serialize = "Err/Orth-should-be-grade-minus1")]
    Err_SLASH_Orth_MINUS_should_MINUS_be_MINUS_grade_MINUS_minus1,
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-soft-kept")]
    Err_SLASH_Orth_MINUS_soft_MINUS_kept,
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-soft-loss")]
    Err_SLASH_Orth_MINUS_soft_MINUS_loss,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-split-tween")]
    Err_SLASH_Orth_MINUS_split_MINUS_tween,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-a-should-be-o0")]
    Err_SLASH_Orth_MINUS_stem_MINUS_a_MINUS_should_MINUS_be_MINUS_o0,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-cons-should-be-je")]
    Err_SLASH_Orth_MINUS_stem_MINUS_cons_MINUS_should_MINUS_be_MINUS_je,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-hard-e-should-be-je")]
    Err_SLASH_Orth_MINUS_stem_MINUS_hard_MINUS_e_MINUS_should_MINUS_be_MINUS_je,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-ja-should-be-je0")]
    Err_SLASH_Orth_MINUS_stem_MINUS_ja_MINUS_should_MINUS_be_MINUS_je0,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-je-should-be-ja")]
    Err_SLASH_Orth_MINUS_stem_MINUS_je_MINUS_should_MINUS_be_MINUS_ja,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-je-should-be-je0")]
    Err_SLASH_Orth_MINUS_stem_MINUS_je_MINUS_should_MINUS_be_MINUS_je0,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-nodent-hard-should-be-tnje")]
    Err_SLASH_Orth_MINUS_stem_MINUS_nodent_MINUS_hard_MINUS_should_MINUS_be_MINUS_tnje,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-stem-soft-should-be-0")]
    Err_SLASH_Orth_MINUS_stem_MINUS_soft_MINUS_should_MINUS_be_MINUS_0,
    /// mdf: (no comment)
    #[strum(serialize = "Err/Orth-t-d")]
    Err_SLASH_Orth_MINUS_t_MINUS_d,
    /// vro: **@CODE@** should be o stem, but is u stem
    #[strum(serialize = "Err/Orth-u-stem")]
    Err_SLASH_Orth_MINUS_u_MINUS_stem,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-v-loss-before-lab")]
    Err_SLASH_Orth_MINUS_v_MINUS_loss_MINUS_before_MINUS_lab,
    /// sms: (no comment)
    #[strum(serialize = "Err/Orth-vow-not-raised")]
    Err_SLASH_Orth_MINUS_vow_MINUS_not_MINUS_raised,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-vowel-stem-je")]
    Err_SLASH_Orth_MINUS_vowel_MINUS_stem_MINUS_je,
    /// vro: **@CODE@**
    #[strum(serialize = "Err/Orth-weak-grade")]
    Err_SLASH_Orth_MINUS_weak_MINUS_grade,
    /// myv: (no comment)
    #[strum(serialize = "Err/Orth-z-to-j")]
    Err_SLASH_Orth_MINUS_z_MINUS_to_MINUS_j,
    /// liv: **@CODE@** is õ should be i lītõm should be lītim 2025-09-09
    #[strum(serialize = "Err/Orth-õ-i")]
    Err_SLASH_Orth_MINUS_õ_MINUS_i,
    /// izh: **@CODE@** Marks Overgeneration of case formatives, such as, exessive with personal pronouns
    #[strum(serialize = "Err/OverG")]
    Err_SLASH_OverG,
    /// fit: **@CODE@** =
    /// gle: **@CODE@** = Compound space error
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// sma: **@CODE@** | Substandard, unormert
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = used for compounds written apart - only retained in the HFST Grammar Checker disambiguation analyser
    /// sms: **@CODE@** used for compounds written apart - only retained in the HFST Grammar Checker disambiguation analyser
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** used for compounds written apart - only retained in the HFST Grammar Checker disambiguation analyser
    /// smn: **@CODE@** - substandard, not in normative fst
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Err/SpaceCmp")]
    Err_SLASH_SpaceCmp,
    /// sma: **@CODE@** | Used to tag spellrelaxed typos (tag is inserted via flag diacritics)
    /// lut: (no comment)
    /// apu: (no comment)
    /// sms: **@CODE@** used to tag spellrelaxed typos (tag is inserted via flag diacritics)
    /// sme: **@CODE@** used to tag spellrelaxed typos (tag is inserted via flag diacritics)
    #[strum(serialize = "Err/Spellrelax")]
    Err_SLASH_Spellrelax,
    /// kal: **@CODE@** = Not authorized spelling: 2-imik
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    #[strum(serialize = "Err/Sub")]
    Err_SLASH_Sub,
    /// cwd: (no comment)
    #[strum(serialize = "Err/Thm")]
    Err_SLASH_Thm,
    /// gle: **@CODE@** = error in Learner corpus
    #[strum(serialize = "Error")]
    Error,
    /// fit: **@CODE@** = Essive
    /// sje: (no comment)
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Essive
    /// izh: **@CODE@**
    /// liv: **@CODE@** = essive
    /// pma: (no comment)
    /// inp: (no comment)
    /// krl: **@CODE@**
    /// xal: (no comment)
    /// sma: **@CODE@** = Essive
    /// lut: (no comment)
    /// sjt: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Essive case
    /// udm: (no comment)
    /// tqb: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// xwo: (no comment)
    /// sms: **@CODE@** Essive
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@**
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = essive
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// olo: **@CODE@** essive
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Essive
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@** essive
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Ess")]
    Ess,
    /// hun: (no comment)
    #[strum(serialize = "Ess/F")]
    Ess_SLASH_F,
    /// hun: (no comment)
    #[strum(serialize = "Ess/M")]
    Ess_SLASH_M,
    /// vep: **@CODE@**  =
    #[strum(serialize = "EssInst")]
    EssInst,
    /// yrk: **@CODE@**
    #[strum(serialize = "Evas")]
    Evas,
    /// gle: **@CODE@** = Simple Event (laugh, sneeze etc.) in transcribed speech
    /// koi: (no comment)
    #[strum(serialize = "Event")]
    Event,
    /// fin: (no comment)
    #[strum(serialize = "Eventv")]
    Eventv,
    /// khk: (no comment)
    #[strum(serialize = "Evi")]
    Evi,
    /// hdn: (no comment)
    /// skf: (no comment)
    #[strum(serialize = "Evid")]
    Evid,
    /// myv: (no comment)
    #[strum(serialize = "EvidFh")]
    EvidFh,
    /// myv: (no comment)
    /// mdf: **@CODE@**:  кле
    #[strum(serialize = "EvidNfh")]
    EvidNfh,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mhr: **@CODE@** = for derivation from A to anoter POS
    /// sms: **@CODE@** = Adjective stem before derivation
    /// myv: (no comment)
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// smn: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mdf: (no comment)
    #[strum(serialize = "Ex/A")]
    Ex_SLASH_A,
    /// myv: (no comment)
    #[strum(serialize = "Ex/Adv")]
    Ex_SLASH_Adv,
    /// fit: **@CODE@**
    /// fkv: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mhr: **@CODE@** = for derivation from N to anoter POS
    /// sms: **@CODE@** = Noun stem before derivation
    /// myv: (no comment)
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// smn: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mdf: (no comment)
    #[strum(serialize = "Ex/N")]
    Ex_SLASH_N,
    /// sms: **@CODE@** = Number stem before derivation
    /// myv: (no comment)
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
    /// myv: (no comment)
    /// olo: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// smn: **@CODE@** - This tag is not added in lexc. The POS tag before derivation is converted into this tag when compiling FST for disambiguation.
    /// mdf: (no comment)
    #[strum(serialize = "Ex/V")]
    Ex_SLASH_V,
    /// som: (no comment)
    /// xak: (no comment)
    #[strum(serialize = "Excl")]
    Excl,
    /// ciw: (no comment)
    #[strum(serialize = "ExclObj")]
    ExclObj,
    /// myv: **@CODE@**:  ськамонза
    /// myv: **@CODE@**:  ськамонзо
    /// myv: (no comment)
    /// mdf: **@CODE@**:  ськамонза
    #[strum(serialize = "Exclusive")]
    Exclusive,
    /// izh: **@CODE@**
    /// liv: **@CODE@** = exessive
    /// vep: **@CODE@**  = essive
    #[strum(serialize = "Exe")]
    Exe,
    /// mhr: **@CODE@** = form уло
    /// hdn: (no comment)
    #[strum(serialize = "Ext")]
    Ext,
    /// myv: (no comment)
    #[strum(serialize = "F")]
    F,
    /// nso: (no comment)
    #[strum(serialize = "FNA")]
    FNA,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Fac")]
    Fac,
    /// apu: (no comment)
    #[strum(serialize = "Fact")]
    Fact,
    /// gle: **@CODE@** = Family Name - proper noun
    #[strum(serialize = "Fam")]
    Fam,
    /// som: (no comment)
    #[strum(serialize = "Far")]
    Far,
    /// som: (no comment)
    #[strum(serialize = "Farther")]
    Farther,
    /// gle: **@CODE@** = Feminine gender
    /// pur: (no comment)
    /// tgl: (no comment)
    /// txi: (no comment)
    /// tqb: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// apu: **@CODE@** uwa
    /// waw: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// moh: (no comment)
    /// myu: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: (no comment)
    /// sqi: (no comment)
    /// mya: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// pad: **@CODE@** feminine nouns usually require a correlating demonstrative ʼida
    /// pad: (no comment)
    /// non: (no comment)
    /// mdf: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Fem")]
    Fem,
    /// mya: (no comment)
    #[strum(serialize = "Female")]
    Female,
    /// mya: (no comment)
    #[strum(serialize = "FemaleSpeaker")]
    FemaleSpeaker,
    /// myv: (no comment)
    #[strum(serialize = "Field/Agr")]
    Field_SLASH_Agr,
    /// myv: (no comment)
    #[strum(serialize = "Field/Anat")]
    Field_SLASH_Anat,
    /// myv: (no comment)
    #[strum(serialize = "Field/Bio")]
    Field_SLASH_Bio,
    /// myv: (no comment)
    #[strum(serialize = "Field/Bot")]
    Field_SLASH_Bot,
    /// myv: (no comment)
    #[strum(serialize = "Field/Chem")]
    Field_SLASH_Chem,
    /// myv: (no comment)
    #[strum(serialize = "Field/Geol")]
    Field_SLASH_Geol,
    /// myv: (no comment)
    #[strum(serialize = "Field/Gram")]
    Field_SLASH_Gram,
    /// myv: (no comment)
    #[strum(serialize = "Field/Hist")]
    Field_SLASH_Hist,
    /// myv: (no comment)
    #[strum(serialize = "Field/Law")]
    Field_SLASH_Law,
    /// myv: (no comment)
    #[strum(serialize = "Field/Mar")]
    Field_SLASH_Mar,
    /// myv: (no comment)
    #[strum(serialize = "Field/Math")]
    Field_SLASH_Math,
    /// myv: (no comment)
    #[strum(serialize = "Field/Med")]
    Field_SLASH_Med,
    /// myv: (no comment)
    #[strum(serialize = "Field/Mus")]
    Field_SLASH_Mus,
    /// myv: (no comment)
    #[strum(serialize = "Field/Relig")]
    Field_SLASH_Relig,
    /// myv: (no comment)
    #[strum(serialize = "Field/Tech")]
    Field_SLASH_Tech,
    /// myv: (no comment)
    #[strum(serialize = "Field/Zool")]
    Field_SLASH_Zool,
    /// gle: **@CODE@** = Filled Pause (eh, em, etc.) in transcribed speech
    #[strum(serialize = "Filler")]
    Filler,
    /// gle: **@CODE@** = sentence final punctuation
    #[strum(serialize = "Fin")]
    Fin,
    /// kpv: **@CODE@**  мог, мон, моз 'so that I won't'
    /// koi: (no comment)
    #[strum(serialize = "Final")]
    Final,
    /// xak: (no comment)
    #[strum(serialize = "Firsth")]
    Firsth,
    /// kpv: **@CODE@**
    /// chp: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// izh: **@CODE@** focus
    /// liv: **@CODE@**
    /// crk: (no comment)
    /// mhr: **@CODE@** =
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle?
    /// smj: (no comment)
    /// udm: (no comment)
    /// skf: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// smn: **@CODE@**
    /// kio: (no comment)
    /// rmf: (no comment)
    /// rmf: (no comment)
    /// rmf: (no comment)
    /// rmf: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Foc")]
    Foc,
    /// sms: **@CODE@** more specifically, exactly, contrast. used with Pron and Adv
    #[strum(serialize = "Foc/AA")]
    Foc_SLASH_AA,
    /// krl: **@CODE@** convert in Apertium _eikä_ "ei" V Neg Act Sg3 Foc/Ka
    #[strum(serialize = "Foc/Ka")]
    Foc_SLASH_Ka,
    /// som: (no comment)
    #[strum(serialize = "Foc/L")]
    Foc_SLASH_L,
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
    /// som: (no comment)
    #[strum(serialize = "Foc/R")]
    Foc_SLASH_R,
    /// som: (no comment)
    #[strum(serialize = "Foc/V")]
    Foc_SLASH_V,
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
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/ba")]
    Foc_SLASH_ba,
    /// som: (no comment)
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/baa")]
    Foc_SLASH_baa,
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/baan")]
    Foc_SLASH_baan,
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
    /// smn: **@CODE@**
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
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/gan")]
    Foc_SLASH_gan,
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/gas")]
    Foc_SLASH_gas,
    /// sma: **@CODE@**:  XXX Document better = Forsterkende particle
    /// smj: (no comment)
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/ge")]
    Foc_SLASH_ge,
    /// smj: (no comment)
    /// sme: **@CODE@**	Focus clitic
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/gen")]
    Foc_SLASH_gen,
    /// sme: **@CODE@**	Focus clitic
    /// smn: **@CODE@**
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
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/gin")]
    Foc_SLASH_gin,
    /// sme: **@CODE@**	Focus clitic
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/gis")]
    Foc_SLASH_gis,
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/go")]
    Foc_SLASH_go,
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
    /// smn: **@CODE@**
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
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/kas")]
    Foc_SLASH_kas,
    /// sms: **@CODE@** This might be used with Use/NG
    /// vep: **@CODE@**  =
    #[strum(serialize = "Foc/ki")]
    Foc_SLASH_ki,
    /// izh: **@CODE@**
    #[strum(serialize = "Foc/kii")]
    Foc_SLASH_kii,
    /// fit: **@CODE@** =
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fit: (no comment)
    /// fkv: **@CODE@** = Focusclitic -kin
    /// smn: **@CODE@**
    /// fin: (no comment)
    #[strum(serialize = "Foc/kin")]
    Foc_SLASH_kin,
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/kis")]
    Foc_SLASH_kis,
    /// izh: **@CODE@**
    #[strum(serialize = "Foc/kä")]
    Foc_SLASH_kä,
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
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/nii")]
    Foc_SLASH_nii,
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
    /// smn: **@CODE@**
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
    /// smn: **@CODE@**
    #[strum(serialize = "Foc/sun")]
    Foc_SLASH_sun,
    /// sms: **@CODE@** -tâma
    #[strum(serialize = "Foc/tama")]
    Foc_SLASH_tama,
    /// tlh: (no comment)
    #[strum(serialize = "Foc/that")]
    Foc_SLASH_that,
    /// tlh: (no comment)
    #[strum(serialize = "Foc/this")]
    Foc_SLASH_this,
    /// sms: **@CODE@** -tõt
    #[strum(serialize = "Foc/tot")]
    Foc_SLASH_tot,
    /// smn: **@CODE@** ... focus particles.
    #[strum(serialize = "Foc/uv")]
    Foc_SLASH_uv,
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
    /// gle: (no comment)
    /// gle: **@CODE@** = words from other languages, mainly English, some Latin
    #[strum(serialize = "Foreign")]
    Foreign,
    /// mya: (no comment)
    #[strum(serialize = "Formal")]
    Formal,
    /// kpv: **@CODE@** = expressions such as аттьӧ, ало, ...
    /// myv: (no comment)
    /// koi: (no comment)
    /// mdf: **@CODE@**:  greetings,
    #[strum(serialize = "Formulaic")]
    Formulaic,
    /// khk: (no comment)
    #[strum(serialize = "Frm")]
    Frm,
    /// apu: **@CODE@** = ma: awa-pyty-ka-ma
    #[strum(serialize = "Frustr")]
    Frustr,
    /// gle: **@CODE@** = Future tense verbal particle
    /// cwd: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = Future participle
    /// lut: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// apu: **@CODE@** Future
    /// chr: (no comment)
    /// crj: (no comment)
    /// yrk: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// koi: **@CODE@**:  future      -ас
    /// pad: (no comment)
    /// eus: (no comment)
    /// kio: (no comment)
    /// khk: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Fut")]
    Fut,
    /// evn: (no comment)
    #[strum(serialize = "Fut1")]
    Fut1,
    /// evn: (no comment)
    #[strum(serialize = "Fut2")]
    Fut2,
    /// evn: (no comment)
    #[strum(serialize = "Fut3")]
    Fut3,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "FutImp")]
    FutImp,
    /// gle: **@CODE@** = Future Indicative verb
    #[strum(serialize = "FutInd")]
    FutInd,
    /// bxr: (no comment)
    /// mrj: **@CODE@** =
    #[strum(serialize = "FutPrc")]
    FutPrc,
    /// khk: (no comment)
    #[strum(serialize = "Fut_plan")]
    Fut_plan,
    /// som: (no comment)
    #[strum(serialize = "F→F")]
    F_F,
    /// som: (no comment)
    #[strum(serialize = "F→M")]
    F_M,
    /// smj: **@CODE@** Grade 2-3 for homonymies with grade 1-2, +N+G3
    /// sme: **@CODE@** Grade 2-3 for homonymies with grade 1-2, +N+G3
    #[strum(serialize = "G3")]
    G3,
    /// smj: **@CODE@** Grade 3, no consonant gradation, +N+G7
    /// sme: **@CODE@** Grade 3, no consonant gradation, +N+G7
    #[strum(serialize = "G7")]
    G7,
    /// kal: **@CODE@**
    #[strum(serialize = "GAJUP")]
    GAJUP,
    /// kal: **@CODE@**
    #[strum(serialize = "GAJUUQ")]
    GAJUUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "GALLAR")]
    GALLAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GALUAQ")]
    GALUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "GALUAR")]
    GALUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GALUTTUAQ")]
    GALUTTUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "GALUTTUAR")]
    GALUTTUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GASAAQ")]
    GASAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "GASAAR")]
    GASAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GASAP")]
    GASAP,
    /// kal: **@CODE@**
    #[strum(serialize = "GASUAR")]
    GASUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GASUGE")]
    GASUGE,
    /// kal: **@CODE@**
    #[strum(serialize = "GASURE")]
    GASURE,
    /// kal: **@CODE@**
    #[strum(serialize = "GE")]
    GE,
    /// kal: **@CODE@**
    #[strum(serialize = "GI")]
    GI,
    /// kal: **@CODE@**
    #[strum(serialize = "GIAQAR")]
    GIAQAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GIAR")]
    GIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GIARTUAAR")]
    GIARTUAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GIARTUR")]
    GIARTUR,
    /// kal: **@CODE@**
    #[strum(serialize = "GIIAAQ")]
    GIIAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "GIIAAR")]
    GIIAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GIIAAT")]
    GIIAAT,
    /// kal: **@CODE@**
    #[strum(serialize = "GIIP")]
    GIIP,
    /// kal: **@CODE@**
    #[strum(serialize = "GIIT")]
    GIIT,
    /// kal: **@CODE@**
    #[strum(serialize = "GIP")]
    GIP,
    /// kal: **@CODE@**
    #[strum(serialize = "GISSAAR")]
    GISSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUMA")]
    GUMA,
    /// kal: **@CODE@**
    #[strum(serialize = "GUMAAR")]
    GUMAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUMALLIR")]
    GUMALLIR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUMINAALLI")]
    GUMINAALLI,
    /// kal: **@CODE@**
    #[strum(serialize = "GUMINAR")]
    GUMINAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUNAR")]
    GUNAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUNNAIR")]
    GUNNAIR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUNNAR")]
    GUNNAR,
    /// kal: **@CODE@**
    #[strum(serialize = "GUP")]
    GUP,
    /// kal: **@CODE@**
    #[strum(serialize = "GUSUP")]
    GUSUP,
    /// kal: **@CODE@**
    /// iku: (no comment)
    #[strum(serialize = "GUUQ")]
    GUUQ,
    /// fit: **@CODE@** = Genitive
    /// sje: (no comment)
    /// gle: **@CODE@** = Genitive case
    /// kpv: **@CODE@** genitive case -лӧн асалан
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Genitive
    /// liv: **@CODE@** = genitive case
    /// mhr: **@CODE@** = genitive
    /// sma: **@CODE@** = Genitive
    /// nob: (no comment)
    /// smj: **@CODE@** = Genitive case
    /// udm: (no comment)
    /// rmy: (no comment)
    /// yrk: **@CODE@** (Genitive)
    /// mns: **@CODE@**
    /// sms: **@CODE@** Genitive
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = genitive case
    /// cho: (no comment)
    /// rmg: **@CODE@**  needed?
    /// som: (no comment)
    /// myv: (no comment)
    /// sqi: (no comment)
    /// olo: **@CODE@** genitive case
    /// koi: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// sme: **@CODE@** - Genitive
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  genitive
    /// rmf: **@CODE@** Genitive
    /// khk: (no comment)
    /// vro: **@CODE@** genitive
    /// mpj: **@CODE@**
    #[strum(serialize = "Gen")]
    Gen,
    /// rmy: (no comment)
    /// rmn: (no comment)
    /// rmf: **@CODE@**  name?
    #[strum(serialize = "Gen/Long")]
    Gen_SLASH_Long,
    /// sms: **@CODE@** Genitive attribute used as adjective?
    /// mdf: (no comment)
    #[strum(serialize = "GenAttr")]
    GenAttr,
    /// khk: (no comment)
    #[strum(serialize = "Gens")]
    Gens,
    /// kpv: **@CODE@**  Gerund This is used with derivations
    /// vot: (no comment)
    /// liv: **@CODE@** = gerund
    /// mhr: **@CODE@** = Gerund
    /// sjd: (no comment)
    /// sma: **@CODE@** | Gerundium
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** gerund +Ess, +Loc +Instr
    /// lit: (no comment)
    /// mrj: **@CODE@** = gerund
    /// vep: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** : Gerund
    /// koi: (no comment)
    /// eus: (no comment)
    /// sme: **@CODE@** Gerund
    /// mdf: (no comment)
    /// khk: (no comment)
    /// khk: (no comment)
    /// vro: **@CODE@** ollõn
    #[strum(serialize = "Ger")]
    Ger,
    /// yrk: **@CODE@**
    #[strum(serialize = "GerFin")]
    GerFin,
    /// sje: (no comment)
    #[strum(serialize = "GerI")]
    GerI,
    /// sje: (no comment)
    #[strum(serialize = "GerII")]
    GerII,
    /// udm: (no comment)
    #[strum(serialize = "GerNeg")]
    GerNeg,
    /// udm: (no comment)
    #[strum(serialize = "GerPos")]
    GerPos,
    /// udm: (no comment)
    #[strum(serialize = "GerTemp")]
    GerTemp,
    /// udm: (no comment)
    #[strum(serialize = "GerTer")]
    GerTer,
    /// khk: (no comment)
    #[strum(serialize = "Ger_Imp")]
    Ger_Imp,
    /// gle: **@CODE@** = General adverb
    #[strum(serialize = "Gn")]
    Gn,
    /// khk: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Gna")]
    Gna,
    /// khk: (no comment)
    #[strum(serialize = "Gna_Prf")]
    Gna_Prf,
    /// khk: (no comment)
    #[strum(serialize = "Gna_imp")]
    Gna_imp,
    /// myv: (no comment)
    #[strum(serialize = "Gr2xxx")]
    Gr2xxx,
    /// kal: (no comment)
    #[strum(serialize = "Gram/-assib")]
    Gram_SLASH__MINUS_assib,
    /// kal: **@CODE@** = ingen epentese ved låneord, så der kan genereres en. bestemt form til Katersat. batteriip versus batterip. ELLER ved vokaler aa+u -> aaju (nutaanngilaq)
    #[strum(serialize = "Gram/-epen")]
    Gram_SLASH__MINUS_epen,
    /// kal: **@CODE@** = minus schwa, angerlaanngilaa
    #[strum(serialize = "Gram/-schwa")]
    Gram_SLASH__MINUS_schwa,
    /// fit: (no comment)
    /// fkv: **@CODE@**: trisyllabic verbs
    /// smj: **@CODE@**:  trisyllabic verbs
    /// sme: **@CODE@**: trisyllabic verbs
    /// smn: **@CODE@**: trisyllabic verbs
    /// vro: **@CODE@**: trisyllabic verbs
    #[strum(serialize = "Gram/3syll")]
    Gram_SLASH_3syll,
    /// kal: **@CODE@** = Acronyms
    #[strum(serialize = "Gram/ACR")]
    Gram_SLASH_ACR,
    /// kal: **@CODE@** = Abbreviation
    #[strum(serialize = "Gram/Abbr")]
    Gram_SLASH_Abbr,
    /// kal: **@CODE@** = Adjektiver optaget direkte inkl. ubøjet anvendelse før kerne
    #[strum(serialize = "Gram/Adj")]
    Gram_SLASH_Adj,
    /// kal: **@CODE@** = Ablative alternative form -minngaanniit/-ninngaanniit. Eller uregelmæssig pujoq->puggup
    #[strum(serialize = "Gram/Alt")]
    Gram_SLASH_Alt,
    /// kal: **@CODE@** = Ablative alternative form 2 -minngarnit/-ninngarnit
    #[strum(serialize = "Gram/Alt2")]
    Gram_SLASH_Alt2,
    /// fit: (no comment)
    /// kal: **@CODE@** = Composita !Særlig tag til NIQ ajor/saper/artor/nalu- konstruktioner ved transitive verber
    /// sma: **@CODE@**
    /// smj: **@CODE@** = Inherent comp, lexicalized derivation
    /// sme: **@CODE@**   Comparative, adverbs
    /// vro: (no comment)
    #[strum(serialize = "Gram/Comp")]
    Gram_SLASH_Comp,
    /// kal: **@CODE@** = Various Pronoun, eg. tamaq, kisi (personkongruensdeklination)
    #[strum(serialize = "Gram/Cong")]
    Gram_SLASH_Cong,
    /// kal: **@CODE@** = Reflexive Pronoun
    #[strum(serialize = "Gram/Coref")]
    Gram_SLASH_Coref,
    /// kal: **@CODE@** = Double-transitive Verb. PL tilføjet 20180319
    #[strum(serialize = "Gram/Db")]
    Gram_SLASH_Db,
    /// kal: **@CODE@** = Demonstative pronoun, demonstrative adverb or demonstrative interjection
    #[strum(serialize = "Gram/Dem")]
    Gram_SLASH_Dem,
    /// smj: **@CODE@** = Inherent diminutive, lexicalized derivation
    #[strum(serialize = "Gram/Dimin")]
    Gram_SLASH_Dimin,
    /// fro: (no comment)
    #[strum(serialize = "Gram/Early")]
    Gram_SLASH_Early,
    /// kal: **@CODE@** = Exclamation !Flyttet fra primære tags 20180420 PL
    #[strum(serialize = "Gram/Exclm")]
    Gram_SLASH_Exclm,
    /// kal: **@CODE@** = TAQ allomorf i formen gaq, for at kunne generere en bestemt form til Katersat, opslag: maligaq -> malikkap, ikke malitap
    #[strum(serialize = "Gram/GAQ")]
    Gram_SLASH_GAQ,
    /// kal: **@CODE@** = gemineret
    #[strum(serialize = "Gram/GEM")]
    Gram_SLASH_GEM,
    /// kal: **@CODE@** = geminering af sidste konsonant i låneord
    #[strum(serialize = "Gram/GEMloan")]
    Gram_SLASH_GEMloan,
    /// kal: **@CODE@** = latent konsonant j gemineret (ergo GEM) som ss, tag brugt til at generere form til Katersat, kikiak -> kikissap (og ikke kikiap, kikiaap)
    #[strum(serialize = "Gram/GEMss")]
    Gram_SLASH_GEMss,
    /// kal: **@CODE@** = Non-marked agentive Verb (used as Half-transitive)
    #[strum(serialize = "Gram/HV")]
    Gram_SLASH_HV,
    /// kal: **@CODE@** = Følger hybriderne for at bestemme stederne hvor strengene kan klippes i to ord ved hybriderne
    #[strum(serialize = "Gram/Hyb")]
    Gram_SLASH_Hyb,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// sma: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// nob: **@CODE@**
    /// fao: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// smj: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// olo: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// sme: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// smn: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    /// fin: (no comment)
    /// vro: **@CODE@**:  Intransitive abbreviation (it takes no argument)
    #[strum(serialize = "Gram/IAbbr")]
    Gram_SLASH_IAbbr,
    /// kal: **@CODE@** = Intransitive Verb !PL 20180319 ny formalisme gennemført livm 20180324
    #[strum(serialize = "Gram/IV")]
    Gram_SLASH_IV,
    /// kal: (no comment)
    #[strum(serialize = "Gram/ImpNeg")]
    Gram_SLASH_ImpNeg,
    /// kal: (no comment)
    #[strum(serialize = "Gram/ImpPos")]
    Gram_SLASH_ImpPos,
    /// kal: **@CODE@** = Indefinitive Pronoun
    #[strum(serialize = "Gram/Indef")]
    Gram_SLASH_Indef,
    /// kal: **@CODE@** = Interrogative Pronoun
    #[strum(serialize = "Gram/Int")]
    Gram_SLASH_Int,
    /// fro: (no comment)
    #[strum(serialize = "Gram/Liason")]
    Gram_SLASH_Liason,
    /// kal: **@CODE@** = Ablative long form -miit/-niit
    #[strum(serialize = "Gram/Lo")]
    Gram_SLASH_Lo,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Intransitive abbreviations that are homonymous with more frequent words. They should only be considered abbreviations in the middle of a sentence.
    /// fao: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// smj: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// olo: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// sme: **@CODE@**:  Intransitive abbreviations that are homonymous
    /// smn: **@CODE@**:  Intransitive abbreviations that are homonymous   with more frequent words. They should only be considered  abbreviations in the middle of a sentence.
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
    /// fao: **@CODE@**:  Transitive abbreviations for which numerals
    /// smj: **@CODE@**:  Transitive abbreviations for which numerals
    /// olo: **@CODE@**:  Transitive abbreviations for which numerals
    /// sme: **@CODE@**:  Transitive abbreviations for which numerals
    /// smn: **@CODE@**:  Transitive abbreviations for which numerals are complements and normal words. The abbreviation usage  the sentence can be considered as true cases.
    /// fin: (no comment)
    /// vro: **@CODE@**:  Transitive abbreviations for which numerals
    #[strum(serialize = "Gram/NumNoAbbr")]
    Gram_SLASH_NumNoAbbr,
    /// kal: **@CODE@** = Ordinal number !pingajuat
    #[strum(serialize = "Gram/Ord")]
    Gram_SLASH_Ord,
    /// kal: **@CODE@** = Passive !used when a derivational morpheme not explicitely denotes a passivisation, or when intransitive flexion changes an active verb into a passive
    #[strum(serialize = "Gram/Pass")]
    Gram_SLASH_Pass,
    /// kal: **@CODE@** = Personal Pronoun
    #[strum(serialize = "Gram/Pers")]
    Gram_SLASH_Pers,
    /// fro: (no comment)
    #[strum(serialize = "Gram/Person")]
    Gram_SLASH_Person,
    /// kal: **@CODE@** = plurale tantum noun: ilaqutariit
    #[strum(serialize = "Gram/PlurTant")]
    Gram_SLASH_PlurTant,
    /// kal: **@CODE@** = Reciprocal !naapipput
    #[strum(serialize = "Gram/Reci")]
    Gram_SLASH_Reci,
    /// kal: **@CODE@** = Reflexive, reciprocal or passive !asavoq
    #[strum(serialize = "Gram/Refl")]
    Gram_SLASH_Refl,
    /// kal: **@CODE@** = Ablative short form -mit/-nit
    #[strum(serialize = "Gram/Sh")]
    Gram_SLASH_Sh,
    /// kal: **@CODE@** = singulare tantum noun: sila
    #[strum(serialize = "Gram/SingTant")]
    Gram_SLASH_SingTant,
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
    /// fao: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// smj: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// olo: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// sme: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// smn: **@CODE@**:  Transitive abbreviation (it needs an argument)
    /// fin: (no comment)
    /// vro: **@CODE@**:  Transitive abbreviation (it needs an argument)
    #[strum(serialize = "Gram/TAbbr")]
    Gram_SLASH_TAbbr,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Both transitive and intransitive abbreviation
    /// sma: **@CODE@**:  Both transitive and intransitive abbreviation
    /// fao: **@CODE@**:  Both transitive and intransitive abbreviation
    /// smj: **@CODE@**:  Both transitive and intransitive abbreviation
    /// olo: **@CODE@**:  Both transitive and intransitive abbreviation
    /// sme: **@CODE@**:  Both transitive and intransitive abbreviation
    /// smn: **@CODE@**:  Both transitive and intransitive abbreviation
    /// vro: **@CODE@**:  Both transitive and intransitive abbreviation
    #[strum(serialize = "Gram/TIAbbr")]
    Gram_SLASH_TIAbbr,
    /// fit: (no comment)
    /// fkv: **@CODE@**:  Transitive abbreviation if the following constituent is numeric
    /// sma: **@CODE@**:  Transitive abbreviation if the following
    /// nob: **@CODE@**
    /// fao: **@CODE@**:  Transitive abbreviation if the following
    /// smj: **@CODE@**:  Transitive abbreviation if the following
    /// rmn: (no comment)
    /// olo: **@CODE@**:  Transitive abbreviation if the following
    /// sme: **@CODE@**:  Transitive abbreviation if the following
    /// smn: **@CODE@**:  Transitive abbreviation if the following     constituent is numeric
    /// fin: (no comment)
    /// vro: **@CODE@**:  Transitive abbreviation if the following
    #[strum(serialize = "Gram/TNumAbbr")]
    Gram_SLASH_TNumAbbr,
    /// kal: **@CODE@** = Transitive Verb !PL 20180319 ny formalisme gennemført livm 20180324
    #[strum(serialize = "Gram/TV")]
    Gram_SLASH_TV,
    /// kal: (no comment)
    #[strum(serialize = "Gram/TransverbNIQ")]
    Gram_SLASH_TransverbNIQ,
    /// kal: **@CODE@** = den additive form af et suffiks, som både kan være additiv og truncativ, SIMA ved t-stammer
    #[strum(serialize = "Gram/additive")]
    Gram_SLASH_additive,
    /// kal: **@CODE@** = assibileret, for at kunne generere en bestemt form til Katersat
    #[strum(serialize = "Gram/assib")]
    Gram_SLASH_assib,
    /// kal: **@CODE@** = epentese i ved låneord, der ender på en konsonant. ELLER ved vokaler aa+u -> aa (nutaajunngilaq)
    #[strum(serialize = "Gram/epen")]
    Gram_SLASH_epen,
    /// kal: **@CODE@** = fusionerende, t+l -> s, iggit+LIR -> iggiser
    #[strum(serialize = "Gram/fus")]
    Gram_SLASH_fus,
    /// kal: **@CODE@** = inchoativt suffiks
    #[strum(serialize = "Gram/inchoative")]
    Gram_SLASH_inchoative,
    /// kal: (no comment)
    #[strum(serialize = "Gram/lex")]
    Gram_SLASH_lex,
    /// kal: **@CODE@** = trunkativ, stærk bøjning LIK
    #[strum(serialize = "Gram/liup")]
    Gram_SLASH_liup,
    /// kal: **@CODE@** = trunkativ, gemination LIK
    #[strum(serialize = "Gram/llip")]
    Gram_SLASH_llip,
    /// kal: **@CODE@** = final vokal + konsonant trunkering, stærk bøjning, geminering LIK
    #[strum(serialize = "Gram/llup")]
    Gram_SLASH_llup,
    /// kal: **@CODE@** = metatese, imeq, ermup
    #[strum(serialize = "Gram/meta")]
    Gram_SLASH_meta,
    /// smj: **@CODE@** = Inherent -r derivation. guollit-guollár
    #[strum(serialize = "Gram/r")]
    Gram_SLASH_r,
    /// kal: **@CODE@** = sidste vokal fjernet, dernæst regressiv assimilation, aaveq->aarrit
    #[strum(serialize = "Gram/regass")]
    Gram_SLASH_regass,
    /// kal: **@CODE@** = replaciv, for at kunne generere replaciv form til Katersat, aappinngilaa framfor aappalinngilaa, når opslaget er aapperpaa
    #[strum(serialize = "Gram/repl")]
    Gram_SLASH_repl,
    /// kal: **@CODE@** = med schwa, angerlaatinngilaa
    #[strum(serialize = "Gram/schwa")]
    Gram_SLASH_schwa,
    /// kal: **@CODE@** = stærk bøjning + trunkering, -up for at kunne vælge en bestemt form til Katersat, aagiak -> aagiaap, ikke aagiap
    #[strum(serialize = "Gram/str")]
    Gram_SLASH_str,
    /// kal: **@CODE@** = stærk bøjning, stammefinal k nasaleret
    #[strum(serialize = "Gram/strnasal")]
    Gram_SLASH_strnasal,
    /// kal: **@CODE@** = den truncative form af et suffiks, som både kan være additiv og truncativ, SIMA ved t-stammer
    #[strum(serialize = "Gram/truncative")]
    Gram_SLASH_truncative,
    /// kal: **@CODE@** = svag bøjning, -p
    #[strum(serialize = "Gram/wea")]
    Gram_SLASH_wea,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Group")]
    Group,
    /// mpj: **@CODE@** = group (different to GROUP as derivational nominal suffix)
    #[strum(serialize = "Grp")]
    Grp,
    /// gle: **@CODE@** = Morphological guesser
    /// kpv: **@CODE@**
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// izh: **@CODE@** Non-dictionary words via regex gring stems (not in use?)
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@** (not in use (?))
    /// xal: (no comment)
    /// sma: **@CODE@**:  for the name guesser ?
    /// lut: (no comment)
    /// sjt: (no comment)
    /// got: (no comment)
    /// hdn: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** Non-dictionary words (not in use)
    /// sms: **@CODE@**
    /// bgs: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** = Non-dictionary words (probably not in  use
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@**
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// sme: **@CODE@** for the name guesser
    /// gur: (no comment)
    /// aym: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** guess tag (not in use)
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Guess")]
    Guess,
    /// khk: (no comment)
    #[strum(serialize = "Guio")]
    Guio,
    /// fin: (no comment)
    #[strum(serialize = "Gyr")]
    Gyr,
    /// kal: **@CODE@**
    #[strum(serialize = "HTR")]
    HTR,
    /// fit: **@CODE@** = Hyphenation mark
    /// fkv: **@CODE@** = Hyphenation mark
    /// sms: **@CODE@**
    #[strum(serialize = "HYPH")]
    HYPH,
    /// chp: (no comment)
    /// evn: (no comment)
    /// lut: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// chr: (no comment)
    /// khk: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Hab")]
    Hab,
    /// bxr: (no comment)
    #[strum(serialize = "HabPrc")]
    HabPrc,
    /// eus: (no comment)
    #[strum(serialize = "Hip")]
    Hip,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  Homonymy
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = First pattern (let us say -ам)
    /// sma: **@CODE@**:  Homonymy
    /// smj: **@CODE@**:  Homonymy
    /// udm: (no comment)
    /// yrk: (no comment)
    /// mns: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** : Homonymy толам used in l element and lexc
    /// vep: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// smn: **@CODE@**:  Homonymy
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom1")]
    Hom1,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  Homonymy
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = Second pattern (let us say -ем)
    /// sma: **@CODE@**:  Homonymy
    /// smj: **@CODE@**:  Homonymy
    /// udm: (no comment)
    /// yrk: (no comment)
    /// mns: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** : Homonymy колем used in l element and lexc
    /// vep: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// smn: **@CODE@**:  Homonymy
    /// smn: **@CODE@** for different homonymous lexemes
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom2")]
    Hom2,
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = Third pattern (if it should exist + even more?)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom3")]
    Hom3,
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Hom4")]
    Hom4,
    /// mhr: **@CODE@** =
    /// fin: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Hom5")]
    Hom5,
    /// mhr: **@CODE@** =
    /// fin: (no comment)
    /// mdf: (no comment)
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
    /// pad: **@CODE@**	Hortative / Hortativo
    #[strum(serialize = "Hor")]
    Hor,
    /// yrk: **@CODE@**
    /// xak: (no comment)
    #[strum(serialize = "Hort")]
    Hort,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Hum")]
    Hum,
    /// mpj: **@CODE@**  = Hypothetical
    #[strum(serialize = "Hyp")]
    Hyp,
    /// apu: (no comment)
    #[strum(serialize = "Hypoth")]
    Hypoth,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "I")]
    I,
    /// crj: (no comment)
    #[strum(serialize = "IA")]
    IA,
    /// kal: **@CODE@**
    #[strum(serialize = "IAR")]
    IAR,
    /// tgl: (no comment)
    #[strum(serialize = "IF")]
    IF,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "II")]
    II,
    /// kal: **@CODE@**
    #[strum(serialize = "ILATSI")]
    ILATSI,
    /// kal: **@CODE@**
    #[strum(serialize = "ILIQI")]
    ILIQI,
    /// rmg: **@CODE@** infinitive marker
    /// non: (no comment)
    #[strum(serialize = "IM")]
    IM,
    /// crj: (no comment)
    #[strum(serialize = "IN")]
    IN,
    /// swe: (no comment)
    /// nob: **@CODE@** = ing-derivation
    #[strum(serialize = "ING")]
    ING,
    /// kal: **@CODE@**
    #[strum(serialize = "INNANNGUAQ")]
    INNANNGUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "INNAQ")]
    INNAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "INNAR")]
    INNAR,
    /// kal: **@CODE@**
    #[strum(serialize = "INNARIAA")]
    INNARIAA,
    /// kal: **@CODE@**
    #[strum(serialize = "IP")]
    IP,
    /// kal: **@CODE@**
    #[strum(serialize = "IR")]
    IR,
    /// kal: **@CODE@**
    #[strum(serialize = "IRNIAQ")]
    IRNIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "IRNIAR")]
    IRNIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "IRSIR")]
    IRSIR,
    /// kal: **@CODE@**
    #[strum(serialize = "IRUSSI")]
    IRUSSI,
    /// kal: **@CODE@**
    #[strum(serialize = "IRUTE")]
    IRUTE,
    /// kal: **@CODE@**
    #[strum(serialize = "ISUA")]
    ISUA,
    /// kal: **@CODE@**
    /// ess: (no comment)
    /// ipk: (no comment)
    #[strum(serialize = "IT")]
    IT,
    /// sje: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "ITRAB")]
    ITRAB,
    /// fit: **@CODE@** intransitive
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Intansitive verb
    /// liv: **@CODE@** = Intransitive verb
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@**:  Intansitive verb
    /// hdn: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Intransitive
    /// bxr: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = intransitive verbs
    /// som: (no comment)
    /// myv: (no comment)
    /// pad: **@CODE@**
    /// sme: **@CODE@** Intransitive Verb, +V+IV
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: (no comment)
    /// khk: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@**
    /// mpj: **@CODE@** Intransitive (i.e. with Abs)
    #[strum(serialize = "IV")]
    IV,
    /// nso: (no comment)
    #[strum(serialize = "IW")]
    IW,
    /// kpv: **@CODE@** These are ideophonic descriptors used to modify the verb --  вырк ливтясь "**flit** and it flew off"
    /// myv: (no comment)
    #[strum(serialize = "Ideoph")]
    Ideoph,
    /// gle: **@CODE@** = Indefinite quantifier/pronoun e.g. aon (any), cibé (whoever), ceachtar/neachtar (either/neither) etc.
    #[strum(serialize = "Idf")]
    Idf,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "Idl")]
    Idl,
    /// tgl: **@CODE@** - Indirect
    /// ceb: **@CODE@** - Indirect
    #[strum(serialize = "Idr")]
    Idr,
    /// khk: (no comment)
    #[strum(serialize = "Ifi")]
    Ifi,
    /// khk: (no comment)
    #[strum(serialize = "Ifi_Evid")]
    Ifi_Evid,
    /// fit: **@CODE@** = Illative
    /// sje: (no comment)
    /// gle: **@CODE@** = n/a
    /// kpv: **@CODE@** illative -ӧ пыран
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Illative
    /// liv: **@CODE@** = illative
    /// mhr: **@CODE@** = illative
    /// sma: **@CODE@**= Illative
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: **@CODE@** = Illative case
    /// udm: (no comment)
    /// skf: (no comment)
    /// apu: **@CODE@** Illative
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Illative
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**  = illative
    /// vep: (no comment)
    /// vep: (no comment)
    /// cho: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** illative
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Illative
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: **@CODE@**:  illative
    /// vro: **@CODE@** illative
    #[strum(serialize = "Ill")]
    Ill,
    /// skf: (no comment)
    #[strum(serialize = "ImFut")]
    ImFut,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// hdn: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Imm")]
    Imm,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "ImmPast")]
    ImmPast,
    /// apu: (no comment)
    #[strum(serialize = "Immin")]
    Immin,
    /// gle: **@CODE@** = Imperative particle (negative)
    /// kal: **@CODE@** = Imperative
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// krl: **@CODE@** Not previously coded +Imp, +Past 2024-05-16
    /// evn: (no comment)
    /// tgl: **@CODE@** - Imperative
    /// epo: (no comment)
    /// crj: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmg: **@CODE@** (fix Imp / Imprt)
    /// myv: (no comment)
    /// xak: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// mdf: (no comment)
    /// ceb: **@CODE@** -
    /// khk: (no comment)
    #[strum(serialize = "Imp")]
    Imp,
    /// gle: **@CODE@** = Imperative mood
    /// som: (no comment)
    #[strum(serialize = "Imper")]
    Imper,
    /// mya: (no comment)
    #[strum(serialize = "Imperative")]
    Imperative,
    /// mpj: **@CODE@**
    #[strum(serialize = "Imperf")]
    Imperf,
    /// cor: (no comment)
    /// som: (no comment)
    #[strum(serialize = "Impers")]
    Impers,
    /// rue: (no comment)
    /// apu: (no comment)
    /// rus: (no comment)
    /// tku: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Impf")]
    Impf,
    /// bxr: (no comment)
    #[strum(serialize = "ImpfPrc")]
    ImpfPrc,
    /// mhr: **@CODE@** = Imperfective (?) -- XXX check this
    /// skf: (no comment)
    #[strum(serialize = "Imprf")]
    Imprf,
    /// chp: (no comment)
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Imprs")]
    Imprs,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Imperative
    /// liv: **@CODE@** = imperative
    /// mhr: **@CODE@** = imperative
    /// sma: **@CODE@** | Imperative
    /// lut: (no comment)
    /// got: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Imperative mood
    /// lit: (no comment)
    /// vep: **@CODE@**  = imperative
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** Imperative
    /// koi: **@CODE@**:  imperative
    /// sme: **@CODE@** Imperative
    /// fin: (no comment)
    /// mdf: **@CODE@**:  imperative
    /// tlh: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Imprt")]
    Imprt,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Imperative, for Neg:  ollem ollh ...
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =  ыма
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// sme: **@CODE@** Alternative, Rather Declamatory Imperative Form - Infrequent not in use
    /// vro: **@CODE@**
    #[strum(serialize = "ImprtII")]
    ImprtII,
    /// hdn: (no comment)
    #[strum(serialize = "Impv")]
    Impv,
    /// qya: (no comment)
    #[strum(serialize = "In")]
    In,
    /// rue: (no comment)
    /// qya: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Inan")]
    Inan,
    /// chp: (no comment)
    #[strum(serialize = "Inc")]
    Inc,
    /// hdn: (no comment)
    #[strum(serialize = "Incep")]
    Incep,
    /// chp: (no comment)
    /// som: (no comment)
    /// mpj: **@CODE@** NB from the reference book, inchoative is used as verbalisation
    #[strum(serialize = "Inch")]
    Inch,
    /// cwd: (no comment)
    /// pma: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// som: (no comment)
    /// tkl: (no comment)
    /// xak: (no comment)
    /// kio: (no comment)
    /// mpj: **@CODE@**
    #[strum(serialize = "Incl")]
    Incl,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kmr: (no comment)
    /// kal: **@CODE@** = Indicative
    /// kpv: **@CODE@** tense
    /// pur: (no comment)
    /// vot: (no comment)
    /// bla: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Indicative
    /// izh: **@CODE@**  moods
    /// liv: **@CODE@** = indicative
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = indicative
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// evn: (no comment)
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** | Indicative
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: **@CODE@** - Indicative
    /// got: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** Verb moods
    /// sms: **@CODE@** Indicative mood
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =  Verb moods
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = indicative
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**  hmm, no Ind marked...
    /// ess: (no comment)
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// zxx: (no comment)
    /// grn: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Indicative
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: **@CODE@**:  indicative
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: **@CODE@** Indicative / Indicativo
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Indicative
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  indicative
    /// ale: (no comment)
    /// rmf: **@CODE@** Verb tense andmood
    /// ceb: **@CODE@** - Indicative
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// ces: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// ipk: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// iku: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Ind")]
    Ind,
    /// fit: **@CODE@** = Indefinitue
    /// sje: (no comment)
    /// kpv: **@CODE@**:  indefinite
    /// swe: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Indefinitue
    /// liv: **@CODE@** = indefinite
    /// crk: (no comment)
    /// mhr: **@CODE@** = Indefinite pronoun
    /// rue: (no comment)
    /// sma: **@CODE@** = Indefinite
    /// nob: **@CODE@** =
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// smj: **@CODE@** = indefinite pronoun
    /// udm: (no comment)
    /// epo: (no comment)
    /// apu: **@CODE@** Indefinitive Pronoun
    /// chr: (no comment)
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Indefinitive Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  indefinite
    /// vep: **@CODE@**  = indefinite
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// sqi: (no comment)
    /// olo: **@CODE@** indefinite
    /// koi: **@CODE@**:  indefinite
    /// sme: **@CODE@** - Indefinitive Pronoun
    /// smn: **@CODE@** - Indefinitive Pronoun
    /// fin: (no comment)
    /// mdf: **@CODE@**:  indefinite
    /// vro: **@CODE@**
    #[strum(serialize = "Indef")]
    Indef,
    /// mhr: **@CODE@** = forms огым, огыт, ите
    #[strum(serialize = "Indep")]
    Indep,
    /// epo: (no comment)
    #[strum(serialize = "Indic")]
    Indic,
    /// hdn: (no comment)
    #[strum(serialize = "Indir")]
    Indir,
    /// gle: **@CODE@** = Indirect relative particle
    #[strum(serialize = "Indirect")]
    Indirect,
    /// epo: (no comment)
    #[strum(serialize = "Indiv")]
    Indiv,
    /// fit: **@CODE@** = Inessive
    /// sje: (no comment)
    /// kpv: **@CODE@** inessive -ын ина
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Inessive
    /// liv: **@CODE@** = inessive
    /// mhr: **@CODE@** = inessive
    /// krl: (no comment)
    /// sma: **@CODE@** = Inesive
    /// fao: (no comment)
    /// smj: **@CODE@** = Inesive case
    /// udm: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = inessive
    /// vep: (no comment)
    /// vep: (no comment)
    /// cho: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** inessive
    /// koi: (no comment)
    /// deu: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  inessive
    /// vro: **@CODE@** inessive
    /// bak: (no comment)
    #[strum(serialize = "Ine")]
    Ine,
    /// xak: (no comment)
    #[strum(serialize = "Iness")]
    Iness,
    /// fit: **@CODE@** = tA Infinitive
    /// sje: (no comment)
    /// gle: **@CODE@** = Infinitival particle
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Infinitive
    /// izh: **@CODE@**
    /// liv: **@CODE@** = infinitive
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** = Infinitive
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** | Infinitive and participles
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: **@CODE@** - Infinitive
    /// nob: **@CODE@** = for infinite verbs
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// mns: **@CODE@** infinite verbforms
    /// sms: **@CODE@**
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** = infinitive
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** : Infinitive
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Infinitive
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** From Sámi, probably delete several of these.
    /// ceb: **@CODE@** -
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// ces: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@** sõimadaq, elädäq
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Inf")]
    Inf,
    /// vro: **@CODE@** sõimama, elämä
    #[strum(serialize = "Inf/mA")]
    Inf_SLASH_mA,
    /// deu: (no comment)
    #[strum(serialize = "Inf00")]
    Inf00,
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
    /// mya: (no comment)
    #[strum(serialize = "Informal")]
    Informal,
    /// fit: **@CODE@** = Instructive
    /// kal: **@CODE@** = Instrumentalis
    /// kpv: **@CODE@** instrumental -ӧн керанторъя
    /// fkv: **@CODE@** = Instructive
    /// liv: **@CODE@** = instrumental -KÕKS
    /// evn: (no comment)
    /// tgl: (no comment)
    /// udm: (no comment)
    /// rmy: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@** тыл
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = instrumental
    /// myv: (no comment)
    /// grn: (no comment)
    /// olo: **@CODE@** instrumental
    /// koi: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// rmf: **@CODE@**  Instrumentaali
    /// ceb: (no comment)
    /// khk: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "Ins")]
    Ins,
    /// lit: (no comment)
    #[strum(serialize = "Inst")]
    Inst,
    /// kpv: **@CODE@**
    /// liv: **@CODE@** = instructive -IN
    /// mhr: **@CODE@** =
    /// mns: **@CODE@**
    /// sms: **@CODE@** -eeʹl
    /// vep: **@CODE@**  = instructive -IN
    /// koi: (no comment)
    #[strum(serialize = "Instr")]
    Instr,
    /// gle: **@CODE@** = Sentence internal punctuation
    /// kal: **@CODE@** = Interrogative
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// mpj: **@CODE@**  = Intentive
    #[strum(serialize = "Int")]
    Int,
    /// kpv: **@CODE@**
    /// nob: **@CODE@** = **hmm, what is this...**
    /// apu: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Intens")]
    Intens,
    /// myv: (no comment)
    #[strum(serialize = "Intensifier")]
    Intensifier,
    /// mrj: **@CODE@** =
    /// myv: **@CODE@**:  intensive pronoun
    /// myv: (no comment)
    /// mdf: **@CODE@**:  intensive pronoun
    #[strum(serialize = "Intensive")]
    Intensive,
    /// fit: **@CODE@** = Interjection
    /// sje: (no comment)
    /// kal: **@CODE@** = Interjection
    /// kpv: **@CODE@**:  interjection   междометтьӧ   междометие
    /// swe: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Interjection
    /// izh: (no comment)
    /// liv: **@CODE@** = interjection
    /// liv: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = interjections
    /// mhr: (no comment)
    /// mhr: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Interjection
    /// hdn: (no comment)
    /// smj: **@CODE@** = Interjection
    /// udm: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// apu: **@CODE@** Interjection
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Interjection
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = interjection
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  interjection
    /// koi: **@CODE@**:  interjection
    /// deu: (no comment)
    /// sme: **@CODE@** - Interjection
    /// smn: **@CODE@**
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  interjection
    /// khk: (no comment)
    /// vro: **@CODE@** Interjections
    /// vro: (no comment)
    /// mpj: **@CODE@**
    #[strum(serialize = "Interj")]
    Interj,
    /// fit: **@CODE@** = Interrogative
    /// sje: (no comment)
    /// kpv: **@CODE@**:  interrogative
    /// vot: (no comment)
    /// bla: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Interrogative
    /// liv: **@CODE@** = interrogative
    /// crk: (no comment)
    /// mhr: **@CODE@** = Interrogative pronoun
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Interrogative
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// smj: **@CODE@** = Interrogative pronoun
    /// udm: (no comment)
    /// apu: **@CODE@** Interrogative Pronoun
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Interrogative Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  interrogative
    /// vep: **@CODE@**  = interrogative
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** interrogative
    /// koi: **@CODE@**:  interrogative
    /// deu: (no comment)
    /// sme: **@CODE@** - Interrogative Pronoun
    /// smn: **@CODE@** - Interrogative Pronoun
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  interrogative
    /// fro: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Interr")]
    Interr,
    /// som: (no comment)
    #[strum(serialize = "Interr/ma")]
    Interr_SLASH_ma,
    /// fro: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "Intj")]
    Intj,
    /// apu: (no comment)
    #[strum(serialize = "Intr")]
    Intr,
    /// hdn: (no comment)
    #[strum(serialize = "Ints")]
    Ints,
    /// bla: (no comment)
    #[strum(serialize = "Invs")]
    Invs,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// otw: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Ipc")]
    Ipc,
    /// nio: (no comment)
    #[strum(serialize = "Ipf")]
    Ipf,
    /// xak: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Ipfv")]
    Ipfv,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "Ipl")]
    Ipl,
    /// bla: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// tku: (no comment)
    /// cho: (no comment)
    /// grn: (no comment)
    /// mpj: **@CODE@**  = Irrealis
    #[strum(serialize = "Irr")]
    Irr,
    /// mya: (no comment)
    #[strum(serialize = "Irrealis")]
    Irrealis,
    /// kal: **@CODE@** = Iterative (morphologically: Causative)
    #[strum(serialize = "IteCau")]
    IteCau,
    /// kpv: **@CODE@**  Iterative form expressing number of times
    /// kpv: (no comment)
    /// evn: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** Iterative form expressing number of times; myv: `кавксть`, kpv: `кыкысь`
    /// koi: (no comment)
    /// koi: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Iter")]
    Iter,
    /// khk: (no comment)
    #[strum(serialize = "Itg")]
    Itg,
    /// gle: **@CODE@** = Interjection
    #[strum(serialize = "Itj")]
    Itj,
    /// gle: **@CODE@** = Intensifier of adjective e.g. sách, ró- etc.
    #[strum(serialize = "Its")]
    Its,
    /// kal: **@CODE@**
    #[strum(serialize = "JA")]
    JA,
    /// kal: **@CODE@**
    #[strum(serialize = "JAALLU")]
    JAALLU,
    /// kal: **@CODE@**
    #[strum(serialize = "JAAR")]
    JAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "JURAR")]
    JURAR,
    /// kal: **@CODE@**
    #[strum(serialize = "JURTUR")]
    JURTUR,
    /// liv: **@CODE@** = jussitive
    /// vro: **@CODE@**
    #[strum(serialize = "Jus")]
    Jus,
    /// kal: **@CODE@**
    #[strum(serialize = "KAA")]
    KAA,
    /// kal: **@CODE@**
    #[strum(serialize = "KAJAAQ")]
    KAJAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KALAAR")]
    KALAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KANNIQ")]
    KANNIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KANNIR")]
    KANNIR,
    /// kal: **@CODE@**
    #[strum(serialize = "KAR")]
    KAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KASIK")]
    KASIK,
    /// kal: **@CODE@**
    #[strum(serialize = "KASIP")]
    KASIP,
    /// kal: **@CODE@**
    #[strum(serialize = "KASSAK")]
    KASSAK,
    /// kal: **@CODE@**
    #[strum(serialize = "KASSAP")]
    KASSAP,
    /// kal: **@CODE@**
    #[strum(serialize = "KATAAR")]
    KATAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KATAP")]
    KATAP,
    /// kal: **@CODE@**
    #[strum(serialize = "KATTAR")]
    KATTAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KILLI")]
    KILLI,
    /// kal: **@CODE@**
    #[strum(serialize = "KIP")]
    KIP,
    /// kal: **@CODE@**
    #[strum(serialize = "KISAR")]
    KISAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KKAAQ")]
    KKAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KKAAR")]
    KKAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KKULUK")]
    KKULUK,
    /// kal: **@CODE@**
    #[strum(serialize = "KKULUP")]
    KKULUP,
    /// kal: **@CODE@**
    #[strum(serialize = "KKUMINAR")]
    KKUMINAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KKUT")]
    KKUT,
    /// kal: **@CODE@**
    #[strum(serialize = "KKUUTAAQ")]
    KKUUTAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KKUUTAAR")]
    KKUUTAAR,
    /// kal: (no comment)
    #[strum(serialize = "KKUUTAAT")]
    KKUUTAAT,
    /// kal: **@CODE@**
    #[strum(serialize = "KUJUK")]
    KUJUK,
    /// kal: **@CODE@**
    #[strum(serialize = "KUJUP")]
    KUJUP,
    /// kal: **@CODE@**
    #[strum(serialize = "KUJUUQ")]
    KUJUUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KUJUUR")]
    KUJUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "KULA")]
    KULA,
    /// kal: **@CODE@**
    #[strum(serialize = "KULAAR")]
    KULAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "KULLAK")]
    KULLAK,
    /// kal: **@CODE@**
    #[strum(serialize = "KULUK")]
    KULUK,
    /// kal: **@CODE@**
    #[strum(serialize = "KULUP")]
    KULUP,
    /// kal: **@CODE@**
    #[strum(serialize = "KULUUQ")]
    KULUUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KULUUR")]
    KULUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "KUQ")]
    KUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "KUSUUR")]
    KUSUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "KUTSUUR")]
    KUTSUUR,
    /// rmf: **@CODE@**  name?
    #[strum(serialize = "Kiire")]
    Kiire,
    /// rmf: **@CODE@**  name?
    #[strum(serialize = "Kiiri")]
    Kiiri,
    /// rmf: **@CODE@**  name?
    #[strum(serialize = "Kiiro")]
    Kiiro,
    /// sms: **@CODE@** Kin term This may need to be removed, let Sem/Hum-kin suffice
    #[strum(serialize = "Kin")]
    Kin,
    /// sme: **@CODE@** - man (different from maid): mii+Pron+Rel+Sg+Acc+Known
    /// smn: **@CODE@** mon , till we found a better tag
    #[strum(serialize = "Known")]
    Known,
    /// epo: (no comment)
    #[strum(serialize = "Kor")]
    Kor,
    /// kal: **@CODE@**
    #[strum(serialize = "LA")]
    LA,
    /// kal: **@CODE@**
    #[strum(serialize = "LAAQ")]
    LAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LAAR")]
    LAAR,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// gle: **@CODE@** = Left side of parwise symbol (parenthesis or quotation mark)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = left parenth
    /// liv: **@CODE@**
    /// crk: (no comment)
    /// mhr: **@CODE@** = paired symbols
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@**:  XXX These should be documented better
    /// tgl: (no comment)
    /// hdn: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = left paranthesis
    /// bxr: (no comment)
    /// vep: **@CODE@**
    /// cho: (no comment)
    /// som: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@**
    /// xak: (no comment)
    /// sme: **@CODE@**  left paranthesis
    /// srs: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// rmf: **@CODE@** paired symbols, parentheses
    /// ceb: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "LEFT")]
    LEFT,
    /// tgl: (no comment)
    #[strum(serialize = "LF")]
    LF,
    /// eus: (no comment)
    #[strum(serialize = "LGen")]
    LGen,
    /// kal: **@CODE@**
    #[strum(serialize = "LI")]
    LI,
    /// kal: **@CODE@**
    #[strum(serialize = "LIAQ")]
    LIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LIAR")]
    LIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIK")]
    LIK,
    /// kal: **@CODE@**
    #[strum(serialize = "LIKKAAQ")]
    LIKKAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LIKKIRSAAR")]
    LIKKIRSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIP")]
    LIP,
    /// kal: **@CODE@**
    #[strum(serialize = "LIR")]
    LIR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIRI")]
    LIRI,
    /// kal: **@CODE@**
    #[strum(serialize = "LIRNGUSAATE")]
    LIRNGUSAATE,
    /// kal: **@CODE@**
    #[strum(serialize = "LIRSAAR")]
    LIRSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIRSUR")]
    LIRSUR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIRTUR")]
    LIRTUR,
    /// kal: **@CODE@**
    #[strum(serialize = "LISAAR")]
    LISAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LISAR")]
    LISAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIUR")]
    LIUR,
    /// kal: **@CODE@**
    #[strum(serialize = "LIUTE")]
    LIUTE,
    /// kal: **@CODE@**
    #[strum(serialize = "LIVIK")]
    LIVIK,
    /// kal: **@CODE@**
    #[strum(serialize = "LLAARE")]
    LLAARE,
    /// kal: **@CODE@**
    #[strum(serialize = "LLAATAA")]
    LLAATAA,
    /// kal: **@CODE@**
    #[strum(serialize = "LLAMMAK")]
    LLAMMAK,
    /// kal: **@CODE@**
    #[strum(serialize = "LLAP")]
    LLAP,
    /// kal: **@CODE@**
    #[strum(serialize = "LLAQQIP")]
    LLAQQIP,
    /// kal: **@CODE@**
    #[strum(serialize = "LLAR")]
    LLAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LLARIP")]
    LLARIP,
    /// kal: **@CODE@**
    #[strum(serialize = "LLATSIAR")]
    LLATSIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LLATTAA")]
    LLATTAA,
    /// kal: **@CODE@**
    #[strum(serialize = "LLATTAAR")]
    LLATTAAR,
    /// ess: (no comment)
    /// ipk: (no comment)
    #[strum(serialize = "LLATU")]
    LLATU,
    /// kal: **@CODE@**
    #[strum(serialize = "LLATUAR")]
    LLATUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LLI")]
    LLI,
    /// kal: **@CODE@**
    #[strum(serialize = "LLIQ")]
    LLIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LLIR")]
    LLIR,
    /// kal: **@CODE@**
    #[strum(serialize = "LLIRPAAQ")]
    LLIRPAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LLIUR")]
    LLIUR,
    /// kal: **@CODE@**
    #[strum(serialize = "LLUAR")]
    LLUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "LLUINNAQ")]
    LLUINNAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LLUINNAQQISSAAQ")]
    LLUINNAQQISSAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LLUINNAR")]
    LLUINNAR,
    /// sma: **@CODE@**:  ad hoc tag for development purposes ?
    #[strum(serialize = "LOAN")]
    LOAN,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "LOC")]
    LOC,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "LPAR")]
    LPAR,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "LQUOT")]
    LQUOT,
    /// kal: **@CODE@**
    /// ess: (no comment)
    /// ipk: (no comment)
    /// iku: (no comment)
    #[strum(serialize = "LU")]
    LU,
    /// kal: **@CODE@**
    #[strum(serialize = "LUP")]
    LUP,
    /// kal: **@CODE@**
    #[strum(serialize = "LUQA")]
    LUQA,
    /// kal: **@CODE@**
    #[strum(serialize = "LUSUUQ")]
    LUSUUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "LUUNNIIT")]
    LUUNNIIT,
    /// kal: **@CODE@**
    #[strum(serialize = "LUUR")]
    LUUR,
    /// sms: **@CODE@** largo variant instead of allegro saaǥǥstõõllâd vs saǥstõõllâd
    #[strum(serialize = "Largo")]
    Largo,
    /// fkv: **@CODE@** = lative (the infinitive, used in Apertium)
    /// liv: **@CODE@** = Lative
    /// mhr: **@CODE@** = lative
    /// mns: **@CODE@** Ын (2007: Дательный падеж)
    /// sms: **@CODE@** Lative lääinas, säämas, toimmpââjas
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = Lative
    /// vep: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** Lative
    /// fin: (no comment)
    /// mdf: **@CODE@**:  lative
    #[strum(serialize = "Lat")]
    Lat,
    /// gle: **@CODE@** = Latin language words
    #[strum(serialize = "Latin")]
    Latin,
    /// gle: **@CODE@** = Lenited forms
    #[strum(serialize = "Len")]
    Len,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Leng")]
    Leng,
    /// tgl: **@CODE@** -  Ligature
    /// tgl: (no comment)
    /// ceb: **@CODE@** -  Ligature
    /// ceb: (no comment)
    #[strum(serialize = "Lig")]
    Lig,
    /// yrk: **@CODE@** limitative
    #[strum(serialize = "Lim")]
    Lim,
    /// gle: **@CODE@** = Locative adverb
    /// kpv: **@CODE@** LocMod, IneMod Быд во шедӧдӧны бур успеваемость Воркута да Инта каръясса, Прилузскӧй да Княжпогостскӧй районъясса школаяс.
    /// vot: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: (no comment)
    /// tgl: (no comment)
    /// smj: **@CODE@** = Locative case
    /// oji: (no comment)
    /// otw: (no comment)
    /// skf: (no comment)
    /// apu: **@CODE@** Locative -ã Locative (at, to), instrumental
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@** Ыт
    /// sms: **@CODE@** Locative
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = Locative
    /// myv: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Locative
    /// xak: (no comment)
    /// sme: **@CODE@** - Locative = Inessive and Ellative
    /// smn: **@CODE@**
    /// kio: (no comment)
    /// mdf: **@CODE@**:  locative
    /// ceb: (no comment)
    /// khk: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@** locative
    /// mpj: **@CODE@**
    #[strum(serialize = "Loc")]
    Loc,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Loc2")]
    Loc2,
    /// ciw: (no comment)
    #[strum(serialize = "LocDist")]
    LocDist,
    /// kpv: **@CODE@** move to Loc
    #[strum(serialize = "LocMod")]
    LocMod,
    /// mhr: **@CODE@** = location, better witho LocusPl to avoid Loc case?
    #[strum(serialize = "LocPl")]
    LocPl,
    /// eus: (no comment)
    #[strum(serialize = "Locp")]
    Locp,
    /// sma: **@CODE@** - ?
    #[strum(serialize = "Logo")]
    Logo,
    /// kal: **@CODE@** = Locative
    #[strum(serialize = "Lok")]
    Lok,
    /// hdn: (no comment)
    /// smj: **@CODE@**  Long form
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Long")]
    Long,
    /// khk: (no comment)
    #[strum(serialize = "Lpar")]
    Lpar,
    /// khk: (no comment)
    #[strum(serialize = "Lquot")]
    Lquot,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Lxc")]
    Lxc,
    /// kal: **@CODE@**
    #[strum(serialize = "MAA")]
    MAA,
    /// kal: **@CODE@**
    #[strum(serialize = "MAANNA")]
    MAANNA,
    /// kal: **@CODE@**
    #[strum(serialize = "MAANNAA")]
    MAANNAA,
    /// kal: **@CODE@**
    #[strum(serialize = "MAAR")]
    MAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "MASSA")]
    MASSA,
    /// kpv: **@CODE@** abessive modifier -тӧм
    #[strum(serialize = "MAbe")]
    MAbe,
    /// khk: (no comment)
    #[strum(serialize = "MF")]
    MF,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "MFN")]
    MFN,
    /// kpv: **@CODE@** habeo modifier а -
    #[strum(serialize = "MHab")]
    MHab,
    /// kal: **@CODE@**
    #[strum(serialize = "MI")]
    MI,
    /// gle: **@CODE@** = Middle punctuation
    /// sme: **@CODE@**  in-word punctuation, typically hyphen, used to indicate a measurement span of some sort
    /// srs: (no comment)
    #[strum(serialize = "MIDDLE")]
    MIDDLE,
    /// kal: **@CODE@**
    #[strum(serialize = "MINAATSIAQ")]
    MINAATSIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "MINIQ")]
    MINIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "MISAAR")]
    MISAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "MIU")]
    MIU,
    /// kal: **@CODE@**
    #[strum(serialize = "MIUQ")]
    MIUQ,
    /// kpv: **@CODE@** locative modifier са -
    #[strum(serialize = "MLoc")]
    MLoc,
    /// kal: **@CODE@**
    #[strum(serialize = "MMIRSUR")]
    MMIRSUR,
    /// bla: (no comment)
    #[strum(serialize = "MT")]
    MT,
    /// kpv: **@CODE@** temporal modifier ся -
    #[strum(serialize = "MTmp")]
    MTmp,
    /// fit: **@CODE@** =
    /// gle: **@CODE@** = Multi word expression
    /// swe: (no comment)
    /// fkv: **@CODE@** = multiword expression, for tokenisation
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// hdn: (no comment)
    /// fao: (no comment)
    /// smj: **@CODE@**  multi word expressions, goes to abbr
    /// sms: **@CODE@** never last element of compound words
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** (both?)
    /// deu: (no comment)
    /// sme: **@CODE@** - Multi-word expressions treated as such in the preprocessor. To be added as first tag after the lemma
    /// smn: **@CODE@** - MultiWord Expression, used for abbreviation extraction for preprocess.sh
    /// fin: (no comment)
    /// mdf: **@CODE@**:  Multiword element
    /// bak: (no comment)
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
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// moh: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Mal")]
    Mal,
    /// mya: (no comment)
    #[strum(serialize = "Male")]
    Male,
    /// mya: (no comment)
    #[strum(serialize = "MaleSpeaker")]
    MaleSpeaker,
    /// epo: (no comment)
    #[strum(serialize = "Mann")]
    Mann,
    /// kpv: **@CODE@** with reference to type of adverb
    /// izh: **@CODE@** = Types of adverbs
    /// lut: (no comment)
    /// slh: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// vep: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// mdf: **@CODE@**:  with reference to type of adverb
    #[strum(serialize = "Manner")]
    Manner,
    /// gle: **@CODE@** = Masculine gender
    /// kmr: (no comment)
    /// som: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Masc")]
    Masc,
    /// som: (no comment)
    #[strum(serialize = "Mass")]
    Mass,
    /// koi: **@CODE@**:  moderative -моз, -кодь  'kind of, -ish' with reference to quality ыджыткодь 'rather big'
    #[strum(serialize = "Mdr")]
    Mdr,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Measr")]
    Measr,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Med")]
    Med,
    /// eus: (no comment)
    #[strum(serialize = "Mgg")]
    Mgg,
    /// lut: (no comment)
    #[strum(serialize = "Mid")]
    Mid,
    /// non: (no comment)
    #[strum(serialize = "Middle")]
    Middle,
    /// chp: (no comment)
    /// deu: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Mod")]
    Mod,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Mod/Subj")]
    Mod_SLASH_Subj,
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
    /// khk: (no comment)
    #[strum(serialize = "Mod_Dss")]
    Mod_Dss,
    /// khk: (no comment)
    #[strum(serialize = "Mod_Foc")]
    Mod_Foc,
    /// chp: (no comment)
    #[strum(serialize = "Mom")]
    Mom,
    /// bla: (no comment)
    #[strum(serialize = "Movg")]
    Movg,
    /// lav: (no comment)
    /// rue: (no comment)
    /// amh: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@** = Gender. MF = Masc or Fem (used for adjs, not nouns)
    /// got: (no comment)
    /// fao: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// apu: **@CODE@**  ywa
    /// rus: (no comment)
    /// cor: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// ron: (no comment)
    /// sqi: (no comment)
    /// pad: **@CODE@** masculine nouns usually require a correlating demonstrative ada
    /// deu: (no comment)
    /// non: (no comment)
    /// mdf: (no comment)
    /// rmf: **@CODE@**
    /// khk: (no comment)
    /// ces: (no comment)
    #[strum(serialize = "Msc")]
    Msc,
    /// kpv: **@CODE@** multiplicative, i.e. iterations
    /// kpv: (no comment)
    /// myv: (no comment)
    /// koi: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Mult")]
    Mult,
    /// kpv: **@CODE@** Special multiword units are analysed with:
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// izh: **@CODE@** Non-dictionary words can be recognised with: Special multiword units
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// xal: (no comment)
    /// sma: **@CODE@**:  Multiword phrase tag ?
    /// lut: (no comment)
    /// sjt: (no comment)
    /// got: (no comment)
    /// hdn: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** multiword units
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** = !! multiword units
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@**
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** multiword tag
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Multi")]
    Multi,
    /// fro: (no comment)
    #[strum(serialize = "MultiWordConj")]
    MultiWordConj,
    /// som: (no comment)
    #[strum(serialize = "M→F")]
    M_F,
    /// som: (no comment)
    #[strum(serialize = "M→M")]
    M_M,
    /// fit: **@CODE@** = Noun
    /// sje: (no comment)
    /// gle: **@CODE@** = n/a (Noun is used) -- The +N tag is in use, TODO: change it
    /// kal: **@CODE@** = Noun
    /// kpv: **@CODE@**:  noun      эмакыв    - существительное
    /// swe: (no comment)
    /// pur: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Noun
    /// izh: **@CODE@**
    /// liv: **@CODE@** = noun
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = nouns
    /// mhr: (no comment)
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** = Noun
    /// lut: (no comment)
    /// sjt: (no comment)
    /// ara: (no comment)
    /// tgl: **@CODE@** -  Noun
    /// nob: **@CODE@** = Open parts of speech
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Noun
    /// udm: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// otw: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Noun
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Noun
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@**
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = noun
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// ess: (no comment)
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// mya: (no comment)
    /// hin: (no comment)
    /// olo: **@CODE@**  noun
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: **@CODE@**:  noun
    /// xak: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Noun
    /// smn: **@CODE@**
    /// srs: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  noun
    /// ale: (no comment)
    /// rmf: **@CODE@** open POS
    /// ceb: **@CODE@** -  Noun
    /// khk: (no comment)
    /// khk: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// ces: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@** Noun
    /// kjh: (no comment)
    /// ipk: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// iku: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "N")]
    N,
    /// bla: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "NA")]
    NA,
    /// kal: **@CODE@**
    #[strum(serialize = "NAAR")]
    NAAR,
    /// bla: (no comment)
    #[strum(serialize = "NAD")]
    NAD,
    /// kal: **@CODE@**
    #[strum(serialize = "NAQ")]
    NAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NAR")]
    NAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NASUGE")]
    NASUGE,
    /// kal: **@CODE@**
    #[strum(serialize = "NASURE")]
    NASURE,
    /// kal: **@CODE@**
    #[strum(serialize = "NAVIAR")]
    NAVIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NAVIIR")]
    NAVIIR,
    /// kal: **@CODE@**
    #[strum(serialize = "NAVIIRSAAR")]
    NAVIIRSAAR,
    /// gle: **@CODE@** = Named Entity Recognition
    #[strum(serialize = "NER")]
    NER,
    /// apu: **@CODE@** Non-Future
    #[strum(serialize = "NFut")]
    NFut,
    /// gle: **@CODE@** = Don't generate non-standard form
    #[strum(serialize = "NG")]
    NG,
    /// kal: **@CODE@**
    #[strum(serialize = "NGA")]
    NGA,
    /// kal: **@CODE@**
    #[strum(serialize = "NGAAQ")]
    NGAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NGAAR")]
    NGAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NGAATSIAQ")]
    NGAATSIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NGAJAK")]
    NGAJAK,
    /// kal: **@CODE@**
    #[strum(serialize = "NGAJAP")]
    NGAJAP,
    /// kal: **@CODE@**
    #[strum(serialize = "NGAJASSAA")]
    NGAJASSAA,
    /// kal: **@CODE@**
    #[strum(serialize = "NGIR")]
    NGIR,
    /// cho: (no comment)
    #[strum(serialize = "NGrade")]
    NGrade,
    /// bla: (no comment)
    #[strum(serialize = "NI")]
    NI,
    /// kal: **@CODE@**
    #[strum(serialize = "NIAALUP")]
    NIAALUP,
    /// kal: **@CODE@**
    #[strum(serialize = "NIAQ")]
    NIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIAR")]
    NIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIARIUTAA")]
    NIARIUTAA,
    /// kal: **@CODE@**
    #[strum(serialize = "NIARSARE")]
    NIARSARE,
    /// bla: (no comment)
    #[strum(serialize = "NID")]
    NID,
    /// kal: **@CODE@**
    #[strum(serialize = "NIINNAR")]
    NIINNAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIKUQ")]
    NIKUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIKUU")]
    NIKUU,
    /// kal: **@CODE@**
    #[strum(serialize = "NIP")]
    NIP,
    /// kal: **@CODE@**
    #[strum(serialize = "NIQ")]
    NIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIQAR")]
    NIQAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIQ_AJUR")]
    NIQ_AJUR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIR")]
    NIR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRAR")]
    NIRAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRLIUR")]
    NIRLIUR,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRLUP")]
    NIRLUP,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRPAA")]
    NIRPAA,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRPAAQ")]
    NIRPAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRPAARPAA")]
    NIRPAARPAA,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRSAQ")]
    NIRSAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRU")]
    NIRU,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRUMAAQ")]
    NIRUMAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIRUMAAR")]
    NIRUMAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NISAQ")]
    NISAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NIUTAA")]
    NIUTAA,
    /// kal: **@CODE@**
    #[strum(serialize = "NIUTE")]
    NIUTE,
    /// kal: **@CODE@**
    #[strum(serialize = "NNAAQ")]
    NNAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NNAP")]
    NNAP,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGIR")]
    NNGIR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGIRSAAR")]
    NNGIRSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGISAANNAR")]
    NNGISAANNAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGIT")]
    NNGIT,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUALLAARE")]
    NNGUALLAARE,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUAQ")]
    NNGUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUAR")]
    NNGUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUARSI")]
    NNGUARSI,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUASIK")]
    NNGUASIK,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUATSIAR")]
    NNGUATSIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUJUUR")]
    NNGUJUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUR")]
    NNGUR,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUUJUIT")]
    NNGUUJUIT,
    /// kal: **@CODE@**
    #[strum(serialize = "NNGUUJUK")]
    NNGUUJUK,
    /// apu: **@CODE@** not possessed
    #[strum(serialize = "NPossd")]
    NPossd,
    /// gle: **@CODE@** = De-nominal verbal noun
    #[strum(serialize = "NStem")]
    NStem,
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
    /// hdn: (no comment)
    /// som: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Near")]
    Near,
    /// mhr: **@CODE@** = Necessive infinitive
    /// mdf: (no comment)
    #[strum(serialize = "Nec")]
    Nec,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// gle: **@CODE@** = Negative particle (n)
    /// chp: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Negation verb ei
    /// liv: **@CODE@** = verb of negation эзь, аволь, иля
    /// crk: (no comment)
    /// mhr: **@CODE@** = Negation verb
    /// mhr: **@CODE@** = Negative participle
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** | negation verb ij
    /// hdn: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// sms: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// cho: (no comment)
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// xak: (no comment)
    /// sme: **@CODE@** Negation Verb, Ii and its forms, ie Ale, Alli, Allot, Ehpet, Eat etc.
    /// fin: (no comment)
    /// mdf: (no comment)
    /// khk: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@** saa-aiq 3 elements in 1 orthographic unit
    #[strum(serialize = "Neg")]
    Neg,
    /// mdf: **@CODE@**:  negative conditional Офтяря- negative protasis
    #[strum(serialize = "NegCnd")]
    NegCnd,
    /// mdf: **@CODE@**:  negative conditional Офтяряль negative protasis
    #[strum(serialize = "NegCndSub")]
    NegCndSub,
    /// mrj: **@CODE@** =  negative passive participle ЫдЫмЫ
    /// fin: (no comment)
    #[strum(serialize = "NegPrc")]
    NegPrc,
    /// gle: **@CODE@** = Negative interrogative verbal particle(q)
    #[strum(serialize = "NegQ")]
    NegQ,
    /// rmn: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// sqi: (no comment)
    /// non: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Neu")]
    Neu,
    /// moh: (no comment)
    #[strum(serialize = "Neuter")]
    Neuter,
    /// xak: (no comment)
    #[strum(serialize = "Nfirsth")]
    Nfirsth,
    /// gle: **@CODE@** = Number particle (m)
    #[strum(serialize = "Nm")]
    Nm,
    /// skf: (no comment)
    #[strum(serialize = "Nmlz")]
    Nmlz,
    /// deu: (no comment)
    #[strum(serialize = "NoArt")]
    NoArt,
    /// myv: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "NoPoss")]
    NoPoss,
    /// sms: **@CODE@** No translation available
    #[strum(serialize = "NoTransl")]
    NoTransl,
    /// sma: **@CODE@**:  A tag to indicate the lack of realised or potential Umlaut
    #[strum(serialize = "NoUml")]
    NoUml,
    /// myv: (no comment)
    #[strum(serialize = "NoVowX")]
    NoVowX,
    /// hdn: (no comment)
    #[strum(serialize = "Nocturn")]
    Nocturn,
    /// fit: **@CODE@** = Nominative
    /// sje: (no comment)
    /// gle: **@CODE@** =
    /// kmr: (no comment)
    /// kal: **@CODE@** = Nominative
    /// kpv: **@CODE@** nominative case нимтан
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Nominative
    /// liv: **@CODE@** = nominative case
    /// mhr: **@CODE@** = nominative
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@** = Nominative
    /// fao: (no comment)
    /// smj: **@CODE@** = Nominative case
    /// udm: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// apu: **@CODE@** Nominative (morphological ZERO)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** Nominative = nominativ
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = nominative case
    /// cho: (no comment)
    /// rmg: **@CODE@**  needed?
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// grn: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@** nominative case
    /// koi: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Nominative
    /// smn: **@CODE@**
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  nominative
    /// rmf: **@CODE@** Nominative
    /// khk: (no comment)
    /// ces: (no comment)
    /// fro: (no comment)
    /// vro: **@CODE@** nominative
    /// tqn: (no comment)
    #[strum(serialize = "Nom")]
    Nom,
    /// kpv: **@CODE@**
    /// liv: **@CODE@**
    /// skf: **@CODE@** Actor Noun From Verb - Nomen Agentis, +N+NomAg
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "NomAct")]
    NomAct,
    /// sje: (no comment)
    /// liv: **@CODE@**
    /// smj: (no comment)
    /// skf: **@CODE@** Actor Noun From Verb - Nomen Agentis, +N+NomAg
    /// sms: **@CODE@** CHECK ME this is not the same as Der/NomAg, which is a derivation
    /// myv: (no comment)
    /// sme: **@CODE@** Actor Noun From Verb - Nomen Agentis, +N+NomAg
    /// smn: **@CODE@**
    /// mdf: (no comment)
    #[strum(serialize = "NomAg")]
    NomAg,
    /// skf: **@CODE@** Instrument Noun From Verb - , +N+NomIns
    #[strum(serialize = "NomIns")]
    NomIns,
    /// amh: (no comment)
    #[strum(serialize = "Nominal")]
    Nominal,
    /// mpj: **@CODE@**  Nominaliser
    #[strum(serialize = "Nomz")]
    Nomz,
    /// bla: (no comment)
    #[strum(serialize = "Non")]
    Non,
    /// hdn: (no comment)
    #[strum(serialize = "NonFact")]
    NonFact,
    /// kpv: **@CODE@** look at this and place somewhere
    #[strum(serialize = "NonHum")]
    NonHum,
    /// gle: **@CODE@** = Adj qualifies pl noun with non-slender ending
    #[strum(serialize = "NotSlen")]
    NotSlen,
    /// gle: **@CODE@** = Noun (common, proper, verbal, substantive)
    #[strum(serialize = "Noun")]
    Noun,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Now")]
    Now,
    /// som: (no comment)
    #[strum(serialize = "Null")]
    Null,
    /// fit: **@CODE@** = Numerals
    /// sje: (no comment)
    /// gle: **@CODE@** = Numeral
    /// kal: **@CODE@** = Numeral
    /// kpv: **@CODE@**:  numeral      лыдакыв   числительное
    /// swe: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Numerals
    /// liv: **@CODE@** = numeral
    /// crk: (no comment)
    /// mhr: **@CODE@** = numerals
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Numerals
    /// nob: **@CODE@**
    /// hdn: (no comment)
    /// smj: **@CODE@** = Numeral
    /// udm: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Numeral
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Numeral
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = numeral
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  numeral
    /// koi: **@CODE@**:  numeral
    /// deu: (no comment)
    /// sme: **@CODE@** - Numeral
    /// smn: **@CODE@**
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  numeral
    /// ceb: **@CODE@** - Numeral
    /// khk: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@** Numerals
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// kca: (no comment)
    /// kca: (no comment)
    #[strum(serialize = "Num")]
    Num,
    /// mdf: (no comment)
    #[strum(serialize = "Num→A")]
    Num_A,
    /// nob: **@CODE@**  For dictionary use., Nynorsk only
    #[strum(serialize = "Nynorsk")]
    Nynorsk,
    /// mhr: **@CODE@** =
    #[strum(serialize = "N→A")]
    N_A,
    /// mdf: **@CODE@** = Chuvash
    #[strum(serialize = "OLang/CHV")]
    OLang_SLASH_CHV,
    /// kal: **@CODE@** = Danish stem: profet
    #[strum(serialize = "OLang/DAN")]
    OLang_SLASH_DAN,
    /// gle: **@CODE@** = - English language words
    /// kal: **@CODE@** = English stem: game
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | English
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - English
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// deu: (no comment)
    /// sme: **@CODE@** = English
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/ENG")]
    OLang_SLASH_ENG,
    /// fit: **@CODE@**
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Finnish
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Finnish
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** olang tags
    /// deu: (no comment)
    /// sme: **@CODE@** = Finnish
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/FIN")]
    OLang_SLASH_FIN,
    /// gle: **@CODE@** = -
    /// sma: **@CODE@** | Hungarian
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Hungarian
    /// rmn: (no comment)
    /// sme: **@CODE@** = Hungarian
    /// smn: **@CODE@**
    #[strum(serialize = "OLang/HUN")]
    OLang_SLASH_HUN,
    /// mns: **@CODE@** = Khanty
    #[strum(serialize = "OLang/KCA")]
    OLang_SLASH_KCA,
    /// gle: **@CODE@** = - Latin language words
    #[strum(serialize = "OLang/LAT")]
    OLang_SLASH_LAT,
    /// myv: (no comment)
    /// mdf: **@CODE@** = Moksha
    #[strum(serialize = "OLang/MDF")]
    OLang_SLASH_MDF,
    /// myv: (no comment)
    /// mdf: **@CODE@** = Erzya
    #[strum(serialize = "OLang/MYV")]
    OLang_SLASH_MYV,
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Norw. nynorsk
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Norw. nynorsk
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// deu: (no comment)
    /// sme: **@CODE@** = Norw. nynorsk
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/NNO")]
    OLang_SLASH_NNO,
    /// fit: **@CODE@** = language code for names from common name source
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Norw. bokmål
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Norw. bokmål
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** olang tags
    /// deu: (no comment)
    /// sme: **@CODE@** = Norw. bokmål
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/NOB")]
    OLang_SLASH_NOB,
    /// sma: **@CODE@** | parallelle navn, navnet skal ikke overføres til andre samisk språk
    /// smj: (no comment)
    /// sms: **@CODE@** - parallelle navn, navnet skal ikke overføres til andre samisk språk
    /// sme: **@CODE@** = parallelle navn, navnet skal ikke overføres til andre samisk språk
    /// smn: (no comment)
    #[strum(serialize = "OLang/PARA")]
    OLang_SLASH_PARA,
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Russian
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// mns: **@CODE@** = Russian
    /// sms: **@CODE@** - Russian
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// deu: (no comment)
    /// sme: **@CODE@** = Russian
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: **@CODE@** = Russian
    /// bak: (no comment)
    #[strum(serialize = "OLang/RUS")]
    OLang_SLASH_RUS,
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | South Sámi
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - South Sámi
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// deu: (no comment)
    /// sme: **@CODE@** = South Sámi
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/SMA")]
    OLang_SLASH_SMA,
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | North Sámi
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - North Sámi
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// deu: (no comment)
    /// sme: **@CODE@** = North Sámi
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/SME")]
    OLang_SLASH_SME,
    /// gle: **@CODE@** = -
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Lule Sámi
    /// nob: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Lule Sámi
    /// rmn: (no comment)
    /// myv: (no comment)
    /// sme: **@CODE@** = Lule Sámi
    /// smn: **@CODE@**
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
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Swedish
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// sms: **@CODE@** - Swedish
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** olang tags
    /// deu: (no comment)
    /// sme: **@CODE@** = Swedish
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/SWE")]
    OLang_SLASH_SWE,
    /// myv: (no comment)
    /// mdf: **@CODE@** = Tatar
    #[strum(serialize = "OLang/TAT")]
    OLang_SLASH_TAT,
    /// fit: **@CODE@**
    /// gle: **@CODE@** = -
    /// swe: (no comment)
    /// fkv: **@CODE@** = Language of common names
    /// sma: **@CODE@** | Undefined
    /// nob: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// mns: **@CODE@** = Undefined
    /// sms: **@CODE@** - Undefined
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** olang tags
    /// deu: (no comment)
    /// sme: **@CODE@** = Undefined
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "OLang/UND")]
    OLang_SLASH_UND,
    /// fin: (no comment)
    #[strum(serialize = "OLang/eng")]
    OLang_SLASH_eng,
    /// gle: **@CODE@** = Object e.g. á = "do a" when obj of VN
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// mya: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Obj")]
    Obj,
    /// tgl: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Obj1")]
    Obj1,
    /// tgl: **@CODE@** - Oblique
    /// rmy: (no comment)
    /// rmn: (no comment)
    /// rmf: **@CODE@**  Obliikvi
    /// ceb: **@CODE@** - Oblique
    #[strum(serialize = "Obl")]
    Obl,
    /// myv: (no comment)
    /// mpj: **@CODE@**  = Obligative
    #[strum(serialize = "Oblig")]
    Oblig,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Obv")]
    Obv,
    /// kca: (no comment)
    #[strum(serialize = "Oc1Du")]
    Oc1Du,
    /// kca: (no comment)
    #[strum(serialize = "Oc1Pl")]
    Oc1Pl,
    /// kca: (no comment)
    #[strum(serialize = "Oc1Sg")]
    Oc1Sg,
    /// apu: (no comment)
    #[strum(serialize = "Oc3M")]
    Oc3M,
    /// yrk: **@CODE@**
    #[strum(serialize = "OcDu3")]
    OcDu3,
    /// lut: (no comment)
    /// slh: (no comment)
    /// luo: (no comment)
    /// apu: **@CODE@** object conjugation first person plural
    /// myv: (no comment)
    /// pad: **@CODE@** object conjugation first person plural
    /// mdf: (no comment)
    #[strum(serialize = "OcPl1")]
    OcPl1,
    /// lut: (no comment)
    /// slh: (no comment)
    /// luo: (no comment)
    /// apu: **@CODE@** object conjugation second person plural
    /// myv: (no comment)
    /// pad: **@CODE@** object conjugation second person plural
    /// mdf: (no comment)
    #[strum(serialize = "OcPl2")]
    OcPl2,
    /// luo: (no comment)
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "OcPl3")]
    OcPl3,
    /// apu: **@CODE@** object conjugation third person plural Feminine
    /// pad: **@CODE@** object conjugation third person plural Feminine
    #[strum(serialize = "OcPl3F")]
    OcPl3F,
    /// apu: **@CODE@** object conjugation third person plural Male
    /// pad: **@CODE@** object conjugation third person plural Male
    #[strum(serialize = "OcPl3M")]
    OcPl3M,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "OcSP3")]
    OcSP3,
    /// mns: **@CODE@** Verb object conjugation
    #[strum(serialize = "OcSg")]
    OcSg,
    /// pur: **@CODE@** object conjugation first person singular on verb
    /// lut: (no comment)
    /// slh: (no comment)
    /// luo: (no comment)
    /// apu: **@CODE@** object conjugation first person singular
    /// myv: (no comment)
    /// pad: **@CODE@** object conjugation first person singular
    /// mdf: (no comment)
    #[strum(serialize = "OcSg1")]
    OcSg1,
    /// pur: **@CODE@** object conjugation second person singular
    /// lut: (no comment)
    /// slh: (no comment)
    /// luo: (no comment)
    /// apu: **@CODE@** object conjugation second person singular
    /// myv: (no comment)
    /// pad: **@CODE@** object conjugation second person singular
    /// mdf: (no comment)
    #[strum(serialize = "OcSg2")]
    OcSg2,
    /// pur: **@CODE@** object conjugation third person singular
    /// luo: (no comment)
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "OcSg3")]
    OcSg3,
    /// apu: **@CODE@** object conjugation third person singular Feminine
    /// pad: **@CODE@** object conjugation third person singular Feminine
    #[strum(serialize = "OcSg3F")]
    OcSg3F,
    /// apu: **@CODE@** object conjugation third person singular Male
    /// pad: **@CODE@** object conjugation third person singular Male
    #[strum(serialize = "OcSg3M")]
    OcSg3M,
    /// cho: (no comment)
    #[strum(serialize = "Only")]
    Only,
    /// myv: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "Onom")]
    Onom,
    /// gle: **@CODE@** = Number Operators, e.g. +,-,*,/ etc.
    #[strum(serialize = "Op")]
    Op,
    /// kal: **@CODE@** = Optative
    /// udm: (no comment)
    /// luo: (no comment)
    /// yrk: **@CODE@**
    /// bxr: (no comment)
    /// tku: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  optative
    /// khk: (no comment)
    #[strum(serialize = "Opt")]
    Opt,
    /// fit: **@CODE@** = Ordinals
    /// sje: (no comment)
    /// gle: **@CODE@** = Ordinal (first, second, third..) i.e.  mo dhá lámh, an chéad dhá theach
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Ordinal
    /// izh: **@CODE@** ordinal
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// rue: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@**:  Ordinal number
    /// lut: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@**
    /// got: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// yrk: (no comment)
    /// xwo: (no comment)
    /// sms: **@CODE@** -
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// grn: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: **@CODE@**:  ordinal + NOrd
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// sme: **@CODE@**   Ordinal Number
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  ordinal
    /// ale: (no comment)
    /// ceb: **@CODE@** - Ordinal (etymologically these are numerals, but syntactically adjective might better suit them. I have even contemplated determiner, but no one else seems to. Jaska 2024-08-04
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// fro: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Ord")]
    Ord,
    /// hdn: (no comment)
    #[strum(serialize = "Order")]
    Order,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Org")]
    Org,
    /// kal: **@CODE@** = uden bindestreg ved låneord, for at kunne generere en bestemt form til Katersat
    #[strum(serialize = "Orth/-Hyph")]
    Orth_SLASH__MINUS_Hyph,
    /// myv: (no comment)
    #[strum(serialize = "Orth/-thirties")]
    Orth_SLASH__MINUS_thirties,
    /// myv: (no comment)
    #[strum(serialize = "Orth/-wiki")]
    Orth_SLASH__MINUS_wiki,
    /// mrj: **@CODE@**
    #[strum(serialize = "Orth/1933")]
    Orth_SLASH_1933,
    /// kal: **@CODE@** = Substandard language (hyphens among other things…): isissavoq, Qaanaaq-mut, fabrikki, poor’lu
    #[strum(serialize = "Orth/Alt")]
    Orth_SLASH_Alt,
    /// kal: **@CODE@** = Archaic language, (stem, morpheme, flexion, or sandhi): tikisimavoq, asallutigik, agpâ (for anípâ)
    #[strum(serialize = "Orth/Arch")]
    Orth_SLASH_Arch,
    /// koi: (no comment)
    #[strum(serialize = "Orth/Bible")]
    Orth_SLASH_Bible,
    /// udm: (no comment)
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Orth/Colloq")]
    Orth_SLASH_Colloq,
    /// kal: **@CODE@** = bindestreg ved låneord, for at kunne generere en bestemt form til Katersat
    #[strum(serialize = "Orth/Hyph")]
    Orth_SLASH_Hyph,
    /// sme: (no comment)
    #[strum(serialize = "Orth/IPA")]
    Orth_SLASH_IPA,
    /// myv: (no comment)
    #[strum(serialize = "Orth/Morph")]
    Orth_SLASH_Morph,
    /// myv: (no comment)
    #[strum(serialize = "Orth/PhonComb")]
    Orth_SLASH_PhonComb,
    /// myv: (no comment)
    #[strum(serialize = "Orth/PhonDeriv")]
    Orth_SLASH_PhonDeriv,
    /// myv: (no comment)
    #[strum(serialize = "Orth/PhonInfl")]
    Orth_SLASH_PhonInfl,
    /// myv: (no comment)
    #[strum(serialize = "Orth/SH")]
    Orth_SLASH_SH,
    /// sme: (no comment)
    #[strum(serialize = "Orth/Strd")]
    Orth_SLASH_Strd,
    /// kal: **@CODE@** = Youth language: asallutigu
    #[strum(serialize = "Orth/Youth")]
    Orth_SLASH_Youth,
    /// myv: (no comment)
    #[strum(serialize = "Orth/fifties")]
    Orth_SLASH_fifties,
    /// kal: **@CODE@** = long i, benziina versus benzina
    #[strum(serialize = "Orth/longi")]
    Orth_SLASH_longi,
    /// myv: (no comment)
    #[strum(serialize = "Orth/nineties")]
    Orth_SLASH_nineties,
    /// myv: (no comment)
    #[strum(serialize = "Orth/seventies")]
    Orth_SLASH_seventies,
    /// kal: **@CODE@** = short i, benzina versus benziina
    #[strum(serialize = "Orth/shorti")]
    Orth_SLASH_shorti,
    /// myv: (no comment)
    #[strum(serialize = "Orth/standard")]
    Orth_SLASH_standard,
    /// myv: (no comment)
    #[strum(serialize = "Orth/standard_wiki")]
    Orth_SLASH_standard_wiki,
    /// myv: (no comment)
    #[strum(serialize = "Orth/thirties")]
    Orth_SLASH_thirties,
    /// myv: (no comment)
    #[strum(serialize = "Orth/wiki")]
    Orth_SLASH_wiki,
    /// kal: **@CODE@** = s used instead of z, bensiina
    #[strum(serialize = "Orth/ztos")]
    Orth_SLASH_ztos,
    /// kmr: (no comment)
    #[strum(serialize = "P1")]
    P1,
    /// crj: (no comment)
    /// tku: (no comment)
    #[strum(serialize = "P2lO")]
    P2lO,
    /// kal: **@CODE@**
    #[strum(serialize = "PAAQ")]
    PAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "PAJAAQ")]
    PAJAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "PAJAAR")]
    PAJAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "PAJUK")]
    PAJUK,
    /// kal: **@CODE@**
    #[strum(serialize = "PAJUP")]
    PAJUP,
    /// kal: **@CODE@**
    #[strum(serialize = "PAK")]
    PAK,
    /// kal: **@CODE@**
    #[strum(serialize = "PALAAQ")]
    PALAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "PALAAR")]
    PALAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "PALLAK")]
    PALLAK,
    /// kal: **@CODE@**
    #[strum(serialize = "PALLAP")]
    PALLAP,
    /// kal: **@CODE@**
    #[strum(serialize = "PALUK")]
    PALUK,
    /// kal: **@CODE@**
    #[strum(serialize = "PALUP")]
    PALUP,
    /// smn: **@CODE@**
    #[strum(serialize = "PAR")]
    PAR,
    /// kal: **@CODE@**
    #[strum(serialize = "PASIP")]
    PASIP,
    /// ciw: (no comment)
    #[strum(serialize = "PCAsp")]
    PCAsp,
    /// tgl: (no comment)
    #[strum(serialize = "PF")]
    PF,
    /// eus: (no comment)
    #[strum(serialize = "PGen")]
    PGen,
    /// chp: (no comment)
    #[strum(serialize = "PI")]
    PI,
    /// kal: **@CODE@**
    #[strum(serialize = "PIAQ")]
    PIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "PIAR")]
    PIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "PILUK")]
    PILUK,
    /// kal: **@CODE@**
    #[strum(serialize = "PILUP")]
    PILUP,
    /// kal: **@CODE@**
    #[strum(serialize = "PILUUR")]
    PILUUR,
    /// chp: (no comment)
    #[strum(serialize = "PNS")]
    PNS,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "PObj")]
    PObj,
    /// som: (no comment)
    #[strum(serialize = "PP")]
    PP,
    /// bla: (no comment)
    #[strum(serialize = "PRO")]
    PRO,
    /// ciw: (no comment)
    #[strum(serialize = "PRONDem")]
    PRONDem,
    /// ciw: (no comment)
    /// ciw: (no comment)
    #[strum(serialize = "PRONDub")]
    PRONDub,
    /// ciw: (no comment)
    /// ciw: (no comment)
    #[strum(serialize = "PRONInter")]
    PRONInter,
    /// ciw: (no comment)
    /// ciw: (no comment)
    #[strum(serialize = "PRONPer")]
    PRONPer,
    /// ciw: (no comment)
    /// ciw: (no comment)
    #[strum(serialize = "PRONSim")]
    PRONSim,
    /// fit: **@CODE@** = Punctuation mark
    /// sje: (no comment)
    /// gle: **@CODE@** = Abbreviation (it seems several languages have two tags :-/
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Punctuation mark
    /// liv: **@CODE@**
    /// crk: (no comment)
    /// mhr: **@CODE@** = other punctuation marks
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@**:  XXX These should be documented better
    /// tgl: (no comment)
    /// hdn: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = punctuation
    /// bxr: (no comment)
    /// vep: **@CODE@**
    /// cho: (no comment)
    /// som: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@**
    /// xak: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@**  punctuation
    /// srs: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// rmf: **@CODE@** other punctuation symbol
    /// ceb: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "PUNCT")]
    PUNCT,
    /// kal: **@CODE@** = Particle-like - partikellignende
    #[strum(serialize = "Pali")]
    Pali,
    /// fit: **@CODE@** = Partitive
    /// kal: **@CODE@** = Participium
    /// vot: (no comment)
    /// fkv: **@CODE@** = Partitive
    /// liv: **@CODE@** = partitive
    /// smj: **@CODE@** = Partitive case
    /// sms: **@CODE@** Partative
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**  = partitive
    /// olo: **@CODE@** partitive
    /// fin: (no comment)
    /// khk: (no comment)
    /// vro: **@CODE@** partitive
    #[strum(serialize = "Par")]
    Par,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Paren")]
    Paren,
    /// kpv: **@CODE@** parenthetical phrase
    /// lut: (no comment)
    /// slh: (no comment)
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: **@CODE@** parenthetical phrase
    #[strum(serialize = "Parenthetic")]
    Parenthetic,
    /// gle: **@CODE@** = Particle (not +Vb) (U)
    /// chp: (no comment)
    #[strum(serialize = "Part")]
    Part,
    /// epo: (no comment)
    #[strum(serialize = "Participle")]
    Participle,
    /// epo: (no comment)
    #[strum(serialize = "Particle")]
    Particle,
    /// fit: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** = Passive
    /// mhr: **@CODE@** = Passive
    /// rue: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@** = for verb voice, mood, tense
    /// fao: (no comment)
    /// slh: (no comment)
    /// apu: **@CODE@**
    /// rus: (no comment)
    /// sms: **@CODE@** - hallat/haddat not in use
    /// sms: **@CODE@** Passive voice
    /// mrj: **@CODE@** =  passive participle ЫмЫ
    /// sme: **@CODE@** - hallat/haddat not in use
    /// smn: **@CODE@**
    /// khk: (no comment)
    #[strum(serialize = "Pass")]
    Pass,
    /// bxr: (no comment)
    #[strum(serialize = "PassPrc")]
    PassPrc,
    /// gle: **@CODE@** = Past tense verbal particle
    /// hdn: (no comment)
    /// qya: (no comment)
    /// epo: (no comment)
    /// som: (no comment)
    #[strum(serialize = "Past")]
    Past,
    /// gle: **@CODE@** = Past Habitual - Gháthchaite (Imperfect Indicative)
    #[strum(serialize = "PastImp")]
    PastImp,
    /// gle: **@CODE@** = Past Indicative tense
    #[strum(serialize = "PastInd")]
    PastInd,
    /// kpv: **@CODE@**
    #[strum(serialize = "PastPtc")]
    PastPtc,
    /// gle: **@CODE@** = Past Subjunctive tense
    #[strum(serialize = "PastSubj")]
    PastSubj,
    /// gle: **@CODE@** = Patronymic particle (p) (e.g. Ó, Ní, Uí, le, de ..)
    /// khk: (no comment)
    #[strum(serialize = "Pat")]
    Pat,
    /// moh: (no comment)
    #[strum(serialize = "PatDu2")]
    PatDu2,
    /// moh: (no comment)
    #[strum(serialize = "PatDu21")]
    PatDu21,
    /// moh: (no comment)
    #[strum(serialize = "PatDu3")]
    PatDu3,
    /// moh: (no comment)
    #[strum(serialize = "PatDu31")]
    PatDu31,
    /// moh: (no comment)
    #[strum(serialize = "PatDu3Fem")]
    PatDu3Fem,
    /// moh: (no comment)
    #[strum(serialize = "PatPl2")]
    PatPl2,
    /// moh: (no comment)
    #[strum(serialize = "PatPl2Sg1")]
    PatPl2Sg1,
    /// moh: (no comment)
    #[strum(serialize = "PatPl3")]
    PatPl3,
    /// moh: (no comment)
    #[strum(serialize = "PatPl3Fem")]
    PatPl3Fem,
    /// moh: (no comment)
    #[strum(serialize = "PatPl3Sg1")]
    PatPl3Sg1,
    /// moh: (no comment)
    #[strum(serialize = "PatSg1")]
    PatSg1,
    /// moh: (no comment)
    #[strum(serialize = "PatSg2")]
    PatSg2,
    /// moh: (no comment)
    #[strum(serialize = "PatSg3Fem")]
    PatSg3Fem,
    /// moh: (no comment)
    #[strum(serialize = "PatSg3Mal")]
    PatSg3Mal,
    /// moh: (no comment)
    #[strum(serialize = "PatSg3Neuter")]
    PatSg3Neuter,
    /// mhr: **@CODE@** = patronym, look at this in other cyr fsts.
    /// olo: (no comment)
    #[strum(serialize = "Patr")]
    Patr,
    /// pma: (no comment)
    #[strum(serialize = "Pauc1")]
    Pauc1,
    /// pma: (no comment)
    #[strum(serialize = "Pauc2")]
    Pauc2,
    /// pma: (no comment)
    #[strum(serialize = "Pauc3")]
    Pauc3,
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
    /// sje: (no comment)
    /// kpv: **@CODE@**:  particle      кывтор   частица
    /// vot: (no comment)
    /// fkv: **@CODE@** = Participle?
    /// liv: **@CODE@** = particle
    /// mhr: **@CODE@** = particles
    /// rue: (no comment)
    /// sma: **@CODE@** = Particle
    /// tgl: **@CODE@** -  Particle
    /// hdn: (no comment)
    /// smj: **@CODE@** = Particle
    /// udm: (no comment)
    /// apu: **@CODE@** Particle
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Particle
    /// bxr: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = particle
    /// myv: (no comment)
    /// olo: **@CODE@**  particle
    /// koi: **@CODE@**:  particle
    /// sme: **@CODE@** - Particle
    /// fin: (no comment)
    /// mdf: **@CODE@**:  particle
    /// ceb: **@CODE@** -  Particle
    /// tlh: (no comment)
    /// vro: **@CODE@**
    /// mpj: **@CODE@**
    #[strum(serialize = "Pcle")]
    Pcle,
    /// otw: (no comment)
    #[strum(serialize = "Pcp")]
    Pcp,
    /// krl: **@CODE@** fourth person?
    /// fin: (no comment)
    #[strum(serialize = "Pe4")]
    Pe4,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "Pej")]
    Pej,
    /// lit: (no comment)
    #[strum(serialize = "PerFreq")]
    PerFreq,
    /// amh: (no comment)
    /// qya: (no comment)
    /// apu: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    /// tlh: (no comment)
    /// mpj: **@CODE@**
    #[strum(serialize = "Perf")]
    Perf,
    /// myv: (no comment)
    /// myv: **@CODE@**:  periferal modifier ськамонза, кавонест
    /// mdf: **@CODE@**:  periferal modifier ськамонза, кавонест
    #[strum(serialize = "PerifMod")]
    PerifMod,
    /// mpj: **@CODE@**
    #[strum(serialize = "Perl")]
    Perl,
    /// fit: **@CODE@** = Personal
    /// sje: (no comment)
    /// gle: **@CODE@** = Personal pronoun
    /// kpv: **@CODE@**:  personal
    /// swe: (no comment)
    /// vot: (no comment)
    /// bla: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Personal
    /// izh: **@CODE@** Types of pronouns
    /// liv: **@CODE@** = personal
    /// crk: (no comment)
    /// mhr: **@CODE@** = Personal pronoun
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@** = Personal
    /// nob: **@CODE@**
    /// hdn: (no comment)
    /// fao: (no comment)
    /// smj: **@CODE@** = Personal pronoun
    /// udm: (no comment)
    /// apu: **@CODE@** Personal Pronoun
    /// apu: **@CODE@** Personal Pronoun
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Personal Pronoun
    /// bxr: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@**:  personal
    /// vep: **@CODE@**  = personal
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** personal
    /// koi: **@CODE@**:  personal
    /// eus: (no comment)
    /// sme: **@CODE@** - Personal Pronoun
    /// smn: **@CODE@** - Personal Pronoun
    /// fin: (no comment)
    /// mdf: **@CODE@**:  personal
    /// ceb: **@CODE@** - Personal
    /// khk: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pers")]
    Pers,
    /// gle: **@CODE@** = Personal name - proper noun
    #[strum(serialize = "PersName")]
    PersName,
    /// tku: (no comment)
    /// xak: (no comment)
    #[strum(serialize = "Pfv")]
    Pfv,
    /// som: (no comment)
    #[strum(serialize = "Pfx")]
    Pfx,
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "Phr")]
    Phr,
    /// fit: **@CODE@** = Plural
    /// sje: (no comment)
    /// gle: **@CODE@** = Plural number
    /// kal: **@CODE@** = Pluralis
    /// kpv: **@CODE@**  plural
    /// chp: (no comment)
    /// vot: (no comment)
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Plural
    /// liv: **@CODE@** = plural
    /// crk: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** = Plural
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// smj: **@CODE@** = Plural number
    /// udm: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// rmy: (no comment)
    /// apu: **@CODE@** Plural
    /// chr: (no comment)
    /// crj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Plural = määŋgailååkk
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**  = plural
    /// cho: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@** plural
    /// koi: (no comment)
    /// eus: (no comment)
    /// sme: **@CODE@** - Plural
    /// srs: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  plural
    /// rmf: **@CODE@** plural
    /// khk: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@** Plural
    #[strum(serialize = "Pl")]
    Pl,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural 1
    /// liv: **@CODE@** = first person plural conjugation
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** | Plural  , 1.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** first person plural
    /// bxr: (no comment)
    /// lit: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@** Plural First Person
    /// myv: (no comment)
    /// tkl: (no comment)
    /// sme: **@CODE@** Plural First Person
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pl1")]
    Pl1,
    /// evn: (no comment)
    /// myu: (no comment)
    #[strum(serialize = "Pl12")]
    Pl12,
    /// udm: (no comment)
    #[strum(serialize = "Pl1Abe")]
    Pl1Abe,
    /// skf: (no comment)
    #[strum(serialize = "Pl1Excl")]
    Pl1Excl,
    /// skf: (no comment)
    #[strum(serialize = "Pl1Incl")]
    Pl1Incl,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural 2
    /// liv: **@CODE@** = second person plural conjugation
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** | Plural  , 2.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// skf: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** second person plural
    /// bxr: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@** Plural Second Person
    /// myv: (no comment)
    /// tkl: (no comment)
    /// sme: **@CODE@** Plural Second Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pl2")]
    Pl2,
    /// udm: (no comment)
    #[strum(serialize = "Pl2Abe")]
    Pl2Abe,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Plural 3
    /// liv: **@CODE@** = third person plural conjugation
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** | Plural  , 3.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** third  person plural
    /// bxr: (no comment)
    /// vep: **@CODE@** Plural Third Person
    /// myv: (no comment)
    /// tkl: (no comment)
    /// sme: **@CODE@** Plural Third Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Pl3")]
    Pl3,
    /// udm: (no comment)
    #[strum(serialize = "Pl3Abe")]
    Pl3Abe,
    /// skf: (no comment)
    #[strum(serialize = "Pl3c")]
    Pl3c,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "PlO")]
    PlO,
    /// gle: **@CODE@** = Place name - proper noun
    /// epo: (no comment)
    #[strum(serialize = "Place")]
    Place,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Plant")]
    Plant,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Plc")]
    Plc,
    /// epo: (no comment)
    #[strum(serialize = "Plu")]
    Plu,
    /// fit: **@CODE@** = Postposition
    /// sje: (no comment)
    /// kpv: **@CODE@**:  postposition   кывбӧр   послелог
    /// chp: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Postposition
    /// liv: **@CODE@** = postposition
    /// mhr: **@CODE@** = postpositions
    /// rue: (no comment)
    /// sma: **@CODE@** = Postposition
    /// hdn: (no comment)
    /// smj: **@CODE@** = Postposition
    /// udm: (no comment)
    /// apu: **@CODE@** Postpostion
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Postposition
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// sms: (no comment)
    /// bxr: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = postposition
    /// myv: (no comment)
    /// olo: **@CODE@**  postposition
    /// koi: **@CODE@**:  postposition
    /// sme: **@CODE@** - Postpostion
    /// fin: (no comment)
    /// mdf: **@CODE@**:  postposition
    /// ceb: **@CODE@** -  Postposition, postposed
    /// vro: **@CODE@** Postposition
    /// mpj: **@CODE@**
    #[strum(serialize = "Po")]
    Po,
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: **@CODE@** positive grade
    /// liv: **@CODE@**
    /// krl: **@CODE@**
    /// rue: (no comment)
    /// sjd: (no comment)
    /// nob: **@CODE@** = For adjectives
    /// fao: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** =  The comparative
    /// vep: **@CODE@**  =
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    /// rmf: **@CODE@** positive
    /// kca: (no comment)
    #[strum(serialize = "Pos")]
    Pos,
    /// gle: **@CODE@** = Possessive pronoun (can be attached to a prep, e.g. im', dá, faoina)
    /// kpv: **@CODE@**:  possessive
    /// pma: (no comment)
    /// evn: (no comment)
    /// rmf: **@CODE@** possessive
    #[strum(serialize = "Poss")]
    Poss,
    /// xak: (no comment)
    #[strum(serialize = "Poss1Nsg")]
    Poss1Nsg,
    /// xak: (no comment)
    #[strum(serialize = "Poss1Sg")]
    Poss1Sg,
    /// xak: (no comment)
    #[strum(serialize = "Poss2Nsg")]
    Poss2Nsg,
    /// xak: (no comment)
    #[strum(serialize = "Poss2Sg")]
    Poss2Sg,
    /// xak: (no comment)
    #[strum(serialize = "Poss3Nsg")]
    Poss3Nsg,
    /// xak: (no comment)
    #[strum(serialize = "Poss3Sg")]
    Poss3Sg,
    /// apu: **@CODE@** Possessed
    #[strum(serialize = "Possd")]
    Possd,
    /// tkl: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Post")]
    Post,
    /// tat: (no comment)
    #[strum(serialize = "Postp")]
    Postp,
    /// fit: **@CODE@** = Potential
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Potential
    /// tgl: **@CODE@** - Potential
    /// smj: (no comment)
    /// skf: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Potential mood
    /// lit: (no comment)
    /// vep: **@CODE@**  = potential linne-
    /// olo: **@CODE@** Potential
    /// pad: (no comment)
    /// eus: (no comment)
    /// sme: **@CODE@** Potential
    /// fin: (no comment)
    /// ceb: **@CODE@** - Potential
    /// vro: **@CODE@**
    #[strum(serialize = "Pot")]
    Pot,
    /// bxr: (no comment)
    #[strum(serialize = "PotPrc")]
    PotPrc,
    /// fit: **@CODE@** = Preposition
    /// sje: (no comment)
    /// kpv: **@CODE@**:  preposition   XX   предлог
    /// vot: (no comment)
    /// fkv: **@CODE@** = Preposition
    /// liv: **@CODE@** = preposition
    /// mhr: **@CODE@** = prepositons
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@** = Preposition
    /// tgl: **@CODE@** -  Preposition
    /// hdn: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Preposition
    /// udm: (no comment)
    /// apu: **@CODE@** Preposition
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Preposition
    /// sms: (no comment)
    /// sms: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = preposition
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  preposition
    /// koi: **@CODE@**:  preposition
    /// deu: (no comment)
    /// sme: **@CODE@** - Preposition
    /// non: (no comment)
    /// fin: (no comment)
    /// ceb: **@CODE@** -  Preposition
    /// vro: **@CODE@** Preposition
    /// mpj: **@CODE@**
    #[strum(serialize = "Pr")]
    Pr,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Prb")]
    Prb,
    /// izh: **@CODE@**
    /// liv: **@CODE@** = participle CHECK! how is this used ?
    /// mhr: **@CODE@** = Participle
    /// udm: (no comment)
    /// mns: **@CODE@**
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Prc")]
    Prc,
    /// myv: (no comment)
    #[strum(serialize = "Prc/Telic")]
    Prc_SLASH_Telic,
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
    /// udm: (no comment)
    #[strum(serialize = "PrcPrfAdn")]
    PrcPrfAdn,
    /// udm: (no comment)
    #[strum(serialize = "PrcPrfNeg")]
    PrcPrfNeg,
    /// udm: (no comment)
    #[strum(serialize = "PrcPrfPred")]
    PrcPrfPred,
    /// udm: (no comment)
    #[strum(serialize = "PrcPrs")]
    PrcPrs,
    /// udm: (no comment)
    #[strum(serialize = "PrcPrsNeg")]
    PrcPrsNeg,
    /// udm: (no comment)
    #[strum(serialize = "PrcPrsPos")]
    PrcPrsPos,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Prcnt")]
    Prcnt,
    /// nob: **@CODE@**
    #[strum(serialize = "Prdt")]
    Prdt,
    /// bla: (no comment)
    #[strum(serialize = "Pre")]
    Pre,
    /// myv: (no comment)
    /// koi: **@CODE@**:  Precative mood is a directive mood that signals that the utterance is a request. -ко on imperative forms equals Precative
    /// mdf: **@CODE@**:  Precative mood is a directive mood that signals that the utterance is a request. imperative + additional vowel and cons -ака forms equal Precative
    #[strum(serialize = "Prec")]
    Prec,
    /// rue: (no comment)
    /// apu: **@CODE@** = ka
    /// rus: (no comment)
    /// yrk: **@CODE@** = predestinative
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Pred")]
    Pred,
    /// myv: (no comment)
    #[strum(serialize = "Pred2")]
    Pred2,
    /// gle: **@CODE@** = Prefix; seperated prefixes in historical texts
    /// lit: (no comment)
    /// myv: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Pref")]
    Pref,
    /// fin: (no comment)
    #[strum(serialize = "Pref-")]
    Pref_MINUS_,
    /// deu: (no comment)
    #[strum(serialize = "Pref/ab")]
    Pref_SLASH_ab,
    /// deu: (no comment)
    #[strum(serialize = "Pref/abhande")]
    Pref_SLASH_abhande,
    /// mns: (no comment)
    #[strum(serialize = "Pref/akwan")]
    Pref_SLASH_akwan,
    /// deu: (no comment)
    #[strum(serialize = "Pref/an")]
    Pref_SLASH_an,
    /// deu: (no comment)
    #[strum(serialize = "Pref/auf")]
    Pref_SLASH_auf,
    /// deu: (no comment)
    #[strum(serialize = "Pref/aus")]
    Pref_SLASH_aus,
    /// deu: (no comment)
    #[strum(serialize = "Pref/auseinander")]
    Pref_SLASH_auseinander,
    /// deu: (no comment)
    #[strum(serialize = "Pref/bei")]
    Pref_SLASH_bei,
    /// deu: (no comment)
    #[strum(serialize = "Pref/dabei")]
    Pref_SLASH_dabei,
    /// deu: (no comment)
    #[strum(serialize = "Pref/daher")]
    Pref_SLASH_daher,
    /// deu: (no comment)
    #[strum(serialize = "Pref/dahin")]
    Pref_SLASH_dahin,
    /// deu: (no comment)
    #[strum(serialize = "Pref/daneben")]
    Pref_SLASH_daneben,
    /// deu: (no comment)
    #[strum(serialize = "Pref/dar")]
    Pref_SLASH_dar,
    /// deu: (no comment)
    #[strum(serialize = "Pref/daran")]
    Pref_SLASH_daran,
    /// deu: (no comment)
    #[strum(serialize = "Pref/darauf")]
    Pref_SLASH_darauf,
    /// deu: (no comment)
    #[strum(serialize = "Pref/davon")]
    Pref_SLASH_davon,
    /// deu: (no comment)
    #[strum(serialize = "Pref/dazwischen")]
    Pref_SLASH_dazwischen,
    /// deu: (no comment)
    #[strum(serialize = "Pref/drauf")]
    Pref_SLASH_drauf,
    /// deu: (no comment)
    #[strum(serialize = "Pref/durch")]
    Pref_SLASH_durch,
    /// deu: (no comment)
    #[strum(serialize = "Pref/durcheinander")]
    Pref_SLASH_durcheinander,
    /// mns: (no comment)
    #[strum(serialize = "Pref/eel")]
    Pref_SLASH_eel,
    /// mns: (no comment)
    #[strum(serialize = "Pref/eelalj")]
    Pref_SLASH_eelalj,
    /// deu: (no comment)
    #[strum(serialize = "Pref/ein")]
    Pref_SLASH_ein,
    /// deu: (no comment)
    #[strum(serialize = "Pref/einher")]
    Pref_SLASH_einher,
    /// deu: (no comment)
    #[strum(serialize = "Pref/empor")]
    Pref_SLASH_empor,
    /// deu: (no comment)
    #[strum(serialize = "Pref/entgegen")]
    Pref_SLASH_entgegen,
    /// deu: (no comment)
    #[strum(serialize = "Pref/entlang")]
    Pref_SLASH_entlang,
    /// deu: (no comment)
    #[strum(serialize = "Pref/entzwei")]
    Pref_SLASH_entzwei,
    /// deu: (no comment)
    #[strum(serialize = "Pref/fehl")]
    Pref_SLASH_fehl,
    /// deu: (no comment)
    #[strum(serialize = "Pref/fertig")]
    Pref_SLASH_fertig,
    /// deu: (no comment)
    #[strum(serialize = "Pref/fort")]
    Pref_SLASH_fort,
    /// deu: (no comment)
    #[strum(serialize = "Pref/fremd")]
    Pref_SLASH_fremd,
    /// deu: (no comment)
    #[strum(serialize = "Pref/gut")]
    Pref_SLASH_gut,
    /// deu: (no comment)
    #[strum(serialize = "Pref/heim")]
    Pref_SLASH_heim,
    /// deu: (no comment)
    #[strum(serialize = "Pref/her")]
    Pref_SLASH_her,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herab")]
    Pref_SLASH_herab,
    /// deu: (no comment)
    #[strum(serialize = "Pref/heran")]
    Pref_SLASH_heran,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herauf")]
    Pref_SLASH_herauf,
    /// deu: (no comment)
    #[strum(serialize = "Pref/heraus")]
    Pref_SLASH_heraus,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herbei")]
    Pref_SLASH_herbei,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herein")]
    Pref_SLASH_herein,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hernieder")]
    Pref_SLASH_hernieder,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herum")]
    Pref_SLASH_herum,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herunter")]
    Pref_SLASH_herunter,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hervor")]
    Pref_SLASH_hervor,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herzu")]
    Pref_SLASH_herzu,
    /// deu: (no comment)
    #[strum(serialize = "Pref/herüber")]
    Pref_SLASH_herüber,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hin")]
    Pref_SLASH_hin,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinab")]
    Pref_SLASH_hinab,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinan")]
    Pref_SLASH_hinan,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinauf")]
    Pref_SLASH_hinauf,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinaus")]
    Pref_SLASH_hinaus,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinein")]
    Pref_SLASH_hinein,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinfort")]
    Pref_SLASH_hinfort,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinter")]
    Pref_SLASH_hinter,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinunter")]
    Pref_SLASH_hinunter,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinweg")]
    Pref_SLASH_hinweg,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinzu")]
    Pref_SLASH_hinzu,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hinüber")]
    Pref_SLASH_hinüber,
    /// deu: (no comment)
    #[strum(serialize = "Pref/hoch")]
    Pref_SLASH_hoch,
    /// deu: (no comment)
    #[strum(serialize = "Pref/irr")]
    Pref_SLASH_irr,
    /// mns: (no comment)
    #[strum(serialize = "Pref/jol")]
    Pref_SLASH_jol,
    /// mns: (no comment)
    #[strum(serialize = "Pref/jola")]
    Pref_SLASH_jola,
    /// mns: (no comment)
    #[strum(serialize = "Pref/jot")]
    Pref_SLASH_jot,
    /// mns: (no comment)
    #[strum(serialize = "Pref/juil")]
    Pref_SLASH_juil,
    /// mns: (no comment)
    #[strum(serialize = "Pref/juw")]
    Pref_SLASH_juw,
    /// mns: (no comment)
    #[strum(serialize = "Pref/juwle")]
    Pref_SLASH_juwle,
    /// deu: (no comment)
    #[strum(serialize = "Pref/kaputt")]
    Pref_SLASH_kaputt,
    /// deu: (no comment)
    #[strum(serialize = "Pref/klar")]
    Pref_SLASH_klar,
    /// mns: (no comment)
    #[strum(serialize = "Pref/kon")]
    Pref_SLASH_kon,
    /// mns: (no comment)
    #[strum(serialize = "Pref/konalj")]
    Pref_SLASH_konalj,
    /// mns: (no comment)
    #[strum(serialize = "Pref/laakkwa")]
    Pref_SLASH_laakkwa,
    /// deu: (no comment)
    #[strum(serialize = "Pref/lang")]
    Pref_SLASH_lang,
    /// mns: (no comment)
    #[strum(serialize = "Pref/lap")]
    Pref_SLASH_lap,
    /// mns: (no comment)
    #[strum(serialize = "Pref/ljaaljt")]
    Pref_SLASH_ljaaljt,
    /// deu: (no comment)
    #[strum(serialize = "Pref/los")]
    Pref_SLASH_los,
    /// deu: (no comment)
    #[strum(serialize = "Pref/längs")]
    Pref_SLASH_längs,
    /// deu: (no comment)
    #[strum(serialize = "Pref/mit")]
    Pref_SLASH_mit,
    /// mns: (no comment)
    #[strum(serialize = "Pref/naaluw")]
    Pref_SLASH_naaluw,
    /// deu: (no comment)
    #[strum(serialize = "Pref/nach")]
    Pref_SLASH_nach,
    /// deu: (no comment)
    #[strum(serialize = "Pref/nahe")]
    Pref_SLASH_nahe,
    /// deu: (no comment)
    #[strum(serialize = "Pref/nieder")]
    Pref_SLASH_nieder,
    /// mns: (no comment)
    #[strum(serialize = "Pref/noox")]
    Pref_SLASH_noox,
    /// deu: (no comment)
    #[strum(serialize = "Pref/näher")]
    Pref_SLASH_näher,
    /// mns: (no comment)
    #[strum(serialize = "Pref/paag")]
    Pref_SLASH_paag,
    /// mns: (no comment)
    #[strum(serialize = "Pref/paalyg")]
    Pref_SLASH_paalyg,
    /// mns: (no comment)
    #[strum(serialize = "Pref/pooxan")]
    Pref_SLASH_pooxan,
    /// mns: (no comment)
    #[strum(serialize = "Pref/pulig")]
    Pref_SLASH_pulig,
    /// mns: (no comment)
    #[strum(serialize = "Pref/raawyg")]
    Pref_SLASH_raawyg,
    /// deu: (no comment)
    #[strum(serialize = "Pref/ran")]
    Pref_SLASH_ran,
    /// deu: (no comment)
    #[strum(serialize = "Pref/rauf")]
    Pref_SLASH_rauf,
    /// deu: (no comment)
    #[strum(serialize = "Pref/raus")]
    Pref_SLASH_raus,
    /// deu: (no comment)
    #[strum(serialize = "Pref/rein")]
    Pref_SLASH_rein,
    /// deu: (no comment)
    #[strum(serialize = "Pref/rum")]
    Pref_SLASH_rum,
    /// deu: (no comment)
    #[strum(serialize = "Pref/runter")]
    Pref_SLASH_runter,
    /// deu: (no comment)
    #[strum(serialize = "Pref/rüber")]
    Pref_SLASH_rüber,
    /// deu: (no comment)
    #[strum(serialize = "Pref/schief")]
    Pref_SLASH_schief,
    /// mns: (no comment)
    #[strum(serialize = "Pref/supyg")]
    Pref_SLASH_supyg,
    /// mns: (no comment)
    #[strum(serialize = "Pref/taara")]
    Pref_SLASH_taara,
    /// deu: (no comment)
    #[strum(serialize = "Pref/tot")]
    Pref_SLASH_tot,
    /// mns: (no comment)
    #[strum(serialize = "Pref/tuw")]
    Pref_SLASH_tuw,
    /// deu: (no comment)
    #[strum(serialize = "Pref/um")]
    Pref_SLASH_um,
    /// deu: (no comment)
    #[strum(serialize = "Pref/unter")]
    Pref_SLASH_unter,
    /// deu: (no comment)
    #[strum(serialize = "Pref/voll")]
    Pref_SLASH_voll,
    /// deu: (no comment)
    #[strum(serialize = "Pref/vonstatten")]
    Pref_SLASH_vonstatten,
    /// deu: (no comment)
    #[strum(serialize = "Pref/vor")]
    Pref_SLASH_vor,
    /// deu: (no comment)
    #[strum(serialize = "Pref/voran")]
    Pref_SLASH_voran,
    /// deu: (no comment)
    #[strum(serialize = "Pref/voraus")]
    Pref_SLASH_voraus,
    /// deu: (no comment)
    #[strum(serialize = "Pref/vorbei")]
    Pref_SLASH_vorbei,
    /// deu: (no comment)
    #[strum(serialize = "Pref/vorwärts")]
    Pref_SLASH_vorwärts,
    /// deu: (no comment)
    #[strum(serialize = "Pref/vorüber")]
    Pref_SLASH_vorüber,
    /// deu: (no comment)
    #[strum(serialize = "Pref/weg")]
    Pref_SLASH_weg,
    /// deu: (no comment)
    #[strum(serialize = "Pref/weiter")]
    Pref_SLASH_weiter,
    /// deu: (no comment)
    #[strum(serialize = "Pref/wieder")]
    Pref_SLASH_wieder,
    /// mns: (no comment)
    #[strum(serialize = "Pref/xomi")]
    Pref_SLASH_xomi,
    /// mns: (no comment)
    #[strum(serialize = "Pref/xoot")]
    Pref_SLASH_xoot,
    /// mns: (no comment)
    #[strum(serialize = "Pref/xot")]
    Pref_SLASH_xot,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zu")]
    Pref_SLASH_zu,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zugrunde")]
    Pref_SLASH_zugrunde,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zurecht")]
    Pref_SLASH_zurecht,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zurück")]
    Pref_SLASH_zurück,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zusammen")]
    Pref_SLASH_zusammen,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zustande")]
    Pref_SLASH_zustande,
    /// deu: (no comment)
    #[strum(serialize = "Pref/zuvor")]
    Pref_SLASH_zuvor,
    /// deu: (no comment)
    #[strum(serialize = "Pref/über")]
    Pref_SLASH_über,
    /// deu: (no comment)
    #[strum(serialize = "Pref/überein")]
    Pref_SLASH_überein,
    /// liv: **@CODE@**
    /// epo: (no comment)
    /// sms: **@CODE@** = prefix
    /// sms: (no comment)
    #[strum(serialize = "Prefix")]
    Prefix,
    /// fit: **@CODE@**
    #[strum(serialize = "Prel")]
    Prel,
    /// gle: **@CODE@** = Preposition
    /// fro: (no comment)
    #[strum(serialize = "Prep")]
    Prep,
    /// gle: **@CODE@** = Copula present & future tense
    /// hdn: (no comment)
    /// epo: (no comment)
    /// som: (no comment)
    /// xak: (no comment)
    /// khk: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Pres")]
    Pres,
    /// gle: **@CODE@** =  Pres Habitual - Gháthláithreach(Verb bí only - and deireann (abair)
    #[strum(serialize = "PresImp")]
    PresImp,
    /// gle: **@CODE@** = Present Indicative
    #[strum(serialize = "PresInd")]
    PresInd,
    /// gle: **@CODE@** = Present Subjunctive
    #[strum(serialize = "PresSubj")]
    PresSubj,
    /// bxr: (no comment)
    #[strum(serialize = "Presc")]
    Presc,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "Presentational")]
    Presentational,
    /// mhr: **@CODE@** = perfective
    /// luo: (no comment)
    /// skf: (no comment)
    /// crj: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// vep: **@CODE@**
    /// tkl: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Prf")]
    Prf,
    /// fit: **@CODE@** = perfect participe
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Preteritum Particip
    /// izh: **@CODE@**  Which one is needed?
    /// liv: **@CODE@**
    /// sjd: (no comment)
    /// sma: **@CODE@** | Infinitive and participles
    /// smj: (no comment)
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  = nu
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** : past participle
    /// deu: (no comment)
    /// sme: **@CODE@** Perfect Participe
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
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
    /// rmg: **@CODE@**
    #[strum(serialize = "PrfPrt")]
    PrfPrt,
    /// rmg: **@CODE@**
    #[strum(serialize = "PrfPtc")]
    PrfPtc,
    /// kmr: (no comment)
    #[strum(serialize = "Pri")]
    Pri,
    /// lit: (no comment)
    #[strum(serialize = "Priv")]
    Priv,
    /// kpv: **@CODE@** move to Abe
    #[strum(serialize = "PrivMod")]
    PrivMod,
    /// kpv: **@CODE@** prolative -ӧд вуджан
    /// liv: **@CODE@** = prolative
    /// evn: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Prolative
    /// vep: **@CODE@**  = prolative
    /// som: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// grn: (no comment)
    /// olo: **@CODE@** prolative
    /// koi: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  prolative
    #[strum(serialize = "Prl")]
    Prl,
    /// gle: **@CODE@** = Pronoun with copula or relative particle
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// mya: (no comment)
    /// fin: (no comment)
    /// fro: (no comment)
    #[strum(serialize = "Pro")]
    Pro,
    /// tgl: **@CODE@** - Progressive
    /// apu: **@CODE@** nany
    /// tku: (no comment)
    /// som: (no comment)
    /// ceb: **@CODE@** -
    /// tlh: (no comment)
    #[strum(serialize = "Prog")]
    Prog,
    /// apu: **@CODE@** na
    #[strum(serialize = "Prog2")]
    Prog2,
    /// apu: **@CODE@** ã (?= Loc)
    #[strum(serialize = "Prog3")]
    Prog3,
    /// oji: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Proh")]
    Proh,
    /// koi: (no comment)
    #[strum(serialize = "Prol")]
    Prol,
    /// fit: **@CODE@** = Pronomen
    /// sje: (no comment)
    /// gle: **@CODE@** = Pronoun
    /// kal: **@CODE@** = Pronoun
    /// kpv: **@CODE@**:  pronoun   нимвежтас   местоимение
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// bla: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Pronomen
    /// izh: **@CODE@**
    /// liv: **@CODE@** = pronoun
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = pronouns
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** = Pronomen
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: **@CODE@** -  Pronoun
    /// nob: **@CODE@** =
    /// got: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Pronouns
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// otw: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Pronoun
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Pronoun
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@**
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = pronoun
    /// rup: (no comment)
    /// tat: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@**  pronoun
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: **@CODE@**:  pronoun
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Pronoun
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  pronoun
    /// ale: (no comment)
    /// rmf: **@CODE@** closed POS
    /// ceb: **@CODE@** -  Pronoun
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@** Pronoun
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Pron")]
    Pron,
    /// mpj: **@CODE@**
    #[strum(serialize = "Pron/Clt")]
    Pron_SLASH_Clt,
    /// moh: (no comment)
    #[strum(serialize = "PronEnd")]
    PronEnd,
    /// moh: (no comment)
    #[strum(serialize = "PronStart")]
    PronStart,
    /// epo: (no comment)
    #[strum(serialize = "Pronoun")]
    Pronoun,
    /// fit: **@CODE@** = Propernoun
    /// gle: **@CODE@** = Proper noun
    /// kal: **@CODE@** = Propernoun
    /// kpv: **@CODE@** proper
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Propernoun
    /// izh: **@CODE@** Types of nouns
    /// liv: **@CODE@** = proper nouns
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = Proper noun
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** =
    /// lut: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@** = Propernouns are tagged +N+Prop
    /// got: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Propernouns
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Propernoun
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// rus: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Proper noun
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** is a noun subtag
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = proper
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Propernoun
    /// smn: **@CODE@** - Propernoun
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  proper
    /// ale: (no comment)
    /// rmf: **@CODE@** POS subtags
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// fro: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
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
    /// cwd: (no comment)
    /// crk: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// crj: (no comment)
    /// som: (no comment)
    /// tkl: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Prox")]
    Prox,
    /// kpv: **@CODE@** ProprietiveMod, HabObjMod Весиг киясыс тӧдсаӧсь, найӧ мугов рӧмаӧсь, кузь чорыд чуньясаӧсь.
    /// apu: **@CODE@** ka- vido verbs from nouns
    #[strum(serialize = "Prp")]
    Prp,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Presens
    /// liv: **@CODE@**
    /// crk: (no comment)
    /// mhr: **@CODE@** = present
    /// evn: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@** | Presens
    /// lut: (no comment)
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// chr: (no comment)
    /// crj: (no comment)
    /// cor: (no comment)
    /// sms: **@CODE@** Present
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// ess: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// grn: (no comment)
    /// olo: **@CODE@** | Present, non-past Tense
    /// koi: **@CODE@**:  present      -ӧ
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Present Tense
    /// smn: **@CODE@**
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ces: (no comment)
    /// fro: (no comment)
    /// vro: **@CODE@**
    /// ipk: (no comment)
    /// mpj: **@CODE@**  Present Tense
    /// iku: (no comment)
    #[strum(serialize = "Prs")]
    Prs,
    /// rue: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "PrsAct")]
    PrsAct,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "PrsImp")]
    PrsImp,
    /// fit: **@CODE@** = presens participe
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// fkv: **@CODE@** = Presence Particip
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Infinitive and participles
    /// lut: (no comment)
    /// smj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** : present participle
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Present Participe
    /// smn: **@CODE@**
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "PrsPrc")]
    PrsPrc,
    /// rmg: **@CODE@**
    #[strum(serialize = "PrsPrt")]
    PrsPrt,
    /// kpv: **@CODE@**
    /// rmg: **@CODE@** TODO fix Ptc vs. Prt
    #[strum(serialize = "PrsPtc")]
    PrsPtc,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Preteritum
    /// liv: **@CODE@**
    /// crk: (no comment)
    /// evn: (no comment)
    /// sma: **@CODE@** | Preteritum
    /// smj: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// crj: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** Preterite
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** | Preterite Tense
    /// pad: (no comment)
    /// eus: (no comment)
    /// sme: **@CODE@** Past Tense, Preterite
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Prt")]
    Prt,
    /// mhr: **@CODE@** = 1st preterite, direct observation
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// koi: **@CODE@**:  preterite 1 -ис
    /// mdf: (no comment)
    #[strum(serialize = "Prt1")]
    Prt1,
    /// mhr: **@CODE@** = 2nd preterite, indirect narrative, conclusion
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// koi: **@CODE@**:  preterite 2 -ӧма
    /// mdf: (no comment)
    #[strum(serialize = "Prt2")]
    Prt2,
    /// udm: (no comment)
    #[strum(serialize = "PrtI")]
    PrtI,
    /// udm: (no comment)
    #[strum(serialize = "PrtII")]
    PrtII,
    /// apu: **@CODE@** -takute 'in the proximity of'
    #[strum(serialize = "Prx")]
    Prx,
    /// liv: **@CODE@** = passive
    /// got: (no comment)
    /// apu: (no comment)
    /// vep: **@CODE@**  = passive voice
    /// olo: **@CODE@** : passive voice
    /// fin: (no comment)
    /// mdf: **@CODE@**:  passive
    /// vro: **@CODE@** passive
    #[strum(serialize = "Pss")]
    Pss,
    /// kmr: (no comment)
    /// rue: (no comment)
    /// skf: (no comment)
    /// rus: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Pst")]
    Pst,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "PstAct")]
    PstAct,
    /// vep: **@CODE@**  =
    #[strum(serialize = "PstI")]
    PstI,
    /// vep: **@CODE@**  =
    #[strum(serialize = "PstII")]
    PstII,
    /// lut: (no comment)
    /// slh: (no comment)
    /// cor: (no comment)
    /// eus: (no comment)
    #[strum(serialize = "Ptc")]
    Ptc,
    /// hdn: (no comment)
    #[strum(serialize = "Ptcl")]
    Ptcl,
    /// gle: **@CODE@** = Abbreviation
    /// fin: (no comment)
    #[strum(serialize = "Punct")]
    Punct,
    /// xak: (no comment)
    /// mpj: **@CODE@**  = Purposive
    #[strum(serialize = "Purp")]
    Purp,
    /// khk: (no comment)
    #[strum(serialize = "Px")]
    Px,
    /// kpv: **@CODE@**
    /// koi: (no comment)
    #[strum(serialize = "Px1")]
    Px1,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Px12Pl")]
    Px12Pl,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Px1Pl")]
    Px1Pl,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Px1Sg")]
    Px1Sg,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "Px21Pl")]
    Px21Pl,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Px2Pl")]
    Px2Pl,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Px2Sg")]
    Px2Sg,
    /// bxr: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Px3")]
    Px3,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Px3Pl")]
    Px3Pl,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Px3Sg")]
    Px3Sg,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Px4Pl")]
    Px4Pl,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Px4Sg")]
    Px4Sg,
    /// bla: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "Px4Sg/Pl")]
    Px4Sg_SLASH_Pl,
    /// srs: (no comment)
    #[strum(serialize = "PxAreal")]
    PxAreal,
    /// sme: **@CODE@** used in pronoun-sme-morph.txt
    #[strum(serialize = "PxCPlComRecipr")]
    PxCPlComRecipr,
    /// sje: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** =  Possessives Dual
    /// smj: **@CODE@** possessive suffix dual first person
    /// yrk: **@CODE@**
    /// lit: (no comment)
    /// olo: (no comment)
    /// sme: **@CODE@**    Dual First Person
    /// smn: **@CODE@**
    #[strum(serialize = "PxDu1")]
    PxDu1,
    /// sje: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** =  Possessives Dual
    /// smj: **@CODE@** possessive suffix dual second person
    /// yrk: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@**    Dual Second Person
    #[strum(serialize = "PxDu2")]
    PxDu2,
    /// sje: (no comment)
    /// vot: (no comment)
    /// sma: **@CODE@** =  Possessives Dual
    /// smj: **@CODE@** possessive suffix dual third person
    /// yrk: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@**    Dual Third Person
    #[strum(serialize = "PxDu3")]
    PxDu3,
    /// srs: (no comment)
    #[strum(serialize = "PxGiven")]
    PxGiven,
    /// srs: (no comment)
    #[strum(serialize = "PxIndef")]
    PxIndef,
    /// pma: (no comment)
    #[strum(serialize = "PxPauc1")]
    PxPauc1,
    /// pma: (no comment)
    #[strum(serialize = "PxPauc2")]
    PxPauc2,
    /// pma: (no comment)
    #[strum(serialize = "PxPauc3")]
    PxPauc3,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Plural 1
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** =  Possessives Plural
    /// lut: (no comment)
    /// smj: **@CODE@** possessive suffix plural first person
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: **@CODE@** Plural First Person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// lit: (no comment)
    /// myu: **@CODE@** Plural First Person Exclusive
    /// vep: **@CODE@**  = -moi
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// pad: **@CODE@** Plural First Person
    /// deu: (no comment)
    /// sme: **@CODE@**    Plural First Person
    /// smn: **@CODE@**
    /// srs: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  first person plural
    /// khk: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "PxPl1")]
    PxPl1,
    /// evn: (no comment)
    /// myu: **@CODE@** Plural First Person Inclusive
    #[strum(serialize = "PxPl12")]
    PxPl12,
    /// skf: (no comment)
    /// som: (no comment)
    #[strum(serialize = "PxPl1Excl")]
    PxPl1Excl,
    /// wac: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "PxPl1Inc")]
    PxPl1Inc,
    /// skf: (no comment)
    /// som: (no comment)
    #[strum(serialize = "PxPl1Incl")]
    PxPl1Incl,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Plural 2
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** =  Possessives Plural
    /// lut: (no comment)
    /// smj: **@CODE@** possessive suffix plural second person
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: **@CODE@** Plural Second Person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// myu: **@CODE@** Plural Second Person
    /// vep: **@CODE@**  = -toi
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// pad: **@CODE@** Plural Second Person
    /// deu: (no comment)
    /// sme: **@CODE@**    Plural Second Person
    /// srs: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  second person plural
    /// khk: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "PxPl2")]
    PxPl2,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Plural 3
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** =  Possessives Plural
    /// smj: **@CODE@** possessive suffix plural plural person
    /// udm: (no comment)
    /// skf: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// myu: **@CODE@** Plural Third Person
    /// vep: **@CODE@**  = -ze
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@**    Plural Third Person
    /// srs: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  third person plural
    /// tlh: (no comment)
    #[strum(serialize = "PxPl3")]
    PxPl3,
    /// apu: **@CODE@** Plural Third Person Feminine
    /// pad: **@CODE@** Plural Third Person Feminine
    #[strum(serialize = "PxPl3F")]
    PxPl3F,
    /// apu: **@CODE@** Plural Third Person Male
    /// pad: **@CODE@** Plural Third Person Male
    #[strum(serialize = "PxPl3M")]
    PxPl3M,
    /// skf: (no comment)
    #[strum(serialize = "PxPl3c")]
    PxPl3c,
    /// srs: (no comment)
    #[strum(serialize = "PxRecip")]
    PxRecip,
    /// krl: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    #[strum(serialize = "PxSP3")]
    PxSP3,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Singular 1
    /// izh: **@CODE@** Possessive suffixes
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** =  Possessives Singular
    /// lut: (no comment)
    /// sjt: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** possessive suffix singular first person
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Singular First Person
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** for possessive suffixes
    /// sms: **@CODE@**
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: (no comment)
    /// ndl: (no comment)
    /// myu: **@CODE@** Singular First Person
    /// vep: **@CODE@**  = -in
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// grn: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: **@CODE@** Singular First Person
    /// deu: (no comment)
    /// sme: **@CODE@**    Singular First Person
    /// smn: **@CODE@**
    /// srs: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  first person singular
    /// ale: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "PxSg1")]
    PxSg1,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Singular 2
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** =  Possessives Singular
    /// lut: (no comment)
    /// smj: **@CODE@** possessive suffix singular second person
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: **@CODE@** Singular Second Person
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// myu: **@CODE@** Singular Second Person
    /// vep: **@CODE@**  =  -iž
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// pad: **@CODE@** Singular Second Person
    /// deu: (no comment)
    /// sme: **@CODE@**    Singular Second Person
    /// srs: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  second person singular
    /// khk: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "PxSg2")]
    PxSg2,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Poss suff: the owner is Singular 3
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** =  Possessives Singular
    /// smj: **@CODE@** possessive suffix singular third person
    /// udm: (no comment)
    /// skf: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// bxr: (no comment)
    /// myu: **@CODE@** Singular Third Person
    /// vep: **@CODE@**  = -ze
    /// myv: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@**    Singular Third Person
    /// srs: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  third person singular
    /// tlh: (no comment)
    #[strum(serialize = "PxSg3")]
    PxSg3,
    /// apu: **@CODE@** Singular Third Person Feminine
    /// som: (no comment)
    /// pad: **@CODE@** Singular Third Person Feminine
    #[strum(serialize = "PxSg3F")]
    PxSg3F,
    /// apu: **@CODE@** Singular Third Person Male
    /// som: (no comment)
    /// wac: (no comment)
    /// pad: **@CODE@** Singular Third Person Male
    /// tqn: (no comment)
    #[strum(serialize = "PxSg3M")]
    PxSg3M,
    /// skf: (no comment)
    /// myu: **@CODE@** Singular Third Person correlating ? reflexive
    #[strum(serialize = "PxSg3c")]
    PxSg3c,
    /// srs: (no comment)
    #[strum(serialize = "PxSg4")]
    PxSg4,
    /// khk: (no comment)
    #[strum(serialize = "PxSp3")]
    PxSp3,
    /// cwd: (no comment)
    /// crk: (no comment)
    #[strum(serialize = "PxX")]
    PxX,
    /// gle: **@CODE@** = Interrogative particle(q)
    /// cho: (no comment)
    /// som: (no comment)
    #[strum(serialize = "Q")]
    Q,
    /// kal: **@CODE@**
    #[strum(serialize = "QAR")]
    QAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QATE")]
    QATE,
    /// kal: **@CODE@**
    #[strum(serialize = "QATIGIIP")]
    QATIGIIP,
    /// kal: **@CODE@**
    #[strum(serialize = "QATIGIIT")]
    QATIGIIT,
    /// kal: **@CODE@**
    #[strum(serialize = "QATTAAR")]
    QATTAAR,
    /// kio: (no comment)
    #[strum(serialize = "QDL")]
    QDL,
    /// kal: **@CODE@**
    #[strum(serialize = "QE")]
    QE,
    /// kal: **@CODE@**
    #[strum(serialize = "QINA")]
    QINA,
    /// vot: (no comment)
    #[strum(serialize = "QNT")]
    QNT,
    /// kal: **@CODE@**
    #[strum(serialize = "QQA")]
    QQA,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAAQ")]
    QQAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAAR")]
    QQAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAJAA")]
    QQAJAA,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAJAR")]
    QQAJAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAMMI")]
    QQAMMI,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAMMIQ")]
    QQAMMIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAMMIR")]
    QQAMMIR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQAR")]
    QQAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQARTUR")]
    QQARTUR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQATAR")]
    QQATAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQATUR")]
    QQATUR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQINNAAQ")]
    QQINNAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "QQINNAAR")]
    QQINNAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQIP")]
    QQIP,
    /// kal: **@CODE@**
    #[strum(serialize = "QQISSAAQ")]
    QQISSAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "QQISSAAR")]
    QQISSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQU")]
    QQU,
    /// kal: **@CODE@**
    #[strum(serialize = "QQURTU")]
    QQURTU,
    /// kal: **@CODE@**
    #[strum(serialize = "QQURTUSI")]
    QQURTUSI,
    /// kal: **@CODE@**
    #[strum(serialize = "QQURTUUQ")]
    QQURTUUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "QQUSAAR")]
    QQUSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQUTE")]
    QQUTE,
    /// kal: **@CODE@**
    #[strum(serialize = "QQUUQE")]
    QQUUQE,
    /// kal: **@CODE@**
    #[strum(serialize = "QQUUR")]
    QQUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "QQUUTE")]
    QQUUTE,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "QUOT")]
    QUOT,
    /// kpv: **@CODE@**:  Quantifier   ХХ   XX
    /// swe: (no comment)
    /// vot: (no comment)
    /// liv: **@CODE@** = quantifier
    /// mhr: **@CODE@** = quantifiers
    /// lut: (no comment)
    /// nob: **@CODE@** = quantifier noen, begge
    /// udm: (no comment)
    /// otw: (no comment)
    /// slh: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@** - SHOULD THIS be here or a Sem/Qnt
    /// rmn: (no comment)
    /// vep: **@CODE@**  = quantifier
    /// tat: (no comment)
    /// tat: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@**  quantifier
    /// koi: **@CODE@**:  Quantifier
    /// deu: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  Quantifier
    /// khk: (no comment)
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
    /// pur: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Focusclitic question -ko
    /// izh: **@CODE@** yes/no question
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// krl: (no comment)
    /// xal: (no comment)
    /// lut: (no comment)
    /// lut: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@**
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** Question and Focus particles
    /// mns: **@CODE@** Question particle -а
    /// mns: (no comment)
    /// sms: **@CODE@** Question particle -a
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// grn: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// sme: **@CODE@**   Question Particle: +Pcle+Qst
    /// smn: **@CODE@** question particle
    /// gur: (no comment)
    /// aym: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@**
    /// rmf: (no comment)
    /// ceb: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
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
    /// chp: (no comment)
    #[strum(serialize = "Qt")]
    Qt,
    /// gle: **@CODE@** = Quantifier
    #[strum(serialize = "Qty")]
    Qty,
    /// fit: **@CODE@** = Hmm, Question?? Interr? Check this.
    /// fkv: **@CODE@** = Quantity
    /// fin: (no comment)
    #[strum(serialize = "Qu")]
    Qu,
    /// epo: (no comment)
    #[strum(serialize = "Qual")]
    Qual,
    /// epo: (no comment)
    #[strum(serialize = "Quest")]
    Quest,
    /// gle: **@CODE@** = all quotation marks double, single etc.
    /// liv: **@CODE@** = quotative, quoted speech
    /// skf: (no comment)
    /// vro: **@CODE@** = quotative, quoted speech
    #[strum(serialize = "Quo")]
    Quo,
    /// fin: (no comment)
    #[strum(serialize = "Quote")]
    Quote,
    /// kal: **@CODE@**
    /// epo: (no comment)
    #[strum(serialize = "R")]
    R,
    /// kal: **@CODE@**
    #[strum(serialize = "RAALLAK")]
    RAALLAK,
    /// kal: **@CODE@**
    #[strum(serialize = "RAAR")]
    RAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RAJUK")]
    RAJUK,
    /// kal: **@CODE@**
    #[strum(serialize = "RALAAQ")]
    RALAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "RALAK")]
    RALAK,
    /// kal: **@CODE@**
    #[strum(serialize = "RAR")]
    RAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RASAAR")]
    RASAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RATAAR")]
    RATAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RATAR")]
    RATAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RIAANNAA")]
    RIAANNAA,
    /// kal: **@CODE@**
    #[strum(serialize = "RIAAT")]
    RIAAT,
    /// kal: **@CODE@**
    #[strum(serialize = "RIANNGUAR")]
    RIANNGUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RIAQ")]
    RIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "RIAR")]
    RIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RIATAAR")]
    RIATAAR,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// gle: **@CODE@** = Right side of parwise symbol (parenthesis or quotation mark)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = right parenth
    /// liv: **@CODE@**
    /// crk: (no comment)
    /// mhr: **@CODE@** = paired symbols
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// sma: **@CODE@**:  XXX These should be documented better
    /// tgl: (no comment)
    /// hdn: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = right paranthesis
    /// bxr: (no comment)
    /// vep: **@CODE@**  =
    /// cho: (no comment)
    /// cho: (no comment)
    /// som: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@**
    /// xak: (no comment)
    /// sme: **@CODE@**  right paranthesis
    /// srs: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// ceb: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "RIGHT")]
    RIGHT,
    /// kal: **@CODE@**
    #[strum(serialize = "RIIR")]
    RIIR,
    /// kal: **@CODE@**
    #[strum(serialize = "RIP")]
    RIP,
    /// kal: **@CODE@**
    #[strum(serialize = "RLAAQ")]
    RLAAQ,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "RPAR")]
    RPAR,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "RQUOT")]
    RQUOT,
    /// kal: **@CODE@**
    #[strum(serialize = "RRAK")]
    RRAK,
    /// kal: **@CODE@**
    #[strum(serialize = "RRATE")]
    RRATE,
    /// apu: (no comment)
    #[strum(serialize = "RS")]
    RS,
    /// kal: **@CODE@**
    #[strum(serialize = "RSUR")]
    RSUR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUJUK")]
    RUJUK,
    /// kal: **@CODE@**
    #[strum(serialize = "RUJUP")]
    RUJUP,
    /// kal: **@CODE@**
    #[strum(serialize = "RUJUUJUTAA")]
    RUJUUJUTAA,
    /// kal: **@CODE@**
    #[strum(serialize = "RUJUUR")]
    RUJUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUJUURUTAA")]
    RUJUURUTAA,
    /// kal: **@CODE@**
    #[strum(serialize = "RULUP")]
    RULUP,
    /// kal: **@CODE@**
    #[strum(serialize = "RULUUR")]
    RULUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUR")]
    RUR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUSAAR")]
    RUSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUSAQ")]
    RUSAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "RUTTUR")]
    RUTTUR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUUSAAR")]
    RUUSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "RUUSAQ")]
    RUUSAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "RUUSAR")]
    RUUSAR,
    /// nob: **@CODE@**  For dictionary testing, Radical Bokmål
    #[strum(serialize = "Radical")]
    Radical,
    /// apu: (no comment)
    #[strum(serialize = "Random")]
    Random,
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
    /// tgl: **@CODE@** -
    /// ceb: **@CODE@** -
    #[strum(serialize = "Rcmp")]
    Rcmp,
    /// xak: (no comment)
    #[strum(serialize = "RcntFut")]
    RcntFut,
    /// xak: (no comment)
    #[strum(serialize = "RcntPst")]
    RcntPst,
    /// tgl: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Rcp")]
    Rcp,
    /// mya: (no comment)
    #[strum(serialize = "Realis")]
    Realis,
    /// epo: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Reas")]
    Reas,
    /// rue: (no comment)
    /// udm: (no comment)
    /// rus: (no comment)
    /// mdf: **@CODE@**:  reciprocal
    /// khk: (no comment)
    #[strum(serialize = "Recip")]
    Recip,
    /// ciw: (no comment)
    #[strum(serialize = "Recip/di")]
    Recip_SLASH_di,
    /// fit: **@CODE@** = Reciprocal
    /// sje: (no comment)
    /// kpv: **@CODE@**:  reciprocal
    /// vot: (no comment)
    /// fkv: **@CODE@** = Reciprocal
    /// liv: **@CODE@** = reciprocal
    /// mhr: **@CODE@** = Reciprocal pronoun
    /// sma: **@CODE@** = Reciprocal
    /// hdn: (no comment)
    /// smj: **@CODE@** = reciprocal pronoun
    /// udm: (no comment)
    /// apu: **@CODE@** Reciprocal Pronoun
    /// apu: (no comment)
    /// chr: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Reciprocal Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  reciprocal
    /// vep: **@CODE@**  = reciprocal
    /// myv: (no comment)
    /// olo: **@CODE@** reciprocal
    /// koi: **@CODE@**:  reciprocal
    /// sme: **@CODE@** - Reciprocal Pronoun
    /// smn: **@CODE@** - Reciprocal Pronoun
    /// fin: (no comment)
    /// mdf: **@CODE@**:  reciprocal
    /// vro: **@CODE@**
    #[strum(serialize = "Recipr")]
    Recipr,
    /// tlh: (no comment)
    #[strum(serialize = "Reciproc")]
    Reciproc,
    /// som: (no comment)
    #[strum(serialize = "Recit")]
    Recit,
    /// som: (no comment)
    #[strum(serialize = "Red")]
    Red,
    /// mpj: **@CODE@**
    #[strum(serialize = "Redpl")]
    Redpl,
    /// gle: **@CODE@** = Reflexive particle
    /// khk: (no comment)
    #[strum(serialize = "Ref")]
    Ref,
    /// fit: **@CODE@** = Reflexive
    /// sje: (no comment)
    /// kpv: **@CODE@**:  reflexive
    /// vot: (no comment)
    /// fkv: **@CODE@** = Reflexive
    /// liv: **@CODE@** = reflexive
    /// mhr: **@CODE@** = Reflexive pronoun
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Reflexive
    /// hdn: (no comment)
    /// smj: **@CODE@** = reflexive pronoun
    /// udm: (no comment)
    /// apu: **@CODE@** Reflexive Pronoun
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@** reflexive
    /// mns: **@CODE@** this is also used for +Nom intensfier
    /// sms: **@CODE@** - Reflexive Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  reflexive
    /// vep: **@CODE@**  = reflexive
    /// myv: (no comment)
    /// olo: **@CODE@** reflexive
    /// koi: **@CODE@**:  reflexive
    /// deu: (no comment)
    /// sme: **@CODE@** - Reflexive Pronoun
    /// smn: **@CODE@** - Reflexive Pronoun
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  reflexive
    /// khk: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Refl")]
    Refl,
    /// yrk: **@CODE@** referential adverbs
    /// yrk: **@CODE@** (referential),
    #[strum(serialize = "Refr")]
    Refr,
    /// fit: **@CODE@** = Relative
    /// sje: (no comment)
    /// gle: **@CODE@** = Relative particle
    /// kal: **@CODE@** = Relative
    /// kpv: **@CODE@**:  relative
    /// vot: (no comment)
    /// fkv: **@CODE@** = Relative
    /// liv: **@CODE@** = relative
    /// mhr: **@CODE@** = Relative pronoun
    /// rue: (no comment)
    /// sma: **@CODE@** = Relative
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// smj: **@CODE@** = relative pronoun
    /// udm: (no comment)
    /// apu: **@CODE@** Relative Pronoun
    /// chr: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** - Relative Pronoun
    /// lit: (no comment)
    /// mrj: **@CODE@**:  relative
    /// vep: **@CODE@**  = relative
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** relative
    /// koi: **@CODE@**:  relative
    /// deu: (no comment)
    /// sme: **@CODE@** - Relative Pronoun
    /// smn: **@CODE@** - Relative Pronoun
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  relative
    /// vro: **@CODE@**
    #[strum(serialize = "Rel")]
    Rel,
    /// gle: **@CODE@** = rel. indirect
    #[strum(serialize = "RelInd")]
    RelInd,
    /// kpv: **@CODE@** relational noun: выв, ув
    /// skf: (no comment)
    /// myv: (no comment)
    /// koi: **@CODE@** relational noun: выв, ув
    /// mdf: **@CODE@**:  relator nouns, mainly meronyms
    #[strum(serialize = "Relat")]
    Relat,
    /// apu: (no comment)
    #[strum(serialize = "Relzr-Obj")]
    Relzr_MINUS_Obj,
    /// apu: (no comment)
    #[strum(serialize = "Relzr-Subj")]
    Relzr_MINUS_Subj,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "RemPast")]
    RemPast,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "RemPst")]
    RemPst,
    /// ceb: **@CODE@** - Remote deixis
    /// ceb: (no comment)
    #[strum(serialize = "Remt")]
    Remt,
    /// skf: (no comment)
    #[strum(serialize = "Rep")]
    Rep,
    /// hdn: (no comment)
    /// chr: (no comment)
    #[strum(serialize = "RepPast")]
    RepPast,
    /// hdn: (no comment)
    #[strum(serialize = "Res")]
    Res,
    /// bxr: (no comment)
    #[strum(serialize = "ResPrc")]
    ResPrc,
    /// som: (no comment)
    #[strum(serialize = "Restr")]
    Restr,
    /// olo: **@CODE@** : This is a work around for olo passive. Olo has a passive conjugation,
    #[strum(serialize = "Rfl")]
    Rfl,
    /// hdn: (no comment)
    #[strum(serialize = "Rflx")]
    Rflx,
    /// hdn: (no comment)
    /// hdn: (no comment)
    #[strum(serialize = "Rfx")]
    Rfx,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Roman numerals I, II, ...
    /// crk: (no comment)
    /// mhr: **@CODE@** = roman numerals
    /// sma: **@CODE@** = Roman numeral
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = remertall
    /// apu: **@CODE@** Roman numeral, subtag for +Num
    /// mns: **@CODE@** - Roman numeral
    /// sms: **@CODE@** - Roman numeral
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Roman numeral, subtag for +Num
    /// smn: **@CODE@** - Roman numeral, subtag for +Num
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Rom")]
    Rom,
    /// fro: (no comment)
    #[strum(serialize = "Roman")]
    Roman,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Route")]
    Route,
    /// khk: (no comment)
    #[strum(serialize = "Rpar")]
    Rpar,
    /// khk: (no comment)
    #[strum(serialize = "Rquot")]
    Rquot,
    /// tgl: (no comment)
    /// ceb: (no comment)
    #[strum(serialize = "Rsn")]
    Rsn,
    /// yrk: **@CODE@** (100% Russian homograph)
    #[strum(serialize = "Rus")]
    Rus,
    /// udm: (no comment)
    #[strum(serialize = "RusV")]
    RusV,
    /// kal: **@CODE@**
    #[strum(serialize = "SAAR")]
    SAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SAR")]
    SAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SARE")]
    SARE,
    /// nso: (no comment)
    #[strum(serialize = "SC")]
    SC,
    /// sje: (no comment)
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "SENT")]
    SENT,
    /// kal: **@CODE@**
    #[strum(serialize = "SI")]
    SI,
    /// kal: **@CODE@**
    #[strum(serialize = "SIAQ")]
    SIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "SII")]
    SII,
    /// kal: **@CODE@**
    #[strum(serialize = "SILAT")]
    SILAT,
    /// kal: **@CODE@**
    #[strum(serialize = "SIMA")]
    SIMA,
    /// kal: **@CODE@**
    #[strum(serialize = "SIMAAR")]
    SIMAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SINNAA")]
    SINNAA,
    /// kal: **@CODE@**
    #[strum(serialize = "SINNAANNGUR")]
    SINNAANNGUR,
    /// kal: **@CODE@**
    #[strum(serialize = "SINNAAQ")]
    SINNAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "SINNAR")]
    SINNAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SIQ")]
    SIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "SIR")]
    SIR,
    /// kal: **@CODE@**
    #[strum(serialize = "SIUR")]
    SIUR,
    /// kal: **@CODE@**
    #[strum(serialize = "SIUT")]
    SIUT,
    /// myv: (no comment)
    #[strum(serialize = "SLoss")]
    SLoss,
    /// myv: (no comment)
    /// mdf: **@CODE@**:  both singular and plural
    /// vro: **@CODE@** Singular and Plural, used for CG and Apertium
    #[strum(serialize = "SP")]
    SP,
    /// deu: (no comment)
    #[strum(serialize = "SP2")]
    SP2,
    /// vot: (no comment)
    #[strum(serialize = "SPAT")]
    SPAT,
    /// kal: **@CODE@**
    #[strum(serialize = "SSA")]
    SSA,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAALIQI")]
    SSAALIQI,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAAR")]
    SSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAMAAQ")]
    SSAMAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAMAAR")]
    SSAMAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAMAATE")]
    SSAMAATE,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAQ")]
    SSAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "SSAQQIP")]
    SSAQQIP,
    /// kal: **@CODE@**
    #[strum(serialize = "SSI")]
    SSI,
    /// kal: **@CODE@**
    #[strum(serialize = "SSIP")]
    SSIP,
    /// kal: **@CODE@**
    #[strum(serialize = "SSUSIQ")]
    SSUSIQ,
    /// vot: (no comment)
    #[strum(serialize = "STATE")]
    STATE,
    /// kal: **@CODE@**
    #[strum(serialize = "SUAQ")]
    SUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "SUAR")]
    SUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SUATAAR")]
    SUATAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SUNGAR")]
    SUNGAR,
    /// kal: **@CODE@**
    #[strum(serialize = "SUNNI")]
    SUNNI,
    /// kal: **@CODE@**
    #[strum(serialize = "SUNNIP")]
    SUNNIP,
    /// kal: **@CODE@**
    #[strum(serialize = "SUR")]
    SUR,
    /// kal: **@CODE@**
    #[strum(serialize = "SURE")]
    SURE,
    /// gle: **@CODE@** =  Subject pronouns: sí, sé and siad are used only when pron follows predicate verb in  subject position, otherwise í, é and iad are used.
    /// bla: (no comment)
    /// got: (no comment)
    /// crj: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "Sbj")]
    Sbj,
    /// srs: (no comment)
    #[strum(serialize = "Sbj3")]
    Sbj3,
    /// srs: (no comment)
    #[strum(serialize = "SbjPl3")]
    SbjPl3,
    /// srs: (no comment)
    #[strum(serialize = "SbjSg3")]
    SbjSg3,
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sc")]
    Sc,
    /// kca: (no comment)
    #[strum(serialize = "Sc1Du")]
    Sc1Du,
    /// kca: (no comment)
    #[strum(serialize = "Sc1Pl")]
    Sc1Pl,
    /// kca: (no comment)
    #[strum(serialize = "Sc1Sg")]
    Sc1Sg,
    /// apu: (no comment)
    #[strum(serialize = "Sc3M")]
    Sc3M,
    /// lut: (no comment)
    #[strum(serialize = "ScCon")]
    ScCon,
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
    /// luo: (no comment)
    /// apu: **@CODE@** subject conjugation first person plural
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// olo: (no comment)
    /// pad: **@CODE@** subject conjugation first person plural
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScPl1")]
    ScPl1,
    /// luo: (no comment)
    /// apu: **@CODE@** subject conjugation second person plural
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// pad: **@CODE@** subject conjugation second person plural
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScPl2")]
    ScPl2,
    /// luo: (no comment)
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// pad: **@CODE@** subject conjugation third person plural
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScPl3")]
    ScPl3,
    /// apu: **@CODE@** subject conjugation third person plural Feminine
    /// pad: **@CODE@** subject conjugation third person plural Feminine
    #[strum(serialize = "ScPl3F")]
    ScPl3F,
    /// apu: **@CODE@** subject conjugation third person plural Male
    /// pad: **@CODE@** subject conjugation third person plural Male
    #[strum(serialize = "ScPl3M")]
    ScPl3M,
    /// vep: **@CODE@**
    #[strum(serialize = "ScSg")]
    ScSg,
    /// pur: **@CODE@** subject conjugation first person singular on verb
    /// luo: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** subject conjugation first person singular
    /// yrk: **@CODE@**
    /// mns: **@CODE@** Verb subject conjugation
    /// myv: (no comment)
    /// olo: (no comment)
    /// pad: **@CODE@** subject conjugation first person singular
    /// mdf: (no comment)
    /// vro: **@CODE@**
    /// tqn: (no comment)
    #[strum(serialize = "ScSg1")]
    ScSg1,
    /// pur: **@CODE@** subject conjugation second person singular
    /// luo: (no comment)
    /// apu: **@CODE@** subject conjugation second person singular
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// pad: **@CODE@** subject conjugation second person singular
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScSg2")]
    ScSg2,
    /// pur: **@CODE@** subject conjugation third person singular
    /// luo: (no comment)
    /// yrk: **@CODE@**
    /// myv: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "ScSg3")]
    ScSg3,
    /// apu: **@CODE@** subject conjugation third person singular Feminine
    /// pad: **@CODE@** subject conjugation third person singular Feminine
    #[strum(serialize = "ScSg3F")]
    ScSg3F,
    /// apu: **@CODE@** subject conjugation third person singular Male
    /// pad: **@CODE@** subject conjugation third person singular Male
    #[strum(serialize = "ScSg3M")]
    ScSg3M,
    /// myv: (no comment)
    #[strum(serialize = "Sel")]
    Sel,
    /// kpv: **@CODE@** Activity
    /// izh: **@CODE@** Activity
    /// liv: **@CODE@** Activity
    /// mhr: **@CODE@** = Activity
    /// krl: **@CODE@** Activity
    /// sma: (no comment)
    /// smj: **@CODE@** = Activity; cleaning, work, occupation, project, photosynthesis
    /// udm: **@CODE@** Activity
    /// yrk: **@CODE@** Activity
    /// sms: **@CODE@** = Activity
    /// mrj: **@CODE@** Activity
    /// myv: (no comment)
    /// olo: **@CODE@** Activity
    /// koi: **@CODE@** Activity
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Activity
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Act_Fruit")]
    Sem_SLASH_Act_Fruit,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@** Activity and Group
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Act_Group")]
    Sem_SLASH_Act_Group,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** = A persons job is an activity, and a place as well
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**   A persons job is an activity, and a place as well
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** = Activity and Route, ie johtolat
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@** Activity and Route, ie johtolat
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Act_Tool-it")]
    Sem_SLASH_Act_Tool_MINUS_it,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Act_Txt")]
    Sem_SLASH_Act_Txt,
    /// sma: (no comment)
    /// smj: **@CODE@** = Webadr
    /// sms: **@CODE@** = Webadr
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Adr")]
    Sem_SLASH_Adr,
    /// kal: **@CODE@** =  Animal, water
    #[strum(serialize = "Sem/Aich")]
    Sem_SLASH_Aich,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Amount
    /// swe: (no comment)
    /// fkv: (no comment)
    /// izh: **@CODE@** Amount
    /// liv: **@CODE@** Amount
    /// mhr: **@CODE@** = Amount
    /// krl: **@CODE@** Amount
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Amount; pile, amount of lynx, amount of food, one fifteenth, ten percent
    /// udm: **@CODE@** Amount
    /// yrk: **@CODE@** Amount
    /// sms: **@CODE@** = Amount
    /// rmn: (no comment)
    /// mrj: **@CODE@** Amount
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Amount
    /// koi: **@CODE@** Amount
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Amount
    /// vro: **@CODE@** Amount
    /// bak: (no comment)
    #[strum(serialize = "Sem/Amount")]
    Sem_SLASH_Amount,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**   Amount and Building
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@**   Amount and Building
    /// smn: **@CODE@**   Amount and Building
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Amount_Build")]
    Sem_SLASH_Amount_Build,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Amount_Semcon")]
    Sem_SLASH_Amount_Semcon,
    /// fit: **@CODE@** = Animal names
    /// kpv: **@CODE@** Animate
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Animal names
    /// izh: **@CODE@** Animate
    /// liv: **@CODE@** Animate
    /// mhr: **@CODE@** = Animate
    /// krl: **@CODE@** Animate
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// smj: **@CODE@** = Animate; dog, reindeer, teddy bear,ndragon, lice
    /// udm: **@CODE@** Animate
    /// nio: (no comment)
    /// yrk: **@CODE@** Animate
    /// mns: (no comment)
    /// sms: **@CODE@** Animate       (names)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Animate
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Animate
    /// koi: **@CODE@** Animate
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Animate
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Animate
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Ani")]
    Sem_SLASH_Ani,
    /// smn: (no comment)
    #[strum(serialize = "Sem/Ani-bird")]
    Sem_SLASH_Ani_MINUS_bird,
    /// sma: (no comment)
    /// smj: **@CODE@** = Fish
    /// sms: **@CODE@** Animate
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Ani-fish")]
    Sem_SLASH_Ani_MINUS_fish,
    /// smn: (no comment)
    #[strum(serialize = "Sem/Ani-insect")]
    Sem_SLASH_Ani_MINUS_insect,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ani_Body")]
    Sem_SLASH_Ani_Body,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Body-abstr_Hum")]
    Sem_SLASH_Ani_Body_MINUS_abstr_Hum,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Build")]
    Sem_SLASH_Ani_Build,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Build-part")]
    Sem_SLASH_Ani_Build_MINUS_part,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Ani_Build_Hum_Obj-clo_Txt")]
    Sem_SLASH_Ani_Build_Hum_Obj_MINUS_clo_Txt,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Build_Hum_Txt")]
    Sem_SLASH_Ani_Build_Hum_Txt,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Group")]
    Sem_SLASH_Ani_Group,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Group_Hum")]
    Sem_SLASH_Ani_Group_Hum,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Ani_Group_Prod-vis")]
    Sem_SLASH_Ani_Group_Prod_MINUS_vis,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Hum")]
    Sem_SLASH_Ani_Hum,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Hum_Plc")]
    Sem_SLASH_Ani_Hum_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Plc")]
    Sem_SLASH_Ani_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Time")]
    Sem_SLASH_Ani_Time,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Ani_Tool")]
    Sem_SLASH_Ani_Tool,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ani_Veh")]
    Sem_SLASH_Ani_Veh,
    /// kpv: **@CODE@** Animal Product
    /// izh: **@CODE@** Animal Product
    /// liv: **@CODE@** Animal Product
    /// mhr: **@CODE@** = Animal Product
    /// krl: **@CODE@** Animal Product
    /// sma: (no comment)
    /// smj: **@CODE@** = Animal Product; sweat, reindeer skin, lice egg, blood for making dumplings, pee
    /// udm: **@CODE@** Animal Product
    /// yrk: **@CODE@** Animal Product
    /// sms: **@CODE@** = Animal Product
    /// mrj: **@CODE@** Animal Product
    /// myv: (no comment)
    /// olo: **@CODE@** Animal Product
    /// koi: **@CODE@** Animal Product
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Animal Product
    /// vro: **@CODE@** Animal Product
    #[strum(serialize = "Sem/Aniprod")]
    Sem_SLASH_Aniprod,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Obj-clo")]
    Sem_SLASH_Aniprod_Obj_MINUS_clo,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Perc-phys")]
    Sem_SLASH_Aniprod_Perc_MINUS_phys,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Plant")]
    Sem_SLASH_Aniprod_Plant,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Aniprod_Plc")]
    Sem_SLASH_Aniprod_Plc,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Aniprod_Plc_Route")]
    Sem_SLASH_Aniprod_Plc_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Substnc")]
    Sem_SLASH_Aniprod_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Aniprod_Substnc_Wthr")]
    Sem_SLASH_Aniprod_Substnc_Wthr,
    /// kpv: **@CODE@**  Anthroponym
    /// krl: (no comment)
    /// rue: (no comment)
    /// rus: (no comment)
    /// sms: **@CODE@** Anthroponym  (names)
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** Antroponym
    #[strum(serialize = "Sem/Ant")]
    Sem_SLASH_Ant,
    /// kpv: **@CODE@**  Anthroponym female
    /// krl: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** Female Antroponym
    #[strum(serialize = "Sem/Ant-Fem")]
    Sem_SLASH_Ant_MINUS_Fem,
    /// koi: (no comment)
    #[strum(serialize = "Sem/Ant-Fem-Sur")]
    Sem_SLASH_Ant_MINUS_Fem_MINUS_Sur,
    /// kpv: **@CODE@**  Anthroponym male
    /// krl: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** Male Antroponym
    #[strum(serialize = "Sem/Ant-Mal")]
    Sem_SLASH_Ant_MINUS_Mal,
    /// kpv: **@CODE@**
    /// deu: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Ant_Fem")]
    Sem_SLASH_Ant_Fem,
    /// kpv: **@CODE@**
    /// deu: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Ant_Mal")]
    Sem_SLASH_Ant_Mal,
    /// nob: **@CODE@**
    /// rmn: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Atr")]
    Sem_SLASH_Atr,
    /// kal: **@CODE@** =  Animal, land
    #[strum(serialize = "Sem/Azo")]
    Sem_SLASH_Azo,
    /// eus: (no comment)
    #[strum(serialize = "Sem/Biz")]
    Sem_SLASH_Biz,
    /// kpv: **@CODE@** Bodypart
    /// izh: **@CODE@** Bodypart
    /// liv: **@CODE@** Bodypart
    /// mhr: **@CODE@** = Bodypart
    /// krl: **@CODE@** Bodypart
    /// sma: (no comment)
    /// smj: **@CODE@** = Bodypart; ear, bone, ear canal, artificial leg, mustache, nervous system
    /// udm: **@CODE@** Bodypart
    /// apu: (no comment)
    /// yrk: **@CODE@** Bodypart
    /// sms: **@CODE@** = Bodypart
    /// mrj: **@CODE@** Bodypart
    /// myv: (no comment)
    /// olo: **@CODE@** Bodypart
    /// koi: **@CODE@** Bodypart
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Bodypart
    /// vro: **@CODE@** Bodypart
    #[strum(serialize = "Sem/Body")]
    Sem_SLASH_Body,
    /// kpv: **@CODE@** siellu, vuoig?a, jierbmi
    /// izh: **@CODE@** siellu, vuoig?a, jierbmi
    /// liv: **@CODE@** siellu, vuoig?a, jierbmi
    /// mhr: **@CODE@** = siellu, vuoig?a, jierbmi
    /// krl: **@CODE@** siellu, vuoig?a, jierbmi
    /// sma: (no comment)
    /// smj: **@CODE@** = Non-physical body part; Sjel, ånd, reason, soul, voice, eyesight, conscience
    /// udm: **@CODE@** siellu, vuoig?a, jierbmi
    /// yrk: **@CODE@** siellu, vuoig?a, jierbmi
    /// sms: **@CODE@** = siellu, vuoig?a, jierbmi
    /// mrj: **@CODE@** siellu, vuoig?a, jierbmi
    /// myv: (no comment)
    /// olo: **@CODE@** siellu, vuoig?a, jierbmi
    /// koi: **@CODE@** siellu, vuoig?a, jierbmi
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** siellu, vuoig?a, jierbmi
    /// vro: **@CODE@** siellu, vuoig?a, jierbmi
    #[strum(serialize = "Sem/Body-abstr")]
    Sem_SLASH_Body_MINUS_abstr,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Body-abstr_Feat-cogn")]
    Sem_SLASH_Body_MINUS_abstr_Feat_MINUS_cogn,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Body-abstr_Feat-psych")]
    Sem_SLASH_Body_MINUS_abstr_Feat_MINUS_psych,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body-abstr_Prod-audio_Semcon")]
    Sem_SLASH_Body_MINUS_abstr_Prod_MINUS_audio_Semcon,
    /// hdn: (no comment)
    #[strum(serialize = "Sem/BodyPart")]
    Sem_SLASH_BodyPart,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Body-abstr")]
    Sem_SLASH_Body_Body_MINUS_abstr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Buildpart")]
    Sem_SLASH_Body_Buildpart,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Clth")]
    Sem_SLASH_Body_Clth,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Clthpart")]
    Sem_SLASH_Body_Clthpart,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Food")]
    Sem_SLASH_Body_Food,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Fruit")]
    Sem_SLASH_Body_Fruit,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Group_Hum")]
    Sem_SLASH_Body_Group_Hum,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Body_Group_Hum_Time")]
    Sem_SLASH_Body_Group_Hum_Time,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Hum")]
    Sem_SLASH_Body_Hum,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Mat")]
    Sem_SLASH_Body_Mat,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Measr")]
    Sem_SLASH_Body_Measr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Obj")]
    Sem_SLASH_Body_Obj,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Plc")]
    Sem_SLASH_Body_Plc,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Body_Plc-elevate")]
    Sem_SLASH_Body_Plc_MINUS_elevate,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_Plc_State")]
    Sem_SLASH_Body_Plc_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Body_State")]
    Sem_SLASH_Body_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Body_Time")]
    Sem_SLASH_Body_Time,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Building
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Building
    /// liv: **@CODE@** Building
    /// mhr: **@CODE@** = Building
    /// krl: **@CODE@** Building
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Building; house, museum, Sámi tent, nest, sandcastle
    /// udm: **@CODE@** Building
    /// nio: (no comment)
    /// yrk: **@CODE@** Building
    /// mns: (no comment)
    /// sms: **@CODE@** = Building
    /// rmn: (no comment)
    /// mrj: **@CODE@** Building
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Building
    /// koi: **@CODE@** Building
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Building
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Building
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Build")]
    Sem_SLASH_Build,
    /// kpv: **@CODE@** Part of Bulding, like the closet
    /// izh: **@CODE@** Part of Bulding, like the closet
    /// liv: **@CODE@** Part of Bulding, like the closet
    /// mhr: **@CODE@** = Part of Bulding, like the closet
    /// krl: **@CODE@** Part of Bulding, like the closet
    /// udm: **@CODE@** Part of Bulding, like the closet
    /// yrk: **@CODE@** Part of Bulding, like the closet
    /// sms: **@CODE@** = Part of Bulding, like the closet
    /// mrj: **@CODE@** Part of Bulding, like the closet
    /// myv: (no comment)
    /// olo: **@CODE@** Part of Bulding, like the closet
    /// koi: **@CODE@** Part of Bulding, like the closet
    /// deu: (no comment)
    /// mdf: **@CODE@** Part of Bulding, like the closet
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build-part_Plc")]
    Sem_SLASH_Build_MINUS_part_Plc,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Room in a building, typically place to be
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Build-room")]
    Sem_SLASH_Build_MINUS_room,
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Clth-part")]
    Sem_SLASH_Build_Clth_MINUS_part,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Build_Clthpart")]
    Sem_SLASH_Build_Clthpart,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Edu_Org")]
    Sem_SLASH_Build_Edu_Org,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Event_Org")]
    Sem_SLASH_Build_Event_Org,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Build_Obj")]
    Sem_SLASH_Build_Obj,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Build_Org")]
    Sem_SLASH_Build_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Build_Plc")]
    Sem_SLASH_Build_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Buildpart")]
    Sem_SLASH_Buildpart,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Buildpart_Cat")]
    Sem_SLASH_Buildpart_Cat,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Buildpart_Cat_Ctain")]
    Sem_SLASH_Buildpart_Cat_Ctain,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Buildpart_Cat_Ctain_Mat")]
    Sem_SLASH_Buildpart_Cat_Ctain_Mat,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Buildpart_Ctain")]
    Sem_SLASH_Buildpart_Ctain,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Buildpart_Ctain_Mat")]
    Sem_SLASH_Buildpart_Ctain_Mat,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// kal: **@CODE@** =  Plant, vegetable
    #[strum(serialize = "Sem/Bveg")]
    Sem_SLASH_Bveg,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Category
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Category
    /// liv: **@CODE@** Category
    /// mhr: **@CODE@** = Category
    /// krl: **@CODE@** Category
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Category; name, subjunction, suffix, password, existential sentence
    /// udm: **@CODE@** Category
    /// yrk: **@CODE@** Category
    /// sms: **@CODE@** = Category
    /// rmn: (no comment)
    /// mrj: **@CODE@** Category
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Category
    /// koi: **@CODE@** Category
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Category
    /// vro: **@CODE@** Category
    /// bak: (no comment)
    #[strum(serialize = "Sem/Cat")]
    Sem_SLASH_Cat,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Cat_Group_Hum")]
    Sem_SLASH_Cat_Group_Hum,
    /// sma: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// smn: (no comment)
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
    /// lav: (no comment)
    /// izh: **@CODE@** Clothes
    /// liv: **@CODE@** Clothes
    /// mhr: **@CODE@** = Clothes
    /// krl: **@CODE@** Clothes
    /// sma: (no comment)
    /// lut: (no comment)
    /// smj: **@CODE@** = Clothes/Clothing; shirt, hat, theater costume, shawl, seatbelt, diaper
    /// udm: **@CODE@** Clothes
    /// nio: (no comment)
    /// yrk: **@CODE@** Clothes
    /// mns: (no comment)
    /// sms: **@CODE@** = Clothes
    /// mrj: **@CODE@** Clothes
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Clothes
    /// koi: **@CODE@** Clothes
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Clothes
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Clothes
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Clth")]
    Sem_SLASH_Clth,
    /// kpv: **@CODE@** Jewelery
    /// izh: **@CODE@** Jewelery
    /// liv: **@CODE@** Jewelery
    /// mhr: **@CODE@** = Jewelery
    /// krl: **@CODE@** Jewelery
    /// sma: (no comment)
    /// smj: **@CODE@** = Jewelery and similar; watch, sunglasses, ring, necklace, crown
    /// udm: **@CODE@** Jewelery
    /// yrk: **@CODE@** Jewelery
    /// sms: **@CODE@** = Jewelery
    /// mrj: **@CODE@** Jewelery
    /// myv: (no comment)
    /// olo: **@CODE@** Jewelery
    /// koi: **@CODE@** Jewelery
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Jewelery
    /// vro: **@CODE@** Jewelery
    #[strum(serialize = "Sem/Clth-jewl")]
    Sem_SLASH_Clth_MINUS_jewl,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Curr")]
    Sem_SLASH_Clth_MINUS_jewl_Curr,
    /// sma: (no comment)
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Curr_Obj")]
    Sem_SLASH_Clth_MINUS_jewl_Curr_Obj,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Curr_Obj_Org")]
    Sem_SLASH_Clth_MINUS_jewl_Curr_Obj_Org,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Fruit")]
    Sem_SLASH_Clth_MINUS_jewl_Fruit,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Money")]
    Sem_SLASH_Clth_MINUS_jewl_Money,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clth-jewl_Obj")]
    Sem_SLASH_Clth_MINUS_jewl_Obj,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Org")]
    Sem_SLASH_Clth_MINUS_jewl_Org,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth-jewl_Plant")]
    Sem_SLASH_Clth_MINUS_jewl_Plant,
    /// kpv: **@CODE@** part of clothes, boallu, sávdnji...
    /// izh: **@CODE@** part of clothes, boallu, sávdnji...
    /// liv: **@CODE@** part of clothes, boallu, sávdnji...
    /// mhr: **@CODE@** = part of clothes, boallu, sávdnji...
    /// krl: **@CODE@** part of clothes, boallu, sávdnji...
    /// udm: **@CODE@** part of clothes, boallu, sávdnji...
    /// yrk: **@CODE@** part of clothes, boallu, sávdnji...
    /// sms: **@CODE@** = part of clothes, boallu, sávdnji...
    /// mrj: **@CODE@** part of clothes, boallu, sávdnji...
    /// myv: (no comment)
    /// olo: **@CODE@** part of clothes, boallu, sávdnji...
    /// koi: **@CODE@** part of clothes, boallu, sávdnji...
    /// deu: (no comment)
    /// mdf: **@CODE@** part of clothes, boallu, sávdnji...
    /// vro: **@CODE@** part of clothes, boallu, sávdnji...
    #[strum(serialize = "Sem/Clth-part")]
    Sem_SLASH_Clth_MINUS_part,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Clth_Hum")]
    Sem_SLASH_Clth_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clth_Obj")]
    Sem_SLASH_Clth_Obj,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Clthpart")]
    Sem_SLASH_Clthpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Clthpart_Plc")]
    Sem_SLASH_Clthpart_Plc,
    /// olo: (no comment)
    #[strum(serialize = "Sem/Color")]
    Sem_SLASH_Color,
    /// sms: **@CODE@**
    /// myv: (no comment)
    #[strum(serialize = "Sem/Constellation")]
    Sem_SLASH_Constellation,
    /// kpv: **@CODE@** Container
    /// izh: **@CODE@** Container
    /// liv: **@CODE@** Container
    /// mhr: **@CODE@** = Container
    /// krl: **@CODE@** Container
    /// sma: (no comment)
    /// smj: **@CODE@** = Container; suitcase, terrarium, closet, container, gas tank
    /// udm: **@CODE@** Container
    /// yrk: **@CODE@** Container
    /// sms: **@CODE@** = Container
    /// mrj: **@CODE@** Container
    /// myv: (no comment)
    /// olo: **@CODE@** Container
    /// koi: **@CODE@** Container
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Container
    /// vro: **@CODE@** Container
    #[strum(serialize = "Sem/Ctain")]
    Sem_SLASH_Ctain,
    /// kpv: **@CODE@** Abstract container like bank account
    /// izh: **@CODE@** Abstract container like bank account
    /// liv: **@CODE@** Abstract container like bank account
    /// mhr: **@CODE@** = Abstract container like bank account
    /// krl: **@CODE@** Abstract container like bank account
    /// sma: (no comment)
    /// smj: **@CODE@** = Abstract container; bank account, fund, account, loan fund, pot, bank account
    /// udm: **@CODE@** Abstract container like bank account
    /// yrk: **@CODE@** Abstract container like bank account
    /// sms: **@CODE@** = Abstract container like bank account
    /// mrj: **@CODE@** Abstract container like bank account
    /// myv: (no comment)
    /// olo: **@CODE@** Abstract container like bank account
    /// koi: **@CODE@** Abstract container like bank account
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Abstract container like bank account
    /// vro: **@CODE@** Abstract container like bank account
    #[strum(serialize = "Sem/Ctain-abstr")]
    Sem_SLASH_Ctain_MINUS_abstr,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain-abstr_Org")]
    Sem_SLASH_Ctain_MINUS_abstr_Org,
    /// kpv: **@CODE@**
    /// izh: **@CODE@**
    /// liv: **@CODE@**
    /// mhr: **@CODE@**
    /// krl: **@CODE@**
    /// sma: (no comment)
    /// smj: **@CODE@** = Eks; lomme/pocket
    /// udm: **@CODE@**
    /// yrk: **@CODE@**
    /// sms: **@CODE@** = Soft container, like a rucksack
    /// mrj: **@CODE@**
    /// myv: (no comment)
    /// olo: **@CODE@**
    /// koi: **@CODE@**
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@**
    /// vro: **@CODE@**
    #[strum(serialize = "Sem/Ctain-clth")]
    Sem_SLASH_Ctain_MINUS_clth,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain-clth_Plant")]
    Sem_SLASH_Ctain_MINUS_clth_Plant,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain-clth_Veh")]
    Sem_SLASH_Ctain_MINUS_clth_Veh,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Feat-phys")]
    Sem_SLASH_Ctain_Feat_MINUS_phys,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Furn")]
    Sem_SLASH_Ctain_Furn,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ctain_Obj-surfc")]
    Sem_SLASH_Ctain_Obj_MINUS_surfc,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Ctain_Plc")]
    Sem_SLASH_Ctain_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Tool")]
    Sem_SLASH_Ctain_Tool,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Ctain_Tool-measr")]
    Sem_SLASH_Ctain_Tool_MINUS_measr,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Ctain_Txt")]
    Sem_SLASH_Ctain_Txt,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Currency like dollár, Not Money
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Currency like dollár, Not Money
    /// liv: **@CODE@** Currency like dollár, Not Money
    /// mhr: **@CODE@** = Currency like dollár, Not Money
    /// krl: **@CODE@** Currency like dollár, Not Money
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Currency; not money, euro, US dollar, denar, Danish crown, currency
    /// udm: **@CODE@** Currency like dollár, Not Money
    /// yrk: **@CODE@** Currency like dollár, Not Money
    /// sms: **@CODE@** = Currency like dollár, Not Money
    /// rmn: (no comment)
    /// mrj: **@CODE@** Currency like dollár, Not Money
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Currency like dollár, Not Money
    /// koi: **@CODE@** Currency like dollár, Not Money
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Currency like dollár, Not Money
    /// vro: **@CODE@** Currency like dollár, Not Money
    /// bak: (no comment)
    #[strum(serialize = "Sem/Curr")]
    Sem_SLASH_Curr,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Curr_Obj")]
    Sem_SLASH_Curr_Obj,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Curr_Org")]
    Sem_SLASH_Curr_Org,
    /// kpv: **@CODE@** Dance
    /// izh: **@CODE@** Dance
    /// liv: **@CODE@** Dance
    /// mhr: **@CODE@** = Dance
    /// krl: **@CODE@** Dance
    /// sma: (no comment)
    /// smj: **@CODE@** = Dance; swing, rumba, ballet, belly dance, war dance
    /// udm: **@CODE@** Dance
    /// yrk: **@CODE@** Dance
    /// sms: **@CODE@** = Dance
    /// mrj: **@CODE@** Dance
    /// myv: (no comment)
    /// olo: **@CODE@** Dance
    /// koi: **@CODE@** Dance
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Dance
    /// vro: **@CODE@** Dance
    #[strum(serialize = "Sem/Dance")]
    Sem_SLASH_Dance,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Dance_Org")]
    Sem_SLASH_Dance_Org,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Dance_Prod-audio")]
    Sem_SLASH_Dance_Prod_MINUS_audio,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Date
    /// sms: **@CODE@** = Number as date
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Date")]
    Sem_SLASH_Date,
    /// kpv: **@CODE@** Direction like GPS-kursa
    /// izh: **@CODE@** Direction like GPS-kursa
    /// liv: **@CODE@** Direction like GPS-kursa
    /// mhr: **@CODE@** = Direction like GPS-kursa
    /// krl: **@CODE@** Direction like GPS-kursa
    /// sma: (no comment)
    /// smj: **@CODE@** = Direction; GPS course, stock exchange price, graph, tendency, starboard
    /// udm: **@CODE@** Direction like GPS-kursa
    /// yrk: **@CODE@** Direction like GPS-kursa
    /// sms: **@CODE@** = Direction like GPS-kursa
    /// mrj: **@CODE@** Direction like GPS-kursa
    /// myv: (no comment)
    /// olo: **@CODE@** Direction like GPS-kursa
    /// koi: **@CODE@** Direction like GPS-kursa
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Direction like GPS-kursa
    /// vro: **@CODE@** Direction like GPS-kursa
    #[strum(serialize = "Sem/Dir")]
    Sem_SLASH_Dir,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Dir_Geom")]
    Sem_SLASH_Dir_Geom,
    /// myv: (no comment)
    #[strum(serialize = "Sem/Divinity")]
    Sem_SLASH_Divinity,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// liv: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// mhr: **@CODE@** = Domain like politics, reindeerherding (a system of actions)
    /// krl: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Domain like politics, reindeerherding (a system of actions), anthropology, punk rock, biology, linguistics, medicine
    /// udm: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// yrk: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// sms: **@CODE@** = Domain like politics, reindeerherding (a system of actions)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// koi: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// vro: **@CODE@** Domain like politics, reindeerherding (a system of actions)
    /// bak: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Domain_Food-med")]
    Sem_SLASH_Domain_Food_MINUS_med,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// bak: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@** Drink
    /// liv: **@CODE@** Drink
    /// mhr: **@CODE@** = Drink
    /// krl: **@CODE@** Drink
    /// sma: (no comment)
    /// smj: **@CODE@** = Drink; tea, beer, UHT milk, soda, communion wine
    /// udm: **@CODE@** Drink
    /// yrk: **@CODE@** Drink
    /// sms: **@CODE@** = Drink
    /// mrj: **@CODE@** Drink
    /// myv: (no comment)
    /// olo: **@CODE@** Drink
    /// koi: **@CODE@** Drink
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Drink
    /// vro: **@CODE@** Drink
    #[strum(serialize = "Sem/Drink")]
    Sem_SLASH_Drink,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Drink_Plant")]
    Sem_SLASH_Drink_Plant,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Drink_Plc")]
    Sem_SLASH_Drink_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Drink_Plc_Substnc")]
    Sem_SLASH_Drink_Plc_Substnc,
    /// deu: (no comment)
    #[strum(serialize = "Sem/Dummy")]
    Sem_SLASH_Dummy,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Dummytag
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Dummytag
    /// liv: **@CODE@** Dummytag
    /// mhr: **@CODE@** = Dummytag
    /// krl: **@CODE@** Dummytag
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Default tag for uncategorized nouns
    /// udm: **@CODE@** Dummytag
    /// yrk: **@CODE@** Dummytag
    /// sms: **@CODE@** = Dummytag
    /// rmn: (no comment)
    /// mrj: **@CODE@** Dummytag
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Dummytag
    /// koi: **@CODE@** Dummytag
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Dummytag
    /// vro: **@CODE@** Dummytag
    /// bak: (no comment)
    #[strum(serialize = "Sem/Dummytag")]
    Sem_SLASH_Dummytag,
    /// fit: **@CODE@** = Education institution
    /// kpv: **@CODE@** Educational event
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Educational event
    /// liv: **@CODE@** Educational event
    /// mhr: **@CODE@** = Educational event
    /// krl: **@CODE@** Educational event
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Educational event; skiing high school skiing academy, course, music lesson, lesson, master
    /// udm: **@CODE@** Educational event
    /// nio: (no comment)
    /// yrk: **@CODE@** Educational event
    /// mns: (no comment)
    /// sms: **@CODE@** = Educational event
    /// rmn: (no comment)
    /// mrj: **@CODE@** Educational event
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Educational event
    /// koi: **@CODE@** Educational event
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Educational event
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Educational event
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Edu")]
    Sem_SLASH_Edu,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Event")]
    Sem_SLASH_Edu_Event,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Edu_Geom")]
    Sem_SLASH_Edu_Geom,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Edu_Geom_Plc-line")]
    Sem_SLASH_Edu_Geom_Plc_MINUS_line,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Group_Hum")]
    Sem_SLASH_Edu_Group_Hum,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Edu_Hum")]
    Sem_SLASH_Edu_Hum,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Mat")]
    Sem_SLASH_Edu_Mat,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Edu_Org")]
    Sem_SLASH_Edu_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Edu_Txt")]
    Sem_SLASH_Edu_Txt,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Event
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Event
    /// liv: **@CODE@** Event
    /// mhr: **@CODE@** = Event
    /// krl: **@CODE@** Event
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Event; wedding, meeting, competition, election, festival
    /// udm: **@CODE@** Event
    /// yrk: **@CODE@** Event
    /// sms: **@CODE@** = Event
    /// rmn: (no comment)
    /// mrj: **@CODE@** Event
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Event
    /// koi: **@CODE@** Event
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Event
    /// vro: **@CODE@** Event
    /// bak: (no comment)
    #[strum(serialize = "Sem/Event")]
    Sem_SLASH_Event,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Food")]
    Sem_SLASH_Event_Food,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Hum")]
    Sem_SLASH_Event_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Event_Org")]
    Sem_SLASH_Event_Org,
    /// kpv: **@CODE@**  сёянін
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Plc")]
    Sem_SLASH_Event_Plc,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Event_Plc-elevate")]
    Sem_SLASH_Event_Plc_MINUS_elevate,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Event_Time")]
    Sem_SLASH_Event_Time,
    /// kpv: **@CODE@** Feature, like Árvu
    /// izh: **@CODE@** Feature, like Árvu
    /// liv: **@CODE@** Feature, like Árvu
    /// mhr: **@CODE@** = Feature, like Árvu
    /// krl: **@CODE@** Feature, like Árvu
    /// sma: (no comment)
    /// smj: **@CODE@** = Feature; Árvu, age difference, homosexuality, femininity, identity, congruence
    /// udm: **@CODE@** Feature, like Árvu
    /// yrk: **@CODE@** Feature, like Árvu
    /// sms: **@CODE@** = Feature, like Árvu
    /// mrj: **@CODE@** Feature, like Árvu
    /// myv: (no comment)
    /// olo: **@CODE@** Feature, like Árvu
    /// koi: **@CODE@** Feature, like Árvu
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Feature, like Árvu
    /// vro: **@CODE@** Feature, like Árvu
    #[strum(serialize = "Sem/Feat")]
    Sem_SLASH_Feat,
    /// kpv: **@CODE@** Psychological feauture
    /// izh: **@CODE@** Psychological feauture
    /// liv: **@CODE@** Psychological feauture
    /// mhr: **@CODE@** = Psychological feauture
    /// krl: **@CODE@** Psychological feauture
    /// sma: (no comment)
    /// smj: **@CODE@** = Measurable feature; radius, diameter, volume, circumference, perimeter, frequency
    /// udm: **@CODE@** Psychological feauture
    /// yrk: **@CODE@** Psychological feauture
    /// sms: **@CODE@** = Psychological feauture
    /// mrj: **@CODE@** Psychological feauture
    /// myv: (no comment)
    /// olo: **@CODE@** Psychological feauture
    /// koi: **@CODE@** Psychological feauture
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Psychological feauture
    /// vro: **@CODE@** Psychological feauture
    #[strum(serialize = "Sem/Feat-measr")]
    Sem_SLASH_Feat_MINUS_measr,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Feat-measr_Plc")]
    Sem_SLASH_Feat_MINUS_measr_Plc,
    /// kpv: **@CODE@** Physiological feature, ivdni, fárda
    /// izh: **@CODE@** Physiological feature, ivdni, fárda
    /// liv: **@CODE@** Physiological feature, ivdni, fárda
    /// mhr: **@CODE@** = Physiological feature, ivdni, fárda
    /// krl: **@CODE@** Physiological feature, ivdni, fárda
    /// sma: (no comment)
    /// smj: **@CODE@** = Physiological feature; size, color, height, shape, weight, horsepower
    /// udm: **@CODE@** Physiological feature, ivdni, fárda
    /// yrk: **@CODE@** Physiological feature, ivdni, fárda
    /// sms: **@CODE@** = Physiological feature, ivdni, fárda
    /// mrj: **@CODE@** Physiological feature, ivdni, fárda
    /// myv: (no comment)
    /// olo: **@CODE@** Physiological feature, ivdni, fárda
    /// koi: **@CODE@** Physiological feature, ivdni, fárda
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Physiological feature, ivdni, fárda
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-phys_Tool-write")]
    Sem_SLASH_Feat_MINUS_phys_Tool_MINUS_write,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-phys_Veh")]
    Sem_SLASH_Feat_MINUS_phys_Veh,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-phys_Wthr")]
    Sem_SLASH_Feat_MINUS_phys_Wthr,
    /// kpv: **@CODE@** Psychological feauture
    /// izh: **@CODE@** Psychological feauture
    /// liv: **@CODE@** Psychological feauture
    /// mhr: **@CODE@** = Psychological feauture
    /// krl: **@CODE@** Psychological feauture
    /// sma: (no comment)
    /// smj: **@CODE@** = Psychological feature; authority, nature, childishness, creativity, arrogance
    /// udm: **@CODE@** Psychological feauture
    /// yrk: **@CODE@** Psychological feauture
    /// sms: **@CODE@** = Psychological feauture
    /// mrj: **@CODE@** Psychological feauture
    /// myv: (no comment)
    /// olo: **@CODE@** Psychological feauture
    /// koi: **@CODE@** Psychological feauture
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Psychological feauture
    /// vro: **@CODE@** Psychological feauture
    #[strum(serialize = "Sem/Feat-psych")]
    Sem_SLASH_Feat_MINUS_psych,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat-psych_Hum")]
    Sem_SLASH_Feat_MINUS_psych_Hum,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Feat-psych_Plc")]
    Sem_SLASH_Feat_MINUS_psych_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Feat_Hum")]
    Sem_SLASH_Feat_Hum,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Feat_Plant")]
    Sem_SLASH_Feat_Plant,
    /// fit: **@CODE@** = Female names
    /// kal: **@CODE@** Forsøg med femininum sml. Sem/Mask
    /// kpv: **@CODE@** Female name
    /// vot: (no comment)
    /// fkv: **@CODE@** = Female names
    /// izh: **@CODE@** Female name
    /// liv: **@CODE@** Female name
    /// mhr: **@CODE@** = Female name
    /// krl: **@CODE@** Female name
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// fao: (no comment)
    /// smj: **@CODE@** = Female name
    /// udm: **@CODE@** Female name
    /// yrk: **@CODE@** Female name
    /// mns: (no comment)
    /// sms: **@CODE@** = Female name
    /// rmn: (no comment)
    /// mrj: **@CODE@** Female name
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// myv: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Female name
    /// koi: **@CODE@** Female name
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Female name
    /// ceb: (no comment)
    /// vro: **@CODE@** Female name
    /// mpj: **@CODE@**
    #[strum(serialize = "Sem/Fem")]
    Sem_SLASH_Fem,
    /// myv: (no comment)
    /// mdf: **@CODE@** Female name
    #[strum(serialize = "Sem/Fem-Patr")]
    Sem_SLASH_Fem_MINUS_Patr,
    /// myv: (no comment)
    /// mdf: **@CODE@** Female name
    #[strum(serialize = "Sem/Fem-Sur")]
    Sem_SLASH_Fem_MINUS_Sur,
    /// kal: **@CODE@** = Attributive
    #[strum(serialize = "Sem/FirstName")]
    Sem_SLASH_FirstName,
    /// kpv: **@CODE@** Food
    /// izh: **@CODE@** Food
    /// liv: **@CODE@** Food
    /// mhr: **@CODE@** = Food
    /// krl: **@CODE@** Food
    /// sma: (no comment)
    /// smj: **@CODE@** = Food; bread, vegetarian food, flour, tobacco, salt
    /// udm: **@CODE@** Food
    /// yrk: **@CODE@** Food
    /// sms: **@CODE@** = Food
    /// mrj: **@CODE@** Food
    /// myv: (no comment)
    /// olo: **@CODE@** Food
    /// koi: **@CODE@** Food
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Food
    /// vro: **@CODE@** Food
    #[strum(serialize = "Sem/Food")]
    Sem_SLASH_Food,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Medicine
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Medicine
    /// liv: **@CODE@** Medicine
    /// mhr: **@CODE@** = Medicine
    /// krl: **@CODE@** Medicine
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Medicine; birth-control pill, asthma medicine, medicine, penicillin, vaccine
    /// udm: **@CODE@** Medicine
    /// yrk: **@CODE@** Medicine
    /// sms: **@CODE@** = Medicine
    /// rmn: (no comment)
    /// mrj: **@CODE@** Medicine
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Medicine
    /// koi: **@CODE@** Medicine
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Medicine
    /// vro: **@CODE@** Medicine
    /// bak: (no comment)
    #[strum(serialize = "Sem/Food-med")]
    Sem_SLASH_Food_MINUS_med,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Food_Obj-surfc")]
    Sem_SLASH_Food_Obj_MINUS_surfc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Food_Part")]
    Sem_SLASH_Food_Part,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Food_Perc-phys")]
    Sem_SLASH_Food_Perc_MINUS_phys,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Food_Plant")]
    Sem_SLASH_Food_Plant,
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Fruit")]
    Sem_SLASH_Fruit,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Fruit_Hum")]
    Sem_SLASH_Fruit_Hum,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Fruit_Sport")]
    Sem_SLASH_Fruit_Sport,
    /// kpv: **@CODE@** Furniture
    /// izh: **@CODE@** Furniture
    /// liv: **@CODE@** Furniture
    /// mhr: **@CODE@** = Furniture
    /// krl: **@CODE@** Furniture
    /// sma: (no comment)
    /// smj: **@CODE@** = Furniture; throne, chair, table, altar, trampoline
    /// udm: **@CODE@** Furniture
    /// yrk: **@CODE@** Furniture
    /// sms: **@CODE@** = Furniture
    /// mrj: **@CODE@** Furniture
    /// myv: (no comment)
    /// olo: **@CODE@** Furniture
    /// koi: **@CODE@** Furniture
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Furniture
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
    /// izh: **@CODE@** Game
    /// liv: **@CODE@** Game
    /// mhr: **@CODE@** = Game
    /// krl: **@CODE@** Game
    /// sma: (no comment)
    /// smj: **@CODE@** = Game; biŋgo bingo, TV game, flipper, paintball, chess
    /// udm: **@CODE@** Game
    /// yrk: **@CODE@** Game
    /// sms: **@CODE@** = Game
    /// mrj: **@CODE@** Game
    /// myv: (no comment)
    /// olo: **@CODE@** Game
    /// koi: **@CODE@** Game
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Game
    /// vro: **@CODE@** Game
    #[strum(serialize = "Sem/Game")]
    Sem_SLASH_Game,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Game_Obj-play")]
    Sem_SLASH_Game_Obj_MINUS_play,
    /// kal: **@CODE@** = Geographical Propernoun
    /// fin: (no comment)
    #[strum(serialize = "Sem/Geo")]
    Sem_SLASH_Geo,
    /// kpv: **@CODE@** Geometrical object
    /// izh: **@CODE@** Geometrical object
    /// liv: **@CODE@** Geometrical object
    /// mhr: **@CODE@** = Geometrical object
    /// krl: **@CODE@** Geometrical object
    /// sma: (no comment)
    /// smj: **@CODE@** = Geometrical object; triangle, triangle, tetrahedron, asymptote, star
    /// udm: **@CODE@** Geometrical object
    /// yrk: **@CODE@** Geometrical object
    /// sms: **@CODE@** = Geometrical object
    /// mrj: **@CODE@** Geometrical object
    /// myv: (no comment)
    /// olo: **@CODE@** Geometrical object
    /// koi: **@CODE@** Geometrical object
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Geometrical object
    /// vro: **@CODE@** Geometrical object
    #[strum(serialize = "Sem/Geom")]
    Sem_SLASH_Geom,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Geom_Hum_Plc")]
    Sem_SLASH_Geom_Hum_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Geom_Obj")]
    Sem_SLASH_Geom_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Geom_Plc")]
    Sem_SLASH_Geom_Plc,
    /// kpv: **@CODE@** Animal or Human Group
    /// vot: (no comment)
    /// lav: (no comment)
    /// izh: **@CODE@** Animal or Human Group
    /// liv: **@CODE@** Animal or Human Group
    /// mhr: **@CODE@** = Animal or Human Group
    /// krl: **@CODE@** Animal or Human Group
    /// sma: (no comment)
    /// lut: (no comment)
    /// smj: **@CODE@** = Animal or Human Group; family, herd, group, indigenous people, delegation
    /// udm: **@CODE@** Animal or Human Group
    /// nio: (no comment)
    /// yrk: **@CODE@** Animal or Human Group
    /// mns: (no comment)
    /// sms: **@CODE@** = Animal or Human Group
    /// mrj: **@CODE@** Animal or Human Group
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Animal or Human Group
    /// koi: **@CODE@** Animal or Human Group
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Animal or Human Group
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Animal or Human Group
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Group")]
    Sem_SLASH_Group,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: (no comment)
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Group_Hum")]
    Sem_SLASH_Group_Hum,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Org")]
    Sem_SLASH_Group_Hum_Org,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Plc")]
    Sem_SLASH_Group_Hum_Plc,
    /// sma: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Plc-abstr")]
    Sem_SLASH_Group_Hum_Plc_MINUS_abstr,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Prod-vis")]
    Sem_SLASH_Group_Hum_Prod_MINUS_vis,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Group_Hum_Time")]
    Sem_SLASH_Group_Hum_Time,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Org")]
    Sem_SLASH_Group_Org,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Group_Plc")]
    Sem_SLASH_Group_Plc,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Group_Prod-vis")]
    Sem_SLASH_Group_Prod_MINUS_vis,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Group_Prod-vis_Txt_Veh")]
    Sem_SLASH_Group_Prod_MINUS_vis_Txt_Veh,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Sign")]
    Sem_SLASH_Group_Sign,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Group_State")]
    Sem_SLASH_Group_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Group_Txt")]
    Sem_SLASH_Group_Txt,
    /// kal: **@CODE@** =  Human
    #[strum(serialize = "Sem/H")]
    Sem_SLASH_H,
    /// kal: **@CODE@** =  Human, group of
    #[strum(serialize = "Sem/HH")]
    Sem_SLASH_HH,
    /// kal: **@CODE@** =  Human, organisation
    #[strum(serialize = "Sem/HHorg")]
    Sem_SLASH_HHorg,
    /// kal: **@CODE@** =  Human, attribute
    #[strum(serialize = "Sem/Hattr")]
    Sem_SLASH_Hattr,
    /// kal: **@CODE@** =  Human, biological
    #[strum(serialize = "Sem/Hbio")]
    Sem_SLASH_Hbio,
    /// kal: **@CODE@** =  Human, family
    #[strum(serialize = "Sem/Hfam")]
    Sem_SLASH_Hfam,
    /// kal: **@CODE@** =  Human, idelogical
    #[strum(serialize = "Sem/Hideo")]
    Sem_SLASH_Hideo,
    /// kal: **@CODE@** =  Human, mythological
    #[strum(serialize = "Sem/Hmyth")]
    Sem_SLASH_Hmyth,
    /// kal: **@CODE@** =  Human, nationality
    #[strum(serialize = "Sem/Hnat")]
    Sem_SLASH_Hnat,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Holiday")]
    Sem_SLASH_Holiday,
    /// kal: **@CODE@** =  Human, profession
    #[strum(serialize = "Sem/Hprof")]
    Sem_SLASH_Hprof,
    /// kal: **@CODE@** =  Human, sick
    #[strum(serialize = "Sem/Hsick")]
    Sem_SLASH_Hsick,
    /// kal: **@CODE@** =  Human, title
    #[strum(serialize = "Sem/Htit")]
    Sem_SLASH_Htit,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kal: **@CODE@** = Non-geographical Propernoun
    /// kpv: **@CODE@** Human
    /// swe: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Human
    /// liv: **@CODE@** Human
    /// mhr: **@CODE@** = Human
    /// krl: **@CODE@** Human
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Human
    /// udm: **@CODE@** Human
    /// nio: (no comment)
    /// yrk: **@CODE@** Human
    /// mns: (no comment)
    /// sms: **@CODE@** = Human
    /// rmn: (no comment)
    /// mrj: **@CODE@** Human
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Human
    /// koi: **@CODE@** Human
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Human
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Human
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Hum")]
    Sem_SLASH_Hum,
    /// kpv: **@CODE@** Human abstract
    /// izh: **@CODE@** Human abstract
    /// liv: **@CODE@** Human abstract
    /// mhr: **@CODE@** = Human abstract
    /// krl: **@CODE@** Human abstract
    /// smj: **@CODE@** = Human abstract
    /// smj: **@CODE@** =
    /// udm: **@CODE@** Human abstract
    /// yrk: **@CODE@** Human abstract
    /// sms: **@CODE@** = Human abstract
    /// mrj: **@CODE@** Human abstract
    /// myv: (no comment)
    /// olo: **@CODE@** Human abstract
    /// koi: **@CODE@** Human abstract
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Human abstract
    /// vro: **@CODE@** Human abstract
    #[strum(serialize = "Sem/Hum-abstr")]
    Sem_SLASH_Hum_MINUS_abstr,
    /// sms: **@CODE@** =
    #[strum(serialize = "Sem/Hum-kin")]
    Sem_SLASH_Hum_MINUS_kin,
    /// koi: **@CODE@** Human professional
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Hum-prof")]
    Sem_SLASH_Hum_MINUS_prof,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Lang")]
    Sem_SLASH_Hum_Lang,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Lang_Plc")]
    Sem_SLASH_Hum_Lang_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Lang_Time")]
    Sem_SLASH_Hum_Lang_Time,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Hum_Mat_Tool")]
    Sem_SLASH_Hum_Mat_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Money")]
    Sem_SLASH_Hum_Money,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Obj")]
    Sem_SLASH_Hum_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Obj_Plc")]
    Sem_SLASH_Hum_Obj_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Org")]
    Sem_SLASH_Hum_Org,
    /// sme: (no comment)
    #[strum(serialize = "Sem/Hum_Org_Pos")]
    Sem_SLASH_Hum_Org_Pos,
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Hum_Part")]
    Sem_SLASH_Hum_Part,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Plant")]
    Sem_SLASH_Hum_Plant,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// smn: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Hum_Sign")]
    Sem_SLASH_Hum_Sign,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_State")]
    Sem_SLASH_Hum_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Tool")]
    Sem_SLASH_Hum_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Hum_Tool-catch")]
    Sem_SLASH_Hum_Tool_MINUS_catch,
    /// sms: **@CODE@** = Human
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Hum_Tool-it")]
    Sem_SLASH_Hum_Tool_MINUS_it,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Veh")]
    Sem_SLASH_Hum_Veh,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Hum_Wthr")]
    Sem_SLASH_Hum_Wthr,
    /// fin: (no comment)
    #[strum(serialize = "Sem/Human")]
    Sem_SLASH_Human,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** = ID
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = ID
    /// mns: (no comment)
    /// sms: **@CODE@** = number as identity tag, not as amount
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@** used with numerals 2023_04_04
    /// bak: (no comment)
    #[strum(serialize = "Sem/ID")]
    Sem_SLASH_ID,
    /// kpv: **@CODE@** Ideology
    /// izh: **@CODE@** Ideology
    /// liv: **@CODE@** Ideology
    /// mhr: **@CODE@** = Ideology
    /// krl: **@CODE@** Ideology
    /// sma: (no comment)
    /// smj: **@CODE@** = Ideology; nomadism, buddhism, feminism, christianity, fanaticism
    /// udm: **@CODE@** Ideology
    /// yrk: **@CODE@** Ideology
    /// sms: **@CODE@** = Ideology
    /// mrj: **@CODE@** Ideology
    /// myv: (no comment)
    /// olo: **@CODE@** Ideology
    /// koi: **@CODE@** Ideology
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Ideology
    /// vro: **@CODE@** Ideology
    #[strum(serialize = "Sem/Ideol")]
    Sem_SLASH_Ideol,
    /// myv: (no comment)
    /// koi: **@CODE@**  Inanimate,
    #[strum(serialize = "Sem/Inanim")]
    Sem_SLASH_Inanim,
    /// sms: **@CODE@** for deprication to Sem/Hum-kin
    /// myv: (no comment)
    /// mdf: **@CODE@** Kin term
    #[strum(serialize = "Sem/Kin")]
    Sem_SLASH_Kin,
    /// myv: (no comment)
    #[strum(serialize = "Sem/Kin_Fem")]
    Sem_SLASH_Kin_Fem,
    /// myv: (no comment)
    #[strum(serialize = "Sem/Kin_Mal")]
    Sem_SLASH_Kin_Mal,
    /// hdn: (no comment)
    #[strum(serialize = "Sem/Kinterm")]
    Sem_SLASH_Kinterm,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Language
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Language
    /// liv: **@CODE@** Language
    /// mhr: **@CODE@** = Language
    /// krl: **@CODE@** Language
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Language; South Sámi, mother tongue, Maori, Yiddish, second language
    /// udm: **@CODE@** Language
    /// yrk: **@CODE@** Language
    /// sms: **@CODE@** = Language
    /// rmn: (no comment)
    /// mrj: **@CODE@** Language
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Language
    /// koi: **@CODE@** Language
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Language
    /// vro: **@CODE@** Language
    /// bak: (no comment)
    #[strum(serialize = "Sem/Lang")]
    Sem_SLASH_Lang,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Lang_Tool")]
    Sem_SLASH_Lang_Tool,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Lang_Tool-catch")]
    Sem_SLASH_Lang_Tool_MINUS_catch,
    /// kal: **@CODE@**  Efternavne
    #[strum(serialize = "Sem/LastName")]
    Sem_SLASH_LastName,
    /// kal: **@CODE@** =  Location, semantic
    #[strum(serialize = "Sem/Lsem")]
    Sem_SLASH_Lsem,
    /// kal: **@CODE@** =  Location, astronomical
    #[strum(serialize = "Sem/Lstar")]
    Sem_SLASH_Lstar,
    /// kal: **@CODE@** tag til adresser af typen cccc@cccc.cc
    #[strum(serialize = "Sem/Mailadresse")]
    Sem_SLASH_Mailadresse,
    /// fit: **@CODE@** = Male names
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Male name
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Male names
    /// izh: **@CODE@** Male name
    /// liv: **@CODE@** Male name
    /// mhr: **@CODE@** = Male name
    /// krl: **@CODE@** Male name
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Male name
    /// udm: **@CODE@** Male name
    /// nio: (no comment)
    /// yrk: **@CODE@** Male name
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Male name
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Male name
    /// vep: **@CODE@**
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Male name
    /// koi: **@CODE@** Male name
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Male name
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Male name
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Mal")]
    Sem_SLASH_Mal,
    /// myv: (no comment)
    /// mdf: **@CODE@** Male name
    #[strum(serialize = "Sem/Mal-Patr")]
    Sem_SLASH_Mal_MINUS_Patr,
    /// myv: (no comment)
    /// mdf: **@CODE@** Male name
    #[strum(serialize = "Sem/Mal-Sur")]
    Sem_SLASH_Mal_MINUS_Sur,
    /// sms: **@CODE@**
    #[strum(serialize = "Sem/Manner")]
    Sem_SLASH_Manner,
    /// kal: **@CODE@** Forsøg med en ny tag til fornavne af maskulinum
    #[strum(serialize = "Sem/Mask")]
    Sem_SLASH_Mask,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Material for producing things
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Material for producing things
    /// liv: **@CODE@** Material for producing things
    /// mhr: **@CODE@** = Material for producing things
    /// krl: **@CODE@** Material for producing things
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Material for producing things; paper, steel, wood, leather, wool
    /// udm: **@CODE@** Material for producing things
    /// yrk: **@CODE@** Material for producing things
    /// sms: **@CODE@** = Material for producing things
    /// rmn: (no comment)
    /// mrj: **@CODE@** Material for producing things
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Material for producing things
    /// koi: **@CODE@** Material for producing things
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Material for producing things
    /// vro: **@CODE@** Material for producing things
    /// bak: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Mat_Txt")]
    Sem_SLASH_Mat_Txt,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Measure
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Measure
    /// liv: **@CODE@** Measure
    /// mhr: **@CODE@** = Measure
    /// krl: **@CODE@** Measure
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Measure; unit of volume, quarter meter, hour, minus degree, wátta watt
    /// udm: **@CODE@** Measure
    /// nio: (no comment)
    /// yrk: **@CODE@** Measure
    /// mns: (no comment)
    /// sms: **@CODE@** = Measure
    /// rmn: (no comment)
    /// mrj: **@CODE@** Measure
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Measure
    /// koi: **@CODE@** Measure
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Measure
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Measure
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Measr")]
    Sem_SLASH_Measr,
    /// sma: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Measr_Obj_Time")]
    Sem_SLASH_Measr_Obj_Time,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Measr_Plc_Time")]
    Sem_SLASH_Measr_Plc_Time,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@** = Sign (e.g. numbers, punctuation)
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Measr_Sign")]
    Sem_SLASH_Measr_Sign,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Measr_Time")]
    Sem_SLASH_Measr_Time,
    /// myv: (no comment)
    #[strum(serialize = "Sem/Mnth")]
    Sem_SLASH_Mnth,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// liv: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// mhr: **@CODE@** = Has to do with money, like wages, not Curr(ency)
    /// krl: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Has to do with money; wages, not Curr(ency), treasure, belongings, debt, food price, bill, pension
    /// udm: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// yrk: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// sms: **@CODE@** = Has to do with money, like wages, not Curr(ency)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// koi: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// vro: **@CODE@** Has to do with money, like wages, not Curr(ency)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Money")]
    Sem_SLASH_Money,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Money_Obj")]
    Sem_SLASH_Money_Obj,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Money_Org")]
    Sem_SLASH_Money_Org,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Money_Part")]
    Sem_SLASH_Money_Part,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Money_Plc")]
    Sem_SLASH_Money_Plc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Money_Txt")]
    Sem_SLASH_Money_Txt,
    /// koi: **@CODE@**  Nonhuman,
    #[strum(serialize = "Sem/NonHum")]
    Sem_SLASH_NonHum,
    /// fit: **@CODE@** = Names of objects
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Object
    /// swe: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Names of objects
    /// izh: **@CODE@** Object
    /// liv: **@CODE@** Object
    /// mhr: **@CODE@** = Object
    /// krl: **@CODE@** Object
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Object; thing, cup, thing, toy, painting
    /// udm: **@CODE@** Object
    /// nio: (no comment)
    /// yrk: **@CODE@** Object
    /// mns: (no comment)
    /// sms: **@CODE@**              (names)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Object
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Object
    /// koi: **@CODE@** Object
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Object
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Object
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Obj")]
    Sem_SLASH_Obj,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Obj-catch")]
    Sem_SLASH_Obj_MINUS_catch,
    /// kpv: **@CODE@** Cloth
    /// izh: **@CODE@** Cloth
    /// liv: **@CODE@** Cloth
    /// mhr: **@CODE@** = Cloth
    /// krl: **@CODE@** Cloth
    /// sma: (no comment)
    /// smj: **@CODE@** = Cloth; carpet, flag, curtain, silk sheets, napkin
    /// udm: **@CODE@** Cloth
    /// yrk: **@CODE@** Cloth
    /// sms: **@CODE@** = Cloth
    /// mrj: **@CODE@** Cloth
    /// myv: (no comment)
    /// olo: **@CODE@** Cloth
    /// koi: **@CODE@** Cloth
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Cloth
    /// vro: **@CODE@** Cloth
    #[strum(serialize = "Sem/Obj-clo")]
    Sem_SLASH_Obj_MINUS_clo,
    /// kpv: **@CODE@** Cloth
    /// izh: **@CODE@** Cloth
    /// liv: **@CODE@** Cloth
    /// mhr: **@CODE@** = Cloth
    /// krl: **@CODE@** Cloth
    /// smj: **@CODE@** =
    /// udm: **@CODE@** Cloth
    /// yrk: **@CODE@** Cloth
    /// sms: **@CODE@** = Cloth
    /// mrj: **@CODE@** Cloth
    /// myv: (no comment)
    /// olo: **@CODE@** Cloth
    /// koi: **@CODE@** Cloth
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Cloth
    /// vro: **@CODE@** Cloth
    #[strum(serialize = "Sem/Obj-cogn")]
    Sem_SLASH_Obj_MINUS_cogn,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** (Electrical) machine or apparatus
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** (Electrical) machine or apparatus
    /// liv: **@CODE@** (Electrical) machine or apparatus
    /// mhr: **@CODE@** = (Electrical) machine or apparatus
    /// krl: **@CODE@** (Electrical) machine or apparatus
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = (Electrical) machine or apparatus; player, lamp, TV, radio, oven
    /// udm: **@CODE@** (Electrical) machine or apparatus
    /// yrk: **@CODE@** (Electrical) machine or apparatus
    /// sms: **@CODE@** = (Electrical) machine or apparatus
    /// rmn: (no comment)
    /// mrj: **@CODE@** (Electrical) machine or apparatus
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** (Electrical) machine or apparatus
    /// koi: **@CODE@** (Electrical) machine or apparatus
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** (Electrical) machine or apparatus
    /// vro: **@CODE@** (Electrical) machine or apparatus
    /// bak: (no comment)
    #[strum(serialize = "Sem/Obj-el")]
    Sem_SLASH_Obj_MINUS_el,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Object with something written on it
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Object with something written on it
    /// liv: **@CODE@** Object with something written on it
    /// mhr: **@CODE@** = Object with something written on it
    /// krl: **@CODE@** Object with something written on it
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Object with something written on it
    /// udm: **@CODE@** Object with something written on it
    /// yrk: **@CODE@** Object with something written on it
    /// sms: **@CODE@** = Object with something written on it
    /// rmn: (no comment)
    /// mrj: **@CODE@** Object with something written on it
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Object with something written on it
    /// koi: **@CODE@** Object with something written on it
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Object with something written on it
    /// vro: **@CODE@** Object with something written on it
    /// bak: (no comment)
    #[strum(serialize = "Sem/Obj-ling")]
    Sem_SLASH_Obj_MINUS_ling,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Obj-ling_Obj-surfc")]
    Sem_SLASH_Obj_MINUS_ling_Obj_MINUS_surfc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** = Play object
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj-play")]
    Sem_SLASH_Obj_MINUS_play,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj-play_Sport")]
    Sem_SLASH_Obj_MINUS_play_Sport,
    /// kpv: **@CODE@** flexible ropelike object
    /// izh: **@CODE@** flexible ropelike object
    /// liv: **@CODE@** flexible ropelike object
    /// mhr: **@CODE@** = flexible ropelike object
    /// krl: **@CODE@** flexible ropelike object
    /// sma: (no comment)
    /// smj: **@CODE@** = flexible ropelike object; barbed wire, thread, rope, cable, dental floss
    /// udm: **@CODE@** flexible ropelike object
    /// yrk: **@CODE@** flexible ropelike object
    /// sms: **@CODE@** = flexible ropelike object
    /// mrj: **@CODE@** flexible ropelike object
    /// myv: (no comment)
    /// olo: **@CODE@** flexible ropelike object
    /// koi: **@CODE@** flexible ropelike object
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** flexible ropelike object
    /// vro: **@CODE@** flexible ropelike object
    #[strum(serialize = "Sem/Obj-rope")]
    Sem_SLASH_Obj_MINUS_rope,
    /// kpv: **@CODE@** Surface object
    /// izh: **@CODE@** Surface object
    /// liv: **@CODE@** Surface object
    /// mhr: **@CODE@** = Surface object
    /// krl: **@CODE@** Surface object
    /// sma: (no comment)
    /// smj: **@CODE@** = Surface object; blackboard, drawing paper, canvas, board (for playing board games), (money) bill
    /// udm: **@CODE@** Surface object
    /// yrk: **@CODE@** Surface object
    /// sms: **@CODE@** = Surface object
    /// mrj: **@CODE@** Surface object
    /// myv: (no comment)
    /// olo: **@CODE@** Surface object
    /// koi: **@CODE@** Surface object
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Surface object
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj_Semcon")]
    Sem_SLASH_Obj_Semcon,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Obj_Sign")]
    Sem_SLASH_Obj_Sign,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Obj_State")]
    Sem_SLASH_Obj_State,
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// smn: (no comment)
    #[strum(serialize = "Sem/Obj_Veh")]
    Sem_SLASH_Obj_Veh,
    /// sms: **@CODE@** (not used so far (?)), note oeu-
    #[strum(serialize = "Sem/Oeuvre")]
    Sem_SLASH_Oeuvre,
    /// sms: **@CODE@** (so far only Biblija, changed to Sem/Txt)
    #[strum(serialize = "Sem/Oeuvre-txt")]
    Sem_SLASH_Oeuvre_MINUS_txt,
    /// fit: **@CODE@** = Names of organisations
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Organisation
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Names of organisations
    /// izh: **@CODE@** Organisation
    /// liv: **@CODE@** Organisation
    /// mhr: **@CODE@** = Organisation
    /// krl: **@CODE@** Organisation
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Organisation; newspaper, supreme court, company, museum, administration
    /// udm: **@CODE@** Organisation
    /// nio: (no comment)
    /// yrk: **@CODE@** Organisation
    /// mns: (no comment)
    /// sms: **@CODE@** Organization (names)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Organisation
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Organisation
    /// koi: **@CODE@** Organisation
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Organisation
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Organisation
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Org")]
    Sem_SLASH_Org,
    /// sme: (no comment)
    /// smn: (no comment)
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
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Org_Prod-audio")]
    Sem_SLASH_Org_Prod_MINUS_audio,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Org_Prod-cogn")]
    Sem_SLASH_Org_Prod_MINUS_cogn,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Org_Prod-vis")]
    Sem_SLASH_Org_Prod_MINUS_vis,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Org_Rule")]
    Sem_SLASH_Org_Rule,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Org_State")]
    Sem_SLASH_Org_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Org_Txt")]
    Sem_SLASH_Org_Txt,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Org_Veh")]
    Sem_SLASH_Org_Veh,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Feature, oassi, bealli
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Feature, oassi, bealli
    /// liv: **@CODE@** Feature, oassi, bealli
    /// mhr: **@CODE@** = Feature, oassi, bealli
    /// krl: **@CODE@** Feature, oassi, bealli
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Feature, part of something; half, part, percent, rest, tenth
    /// udm: **@CODE@** Feature, oassi, bealli
    /// yrk: **@CODE@** Feature, oassi, bealli
    /// sms: **@CODE@** = Feature, oassi, bealli
    /// rmn: (no comment)
    /// mrj: **@CODE@** Feature, oassi, bealli
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Feature, oassi, bealli
    /// koi: **@CODE@** Feature, oassi, bealli
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Feature, oassi, bealli
    /// vro: **@CODE@** Feature, oassi, bealli
    /// bak: (no comment)
    #[strum(serialize = "Sem/Part")]
    Sem_SLASH_Part,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Part_Plc")]
    Sem_SLASH_Part_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Part_Plc_Prod-audio")]
    Sem_SLASH_Part_Plc_Prod_MINUS_audio,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Part_Prod-cogn")]
    Sem_SLASH_Part_Prod_MINUS_cogn,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Part_Substnc")]
    Sem_SLASH_Part_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Part_Txt")]
    Sem_SLASH_Part_Txt,
    /// kpv: **@CODE@**  Patronym
    /// udm: (no comment)
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// koi: **@CODE@** Patronymic
    /// mdf: (no comment)
    #[strum(serialize = "Sem/Patr")]
    Sem_SLASH_Patr,
    /// kpv: **@CODE@**  Patronym female
    /// krl: (no comment)
    /// udm: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** Female Patronymic
    #[strum(serialize = "Sem/Patr-Fem")]
    Sem_SLASH_Patr_MINUS_Fem,
    /// kpv: **@CODE@**  Patronym male
    /// krl: (no comment)
    /// udm: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** Male Patronymic
    #[strum(serialize = "Sem/Patr-Mal")]
    Sem_SLASH_Patr_MINUS_Mal,
    /// kpv: **@CODE@** Cognative perception
    /// izh: **@CODE@** Cognative perception
    /// liv: **@CODE@** Cognative perception
    /// mhr: **@CODE@** = Cognative perception
    /// krl: **@CODE@** Cognative perception
    /// smj: **@CODE@** = Cloth
    /// udm: **@CODE@** Cognative perception
    /// yrk: **@CODE@** Cognative perception
    /// sms: **@CODE@** = Cloth
    /// mrj: **@CODE@** Cognative perception
    /// myv: (no comment)
    /// olo: **@CODE@** Cognative perception
    /// koi: **@CODE@** Cognative perception
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Cognative perception
    /// vro: **@CODE@** Cognative perception
    #[strum(serialize = "Sem/Perc-cogn")]
    Sem_SLASH_Perc_MINUS_cogn,
    /// kpv: **@CODE@** Emotional perception
    /// izh: **@CODE@** Emotional perception
    /// liv: **@CODE@** Emotional perception
    /// mhr: **@CODE@** = Emotional perception
    /// krl: **@CODE@** Emotional perception
    /// sma: (no comment)
    /// smj: **@CODE@** = Emotional perception; fear, feeling of identity, empathy, sadness, working motivation
    /// udm: **@CODE@** Emotional perception
    /// yrk: **@CODE@** Emotional perception
    /// sms: **@CODE@** = Emotional perception
    /// mrj: **@CODE@** Emotional perception
    /// myv: (no comment)
    /// olo: **@CODE@** Emotional perception
    /// koi: **@CODE@** Emotional perception
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Emotional perception
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Perc-emo_Wthr")]
    Sem_SLASH_Perc_MINUS_emo_Wthr,
    /// kpv: **@CODE@** Physical perception
    /// izh: **@CODE@** Physical perception
    /// liv: **@CODE@** Physical perception
    /// mhr: **@CODE@** = Physical perception
    /// krl: **@CODE@** Physical perception
    /// sma: (no comment)
    /// smj: **@CODE@** = Physical perception; shoulder pain, gass smell, sleep (during the night), need of sleep, hunger
    /// udm: **@CODE@** Physical perception
    /// yrk: **@CODE@** Physical perception
    /// sms: **@CODE@** = Physical perception
    /// mrj: **@CODE@** Physical perception
    /// myv: (no comment)
    /// olo: **@CODE@** Physical perception
    /// koi: **@CODE@** Physical perception
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Physical perception
    /// vro: **@CODE@** Physical perception
    #[strum(serialize = "Sem/Perc-phys")]
    Sem_SLASH_Perc_MINUS_phys,
    /// kpv: **@CODE@** Physical perception
    /// izh: **@CODE@** Physical perception
    /// liv: **@CODE@** Physical perception
    /// mhr: **@CODE@** = Physical perception
    /// krl: **@CODE@** Physical perception
    /// smj: **@CODE@** = Psychic perception
    /// udm: **@CODE@** Physical perception
    /// yrk: **@CODE@** Physical perception
    /// sms: **@CODE@** = Psychical perception
    /// mrj: **@CODE@** Physical perception
    /// myv: (no comment)
    /// olo: **@CODE@** Physical perception
    /// koi: **@CODE@** Physical perception
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Physical perception
    /// vro: **@CODE@** Physical perception
    #[strum(serialize = "Sem/Perc-psych")]
    Sem_SLASH_Perc_MINUS_psych,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Phonenr")]
    Sem_SLASH_Phonenr,
    /// kpv: **@CODE@** Plant
    /// vot: (no comment)
    /// lav: (no comment)
    /// izh: **@CODE@** Plant
    /// liv: **@CODE@** Plant
    /// mhr: **@CODE@** = Plant
    /// krl: **@CODE@** Plant
    /// sma: (no comment)
    /// lut: (no comment)
    /// smj: **@CODE@** = Plant; lichen, plant, cucumber, bluebell, poisonous mushroom
    /// udm: **@CODE@** Plant
    /// nio: (no comment)
    /// yrk: **@CODE@** Plant
    /// mns: (no comment)
    /// sms: **@CODE@** = Plant
    /// mrj: **@CODE@** Plant
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Plant
    /// koi: **@CODE@** Plant
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Plant
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Plant
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Plant")]
    Sem_SLASH_Plant,
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plant-fungus")]
    Sem_SLASH_Plant_MINUS_fungus,
    /// kpv: **@CODE@** Plant part
    /// izh: **@CODE@** Plant part
    /// liv: **@CODE@** Plant part
    /// mhr: **@CODE@** = Plant part
    /// krl: **@CODE@** Plant part
    /// udm: **@CODE@** Plant part
    /// yrk: **@CODE@** Plant part
    /// sms: **@CODE@** = Plant part
    /// mrj: **@CODE@** Plant part
    /// myv: (no comment)
    /// olo: **@CODE@** Plant part
    /// koi: **@CODE@** Plant part
    /// deu: (no comment)
    /// mdf: **@CODE@** Plant part
    /// vro: **@CODE@** Plant part
    #[strum(serialize = "Sem/Plant-part")]
    Sem_SLASH_Plant_MINUS_part,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plant_Plant-part")]
    Sem_SLASH_Plant_Plant_MINUS_part,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plant_Plantpart")]
    Sem_SLASH_Plant_Plantpart,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plant_Plc")]
    Sem_SLASH_Plant_Plc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Plant_Time_Wthr")]
    Sem_SLASH_Plant_Time_Wthr,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plant_Tool")]
    Sem_SLASH_Plant_Tool,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plant_Tool-measr")]
    Sem_SLASH_Plant_Tool_MINUS_measr,
    /// sma: (no comment)
    /// smj: **@CODE@** = Plant part; twig, leaf, root, seed, pine trunk
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plantpart")]
    Sem_SLASH_Plantpart,
    /// smj: **@CODE@** = Play
    #[strum(serialize = "Sem/Play")]
    Sem_SLASH_Play,
    /// fit: **@CODE@** = Place names
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Place
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Place names
    /// izh: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// krl: **@CODE@** Place
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Place; world, nature, airport, workplace, fireplace
    /// udm: **@CODE@** Place
    /// nio: (no comment)
    /// yrk: **@CODE@** Place
    /// mns: **@CODE@**
    /// sms: **@CODE@** Place name   (names)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Place
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Place
    /// koi: **@CODE@** Place
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Place
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Place
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Plc")]
    Sem_SLASH_Plc,
    /// kpv: **@CODE@** Abstract place
    /// izh: **@CODE@** Abstract place
    /// liv: **@CODE@** Abstract place
    /// mhr: **@CODE@** = Abstract place
    /// krl: **@CODE@** Abstract place
    /// sma: (no comment)
    /// smj: **@CODE@** = Abstract place; bachelor level, job market, third place, Troms website, address
    /// udm: **@CODE@** Abstract place
    /// yrk: **@CODE@** Abstract place
    /// sms: **@CODE@** = Abstract place
    /// mrj: **@CODE@** Abstract place
    /// myv: (no comment)
    /// olo: **@CODE@** Abstract place
    /// koi: **@CODE@** Abstract place
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Abstract place
    /// vro: **@CODE@** Abstract place
    #[strum(serialize = "Sem/Plc-abstr")]
    Sem_SLASH_Plc_MINUS_abstr,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc-abstr_Rel_State")]
    Sem_SLASH_Plc_MINUS_abstr_Rel_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
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
    /// izh: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// krl: **@CODE@** Place
    /// sma: (no comment)
    /// smj: **@CODE@** = Elevated place; mountain, peak, mountain top, scree, volcano
    /// udm: **@CODE@** Place
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** = Place
    /// mrj: **@CODE@** Place
    /// myv: (no comment)
    /// olo: **@CODE@** Place
    /// koi: **@CODE@** Place
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Place
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc-elevate")]
    Sem_SLASH_Plc_MINUS_elevate,
    /// kpv: **@CODE@** Place
    /// izh: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// krl: **@CODE@** Place
    /// sma: (no comment)
    /// smj: **@CODE@** = Place limitations; national border, border, finish line, stop line, equator
    /// udm: **@CODE@** Place
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** = Place
    /// mrj: **@CODE@** Place
    /// myv: (no comment)
    /// olo: **@CODE@** Place
    /// koi: **@CODE@** Place
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Place
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc-line")]
    Sem_SLASH_Plc_MINUS_line,
    /// kpv: **@CODE@** Place
    /// izh: **@CODE@** Place
    /// liv: **@CODE@** Place
    /// mhr: **@CODE@** = Place
    /// krl: **@CODE@** Place
    /// sma: (no comment)
    /// smj: **@CODE@** = Water; river, lake, polar sea, sea, well
    /// udm: **@CODE@** Place
    /// yrk: **@CODE@** Place
    /// sms: **@CODE@** = Place
    /// mrj: **@CODE@** Place
    /// myv: (no comment)
    /// olo: **@CODE@** Place
    /// koi: **@CODE@** Place
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Place
    /// vro: **@CODE@** Place
    #[strum(serialize = "Sem/Plc-water")]
    Sem_SLASH_Plc_MINUS_water,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Pos")]
    Sem_SLASH_Plc_Pos,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Route")]
    Sem_SLASH_Plc_Route,
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plc_Semcon")]
    Sem_SLASH_Plc_Semcon,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plc_State")]
    Sem_SLASH_Plc_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Substnc")]
    Sem_SLASH_Plc_Substnc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Substnc_Wthr")]
    Sem_SLASH_Plc_Substnc_Wthr,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Time")]
    Sem_SLASH_Plc_Time,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plc_Time_Wthr")]
    Sem_SLASH_Plc_Time_Wthr,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Tool-catch")]
    Sem_SLASH_Plc_Tool_MINUS_catch,
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/Plc_Txt")]
    Sem_SLASH_Plc_Txt,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Plc_Wthr")]
    Sem_SLASH_Plc_Wthr,
    /// kpv: **@CODE@** Position (as in social position job)
    /// izh: **@CODE@** Position (as in social position job)
    /// liv: **@CODE@** Position (as in social position job)
    /// mhr: **@CODE@** = Position (as in social position job)
    /// krl: **@CODE@** Position (as in social position job)
    /// sma: (no comment)
    /// smj: **@CODE@** = Position (as in social position job); 50% position, presidency, fixed position, kindergarten place, role in society
    /// udm: **@CODE@** Position (as in social position job)
    /// yrk: **@CODE@** Position (as in social position job)
    /// sms: **@CODE@** = Position (as in social position job)
    /// mrj: **@CODE@** Position (as in social position job)
    /// myv: (no comment)
    /// olo: **@CODE@** Position (as in social position job)
    /// koi: **@CODE@** Position (as in social position job)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Position (as in social position job)
    /// vro: **@CODE@** Position (as in social position job)
    #[strum(serialize = "Sem/Pos")]
    Sem_SLASH_Pos,
    /// rmn: (no comment)
    #[strum(serialize = "Sem/Prg")]
    Sem_SLASH_Prg,
    /// kpv: **@CODE@** Process
    /// izh: **@CODE@** Process
    /// liv: **@CODE@** Process
    /// mhr: **@CODE@** = Process
    /// krl: **@CODE@** Process
    /// sma: (no comment)
    /// smj: **@CODE@** = Process
    /// udm: **@CODE@** Process
    /// yrk: **@CODE@** Process
    /// sms: **@CODE@** = Process
    /// mrj: **@CODE@** Process
    /// myv: (no comment)
    /// olo: **@CODE@** Process
    /// koi: **@CODE@** Process
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Process
    /// vro: **@CODE@** Process
    #[strum(serialize = "Sem/Process")]
    Sem_SLASH_Process,
    /// kpv: **@CODE@** Product
    /// izh: **@CODE@** Product
    /// liv: **@CODE@** Product
    /// mhr: **@CODE@** = Product
    /// krl: **@CODE@** Product
    /// sma: (no comment)
    /// smj: **@CODE@** = Product
    /// udm: **@CODE@** Product
    /// yrk: **@CODE@** Product
    /// sms: **@CODE@** = Product
    /// mrj: **@CODE@** Product
    /// myv: (no comment)
    /// olo: **@CODE@** Product
    /// koi: **@CODE@** Product
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Product
    /// vro: **@CODE@** Product
    #[strum(serialize = "Sem/Prod")]
    Sem_SLASH_Prod,
    /// kpv: **@CODE@** Audio product
    /// izh: **@CODE@** Audio product
    /// liv: **@CODE@** Audio product
    /// mhr: **@CODE@** = Audio product
    /// krl: **@CODE@** Audio product
    /// sma: (no comment)
    /// smj: **@CODE@** = Audio product; yoik, roar, Beatles song, Bible psalm, blues
    /// udm: **@CODE@** Audio product
    /// yrk: **@CODE@** Audio product
    /// sms: **@CODE@** = Audio product
    /// mrj: **@CODE@** Audio product
    /// myv: (no comment)
    /// olo: **@CODE@** Audio product
    /// koi: **@CODE@** Audio product
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Audio product
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
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Prod-audio_Txt")]
    Sem_SLASH_Prod_MINUS_audio_Txt,
    /// kpv: **@CODE@** Cognition product
    /// izh: **@CODE@** Cognition product
    /// liv: **@CODE@** Cognition product
    /// mhr: **@CODE@** = Cognition product
    /// krl: **@CODE@** Cognition product
    /// sma: (no comment)
    /// smj: **@CODE@** = Cognition product; thought, decision, knowledge, lack of understanding, requirement
    /// udm: **@CODE@** Cognition product
    /// yrk: **@CODE@** Cognition product
    /// sms: **@CODE@** = Cognition product
    /// mrj: **@CODE@** Cognition product
    /// myv: (no comment)
    /// olo: **@CODE@** Cognition product
    /// koi: **@CODE@** Cognition product
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Cognition product
    /// vro: **@CODE@** Cognition product
    #[strum(serialize = "Sem/Prod-cogn")]
    Sem_SLASH_Prod_MINUS_cogn,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Prod-cogn_Txt")]
    Sem_SLASH_Prod_MINUS_cogn_Txt,
    /// kpv: **@CODE@** Linguistic product
    /// izh: **@CODE@** Linguistic product
    /// liv: **@CODE@** Linguistic product
    /// mhr: **@CODE@** = Linguistic product
    /// krl: **@CODE@** Linguistic product
    /// sma: (no comment)
    /// smj: **@CODE@** = Linguistic product; message, question, agreement, translation, criticism
    /// udm: **@CODE@** Linguistic product
    /// yrk: **@CODE@** Linguistic product
    /// sms: **@CODE@** = Linguistic product
    /// mrj: **@CODE@** Linguistic product
    /// myv: (no comment)
    /// olo: **@CODE@** Linguistic product
    /// koi: **@CODE@** Linguistic product
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Linguistic product
    /// vro: **@CODE@** Linguistic product
    #[strum(serialize = "Sem/Prod-ling")]
    Sem_SLASH_Prod_MINUS_ling,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Visual product
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Visual product
    /// liv: **@CODE@** Visual product
    /// mhr: **@CODE@** = Visual product
    /// krl: **@CODE@** Visual product
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Visual product; picture, film, TV series, documentary, art
    /// udm: **@CODE@** Visual product
    /// yrk: **@CODE@** Visual product
    /// sms: **@CODE@** = Visual product
    /// rmn: (no comment)
    /// mrj: **@CODE@** Visual product
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Visual product
    /// koi: **@CODE@** Visual product
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Visual product
    /// vro: **@CODE@** Visual product
    /// bak: (no comment)
    #[strum(serialize = "Sem/Prod-vis")]
    Sem_SLASH_Prod_MINUS_vis,
    /// kpv: **@CODE@** Relation
    /// izh: **@CODE@** Relation
    /// liv: **@CODE@** Relation
    /// mhr: **@CODE@** = Relation
    /// krl: **@CODE@** Relation
    /// sma: (no comment)
    /// smj: **@CODE@** = Relation; relation, dependency, subordination, analogy, equivalence
    /// udm: **@CODE@** Relation
    /// yrk: **@CODE@** Relation
    /// sms: **@CODE@** = Relation
    /// mrj: **@CODE@** Relation
    /// myv: (no comment)
    /// olo: **@CODE@** Relation
    /// koi: **@CODE@** Relation
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Relation
    /// vro: **@CODE@** Relation
    #[strum(serialize = "Sem/Rel")]
    Sem_SLASH_Rel,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Name of a Route
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Name of a Route
    /// liv: **@CODE@** Name of a Route
    /// mhr: **@CODE@** = Name of a Route
    /// krl: **@CODE@** Name of a Route
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Route-like place; street, path, corridor, bridge, winter path
    /// udm: **@CODE@** Name of a Route
    /// nio: (no comment)
    /// yrk: **@CODE@** Name of a Route
    /// mns: (no comment)
    /// sms: **@CODE@** = Route
    /// rmn: (no comment)
    /// mrj: **@CODE@** Name of a Route
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Name of a Route
    /// koi: **@CODE@** Name of a Route
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Name of a Route
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Name of a Route
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Route")]
    Sem_SLASH_Route,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Route_State")]
    Sem_SLASH_Route_State,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Route_Txt")]
    Sem_SLASH_Route_Txt,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Rule or convention
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Rule or convention
    /// liv: **@CODE@** Rule or convention
    /// mhr: **@CODE@** = Rule or convention
    /// krl: **@CODE@** Rule or convention
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Rule or convention; cultural tradition, abortion law, EU rule, law of cosines, fair play
    /// udm: **@CODE@** Rule or convention
    /// yrk: **@CODE@** Rule or convention
    /// sms: **@CODE@** = Rule or convention
    /// rmn: (no comment)
    /// mrj: **@CODE@** Rule or convention
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Rule or convention
    /// koi: **@CODE@** Rule or convention
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Rule or convention
    /// vro: **@CODE@** Rule or convention
    /// bak: (no comment)
    #[strum(serialize = "Sem/Rule")]
    Sem_SLASH_Rule,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Rule_Txt")]
    Sem_SLASH_Rule_Txt,
    /// myv: (no comment)
    #[strum(serialize = "Sem/Rvr")]
    Sem_SLASH_Rvr,
    /// kpv: **@CODE@** Semantic concept
    /// izh: **@CODE@** Semantic concept
    /// liv: **@CODE@** Semantic concept
    /// mhr: **@CODE@** = Semantic concept
    /// krl: **@CODE@** Semantic concept
    /// sma: (no comment)
    /// smj: **@CODE@** = Abstract semantic concept; result, objective, reason, alternative, case
    /// udm: **@CODE@** Semantic concept
    /// yrk: **@CODE@** Semantic concept
    /// sms: **@CODE@** = Semantic concept
    /// mrj: **@CODE@** Semantic concept
    /// myv: (no comment)
    /// olo: **@CODE@** Semantic concept
    /// koi: **@CODE@** Semantic concept
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Semantic concept
    /// vro: **@CODE@** Semantic concept
    #[strum(serialize = "Sem/Semcon")]
    Sem_SLASH_Semcon,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/Semcon_State")]
    Sem_SLASH_Semcon_State,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Semcon_Txt")]
    Sem_SLASH_Semcon_Txt,
    /// sma: (no comment)
    #[strum(serialize = "Sem/Semcon_Wthr")]
    Sem_SLASH_Semcon_Wthr,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Sign (e.g. numbers, punctuation)
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Sign (e.g. numbers, punctuation)
    /// liv: **@CODE@** Sign (e.g. numbers, punctuation)
    /// mhr: **@CODE@** = Sign (e.g. numbers, punctuation)
    /// krl: **@CODE@** Sign (e.g. numbers, punctuation)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Sign (e.g. numbers, punctuation); ID number, ČSV letters, quotation mark, hieroglyph, symbol
    /// udm: **@CODE@** Sign (e.g. numbers, punctuation)
    /// yrk: **@CODE@** Sign (e.g. numbers, punctuation)
    /// sms: **@CODE@** = Sign (e.g. numbers, punctuation)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Sign (e.g. numbers, punctuation)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Sign (e.g. numbers, punctuation)
    /// koi: **@CODE@** Sign (e.g. numbers, punctuation)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Sign (e.g. numbers, punctuation)
    /// vro: **@CODE@** Sign (e.g. numbers, punctuation)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Sign")]
    Sem_SLASH_Sign,
    /// mpj: **@CODE@**
    #[strum(serialize = "Sem/Spat")]
    Sem_SLASH_Spat,
    /// kpv: **@CODE@** Sport
    /// izh: **@CODE@** Sport
    /// liv: **@CODE@** Sport
    /// mhr: **@CODE@** = Sport
    /// krl: **@CODE@** Sport
    /// sma: (no comment)
    /// smj: **@CODE@** = Sport; table tennis, judo, motor cross, ice hockey, floorball
    /// udm: **@CODE@** Sport
    /// yrk: **@CODE@** Sport
    /// sms: **@CODE@** = Sport
    /// mrj: **@CODE@** Sport
    /// myv: (no comment)
    /// olo: **@CODE@** Sport
    /// koi: **@CODE@** Sport
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Sport
    /// vro: **@CODE@** Sport
    #[strum(serialize = "Sem/Sport")]
    Sem_SLASH_Sport,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@**
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = State; hurry, captivity, anarchy, biodiversity, chaos
    /// udm: **@CODE@**
    /// yrk: **@CODE@**
    /// sms: **@CODE@** =
    /// rmn: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@**
    /// koi: **@CODE@**
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**
    /// vro: **@CODE@**
    /// bak: (no comment)
    #[strum(serialize = "Sem/State")]
    Sem_SLASH_State,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Illness
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Illness
    /// liv: **@CODE@** Illness
    /// mhr: **@CODE@** = Illness
    /// krl: **@CODE@** Illness
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Illness; allergy, cold, autism, dementia, somnambulism
    /// udm: **@CODE@** Illness
    /// yrk: **@CODE@** Illness
    /// sms: **@CODE@** = Illness
    /// rmn: (no comment)
    /// mrj: **@CODE@** Illness
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Illness
    /// koi: **@CODE@** Illness
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Illness
    /// vro: **@CODE@** Illness
    /// bak: (no comment)
    #[strum(serialize = "Sem/State-sick")]
    Sem_SLASH_State_MINUS_sick,
    /// smj: **@CODE@** =
    /// sms: **@CODE@**
    /// sme: (no comment)
    /// smn: (no comment)
    #[strum(serialize = "Sem/State-sick_Substnc")]
    Sem_SLASH_State_MINUS_sick_Substnc,
    /// smj: **@CODE@** =
    #[strum(serialize = "Sem/State_Veh")]
    Sem_SLASH_State_Veh,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Substance, like Air and Water
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Substance, like Air and Water
    /// liv: **@CODE@** Substance, like Air and Water
    /// mhr: **@CODE@** = Substance, like Air and Water
    /// krl: **@CODE@** Substance, like Air and Water
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Substance; Water, sand, air, smoke, carbohydrate, vitamin, dust
    /// udm: **@CODE@** Substance, like Air and Water
    /// yrk: **@CODE@** Substance, like Air and Water
    /// sms: **@CODE@** = Substance, like Air and Water
    /// rmn: (no comment)
    /// mrj: **@CODE@** Substance, like Air and Water
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Substance, like Air and Water
    /// koi: **@CODE@** Substance, like Air and Water
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Substance, like Air and Water
    /// vro: **@CODE@** Substance, like Air and Water
    /// bak: (no comment)
    #[strum(serialize = "Sem/Substnc")]
    Sem_SLASH_Substnc,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Substnc_Wthr")]
    Sem_SLASH_Substnc_Wthr,
    /// fit: **@CODE@** = Surnames
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Surname
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Surnames
    /// izh: **@CODE@** Surname
    /// liv: **@CODE@** Surname
    /// mhr: **@CODE@** = Surname
    /// krl: **@CODE@** Surname
    /// rue: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Surname
    /// udm: **@CODE@** Surname
    /// rus: (no comment)
    /// yrk: **@CODE@** Surname
    /// mns: (no comment)
    /// sms: **@CODE@** Surname       (names)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Surname
    /// vep: **@CODE@**  =
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** Surname
    /// koi: **@CODE@** Surname
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Surname
    /// ceb: (no comment)
    /// vro: **@CODE@** Surname
    /// mpj: **@CODE@**
    #[strum(serialize = "Sem/Sur")]
    Sem_SLASH_Sur,
    /// kpv: **@CODE@** Surname female
    /// krl: **@CODE@** Female Surname
    /// udm: **@CODE@** Surname
    /// olo: **@CODE@** Female Surname
    /// koi: **@CODE@** Surname
    #[strum(serialize = "Sem/Sur-Fem")]
    Sem_SLASH_Sur_MINUS_Fem,
    /// kpv: **@CODE@** Surname male
    /// krl: **@CODE@** Male Surname
    /// udm: **@CODE@** Surname
    /// olo: **@CODE@** Male Surname
    /// koi: **@CODE@** Surname
    #[strum(serialize = "Sem/Sur-Mal")]
    Sem_SLASH_Sur_MINUS_Mal,
    /// kpv: **@CODE@** Symbol
    /// izh: **@CODE@** Symbol
    /// liv: **@CODE@** Symbol
    /// mhr: **@CODE@** = Symbol
    /// krl: **@CODE@** Symbol
    /// smj: **@CODE@** = Symbol
    /// udm: **@CODE@** Symbol
    /// yrk: **@CODE@** Symbol
    /// sms: **@CODE@** = Symbol
    /// mrj: **@CODE@** Symbol
    /// myv: (no comment)
    /// olo: **@CODE@** Symbol
    /// koi: **@CODE@** Symbol
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Symbol
    /// vro: **@CODE@** Symbol
    #[strum(serialize = "Sem/Symbol")]
    Sem_SLASH_Symbol,
    /// mpj: **@CODE@**
    #[strum(serialize = "Sem/Temp")]
    Sem_SLASH_Temp,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kal: **@CODE@** = Temporal particle !The random adverbial 'phrases' like -riutaa -isaa etc.
    /// kpv: **@CODE@** Time
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Time
    /// liv: **@CODE@** Time
    /// mhr: **@CODE@** = Time
    /// krl: **@CODE@** Time
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Time; áigi time, April, half an hour, Tuesday, deadline
    /// udm: **@CODE@** Time
    /// nio: (no comment)
    /// yrk: **@CODE@** Time
    /// mns: (no comment)
    /// sms: **@CODE@** = Time
    /// sms: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Time
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Time
    /// koi: **@CODE@** Time
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Time
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Time
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Time")]
    Sem_SLASH_Time,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Time
    /// mns: (no comment)
    /// sms: **@CODE@** = Time
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Time-clock")]
    Sem_SLASH_Time_MINUS_clock,
    /// kpv: (no comment)
    /// izh: **@CODE@**
    /// liv: (no comment)
    /// mhr: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// smj: **@CODE@** =
    /// udm: (no comment)
    /// yrk: (no comment)
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "Sem/Time_Wthr")]
    Sem_SLASH_Time_Wthr,
    /// gle: **@CODE@** =
    #[strum(serialize = "Sem/Title")]
    Sem_SLASH_Title,
    /// kpv: **@CODE@** Prototypical tool for repairing things
    /// izh: **@CODE@** Prototypical tool for repairing things
    /// liv: **@CODE@** Prototypical tool for repairing things
    /// mhr: **@CODE@** = Prototypical tool for repairing things
    /// krl: **@CODE@** Prototypical tool for repairing things
    /// sma: (no comment)
    /// smj: **@CODE@** = Prototypical tool for repairing things; axe, knife, fire striker, plastic hammer, wrench
    /// udm: **@CODE@** Prototypical tool for repairing things
    /// yrk: **@CODE@** Prototypical tool for repairing things
    /// sms: **@CODE@** = Prototypical tool for repairing things
    /// mrj: **@CODE@** Prototypical tool for repairing things
    /// myv: (no comment)
    /// olo: **@CODE@** Prototypical tool for repairing things
    /// koi: **@CODE@** Prototypical tool for repairing things
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Prototypical tool for repairing things
    /// vro: **@CODE@** Prototypical tool for repairing things
    #[strum(serialize = "Sem/Tool")]
    Sem_SLASH_Tool,
    /// kpv: **@CODE@** Tool used for catching (e.g. fish)
    /// izh: **@CODE@** Tool used for catching (e.g. fish)
    /// liv: **@CODE@** Tool used for catching (e.g. fish)
    /// mhr: **@CODE@** = Tool used for catching (e.g. fish)
    /// krl: **@CODE@** Tool used for catching (e.g. fish)
    /// sma: (no comment)
    /// smj: **@CODE@** = Tool used for catching; artificial fly, fishing net for cod, fishing rod, lasso used in wintertime, mouse trap
    /// udm: **@CODE@** Tool used for catching (e.g. fish)
    /// yrk: **@CODE@** Tool used for catching (e.g. fish)
    /// sms: **@CODE@** = Tool used for catching (e.g. fish)
    /// mrj: **@CODE@** Tool used for catching (e.g. fish)
    /// myv: (no comment)
    /// olo: **@CODE@** Tool used for catching (e.g. fish)
    /// koi: **@CODE@** Tool used for catching (e.g. fish)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Tool used for catching (e.g. fish)
    /// vro: **@CODE@** Tool used for catching (e.g. fish)
    #[strum(serialize = "Sem/Tool-catch")]
    Sem_SLASH_Tool_MINUS_catch,
    /// kpv: **@CODE@** Tool used for cleaning
    /// izh: **@CODE@** Tool used for cleaning
    /// liv: **@CODE@** Tool used for cleaning
    /// mhr: **@CODE@** = Tool used for cleaning
    /// krl: **@CODE@** Tool used for cleaning
    /// sma: (no comment)
    /// smj: **@CODE@** = Tool used for cleaning; broom, vegetable brush, toothbrush, cleaning equipment
    /// udm: **@CODE@** Tool used for cleaning
    /// yrk: **@CODE@** Tool used for cleaning
    /// sms: **@CODE@** = Tool used for cleaning
    /// mrj: **@CODE@** Tool used for cleaning
    /// myv: (no comment)
    /// olo: **@CODE@** Tool used for cleaning
    /// koi: **@CODE@** Tool used for cleaning
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Tool used for cleaning
    /// vro: **@CODE@** Tool used for cleaning
    #[strum(serialize = "Sem/Tool-clean")]
    Sem_SLASH_Tool_MINUS_clean,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Tool used in IT
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Tool used in IT
    /// liv: **@CODE@** Tool used in IT
    /// mhr: **@CODE@** = Tool used in IT
    /// krl: **@CODE@** Tool used in IT
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Tool used in IT/tool within IT; IT infrastructure, analyzer, searching function, computer program, browser
    /// udm: **@CODE@** Tool used in IT
    /// yrk: **@CODE@** Tool used in IT
    /// sms: **@CODE@** = Tool used in IT
    /// rmn: (no comment)
    /// mrj: **@CODE@** Tool used in IT
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** Tool used in IT
    /// koi: **@CODE@** Tool used in IT
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Tool used in IT
    /// vro: **@CODE@** Tool used in IT
    /// bak: (no comment)
    #[strum(serialize = "Sem/Tool-it")]
    Sem_SLASH_Tool_MINUS_it,
    /// kpv: **@CODE@** Tool used for measuring
    /// izh: **@CODE@** Tool used for measuring
    /// liv: **@CODE@** Tool used for measuring
    /// mhr: **@CODE@** = Tool used for measuring
    /// krl: **@CODE@** Tool used for measuring
    /// smj: **@CODE@** = Tool used for measuring; barometer, hourglass, ruler, spirit level, scale
    /// udm: **@CODE@** Tool used for measuring
    /// yrk: **@CODE@** Tool used for measuring
    /// sms: **@CODE@** = Tool used for measuring
    /// mrj: **@CODE@** Tool used for measuring
    /// myv: (no comment)
    /// olo: **@CODE@** Tool used for measuring
    /// koi: **@CODE@** Tool used for measuring
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Tool used for measuring
    /// vro: **@CODE@** Tool used for measuring
    #[strum(serialize = "Sem/Tool-measr")]
    Sem_SLASH_Tool_MINUS_measr,
    /// kpv: **@CODE@** Music instrument
    /// izh: **@CODE@** Music instrument
    /// liv: **@CODE@** Music instrument
    /// mhr: **@CODE@** = Music instrument
    /// krl: **@CODE@** Music instrument
    /// sma: (no comment)
    /// smj: **@CODE@** = Musical instrument; shaman drum, guitar, violin, musical instrument, jaw harp
    /// smj: **@CODE@** =
    /// udm: **@CODE@** Music instrument
    /// yrk: **@CODE@** Music instrument
    /// sms: **@CODE@** = Music instrument
    /// mrj: **@CODE@** Music instrument
    /// myv: (no comment)
    /// olo: **@CODE@** Music instrument
    /// koi: **@CODE@** Music instrument
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Music instrument
    /// vro: **@CODE@** Music instrument
    #[strum(serialize = "Sem/Tool-music")]
    Sem_SLASH_Tool_MINUS_music,
    /// kpv: **@CODE@** Writing tool
    /// izh: **@CODE@** Writing tool
    /// liv: **@CODE@** Writing tool
    /// mhr: **@CODE@** = Writing tool
    /// krl: **@CODE@** Writing tool
    /// sma: (no comment)
    /// smj: **@CODE@** = Writing tool; colored pen, pencil, chalk, paintbrush, paint
    /// udm: **@CODE@** Writing tool
    /// yrk: **@CODE@** Writing tool
    /// sms: **@CODE@** = Writing tool
    /// mrj: **@CODE@** Writing tool
    /// myv: (no comment)
    /// olo: **@CODE@** Writing tool
    /// koi: **@CODE@** Writing tool
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Writing tool
    /// vro: **@CODE@** Writing tool
    #[strum(serialize = "Sem/Tool-write")]
    Sem_SLASH_Tool_MINUS_write,
    /// krl: (no comment)
    #[strum(serialize = "Sem/Top")]
    Sem_SLASH_Top,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Text (girji, lávlla...)
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Text (girji, lávlla...)
    /// liv: **@CODE@** Text (girji, lávlla...)
    /// mhr: **@CODE@** = Text (girji, lávlla...)
    /// krl: **@CODE@** Text (girji, lávlla...)
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Written document; paper, book, letter, e-mail, song
    /// udm: **@CODE@** Text (girji, lávlla...)
    /// nio: (no comment)
    /// yrk: **@CODE@** Text (girji, lávlla...)
    /// mns: (no comment)
    /// sms: **@CODE@** = Text (girji, lávlla...)
    /// rmn: (no comment)
    /// mrj: **@CODE@** Text (girji, lávlla...)
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Text (girji, lávlla...)
    /// koi: **@CODE@** Text (girji, lávlla...)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Text (girji, lávlla...)
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Text (girji, lávlla...)
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Txt")]
    Sem_SLASH_Txt,
    /// kal: **@CODE@** = Unit !Til besværlige låneord som 'time' etc., der optræder absolut
    #[strum(serialize = "Sem/Unit")]
    Sem_SLASH_Unit,
    /// kal: **@CODE@** tag til adresser af typen www.ccc.cc
    #[strum(serialize = "Sem/Url")]
    Sem_SLASH_Url,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Vehicle
    /// swe: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** =
    /// izh: **@CODE@** Vehicle
    /// liv: **@CODE@** Vehicle
    /// mhr: **@CODE@** = Vehicle
    /// krl: **@CODE@** Vehicle
    /// sma: (no comment)
    /// lut: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Vehicle; car, boat, sled, vehicle, draft reindeer, bicycle
    /// udm: **@CODE@** Vehicle
    /// nio: (no comment)
    /// yrk: **@CODE@** Vehicle
    /// mns: (no comment)
    /// sms: **@CODE@** = Vehicle
    /// rmn: (no comment)
    /// mrj: **@CODE@** Vehicle
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** Vehicle
    /// koi: **@CODE@** Vehicle
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@** Vehicle
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** Vehicle
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Veh")]
    Sem_SLASH_Veh,
    /// kal: **@CODE@** =  Vehicle, ground
    #[strum(serialize = "Sem/Vground")]
    Sem_SLASH_Vground,
    /// kpv: **@CODE@** Weapon
    /// izh: **@CODE@** Weapon
    /// liv: **@CODE@** Weapon
    /// mhr: **@CODE@** = Weapon
    /// krl: **@CODE@** Weapon
    /// sma: (no comment)
    /// smj: **@CODE@** = Weapon; rifle, bow, sword, arrow, war axe
    /// udm: **@CODE@** Weapon
    /// yrk: **@CODE@** Weapon
    /// sms: **@CODE@** = Weapon
    /// mrj: **@CODE@** Weapon
    /// myv: (no comment)
    /// olo: **@CODE@** Weapon
    /// koi: **@CODE@** Weapon
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** Weapon
    /// vro: **@CODE@** Weapon
    #[strum(serialize = "Sem/Wpn")]
    Sem_SLASH_Wpn,
    /// kpv: **@CODE@** The Weather or the state of ground
    /// vot: (no comment)
    /// lav: (no comment)
    /// izh: **@CODE@** The Weather or the state of ground
    /// liv: **@CODE@** The Weather or the state of ground
    /// mhr: **@CODE@** = The Weather or the state of ground
    /// krl: **@CODE@** The Weather or the state of ground
    /// sma: (no comment)
    /// lut: (no comment)
    /// smj: **@CODE@** = The Weather or the state of ground; cloudy weather, wind, driving conditions, night sunlight, rain shower
    /// udm: **@CODE@** The Weather or the state of ground
    /// nio: (no comment)
    /// yrk: **@CODE@** The Weather or the state of ground
    /// mns: (no comment)
    /// sms: **@CODE@** = The Weather or the state of ground
    /// mrj: **@CODE@** The Weather or the state of ground
    /// vep: **@CODE@**  =
    /// rup: (no comment)
    /// tat: (no comment)
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@** The Weather or the state of ground
    /// koi: **@CODE@** The Weather or the state of ground
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// mdf: **@CODE@** The Weather or the state of ground
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// vro: **@CODE@** The Weather or the state of ground
    /// mpj: **@CODE@**
    /// kca: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Sem/Wthr")]
    Sem_SLASH_Wthr,
    /// fit: **@CODE@**
    /// gle: **@CODE@** =
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// krl: (no comment)
    /// sma: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** = Year
    /// sms: **@CODE@** = Year
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@**
    /// bak: (no comment)
    #[strum(serialize = "Sem/Year")]
    Sem_SLASH_Year,
    /// kal: (no comment)
    #[strum(serialize = "Sem/ac-cat")]
    Sem_SLASH_ac_MINUS_cat,
    /// kal: (no comment)
    #[strum(serialize = "Sem/be_attribute")]
    Sem_SLASH_be_attribute,
    /// kal: **@CODE@** =  Object Countable, human-made
    #[strum(serialize = "Sem/cc-h")]
    Sem_SLASH_cc_MINUS_h,
    /// kal: (no comment)
    #[strum(serialize = "Sem/characterize")]
    Sem_SLASH_characterize,
    /// kal: **@CODE@** =  Object Mass Noun, human-made
    #[strum(serialize = "Sem/cm-h")]
    Sem_SLASH_cm_MINUS_h,
    /// kal: (no comment)
    #[strum(serialize = "Sem/cm-liq-h")]
    Sem_SLASH_cm_MINUS_liq_MINUS_h,
    /// kal: (no comment)
    #[strum(serialize = "Sem/cm-rem")]
    Sem_SLASH_cm_MINUS_rem,
    /// krl: (no comment)
    #[strum(serialize = "Sem/cog")]
    Sem_SLASH_cog,
    /// kal: (no comment)
    #[strum(serialize = "Sem/create_semantic")]
    Sem_SLASH_create_semantic,
    /// kal: (no comment)
    #[strum(serialize = "Sem/do")]
    Sem_SLASH_do,
    /// kal: **@CODE@** =  Domain
    #[strum(serialize = "Sem/domain")]
    Sem_SLASH_domain,
    /// kal: **@CODE@** =  Temporal, duration
    #[strum(serialize = "Sem/dur")]
    Sem_SLASH_dur,
    /// kal: (no comment)
    #[strum(serialize = "Sem/end")]
    Sem_SLASH_end,
    /// kal: **@CODE@** fx Avannaata Qimussersua
    #[strum(serialize = "Sem/event")]
    Sem_SLASH_event,
    /// kal: **@CODE@** =  Feature, anatomical
    #[strum(serialize = "Sem/f-an")]
    Sem_SLASH_f_MINUS_an,
    /// kal: **@CODE@** =  Feature, quantifiable
    #[strum(serialize = "Sem/f-q")]
    Sem_SLASH_f_MINUS_q,
    /// kal: (no comment)
    #[strum(serialize = "Sem/grow")]
    Sem_SLASH_grow,
    /// kal: **@CODE@** =  Institution
    #[strum(serialize = "Sem/inst")]
    Sem_SLASH_inst,
    /// kal: (no comment)
    #[strum(serialize = "Sem/jcrea")]
    Sem_SLASH_jcrea,
    /// kal: (no comment)
    #[strum(serialize = "Sem/jimp")]
    Sem_SLASH_jimp,
    /// kal: (no comment)
    #[strum(serialize = "Sem/jquant")]
    Sem_SLASH_jquant,
    /// kal: (no comment)
    #[strum(serialize = "Sem/jshape")]
    Sem_SLASH_jshape,
    /// kal: (no comment)
    #[strum(serialize = "Sem/jsize")]
    Sem_SLASH_jsize,
    /// kal: (no comment)
    #[strum(serialize = "Sem/jtemp")]
    Sem_SLASH_jtemp,
    /// kal: (no comment)
    #[strum(serialize = "Sem/juse")]
    Sem_SLASH_juse,
    /// kal: **@CODE@** =  Language
    #[strum(serialize = "Sem/ling")]
    Sem_SLASH_ling,
    /// kal: **@CODE@** tag til at markere usikker possessor for misse
    #[strum(serialize = "Sem/misse")]
    Sem_SLASH_misse,
    /// kal: **@CODE@** =  Temporal, month
    #[strum(serialize = "Sem/month")]
    Sem_SLASH_month,
    /// kal: (no comment)
    #[strum(serialize = "Sem/move_dir")]
    Sem_SLASH_move_dir,
    /// kal: **@CODE@** =  Temporal, occasion
    #[strum(serialize = "Sem/occ")]
    Sem_SLASH_occ,
    /// kal: **@CODE@** =  Temporal, period of/in time
    #[strum(serialize = "Sem/per")]
    Sem_SLASH_per,
    /// kal: (no comment)
    #[strum(serialize = "Sem/reach")]
    Sem_SLASH_reach,
    /// kal: (no comment)
    #[strum(serialize = "Sem/remove")]
    Sem_SLASH_remove,
    /// kal: (no comment)
    #[strum(serialize = "Sem/rush")]
    Sem_SLASH_rush,
    /// kal: (no comment)
    #[strum(serialize = "Sem/sem")]
    Sem_SLASH_sem,
    /// kal: **@CODE@** =  Disease
    #[strum(serialize = "Sem/sick")]
    Sem_SLASH_sick,
    /// kal: **@CODE@** =  Temporal
    #[strum(serialize = "Sem/temp")]
    Sem_SLASH_temp,
    /// kal: **@CODE@** =  Therapy
    #[strum(serialize = "Sem/therapy")]
    Sem_SLASH_therapy,
    /// kal: **@CODE@** =  Tool
    #[strum(serialize = "Sem/tool")]
    Sem_SLASH_tool,
    /// kal: (no comment)
    #[strum(serialize = "Sem/turn_into")]
    Sem_SLASH_turn_into,
    /// kal: (no comment)
    #[strum(serialize = "Sem/use")]
    Sem_SLASH_use,
    /// kal: (no comment)
    #[strum(serialize = "Sem/vehicle")]
    Sem_SLASH_vehicle,
    /// kal: **@CODE@** =  Weather
    #[strum(serialize = "Sem/wea")]
    Sem_SLASH_wea,
    /// kmr: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Sent")]
    Sent,
    /// smj: (no comment)
    /// smn: **@CODE@**
    #[strum(serialize = "SentInit")]
    SentInit,
    /// mpj: **@CODE@**
    #[strum(serialize = "SentMod")]
    SentMod,
    /// som: (no comment)
    /// tlh: (no comment)
    #[strum(serialize = "Sep")]
    Sep,
    /// mpj: **@CODE@** Serial
    #[strum(serialize = "Ser")]
    Ser,
    /// hun: (no comment)
    #[strum(serialize = "Ses")]
    Ses,
    /// fit: **@CODE@** = Singular
    /// sje: (no comment)
    /// gle: **@CODE@** = Singular
    /// kmr: (no comment)
    /// kal: **@CODE@** = Singularis
    /// kpv: **@CODE@**  singular
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// bla: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Singular
    /// izh: **@CODE@**
    /// liv: **@CODE@** = singular
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// amh: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** = Singular
    /// lut: (no comment)
    /// sjt: (no comment)
    /// ara: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@** =
    /// got: (no comment)
    /// hdn: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** = Singular number
    /// udm: (no comment)
    /// tqb: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** Singular
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// xwo: (no comment)
    /// mns: **@CODE@**
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
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@**
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = singular
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// ess: (no comment)
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// zxx: (no comment)
    /// grn: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@**  singular
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// xak: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Singular
    /// smn: **@CODE@**
    /// srs: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  singular
    /// ale: (no comment)
    /// rmf: **@CODE@** singular
    /// ceb: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// ces: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@** Singular
    /// kjh: (no comment)
    /// ipk: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// iku: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Sg")]
    Sg,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kpv: **@CODE@** person тэа-меа
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Singular 1
    /// izh: **@CODE@** Verb person tags
    /// liv: **@CODE@** = first person singular conjugation
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@** | Singular, 1.person
    /// lut: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@** =
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// rus: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**  Personal pronouns are marked as
    /// sms: **@CODE@** first person singular
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** = Verb personal forms
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@** Singular First Person
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// nso: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Singular First Person
    /// smn: **@CODE@**
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** Verb personal tags
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// ces: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Sg1")]
    Sg1,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Singular 2
    /// liv: **@CODE@** = second person singular conjugation
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** | Singular, 2.person
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** second person singular
    /// bxr: (no comment)
    /// vep: **@CODE@** Singular Second Person
    /// myv: (no comment)
    /// tkl: (no comment)
    /// sme: **@CODE@** Singular Second Person
    /// fin: (no comment)
    /// fin: (no comment)
    /// ceb: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "Sg2")]
    Sg2,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = Singular 3
    /// liv: **@CODE@** = third person singular conjugation
    /// mhr: **@CODE@** =
    /// evn: (no comment)
    /// sma: **@CODE@** | Singular, 3.person
    /// fao: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@** third person singular
    /// bxr: (no comment)
    /// vep: **@CODE@** Singular Third Person
    /// myv: (no comment)
    /// tkl: (no comment)
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
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// tku: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "SgO")]
    SgO,
    /// smj: **@CODE@**  Short form
    #[strum(serialize = "Sh")]
    Sh,
    /// gle: **@CODE@** = Short determiner, e.g. m', d'
    /// hdn: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Short")]
    Short,
    /// khk: (no comment)
    #[strum(serialize = "Sim")]
    Sim,
    /// gle: **@CODE@** = Simple preposition
    /// hdn: (no comment)
    #[strum(serialize = "Simp")]
    Simp,
    /// qya: (no comment)
    /// epo: (no comment)
    #[strum(serialize = "Sing")]
    Sing,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Sint")]
    Sint,
    /// hdn: (no comment)
    #[strum(serialize = "Sit-Pl")]
    Sit_MINUS_Pl,
    /// hdn: (no comment)
    #[strum(serialize = "Sit-Sg")]
    Sit_MINUS_Sg,
    /// hun: (no comment)
    #[strum(serialize = "Sla")]
    Sla,
    /// gle: **@CODE@** = Adj qualifies a plural noun ending in a slender consonant
    #[strum(serialize = "Slender")]
    Slender,
    /// cor: (no comment)
    #[strum(serialize = "Smut")]
    Smut,
    /// kpv: **@CODE@** segment ordering: case, person
    /// mhr: **@CODE@** = Suffix ordering: Case + Possessive Person marking
    /// mrj: **@CODE@** = Suffix ordering: Case + Possessive Person marking
    /// koi: (no comment)
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
    /// koi: (no comment)
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
    /// tgl: **@CODE@** - Social
    /// ceb: **@CODE@** - Social
    #[strum(serialize = "Soc")]
    Soc,
    /// sms: **@CODE@**
    /// sme: **@CODE@** foreløpig lagt til Sg Loc -n, som er en sub-form
    #[strum(serialize = "South")]
    South,
    /// fit: **@CODE@** = ?
    /// gle: **@CODE@** =
    /// fkv: **@CODE@** =
    /// sma: (no comment)
    /// nob: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    /// fao: **@CODE@** – used to designate numerical spans, like date or length ranges
    /// smj: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// olo: (no comment)
    /// sme: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    /// smn: **@CODE@** - used for numerical expressions denoting spans or intervals, like 5-10, 2012-2015, etc
    /// mdf: **@CODE@**:  this must be n-dash in numbers
    #[strum(serialize = "Span")]
    Span,
    /// kpv: **@CODE@** spatial
    /// vot: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// smn: **@CODE@**    Spatial adverbs
    /// mdf: **@CODE@**:  spatial
    /// kca: (no comment)
    /// kca: (no comment)
    /// kca: (no comment)
    /// kca: (no comment)
    #[strum(serialize = "Spat")]
    Spat,
    /// fin: (no comment)
    #[strum(serialize = "Spell/NoSugg")]
    Spell_SLASH_NoSugg,
    /// kpv: **@CODE@** foreign source apparently 2015-09-08
    /// koi: **@CODE@** foreign source apparently 2015-09-08
    #[strum(serialize = "Src/F")]
    Src_SLASH_F,
    /// gle: **@CODE@** = start bracket, quote etc
    #[strum(serialize = "St")]
    St,
    /// hdn: (no comment)
    #[strum(serialize = "Stand-Pl")]
    Stand_MINUS_Pl,
    /// hdn: (no comment)
    #[strum(serialize = "Stand-Sg")]
    Stand_MINUS_Sg,
    /// gle: **@CODE@** = start of error/correction
    #[strum(serialize = "Start")]
    Start,
    /// bla: (no comment)
    #[strum(serialize = "Stat")]
    Stat,
    /// vot: (no comment)
    /// koi: (no comment)
    #[strum(serialize = "State")]
    State,
    /// myv: (no comment)
    #[strum(serialize = "Stem/е")]
    Stem_SLASH_е,
    /// myv: (no comment)
    #[strum(serialize = "Stem/э")]
    Stem_SLASH_э,
    /// fro: (no comment)
    #[strum(serialize = "Stress")]
    Stress,
    /// gle: **@CODE@** = same plural form for all cases
    #[strum(serialize = "Strong")]
    Strong,
    /// som: (no comment)
    #[strum(serialize = "Sty")]
    Sty,
    /// som: (no comment)
    #[strum(serialize = "Sty/D")]
    Sty_SLASH_D,
    /// som: (no comment)
    #[strum(serialize = "Sty/R")]
    Sty_SLASH_R,
    /// som: (no comment)
    #[strum(serialize = "Sty/TODO")]
    Sty_SLASH_TODO,
    /// som: (no comment)
    #[strum(serialize = "Sty/i")]
    Sty_SLASH_i,
    /// mya: (no comment)
    #[strum(serialize = "Sub")]
    Sub,
    /// gle: **@CODE@** = Subjunctive mood/particle
    /// som: (no comment)
    /// eus: (no comment)
    /// mpj: **@CODE@**   Subj = abs with intransitive verb, erg with transitive verb
    #[strum(serialize = "Subj")]
    Subj,
    /// myv: (no comment)
    #[strum(serialize = "Subj/Zero")]
    Subj_SLASH_Zero,
    /// gle: **@CODE@** = Subordinating conjunction
    /// yrk: **@CODE@**
    #[strum(serialize = "Subord")]
    Subord,
    /// sma: **@CODE@** - used for adverbs
    /// sme: **@CODE@**   Embedded Question Particle: +Adv+Subqst
    #[strum(serialize = "Subqst")]
    Subqst,
    /// gle: **@CODE@** = substantive noun, functions like a noun, but lacks full inflectional pardigm
    /// myv: (no comment)
    #[strum(serialize = "Subst")]
    Subst,
    /// gle: **@CODE@** = -s vern suffix e.g. a bhíonns
    #[strum(serialize = "Suf")]
    Suf,
    /// epo: (no comment)
    /// cor: (no comment)
    /// fin: (no comment)
    #[strum(serialize = "Suff")]
    Suff,
    /// khk: (no comment)
    #[strum(serialize = "Sug")]
    Sug,
    /// gle: **@CODE@** = Superlative particle (s), e.g. is
    /// swe: (no comment)
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// som: (no comment)
    /// olo: **@CODE@**
    /// deu: (no comment)
    /// sme: **@CODE@** Supine
    /// khk: (no comment)
    /// vro: **@CODE@** olõman, olõmaldaq; oldama
    #[strum(serialize = "Sup")]
    Sup,
    /// fit: **@CODE@** =
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// vot: (no comment)
    /// lav: **@CODE@** superlative grade
    /// fkv: **@CODE@** = Superlative
    /// liv: **@CODE@**
    /// mhr: **@CODE@** = superlative
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@**:  superlative
    /// eus: (no comment)
    /// deu: (no comment)
    /// non: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  superlative
    /// vro: **@CODE@**
    #[strum(serialize = "Superl")]
    Superl,
    /// pur: (no comment)
    /// tgl: (no comment)
    /// txi: (no comment)
    /// tqb: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// waw: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// bgs: (no comment)
    /// myu: (no comment)
    /// rmu: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// pad: (no comment)
    /// gvp: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Sur")]
    Sur,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Surprise")]
    Surprise,
    /// fit: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// sje: (no comment)
    /// gle: (no comment)
    /// kmr: (no comment)
    /// kal: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// kpv: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// swe: (no comment)
    /// pur: (no comment)
    /// chp: (no comment)
    /// vot: (no comment)
    /// tuv: (no comment)
    /// lav: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// izh: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// liv: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// crk: (no comment)
    /// mhr: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// krl: (no comment)
    /// evn: (no comment)
    /// rue: (no comment)
    /// sjd: (no comment)
    /// amh: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@**:  Symbols in the text stream, like £, €, ©
    /// lut: (no comment)
    /// sjt: (no comment)
    /// ara: (no comment)
    /// nob: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// oji: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// otw: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// cor: (no comment)
    /// hun: (no comment)
    /// yrk: (no comment)
    /// xwo: (no comment)
    /// mns: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// sms: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// tku: (no comment)
    /// rmn: (no comment)
    /// moh: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**  = independent symbols in the text stream, like £, €, ©
    /// cho: (no comment)
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**  = independent symbols in the text stream, like £, €, ©
    /// ess: (no comment)
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// ron: (no comment)
    /// zxx: (no comment)
    /// grn: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// nso: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// hin: (no comment)
    /// olo: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// tel: (no comment)
    /// sme: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// smn: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// srs: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// ceb: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// ces: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: (no comment)
    /// kjh: (no comment)
    /// ipk: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@** = independent symbols in the text stream, like £, €, ©
    /// bwi: (no comment)
    /// iku: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Symbol")]
    Symbol,
    /// kal: **@CODE@**
    #[strum(serialize = "T")]
    T,
    /// kal: **@CODE@** = sukkatsippoq, men: nillertippoq
    #[strum(serialize = "T(S)IP")]
    T_LEFTPAREN_S_RIGHTPAREN_IP,
    /// kal: **@CODE@**
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "TA")]
    TA,
    /// kal: **@CODE@**
    #[strum(serialize = "TAAMA")]
    TAAMA,
    /// kal: **@CODE@**
    #[strum(serialize = "TAAQ")]
    TAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TAAR")]
    TAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TAAVA")]
    TAAVA,
    /// kal: **@CODE@**
    #[strum(serialize = "TALLAP")]
    TALLAP,
    /// kal: **@CODE@**
    #[strum(serialize = "TAQ")]
    TAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TAR")]
    TAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TARE")]
    TARE,
    /// kal: **@CODE@**
    #[strum(serialize = "TARIAARUTE")]
    TARIAARUTE,
    /// kal: **@CODE@**
    #[strum(serialize = "TARIAQ")]
    TARIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TARIAQAR")]
    TARIAQAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TASSAA")]
    TASSAA,
    /// apu: (no comment)
    #[strum(serialize = "TCaus")]
    TCaus,
    /// khk: (no comment)
    #[strum(serialize = "TD")]
    TD,
    /// kal: **@CODE@**
    #[strum(serialize = "TE")]
    TE,
    /// kal: **@CODE@** = pga. ~/langtech/shared-mul/src/fst/stems/telephone.lexc
    /// smj: (no comment)
    #[strum(serialize = "TEL")]
    TEL,
    /// vot: (no comment)
    #[strum(serialize = "TEMP")]
    TEMP,
    /// mhr: **@CODE@** =
    #[strum(serialize = "TEST")]
    TEST,
    /// cwd: (no comment)
    /// crk: (no comment)
    /// crj: (no comment)
    /// kio: (no comment)
    #[strum(serialize = "TI")]
    TI,
    /// kal: **@CODE@**
    #[strum(serialize = "TIGE")]
    TIGE,
    /// kal: **@CODE@**
    #[strum(serialize = "TIP")]
    TIP,
    /// kal: **@CODE@**
    #[strum(serialize = "TIR")]
    TIR,
    /// kal: **@CODE@**
    #[strum(serialize = "TITAAR")]
    TITAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TITIR")]
    TITIR,
    /// sma: **@CODE@** = Code for items that have not been modeled yet
    /// smj: **@CODE@** = Code for items that have not been modeled yet
    /// som: (no comment)
    #[strum(serialize = "TODO")]
    TODO,
    /// sje: (no comment)
    /// sme: (no comment)
    #[strum(serialize = "TRAB")]
    TRAB,
    /// sme: (no comment)
    #[strum(serialize = "TRNUM")]
    TRNUM,
    /// sje: (no comment)
    #[strum(serialize = "TRNUMAB")]
    TRNUMAB,
    /// kal: **@CODE@**
    #[strum(serialize = "TSAALI")]
    TSAALI,
    /// kal: **@CODE@**
    #[strum(serialize = "TSAALIUR")]
    TSAALIUR,
    /// kal: **@CODE@**
    #[strum(serialize = "TSAP")]
    TSAP,
    /// kal: **@CODE@**
    #[strum(serialize = "TSIALAK")]
    TSIALAK,
    /// kal: **@CODE@**
    #[strum(serialize = "TSIAQ")]
    TSIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TSIAR")]
    TSIAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TSIISAA")]
    TSIISAA,
    /// kal: **@CODE@**
    #[strum(serialize = "TSIR")]
    TSIR,
    /// kal: **@CODE@**
    #[strum(serialize = "TTAAQ")]
    TTAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TU")]
    TU,
    /// kal: **@CODE@**
    #[strum(serialize = "TUAQ")]
    TUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TUAR")]
    TUAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TUARANNGUU")]
    TUARANNGUU,
    /// kal: **@CODE@**
    #[strum(serialize = "TUATAAR")]
    TUATAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TUGAQ")]
    TUGAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TUKASSAA")]
    TUKASSAA,
    /// kal: **@CODE@**
    #[strum(serialize = "TUQ")]
    TUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TUQAQ")]
    TUQAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TUR")]
    TUR,
    /// kal: **@CODE@**
    #[strum(serialize = "TURSUU")]
    TURSUU,
    /// kal: **@CODE@**
    #[strum(serialize = "TURUJUSSUANNGUR")]
    TURUJUSSUANNGUR,
    /// kal: **@CODE@**
    #[strum(serialize = "TURUJUSSUU")]
    TURUJUSSUU,
    /// kal: **@CODE@**
    #[strum(serialize = "TURUJUU")]
    TURUJUU,
    /// kal: **@CODE@**
    #[strum(serialize = "TUSSAA")]
    TUSSAA,
    /// kal: **@CODE@**
    #[strum(serialize = "TUSSANNGUR")]
    TUSSANNGUR,
    /// kal: **@CODE@**
    #[strum(serialize = "TUU")]
    TUU,
    /// kal: **@CODE@**
    #[strum(serialize = "TUUQ")]
    TUUQ,
    /// kal: **@CODE@**
    #[strum(serialize = "TUUR")]
    TUUR,
    /// kal: **@CODE@**
    #[strum(serialize = "TUUSAAR")]
    TUUSAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "TUUSI")]
    TUUSI,
    /// fit: **@CODE@** transitive
    /// sje: (no comment)
    /// kpv: **@CODE@**
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Transitive verb
    /// izh: **@CODE@**  transitivity:
    /// liv: **@CODE@** = Transitive verb
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// evn: (no comment)
    /// xal: (no comment)
    /// sma: **@CODE@**:  Transitive verb
    /// lut: (no comment)
    /// sjt: (no comment)
    /// got: (no comment)
    /// hdn: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** for transitive, intransitive
    /// sms: **@CODE@** Transitive
    /// bgs: (no comment)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =  to transitivity:
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@** = transitive and
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: **@CODE@**
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: **@CODE@**
    /// sme: **@CODE@**    Transitive Verb, +V+TV
    /// gur: (no comment)
    /// aym: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** transitivity tags
    /// ceb: (no comment)
    /// khk: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@** Transitive (i.e. with Erg + Abs)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "TV")]
    TV,
    /// vep: **@CODE@**
    /// tat: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** This alerts workers of work to be done
    /// mdf: (no comment)
    #[strum(serialize = "TYÄ")]
    TYÄ,
    /// gle: **@CODE@** = Temporal e.g. inniu, amárach etc.
    /// kpv: **@CODE@** temporal
    /// vot: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  =
    /// vep: (no comment)
    /// myv: (no comment)
    /// myv: (no comment)
    /// olo: (no comment)
    /// koi: (no comment)
    /// xak: (no comment)
    /// smn: **@CODE@** Temporal adverbs
    /// mdf: **@CODE@**:  temporal
    /// vro: **@CODE@**
    #[strum(serialize = "Temp")]
    Temp,
    /// fin: (no comment)
    #[strum(serialize = "Tempr")]
    Tempr,
    /// kpv: **@CODE@** Terminative -ӧдз матыстчан
    /// vot: (no comment)
    /// evn: (no comment)
    /// udm: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// olo: **@CODE@** Terminative
    /// koi: (no comment)
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
    /// koi: (no comment)
    #[strum(serialize = "TerI")]
    TerI,
    /// koi: (no comment)
    #[strum(serialize = "TerII")]
    TerII,
    /// bxr: (no comment)
    /// koi: (no comment)
    /// xak: (no comment)
    #[strum(serialize = "Term")]
    Term,
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
    /// epo: (no comment)
    #[strum(serialize = "Thing")]
    Thing,
    /// otw: (no comment)
    #[strum(serialize = "Thm1")]
    Thm1,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "Thm1Pl")]
    Thm1Pl,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "Thm1Sg")]
    Thm1Sg,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "Thm2")]
    Thm2,
    /// otw: (no comment)
    #[strum(serialize = "Thm2a")]
    Thm2a,
    /// otw: (no comment)
    #[strum(serialize = "Thm2b")]
    Thm2b,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "ThmDir")]
    ThmDir,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "ThmInv")]
    ThmInv,
    /// oji: (no comment)
    #[strum(serialize = "ThmNeg")]
    ThmNeg,
    /// otw: (no comment)
    #[strum(serialize = "ThmNul")]
    ThmNul,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "ThmPas")]
    ThmPas,
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "ThmPos")]
    ThmPos,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// epo: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Time")]
    Time,
    /// cho: (no comment)
    #[strum(serialize = "Tns")]
    Tns,
    /// lut: (no comment)
    /// slh: (no comment)
    /// mya: (no comment)
    /// khk: (no comment)
    #[strum(serialize = "Top")]
    Top,
    /// kpv: **@CODE@**
    /// lut: (no comment)
    /// slh: (no comment)
    /// koi: **@CODE@**:  universal кыкнан; кыкнанным
    /// koi: (no comment)
    #[strum(serialize = "Tot")]
    Tot,
    /// fit: **@CODE@** = Translaive
    /// kpv: **@CODE@** translative -ті вуджан
    /// vot: (no comment)
    /// fkv: **@CODE@** = Translaive
    /// liv: **@CODE@** = translative
    /// udm: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@** Ыг
    /// vep: **@CODE@**  = translative
    /// myv: (no comment)
    /// olo: **@CODE@** translative
    /// koi: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  translative
    /// vro: **@CODE@** translative
    #[strum(serialize = "Tra")]
    Tra,
    /// mns: **@CODE@** hmm, Trans and TV?
    #[strum(serialize = "Trans")]
    Trans,
    /// mpj: **@CODE@**
    #[strum(serialize = "Trel")]
    Trel,
    /// kal: **@CODE@** = Terminalis
    #[strum(serialize = "Trm")]
    Trm,
    /// fin: (no comment)
    #[strum(serialize = "TruncPrefix")]
    TruncPrefix,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// sms: **@CODE@**
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Txt")]
    Txt,
    /// pad: (no comment)
    #[strum(serialize = "TypeKa")]
    TypeKa,
    /// pad: (no comment)
    #[strum(serialize = "TypeZero")]
    TypeZero,
    /// gle: **@CODE@** = Typos, e.g. ta/ata  instead of tá/atá
    #[strum(serialize = "Typo")]
    Typo,
    /// kal: **@CODE@**
    #[strum(serialize = "U")]
    U,
    /// kal: **@CODE@**
    #[strum(serialize = "UAAR")]
    UAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "UKU")]
    UKU,
    /// kal: **@CODE@**
    #[strum(serialize = "UKUA")]
    UKUA,
    /// kal: **@CODE@**
    #[strum(serialize = "UMI")]
    UMI,
    /// kal: **@CODE@**
    #[strum(serialize = "UMMI")]
    UMMI,
    /// kal: **@CODE@**
    #[strum(serialize = "UMMIR")]
    UMMIR,
    /// kal: **@CODE@**
    #[strum(serialize = "UNA")]
    UNA,
    /// kal: **@CODE@**
    #[strum(serialize = "UR")]
    UR,
    /// kal: **@CODE@**
    /// kpv: (no comment)
    /// lut: (no comment)
    /// slh: (no comment)
    /// myv: (no comment)
    /// koi: (no comment)
    /// smn: **@CODE@**
    /// mdf: (no comment)
    #[strum(serialize = "URL")]
    URL,
    /// kal: **@CODE@**
    #[strum(serialize = "USAAQ")]
    USAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "USAAR")]
    USAAR,
    /// kal: **@CODE@**
    #[strum(serialize = "USAP")]
    USAP,
    /// kal: **@CODE@**
    #[strum(serialize = "USAQ")]
    USAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "USAR")]
    USAR,
    /// kal: **@CODE@**
    #[strum(serialize = "USIAQ")]
    USIAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "USIQ")]
    USIQ,
    /// kal: **@CODE@**
    #[strum(serialize = "UTE")]
    UTE,
    /// kal: **@CODE@**
    #[strum(serialize = "UTIGE")]
    UTIGE,
    /// sma: **@CODE@**:  A tag to indicate realised or potential Umlaut
    #[strum(serialize = "Uml")]
    Uml,
    /// izh: **@CODE@**  universal quantifier
    /// epo: (no comment)
    #[strum(serialize = "Univ")]
    Univ,
    /// mpj: **@CODE@**  = Unrealised
    #[strum(serialize = "Unr")]
    Unr,
    /// bla: (no comment)
    #[strum(serialize = "Unspec")]
    Unspec,
    /// gle: (no comment)
    #[strum(serialize = "Use")]
    Use,
    /// fit: **@CODE@** = ?
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// fkv: **@CODE@** never retained in the HFST Grammar Checker disambiguation analyser
    /// sma: **@CODE@** | **never** retained in the HFST Grammar Checker disambiguation analyser
    /// nob: **@CODE@** never retained in the HFST Grammar Checker disambiguation analyser
    /// fao: **@CODE@** — **never** retained in the HFST Grammar Checker disambiguation analyser
    /// smj: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// sms: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// rmn: (no comment)
    /// vep: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// myv: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// olo: **@CODE@**  (?)
    /// koi: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// sme: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// smn: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    /// mdf: **@CODE@** – **never** retained in the HFST Grammar Checker disambiguation analyser
    #[strum(serialize = "Use/-GC")]
    Use_SLASH__MINUS_GC,
    /// gle: **@CODE@** =
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// mhr: **@CODE@** Excluded in PLX-speller
    /// sma: **@CODE@** | Excluded in PLX speller
    /// nob: **@CODE@**
    /// smj: **@CODE@** – Excluded from PLX speller
    /// rmn: (no comment)
    /// vep: **@CODE@** Excluded in PLX-speller
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// deu: (no comment)
    /// sme: **@CODE@** Excluded in PLX-speller
    /// smn: **@CODE@** - do not include in Polderland spellers (most likely irrelevant for smn)
    /// bak: (no comment)
    #[strum(serialize = "Use/-PLX")]
    Use_SLASH__MINUS_PLX,
    /// fit: (no comment)
    /// sje: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// swe: (no comment)
    /// fkv: **@CODE@** = for tokenisation
    /// mhr: **@CODE@** Do not include in fsts made for hfst-pmatch
    /// sma: **@CODE@** | Do not include in fst's made for hfst-pmatch
    /// nob: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** – Do not include in fst's made for hfst-pmatch
    /// mns: **@CODE@** = for preprocessing
    /// sms: **@CODE@** -
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// vep: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** Do not include in fst's made for hfst-pmatch
    /// smn: **@CODE@** = for preprocessing
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Use/-PMatch")]
    Use_SLASH__MINUS_PMatch,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Use/-Ped")]
    Use_SLASH__MINUS_Ped,
    /// fit: **@CODE@** = Excluded in speller
    /// sje: (no comment)
    /// gle: **@CODE@** =
    /// kal: **@CODE@** = Do not include in speller
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// fkv: **@CODE@** = Excluded in speller
    /// izh: **@CODE@**  do not suggest
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** = accepted in normative FST but not in speller
    /// krl: **@CODE@**
    /// xal: (no comment)
    /// sma: **@CODE@** | Excluded from speller
    /// lut: (no comment)
    /// sjt: (no comment)
    /// nob: **@CODE@**
    /// got: (no comment)
    /// hdn: (no comment)
    /// fao: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// smj: **@CODE@** – Excluded from speller
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// chr: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@** do not use in speller
    /// sms: **@CODE@** Excluded in speller
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@**
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@** Orthographically correct, typically perifer words, excluded in speller because they cause trouble for frequent words
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@** Excluded in speller
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// deu: **@CODE@** Excluded in speller
    /// sme: **@CODE@** Orthographically correct, typically perifer words, excluded in speller because they cause trouble for frequent words
    /// smn: **@CODE@** - do not include in speller (even though the entry is formally correct)
    /// gur: (no comment)
    /// aym: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@** tagging other forms not to be accepted by the speller
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: **@CODE@**
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Use/-Spell")]
    Use_SLASH__MINUS_Spell,
    /// fit: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sje: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// gle: **@CODE@** =
    /// pur: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fkv: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// pma: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mhr: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// krl: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sma: **@CODE@** | **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// lut: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// nob: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fao: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// txi: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// smj: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// tqb: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// slh: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// aan: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// waw: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mpu: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// wyr: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// cor: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mns: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sms: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// bgs: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// rmn: (no comment)
    /// myu: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vep: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// rmg: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// myv: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// tkl: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// olo: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// udu: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// wac: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// pad: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sme: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// smn: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// non: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fin: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mdf: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// gvp: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vro: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// ora: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// bwi: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// bak: **@CODE@** – **never** retained in the HFST Text-To-Speech disambiguation tokeniser
    #[strum(serialize = "Use/-TTS")]
    Use_SLASH__MINUS_TTS,
    /// gle: (no comment)
    /// nob: **@CODE@**
    /// rmn: (no comment)
    #[strum(serialize = "Use/-TTS-")]
    Use_SLASH__MINUS_TTS_MINUS_,
    /// gle: (no comment)
    /// nob: **@CODE@**
    /// rmn: (no comment)
    #[strum(serialize = "Use/-TTS–")]
    Use_SLASH__MINUS_TTS_EMDASH_,
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Use/Ant")]
    Use_SLASH_Ant,
    /// fin: (no comment)
    #[strum(serialize = "Use/Arch")]
    Use_SLASH_Arch,
    /// fit: (no comment)
    /// sje: (no comment)
    /// gle: **@CODE@** =
    /// kal: **@CODE@** = pga. ~/langtech/shared-mul/src/fst/stems/telephone.lexc
    /// swe: (no comment)
    /// fkv: **@CODE@** for numerals, copied from sme
    /// mhr: **@CODE@** circular paths (old ^C^)
    /// krl: **@CODE@** – avoid infinite stuff in generative apps
    /// sma: **@CODE@** | Circular path
    /// nob: **@CODE@** = circular string
    /// fao: (no comment)
    /// smj: **@CODE@** – Circular path
    /// mns: **@CODE@** =
    /// sms: **@CODE@** circular paths (old ^C^)
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**
    /// vep: **@CODE@** circular paths (old ^C^)
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** circular paths (old ^C^)
    /// deu: (no comment)
    /// sme: **@CODE@** circular paths (old ^C^)
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// fin: (no comment)
    /// mdf: (no comment)
    /// bak: (no comment)
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
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** circular paths for the numerals (old ^N^)
    /// sme: **@CODE@** circular paths for the numerals (old ^N^)
    /// mdf: (no comment)
    #[strum(serialize = "Use/CircN")]
    Use_SLASH_CircN,
    /// mdf: (no comment)
    #[strum(serialize = "Use/Colloq")]
    Use_SLASH_Colloq,
    /// sma: **@CODE@** | For words without formal normalization. Divvun suggest that this should be normative. Included in speller. Based on 2010 normative decision & Ove Lorentz' suggestions for the norm.
    /// smj: **@CODE@** – For words without formal normalization.  Divvun suggest that this should be normative. Included in speller.
    /// sme: (no comment)
    #[strum(serialize = "Use/DNorm")]
    Use_SLASH_DNorm,
    /// smj: **@CODE@** – Derrogatory word. Recognized, but not suggested in speller, same as SpellNoSugg
    #[strum(serialize = "Use/Derrog")]
    Use_SLASH_Derrog,
    /// mhr: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// fao: (no comment)
    #[strum(serialize = "Use/Disamb")]
    Use_SLASH_Disamb,
    /// sjd: (no comment)
    #[strum(serialize = "Use/Elid")]
    Use_SLASH_Elid,
    /// fit: (no comment)
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// swe: (no comment)
    /// fkv: **@CODE@** only retained in the HFST Grammar Checker disambiguation analyser
    /// mhr: **@CODE@** only retained in the HFST Grammar Checker disambiguation analyser
    /// sma: **@CODE@** | **only** retained in the HFST Grammar Checker disambiguation analyser
    /// nob: **@CODE@** only retained in the HFST Grammar Checker disambiguation analyser
    /// fao: **@CODE@** — **only** retained in the HFST Grammar Checker disambiguation analyser
    /// smj: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// mns: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// sms: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// rmn: (no comment)
    /// vep: **@CODE@** ??? typo?, occurs once.
    /// vep: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// rmg: **@CODE@**
    /// myv: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// koi: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// deu: (no comment)
    /// sme: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// sme: (no comment)
    /// smn: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// fin: (no comment)
    /// mdf: **@CODE@** – **only** retained in the HFST Grammar Checker disambiguation analyser
    /// bak: (no comment)
    #[strum(serialize = "Use/GC")]
    Use_SLASH_GC,
    /// kal: **@CODE@** = For generating split hybrids. Should not be included in speller, analyser, or norm-generator
    #[strum(serialize = "Use/Hybrid")]
    Use_SLASH_Hybrid,
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
    /// smn: **@CODE@**  generate only for MT
    #[strum(serialize = "Use/MT")]
    Use_SLASH_MT,
    /// mhr: **@CODE@** marginal
    /// sma: **@CODE@** | Marginal, korrekte, eksisterende former, men som er sjeldne. vi kan fjerne disse ordene f.eks fra speller, fordi de er så sjeldne og lite i bruk at de lemma som ligger nært kan bli forvekslet.
    /// smj: **@CODE@** – Marginal, but normative lemmas. Not in speller.
    /// sms: **@CODE@** marginal (?)
    /// som: (no comment)
    /// myv: (no comment)
    /// olo: **@CODE@** marginal (?)
    /// deu: **@CODE@** marginal (?)
    /// fin: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "Use/Marg")]
    Use_SLASH_Marg,
    /// fin: (no comment)
    #[strum(serialize = "Use/N")]
    Use_SLASH_N,
    /// fit: (no comment)
    /// sje: (no comment)
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** This is used for minimizing excess generation in apertium kpv-koi
    /// swe: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** only accept, not generate (for MT and Oahpa use)
    /// liv: **@CODE@** no generation
    /// crk: (no comment)
    /// mhr: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// krl: **@CODE@** do not generate jaska 20250826
    /// rue: (no comment)
    /// sma: **@CODE@** | Do not generate <br/> for isme-ped.fst and apertium
    /// nob: **@CODE@**
    /// nob: **@CODE@** not-generate, for ped generation isme-ped.fst and MT
    /// fao: (no comment)
    /// smj: **@CODE@** – Do not generate, only for Oahpa and MT. In speller.
    /// crj: (no comment)
    /// rus: (no comment)
    /// mns: **@CODE@** do not generate, but accept
    /// sms: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// bxr: (no comment)
    /// rmn: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@** do not generate
    /// vep: **@CODE@** not-generate, for ped generation isme-ped.fst and MT
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// som: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// deu: (no comment)
    /// sme: **@CODE@** not-generate, for ped generation isme-ped.fst and MT
    /// smn: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// fin: (no comment)
    /// mdf: (no comment)
    /// rmf: **@CODE@** For correct parallel forms, this one shall not be preferred when only one is needed
    /// vro: **@CODE@** No generation
    /// vro: (no comment)
    /// vro: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "Use/NG")]
    Use_SLASH_NG,
    /// fao: (no comment)
    #[strum(serialize = "Use/NGA")]
    Use_SLASH_NGA,
    /// sje: (no comment)
    /// mhr: **@CODE@** Not for miniparadigm in VD dicts
    /// smj: **@CODE@** – Not for miniparadigm in VD dicts
    /// sms: **@CODE@** not-generate, for ped generation isme-ped.fst
    /// vep: **@CODE@** Not for miniparadigm in NDS dicts
    /// sme: **@CODE@** Not for miniparadigm in NDS dicts
    /// smn: **@CODE@** Not for miniparadigm in NDS dicts
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
    /// rue: (no comment)
    /// rus: (no comment)
    #[strum(serialize = "Use/Obs")]
    Use_SLASH_Obs,
    /// sjd: (no comment)
    #[strum(serialize = "Use/Orth")]
    Use_SLASH_Orth,
    /// fit: (no comment)
    /// sje: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// gle: **@CODE@** =
    /// kpv: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// swe: (no comment)
    /// fkv: **@CODE@** = for tokenisation
    /// sma: **@CODE@** | Do *only* include in fst's for hfst-pmatch
    /// nob: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// nob: **@CODE@**
    /// fao: **@CODE@** means that the following is only used in the analyser feeding the disambiguator. This is missing.
    /// smj: **@CODE@** – Only use in fst's targeted for `hfst-pmatch`
    /// cor: (no comment)
    /// mns: **@CODE@** = for preprocessing
    /// sms: **@CODE@** -
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// vep: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// rmg: **@CODE@**
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** means that the following is only used in the analyser feeding the disambiguator
    /// smn: **@CODE@** = for preprocessing
    /// fin: **@CODE@** means that the following is only used in the analyser feeding the disambiguator. This is missing.
    /// mdf: (no comment)
    /// vro: **@CODE@**
    /// bak: (no comment)
    #[strum(serialize = "Use/PMatch")]
    Use_SLASH_PMatch,
    /// sma: **@CODE@** | Remove from pedagogical speller
    /// smj: **@CODE@** – Remove from pedagogical speller
    /// sms: **@CODE@** for use with pedagogical work
    /// som: (no comment)
    #[strum(serialize = "Use/Ped")]
    Use_SLASH_Ped,
    /// fin: (no comment)
    #[strum(serialize = "Use/Rare")]
    Use_SLASH_Rare,
    /// fit: **@CODE@** = recognized but not suggested in speller
    /// gle: **@CODE@** =
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// vot: (no comment)
    /// fkv: **@CODE@** = recognized but not suggested in speller
    /// mhr: **@CODE@** recognized but not suggested in speller
    /// krl: **@CODE@** recognized but not suggested in speller
    /// sma: **@CODE@** | Recognized but not suggested in speller
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// smj: **@CODE@** – Recognized, but not suggested in speller
    /// smj: (no comment)
    /// yrk: **@CODE@** recognized but not suggested in speller
    /// mns: **@CODE@** recognized but not suggested in speller
    /// sms: **@CODE@** recognized but not suggested in speller
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// vep: **@CODE@** recognized but not suggested in speller
    /// rmg: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// tkl: (no comment)
    /// olo: **@CODE@** recognized but not suggested in speller
    /// koi: (no comment)
    /// deu: **@CODE@** recognized but not suggested in speller
    /// sme: **@CODE@** recognized but not suggested in speller
    /// sme: (no comment)
    /// smn: **@CODE@** - Recognized, but not suggested in speller
    /// fin: (no comment)
    /// mdf: (no comment)
    /// vro: **@CODE@** not suggested in speller
    /// bak: (no comment)
    #[strum(serialize = "Use/SpellNoSugg")]
    Use_SLASH_SpellNoSugg,
    /// fit: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sje: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// gle: **@CODE@** =
    /// pur: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fkv: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// pma: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mhr: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// krl: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sma: **@CODE@** | **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// lut: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// nob: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fao: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// txi: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// smj: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// tqb: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// slh: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// aan: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// waw: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mpu: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// wyr: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// cor: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// mns: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sms: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// bgs: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// rmn: (no comment)
    /// myu: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vep: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vep: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// rmg: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// myv: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// tkl: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// olo: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// udu: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// wac: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// pad: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// sme: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// smn: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// non: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fin: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// fin: (no comment)
    /// mdf: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// gvp: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// vro: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// ora: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// bwi: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    /// bak: **@CODE@** – **only** retained in the HFST Text-To-Speech disambiguation tokeniser
    #[strum(serialize = "Use/TTS")]
    Use_SLASH_TTS,
    /// mhr: **@CODE@** =  Dealing with lative form 2012-10-27 аваеш, пашаш
    #[strum(serialize = "Use/Test")]
    Use_SLASH_Test,
    /// fin: (no comment)
    #[strum(serialize = "Use/sub")]
    Use_SLASH_sub,
    /// swe: (no comment)
    /// rmu: **@CODE@** = i bruk
    #[strum(serialize = "Utr")]
    Utr,
    /// fit: **@CODE@** = Verb
    /// sje: (no comment)
    /// gle: **@CODE@** = n/a (Verb is used)
    /// kal: **@CODE@** = Verb
    /// kpv: **@CODE@**:  verb      кадакыв   глагол
    /// chp: (no comment)
    /// vot: (no comment)
    /// cwd: (no comment)
    /// fkv: **@CODE@** = Verb
    /// liv: **@CODE@** = verb
    /// crk: (no comment)
    /// mhr: **@CODE@** = verbs
    /// evn: (no comment)
    /// rue: (no comment)
    /// sma: **@CODE@** = Verb
    /// tgl: **@CODE@** -  Verb
    /// hdn: (no comment)
    /// smj: **@CODE@** = Verb
    /// udm: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    /// epo: (no comment)
    /// apu: **@CODE@** Verb
    /// chr: (no comment)
    /// crj: (no comment)
    /// rus: (no comment)
    /// yrk: **@CODE@**
    /// mns: **@CODE@**
    /// sms: **@CODE@** = Verb
    /// bxr: (no comment)
    /// tku: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@**
    /// vep: **@CODE@**  = verb
    /// som: (no comment)
    /// myv: (no comment)
    /// nso: (no comment)
    /// mya: (no comment)
    /// olo: **@CODE@**  verb
    /// koi: **@CODE@**:  verb
    /// xak: (no comment)
    /// eus: (no comment)
    /// deu: (no comment)
    /// sme: **@CODE@** - Verb
    /// non: (no comment)
    /// kio: (no comment)
    /// fin: (no comment)
    /// mdf: **@CODE@**:  verb
    /// ceb: **@CODE@** -  Verb
    /// khk: (no comment)
    /// fro: (no comment)
    /// tlh: (no comment)
    /// vro: **@CODE@** Verb
    /// mpj: **@CODE@**
    #[strum(serialize = "V")]
    V,
    /// som: (no comment)
    #[strum(serialize = "V/")]
    V_SLASH_,
    /// som: (no comment)
    #[strum(serialize = "V/ah")]
    V_SLASH_ah,
    /// bla: (no comment)
    /// oji: (no comment)
    /// otw: (no comment)
    #[strum(serialize = "VAI")]
    VAI,
    /// otw: (no comment)
    #[strum(serialize = "VAIO")]
    VAIO,
    /// kal: **@CODE@**
    #[strum(serialize = "VALLAAQ")]
    VALLAAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "VALLAAR")]
    VALLAAR,
    /// sje: (no comment)
    /// kpv: **@CODE@** тӧм Participle verbal adjective, see also Der/Abe
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// sjd: (no comment)
    /// sma: **@CODE@** | Verb Abessive
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// koi: (no comment)
    /// sme: **@CODE@** VerbAbbesive
    /// vro: **@CODE@**
    #[strum(serialize = "VAbess")]
    VAbess,
    /// myv: (no comment)
    /// mdf: (no comment)
    #[strum(serialize = "VAbl")]
    VAbl,
    /// kpv: **@CODE@** тӧг Gerund
    /// koi: (no comment)
    #[strum(serialize = "VCar")]
    VCar,
    /// gle: **@CODE@** = ditransitive verb
    #[strum(serialize = "VD")]
    VD,
    /// gle: **@CODE@** = - form used before a word starting with a vowel or f+vowel
    #[strum(serialize = "VF")]
    VF,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// sma: **@CODE@** | Verbgenitive
    /// smj: (no comment)
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// sms: **@CODE@**
    /// lit: (no comment)
    /// vep: **@CODE@**
    /// som: (no comment)
    /// myv: (no comment)
    /// sme: **@CODE@** VerbGenitive
    /// smn: **@CODE@**
    /// mdf: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "VGen")]
    VGen,
    /// gle: **@CODE@** = intransitive verb
    #[strum(serialize = "VI")]
    VI,
    /// bla: (no comment)
    #[strum(serialize = "VII")]
    VII,
    /// kal: **@CODE@**
    /// ess: (no comment)
    /// ipk: (no comment)
    #[strum(serialize = "VIK")]
    VIK,
    /// kal: **@CODE@**
    #[strum(serialize = "VIP")]
    VIP,
    /// kal: **@CODE@**
    #[strum(serialize = "VISSUAQ")]
    VISSUAQ,
    /// kal: **@CODE@**
    #[strum(serialize = "VISSUR")]
    VISSUR,
    /// cor: (no comment)
    #[strum(serialize = "VN")]
    VN,
    /// hdn: (no comment)
    #[strum(serialize = "VNear")]
    VNear,
    /// gle: **@CODE@** = transitive verb
    #[strum(serialize = "VT")]
    VT,
    /// bla: (no comment)
    #[strum(serialize = "VTA")]
    VTA,
    /// gle: **@CODE@** = transitive & intransitive verb
    /// bla: (no comment)
    #[strum(serialize = "VTI")]
    VTI,
    /// kpv: **@CODE@** тӧдз Gerund
    /// koi: (no comment)
    #[strum(serialize = "VTer")]
    VTer,
    /// kal: **@CODE@**
    #[strum(serialize = "VVAARIK")]
    VVAARIK,
    /// kal: **@CODE@**
    #[strum(serialize = "VVAARIP")]
    VVAARIP,
    /// hdn: (no comment)
    #[strum(serialize = "Val/0")]
    Val_SLASH_0,
    /// hdn: (no comment)
    #[strum(serialize = "Val/0X")]
    Val_SLASH_0X,
    /// hdn: (no comment)
    #[strum(serialize = "Val/A")]
    Val_SLASH_A,
    /// hdn: (no comment)
    #[strum(serialize = "Val/AC")]
    Val_SLASH_AC,
    /// hdn: (no comment)
    #[strum(serialize = "Val/ACO")]
    Val_SLASH_ACO,
    /// hdn: (no comment)
    #[strum(serialize = "Val/ACOX")]
    Val_SLASH_ACOX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/ACR")]
    Val_SLASH_ACR,
    /// hdn: (no comment)
    #[strum(serialize = "Val/ACRX")]
    Val_SLASH_ACRX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/ACX")]
    Val_SLASH_ACX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/AO")]
    Val_SLASH_AO,
    /// hdn: (no comment)
    #[strum(serialize = "Val/AOR")]
    Val_SLASH_AOR,
    /// hdn: (no comment)
    #[strum(serialize = "Val/AORX")]
    Val_SLASH_AORX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/AOX")]
    Val_SLASH_AOX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/AX")]
    Val_SLASH_AX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/I")]
    Val_SLASH_I,
    /// hdn: (no comment)
    #[strum(serialize = "Val/IC")]
    Val_SLASH_IC,
    /// hdn: (no comment)
    #[strum(serialize = "Val/ICX")]
    Val_SLASH_ICX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/IO")]
    Val_SLASH_IO,
    /// hdn: (no comment)
    #[strum(serialize = "Val/IOX")]
    Val_SLASH_IOX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/IX")]
    Val_SLASH_IX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/P")]
    Val_SLASH_P,
    /// hdn: (no comment)
    #[strum(serialize = "Val/PC")]
    Val_SLASH_PC,
    /// hdn: (no comment)
    #[strum(serialize = "Val/PCX")]
    Val_SLASH_PCX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/PO")]
    Val_SLASH_PO,
    /// hdn: (no comment)
    #[strum(serialize = "Val/POX")]
    Val_SLASH_POX,
    /// hdn: (no comment)
    #[strum(serialize = "Val/PX")]
    Val_SLASH_PX,
    /// gle: **@CODE@** = variant spelling e.g. rabh instead of raibh or dheachaidh
    /// cwd: (no comment)
    #[strum(serialize = "Var")]
    Var,
    /// cwd: (no comment)
    #[strum(serialize = "Var/East")]
    Var_SLASH_East,
    /// cwd: (no comment)
    #[strum(serialize = "Var/West")]
    Var_SLASH_West,
    /// gle: **@CODE@** = Verbal particle (Q)
    #[strum(serialize = "Vb")]
    Vb,
    /// bla: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    #[strum(serialize = "Vblz")]
    Vblz,
    /// gle: **@CODE@** = Vocative particle
    #[strum(serialize = "Vc")]
    Vc,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Veh")]
    Veh,
    /// gle: **@CODE@** = Verb
    /// kmr: (no comment)
    #[strum(serialize = "Verb")]
    Verb,
    /// gle: **@CODE@** = Verbal noun
    #[strum(serialize = "Verbal")]
    Verbal,
    /// kal: **@CODE@** = Vialis
    #[strum(serialize = "Via")]
    Via,
    /// apu: (no comment)
    #[strum(serialize = "Vido")]
    Vido,
    /// apu: (no comment)
    #[strum(serialize = "Vids")]
    Vids,
    /// lut: (no comment)
    /// slh: (no comment)
    #[strum(serialize = "Vnoun")]
    Vnoun,
    /// gle: **@CODE@** = Vocative case
    /// kpv: **@CODE@** Vocative ??
    /// cwd: (no comment)
    /// liv: **@CODE@** = Vocative
    /// crk: (no comment)
    /// mhr: **@CODE@** = vocative
    /// oji: (no comment)
    /// qya: (no comment)
    /// otw: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// mrj: **@CODE@** =
    /// vep: **@CODE@**  = Vocative
    /// myv: (no comment)
    /// koi: (no comment)
    /// mdf: **@CODE@**:  Vocative
    /// mpj: **@CODE@** = voc
    #[strum(serialize = "Voc")]
    Voc,
    /// bxr: (no comment)
    #[strum(serialize = "Vol")]
    Vol,
    /// gle: **@CODE@** = Vowel-initial : used to allow past-tense Len e.g. d´ith
    #[strum(serialize = "Vow")]
    Vow,
    /// sje: (no comment)
    #[strum(serialize = "Vsubst")]
    Vsubst,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**  =
    /// som: (no comment)
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "V→A")]
    V_A,
    /// pur: (no comment)
    /// vot: (no comment)
    /// lav: (no comment)
    /// izh: **@CODE@**
    /// liv: **@CODE@**
    /// pma: (no comment)
    /// inp: (no comment)
    /// mhr: **@CODE@** =
    /// krl: **@CODE@**
    /// xal: (no comment)
    /// lut: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// udm: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// nio: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@**
    /// xwo: (no comment)
    /// mns: **@CODE@**
    /// bgs: (no comment)
    /// rmn: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// mrj: **@CODE@** =
    /// ndl: (no comment)
    /// myu: (no comment)
    /// vep: **@CODE@**
    /// rup: (no comment)
    /// tat: (no comment)
    /// tyv: (no comment)
    /// rmg: **@CODE@**
    /// sel: (no comment)
    /// som: (no comment)
    /// myv: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// tkl: (no comment)
    /// olo: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// koi: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// mdf: (no comment)
    /// ale: (no comment)
    /// rmf: **@CODE@**
    /// ceb: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// tlh: (no comment)
    /// sto: (no comment)
    /// vro: **@CODE@**
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// mpj: (no comment)
    /// bwi: (no comment)
    /// kca: (no comment)
    /// bak: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "V→N")]
    V_N,
    /// vot: (no comment)
    /// liv: **@CODE@**
    /// mhr: **@CODE@** =
    /// udm: (no comment)
    /// yrk: **@CODE@**
    /// vep: **@CODE@**
    /// som: (no comment)
    /// olo: (no comment)
    /// vro: **@CODE@**
    #[strum(serialize = "V→V")]
    V_V,
    /// crj: (no comment)
    #[strum(serialize = "WAK")]
    WAK,
    /// kpv: **@CODE@** to mark intermediate solutions
    /// lav: **@CODE@** Underdeveloped
    /// mhr: **@CODE@** = nouns
    /// udm: (no comment)
    /// hun: (no comment)
    /// yrk: **@CODE@** WORK HAS TO BE DONE Do not remove, replaces +TYÄ
    /// mrj: **@CODE@** = Error?
    /// myv: (no comment)
    /// koi: (no comment)
    /// vro: **@CODE@** (eng) work needed
    #[strum(serialize = "WORK")]
    WORK,
    /// mpj: **@CODE@** = warning
    #[strum(serialize = "Warn")]
    Warn,
    /// gle: **@CODE@** = Weak plural (noun)
    /// mhr: **@CODE@** = weak (?) form
    #[strum(serialize = "Weak")]
    Weak,
    /// gle: **@CODE@** = wh word?
    #[strum(serialize = "Wh")]
    Wh,
    /// pur: (no comment)
    /// pma: (no comment)
    /// inp: (no comment)
    /// xal: (no comment)
    /// sjt: (no comment)
    /// tgl: (no comment)
    /// got: (no comment)
    /// txi: (no comment)
    /// nds: (no comment)
    /// tqb: (no comment)
    /// qya: (no comment)
    /// rmy: (no comment)
    /// slh: (no comment)
    /// aan: (no comment)
    /// luo: (no comment)
    /// skf: (no comment)
    /// dag: (no comment)
    /// apu: (no comment)
    /// waw: (no comment)
    /// tir: (no comment)
    /// mpu: (no comment)
    /// wyr: (no comment)
    /// hun: (no comment)
    /// xwo: (no comment)
    /// bgs: (no comment)
    /// lit: (no comment)
    /// ckt: (no comment)
    /// zul: (no comment)
    /// ndl: (no comment)
    /// myu: (no comment)
    /// tyv: (no comment)
    /// sel: (no comment)
    /// zxx: (no comment)
    /// rmu: (no comment)
    /// bul: (no comment)
    /// eng: (no comment)
    /// sqi: (no comment)
    /// dsb: (no comment)
    /// udu: (no comment)
    /// wac: (no comment)
    /// aka: (no comment)
    /// pad: (no comment)
    /// gur: (no comment)
    /// aym: (no comment)
    /// ale: (no comment)
    /// gvp: (no comment)
    /// dgr: (no comment)
    /// sto: (no comment)
    /// kjh: (no comment)
    /// tha: (no comment)
    /// tau: (no comment)
    /// ora: (no comment)
    /// bwi: (no comment)
    /// tqn: (no comment)
    #[strum(serialize = "Wthr")]
    Wthr,
    /// swe: (no comment)
    /// cwd: (no comment)
    /// crk: (no comment)
    /// nob: **@CODE@**  denoting not-checked.
    /// kio: (no comment)
    #[strum(serialize = "X")]
    X,
    /// gle: **@CODE@** =  XML tags in the text, e.g. <p>, <title> etc.
    #[strum(serialize = "XMLTag")]
    XMLTag,
    /// ciw: (no comment)
    #[strum(serialize = "XObj")]
    XObj,
    /// eus: (no comment)
    #[strum(serialize = "XXX")]
    XXX,
    /// gle: **@CODE@** =  Indecipherable speech (in transcribed speech)
    #[strum(serialize = "Xxx")]
    Xxx,
    /// kpv: **@CODE@**:  Zero collective кодныс
    /// koi: (no comment)
    #[strum(serialize = "ZeroColl")]
    ZeroColl,
    /// myv: (no comment)
    #[strum(serialize = "bahuvrihi")]
    bahuvrihi,
    /// cho: (no comment)
    /// deu: (no comment)
    #[strum(serialize = "cmp")]
    cmp,
    /// kal: **@CODE@** = clitic to separate clitics from suffixes with identical upper form in disambiguator.cg3 (TTAAQ)
    #[strum(serialize = "encl")]
    encl,
    /// gle: **@CODE@** = h prefixed to a vowel-initial word
    #[strum(serialize = "hPref")]
    hPref,
    /// eus: (no comment)
    #[strum(serialize = "ifl")]
    ifl,
    /// eus: (no comment)
    #[strum(serialize = "ifl/f")]
    ifl_SLASH_f,
    /// eus: (no comment)
    #[strum(serialize = "ifl/m")]
    ifl_SLASH_m,
    /// mdf: (no comment)
    #[strum(serialize = "mwe")]
    mwe,
    /// khk: (no comment)
    #[strum(serialize = "p1")]
    p1,
    /// khk: (no comment)
    #[strum(serialize = "p2")]
    p2,
    /// khk: (no comment)
    #[strum(serialize = "p3")]
    p3,
    /// cho: (no comment)
    /// deu: (no comment)
    #[strum(serialize = "pos")]
    pos,
    /// fin: (no comment)
    #[strum(serialize = "s")]
    s,
    /// cho: (no comment)
    /// deu: (no comment)
    #[strum(serialize = "sup")]
    sup,
    /// apu: **@CODE@** used to tag neutralized i/y and ĩ/ỹ /usado para mostrar neutralidade de i/y e ĩ/ỹ
    #[strum(serialize = "v/iy")]
    v_SLASH_iy,
    /// fit: **@CODE@**
    /// gle: **@CODE@** = n/a
    /// kpv: **@CODE@**
    /// swe: (no comment)
    /// fkv: **@CODE@** =
    /// fkv: **@CODE@**:  variant 1
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 1
    /// lut: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@**
    /// fao: (no comment)
    /// fao: **@CODE@** - Paradigm identifier (e.g. gera+v1 = ger)
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** version tags
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "v1")]
    v1,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// tgl: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v10")]
    v10,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v11")]
    v11,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v12")]
    v12,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v13")]
    v13,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v14")]
    v14,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v15")]
    v15,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v16")]
    v16,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v17")]
    v17,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v18")]
    v18,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v19")]
    v19,
    /// fit: **@CODE@**
    /// gle: **@CODE@** = n/a
    /// kpv: **@CODE@**
    /// fkv: **@CODE@** =
    /// fkv: **@CODE@**:  variant 2
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 2
    /// lut: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@**
    /// fao: **@CODE@** - Paradigm identifier (e.g. gera+v2 = gerar)
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// rmn: (no comment)
    /// vep: **@CODE@**
    /// rmg: **@CODE@**
    /// myv: (no comment)
    /// rmu: **@CODE@** = i bruk
    /// olo: **@CODE@** version tags
    /// koi: (no comment)
    /// deu: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@**
    /// fin: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    /// bak: (no comment)
    #[strum(serialize = "v2")]
    v2,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v20")]
    v20,
    /// kpv: **@CODE@**
    /// lut: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v21")]
    v21,
    /// kpv: **@CODE@**
    /// lut: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v22")]
    v22,
    /// kpv: **@CODE@**
    /// lut: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v23")]
    v23,
    /// kpv: **@CODE@**
    /// lut: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v24")]
    v24,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 3
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 3
    /// lut: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// myv: (no comment)
    /// olo: **@CODE@** version tags
    /// koi: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@**
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v3")]
    v3,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 4
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 4
    /// lut: (no comment)
    /// tgl: (no comment)
    /// nob: **@CODE@**
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// vep: **@CODE@**
    /// myv: (no comment)
    /// olo: **@CODE@** version tags
    /// koi: (no comment)
    /// sme: (no comment)
    /// smn: **@CODE@**  for different paradigm variants of same lexeme
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v4")]
    v4,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 5
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 5
    /// lut: (no comment)
    /// tgl: (no comment)
    /// smj: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// olo: **@CODE@** version tags
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v5")]
    v5,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 6
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 6
    /// lut: (no comment)
    /// tgl: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v6")]
    v6,
    /// kpv: **@CODE@**
    /// fkv: **@CODE@**:  variant 7
    /// mhr: (no comment)
    /// sma: **@CODE@**:  variant 7
    /// lut: (no comment)
    /// tgl: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// skf: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v7")]
    v7,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// tgl: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
    /// ceb: (no comment)
    /// vro: (no comment)
    #[strum(serialize = "v8")]
    v8,
    /// kpv: **@CODE@**
    /// mhr: (no comment)
    /// lut: (no comment)
    /// tgl: (no comment)
    /// udm: (no comment)
    /// slh: (no comment)
    /// apu: (no comment)
    /// mns: **@CODE@**
    /// sms: **@CODE@**
    /// myv: (no comment)
    /// koi: (no comment)
    /// sme: (no comment)
    /// mdf: (no comment)
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
        since = "0.2",
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

    #[test]
    fn serialize_and_deserialize() {
        let string: String = serde_json::to_string(&Tag::N).unwrap();
        let x: Tag = serde_json::from_str("\"N\"").expect("parsed as tag");
        assert_eq!(x, Tag::N);
    }
}
