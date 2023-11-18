{
  description =
    "Modelfarm-Proxy: A reverse proxy for the Modelfarm API. Allows you to use Modelfarm on local (useful until Replit supports Rust)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flakebox = {
      url = "github:rustshop/flakebox";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        flakeboxLib = flakebox.lib.${system} { };
        rustSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = "modelfarm-proxy";
            path = ./.;
          };
          paths = [ "Cargo.toml" "Cargo.lock" ".cargo" "src" ];
        };

        outputs = (flakeboxLib.craneMultiBuild { }) (craneLib':
          let
            craneLib = (craneLib'.overrideArgs {
              pname = "flexbox-multibuild";
              src = rustSrc;
            });
          in rec {
            workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
            workspaceBuild =
              craneLib.buildWorkspace { cargoArtifacts = workspaceDeps; };
            modelfarm-proxy = craneLib.buildPackage { };
          });
      in {
        devShells = flakeboxLib.mkShells {
          packages = [ ];
          buildInputs = [ nixpkgs.starship ];
          shellHook = ''
            eval "$(starship init bash)"
          '';
        };
      });
}
