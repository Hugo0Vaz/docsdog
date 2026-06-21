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
          mkdocsWithMaterial = pkgs.python313.withPackages (ps: [
            ps.mkdocs-material
          ]);
        in
        {
          devShells.default = pkgs.mkShell {
            name = "docsdog-docs";

            buildInputs = [
              mkdocsWithMaterial
            ];

            shellHook = ''
              # Prepare docs directory with symlinks to spec files and assets
              ln -sf ../../docsdog-spec/specification.md mkdocs/docs/specification.md
              ln -sf ../../docsdog-spec/scan.schema.json mkdocs/docs/scan.schema.json
              ln -sf ../../docsdog-spec/relationship.schema.json mkdocs/docs/relationship.schema.json
              ln -sf ../../docsdog-spec/scan-example.json mkdocs/docs/scan-example.json
              ln -sf ../../assets mkdocs/docs/assets

              echo "🐕   Docs Dog — Documentation Environment"
              echo "      mkdocs:   $(mkdocs --version)"
              echo ""
              echo "  Commands:"
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
              # Copy external files into mkdocs/docs/ so mkdocs can find them
              cp docsdog-spec/specification.md mkdocs/docs/
              cp docsdog-spec/scan.schema.json mkdocs/docs/
              cp docsdog-spec/relationship.schema.json mkdocs/docs/
              cp docsdog-spec/scan-example.json mkdocs/docs/
              cp -r assets mkdocs/docs/

              mkdocs build -f mkdocs/mkdocs.yml --strict --site-dir $out
            '';

            # mkdocs build already writes to --site-dir, nothing to install
            dontInstall = true;
          };
        };
    };
}
