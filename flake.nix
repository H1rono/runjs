{
  description = "Roll my own javascript runtime";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, ... }:
    let
      # key: https://github.com/numtide/flake-utils?tab=readme-ov-file#defaultsystems--system
      # value: https://github.com/denoland/rusty_v8/releases/tag/v0.83.1
      rusty-v8-archive = {
        x86_64-linux = {
          url = "https://github.com/denoland/rusty_v8/releases/download/v0.83.1/librusty_v8_release_x86_64-unknown-linux-gnu.a";
          hash = "sha256-0cCpFMPpFWTvoU3+HThYDDTQO7DdpdVDDer5k+3HQFY=";
        };
        aarch64-linux = {
          url = "https://github.com/denoland/rusty_v8/releases/download/v0.83.1/librusty_v8_release_aarch64-unknown-linux-gnu.a";
          hash = "sha256-fOyJiD0raHxl+5tDWSpH/MbdBUqNY+HCKmTulYLXEYI=";
        };
        x86_64-darwin = {
          url = "https://github.com/denoland/rusty_v8/releases/download/v0.83.1/librusty_v8_release_x86_64-apple-darwin.a";
          hash = "sha256-JwZ1FrU/MZeEnvSPDojvDdDxIF/bdZBPRCXrjbBb7WM=";
        };
        aarch64-darwin = {
          url = "https://github.com/denoland/rusty_v8/releases/download/v0.83.1/librusty_v8_release_aarch64-apple-darwin.a";
          hash = "sha256-ajmr+SGj3L8TT+17NPkNcwQFESpIZuUul12Pp1oJAkY=";
        };
      };
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        inherit (pkgs) lib;

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        jsFileFilter = path: _type: builtins.match ".*(ts|js)$" path != null;
        src = lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type: (jsFileFilter path type) || (craneLib.filterCargoSources path type);
        };

        v8Archive = pkgs.fetchurl rusty-v8-archive.${system};

        commonArgs = {
          stdenv = pkgs.clangStdenv;
          inherit src;
          strictDeps = true;
          # Common arguments can be set here to avoid repeating them later
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            # Add additional build inputs here
            openssl
          ] ++ lib.optionals stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            libiconv
            darwin.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          # Additional environment variables can be set directly
          RUSTY_V8_ARCHIVE = "${v8Archive}";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        build = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit build;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          fmt = craneLib.cargoFmt commonArgs;
        };
        packages.default = craneLib.buildPackage commonArgs;
        pakcages.withoutDeps = craneLib.buildPackage commonArgs;

        apps.default = flake-utils.lib.mkApp {
          drv = build;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            # pkgs.ripgrep
          ];
        };
      }
    );
}
