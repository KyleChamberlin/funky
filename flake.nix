{
  description = "funky — Turn command history into reusable shell functions.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem =
        { self', pkgs, lib, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;

            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [ pkg-config ];

            meta = {
              description = cargoToml.package.description;
              homepage = cargoToml.package.homepage;
              license = lib.licenses.gpl3Plus;
              mainProgram = "funky";
            };
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.packages.default ];
            packages = with pkgs; [
              cargo
              clippy
              rustfmt
              rust-analyzer
            ];
          };

          checks.build = self'.packages.default;
        };

      flake = {
        overlays.default = final: _prev: {
          funky = inputs.self.packages.${final.system}.default;
        };
      };
    };
}
