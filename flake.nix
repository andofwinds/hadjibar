{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { ... } @ inputs:
		with inputs;
		flake-utils.lib.eachDefaultSystem ( system:
			let
				overlays = [ (import rust-overlay) ];
				pkgs = import nixpkgs { inherit system overlays; };
				
				rustVer = pkgs.rust-bin.stable.latest.default;
				rust = pkgs.makeRustPlatform {
					cargo = rustVer;
					rustc = rustVer;
				};
				rustBuild = rust.buildRustPackage {
					pname = "hadjibar";
					version = "0.1.0";
					src = pkgs.lib.cleanSource ./.;
					cargoLock.lockFile = ./Cargo.lock;
					nativeBuildInputs = with pkgs; [
						
						pkg-config
					];
					buildInputs = with pkgs; [
						pkg-config
						gtk4-layer-shell
						gtk4.dev
						glib
						pango
					];
				};

			in {
				defaultPackage = rustBuild;

				devShells.default = with pkgs; mkShell {
					buildInputs = [
						pkg-config
						gtk4-layer-shell
						gtk4.dev
						glib
						pango
					];
				};
			}
		);
}
