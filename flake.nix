{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    nix-skyline-rs.url = "github:Naxdy/nix-skyline-rs";
    nix-skyline-rs.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nix-skyline-rs,
      nixpkgs,
      ...
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            inherit system;
          }
        );
    in
    {
      packages = forEachSupportedSystem (
        { system, ... }:
        {
          default = self.packages.${system}.less-delay;

          less-delay =
            let
              cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
            in
            nix-skyline-rs.lib.${system}.mkNroPackage {
              pname = cargoTOML.package.name;
              version = cargoTOML.package.version;
              src = self;
            };
        }
      );

      devShells = forEachSupportedSystem (
        { system, ... }:
        {
          inherit (nix-skyline-rs.devShells.${system}) default;
        }
      );
    };
}
