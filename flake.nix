{
  description = "Docs Dog — Opinionated project documentation tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ { self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = { pkgs, system, ... }:
        let
          mkdocsWithMaterial = pkgs.python313.withPackages (ps: [
            ps.mkdocs-material
            ps.mkdocs-gen-files
          ]);
        in
        {
          devShells.default = pkgs.mkShell {
            name = "docsdog";

            buildInputs = [
              mkdocsWithMaterial
              pkgs.cargo
              pkgs.rustc
              pkgs.rustfmt
              pkgs.clippy
              pkgs.rust-analyzer
              pkgs.just
            ];

            shellHook = ''
              echo "🐕   Docs Dog"
              echo "      rustc:    $(rustc --version)"
              echo "      cargo:    $(cargo --version)"
              echo "      mkdocs:   $(mkdocs --version)"
              echo ""
              echo "  docsdog-cli:"
              echo "    cargo build       Build the project"
              echo "    cargo run         Run the CLI"
              echo "    cargo test        Run tests"
              echo "    cargo fmt         Format code"
              echo "    cargo clippy      Lint code"
              echo ""
              echo "  docs:"
              echo "    mkdocs serve -f mkdocs/mkdocs.yml    Start live-reloading docs server"
              echo "    mkdocs build -f mkdocs/mkdocs.yml    Build static site"
              echo ""
            '';
          };

          packages.docs = pkgs.stdenv.mkDerivation {
            name = "docsdog-docs";
            src = ./.;

            nativeBuildInputs = [
              mkdocsWithMaterial
            ];

            buildPhase = ''
              mkdocs build -f mkdocs/mkdocs.yml --strict --site-dir $out
            '';
            dontInstall = true;
          };
        };
    };
}
