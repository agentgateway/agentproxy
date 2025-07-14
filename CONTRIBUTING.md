# Contributing to AgentGateway

Thank you for your interest in contributing to AgentGateway! This guide will help you contribute safely and effectively while maintaining security best practices.

## 🚀 Quick Start

1. **Set up your local environment securely:**
   ```bash
   git clone https://github.com/WingedCommerce-LLC/agentgateway.git
   cd agentgateway
   ./scripts/setup-local-dev.sh
   ```

2. **Read the security guidelines:**
   - Review [SECURITY.md](SECURITY.md) thoroughly
   - Understand what information should never be committed
   - Set up pre-commit hooks for protection

3. **Create your feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🛡️ Security-First Development

### Before You Start
- [ ] Read [SECURITY.md](SECURITY.md) completely
- [ ] Run `./scripts/setup-local-dev.sh` to configure security
- [ ] Verify pre-commit hooks are working
- [ ] Understand the template system for configurations

### During Development
- [ ] Use environment variables for local settings
- [ ] Never commit personal information
- [ ] Use `.vscode-templates/` for IDE configuration
- [ ] Test with default environment variables

### Before Submitting
- [ ] Run security checks: `git add . && git commit -m "test"`
- [ ] Verify no local paths in code
- [ ] Test that others can reproduce your setup
- [ ] Update documentation if needed

## 📋 Development Workflow

### 1. Environment Setup
```bash
# Copy and customize environment
cp .env.example .env
vim .env  # Set your local values

# Set up VSCode (if using)
cp .vscode-templates/*.template .vscode/
cd .vscode && for f in *.template; do mv "$f" "${f%.template}"; done

# Customize local configuration
cp manifests/localhost-config.example.json manifests/localhost-config.json
vim manifests/localhost-config.json
```

### 2. Making Changes
```bash
# Create feature branch
git checkout -b feature/amazing-feature

# Make your changes
# ... code, test, iterate ...

# Commit with security validation
git add .
git commit -m "feat: add amazing feature"
```

### 3. Testing
```bash
# Test Rust components
cargo test
cargo clippy

# Test UI components (if applicable)
cd ui
npm test
npm run build

# Test with clean environment
unset PYTHON_INTERPRETER
unset AGENTGATEWAY_PORT
# Verify defaults work
```

### 4. Submitting
```bash
# Push to your fork
git push origin feature/amazing-feature

# Create pull request with security checklist
```

## 🔍 Code Review Guidelines

### For Contributors
- **Self-review first:** Check your changes for sensitive information
- **Test portability:** Ensure others can use your changes
- **Document changes:** Update relevant documentation
- **Follow patterns:** Use established security patterns

### For Reviewers
- **Security scan:** Look for personal information or local paths
- **Configuration check:** Verify environment variables are used
- **Template compliance:** Ensure templates are used correctly
- **Documentation review:** Check that security docs are updated

## 🚫 Common Security Mistakes

### ❌ What NOT to Do
```bash
# Don't commit personal information
git add .vscode/settings.json  # Contains local paths
git commit -m "Fix by mkostreba@wingedcommerce.com"

# Don't hardcode local paths
"python_path": "/home/mkostreba/.venv/bin/python"
"config_file": "/Users/john/project/config.json"

# Don't commit environment files
git add .env
git add localhost-config.json
```

### ✅ What TO Do
```bash
# Use templates and environment variables
git add .vscode-templates/settings.json.template
git commit -m "feat: improve Python integration"

# Use portable paths
"python_path": "${env:PYTHON_INTERPRETER:python3}"
"config_file": "${workspaceFolder}/config/default.json"

# Use example files
git add .env.example
git add manifests/localhost-config.example.json
```

## 🧪 Testing Guidelines

### Local Testing
```bash
# Test with your environment
cargo test
npm test

# Test with clean environment
env -i PATH="$PATH" cargo test
env -i PATH="$PATH" npm test

# Test setup script
rm -rf .vscode .env manifests/localhost-config.json
./scripts/setup-local-dev.sh
```

### Security Testing
```bash
# Test pre-commit hook
echo "mkostreba" > test-file.txt
git add test-file.txt
git commit -m "test"  # Should fail

# Test for sensitive patterns
grep -r "wingedcommerce\|mkostreba" . --exclude-dir=.git
```

## 📝 Documentation Standards

### Code Documentation
- Document security considerations
- Explain environment variable usage
- Provide setup examples
- Include troubleshooting tips

### Security Documentation
- Update SECURITY.md for new patterns
- Document new environment variables in .env.example
- Update templates when adding new configurations
- Explain security implications of changes

## 🔄 Upstream Sync Process

### Before Syncing
1. **Clean your branches:**
   ```bash
   git checkout main
   git pull upstream main
   git push origin main
   ```

2. **Security validation:**
   ```bash
   # Check for sensitive information
   grep -r "mkostreba\|wingedcommerce" . --exclude-dir=.git
   
   # Verify no local configs
   git status --porcelain | grep -E "\.(local|env)$"
   ```

3. **Test portability:**
   ```bash
   # Ensure others can use your changes
   ./scripts/setup-local-dev.sh
   cargo build
   ```

### During Sync
- Coordinate with team for any breaking changes
- Test that upstream changes work with local setup
- Update security patterns if needed

## 🐛 Issue Reporting

### Security Issues
- **Never include sensitive information** in issue reports
- Use generic examples instead of real data
- Report security vulnerabilities privately first
- Follow responsible disclosure practices

### Bug Reports
```markdown
## Bug Description
Clear description without personal information

## Environment
- OS: Linux/macOS/Windows
- Rust version: 1.70+
- Python version: 3.10+

## Steps to Reproduce
1. Use generic setup steps
2. Avoid local paths
3. Use environment variables

## Expected vs Actual
What should happen vs what actually happens
```

## 🎯 Feature Requests

### Security Considerations
- Consider security implications of new features
- Propose secure defaults
- Think about configuration management
- Consider impact on templates and examples

### Implementation Guidelines
- Use environment variables for configuration
- Provide secure examples
- Update security documentation
- Test with clean environment

## 🤝 Community Guidelines

### Communication
- Be respectful and inclusive
- Help others with security best practices
- Share knowledge about secure development
- Provide constructive feedback

### Collaboration
- Review others' code for security issues
- Share security tips and tricks
- Help improve documentation
- Contribute to security tooling

## 📚 Resources

### Security
- [SECURITY.md](SECURITY.md) - Comprehensive security guidelines
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Git Security Best Practices](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History)

### Development
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [VSCode Rust Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Project Specific
- [README.md](README.md) - Project overview
- [.vscode-templates/README.md](.vscode-templates/README.md) - VSCode setup
- [.env.example](.env.example) - Environment configuration

## 🆘 Getting Help

### Security Questions
- Review [SECURITY.md](SECURITY.md) first
- Ask in discussions without sensitive information
- Contact maintainers privately for security issues

### Development Questions
- Check existing issues and discussions
- Provide minimal reproducible examples
- Use generic paths and configurations
- Be specific about your environment

### Setup Issues
- Run `./scripts/setup-local-dev.sh` first
- Check that templates are properly copied
- Verify environment variables are set
- Test with clean environment

---

**Remember:** Security is everyone's responsibility. When in doubt, ask for help rather than risk exposing sensitive information.

Thank you for contributing securely! 🚀
