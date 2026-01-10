{
  description = "Pavan's CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

    in
    {

      formatter.${system} = nixpkgs.legacyPackages.x86_64-linux.nixfmt-rfc-style;

      packages.${system} = {
        # TODO:: Reuse the below devShell packages here as well.
        ci = pkgs.buildEnv {
          name = "ci-packages";
          paths = with pkgs; [
            just

            rustup
            pkg-config

            yq
          ];
        };
        cli = pkgs.callPackage ./nix/packages/cli.nix { };
      };

      devShells.${system}.default = pkgs.mkShell {
        name = "dev";
        buildInputs = with pkgs; [
          nixfmt-rfc-style
          nixd

          just

          rustup
          pkg-config

          yq
        ];

        shellHook = ''
          rustup default nightly
        '';
      };
    };
}
