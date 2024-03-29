use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use regex::{Captures, Regex};
use semver::{Version, VersionReq};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io::{self, Write as _};
use std::path::PathBuf;
use std::process::{self, Command};

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    let preprocessor = Spec::new();

    if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

struct Spec {
    deny_warnings: bool,
    rule_re: Regex,
    admonition_re: Regex,
    std_link_re: Regex,
    std_link_extract_re: Regex,
}

impl Spec {
    pub fn new() -> Spec {
        // This is roughly a rustdoc intra-doc link definition.
        let std_link = r"(?: [a-z]+@ )?
                         (?: std|core|alloc|proc_macro|test )
                         (?: ::[A-Za-z_!:<>{}()\[\]]+ )?";
        Spec {
            deny_warnings: std::env::var("SPEC_DENY_WARNINGS").as_deref() == Ok("1"),
            rule_re: Regex::new(r"(?m)^r\[([^]]+)]$").unwrap(),
            admonition_re: Regex::new(
                r"(?m)^ *> \[!(?<admon>[^]]+)\]\n(?<blockquote>(?: *> .*\n)+)",
            )
            .unwrap(),
            std_link_re: Regex::new(&format!(
                r"(?x)
                    (?:
                        ( \[`[^`]+`\] ) \( ({std_link}) \)
                    )
                    | (?:
                        ( \[`{std_link}`\] )
                    )
                 "
            ))
            .unwrap(),
            std_link_extract_re: Regex::new(
                r#"<li><a [^>]*href="(https://doc.rust-lang.org/[^"]+)""#,
            )
            .unwrap(),
        }
    }

    /// Converts lines that start with `r[…]` into a "rule" which has special
    /// styling and can be linked to.
    fn rule_definitions(
        &self,
        chapter: &Chapter,
        found_rules: &mut BTreeMap<String, (PathBuf, PathBuf)>,
    ) -> String {
        let source_path = chapter.source_path.clone().unwrap_or_default();
        let path = chapter.path.clone().unwrap_or_default();
        self.rule_re
            .replace_all(&chapter.content, |caps: &Captures| {
                let rule_id = &caps[1];
                if let Some((old, _)) =
                    found_rules.insert(rule_id.to_string(), (source_path.clone(), path.clone()))
                {
                    let message = format!(
                        "rule `{rule_id}` defined multiple times\n\
                        First location: {old:?}\n\
                        Second location: {source_path:?}"
                    );
                    if self.deny_warnings {
                        panic!("error: {message}");
                    } else {
                        eprintln!("warning: {message}");
                    }
                }
                format!(
                    "<div class=\"rule\" id=\"{rule_id}\">\
                     <a class=\"rule-link\" href=\"#{rule_id}\">[{rule_id}]</a>\
                     </div>\n"
                )
            })
            .to_string()
    }

    /// Generates link references to all rules on all pages, so you can easily
    /// refer to rules anywhere in the book.
    fn auto_link_references(
        &self,
        chapter: &Chapter,
        found_rules: &BTreeMap<String, (PathBuf, PathBuf)>,
    ) -> String {
        let current_path = chapter.path.as_ref().unwrap().parent().unwrap();
        let definitions: String = found_rules
            .iter()
            .map(|(rule_id, (_, path))| {
                let relative = pathdiff::diff_paths(path, current_path).unwrap();
                format!("[{rule_id}]: {}#{rule_id}\n", relative.display())
            })
            .collect();
        format!(
            "{}\n\
            {definitions}",
            chapter.content
        )
    }

    /// Converts blockquotes with special headers into admonitions.
    ///
    /// The blockquote should look something like:
    ///
    /// ```
    /// > [!WARNING]
    /// > ...
    /// ```
    ///
    /// This will add a `<div class="warning">` around the blockquote so that
    /// it can be styled differently. Any text between the brackets that can
    /// be a CSS class is valid. The actual styling needs to be added in a CSS
    /// file.
    fn admonitions(&self, chapter: &Chapter) -> String {
        self.admonition_re
            .replace_all(&chapter.content, |caps: &Captures| {
                let lower = caps["admon"].to_lowercase();
                format!(
                    "<div class=\"{lower}\">\n\n{}\n\n</div>\n",
                    &caps["blockquote"]
                )
            })
            .to_string()
    }

    /// Converts links to the standard library to the online documentation in
    /// a fashion similar to rustdoc intra-doc links.
    fn std_links(&self, chapter: &Chapter) -> String {
        // This is very hacky, but should work well enough.
        //
        // Collect all standard library links.
        //
        // links are tuples of ("[`std::foo`]", None) for links without dest,
        // or ("[`foo`]", "std::foo") with a dest.
        let mut links: Vec<_> = self
            .std_link_re
            .captures_iter(&chapter.content)
            .map(|cap| {
                if let Some(no_dest) = cap.get(3) {
                    (no_dest.as_str(), None)
                } else {
                    (
                        cap.get(1).unwrap().as_str(),
                        Some(cap.get(2).unwrap().as_str()),
                    )
                }
            })
            .collect();
        if links.is_empty() {
            return chapter.content.clone();
        }
        links.sort();
        links.dedup();

        // Write a Rust source file to use with rustdoc to generate intra-doc links.
        let tmp = tempfile::TempDir::with_prefix("mdbook-spec-").unwrap();
        let src_path = tmp.path().join("a.rs");
        // Allow redundant since there could some in-scope things that are
        // technically not necessary, but we don't care about (like
        // [`Option`](std::option::Option)).
        let mut src = format!(
            "#![deny(rustdoc::broken_intra_doc_links)]\n\
             #![allow(rustdoc::redundant_explicit_links)]\n"
        );
        for (link, dest) in &links {
            write!(src, "//! - {link}").unwrap();
            if let Some(dest) = dest {
                write!(src, "({})", dest).unwrap();
            }
            src.push('\n');
        }
        writeln!(
            src,
            "extern crate alloc;\n\
             extern crate proc_macro;\n\
             extern crate test;\n"
        )
        .unwrap();
        fs::write(&src_path, &src).unwrap();
        let output = Command::new("rustdoc")
            .arg("--edition=2021")
            .arg(&src_path)
            .current_dir(tmp.path())
            .output()
            .expect("rustdoc installed");
        if !output.status.success() {
            eprintln!(
                "error: failed to extract std links ({:?}) in chapter {} ({:?})\n",
                output.status,
                chapter.name,
                chapter.source_path.as_ref().unwrap()
            );
            io::stderr().write_all(&output.stderr).unwrap();
            process::exit(1);
        }

        // Extract the links from the generated html.
        let generated =
            fs::read_to_string(tmp.path().join("doc/a/index.html")).expect("index.html generated");
        let urls: Vec<_> = self
            .std_link_extract_re
            .captures_iter(&generated)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();
        if urls.len() != links.len() {
            eprintln!(
                "error: expected rustdoc to generate {} links, but found {} in chapter {} ({:?})",
                links.len(),
                urls.len(),
                chapter.name,
                chapter.source_path.as_ref().unwrap()
            );
            process::exit(1);
        }

        // Replace any disambiguated links with just the disambiguation.
        let mut output = self
            .std_link_re
            .replace_all(&chapter.content, |caps: &Captures| {
                if let Some(dest) = caps.get(2) {
                    // Replace destination parenthesis with a link definition (square brackets).
                    format!("{}[{}]", &caps[1], dest.as_str())
                } else {
                    caps[0].to_string()
                }
            })
            .to_string();

        // Append the link definitions to the bottom of the chapter.
        write!(output, "\n").unwrap();
        for ((link, dest), url) in links.iter().zip(urls) {
            if let Some(dest) = dest {
                write!(output, "[{dest}]: {url}\n").unwrap();
            } else {
                write!(output, "{link}: {url}\n").unwrap();
            }
        }

        output
    }
}

impl Preprocessor for Spec {
    fn name(&self) -> &str {
        "nop-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut found_rules = BTreeMap::new();
        for section in &mut book.sections {
            let BookItem::Chapter(ch) = section else {
                continue;
            };
            if ch.is_draft_chapter() {
                continue;
            }
            ch.content = self.rule_definitions(&ch, &mut found_rules);
            ch.content = self.admonitions(&ch);
            ch.content = self.std_links(&ch);
        }
        for section in &mut book.sections {
            let BookItem::Chapter(ch) = section else {
                continue;
            };
            if ch.is_draft_chapter() {
                continue;
            }
            ch.content = self.auto_link_references(&ch, &found_rules);
        }

        Ok(book)
    }
}
