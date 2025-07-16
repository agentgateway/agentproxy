# VSCode Configuration Templates

This directory contains sanitized VSCode configuration templates that can be customized for local development without exposing personal information.

## Setup Instructions

1. Copy the template files to `.vscode/` directory:
   ```bash
   cp .vscode-templates/*.template .vscode/
   ```

2. Rename the template files:
   ```bash
   cd .vscode/
   mv settings.json.template settings.json
   mv launch.json.template launch.json
   mv tasks.json.template tasks.json
   ```

3. Customize the files for your local environment by:
   - Setting environment variables
   - Updating paths to match your system
   - Configuring your preferred Python interpreter

## Environment Variables

The templates use these environment variables that you can set in your shell:

- `PYTHON_INTERPRETER`: Path to your Python interpreter (default: `python3`)
- `RUST_ANALYZER_PATH`: Path to rust-analyzer if not in PATH
- `AGENTGATEWAY_DEBUG_PORT`: Debug port for the application (default: `5005`)

## Security Notes

- Never commit the actual `.vscode/` directory
- Use relative paths and environment variables only
- Avoid hardcoding personal paths or system-specific configurations
- Review configurations before committing any changes

## Customization Examples

### Python Interpreter
Instead of hardcoding `/home/username/.venv/bin/python`, use:
- Environment variable: `${env:PYTHON_INTERPRETER}`
- Workspace relative: `${workspaceFolder}/.venv/bin/python`
- System default: `python3`

### Debug Configuration
Use environment variables for ports and paths:
- `${env:AGENTGATEWAY_DEBUG_PORT}` instead of hardcoded ports
- `${workspaceFolder}` for all workspace-relative paths
