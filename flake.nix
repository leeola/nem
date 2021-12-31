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
        rusttmp = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain;
        rust = rusttmp.override {
          extensions = [ "rust-analysis" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
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
            pkgs.nodejs
            pkgs.nodePackages.npm
          ];
        };
      }
    );
}
