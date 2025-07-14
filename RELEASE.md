# Release Process

This document describes the automated release process for AgentGateway.

## Automated Releases

The project uses GitHub Actions to automatically create releases when all tests pass. There are two ways to trigger a release:

### 1. Automatic Beta Releases

Beta releases are automatically created when code is pushed to the `main` branch, provided all tests pass.

- **Trigger**: Push to `main` branch
- **Version**: Increments patch version with beta suffix (e.g., `0.4.13-beta.20250714141500`)
- **Process**: 
  1. Runs comprehensive test suite (tests, lint, build, code generation check, docker build)
  2. Creates git tag and GitHub release
  3. Triggers binary and Docker image builds
  4. Updates release with generated notes

### 2. Manual Releases

You can manually trigger releases with different version types:

1. Go to the [Actions tab](../../actions/workflows/automated-release.yml)
2. Click "Run workflow"
3. Select the release type:
   - **beta**: Patch increment with beta suffix
   - **patch**: Bug fixes (0.4.12 → 0.4.13)
   - **minor**: New features (0.4.12 → 0.5.0)
   - **major**: Breaking changes (0.4.12 → 1.0.0)

## Release Workflow

The automated release process follows these steps:

### 1. Test Suite (`test-suite` job)
- Runs `make test` - All unit and integration tests
- Runs `make lint` - Code quality checks

### 2. Multi-Platform Build Test (`build-test` job)
- Tests builds on:
  - Ubuntu (x86_64-unknown-linux-musl)
  - Ubuntu ARM (aarch64-unknown-linux-musl)  
  - macOS (aarch64-apple-darwin)
- Builds UI components
- Verifies all targets compile successfully

### 3. Code Generation Check (`check-code-gen` job)
- Runs `make generate-apis`
- Ensures generated code is up to date

### 4. Docker Build Test (`docker-test` job)
- Tests Docker image builds
- Verifies containerization works

### 5. Release Creation (`create-release` job)
- Calculates new version number
- Creates git tag
- Generates release notes from commit history

### 6. Binary and Image Build (`trigger-release` job)
- Triggers the existing `release.yml` workflow
- Builds binaries for all platforms
- Creates Docker images for multiple architectures
- Uploads release assets

### 7. Release Finalization (`update-release` job)
- Updates GitHub release with detailed notes
- Includes installation instructions
- Links to documentation

## Release Assets

Each release includes:

### Binaries
- `agentgateway-linux-amd64` - Linux x86_64
- `agentgateway-linux-arm64` - Linux ARM64
- `agentgateway-darwin-arm64` - macOS ARM64
- SHA256 checksums for all binaries

### Docker Images
- `ghcr.io/agentgateway/agentgateway:VERSION` - Main image
- `ghcr.io/agentgateway/agentgateway:latest-ext` - Extended image
- Multi-architecture support (amd64, arm64)

## Version Numbering

The project follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)
- **BETA**: Pre-release versions with timestamp suffix

Current version is defined in `Cargo.toml` and automatically incremented.

## Release Notes

Release notes are automatically generated and include:

- List of commits since the last release
- Installation instructions
- Docker usage examples
- Links to documentation
- SHA256 checksums for verification

## Prerequisites

For releases to work properly:

1. **All tests must pass** - The workflow will fail if any test fails
2. **Code generation must be current** - Run `make generate-apis` if needed
3. **Docker build must succeed** - Containerization must work
4. **Multi-platform builds must work** - All target platforms must compile

## Troubleshooting

### Release Failed
1. Check the [Actions tab](../../actions) for detailed error logs
2. Common issues:
   - Test failures: Fix failing tests and push again
   - Build failures: Check platform-specific build issues
   - Docker issues: Verify Dockerfile and dependencies

### Missing Release Assets
- The release workflow may still be running
- Check the `release.yml` workflow status
- Assets are uploaded after the main workflow completes

### Version Issues
- Versions are calculated from `Cargo.toml`
- Beta versions include timestamp to ensure uniqueness
- Manual releases allow choosing the increment type

## Manual Release Process (Fallback)

If the automated process fails, you can create releases manually:

1. Update version in `Cargo.toml`
2. Create and push a git tag: `git tag v0.4.13 && git push origin v0.4.13`
3. The existing `release.yml` workflow will trigger automatically
4. Manually create GitHub release with generated binaries

## Security

- Release workflows run with minimal required permissions
- Only `contents: write` for creating releases and tags
- Docker images are signed and pushed to GitHub Container Registry
- All binaries include SHA256 checksums for verification

## Monitoring

Monitor release health through:

- [GitHub Actions](../../actions) - Workflow status
- [Releases page](../../releases) - Published releases
- [Container Registry](../../pkgs/container/agentgateway) - Docker images

---

For questions about the release process, please open an issue or check the workflow files in `.github/workflows/`.
