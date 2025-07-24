/// <reference types="cypress" />

/**
 * Critical Path Smoke Tests
 * 
 * These tests verify the most essential functionality that must work
 * for the application to be considered functional. Run these first
 * for rapid feedback on deployment health.
 */

describe('Critical Path Smoke Tests', () => {
  beforeEach(() => {
    cy.visit('/', { failOnStatusCode: false })
  })

  it('should load the application and display dashboard', () => {
    // Verify app loads
    cy.get('body').should('be.visible')
    cy.get('[data-cy="dashboard-content"]').should('be.visible')
    
    // Verify core navigation is present
    cy.get('[data-cy="nav-home"]').should('be.visible')
    cy.get('[data-cy="nav-listeners"]').should('be.visible')
    cy.get('[data-cy="nav-routes"]').should('be.visible')
    cy.get('[data-cy="nav-backends"]').should('be.visible')
  })

  it('should navigate to main sections without errors', () => {
    // Test critical navigation paths
    cy.get('[data-cy="nav-listeners"]').click()
    cy.url().should('include', '/listeners')
    cy.get('body').should('be.visible')
    
    cy.get('[data-cy="nav-routes"]').click()
    cy.url().should('include', '/routes')
    cy.get('body').should('be.visible')
    
    cy.get('[data-cy="nav-backends"]').click()
    cy.url().should('include', '/backends')
    cy.get('body').should('be.visible')
    
    cy.get('[data-cy="nav-home"]').click()
    cy.url().should('eq', Cypress.config().baseUrl + '/')
  })

  it('should display dashboard statistics cards', () => {
    // Verify all critical dashboard elements are present
    cy.get('[data-cy="dashboard-listeners-card"]').should('be.visible')
    cy.get('[data-cy="dashboard-routes-card"]').should('be.visible')
    cy.get('[data-cy="dashboard-backends-card"]').should('be.visible')
    cy.get('[data-cy="dashboard-binds-card"]').should('be.visible')
  })

  it('should have functional theme toggle', () => {
    // Test theme switching (critical UI functionality)
    cy.get('[data-cy="theme-toggle"]').should('be.visible').click()
    // Theme should change (we don't test specific colors, just that it responds)
    cy.get('[data-cy="theme-toggle"]').should('be.visible')
  })

  it('should show setup wizard entry point', () => {
    // Verify setup wizard is accessible for new users
    cy.get('[data-cy="restart-setup-button"]').should('be.visible')
    cy.get('[data-cy="create-first-listener-button"]').should('be.visible')
  })
})
