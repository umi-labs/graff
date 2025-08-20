# Release Guide

This guide explains how to use the release automation system for Graff.

## Prerequisites

Before you can release, make sure you have the following installed:

```bash
# Basic requirements
cargo --version
git --version
```

## Quick Release

To perform a complete release with automatic version bumping:

```bash
# Bump patch version (0.1.0 -> 0.1.1)
./scripts/release.sh --patch

# Bump minor version (0.1.0 -> 0.2.0)
./scripts/release.sh --minor

# Bump major version (0.1.0 -> 1.0.0)
./scripts/release.sh --major

# Or specify exact version
./scripts/release.sh 1.0.0
```

This will:
1. ✅ Auto-bump version (if using --patch/--minor/--major)
2. ✅ Run all tests
3. ✅ Check code formatting and lints
4. ✅ Build the release version
5. ✅ Update version in Cargo.toml
6. ✅ Commit the version change
7. ✅ Create a git tag
8. ✅ Push everything to GitHub

## Advanced Usage

### Dry Run

Test what the release process would do without making changes:

```bash
./scripts/release.sh --dry-run 1.0.0
```

### Skip Steps

You can skip certain steps if needed:

```bash
# Skip tests (not recommended for production releases)
./scripts/release.sh --patch --skip-tests

# Skip lints
./scripts/release.sh --minor --skip-lints

# Skip build
./scripts/release.sh --major --skip-build

# Skip cargo dist build (not needed)
./scripts/release.sh --patch --skip-dist

# Skip multiple steps
./scripts/release.sh --minor --skip-tests --skip-lints
```

### Version Formats

The script supports both automatic version bumping and manual version specification:

```bash
# Automatic version bumping
./scripts/release.sh --patch          # 0.1.0 -> 0.1.1
./scripts/release.sh --minor          # 0.1.0 -> 0.2.0
./scripts/release.sh --major          # 0.1.0 -> 1.0.0

# Manual version specification
./scripts/release.sh 1.0.0            # Standard release
./scripts/release.sh 1.0.1            # Patch release
./scripts/release.sh 1.1.0            # Minor release
./scripts/release.sh 2.0.0            # Major release
./scripts/release.sh 1.0.0-beta.1     # Pre-release
./scripts/release.sh 1.0.0-rc.1       # Release candidate
```

## Configuration

### Cargo Dist Configuration

The `Cargo.dist.toml` file configures how cargo dist builds and distributes your project. Key settings:

- **Targets**: Supported platforms (Linux, macOS, Windows)
- **Artifacts**: What gets included in the distribution
- **GitHub**: Release settings and templates
- **Changelog**: Automatic changelog generation

### Customizing the Configuration

Edit `Cargo.dist.toml` to customize:

1. **Repository URLs**: Update `repository`, `homepage`, etc.
2. **Authors**: Add your name and email
3. **Targets**: Add/remove supported platforms
4. **Release Notes**: Customize the release template
5. **Changelog**: Configure changelog sections

## Release Process

### 1. Pre-Release Checklist

Before releasing, ensure:

- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] No lint warnings (`cargo clippy --release -- -D warnings`)
- [ ] Documentation is up-to-date
- [ ] Changelog is updated
- [ ] You're on the prod branch
- [ ] No uncommitted changes

### 2. Release Steps

1. **Run the release script**:
   ```bash
   # For automatic version bumping
   ./scripts/release.sh --patch
   ./scripts/release.sh --minor
   ./scripts/release.sh --major
   
   # Or for specific version
   ./scripts/release.sh 1.0.0
   ```

2. **Monitor the process**:
   - The script will show progress for each step
   - Any failures will stop the process with clear error messages

3. **Verify the release**:
   - Check GitHub for the new tag
   - Verify the release assets are uploaded
   - Test the distributed binaries

### 3. Post-Release

After a successful release:

- [ ] Update any external documentation
- [ ] Announce the release (if applicable)
- [ ] Monitor for any issues
- [ ] Plan the next release

## Troubleshooting

### Common Issues

**"cargo-dist is not installed"**
```bash
cargo install cargo-dist
```

**"You have uncommitted changes"**
```bash
git add .
git commit -m "Your commit message"
```

**"Tag already exists"**
```bash
# Check existing tags
git tag -l

# Delete the tag if needed (be careful!)
git tag -d v1.0.0
git push origin :refs/tags/v1.0.0
```

**"Failed to push to GitHub"**
- Check your GitHub credentials
- Ensure you have write access to the repository
- Verify your SSH keys or tokens are configured

### Getting Help

If you encounter issues:

1. **Check the logs**: The script provides detailed error messages
2. **Use dry-run**: Test with `--dry-run` to see what would happen
3. **Check prerequisites**: Ensure all tools are installed
4. **Review configuration**: Verify `Cargo.dist.toml` settings

## Automation

### GitHub Actions

The release process can be automated with GitHub Actions. The workflow would:

1. Trigger on tag creation
2. Run tests and builds
3. Create GitHub releases
4. Upload distribution assets

### CI/CD Integration

For continuous deployment:

1. **Development**: Use feature branches and pull requests
2. **Testing**: Automated tests on every commit
3. **Release**: Manual trigger with the release script on prod branch
4. **Deployment**: Manual deployment from git tags

## Best Practices

### Version Management

- Use semantic versioning consistently
- Update the changelog for every release
- Tag releases immediately after creation
- Never modify released tags

### Quality Assurance

- Always run tests before releasing
- Use dry-run mode for testing
- Review changes before committing
- Test the distributed binaries

### Communication

- Update documentation with new features
- Announce breaking changes clearly
- Provide migration guides when needed
- Respond to issues promptly

## Examples

### Minor Release

```bash
# Update changelog first
# Then release with automatic minor bump
./scripts/release.sh --minor
```

### Hotfix Release

```bash
# Quick fix release with patch bump
./scripts/release.sh --patch --skip-tests
```

### Major Release

```bash
# Major version bump
./scripts/release.sh --major
```

### Pre-release

```bash
# Beta release (manual version)
./scripts/release.sh 1.0.0-beta.1
```

### Development Release

```bash
# Quick development release
./scripts/release.sh --patch --skip-tests --skip-lints
```
