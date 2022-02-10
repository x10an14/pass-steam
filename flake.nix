{
  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{
    self
    , utils
    , naersk
    , nixpkgs
  }: utils.lib.eachDefaultSystem (system:
    let
      nixpkgs = import inputs.nixpkgs {inherit system;};
      naersk-lib = nixpkgs.callPackage inputs.naersk {};
    in {
      defaultPackage = naersk-lib.buildPackage ./.;

      defaultApp = utils.lib.mkApp {
        drv = self.defaultPackage."${system}";
      };

      devShell = with nixpkgs; mkShell {
        buildInputs = [
          cargo
          cargo-edit
          rustc
          rustfmt
          rustPackages.clippy
        ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
    });
}
