{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      inherit (pkgs) mkShell;
      toolchain = pkgs.rust-bin.nightly.latest.default;
    in
    {
      devShells =
        let
          t = toolchain.override {
            extensions = [ "rust-analyzer" "rust-src" ];
          };
        in
        {
          default = mkShell {
            packages = with pkgs;[
              t
              pre-commit
            ];
            RUST_SRC_PATH = "${t}";
          };
        };
    });
}
