{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-flake = {
      url = "github:juspay/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      perSystem =
        { self', inputs', ... }:
        let
          pkgs = inputs'.nixpkgs.legacyPackages;
        in
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.devShells.rust ];
            packages = [
              pkgs.elf2uf2-rs
              pkgs.probe-rs
            ];
          };
        };
    };
}
