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
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        anyhow::bail!("usage: PROG <template-file>");
    }

    let template_file = &args[1];

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

    let code = std::fs::read_to_string(template_file)?;
    let code = code.replace("//%%%%%GENERATED_CODE_HERE%%%%%", &variants_code);

    let mut stdout = std::io::stdout().lock();
    std::io::Write::write_all(&mut stdout, code.as_bytes())?;
    Ok(())
}
