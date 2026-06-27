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

          pythonDevShell = import ./docsdog-cli/python-dev-shell.nix {
            inherit pkgs system;
          };
        in
        {
          devShells = {
            # Documentation shell (root of the repo)
            default = pkgs.mkShell {
            name = "docsdog-docs";

            buildInputs = [
              mkdocsWithMaterial
            ];

            shellHook = ''
              echo "🐕   Docs Dog — Documentation Environment"
              echo "      mkdocs:   $(mkdocs --version)"
              echo ""
              echo "  Commands:"
              echo "    mkdocs serve -f mkdocs/mkdocs.yml    Start live-reloading docs server"
              echo "    mkdocs build -f mkdocs/mkdocs.yml    Build static site"
              echo ""
              echo "  Source files (docsdog-spec/*, assets/*) are copied into the"
              echo "  docs dir at build time by the mkdocs-gen-files plugin."
              echo ""
            '';
          };

            # Python CLI shell (docsdog-cli/ directory)
            cli = pythonDevShell;
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
