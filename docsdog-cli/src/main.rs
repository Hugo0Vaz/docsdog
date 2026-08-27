use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

// ── CLI definition ───────────────────────────────────────────────

/// Scaffold and generate documentation templates for your project.
#[derive(Parser)]
#[command(name = "docsdog", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold the docs/ directory with index documents.
    Init {
        /// Use bare (placeholder-only) templates for index documents.
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Template language (e.g. "en", "pt-br").
        #[arg(long, default_value = "en")]
        lang: String,
    },
    /// Generate a new tracked document from a template.
    #[command(subcommand)]
    Make(MakeTarget),
}

#[derive(Subcommand)]
enum MakeTarget {
    /// Generate an Architecture Decision Record (ADR).
    Adr {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (fewer sections).
        #[arg(long, default_value_t = false)]
        minimal: bool,
        /// Template language (e.g. "en", "pt-br").
        #[arg(long, default_value = "en")]
        lang: String,
    },
    /// Generate a Requirement.
    Req {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (fewer sections).
        #[arg(long, default_value_t = false)]
        minimal: bool,
        /// Template language (e.g. "en", "pt-br").
        #[arg(long, default_value = "en")]
        lang: String,
    },
    /// Generate a Use Case.
    Uc {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (UML Simplified instead of Cockburn).
        #[arg(long, default_value_t = false)]
        minimal: bool,
        /// Template language (e.g. "en", "pt-br").
        #[arg(long, default_value = "en")]
        lang: String,
    },
    /// Generate a User Story.
    Us {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (fewer sections).
        #[arg(long, default_value_t = false)]
        minimal: bool,
        /// Template language (e.g. "en", "pt-br").
        #[arg(long, default_value = "en")]
        lang: String,
    },
}

// ── Document type metadata ────────────────────────────────────────

/// Metadata for each tracked document type.
struct DocType {
    /// Human-readable name for error messages.
    label: &'static str,
    /// Directory name under docs/.
    dir: &'static str,
    /// Filename prefix (e.g. "ADR", "REQ").
    prefix: &'static str,
    /// Whether a -minimal variant exists.
    has_minimal: bool,
}

const ADR: DocType = DocType { label: "adr", dir: "adr", prefix: "ADR", has_minimal: true };
const REQ: DocType = DocType { label: "req", dir: "req", prefix: "REQ", has_minimal: false };
const UC:  DocType = DocType { label: "uc",  dir: "uc",  prefix: "UC",  has_minimal: true };
const US:  DocType = DocType { label: "us",  dir: "us",  prefix: "US",  has_minimal: false };

// ── Template lookup ───────────────────────────────────────────────

/// Returns the content of the individual (tracked) template for a document type
/// and flag combination. Returns `None` if the variant doesn't exist.
fn get_make_template(doc: &DocType, bare: bool, minimal: bool, lang: &str) -> Option<&'static str> {
    match lang {
        "en" => get_make_template_en(doc, bare, minimal),
        "pt-br" => get_make_template_ptbr(doc, bare, minimal),
        _ => {
            eprintln!("Error: unsupported language '{}'. Supported: en, pt-br.", lang);
            std::process::exit(1);
        }
    }
}

fn get_make_template_en(doc: &DocType, bare: bool, minimal: bool) -> Option<&'static str> {
    enum DocTypeKind { Adr, Req, Uc, Us }
    let kind = match doc.label {
        "adr" => DocTypeKind::Adr,
        "req" => DocTypeKind::Req,
        "uc"  => DocTypeKind::Uc,
        "us"  => DocTypeKind::Us,
        _ => unreachable!(),
    };
    match (kind, bare, minimal) {
        (DocTypeKind::Adr, false, false) => Some(include_str!("templates/en/adr/adr-template.md")),
        (DocTypeKind::Adr, true,  false) => Some(include_str!("templates/en/adr/adr-template-bare.md")),
        (DocTypeKind::Adr, false, true ) => Some(include_str!("templates/en/adr/adr-template-minimal.md")),
        (DocTypeKind::Adr, true,  true ) => Some(include_str!("templates/en/adr/adr-template-bare-minimal.md")),
        (DocTypeKind::Req, false, false) => Some(include_str!("templates/en/srs/req-template.md")),
        (DocTypeKind::Req, true,  false) => Some(include_str!("templates/en/srs/req-template-bare.md")),
        (DocTypeKind::Req, _,     true ) => None,
        (DocTypeKind::Uc,  false, false) => Some(include_str!("templates/en/ucs/uc-template.md")),
        (DocTypeKind::Uc,  true,  false) => Some(include_str!("templates/en/ucs/uc-template-bare.md")),
        (DocTypeKind::Uc,  false, true ) => Some(include_str!("templates/en/ucs/uc-template-minimal.md")),
        (DocTypeKind::Uc,  true,  true ) => Some(include_str!("templates/en/ucs/uc-template-bare-minimal.md")),
        (DocTypeKind::Us,  false, false) => Some(include_str!("templates/en/us/us-template.md")),
        (DocTypeKind::Us,  true,  false) => Some(include_str!("templates/en/us/us-template-bare.md")),
        (DocTypeKind::Us,  _,     true ) => None,
    }
}

fn get_make_template_ptbr(doc: &DocType, bare: bool, minimal: bool) -> Option<&'static str> {
    enum DocTypeKind { Adr, Req, Uc, Us }
    let kind = match doc.label {
        "adr" => DocTypeKind::Adr,
        "req" => DocTypeKind::Req,
        "uc"  => DocTypeKind::Uc,
        "us"  => DocTypeKind::Us,
        _ => unreachable!(),
    };
    match (kind, bare, minimal) {
        (DocTypeKind::Adr, false, false) => Some(include_str!("templates/pt-br/adr/adr-template.md")),
        (DocTypeKind::Adr, true,  false) => Some(include_str!("templates/pt-br/adr/adr-template-bare.md")),
        (DocTypeKind::Adr, false, true ) => Some(include_str!("templates/pt-br/adr/adr-template-minimal.md")),
        (DocTypeKind::Adr, true,  true ) => Some(include_str!("templates/pt-br/adr/adr-template-bare-minimal.md")),
        (DocTypeKind::Req, false, false) => Some(include_str!("templates/pt-br/srs/req-template.md")),
        (DocTypeKind::Req, true,  false) => Some(include_str!("templates/pt-br/srs/req-template-bare.md")),
        (DocTypeKind::Req, _,     true ) => None,
        (DocTypeKind::Uc,  false, false) => Some(include_str!("templates/pt-br/ucs/uc-template.md")),
        (DocTypeKind::Uc,  true,  false) => Some(include_str!("templates/pt-br/ucs/uc-template-bare.md")),
        (DocTypeKind::Uc,  false, true ) => Some(include_str!("templates/pt-br/ucs/uc-template-minimal.md")),
        (DocTypeKind::Uc,  true,  true ) => Some(include_str!("templates/pt-br/ucs/uc-template-bare-minimal.md")),
        (DocTypeKind::Us,  false, false) => Some(include_str!("templates/pt-br/us/us-template.md")),
        (DocTypeKind::Us,  true,  false) => Some(include_str!("templates/pt-br/us/us-template-bare.md")),
        (DocTypeKind::Us,  _,     true ) => None,
    }
}

/// Returns the content of an index (container) template for `dir` and flag.
fn get_init_template(dir: &str, bare: bool, lang: &str) -> &'static str {
    match lang {
        "en" => get_init_template_en(dir, bare),
        "pt-br" => get_init_template_ptbr(dir, bare),
        _ => {
            eprintln!("Error: unsupported language '{}'. Supported: en, pt-br.", lang);
            std::process::exit(1);
        }
    }
}

fn get_init_template_en(dir: &str, bare: bool) -> &'static str {
    match (dir, bare) {
        ("adr", false) => include_str!("templates/en/adr/adrs-template.md"),
        ("adr", true)  => include_str!("templates/en/adr/adrs-template-bare.md"),
        ("req", false) => include_str!("templates/en/srs/srs-template.md"),
        ("req", true)  => include_str!("templates/en/srs/srs-template-bare.md"),
        ("uc",  false) => include_str!("templates/en/ucs/ucs-template.md"),
        ("uc",  true)  => include_str!("templates/en/ucs/ucs-template-bare.md"),
        ("us",  false) => include_str!("templates/en/us/uss-template.md"),
        ("us",  true)  => include_str!("templates/en/us/uss-template-bare.md"),
        _ => unreachable!(),
    }
}

fn get_init_template_ptbr(dir: &str, bare: bool) -> &'static str {
    match (dir, bare) {
        ("adr", false) => include_str!("templates/pt-br/adr/adrs-template.md"),
        ("adr", true)  => include_str!("templates/pt-br/adr/adrs-template-bare.md"),
        ("req", false) => include_str!("templates/pt-br/srs/srs-template.md"),
        ("req", true)  => include_str!("templates/pt-br/srs/srs-template-bare.md"),
        ("uc",  false) => include_str!("templates/pt-br/ucs/ucs-template.md"),
        ("uc",  true)  => include_str!("templates/pt-br/ucs/ucs-template-bare.md"),
        ("us",  false) => include_str!("templates/pt-br/us/uss-template.md"),
        ("us",  true)  => include_str!("templates/pt-br/us/uss-template-bare.md"),
        _ => unreachable!(),
    }
}

/// Each init entry: (directory under docs/, name of the index file inside it).
const INIT_ENTRIES: &[(&str, &str)] = &[
    ("adr", "adrs.md"),
    ("req", "srs.md"),
    ("uc",  "ucs.md"),
    ("us",  "uss.md"),
];

// ── Numbering ─────────────────────────────────────────────────────

/// Scan `dir` for files matching `PREFIX-NNN.md` and return max NNN + 1.
/// Returns 1 if no matching files exist or the directory doesn't exist.
fn next_number(dir: &Path, prefix: &str) -> u32 {
    let pattern = format!("{}-", prefix);
    let mut max: u32 = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(&pattern) && name.ends_with(".md") {
                let inner = &name[pattern.len()..name.len() - 3]; // strip ".md"
                if let Ok(n) = inner.parse::<u32>() {
                    max = max.max(n);
                }
            }
        }
    }
    max + 1
}

// ── Commands ──────────────────────────────────────────────────────

fn cmd_init(bare: bool, lang: &str) {
    let base = docs_dir();
    for (dir, file) in INIT_ENTRIES {
        let dir_path = base.join(dir);
        fs::create_dir_all(&dir_path).unwrap_or_else(|e| {
            eprintln!("Error: could not create directory docs/{}: {e}", dir);
            std::process::exit(1);
        });
        let template = get_init_template(dir, bare, lang);
        let dest = dir_path.join(file);
        fs::write(&dest, template).unwrap_or_else(|e| {
            eprintln!("Error: could not write docs/{dir}/{file}: {e}");
            std::process::exit(1);
        });
        println!("Created docs/{dir}/{file}");
    }
}

fn cmd_make(doc: &DocType, bare: bool, minimal: bool, lang: &str) {
    // Validate flag availability
    if minimal && !doc.has_minimal {
        eprintln!(
            "Error: --minimal is not available for '{}'. No minimal template exists.",
            doc.label
        );
        std::process::exit(1);
    }

    let template = get_make_template(doc, bare, minimal, lang)
        .expect("template should exist after validation");

    let dir_path = docs_dir().join(doc.dir);
    fs::create_dir_all(&dir_path).unwrap_or_else(|e| {
        eprintln!("Error: could not create directory docs/{}: {e}", doc.dir);
        std::process::exit(1);
    });

    let n = next_number(&dir_path, doc.prefix);
    let filename = format!("{}-{:03}.md", doc.prefix, n);
    let dest = dir_path.join(&filename);
    fs::write(&dest, template).unwrap_or_else(|e| {
        eprintln!("Error: could not write docs/{}/{}: {e}", doc.dir, filename);
        std::process::exit(1);
    });
    println!("Created docs/{}/{}", doc.dir, filename);
}

fn docs_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("Error: could not determine current directory: {e}");
        std::process::exit(1);
    })
    .join("docs")
}

// ── Entry point ───────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { bare, lang } => cmd_init(bare, &lang),
        Commands::Make(target) => match target {
            MakeTarget::Adr { bare, minimal, lang } => cmd_make(&ADR, bare, minimal, &lang),
            MakeTarget::Req { bare, minimal, lang } => cmd_make(&REQ, bare, minimal, &lang),
            MakeTarget::Uc  { bare, minimal, lang } => cmd_make(&UC,  bare, minimal, &lang),
            MakeTarget::Us  { bare, minimal, lang } => cmd_make(&US,  bare, minimal, &lang),
        },
    }
}