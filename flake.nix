{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-21.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        # TODO: How can i specify extensions without using a temp var?
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
        # Not sure specifying any extensions is needed. Keeping this here for future.
        # rust = rusttmp.override {
        #   extensions = [ "rust-analysis" ];
        # };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-analyzer
            rust
            pkgs.pkg-config
            pkgs.binutils
            pkgs.gcc
            pkgs.openssl
            pkgs.sqlite
            pkgs.nodePackages.npm
          ];
        };
      }
    );
}
