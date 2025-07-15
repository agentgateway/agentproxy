# Install Scripts

This directory contains scripts for installing agentgateway.

## Files

- `get-agentproxy` - Main install script for agentgateway
- `test-get-agentproxy.sh` - Test script for the install script
- `report_build_info.sh` - Build information reporting script

## Install Script Testing

### Running Tests

To test the install script functionality:

```bash
# Run install script tests only
make test-install-script

# Run all tests (including install script tests)
make test-all
```

Or run the test script directly:

```bash
./common/scripts/test-get-agentproxy.sh
```

### Test Coverage

The test script (`test-get-agentproxy.sh`) covers the following scenarios:

1. **Version Flag Test**: Verifies that `agentgateway --version` works correctly
2. **Version Subcommand Rejection**: Confirms that `agentgateway version` fails with the expected error
3. **Install Script Function Test**: Tests the `checkAgentGatewayInstalledVersion` function from the install script
4. **Old Method Failure**: Verifies that the old broken version check method fails as expected
5. **New Method Success**: Confirms that the new version check method extracts version information correctly

### GitHub Issue #61 Fix

The install script was fixed to resolve [GitHub issue #61](https://github.com/agentgateway/agentgateway/issues/61) where users encountered:

```
error: unexpected argument 'version' found
```

**Root Cause**: The install script was calling `agentgateway version --template="{{ .Version }}"` but the agentgateway binary only supports the `--version` flag, not a `version` subcommand.

**Fix**: Changed the version check in `checkAgentGatewayInstalledVersion()` function from:
```bash
local version=$("${AGENTGATEWAY_INSTALL_DIR}/${BINARY_NAME}" version --template="{{ .Version }}")
```

To:
```bash
local version=$("${AGENTGATEWAY_INSTALL_DIR}/${BINARY_NAME}" --version)
```

This fix ensures the install script works correctly without showing version errors to users.

### Test Output

When all tests pass, you should see output similar to:

```
[INFO] Starting tests for get-agentproxy install script
[INFO] Testing the fix for GitHub issue #61: Install script shows version error

Running test: agentgateway --version flag works
[PASS] agentgateway --version works correctly

Running test: agentgateway version subcommand fails as expected
[PASS] agentgateway version subcommand correctly fails with expected error

Running test: Install script version check function works
[PASS] checkAgentGatewayInstalledVersion function works correctly

Running test: Old version check method fails as expected
[PASS] Old version check method correctly fails (confirming the bug existed)

Running test: New version check extracts version information
[PASS] New version check successfully extracts version information

=== Test Summary ===
Tests run: 5
Tests passed: 5
Tests failed: 0

All tests passed! The install script version check fix is working correctly.
