# SOPS Encryption Generator

GitHub Action to re-encrypt SOPS-encrypted files with updated GPG public keys.

## Usage

### Using the Published Action

```yaml
- name: Re-encrypt secrets
  uses: microscaler/sops-encryption-generator@v1
  with:
    private_key: ${{ secrets.GPG_KEY }}
    public_keys: ${{ steps.users.outputs.data }}
    flux_key: ${{ secrets.FLUX_GPG_PUBLIC_KEY }}
    secrets_pattern: '**/application.secrets.env'
    sops_version: '3.10.2'
```

### Using a Specific Version

```yaml
- name: Re-encrypt secrets
  uses: microscaler/sops-encryption-generator@v1.0.0
  with:
    private_key: ${{ secrets.GPG_KEY }}
    public_keys: ${{ steps.users.outputs.data }}
    flux_key: ${{ secrets.FLUX_GPG_PUBLIC_KEY }}
    secrets_pattern: '**/application.secrets.env'
    sops_version: '3.10.2'
```

### Using the Latest from Main Branch

```yaml
- name: Re-encrypt secrets
  uses: microscaler/sops-encryption-generator@main
  with:
    private_key: ${{ secrets.GPG_KEY }}
    public_keys: ${{ steps.users.outputs.data }}
    flux_key: ${{ secrets.FLUX_GPG_PUBLIC_KEY }}
    secrets_pattern: '**/application.secrets.env'
    sops_version: '3.10.2'
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `private_key` | Base64-encoded GPG private key | Yes | - |
| `public_keys` | JSON output from get-users-with-access-on-repo | No | `{"users":[]}` |
| `flux_key` | Flux GPG public key (base64-encoded) | No | - |
| `secrets_pattern` | Glob pattern for secret files | No | `**/application.secrets.env` |
| `sops_version` | SOPS version to use | No | `3.10.2` |

## How It Works

1. Imports the GPG private key
2. Collects all public keys from `public_keys` JSON and `flux_key`
3. Imports all public keys into GPG keyring
4. Finds all secret files matching the pattern
5. Re-encrypts each file with all keys in the keyring
6. Files are updated in-place

## Development

### Building Locally

```bash
# Build the Rust binary
cargo build --release

# Build Docker image
docker build -t sops-encryption-generator .
```

### Testing Locally

```bash
export INPUT_PRIVATE_KEY="base64-encoded-key"
export INPUT_PUBLIC_KEYS='{"users":[]}'
export INPUT_FLUX_KEY="base64-encoded-key"
export INPUT_SECRETS_PATTERN="**/application.secrets.env"
export INPUT_SOPS_VERSION="3.10.2"
cargo run
```

## Publishing

This action is automatically published to GitHub Container Registry (ghcr.io) when:
- Code is pushed to `main` or `master` branch
- A new tag starting with `v` is created (e.g., `v1.0.0`)

The published image is available at:
- `ghcr.io/microscaler/sops-encryption-generator:latest`
- `ghcr.io/microscaler/sops-encryption-generator:v1.0.0` (for tagged releases)
- `ghcr.io/microscaler/sops-encryption-generator:main` (for main branch)

## Implementation

Built with Rust. Requires SOPS and GPG to be installed (handled in Dockerfile).
