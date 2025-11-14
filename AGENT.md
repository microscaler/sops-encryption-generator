# Agent Guide - SOPS Encryption Generator Action

This guide helps AI agents understand how to work with this GitHub Action.

## 🎯 Quick Start

1. **Check GitHub Issues First** - Always review open issues before starting work
2. **This is a GitHub Action** - Built with Rust, runs in GitHub Actions workflows
3. **Handles SOPS Encryption** - Re-encrypts SOPS files with updated GPG keys
4. **Update Issues** - Keep issues updated with progress and findings

## 📋 Working with Issues

### Finding Relevant Issues

```bash
# List all open issues for this action
gh issue list --repo microscaler/sops-encryption-generator

# Search for specific issues
gh issue list --repo microscaler/sops-encryption-generator --label "bug"
gh issue list --repo microscaler/sops-encryption-generator --label "enhancement"
```

### Before Starting Work

1. **Check Open Issues** - Review GitHub issues before starting
2. **Understand the Action** - Read `README.md` for usage and purpose
3. **Review Recent Changes** - Check git log and recent PRs
4. **Understand SOPS** - Know how SOPS encryption works

## 🛠️ Development Workflow

### 1. Create or Assign Issue

```bash
# Create new issue
gh issue create \
  --repo microscaler/sops-encryption-generator \
  --title "Issue title" \
  --body-file issue-body.md \
  --label "bug"

# Assign issue to yourself
gh issue edit <ISSUE_NUMBER> \
  --repo microscaler/sops-encryption-generator \
  --add-assignee @me
```

### 2. Create Branch

```bash
git checkout -b fix/issue-<NUMBER>-short-description
```

### 3. Make Changes

- Follow Rust best practices
- Test locally with `cargo test`
- Build with `cargo build --release`
- Test Docker build: `docker build -t sops-encryption-generator .`
- **Never commit GPG keys** - They're gitignored for a reason!

### 4. Update Issue

```bash
gh issue comment <ISSUE_NUMBER> \
  --repo microscaler/sops-encryption-generator \
  --body "Progress update: Implemented X, testing Y"
```

### 5. Create PR

```bash
gh pr create \
  --repo microscaler/sops-encryption-generator \
  --title "Fix: Issue title" \
  --body "Fixes #<ISSUE_NUMBER>"
```

## 📚 Key Files

- `README.md` - Usage documentation
- `action.yml` - GitHub Action definition
- `src/main.rs` - Main Rust code
- `Dockerfile` - Container build (includes SOPS and GPG)
- `Cargo.toml` - Rust dependencies

## 🚨 Important Rules

1. **Always check issues first** - Don't duplicate work
2. **Test locally** - Use `cargo test` and `cargo run`
3. **Build Docker image** - Verify Docker build works
4. **Never commit GPG keys** - They're in .gitignore
5. **Update documentation** - Keep README.md current
6. **Follow Rust conventions** - Use `cargo fmt` and `cargo clippy`
7. **Understand SOPS** - Know how SOPS encryption/decryption works

## 🔗 Related Resources

- **Repository:** https://github.com/microscaler/sops-encryption-generator
- **GitHub Actions Docs:** https://docs.github.com/en/actions
- **SOPS Docs:** https://github.com/getsops/sops
- **Rust Docs:** https://doc.rust-lang.org/

## ✅ Checklist Before PR

- [ ] Issue created or assigned
- [ ] Branch created from main
- [ ] Changes made and tested
- [ ] `cargo test` passes
- [ ] `cargo build --release` succeeds
- [ ] Docker build works
- [ ] No GPG keys committed
- [ ] Documentation updated
- [ ] Issue updated with progress
- [ ] PR created with issue reference

---

**Remember:** Always check issues first, update them as you work, and link PRs to issues!

