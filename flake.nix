{
  inputs = {
    crane.url = "github:ipetkov/crane";
    utils.url = "github:numtide/flake-utils";
    nixpkgs.follows = "crane/nixpkgs";
  };

  outputs = {
    self,
    crane,
    utils,
    nixpkgs,
    ...
  }:
    let
      supportedSystems = ["x86_64-linux" "x86_64-darwin" "aarch64-linux"];
    in
    utils.lib.eachSystem supportedSystems
    (
      system:
        {
          packages.oura = crane.lib.${system}.buildPackage {
            src = self;
            cargoExtraArgs = "--features kafkasink";
            nativeBuildInputs = with import nixpkgs { inherit system; }; [ perl ];
          };
          packages.default = self.packages.${system}.oura;
        }
    );
}
