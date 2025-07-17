# AgentGateway E2E Implementation Plan

## Overview

This document provides a comprehensive, phase-based implementation plan for E2E testing of the AgentGateway UI. Based on codebase analysis, this plan addresses the gaps between our journey map and the actual implementation, providing specific todo items for systematic E2E test development.

## Critical Findings from Codebase Analysis

### Current State Assessment
- ✅ **UI Structure**: Well-organized component structure exists
- ✅ **Setup Wizard**: 6-step wizard implementation (Welcome, Listener, Routes, Backends, Policies, Review)
- ✅ **Playground**: MCP and A2A connection support with ActionPanel, CapabilitiesList, ResponseDisplay
- ✅ **Navigation**: Next.js routing with proper sidebar structure
- ✅ **Cypress Setup**: Complete E2E testing infrastructure implemented
- ✅ **Data-cy Attributes**: Foundation components now have comprehensive data-cy attributes
- ❌ **Test Coverage**: Limited E2E tests exist (foundation only)

### Journey Map Corrections Made
1. **Setup Wizard Steps**: Confirmed 6 steps implementation
2. **Playground Components**: ActionPanel, CapabilitiesList, ResponseDisplay documented
3. **Navigation Pattern**: Next.js routing patterns confirmed
4. **Component Structure**: Aligned with actual component names and hierarchy

---

## Phase 1: Foundation & Validation ✅ COMPLETE
**Duration**: 1-2 weeks  
**Priority**: Critical  
**Dependencies**: None

### 1.1 Codebase Audit & Validation ✅ COMPLETE

#### Completed Items:
- ✅ **Audited all UI components for existing data-cy attributes**
  - Confirmed zero existing data-cy usage in original codebase
  - Documented baseline state and requirements
  - Created systematic approach for implementation

- ✅ **Validated journey map against actual component structure**
  - Confirmed 6-step setup wizard structure
  - Documented actual component hierarchy
  - Updated journey maps with correct component names

- ✅ **Component structure deep dive**
  - Mapped actual component hierarchy in `ui/src/components/`
  - Documented props and state management patterns
  - Identified key interactive elements requiring data-cy attributes

### 1.2 Cypress Infrastructure Setup ✅ COMPLETE

#### Completed Items:
- ✅ **Installed Cypress dependencies**
  - Cypress already available in project dependencies
  - TypeScript support configured
  - Testing utilities integrated

- ✅ **Created Cypress configuration**
  - Created `ui/cypress.config.ts` with comprehensive settings
  - Configured base URL, timeouts, and viewport settings
  - Set up folder structure: `e2e/`, `fixtures/`, `support/`

- ✅ **Set up TypeScript support**
  - Configured TypeScript for Cypress tests
  - Added type definitions for custom commands
  - Created proper tsconfig for test files

- ✅ **Created initial test structure**
  ```
  ui/cypress/
  ├── e2e/
  │   └── foundation/
  │       └── app-loads.cy.ts
  ├── fixtures/
  │   └── configurations/
  │       └── basic-config.json
  └── support/
      ├── commands.ts
      └── e2e.ts
  ```

### 1.3 Test Environment Configuration ✅ COMPLETE

#### Completed Items:
- ✅ **Created test data fixtures**
  - Basic configuration templates in `basic-config.json`
  - Test data structure for listeners, routes, backends, policies
  - Foundation for future test data expansion

- ✅ **Set up test environment**
  - Custom Cypress commands for common operations
  - Error handling and global configuration
  - Test utilities for navigation and dashboard testing

---

## Phase 2: Data-cy Attribute Implementation ✅ PARTIALLY COMPLETE
**Duration**: 2-3 weeks  
**Priority**: Critical  
**Dependencies**: Phase 1 complete

### 2.1 Navigation Components ✅ COMPLETE

#### Completed Items:
- ✅ **Updated `ui/src/components/app-sidebar.tsx`**
  - Added `data-cy="nav-home"` to Home navigation
  - Added `data-cy="nav-listeners"` to Listeners navigation
  - Added `data-cy="nav-routes"` to Routes navigation
  - Added `data-cy="nav-backends"` to Backends navigation
  - Added `data-cy="nav-policies"` to Policies navigation
  - Added `data-cy="nav-playground"` to Playground navigation
  - Added `data-cy="restart-setup-button"` to Restart Setup button
  - Added `data-cy="theme-toggle"` to theme toggle

- ✅ **Updated main layout components**
  - Added navigation badges with `data-cy="nav-badge-{section}"`
  - Added sidebar navigation controls
  - Implemented consistent naming conventions

### 2.2 Dashboard/Home Page ✅ COMPLETE

#### Completed Items:
- ✅ **Updated `ui/src/app/page.tsx`**
  - Added `data-cy="dashboard-content"` to main content area
  - Added `data-cy="dashboard-loading"` to loading states
  - Added `data-cy="dashboard-statistics-card"` to stat cards
  - Added `data-cy="dashboard-listeners-count"` to listener count
  - Added `data-cy="dashboard-routes-count"` to route count
  - Added `data-cy="dashboard-backends-count"` to backend count
  - Added `data-cy="dashboard-binds-count"` to binds count
  - Added `data-cy="create-first-listener-button"` to getting started button
  - Added `data-cy="run-setup-wizard-button"` to wizard button

### 2.3 Setup Wizard Components ✅ PARTIALLY COMPLETE

#### Completed Items:
- ✅ **Updated `ui/src/components/setup-wizard/index.tsx`**
  - Added `data-cy="setup-wizard-container"` to main container
  - Added `data-cy="wizard-step-{number}"` to each step indicator
  - Added `data-cy="wizard-progress-indicator"` to progress bar

- ✅ **Updated `ui/src/components/setup-wizard/WelcomeStep.tsx`**
  - Added `data-cy="wizard-welcome-step"` to step container
  - Added `data-cy="wizard-welcome-next"` to Next button
  - Added `data-cy="wizard-welcome-skip"` to Skip button

#### Remaining Items:
- [ ] **Update `ui/src/components/setup-wizard/ListenerStep.tsx`**
  - [ ] Add `data-cy="wizard-listener-step"` to step container
  - [ ] Add `data-cy="listener-name-input"` to name field
  - [ ] Add `data-cy="listener-port-input"` to port field
  - [ ] Add `data-cy="listener-protocol-select"` to protocol dropdown
  - [ ] Add `data-cy="listener-hostname-input"` to hostname field
  - [ ] Add `data-cy="wizard-listener-next"` to Next button
  - [ ] Add `data-cy="wizard-listener-previous"` to Previous button

- [ ] **Update `ui/src/components/setup-wizard/RouteStep.tsx`**
  - [ ] Add `data-cy="wizard-route-step"` to step container
  - [ ] Add `data-cy="route-name-input"` to route name field
  - [ ] Add `data-cy="route-path-input"` to path field
  - [ ] Add `data-cy="route-match-type-select"` to match type dropdown
  - [ ] Add `data-cy="wizard-route-next"` to Next button
  - [ ] Add `data-cy="wizard-route-previous"` to Previous button

- [ ] **Update `ui/src/components/setup-wizard/BackendStep.tsx`**
  - [ ] Add `data-cy="wizard-backend-step"` to step container
  - [ ] Add `data-cy="backend-type-select"` to backend type dropdown
  - [ ] Add `data-cy="backend-name-input"` to backend name field
  - [ ] Add `data-cy="wizard-backend-next"` to Next button
  - [ ] Add `data-cy="wizard-backend-previous"` to Previous button

- [ ] **Update `ui/src/components/setup-wizard/PolicyStep.tsx`**
  - [ ] Add `data-cy="wizard-policy-step"` to step container
  - [ ] Add policy-specific form attributes
  - [ ] Add `data-cy="wizard-policy-next"` to Next button
  - [ ] Add `data-cy="wizard-policy-previous"` to Previous button

- [ ] **Update `ui/src/components/setup-wizard/ReviewStep.tsx`**
  - [ ] Add `data-cy="wizard-review-step"` to step container
  - [ ] Add `data-cy="configuration-summary"` to summary display
  - [ ] Add `data-cy="wizard-complete"` to Complete button
  - [ ] Add `data-cy="wizard-review-previous"` to Previous button

### 2.4 Configuration Management Pages

#### Todo Items:
- [ ] **Update `ui/src/app/listeners/page.tsx`**
  - [ ] Add `data-cy="listeners-page"` to page container
  - [ ] Add `data-cy="add-listener-button"` to add button
  - [ ] Add `data-cy="listener-card-{id}"` to listener cards
  - [ ] Add `data-cy="listener-edit-button"` to edit buttons
  - [ ] Add `data-cy="listener-delete-button"` to delete buttons

- [ ] **Update `ui/src/app/routes/page.tsx`**
  - [ ] Add `data-cy="routes-page"` to page container
  - [ ] Add `data-cy="add-route-button"` to add button
  - [ ] Add `data-cy="route-card-{id}"` to route cards
  - [ ] Add `data-cy="route-edit-button"` to edit buttons
  - [ ] Add `data-cy="route-delete-button"` to delete buttons

- [ ] **Update `ui/src/app/backends/page.tsx`**
  - [ ] Add `data-cy="backends-page"` to page container
  - [ ] Add `data-cy="add-backend-button"` to add button
  - [ ] Add `data-cy="backend-card-{id}"` to backend cards
  - [ ] Add `data-cy="backend-edit-button"` to edit buttons
  - [ ] Add `data-cy="backend-delete-button"` to delete buttons

- [ ] **Update `ui/src/app/policies/page.tsx`**
  - [ ] Add `data-cy="policies-page"` to page container
  - [ ] Add policy-specific attributes

### 2.5 Playground Components

#### Todo Items:
- [ ] **Update `ui/src/app/playground/page.tsx`**
  - [ ] Add `data-cy="playground-page"` to page container
  - [ ] Add `data-cy="connection-type-selector"` to connection type selection
  - [ ] Add `data-cy="connect-button"` to connect button
  - [ ] Add `data-cy="disconnect-button"` to disconnect button

- [ ] **Update `ui/src/components/playground/ActionPanel.tsx`**
  - [ ] Add `data-cy="action-panel"` to main container
  - [ ] Add `data-cy="tool-parameter-form"` to parameter form
  - [ ] Add `data-cy="run-tool-button"` to run button
  - [ ] Add `data-cy="tool-parameter-{key}"` to each parameter input
  - [ ] Add `data-cy="json-error-message"` to JSON validation errors

- [ ] **Update `ui/src/components/playground/CapabilitiesList.tsx`**
  - [ ] Add `data-cy="capabilities-list"` to main container
  - [ ] Add `data-cy="tool-card-{name}"` to tool cards
  - [ ] Add `data-cy="tool-selector"` to tool selection elements

- [ ] **Update `ui/src/components/playground/ResponseDisplay.tsx`**
  - [ ] Add `data-cy="response-display"` to main container
  - [ ] Add `data-cy="response-content"` to response content
  - [ ] Add `data-cy="response-error"` to error displays

### 2.6 Form Components

#### Todo Items:
- [ ] **Update form components in `ui/src/components/forms/`**
  - [ ] Add `data-cy` attributes to all form inputs
  - [ ] Add validation error attributes
  - [ ] Add form submission attributes

- [ ] **Update UI components in `ui/src/components/ui/`**
  - [ ] Add `data-cy` attributes to buttons, inputs, selects
  - [ ] Focus on interactive elements used in forms

---

## Phase 3: Core Journey Tests
**Duration**: 3-4 weeks  
**Priority**: High  
**Dependencies**: Phase 2 complete

### 3.1 Dashboard/Overview Tests ✅ PARTIALLY COMPLETE

#### Completed Items:
- ✅ **Created `ui/cypress/e2e/foundation/app-loads.cy.ts`**
  - Tests dashboard loads with proper statistics
  - Verifies all statistic cards display correctly
  - Tests navigation links from dashboard
  - Verifies getting started flow for empty state

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/foundation/dashboard-navigation.cy.ts`**
  - [ ] Test navigation to all sections from dashboard
  - [ ] Verify active state indicators
  - [ ] Test breadcrumb navigation
  - [ ] Verify navigation badges update correctly

### 3.2 Setup Wizard E2E Flow Tests

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/setup-wizard/wizard-complete-flow.cy.ts`**
  - [ ] Test complete wizard flow with valid inputs
  - [ ] Verify each step transitions correctly
  - [ ] Test configuration persistence
  - [ ] Verify redirect to dashboard on completion

- [ ] **Create `ui/cypress/e2e/setup-wizard/wizard-navigation.cy.ts`**
  - [ ] Test forward/backward navigation
  - [ ] Verify form state preservation during navigation
  - [ ] Test skip functionality
  - [ ] Test wizard restart functionality

- [ ] **Create `ui/cypress/e2e/setup-wizard/wizard-validation.cy.ts`**
  - [ ] Test form validation on each step
  - [ ] Verify error message display
  - [ ] Test required field validation
  - [ ] Test format validation (ports, URLs, etc.)

### 3.3 Configuration Management Tests

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/configuration/listeners-crud.cy.ts`**
  - [ ] Test listener creation workflow
  - [ ] Test listener editing functionality
  - [ ] Test listener deletion with confirmation
  - [ ] Test listener list updates

- [ ] **Create `ui/cypress/e2e/configuration/routes-crud.cy.ts`**
  - [ ] Test route creation with different match types
  - [ ] Test route editing and updates
  - [ ] Test route deletion
  - [ ] Test route-listener relationships

- [ ] **Create `ui/cypress/e2e/configuration/backends-crud.cy.ts`**
  - [ ] Test backend creation for different types
  - [ ] Test backend configuration forms
  - [ ] Test backend editing and deletion
  - [ ] Test backend-route relationships

### 3.4 Navigation and State Management Tests

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/navigation/sidebar-navigation.cy.ts`**
  - [ ] Test all sidebar navigation links
  - [ ] Verify active state indicators
  - [ ] Test navigation badges
  - [ ] Test theme toggle functionality

- [ ] **Create `ui/cypress/e2e/navigation/deep-linking.cy.ts`**
  - [ ] Test direct URL access to pages
  - [ ] Verify proper page loading
  - [ ] Test navigation context preservation

---

## Phase 4: Advanced Feature Tests
**Duration**: 2-3 weeks  
**Priority**: Medium  
**Dependencies**: Phase 3 complete

### 4.1 Playground Testing

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/playground/mcp-testing.cy.ts`**
  - [ ] Test MCP connection establishment
  - [ ] Test tool discovery and selection
  - [ ] Test tool parameter input and execution
  - [ ] Test response display and error handling

- [ ] **Create `ui/cypress/e2e/playground/a2a-testing.cy.ts`**
  - [ ] Test A2A connection setup
  - [ ] Test skill selection and execution
  - [ ] Test message composition and sending
  - [ ] Test response handling

- [ ] **Create `ui/cypress/e2e/playground/http-testing.cy.ts`**
  - [ ] Test HTTP endpoint configuration
  - [ ] Test request building and sending
  - [ ] Test response analysis
  - [ ] Test different HTTP methods

### 4.2 Integration Tests

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/integration/end-to-end-configuration.cy.ts`**
  - [ ] Test complete configuration workflow
  - [ ] Verify configuration relationships
  - [ ] Test configuration validation
  - [ ] Test configuration persistence

- [ ] **Create `ui/cypress/e2e/integration/configuration-persistence.cy.ts`**
  - [ ] Test configuration saves correctly
  - [ ] Test page refresh maintains state
  - [ ] Test navigation preserves configuration
  - [ ] Test error recovery

### 4.3 Error Handling and Validation Tests

#### Todo Items:
- [ ] **Create `ui/cypress/e2e/error-handling/form-validation.cy.ts`**
  - [ ] Test all form validation scenarios
  - [ ] Test error message display
  - [ ] Test error recovery workflows
  - [ ] Test validation across all forms

- [ ] **Create `ui/cypress/e2e/error-handling/connection-errors.cy.ts`**
  - [ ] Test backend connection failures
  - [ ] Test network error handling
  - [ ] Test timeout scenarios
  - [ ] Test error message clarity

---

## Phase 5: CI/CD Integration & Maintenance
**Duration**: 1-2 weeks  
**Priority**: Medium  
**Dependencies**: Phase 4 complete

### 5.1 CI/CD Pipeline Setup

#### Todo Items:
- [ ] **Create `.github/workflows/e2e-tests.yml`**
  - [ ] Set up GitHub Actions workflow
  - [ ] Configure test environment setup
  - [ ] Add test execution and reporting
  - [ ] Set up artifact collection

- [ ] **Configure test execution**
  - [ ] Set up parallel test execution
  - [ ] Configure test retries and timeouts
  - [ ] Add test result reporting
  - [ ] Set up failure notifications

### 5.2 Test Data Management

#### Todo Items:
- [ ] **Create test data management system**
  - [ ] Set up test data fixtures
  - [ ] Create data cleanup procedures
  - [ ] Add test data versioning
  - [ ] Create data seeding scripts

- [ ] **Create custom Cypress commands**
  - [ ] Add common workflow commands
  - [ ] Create data setup/teardown commands
  - [ ] Add assertion helpers
  - [ ] Create utility functions

### 5.3 Monitoring and Maintenance

#### Todo Items:
- [ ] **Set up test monitoring**
  - [ ] Add test execution metrics
  - [ ] Set up failure tracking
  - [ ] Create performance monitoring
  - [ ] Add test coverage reporting

- [ ] **Create maintenance procedures**
  - [ ] Document test update procedures
  - [ ] Create test review guidelines
  - [ ] Set up regular test audits
  - [ ] Add test performance optimization

---

## Implementation Guidelines

### Data-cy Naming Conventions
- **Navigation**: `nav-{section}` (e.g., `nav-dashboard`, `nav-listeners`)
- **Forms**: `{entity}-{field}-{type}` (e.g., `listener-name-input`, `route-path-select`)
- **Actions**: `{action}-{entity}-button` (e.g., `add-listener-button`, `save-route-button`)
- **Lists**: `{entity}-list`, `{entity}-card-{id}` (e.g., `listener-list`, `listener-card-1`)
- **Status**: `{entity}-{status}` (e.g., `backend-status-healthy`, `connection-status-error`)

### Test Structure Guidelines
- Use Page Object Model for complex interactions
- Keep tests independent and atomic
- Use descriptive test names and comments
- Implement proper wait strategies
- Use fixtures for test data

### Quality Assurance
- All interactive elements must have data-cy attributes
- Tests must be reliable and not flaky
- Error scenarios must be covered
- Performance impact must be minimal
- Accessibility must be maintained

---

## Success Metrics

### Phase Completion Criteria
- **Phase 1**: ✅ Cypress setup complete, codebase audited
- **Phase 2**: 🔄 All interactive elements have data-cy attributes (60% complete)
- **Phase 3**: ⏳ Core user journeys have comprehensive test coverage
- **Phase 4**: ⏳ Advanced features tested, error scenarios covered
- **Phase 5**: ⏳ CI/CD pipeline operational, maintenance procedures in place

### Overall Success Indicators
- 90%+ test coverage of user journeys
- All critical paths tested
- CI/CD pipeline passing consistently
- Test execution time under 10 minutes
- Zero flaky tests in main test suite

---

## Risk Mitigation

### Technical Risks
- **Component changes breaking tests**: Use stable data-cy selectors
- **Test flakiness**: Implement proper wait strategies and retries
- **Performance impact**: Minimize data-cy attribute overhead
- **Maintenance burden**: Create clear documentation and procedures

### Process Risks
- **Incomplete implementation**: Use phase-based approach with checkpoints
- **Team adoption**: Provide training and clear guidelines
- **CI/CD failures**: Implement proper error handling and notifications
- **Test data management**: Create robust data setup/cleanup procedures

---

## Progress Summary

### ✅ Completed (Phase 1 + Partial Phase 2)
- **Cypress Infrastructure**: Complete setup with TypeScript support
- **Navigation Components**: Full data-cy attribute implementation
- **Dashboard Components**: Complete data-cy attribute coverage
- **Setup Wizard Foundation**: Container, progress indicator, and welcome step
- **Foundation Tests**: Basic application loading and navigation tests
- **Custom Commands**: Utility functions for common test operations
- **Test Fixtures**: Basic configuration data for testing

### 🔄 In Progress (Phase 2 Continuation)
- **Setup Wizard Steps**: Remaining wizard steps need data-cy attributes
- **Configuration Pages**: Listeners, routes, backends, policies pages
- **Playground Components**: MCP, A2A, HTTP testing interfaces

### ⏳ Upcoming (Phases 3-5)
- **Comprehensive Test Suite**: Full E2E test coverage
- **Advanced Feature Testing**: Playground, integrations, error handling
- **CI/CD Integration**: Automated testing pipeline
- **Maintenance Framework**: Long-term test management

---

## Notes

- **Always update this todo list before final commit of any functionality**
- Each phase should be completed and validated before moving to the next
- Regular reviews should be conducted to ensure quality and coverage
- Test failures should be addressed immediately to maintain pipeline health
- Documentation should be updated as implementation progresses

**Last Updated**: January 17, 2025
**Current Status**: Phase 1 Complete, Phase 2 60% Complete
**Next Priority**: Complete remaining setup wizard data-cy attributes
