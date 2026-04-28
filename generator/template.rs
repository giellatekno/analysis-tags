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
    //%%%%%GENERATED_CODE_HERE%%%%%
}

impl Tag {
    pub fn is_pos(&self) -> bool {
        use Tag::{Adp, Det, Interj, Num, Pcle, Po, Pr, Pron, A, ABBR, CC, CLB, CS, N, URL, V};
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
        use Tag::{Dem, Indef, Interr, Neg, NomAg, Ord, Pers, Prop, Qu, Recipr, Refl, Rel, G3, G7};
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
