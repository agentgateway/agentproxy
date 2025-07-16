# Security Guidelines

This document outlines security practices and guidelines for the AgentGateway project to prevent sensitive information leakage and maintain secure development practices.

## 🚨 Critical Security Rules

### 1. **Never Commit Personal Information**
- Personal email addresses
- Local usernames or hostnames
- Company-specific internal URLs
- Local file paths (e.g., `/home/username/`)

### 2. **Never Commit Local Configuration**
- `.vscode/` directory files
- `.env` files (use `.env.example` instead)
- `localhost-config.json` (use `localhost-config.example.json`)
- Any files ending in `.local`

### 3. **Use Templates and Environment Variables**
- Use `.vscode-templates/` for VSCode configuration
- Use environment variables for local-specific settings
- Use example files for configuration templates

## 🛡️ Security Implementation

### Pre-commit Protection
A pre-commit hook (`.githooks/pre-commit`) automatically scans for:
- Personal information patterns
- Local file paths
- Hardcoded credentials
- VSCode configuration files
- Local configuration files

**Setup:**
```bash
# Run the setup script to configure git hooks
./scripts/setup-local-dev.sh

# Or manually configure
git config core.hooksPath .githooks
```

### Environment Variable System
Use environment variables for all local-specific configuration:

```bash
# Copy and customize
cp .env.example .env

# Set your local values
export PYTHON_INTERPRETER=/usr/local/bin/python3.12
export AGENTGATEWAY_PORT=8080
```

### VSCode Configuration
Use sanitized templates instead of committing actual VSCode files:

```bash
# Copy templates to local .vscode/
cp .vscode-templates/*.template .vscode/
cd .vscode/
mv settings.json.template settings.json
mv launch.json.template launch.json
mv tasks.json.template tasks.json
```

## 🔍 Security Patterns to Avoid

### Personal Information
```bash
# ❌ BAD - Personal information
"michael.kostreba@wingedcommerce.com"
"mkostreba"
"wc-p3-001"

# ✅ GOOD - Generic or environment variables
"${USER}@${COMPANY_DOMAIN}"
"${env:USERNAME}"
"${env:HOSTNAME}"
```

### Local Paths
```bash
# ❌ BAD - Hardcoded local paths
"/home/mkostreba/.venv/bin/python"
"C:\\Users\\mkostreba\\project"

# ✅ GOOD - Relative or environment-based paths
"${workspaceFolder}/.venv/bin/python"
"${env:PYTHON_INTERPRETER}"
"./.venv/bin/python"
```

### Configuration Files
```bash
# ❌ BAD - Committing local configs
git add .vscode/settings.json
git add .env
git add manifests/localhost-config.json

# ✅ GOOD - Using templates
git add .vscode-templates/settings.json.template
git add .env.example
git add manifests/localhost-config.example.json
```

## 🔧 Local Development Setup

### Initial Setup
1. **Run the setup script:**
   ```bash
   ./scripts/setup-local-dev.sh
   ```

2. **Customize your environment:**
   ```bash
   # Edit your local environment
   vim .env
   
   # Customize VSCode settings
   vim .vscode/settings.json
   
   # Customize local config
   vim manifests/localhost-config.json
   ```

3. **Verify security:**
   ```bash
   # Test pre-commit hook
   git add .
   git commit -m "Test commit"
   ```

### Daily Development Workflow
1. **Before committing:**
   - Review changes for sensitive information
   - Use `git diff --cached` to check staged files
   - Let pre-commit hook validate

2. **Before pushing to upstream:**
   - Double-check no local configs are included
   - Verify environment variables are used
   - Test that others can use your changes

## 🚫 Files Never to Commit

### IDE/Editor Files
- `.vscode/settings.json`
- `.vscode/launch.json`
- `.vscode/tasks.json`
- `.idea/`
- `*.swp`, `*.swo`

### Environment Files
- `.env`
- `.env.local`
- `.env.development.local`
- Any file ending in `.local`

### Local Configuration
- `manifests/localhost-config.json`
- `config.local.*`
- `*-local.json`
- `dev-config.json`

### Personal Files
- `notes.md`
- `TODO.md`
- `scratch/`
- `*.backup`

## 🔄 Upstream Sync Security

### Before Syncing with Upstream
1. **Clean local references:**
   ```bash
   # Check for sensitive patterns
   grep -r "mkostreba\|wingedcommerce\|wc-p3-001" . --exclude-dir=.git
   
   # Verify no local configs
   git status --porcelain | grep -E "\.(local|env)$"
   ```

2. **Validate branch state:**
   ```bash
   # Ensure clean working directory
   git status
   
   # Check what will be pushed
   git log origin/main..HEAD --oneline
   ```

3. **Test configuration portability:**
   ```bash
   # Verify others can use your changes
   unset PYTHON_INTERPRETER
   unset AGENTGATEWAY_PORT
   # Test that defaults work
   ```

## 🛠️ Git History Cleanup

If sensitive information has been committed, clean it up:

### Using BFG Repo-Cleaner (Recommended)
```bash
# Install BFG
# Download from: https://rtyley.github.io/bfg-repo-cleaner/

# Create backup
git branch backup-$(date +%Y%m%d)

# Clean sensitive data
java -jar bfg.jar --replace-text sensitive-patterns.txt
git reflog expire --expire=now --all && git gc --prune=now --aggressive

# Force push (coordinate with team)
git push --force-with-lease origin main
```

### Using git filter-branch
```bash
# Remove specific patterns
git filter-branch --tree-filter 'find . -name "*.json" -exec sed -i "s/mkostreba/USERNAME/g" {} \;' HEAD

# Remove files entirely
git filter-branch --index-filter 'git rm --cached --ignore-unmatch .vscode/settings.json' HEAD
```

## 📋 Security Checklist

### Before Each Commit
- [ ] No personal information in files
- [ ] No local file paths
- [ ] No hardcoded credentials
- [ ] Using environment variables
- [ ] Pre-commit hook passes

### Before Each Push
- [ ] Changes work with default environment
- [ ] No `.vscode/` files included
- [ ] No `.env` files included
- [ ] Configuration uses templates
- [ ] Others can reproduce setup

### Before Upstream Sync
- [ ] All local branches clean
- [ ] No company-specific references
- [ ] Documentation updated
- [ ] Security patterns followed
- [ ] Team coordination complete

## 🆘 Security Incident Response

### If Sensitive Data is Committed
1. **Immediate action:**
   ```bash
   # Don't push if not already pushed
   git reset --soft HEAD~1
   
   # If already pushed, coordinate with team
   # Plan git history rewrite
   ```

2. **Clean up:**
   - Use BFG or git filter-branch
   - Force push with team coordination
   - Update all local clones

3. **Prevention:**
   - Review why pre-commit hook didn't catch it
   - Update patterns if needed
   - Improve documentation

### If Local Config is Exposed
1. **Rotate any credentials**
2. **Update security patterns**
3. **Review access logs**
4. **Improve detection**

## 📚 Additional Resources

- [Git Security Best Practices](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History)
- [Environment Variable Management](https://12factor.net/config)
- [VSCode Settings Security](https://code.visualstudio.com/docs/getstarted/settings)

## 🤝 Team Coordination

### For Git History Rewrites
1. **Announce to team** before starting
2. **Coordinate timing** to minimize disruption
3. **Provide clear instructions** for re-cloning
4. **Verify all team members** have updated

### For Security Updates
1. **Document changes** in this file
2. **Update pre-commit hooks** as needed
3. **Test with team members**
4. **Provide migration guides**

---

**Remember:** Security is everyone's responsibility. When in doubt, ask for review before committing or pushing changes.
