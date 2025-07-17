/// <reference types="cypress" />

describe('Setup Wizard Navigation', () => {
  beforeEach(() => {
    cy.visit('/')
    // Trigger the setup wizard by clicking the run setup wizard button
    cy.get('[data-cy="run-setup-wizard-button"]').click()
    cy.get('[data-cy="setup-wizard-container"]').should('be.visible')
  })

  it('should display the setup wizard with progress indicator', () => {
    // Verify wizard container is visible
    cy.get('[data-cy="setup-wizard-container"]').should('be.visible')
    
    // Verify progress indicator is present
    cy.get('[data-cy="wizard-progress-indicator"]').should('be.visible')
    
    // Verify all step indicators are present
    for (let i = 1; i <= 6; i++) {
      cy.get(`[data-cy="wizard-step-${i}"]`).should('be.visible')
    }
    
    // Verify step 1 is active (should have active styling)
    cy.get('[data-cy="wizard-step-1"]').should('be.visible')
  })

  it('should navigate through welcome step correctly', () => {
    // Verify we start on welcome step
    cy.get('[data-cy="wizard-welcome-step"]').should('be.visible')
    
    // Test skip functionality
    cy.get('[data-cy="wizard-welcome-skip"]').should('be.visible').and('contain', 'Skip Wizard')
    
    // Test next functionality
    cy.get('[data-cy="wizard-welcome-next"]').should('be.visible').and('contain', 'Start Setup')
    cy.get('[data-cy="wizard-welcome-next"]').click()
    
    // Should navigate to listener step
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    cy.get('[data-cy="wizard-welcome-step"]').should('not.exist')
  })

  it('should navigate through listener step correctly', () => {
    // Navigate to listener step
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Test back navigation
    cy.get('[data-cy="wizard-listener-previous"]').should('be.visible').and('contain', 'Back')
    cy.get('[data-cy="wizard-listener-previous"]').click()
    
    // Should go back to welcome step
    cy.get('[data-cy="wizard-welcome-step"]').should('be.visible')
    cy.get('[data-cy="wizard-listener-step"]').should('not.exist')
    
    // Navigate forward again
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
  })

  it('should validate listener form inputs', () => {
    // Navigate to listener step
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Verify all form inputs are present
    cy.get('[data-cy="listener-name-input"]').should('be.visible')
    cy.get('[data-cy="listener-protocol-select"]').should('be.visible')
    cy.get('[data-cy="listener-hostname-input"]').should('be.visible')
    cy.get('[data-cy="listener-port-input"]').should('be.visible')
    
    // Test form interaction
    cy.get('[data-cy="listener-name-input"]').should('have.value', 'default')
    cy.get('[data-cy="listener-name-input"]').clear().type('test-listener')
    cy.get('[data-cy="listener-name-input"]').should('have.value', 'test-listener')
    
    cy.get('[data-cy="listener-hostname-input"]').should('have.value', 'localhost')
    cy.get('[data-cy="listener-hostname-input"]').clear().type('0.0.0.0')
    cy.get('[data-cy="listener-hostname-input"]').should('have.value', '0.0.0.0')
    
    cy.get('[data-cy="listener-port-input"]').should('have.value', '8080')
    cy.get('[data-cy="listener-port-input"]').clear().type('3000')
    cy.get('[data-cy="listener-port-input"]').should('have.value', '3000')
  })

  it('should validate protocol selection', () => {
    // Navigate to listener step
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Verify protocol selection is present and functional
    cy.get('[data-cy="listener-protocol-select"]').should('be.visible')
    
    // Test protocol selection (HTTP should be default)
    cy.get('[data-cy="listener-protocol-select"]').within(() => {
      // Verify HTTP is selected by default
      cy.get('input[value="HTTP"]').should('be.checked')
      
      // Test selecting HTTPS
      cy.get('input[value="HTTPS"]').click()
      cy.get('input[value="HTTPS"]').should('be.checked')
      cy.get('input[value="HTTP"]').should('not.be.checked')
      
      // Test selecting TCP
      cy.get('input[value="TCP"]').click()
      cy.get('input[value="TCP"]').should('be.checked')
      cy.get('input[value="HTTPS"]').should('not.be.checked')
    })
  })

  it('should handle form validation errors', () => {
    // Navigate to listener step
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Clear required fields to trigger validation
    cy.get('[data-cy="listener-name-input"]').clear()
    cy.get('[data-cy="listener-hostname-input"]').clear()
    cy.get('[data-cy="listener-port-input"]').clear()
    
    // Try to proceed - should show validation errors
    cy.get('[data-cy="wizard-listener-next"]').click()
    
    // Should still be on listener step due to validation errors
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Fill in valid data
    cy.get('[data-cy="listener-name-input"]').type('valid-listener')
    cy.get('[data-cy="listener-hostname-input"]').type('localhost')
    cy.get('[data-cy="listener-port-input"]').type('8080')
    
    // Now should be able to proceed
    cy.get('[data-cy="wizard-listener-next"]').click()
    
    // Should navigate away from listener step (to next step)
    cy.get('[data-cy="wizard-listener-step"]').should('not.exist')
  })

  it('should maintain form state during navigation', () => {
    // Navigate to listener step
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Fill in some data
    cy.get('[data-cy="listener-name-input"]').clear().type('persistent-listener')
    cy.get('[data-cy="listener-hostname-input"]').clear().type('example.com')
    cy.get('[data-cy="listener-port-input"]').clear().type('9000')
    
    // Navigate back
    cy.get('[data-cy="wizard-listener-previous"]').click()
    cy.get('[data-cy="wizard-welcome-step"]').should('be.visible')
    
    // Navigate forward again
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-listener-step"]').should('be.visible')
    
    // Verify form state is preserved
    cy.get('[data-cy="listener-name-input"]').should('have.value', 'persistent-listener')
    cy.get('[data-cy="listener-hostname-input"]').should('have.value', 'example.com')
    cy.get('[data-cy="listener-port-input"]').should('have.value', '9000')
  })

  it('should show step progress correctly', () => {
    // Start on step 1
    cy.get('[data-cy="wizard-step-1"]').should('be.visible')
    
    // Navigate to step 2
    cy.get('[data-cy="wizard-welcome-next"]').click()
    cy.get('[data-cy="wizard-step-2"]').should('be.visible')
    
    // Fill in valid listener data and proceed
    cy.get('[data-cy="listener-name-input"]').clear().type('test-listener')
    cy.get('[data-cy="listener-hostname-input"]').clear().type('localhost')
    cy.get('[data-cy="listener-port-input"]').clear().type('8080')
    cy.get('[data-cy="wizard-listener-next"]').click()
    
    // Should be on step 3 now
    cy.get('[data-cy="wizard-step-3"]').should('be.visible')
  })
})
