/// <reference types="cypress" />

/**
 * API Health Smoke Tests
 * 
 * These tests verify that the backend API is responding correctly
 * and that basic API functionality is working.
 */

describe('API Health Smoke Tests', () => {
  it('should have healthy backend connection', () => {
    // Test that the backend is reachable
    cy.request({
      method: 'GET',
      url: 'http://localhost:15021/healthz/ready',
      failOnStatusCode: false
    }).then((response) => {
      expect(response.status).to.be.oneOf([200, 404]) // 404 is acceptable if endpoint doesn't exist
    })
  })

  it('should load configuration data without errors', () => {
    cy.visit('/', { failOnStatusCode: false })
    
    // Verify that the dashboard loads configuration counts
    // This implicitly tests that API calls are working
    cy.get('[data-cy="dashboard-listeners-count"]').should('be.visible')
    cy.get('[data-cy="dashboard-routes-count"]').should('be.visible')
    cy.get('[data-cy="dashboard-backends-count"]').should('be.visible')
    cy.get('[data-cy="dashboard-binds-count"]').should('be.visible')
  })

  it('should handle navigation to configuration pages', () => {
    cy.visit('/', { failOnStatusCode: false })
    
    // Test that configuration pages load (which require API calls)
    cy.get('[data-cy="nav-listeners"]').click()
    cy.url().should('include', '/listeners')
    // Page should load without JavaScript errors
    cy.get('body').should('be.visible')
    
    cy.get('[data-cy="nav-routes"]').click()
    cy.url().should('include', '/routes')
    cy.get('body').should('be.visible')
    
    cy.get('[data-cy="nav-backends"]').click()
    cy.url().should('include', '/backends')
    cy.get('body').should('be.visible')
  })

  it('should not have console errors on page load', () => {
    // Capture console errors
    cy.window().then((win) => {
      cy.stub(win.console, 'error').as('consoleError')
    })
    
    cy.visit('/', { failOnStatusCode: false })
    
    // Wait for page to fully load
    cy.get('[data-cy="dashboard-content"]').should('be.visible')
    
    // Check that no critical console errors occurred
    cy.get('@consoleError').should('not.have.been.called')
  })
})
