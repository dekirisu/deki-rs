{
  description = "Rust development shell with rust-src";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          rustSrc = pkgs.rustPlatform.rustcSrc;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              nodejs
              rustc
              cargo
              rustfmt
              clippy
              rust-analyzer
              rustSrc
            ];

            RUST_SRC_PATH = "${rustSrc}/library";
          };
        });
    };
}
