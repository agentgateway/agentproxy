#!/bin/bash

# AgentGateway E2E Test Runner
# This script automatically sets up the environment and runs E2E tests

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BACKEND_PORT=8080
UI_PORT=3000
BACKEND_URL="http://localhost:${BACKEND_PORT}"
UI_URL="http://localhost:${UI_PORT}/ui"
TIMEOUT=60
PARALLEL_WORKERS=4

# Default values
MODE="parallel"
HEADLESS=true
CLEANUP=true
VERBOSE=false

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

AgentGateway E2E Test Runner - Automatically sets up environment and runs tests

OPTIONS:
    -m, --mode MODE         Test execution mode: parallel, sequential, interactive (default: parallel)
    -w, --workers NUM       Number of parallel workers (default: 4)
    -t, --timeout SEC       Timeout for service startup (default: 60)
    --no-cleanup           Don't cleanup processes after tests
    --headed               Run tests in headed mode (visible browser)
    --verbose              Enable verbose logging
    -h, --help             Show this help message

EXAMPLES:
    $0                                    # Run tests in parallel mode
    $0 --mode sequential                  # Run tests sequentially
    $0 --mode interactive                 # Open Cypress test runner
    $0 --workers 8 --verbose            # Run with 8 workers and verbose logging
    $0 --headed --no-cleanup             # Run with visible browser, don't cleanup

ENVIRONMENT VARIABLES:
    AGENTGATEWAY_BINARY    Path to agentgateway binary (default: auto-detect)
    SKIP_BUILD            Skip building agentgateway (default: false)
    SKIP_BACKEND          Skip starting backend (assume already running)
    SKIP_UI               Skip starting UI (assume already running)
    CI                    CI mode - enables additional logging (default: false)

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -w|--workers)
            PARALLEL_WORKERS="$2"
            shift 2
            ;;
        -t|--timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        --no-cleanup)
            CLEANUP=false
            shift
            ;;
        --headed)
            HEADLESS=false
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Validate mode
if [[ ! "$MODE" =~ ^(parallel|sequential|interactive)$ ]]; then
    print_error "Invalid mode: $MODE. Must be one of: parallel, sequential, interactive"
    exit 1
fi

# Global variables for process tracking
BACKEND_PID=""
UI_PID=""

# Cleanup function
cleanup() {
    if [[ "$CLEANUP" == "true" ]]; then
        print_status "Cleaning up processes..."
        
        if [[ -n "$BACKEND_PID" ]]; then
            print_status "Stopping AgentGateway backend (PID: $BACKEND_PID)"
            kill $BACKEND_PID 2>/dev/null || true
            wait $BACKEND_PID 2>/dev/null || true
        fi
        
        if [[ -n "$UI_PID" ]]; then
            print_status "Stopping UI development server (PID: $UI_PID)"
            kill $UI_PID 2>/dev/null || true
            wait $UI_PID 2>/dev/null || true
        fi
        
        # Kill any remaining processes on our ports
        lsof -ti:$BACKEND_PORT | xargs kill -9 2>/dev/null || true
        lsof -ti:$UI_PORT | xargs kill -9 2>/dev/null || true
        
        print_success "Cleanup completed"
    else
        print_warning "Skipping cleanup - processes left running"
        if [[ -n "$BACKEND_PID" ]]; then
            print_warning "AgentGateway backend PID: $BACKEND_PID"
        fi
        if [[ -n "$UI_PID" ]]; then
            print_warning "UI development server PID: $UI_PID"
        fi
    fi
}

# Set up signal handlers
trap cleanup EXIT
trap 'print_error "Script interrupted"; exit 1' INT TERM

# Function to wait for service to be ready
wait_for_service() {
    local url=$1
    local service_name=$2
    local timeout=$3
    
    print_status "Waiting for $service_name to be ready at $url..."
    
    local count=0
    while [[ $count -lt $timeout ]]; do
        if curl -s -f "$url" > /dev/null 2>&1; then
            print_success "$service_name is ready!"
            return 0
        fi
        
        if [[ $((count % 10)) -eq 0 ]] && [[ $count -gt 0 ]]; then
            print_status "Still waiting for $service_name... (${count}s elapsed)"
        fi
        
        sleep 1
        ((count++))
    done
    
    print_error "$service_name failed to start within ${timeout}s"
    return 1
}

# Function to find agentgateway binary
find_agentgateway_binary() {
    if [[ -n "$AGENTGATEWAY_BINARY" ]]; then
        if [[ -x "$AGENTGATEWAY_BINARY" ]]; then
            echo "$AGENTGATEWAY_BINARY"
            return 0
        else
            print_error "AGENTGATEWAY_BINARY is set but not executable: $AGENTGATEWAY_BINARY"
            return 1
        fi
    fi
    
    # Try common locations
    local candidates=(
        "./target/release/agentgateway"
        "./target/debug/agentgateway"
        "./target/release/agentgateway-app"
        "./target/debug/agentgateway-app"
        "$(which agentgateway 2>/dev/null)"
        "$(which agentgateway-app 2>/dev/null)"
    )
    
    for candidate in "${candidates[@]}"; do
        if [[ -x "$candidate" ]]; then
            echo "$candidate"
            return 0
        fi
    done
    
    return 1
}

# Function to build agentgateway if needed
build_agentgateway() {
    if [[ "$SKIP_BUILD" == "true" ]]; then
        print_status "Skipping build (SKIP_BUILD=true)"
        return 0
    fi
    
    print_status "Building AgentGateway..."
    
    # Always show build output for debugging
    if [[ "$VERBOSE" == "true" ]] || [[ "$CI" == "true" ]]; then
        cargo build --release --bin agentgateway
        local build_result=$?
    else
        # Capture output for error reporting
        local build_output
        build_output=$(cargo build --release --bin agentgateway 2>&1)
        local build_result=$?
        
        if [[ $build_result -ne 0 ]]; then
            print_error "AgentGateway build failed with output:"
            echo "$build_output"
        fi
    fi
    
    if [[ $build_result -eq 0 ]]; then
        print_success "AgentGateway build completed"
    else
        print_error "AgentGateway build failed"
        return 1
    fi
}

# Function to start AgentGateway backend
start_backend() {
    print_status "Starting AgentGateway backend..."
    
    # Check if port is already in use
    if lsof -Pi :$BACKEND_PORT -sTCP:LISTEN -t >/dev/null; then
        print_warning "Port $BACKEND_PORT is already in use"
        if curl -s -f "$BACKEND_URL/health" > /dev/null 2>&1; then
            print_success "AgentGateway is already running and healthy"
            return 0
        else
            print_error "Port $BACKEND_PORT is occupied by another service"
            return 1
        fi
    fi
    
    # Find or build the binary
    local binary
    if ! binary=$(find_agentgateway_binary); then
        print_status "AgentGateway binary not found, building..."
        build_agentgateway
        if ! binary=$(find_agentgateway_binary); then
            print_error "Failed to find AgentGateway binary after build"
            return 1
        fi
    fi
    
    print_status "Using AgentGateway binary: $binary"
    
    # Start the backend with test configuration
    if [[ "$VERBOSE" == "true" ]]; then
        "$binary" --file test-config.yaml &
    else
        "$binary" --file test-config.yaml > /dev/null 2>&1 &
    fi
    
    BACKEND_PID=$!
    print_status "AgentGateway backend started (PID: $BACKEND_PID)"
    
    # Wait for it to be ready (use readiness endpoint)
    if ! wait_for_service "http://localhost:15021/healthz/ready" "AgentGateway backend" $TIMEOUT; then
        return 1
    fi
    
    return 0
}

# Function to start UI development server
start_ui() {
    print_status "Starting UI development server..."
    
    # Check if port is already in use
    if lsof -Pi :$UI_PORT -sTCP:LISTEN -t >/dev/null; then
        print_warning "Port $UI_PORT is already in use"
        if curl -s -f "$UI_URL" > /dev/null 2>&1; then
            print_success "UI development server is already running"
            return 0
        else
            print_error "Port $UI_PORT is occupied by another service"
            return 1
        fi
    fi
    
    # Navigate to UI directory
    if [[ ! -d "ui" ]]; then
        print_error "UI directory not found. Please run this script from the project root."
        return 1
    fi
    
    cd ui
    
    # Install dependencies if needed
    if [[ ! -d "node_modules" ]]; then
        print_status "Installing UI dependencies..."
        if [[ "$VERBOSE" == "true" ]]; then
            npm install
        else
            npm install > /dev/null 2>&1
        fi
    fi
    
    # Start the development server
    print_status "Starting Next.js development server..."
    if [[ "$VERBOSE" == "true" ]]; then
        npm run dev &
    else
        npm run dev > /dev/null 2>&1 &
    fi
    
    UI_PID=$!
    print_status "UI development server started (PID: $UI_PID)"
    
    # Wait for it to be ready
    if ! wait_for_service "$UI_URL" "UI development server" $TIMEOUT; then
        cd ..
        return 1
    fi
    
    cd ..
    return 0
}

# Function to run tests
run_tests() {
    print_status "Running E2E tests in $MODE mode..."
    
    cd ui
    
    local exit_code=0
    
    case $MODE in
        "parallel")
            print_status "Running tests with $PARALLEL_WORKERS workers..."
            if [[ "$VERBOSE" == "true" ]]; then
                npm run test:e2e:parallel -- --workers $PARALLEL_WORKERS --debug
            else
                npm run test:e2e:parallel -- --workers $PARALLEL_WORKERS
            fi
            exit_code=$?
            ;;
        "sequential")
            print_status "Running tests sequentially..."
            if [[ "$HEADLESS" == "true" ]]; then
                npm run e2e
            else
                npm run cypress:run:headed
            fi
            exit_code=$?
            ;;
        "interactive")
            print_status "Opening Cypress test runner..."
            npm run e2e:open
            exit_code=$?
            ;;
    esac
    
    cd ..
    
    if [[ $exit_code -eq 0 ]]; then
        print_success "E2E tests completed successfully!"
    else
        print_error "E2E tests failed with exit code: $exit_code"
    fi
    
    return $exit_code
}

# Main execution
main() {
    print_status "AgentGateway E2E Test Runner"
    print_status "Mode: $MODE"
    if [[ "$MODE" == "parallel" ]]; then
        print_status "Workers: $PARALLEL_WORKERS"
    fi
    print_status "Timeout: ${TIMEOUT}s"
    print_status "Cleanup: $CLEANUP"
    echo
    
    # Check prerequisites
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is required but not installed"
        exit 1
    fi
    
    if ! command -v node &> /dev/null; then
        print_error "Node.js is required but not installed"
        exit 1
    fi
    
    if ! command -v npm &> /dev/null; then
        print_error "npm is required but not installed"
        exit 1
    fi
    
    # Start services
    if ! start_backend; then
        print_error "Failed to start AgentGateway backend"
        exit 1
    fi
    
    if ! start_ui; then
        print_error "Failed to start UI development server"
        exit 1
    fi
    
    # Run tests
    if ! run_tests; then
        print_error "E2E tests failed"
        exit 1
    fi
    
    print_success "All E2E tests completed successfully!"
}

# Run main function
main "$@"
