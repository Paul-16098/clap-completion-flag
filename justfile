[windows]
set shell := ["pwsh", "-c"]

export RUST_LOG := 'debug'

# Release version
[arg('version', pattern='^\d+\.\d+\.\d+|$', help="version to release, e.g., 1.0.0")]
[confirm("Are you sure you want to release version " + version + " ?")]
[script('nu')]
release version:
    # Get the current version from Cargo.toml
    open ./Cargo.toml |{{ if version != "" { ' update package.version ' + version + ' |' } else { '' } }} save ./Cargo.toml --force

    # Fetch latest dependencies
    cargo fetch

    # Stage and commit changes
    git add Cargo.toml Cargo.lock
    git commit -m $"chore\(release): bump version to {{ version }}"
    git tag -a v{{ version }} -m $"Release version {{ version }}"

    git push --tags
