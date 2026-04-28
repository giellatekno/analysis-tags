use std::collections::HashMap;
use std::path::PathBuf;

const ROOT_LEXC_PATH: &str = "src/fst/morphology/root.lexc";

/// Returns a [`str`] with the prefix and suffix of `substr` removed, or
/// `None` if the string doesn't both start and end with `substr`.
pub fn strip<'a>(string: &'a str, substr: &str) -> Option<&'a str> {
    string
        .strip_prefix(substr)
        .and_then(|s| s.strip_suffix(substr))
}

/// Error returned when trying to find the root directory
/// from the gut config.
#[derive(thiserror::Error, Debug)]
pub enum FindGutRootError {
    /// There is no gut config dir on the system
    #[error("no config dir for user")]
    NoConfigDir,
    /// Can't read the gut app.toml config file. Contains the inner
    /// [`std::io::Error`].
    #[error("error reading file: {0}")]
    ReadFileError(#[from] std::io::Error),
    /// Error in the app.toml: contains no "root" property
    #[error("no 'root' property in app.toml")]
    NoRootPropertyInAppToml,
}

/// Return the path to the `gut` root on this system, as a `PathBuf`, by
/// reading the `root` property of `$HOME/.config/gut/app.toml`. Returns
/// None
/// `root` property of it, to find the gut root on this system. Returns
/// [`None`] if anything fails.
pub fn find_gut_root() -> Result<PathBuf, FindGutRootError> {
    let app_toml_path = dirs::config_dir()
        .ok_or(FindGutRootError::NoConfigDir)?
        .join("gut/app.toml");
    let app_toml_contents = std::fs::read_to_string(app_toml_path)?;

    for line in app_toml_contents.lines() {
        let Some((prop, value)) = line.split_once('=') else {
            // line doesn't contain a '='
            continue;
        };
        let prop = prop.trim();
        if prop != "root" {
            continue;
        }

        let value = value.trim();
        let value = if let Some(stripped) = strip(value, "\"") {
            stripped
        } else {
            value
        };

        return Ok(PathBuf::from(value));
    }

    Err(FindGutRootError::NoRootPropertyInAppToml)
}

fn skip_spaces(input: &str) -> &str {
    input.trim_start()
}

fn none_on_empty(s: &str) -> Option<&str> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

struct Line<'a> {
    /// The tag how it appears in the lexc source
    original_tag: &'a str,
    /// The tag after making it rust-code-variant friendly
    cleaned_tag: String,
    /// If there is a comment, the comment, otherwise None.
    comment: Option<&'a str>,
}

/// Parse the line into a tag, original tag, and, if present, the comment.
fn parse_line<'a>(line: &'a str) -> Option<Line<'a>> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let original_tag = find_tag(line)?;
    let cleaned_tag = clean_tag(original_tag)?;

    Some(Line {
        original_tag,
        cleaned_tag,
        comment: find_comment(line),
    })
}

/// Find the tag on the line. If the line doesn't start with "+", return None.
///
/// The tag is everything from (not including) the first "+", and the ending character.
///
/// Many characters end the reading of the tag. These are:
///
/// - (implicit) newline ('\n'): A tag cannot span more than one line. However, we iterate
///                              over lines anyway, so this isn't really handled here.
/// - ' ' or '\t': A tag never has spaces in them, so we know the tag is done here.
/// - '!': Exclamation mark is the comment character in lexc.
/// - '+': Plus is the character that denotes a new tag starting, so the tag must be done
///        here.
/// - ':', '%', '’', '^', ',': Any other of these sign characters are used for some other
///                            purpose, but is definetely not a tag we are interested in.
/// The ending character is any whitespace, tab, the comment character (!), or the colon,
/// percent, or a plus (which indicates another tag).
fn find_tag(line: &str) -> Option<&str> {
    fn end_char(ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '!' | '+' | ':' | '%' | '’' | '^' | ',')
    }
    line.strip_prefix("+")
        .map(|line| line.find(end_char).map(|i| &line[..i]).unwrap_or(line))
}

/// Make the tag into a valid enum variant, but keep the original, because we need to be
/// able to parse it, and print it out.
fn clean_tag(tag: &str) -> Option<String> {
    let mut chars = tag.chars();

    let mut out = String::new();
    // the ? returns None if no first char (i.e. empty empty)
    match chars.next()? {
        digit @ '0'..='9' => {
            out.push('_');
            out.push(digit);
        }
        ch => out.push(ch),
    }

    for ch in chars {
        match ch {
            'ǥ' => out.push_str("_gline_"),
            // cyrillic M -> latin M
            'М' => out.push('M'),
            // cyrillic a -> latin a
            'а' => out.push_str("_CYRILLIC_a"),
            '%' | '→' => out.push('_'),
            '-' => out.push_str("_MINUS_"),
            '.' => out.push_str("_DOT_"),
            '(' => out.push_str("_LEFTPAREN_"),
            ')' => out.push_str("_RIGHTPAREN_"),
            '/' => out.push_str("_SLASH_"),
            // an actual En Dash was found, probably meant just a minus/hyphen
            '–' => out.push_str("_EMDASH_"),
            // Seems to only appear in the tag "Cmp#"
            '#' => out.push_str("_POUND"),
            other => out.push(other),
        }
    }

    Some(out)
}

/// Find the comment on the line, or `None` if there is no comment. A line with a comment
/// looks like this: `+Foc/kaan  !!≈ * **@CODE@** = Focusclitic -kaan`
fn find_comment(line: &str) -> Option<&str> {
    let marker_index = line.find("**@CODE@**")?;
    let line = &line[marker_index..];
    let line = skip_spaces(line);
    let line = none_on_empty(line)?;

    let line = if let Some(stripped) = line.strip_prefix(|ch| ch == '=' || ch == ':') {
        stripped
    } else {
        line
    };
    let line = skip_spaces(line);
    let line = none_on_empty(line)?;

    // The comment is now everything until end of line
    Some(line)
}

fn main() -> anyhow::Result<()> {
    // map of original tag (String) -> (converted_tag, [(lang, explanation), ...])
    let mut tags: HashMap<String, (String, Vec<(String, Option<String>)>)> = HashMap::new();

    walkdir::WalkDir::new(find_gut_root()?.join("giellalt"))
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .for_each(|e| {
            let e = match e {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("directory entry error: {e}");
                    return;
                }
            };

            let dir_name = e
                .file_name()
                .to_str()
                .expect("all our file names are valid utf-8");

            let Some(langname) = dir_name.strip_prefix("lang-") else {
                // doesn't start with "lang-", ignore
                return;
            };

            // special case: skip sjd-x-private, it is not used (yet)
            if langname == "sjd-x-private" {
                return;
            }

            let path = e.path().join(ROOT_LEXC_PATH);
            let string = match std::fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: cannot read file: {e}");
                    return;
                }
            };

            for line in string.split('\n') {
                let Some(line) = parse_line(line) else {
                    continue;
                };

                let original_tag = line.original_tag.to_string();
                let cleaned_tag = line.cleaned_tag;
                let comment = line.comment.map(|c| c.to_string());

                let item = tags
                    .entry(original_tag)
                    .or_insert_with(|| (cleaned_tag, vec![]));
                item.1.push((langname.to_string(), comment));
            }
        });

    let mut tags = tags.into_iter().collect::<Vec<_>>();
    tags.sort_by(|(tag_a, _), (tag_b, _)| tag_a.cmp(tag_b));

    let mut variants_code = String::new();

    for (original_tag, (cleaned_tag, langs)) in tags.into_iter() {
        for (langname, comment) in langs {
            let comment = if let Some(ref comment) = comment {
                comment.as_str()
            } else {
                "(no comment)"
            };
            variants_code.push_str("    /// ");
            variants_code.push_str(&langname);
            variants_code.push_str(": ");
            variants_code.push_str(comment);
            variants_code.push('\n');
        }

        variants_code.push_str("    #[strum(serialize = \"");
        variants_code.push_str(&original_tag);
        variants_code.push_str("\")]\n    ");
        variants_code.push_str(&cleaned_tag);
        variants_code.push_str(",\n");
    }

    let code = format!(
        "
        #![allow(non_camel_case_types)]
        use strum_macros::{{AsRefStr, EnumString}};
        /// Error type returned from `Tag::try_from` if no tag is matched.
        #[derive(Debug)]
        pub struct UnknownTagError(String);
        impl ::std::fmt::Display for UnknownTagError {{
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
                write!(f, \"{{}}\", self.0)
            }}
        }}
        impl ::std::error::Error for UnknownTagError {{}}
        /// Initializer for the `UnknownTagError` error, which sets the offending string.
        pub fn unknown_tag(s: &str) -> UnknownTagError {{
            UnknownTagError(s.to_string())
        }}
        /// An fst Tag. Every single possible tag in our infrastructure is its own
        /// variant.
        #[derive(Debug, PartialEq, AsRefStr, EnumString)]
        #[strum(
            parse_err_fn = unknown_tag,
            parse_err_ty = UnknownTagError
        )]
        pub enum Tag {{
            {variants_code}
        }}
        {IMPL_CODE}
    "
    );
    //let mut code = String::new();
    //code.push_str("use strum_macros::{AsRefStr, EnumString};\n");
    //code.push_str("#[derive(Debug)]\n");
    //code.push_str("pub struct UnknownTagError(String);\n");
    //code.push_str("impl ::std::fmt::Display for UnknownTagError {\n");
    //code.push_str("    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {\n");
    //code.push_str("        write!(f, \"{}\", self.0)\n");
    //code.push_str("    }\n");
    //code.push_str("}\n");
    //code.push_str("impl ::std::error::Error for UnknownTagError {}\n");
    //code.push_str("pub fn unknown_tag(s: &str) -> UnknownTagError {\n");
    //code.push_str("    UnknownTagError(s.to_string())\n");
    //code.push_str("}\n");
    //code.push_str("#[derive(Debug, PartialEq, AsRefStr, EnumString)]\n");
    //code.push_str("#[strum(\n");
    //code.push_str("    parse_err_fn = unknown_tag,\n");
    //code.push_str("    parse_err_ty = UnknownTagError\n");
    //code.push_str(")]\n");
    //code.push_str("pub enum Tag {\n");
    //code.push_str(&variants_code);
    //code.push_str("}");

    let mut stdout = std::io::stdout().lock();
    std::io::Write::write_all(&mut stdout, code.as_bytes())?;
    Ok(())
}

const IMPL_CODE: &str = r#"
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
}
"#;
