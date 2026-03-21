{
  description = "Rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-linux";
      system2 = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
    in {
      devShells."${system}".default = pkgs.mkShell {
        name = "python dev shell";
        shellHook = ''
          echo "Welcome to the python dev shell"
          nu
        '';

        packages = with pkgs; [
          (python314.withPackages(p: [
            p.grpcio
            p.grpcio-tools
          ]))
        ];
      };
    };
}
