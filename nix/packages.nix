{ inputs, ...}:
{
  perSystem = { pkgs, lib, ... }:
    let
      inherit (pkgs) buildNpmPackage;
      inherit (lib) cleanSourceWith optionals hasSuffix hasInfix;

      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      node_modules = buildNpmPackage {
        name = "frontend-deps";
        src = cleanSourceWith {
          src = ../frontend;
          filter = path: _type:
            (builtins.match ".*/frontend/package.*json" path) != null;
        };

        dontNpmBuild = true;

        npmDepsHash = "sha256-zOjFEOkkM5ReVGud1660UdrDq6f+PaUuELyIHRYAt2k=";
        # npmDepsHash = lib.fakeHash;
      };

      src = cleanSourceWith {
        src = ../.;
        filter = path: type:
          (hasSuffix "\.html" path)  ||
          (hasSuffix "\.scss" path)  ||
          (hasInfix "/assets/" path) ||
          (craneLib.filterCargoSources path type);
      };

      commonArgs = {
        inherit src;
        strictDeps = true;

        buildInputs = [
          # Add additional build inputs here
        ] ++ optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.libiconv
        ];
      };

      nativeArgs = commonArgs // {
        
      };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

      server = craneLib.buildPackage (nativeArgs // {
        inherit cargoArtifacts;

        FRONTEND_DIST = frontend;
      });

      wasmArgs = commonArgs // {
        pname = "homelab-server-manager-wasm";
        cargoExtraArgs = "--package=frontend";
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
      };

      cargoArtifactsWasm = craneLib.buildDepsOnly wasmArgs;

      frontend = craneLib.buildTrunkPackage (wasmArgs // {
        pname = "homelab-server-manager-frontend";
        cargoArtifacts = cargoArtifactsWasm;
        trunkIndexPath = "frontend/index.html";

        # The version of wasm-bindgen-cli here must match the one from Cargo.lock.
        wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
          version = "0.2.93";
          hash = "sha256-DDdu5mM3gneraM85pAepBXWn3TMofarVR4NbjMdz3r0=";
          cargoHash = "sha256-birrg+XABBHHKJxfTKAMSlmTVYLmnmqMDfRnmG6g/YQ=";
        };

        postPatch = ''
          mkdir -p ./frontend/node_modules
          cp -r ${node_modules}/lib/node_modules/frontend/node_modules/* ./frontend/node_modules/
        '';
      });

    in {
      packages = {
        default = server;
      };
    };
}