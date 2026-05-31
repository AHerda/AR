{
  description = "Rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system2 = "aarch64-linux";
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
    in {
      devShells."${system}".default = pkgs.mkShell {
        name = "python dev shell";
        shellHook = ''
          echo "Welcome to the python dev shell"
        '';

        packages = with pkgs; [
          (python314.withPackages(p: []))
        ];
      };
    };
}
