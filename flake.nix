{
  description = "Flake for github:mahyarmirrashed/mastermind";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        mastermind = pkgs.rustPlatform.buildRustPackage {
          pname = "mastermind";
          version = cargoToml.package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            installShellFiles
            scdoc
          ];

          postInstall = ''
            scdoc < doc/mastermind.6.scd > mastermind.6
            installManPage mastermind.6
          '';
        };
      in
      {
        packages.default = mastermind;

        apps.default = {
          type = "app";
          program = "${mastermind}/bin/mastermind";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };
      }
    );
}
