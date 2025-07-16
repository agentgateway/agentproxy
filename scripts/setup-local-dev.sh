#!/bin/bash

# Local Development Setup Script
# This script helps set up a secure local development environment

set -e

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_error() {
    echo -e "${RED}ERROR: $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

print_success() {
    echo -e "${GREEN}SUCCESS: $1${NC}"
}

print_info() {
    echo -e "${BLUE}INFO: $1${NC}"
}

print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Check if we're in the project root
if [ ! -f "Cargo.toml" ] || [ ! -d "crates" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

print_header "AgentGateway Local Development Setup"

# 1. Set up git hooks
print_info "Setting up git hooks..."
if [ -f ".githooks/pre-commit" ]; then
    git config core.hooksPath .githooks
    print_success "Git hooks configured"
else
    print_warning "Pre-commit hook not found, skipping git hooks setup"
fi

# 2. Set up VSCode configuration
print_info "Setting up VSCode configuration..."
if [ -d ".vscode-templates" ]; then
    if [ ! -d ".vscode" ]; then
        mkdir -p .vscode
    fi
    
    # Copy templates if they don't exist
    for template in .vscode-templates/*.template; do
        if [ -f "$template" ]; then
            filename=$(basename "$template" .template)
            target=".vscode/$filename"
            
            if [ ! -f "$target" ]; then
                cp "$template" "$target"
                print_success "Created $target from template"
            else
                print_info "$target already exists, skipping"
            fi
        fi
    done
else
    print_warning "VSCode templates not found, skipping VSCode setup"
fi

# 3. Set up environment variables
print_info "Setting up environment configuration..."
if [ -f ".env.example" ]; then
    if [ ! -f ".env" ]; then
        cp .env.example .env
        print_success "Created .env from .env.example"
        print_warning "Please customize .env for your local environment"
    else
        print_info ".env already exists, skipping"
    fi
else
    print_warning ".env.example not found, skipping environment setup"
fi

# 4. Set up local configuration
print_info "Setting up local configuration files..."
if [ -f "manifests/localhost-config.example.json" ]; then
    if [ ! -f "manifests/localhost-config.json" ]; then
        cp manifests/localhost-config.example.json manifests/localhost-config.json
        print_success "Created localhost-config.json from example"
        print_warning "Please customize localhost-config.json for your local environment"
    else
        print_info "localhost-config.json already exists, skipping"
    fi
fi

# 5. Check Python environment
print_info "Checking Python environment..."
if command -v python3 &> /dev/null; then
    PYTHON_VERSION=$(python3 --version)
    print_success "Found Python: $PYTHON_VERSION"
    
    # Check for virtual environment
    if [ ! -d ".venv" ]; then
        print_info "Creating Python virtual environment..."
        python3 -m venv .venv
        print_success "Created .venv"
    else
        print_info "Virtual environment already exists"
    fi
else
    print_warning "Python3 not found, please install Python 3.10+"
fi

# 6. Check Rust environment
print_info "Checking Rust environment..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Found Rust: $RUST_VERSION"
    
    if command -v cargo &> /dev/null; then
        print_success "Cargo is available"
    else
        print_warning "Cargo not found"
    fi
else
    print_warning "Rust not found, please install Rust"
fi

# 7. Check Node.js for UI development
print_info "Checking Node.js environment..."
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    print_success "Found Node.js: $NODE_VERSION"
    
    if command -v npm &> /dev/null; then
        print_success "npm is available"
    else
        print_warning "npm not found"
    fi
else
    print_warning "Node.js not found, needed for UI development"
fi

# 8. Security check
print_header "Security Validation"
print_info "Running security checks..."

# Check if .vscode is in .gitignore
if grep -q "^\.vscode/" .gitignore; then
    print_success ".vscode/ is properly ignored"
else
    print_warning ".vscode/ should be added to .gitignore"
fi

# Check if .env is in .gitignore
if grep -q "^\.env$" .gitignore; then
    print_success ".env is properly ignored"
else
    print_warning ".env should be added to .gitignore"
fi

# Check for any committed sensitive files
SENSITIVE_FILES=()
if [ -f ".vscode/settings.json" ] && git ls-files --error-unmatch .vscode/settings.json &> /dev/null; then
    SENSITIVE_FILES+=(".vscode/settings.json")
fi

if [ -f ".env" ] && git ls-files --error-unmatch .env &> /dev/null; then
    SENSITIVE_FILES+=(".env")
fi

if [ ${#SENSITIVE_FILES[@]} -gt 0 ]; then
    print_error "Found committed sensitive files:"
    for file in "${SENSITIVE_FILES[@]}"; do
        echo "  - $file"
    done
    print_warning "These files should be removed from git tracking"
fi

# 9. Final instructions
print_header "Setup Complete"
print_success "Local development environment setup completed!"

echo ""
print_info "Next steps:"
echo "1. Customize .env with your local settings"
echo "2. Customize .vscode/ files if needed"
echo "3. Customize manifests/localhost-config.json"
echo "4. Run 'cargo build' to test Rust setup"
echo "5. Run 'cd ui && npm install' to set up UI dependencies"

echo ""
print_info "Security reminders:"
echo "- Never commit .vscode/, .env, or localhost-config.json files"
echo "- Use environment variables for local-specific settings"
echo "- The pre-commit hook will help prevent accidental commits"
echo "- Review changes before pushing to upstream"

echo ""
print_info "For more information, see:"
echo "- .vscode-templates/README.md for VSCode setup"
echo "- .env.example for environment variable documentation"
echo "- SECURITY.md for security guidelines (when created)"

print_success "Happy coding! 🚀"
