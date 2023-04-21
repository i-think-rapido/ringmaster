let
    pkgs = import <nixpkgs> {};
in
    # Configure the dependency of your shell
    # Add support for clang for bindgen in godot-rust
    pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
        buildInputs = with pkgs; [
            # Rust related dependencies
            rustup
            rust-analyzer
        ];
    }
