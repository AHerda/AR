{
  description = "Python flake";

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
          mkdir -p input
          curl -o input/t1/frankenstein.txt https://www.gutenberg.org/files/84/84-0.txt
          curl -o input/t1/dracula.txt https://www.gutenberg.org/files/345/345-0.txt
          curl -o input/t1/sherlock.txt https://www.gutenberg.org/files/1661/1661-0.txt
          curl -o input/t4.csv https://noaa-ghcn-pds.s3.amazonaws.com/csv/by_year/2025.csv
          mkdir -p output
          rm -rf output/*
          echo "Welcome to the python dev shell"
          nu
        '';

        packages = with pkgs; [
          (python314.withPackages(p: [
            p.numpy
          ]))
          hadoop
        ];
      };
    };
}
