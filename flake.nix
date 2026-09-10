{
  description = "CDK Wallet Workshop Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
        let
          overlays = [ (import rust-overlay) ];

          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              protobuf
              openssl
              pkg-config
            ];

            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        }
    );
}
