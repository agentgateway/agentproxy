export default function SimplePage() {
  return (
    <div data-cy="dashboard-content">
      <h1>AgentGateway Dashboard</h1>
      <p>Simple test page to verify E2E tests work</p>
      <button data-cy="create-first-listener-button">Create First Listener</button>
      <button data-cy="run-setup-wizard-button">Run Setup Wizard</button>
    </div>
  );
}
