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

      perSystem = { pkgs, ... }:
        let
          phpBase = pkgs.php;
          php = phpBase.buildEnv {
            extensions = { enabled, all }:
              enabled ++ (with all; [
                mbstring
                xml
                dom
                curl
                zip
                openssl
                tokenizer
                fileinfo
                pdo
                intl
              ]);
            extraConfig = ''
              memory_limit = 512M
            '';
          };

          mkdocsWithMaterial = pkgs.python313.withPackages (ps: [
            ps.mkdocs-material
          ]);
        in
        {
          devShells = {
            default = pkgs.mkShell {
              name = "docsdog-php";

              buildInputs = [
                php
                phpBase.packages.composer
              ];

              shellHook = ''
                echo "🐕   Docs Dog — PHP Development Environment"
                echo "      PHP:      $(php --version | head -1)"
                echo "      Composer: $(composer --version 2>/dev/null | head -1)"
                echo ""
              '';
            };

            docs = pkgs.mkShell {
              name = "docsdog-docs";

              buildInputs = [
                mkdocsWithMaterial
              ];

              shellHook = ''
                # Prepare docs directory with symlinks to spec files
                ln -sf ../docsdog-spec/specification.md docs/specification.md
                ln -sf ../docsdog-spec/scan.schema.json docs/scan.schema.json
                ln -sf ../docsdog-spec/relationship.schema.json docs/relationship.schema.json
                ln -sf ../docsdog-spec/scan-example.json docs/scan-example.json

                echo "🐕   Docs Dog — Documentation Environment"
                echo "      mkdocs:   $(mkdocs --version)"
                echo ""
                echo "  Commands:"
                echo "    mkdocs serve    Start live-reloading docs server"
                echo "    mkdocs build    Build static site"
                echo ""
              '';
            };
          };

          packages.docs = pkgs.stdenv.mkDerivation {
            name = "docsdog-docs";
            src = ./.;

            nativeBuildInputs = [
              mkdocsWithMaterial
            ];

            preBuild = ''
              cp docsdog-spec/specification.md docs/
              cp docsdog-spec/scan.schema.json docs/
              cp docsdog-spec/relationship.schema.json docs/
              cp docsdog-spec/scan-example.json docs/
            '';

            buildPhase = ''
              mkdocs build --strict --site-dir $out
            '';

            # mkdocs build already writes to --site-dir, nothing to install
            dontInstall = true;
          };
        };
    };
}
