{
  description = "Menu scripts";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";

    # Dependencies for the CLI
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      inherit (pkgs) lib;

      craneLib = (crane.mkLib pkgs).overrideToolchain (p:
        p.rust-bin.stable.latest.default.override {
          targets = ["x86_64-unknown-linux-musl"];
        });

      # Custom filter for source files
      cleanSourceFilter = path: type: (craneLib.filterCargoSources path type);

      my-crate = craneLib.buildPackage {
        src = lib.cleanSourceWith {
          src = ./.;
          filter = cleanSourceFilter;
          name = "source";
        };
        strictDeps = true;

        TARGET_CC = let
          cc = pkgs.pkgsStatic.stdenv.cc;
        in "${pkgs.pkgsStatic.stdenv.cc}/bin/${cc.targetPrefix}cc";

        OPENSSL_STATIC = "1";
        OPENSSL_LIB_DIR = "${pkgs.pkgsStatic.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.pkgsStatic.openssl.dev}/include";

        CARGO_PROFILE = "release-package";
        CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
        CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
      };
    in {
      checks = {inherit my-crate;};

      packages.default = my-crate;
    });
}
