#!/bin/bash

# Git History Cleanup Script
# This script helps clean sensitive information from git history
# WARNING: This will rewrite git history - coordinate with your team!

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

print_header "Git History Cleanup for AgentGateway"
print_warning "This script will rewrite git history to remove sensitive information"
print_warning "ALL TEAM MEMBERS will need to re-clone after this process"

# Confirmation
echo ""
read -p "Are you sure you want to proceed? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    print_info "Cleanup cancelled"
    exit 0
fi

# Create sensitive patterns file
print_info "Creating sensitive patterns file..."
cat > sensitive-patterns.txt << 'EOF'
# Personal information
mkostreba=USERNAME
michael.kostreba@wingedcommerce.com=user@company.com
wingedcommerce.com=company.com
wc-p3-001=hostname

# Local paths (regex patterns for BFG)
regex:/home/[^/]+/=/home/user/
regex:/Users/[^/]+/=/Users/user/
regex:C:\\Users\\[^\\]+\\=C:\Users\user\

# Company-specific references
WingedCommerce-LLC=Company-LLC
WingedCommerce=Company
EOF

print_success "Created sensitive-patterns.txt"

# Create backup branch
BACKUP_BRANCH="backup-$(date +%Y%m%d-%H%M%S)"
print_info "Creating backup branch: $BACKUP_BRANCH"
git branch "$BACKUP_BRANCH"
print_success "Backup branch created: $BACKUP_BRANCH"

# Check if BFG is available
if ! command -v java &> /dev/null; then
    print_error "Java is required for BFG Repo-Cleaner"
    print_info "Please install Java and try again"
    exit 1
fi

# Download BFG if not present
BFG_JAR="bfg-1.14.0.jar"
if [ ! -f "$BFG_JAR" ]; then
    print_info "Downloading BFG Repo-Cleaner..."
    if command -v wget &> /dev/null; then
        wget -O "$BFG_JAR" "https://repo1.maven.org/maven2/com/madgag/bfg/1.14.0/bfg-1.14.0.jar"
    elif command -v curl &> /dev/null; then
        curl -L -o "$BFG_JAR" "https://repo1.maven.org/maven2/com/madgag/bfg/1.14.0/bfg-1.14.0.jar"
    else
        print_error "Please download BFG manually from: https://rtyley.github.io/bfg-repo-cleaner/"
        print_info "Save it as: $BFG_JAR"
        exit 1
    fi
    print_success "Downloaded BFG Repo-Cleaner"
fi

# Run BFG to clean sensitive data
print_header "Running BFG Repo-Cleaner"
print_info "This may take a few minutes..."

java -jar "$BFG_JAR" --replace-text sensitive-patterns.txt --no-blob-protection .

print_success "BFG cleanup completed"

# Clean up git repository
print_header "Cleaning Git Repository"
print_info "Expiring reflog and running garbage collection..."

git reflog expire --expire=now --all
git gc --prune=now --aggressive

print_success "Git cleanup completed"

# Verify cleanup
print_header "Verification"
print_info "Checking for remaining sensitive patterns..."

FOUND_ISSUES=0

# Check for sensitive patterns in current files
if grep -r "mkostreba\|michael\.kostreba\|wingedcommerce\.com\|wc-p3-001" . --exclude-dir=.git --exclude="sensitive-patterns.txt" --exclude="$BFG_JAR" --exclude-dir=target 2>/dev/null; then
    print_warning "Found sensitive patterns in current files (may be in documentation)"
    FOUND_ISSUES=1
fi

# Check git log for sensitive information
if git log --all --grep="mkostreba\|michael\.kostreba\|wingedcommerce\.com\|wc-p3-001" --oneline | head -5; then
    print_warning "Found sensitive patterns in commit messages"
    FOUND_ISSUES=1
fi

if [ $FOUND_ISSUES -eq 0 ]; then
    print_success "No obvious sensitive patterns found in verification"
else
    print_warning "Some patterns found - review manually"
fi

# Instructions for next steps
print_header "Next Steps"
print_info "Git history has been cleaned. Next actions:"
echo ""
echo "1. Review the changes:"
echo "   git log --oneline -10"
echo ""
echo "2. Test that the project still works:"
echo "   cargo build"
echo "   ./scripts/setup-local-dev.sh"
echo ""
echo "3. If satisfied, force push (COORDINATE WITH TEAM FIRST):"
echo "   git push --force-with-lease origin main"
echo ""
echo "4. Notify team members to re-clone:"
echo "   git clone https://github.com/WingedCommerce-LLC/agentgateway.git"
echo ""
echo "5. Clean up:"
echo "   rm sensitive-patterns.txt $BFG_JAR"

print_warning "Remember: All team members must re-clone after force push!"

# Cleanup option
echo ""
read -p "Clean up temporary files now? (yes/no): " cleanup
if [ "$cleanup" = "yes" ]; then
    rm -f sensitive-patterns.txt "$BFG_JAR"
    print_success "Temporary files cleaned up"
fi

print_success "Git history cleanup completed!"
print_info "Backup branch available: $BACKUP_BRANCH"
