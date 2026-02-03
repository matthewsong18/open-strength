{
  description = "Core Business Logic - Dev Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # rust-overlay is used so that the rust version can be specified as stable
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        # Adding the rust overlay pkgs
        overlays = [ (import rust-overlay) ];
      };
      
      # Specify the latest stable rust version
      toolchain = pkgs.rust-bin.stable.latest.default.override {
        # Adding eextensions for IDE
        extensions = [ "rust-src" "rust-analyzer" "clippy" ];
      };
    in
    {
      # Creating a dev shell for consistent testing environment
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          toolchain
          # Adding useful packages
          pkgs.bacon
          pkgs.cargo-watch
        ];

        # ENV variables
        RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        
        shellHook = ''
          echo "Environment loaded. Ready to test."
        '';
      };
    };
}
