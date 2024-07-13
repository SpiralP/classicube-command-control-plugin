{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
  };

  outputs = { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;

      makePackages = (system: dev:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
          rustManifest = lib.importTOML ./Cargo.toml;

          defaultAttrs = {
            pname = rustManifest.package.name;
            version = rustManifest.package.version;

            src = lib.sourceByRegex ./. [
              "^\.cargo(/.*)?$"
              "^build\.rs$"
              "^Cargo\.(lock|toml)$"
              "^src(/.*)?$"
            ];

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = { };
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
              rustPlatform.bindgenHook
            ] ++ (if dev then
              with pkgs; ([
                cargo-release
                clippy
                (rustfmt.override { asNightly = true; })
                rust-analyzer
              ]) else [ ]);
          };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage defaultAttrs;

          debug = (pkgs.enableDebugging {
            inherit (pkgs) stdenv;
            override = (attrs: pkgs.makeRustPlatform ({
              inherit (pkgs) rustc cargo;
            } // attrs));
          }).buildRustPackage (
            (defaultAttrs // {
              pname = "${defaultAttrs.pname}-debug";

              buildType = "debug";

              hardeningDisable = [ "all" ];
            })
          );
        }
      );
    in
    builtins.foldl' lib.recursiveUpdate { } (builtins.map
      (system: {
        devShells.${system} = makePackages system true;
        packages.${system} = makePackages system false;
      })
      lib.systems.flakeExposed);
}
