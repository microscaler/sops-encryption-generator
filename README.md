# SOPS Encryption Generator

GitHub Action to re-encrypt SOPS-encrypted files with updated GPG public keys.

## Usage

```yaml
- name: Re-encrypt secrets
  uses: ./hack/actions/sops-encryption-generator
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

## Implementation

Built with Rust. Requires SOPS and GPG to be installed (handled in Dockerfile).

## Future

This action will be moved to a separate repository:
- `github.com/microscaler/sops-encryption-generator`

