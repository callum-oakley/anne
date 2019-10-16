use failure::{err_msg, format_err};
use handlebars::Handlebars;
use pulldown_cmark::{html, CowStr, Event, Parser, Tag};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Serialize)]
struct Page {
    content: String,
    metadata: toml::Value,
    path: PathBuf,
    source: PathBuf,
    template: String,
}

impl Page {
    fn new(source: PathBuf, dest: PathBuf) -> Result<Page> {
        let md = fs::read_to_string(&source)?;
        let mut parser = Parser::new(&md);

        let metadata = Self::read_metadata(&mut parser)?;

        let template = if let Some(template_value) = metadata.get("template") {
            if let Some(template) = template_value.as_str() {
                Ok(template.to_owned())
            } else {
                Err(err_msg("template field must be a string"))
            }
        } else {
            Err(err_msg("metadata must contain a template field"))
        }?;

        let mut content = String::new();
        html::push_html(&mut content, parser);

        Ok(Page {
            content,
            metadata,
            path: dest,
            source,
            template,
        })
    }

    fn read_metadata(parser: &mut Parser) -> Result<toml::Value> {
        if let Some(Event::Start(Tag::CodeBlock(_))) = parser.next() {
            // Consume the event, but do nothing with it as long as it is the right shape.
        } else {
            return Err(err_msg("page must begin with a fenced code block"));
        };

        let metadata = if let Some(Event::Text(CowStr::Borrowed(s))) = parser.next() {
            s.parse::<toml::Value>()
                .map_err(|err| format_err!("failed to parse metadata as toml ({})", err,))
        } else {
            Err(err_msg("expected code block to contain borrowed text"))
        }?;

        if let Some(Event::End(Tag::CodeBlock(_))) = parser.next() {
            // Consume the event, but do nothing with it as long as it is the right shape.
        } else {
            return Err(err_msg("expected end of code block"));
        };

        Ok(metadata)
    }
}

#[derive(Debug, Serialize)]
struct Focus<'a> {
    dot: &'a Page,
    root: &'a HashMap<String, Page>,
}

fn is_md(path: &Path) -> bool {
    path.extension().map_or(false, |s| s == "md")
}

fn run() -> Result<()> {
    let template_dir = Path::new("templates");
    let content_dir = Path::new("content");
    let build_dir = Path::new("build");

    let mut templates = Handlebars::new();
    for entry in walkdir::WalkDir::new(template_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            templates.register_template_file(
                entry.path().file_stem().unwrap().to_str().unwrap(),
                entry.path(),
            )?;
        }
    }

    let mut pages = HashMap::new();

    if build_dir.exists() {
        fs::remove_dir_all(build_dir)?;
    }
    fs::create_dir(build_dir)?;

    for entry in walkdir::WalkDir::new(content_dir).min_depth(1) {
        let entry = entry?;

        if entry.file_type().is_dir() {
            fs::create_dir(build_dir.join(entry.path().strip_prefix(content_dir)?))?;
        } else if is_md(entry.path()) {
            pages.insert(
                entry
                    .path()
                    .strip_prefix(content_dir)?
                    .to_str()
                    .unwrap()
                    .to_owned(),
                Page::new(
                    entry.path().to_owned(),
                    entry
                        .path()
                        .strip_prefix(content_dir)?
                        .with_extension("")
                        .to_owned(),
                )
                .map_err(|err| format_err!("{}: {}", entry.path().display(), err))?,
            );
        } else {
            let dest = build_dir.join(entry.path().strip_prefix(content_dir)?);
            println!("{} -> {}", entry.path().display(), dest.display());
            fs::hard_link(entry.path(), dest)?;
        }
    }

    for (_, page) in &pages {
        let dest = build_dir.join(page.path.with_extension("html").to_owned());
        println!("{} -> {}", page.source.display(), dest.display());
        fs::write(
            dest,
            templates.render(
                &page.template,
                &Focus {
                    dot: page,
                    root: &pages,
                },
            )?,
        )?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}
