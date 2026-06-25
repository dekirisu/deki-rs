{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: {
    devShells.x86_64-linux.default =
      nixpkgs.legacyPackages.x86_64-linux.mkShell {
        buildInputs = [
          nixpkgs.legacyPackages.x86_64-linux.rustc
          nixpkgs.legacyPackages.x86_64-linux.cargo
        ];
      };
  };
}
