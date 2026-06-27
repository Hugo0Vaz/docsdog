use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

// ─── CLI definition ────────────────────────────────────────────────

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
    },
    /// Generate a Requirement.
    Req {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (fewer sections).
        #[arg(long, default_value_t = false)]
        minimal: bool,
    },
    /// Generate a Use Case.
    Uc {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (UML Simplified instead of Cockburn).
        #[arg(long, default_value_t = false)]
        minimal: bool,
    },
    /// Generate a User Story.
    Us {
        /// Use bare template (placeholders only, no guidance).
        #[arg(long, default_value_t = false)]
        bare: bool,
        /// Use minimal template (fewer sections).
        #[arg(long, default_value_t = false)]
        minimal: bool,
    },
}

// ─── Document type metadata ────────────────────────────────────────

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

// ─── Template lookup ───────────────────────────────────────────────

/// Returns the content of the individual (tracked) template for a document type
/// and flag combination. Returns `None` if the variant doesn't exist.
fn get_make_template(doc: &DocType, bare: bool, minimal: bool) -> Option<&'static str> {
    use DocTypeKind::*;
    let kind = match doc.label {
        "adr" => Adr,
        "req" => Req,
        "uc"  => Uc,
        "us"  => Us,
        _ => unreachable!(),
    };
    match (kind, bare, minimal) {
        // ADR — all 4 variants
        (Adr, false, false) => Some(include_str!("templates/en/adr/adr-template.md")),
        (Adr, true,  false) => Some(include_str!("templates/en/adr/adr-template-bare.md")),
        (Adr, false, true ) => Some(include_str!("templates/en/adr/adr-template-minimal.md")),
        (Adr, true,  true ) => Some(include_str!("templates/en/adr/adr-template-bare-minimal.md")),
        // REQ — full + bare only
        (Req, false, false) => Some(include_str!("templates/en/srs/req-template.md")),
        (Req, true,  false) => Some(include_str!("templates/en/srs/req-template-bare.md")),
        (Req, _,     true ) => None,
        // UC — all 4 variants
        (Uc,  false, false) => Some(include_str!("templates/en/ucs/uc-template.md")),
        (Uc,  true,  false) => Some(include_str!("templates/en/ucs/uc-template-bare.md")),
        (Uc,  false, true ) => Some(include_str!("templates/en/ucs/uc-template-minimal.md")),
        (Uc,  true,  true ) => Some(include_str!("templates/en/ucs/uc-template-bare-minimal.md")),
        // US — full + bare only
        (Us,  false, false) => Some(include_str!("templates/en/us/us-template.md")),
        (Us,  true,  false) => Some(include_str!("templates/en/us/us-template-bare.md")),
        (Us,  _,     true ) => None,
    }
}

/// Lookup discriminant (not public).
enum DocTypeKind { Adr, Req, Uc, Us }

/// Returns the content of an index (container) template for `dir` and flag.
fn get_init_template(dir: &str, bare: bool) -> &'static str {
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

/// Each init entry: (directory under docs/, name of the index file inside it).
const INIT_ENTRIES: &[(&str, &str)] = &[
    ("adr", "adrs.md"),
    ("req", "srs.md"),
    ("uc",  "ucs.md"),
    ("us",  "uss.md"),
];

// ─── Numbering ─────────────────────────────────────────────────────

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

// ─── Commands ──────────────────────────────────────────────────────

fn cmd_init(bare: bool) {
    let base = docs_dir();
    for (dir, file) in INIT_ENTRIES {
        let dir_path = base.join(dir);
        fs::create_dir_all(&dir_path).unwrap_or_else(|e| {
            eprintln!("Error: could not create directory docs/{}: {e}", dir);
            std::process::exit(1);
        });
        let template = get_init_template(dir, bare);
        let dest = dir_path.join(file);
        fs::write(&dest, template).unwrap_or_else(|e| {
            eprintln!("Error: could not write docs/{dir}/{file}: {e}");
            std::process::exit(1);
        });
        println!("Created docs/{dir}/{file}");
    }
}

fn cmd_make(doc: &DocType, bare: bool, minimal: bool) {
    // Validate flag availability
    if minimal && !doc.has_minimal {
        eprintln!(
            "Error: --minimal is not available for '{}'. No minimal template exists.",
            doc.label
        );
        std::process::exit(1);
    }

    let template = get_make_template(doc, bare, minimal)
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

// ─── Entry point ───────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { bare } => cmd_init(bare),
        Commands::Make(target) => match target {
            MakeTarget::Adr { bare, minimal } => cmd_make(&ADR, bare, minimal),
            MakeTarget::Req { bare, minimal } => cmd_make(&REQ, bare, minimal),
            MakeTarget::Uc  { bare, minimal } => cmd_make(&UC,  bare, minimal),
            MakeTarget::Us  { bare, minimal } => cmd_make(&US,  bare, minimal),
        },
    }
}
