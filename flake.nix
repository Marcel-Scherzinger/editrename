{
    description = "Rename files by editing a file";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        flake-parts.url = "github:hercules-ci/flake-parts";
    };
    outputs = inputs @ {flake-parts, ...}:
        flake-parts.lib.mkFlake {inherit inputs;} {
            imports = [];
            systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];
            perSystem = {
                # config,
                self',
                # inputs',
                pkgs,
                # system,
                ...
            }: {
                # Per-system attributes can be defined here. The self' and inputs'
                # module parameters provide easy access to attributes of the same
                # system.

                # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;

                formatter = pkgs.alejandra;

                packages.editrename = pkgs.rustPlatform.buildRustPackage {
                    name = "editrename";
                    src = ./.;
                    buildInputs = [];
                    nativeBuildInputs = [];
                    cargoHash = "sha256-5liuMc4gRHF+iQ5ucXrzfC4c9bqtSXw+weYIpIwa2hI=";
                };
                packages.default = self'.packages.editrename;
            };
            flake = {
                # The usual flake attributes can be defined here, including system-
                # agnostic ones like nixosModule and system-enumerating ones, although
                # those are more easily expressed in perSystem.
            };
        };
}
