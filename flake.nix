{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    git-hooks.url = "github:cachix/git-hooks.nix";
    flake-compat.url = "https://git.lix.systems/lix-project/flake-compat/archive/main.tar.gz";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      # inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    inputs@{
      flake-parts,
      git-hooks,
      treefmt-nix,
      crane,
      fenix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        git-hooks.flakeModule
        treefmt-nix.flakeModule
        ./nix/hooks.nix
        ./nix/fmt.nix
        ./nix/shells.nix
      ];
      systems = [ "x86_64-linux" ];
      perSystem =
        {
          pkgs,
          inputs',
          ...
        }:
        let
          rustToolchain = inputs'.fenix.packages.combine [
            inputs'.fenix.packages.complete.toolchain
            inputs'.fenix.packages.targets.wasm32-wasip2.latest.rust-std
          ];
        in
        {
          _module.args = {
            inherit rustToolchain;

            craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          };
        };
    };
}
