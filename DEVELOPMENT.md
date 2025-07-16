# Local Development

For instructions on how to run everything locally, see the [DEVELOPMENT.md](DEVELOPMENT.md) file.

## Build from Source

Requirements:
- Rust 1.86+
- npm 10+

Build the agentgateway UI:

```bash
cd ui
npm install
npm run build
```

Build the agentgateway binary:

```bash
cd ..
make build
```

If you encounter an authentication error to the schemars repo in GitHub, try set `CARGO_NET_GIT_FETCH_WITH_CLI=true` and rerun `make build`.

Run the agentgateway binary:

```bash
./target/release/agentgateway
```
Open your browser and navigate to `http://localhost:15000/ui` to see the agentgateway UI.

## Testing

To run all tests, you need to set the required environment variables:

```bash
# Set required environment variables and run all tests
unset CARGO_BUILD_JOBS && \
export NAMESPACE=test_namespace && \
export GATEWAY=test_gateway && \
export WORKER_THREADS=4 && \
export CPU_LIMIT=4 && \
export INSTANCE_IP=127.0.0.1 && \
export POD_NAME=test-pod && \
export POD_NAMESPACE=test_namespace && \
export NODE_NAME=test-node && \
cargo test --all-targets
```

Or use the Makefile target:

```bash
make test
```

### Running Specific Test Types

**Unit Tests Only:**
```bash
# With environment variables set (as above)
cargo test --all-targets
```

**Integration Tests:**
```bash
# With environment variables set (as above)
cargo test --test integration
```

**Benchmarks:**
```bash
# With environment variables set (as above)
cargo bench
```

### Environment Variables Required for Testing

The following environment variables are required for all tests to pass:

- `NAMESPACE`: Kubernetes namespace (use `test_namespace` for testing)
- `GATEWAY`: Gateway name (use `test_gateway` for testing)  
- `WORKER_THREADS`: Number of worker threads (use `4` for testing)
- `CPU_LIMIT`: CPU limit (use `4` for testing)
- `INSTANCE_IP`: Instance IP address (use `127.0.0.1` for testing)
- `POD_NAME`: Pod name (use `test-pod` for testing)
- `POD_NAMESPACE`: Pod namespace (use `test_namespace` for testing)
- `NODE_NAME`: Node name (use `test-node` for testing)

**Note:** The `CARGO_BUILD_JOBS` environment variable should be unset if it exists, as an empty value can cause test failures.
