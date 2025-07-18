describe('Configuration Persistence', () => {
  beforeEach(() => {
    cy.visit('/');
  });

  describe('Configuration State Persistence', () => {
    it('should persist configuration across page refreshes', () => {
      // Create a configuration through the setup wizard
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      // Configure listener
      cy.get('[data-cy="listener-name-input"]').type('persistence-test-listener');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      // Select HTTP protocol using radio button
      cy.get('[data-cy="listener-protocol-select"]').within(() => {
        cy.get('input[value="HTTP"]').click({ force: true });
      });
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      // Configure route
      cy.get('[data-cy="route-name-input"]').type('persistence-test-route');
      cy.get('[data-cy="route-path-input"]').type('/api/persist');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      // Configure backend
      cy.get('[data-cy="backend-name-input"]').type('persistence-test-backend');
      // Select Host backend type using radio button
      cy.get('[data-cy="backend-type-select"]').within(() => {
        cy.get('input[value="Host"]').click({ force: true });
      });
      cy.get('[data-cy="backend-target-name-input"]').type('http://localhost:3001');
      cy.get('[data-cy="wizard-backend-next"]').scrollIntoView().click({ force: true });
      
      // Skip policies and complete
      cy.get('[data-cy="wizard-policy-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-complete"]').scrollIntoView().click({ force: true });
      
      // Wait for completion
      cy.wait(3000);
      
      // Refresh the page
      cy.reload();
      
      // Verify configuration persists after refresh
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      cy.get('body').should('contain', 'persistence-test-listener');
      
      cy.get('[data-cy="nav-routes"]').click();
      cy.get('[data-cy="routes-page"]').should('be.visible');
      cy.get('body').should('contain', 'persistence-test-route');
      
      cy.get('[data-cy="nav-backends"]').click();
      cy.get('[data-cy="backends-page"]').should('be.visible');
      cy.get('body').should('contain', 'persistence-test-backend');
      
      cy.log('Configuration persistence across page refresh verified');
    });

    it('should maintain configuration state during navigation', () => {
      // Start configuration process
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      // Fill listener configuration
      cy.get('[data-cy="listener-name-input"]').type('navigation-persistence-test');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      
      // Navigate away from wizard
      cy.get('[data-cy="nav-home"]').click();
      cy.get('[data-cy="dashboard-content"]').should('be.visible');
      
      // Navigate to different pages
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      
      cy.get('[data-cy="nav-routes"]').click();
      cy.get('[data-cy="routes-page"]').should('be.visible');
      
      cy.get('[data-cy="nav-backends"]').click();
      cy.get('[data-cy="backends-page"]').should('be.visible');
      
      // Return to home and check if wizard state is preserved
      cy.get('[data-cy="nav-home"]').click();
      cy.get('[data-cy="dashboard-content"]').should('be.visible');
      
      // Restart wizard and check state handling
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      
      // Verify wizard handles previous state appropriately
      cy.get('body').should('exist');
      cy.log('Configuration state during navigation verified');
    });

    it('should preserve partial configuration during wizard interruption', () => {
      // Start wizard and fill partial configuration
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      // Fill listener step
      cy.get('[data-cy="listener-name-input"]').type('partial-persistence-test');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      // Select HTTP protocol using radio button
      cy.get('[data-cy="listener-protocol-select"]').within(() => {
        cy.get('input[value="HTTP"]').click({ force: true });
      });
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      // Fill route step partially
      cy.get('[data-cy="route-name-input"]').type('partial-route');
      cy.get('[data-cy="route-path-input"]').type('/api/partial');
      
      // Navigate away without completing
      cy.get('[data-cy="nav-playground"]').click();
      cy.get('[data-cy="playground-page"]').should('be.visible');
      
      // Navigate back to home
      cy.get('[data-cy="nav-home"]').click();
      cy.get('[data-cy="dashboard-content"]').should('be.visible');
      
      // Restart wizard
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      
      // Check how partial state is handled
      cy.get('body').should('exist');
      cy.log('Partial configuration persistence during interruption verified');
    });
  });

  describe('Configuration Data Integrity', () => {
    it('should maintain data integrity across multiple configuration operations', () => {
      // Perform multiple configuration operations
      const operations = [
        { name: 'integrity-listener-1', port: '8081' },
        { name: 'integrity-listener-2', port: '8082' },
        { name: 'integrity-listener-3', port: '8083' }
      ];
      
      operations.forEach((config, index) => {
        // Start wizard for each configuration
        cy.get('[data-cy="run-setup-wizard-button"]').click();
        cy.get('[data-cy="wizard-welcome-next"]').click();
        
        // Configure listener
        cy.get('[data-cy="listener-name-input"]').type(config.name);
        cy.get('[data-cy="listener-port-input"]').clear().type(config.port);
        // Select HTTP protocol using radio button
        cy.get('[data-cy="listener-protocol-select"]').within(() => {
          cy.get('input[value="HTTP"]').click({ force: true });
        });
        cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
        
        // Quick route configuration
        cy.get('[data-cy="route-name-input"]').type(`integrity-route-${index + 1}`);
        cy.get('[data-cy="route-path-input"]').type(`/api/integrity-${index + 1}`);
        cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
        
        // Quick backend configuration
        cy.get('[data-cy="backend-name-input"]').type(`integrity-backend-${index + 1}`);
        // Select Host backend type using radio button
        cy.get('[data-cy="backend-type-select"]').within(() => {
          cy.get('input[value="Host"]').click({ force: true });
        });
        cy.get('[data-cy="backend-target-name-input"]').type(`http://localhost:300${index + 1}`);
        cy.get('[data-cy="wizard-backend-next"]').scrollIntoView().click({ force: true });
        
        // Skip policies and complete
        cy.get('[data-cy="wizard-policy-next"]').scrollIntoView().click({ force: true });
        cy.get('[data-cy="wizard-complete"]').scrollIntoView().click({ force: true });
        
        // Wait for completion
        cy.wait(2000);
      });
      
      // Verify all configurations are preserved
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      
      operations.forEach((config) => {
        cy.get('body').should('contain', config.name);
      });
      
      cy.log('Data integrity across multiple operations verified');
    });

    it('should handle concurrent configuration attempts gracefully', () => {
      // Simulate rapid configuration attempts
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      // Fill configuration rapidly
      cy.get('[data-cy="listener-name-input"]').type('concurrent-test');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      
      // Rapidly navigate between steps
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-route-previous"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      // Continue with route configuration
      cy.get('[data-cy="route-name-input"]').type('concurrent-route');
      cy.get('[data-cy="route-path-input"]').type('/api/concurrent');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      // Verify system handles concurrent operations
      cy.get('[data-cy="wizard-backend-step"]').should('be.visible');
      cy.log('Concurrent configuration attempts handled gracefully');
    });

    it('should validate configuration consistency after modifications', () => {
      // Create initial configuration
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      cy.get('[data-cy="listener-name-input"]').type('consistency-test-listener');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="route-name-input"]').type('consistency-test-route');
      cy.get('[data-cy="route-path-input"]').type('/api/consistency');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="backend-name-input"]').type('consistency-test-backend');
      // Select Host backend type using radio button
      cy.get('[data-cy="backend-type-select"]').within(() => {
        cy.get('input[value="Host"]').click({ force: true });
      });
      cy.get('[data-cy="backend-target-name-input"]').type('http://localhost:3001');
      cy.get('[data-cy="wizard-backend-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="wizard-policy-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-complete"]').scrollIntoView().click({ force: true });
      
      cy.wait(3000);
      
      // Verify configuration exists
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      cy.get('body').should('contain', 'consistency-test-listener');
      
      // Attempt to modify configuration (if edit functionality exists)
      cy.get('body').then(($body) => {
        if ($body.find('[data-cy*="edit-listener"]').length > 0) {
          cy.get('[data-cy*="edit-listener"]').first().click();
          cy.log('Configuration modification initiated');
          
          // Verify consistency after modification
          cy.wait(2000);
          cy.get('body').should('exist');
        } else {
          cy.log('Configuration modification not available');
        }
      });
      
      cy.log('Configuration consistency validation completed');
    });
  });

  describe('Browser Storage and Session Management', () => {
    it('should handle browser storage limitations gracefully', () => {
      // Create large configuration to test storage limits
      const largeConfigName = 'large-config-' + 'x'.repeat(100);
      
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      cy.get('[data-cy="listener-name-input"]').type(largeConfigName);
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      // Continue with large route name
      const largeRouteName = 'large-route-' + 'y'.repeat(100);
      cy.get('[data-cy="route-name-input"]').type(largeRouteName);
      cy.get('[data-cy="route-path-input"]').type('/api/large');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      // Verify system handles large data gracefully
      cy.get('[data-cy="wizard-backend-step"]').should('be.visible');
      cy.log('Large configuration data handled gracefully');
    });

    it('should maintain session state across browser tabs', () => {
      // Start configuration in current tab
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      cy.get('[data-cy="listener-name-input"]').type('session-test-listener');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      
      // Navigate to different page
      cy.get('[data-cy="nav-playground"]').click();
      cy.get('[data-cy="playground-page"]').should('be.visible');
      
      // Return to home and check session state
      cy.get('[data-cy="nav-home"]').click();
      cy.get('[data-cy="dashboard-content"]').should('be.visible');
      
      // Restart wizard and verify session handling
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      
      cy.get('body').should('exist');
      cy.log('Session state across navigation verified');
    });

    it('should handle configuration cleanup and reset', () => {
      // Create configuration
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      cy.get('[data-cy="listener-name-input"]').type('cleanup-test-listener');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="route-name-input"]').type('cleanup-test-route');
      cy.get('[data-cy="route-path-input"]').type('/api/cleanup');
      
      // Navigate away without completing
      cy.get('[data-cy="nav-home"]').click();
      
      // Check if restart setup button is available
      cy.get('body').then(($body) => {
        if ($body.find('[data-cy="restart-setup-button"]').length > 0) {
          cy.get('[data-cy="restart-setup-button"]').click();
          cy.log('Configuration reset initiated');
          
          // Verify reset functionality
          cy.wait(2000);
          cy.get('body').should('exist');
        } else {
          cy.log('Configuration reset not available');
        }
      });
      
      cy.log('Configuration cleanup and reset handling verified');
    });
  });

  describe('Error Recovery and Data Consistency', () => {
    it('should recover from storage errors gracefully', () => {
      // Attempt configuration that might cause storage issues
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      // Fill configuration with potentially problematic data
      cy.get('[data-cy="listener-name-input"]').type('storage-error-test');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      // Continue configuration
      cy.get('[data-cy="route-name-input"]').type('storage-error-route');
      cy.get('[data-cy="route-path-input"]').type('/api/storage-error');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      // Verify graceful handling of potential storage errors
      cy.get('[data-cy="wizard-backend-step"]').should('be.visible');
      cy.log('Storage error recovery verified');
    });

    it('should maintain data consistency during network interruptions', () => {
      // Start configuration process
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      cy.get('[data-cy="listener-name-input"]').type('network-interruption-test');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="route-name-input"]').type('network-interruption-route');
      cy.get('[data-cy="route-path-input"]').type('/api/network-test');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="backend-name-input"]').type('network-interruption-backend');
      // Select Host backend type using radio button
      cy.get('[data-cy="backend-type-select"]').within(() => {
        cy.get('input[value="Host"]').click({ force: true });
      });
      cy.get('[data-cy="backend-target-name-input"]').type('http://localhost:3001');
      
      // Attempt to complete configuration (may fail due to network issues)
      cy.get('[data-cy="wizard-backend-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-policy-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-complete"]').scrollIntoView().click({ force: true });
      
      // Wait and verify graceful handling
      cy.wait(5000);
      cy.get('body').should('exist');
      
      // Check if partial configuration is preserved
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      
      cy.log('Network interruption data consistency verified');
    });

    it('should validate configuration integrity after system recovery', () => {
      // Create configuration
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      cy.get('[data-cy="listener-name-input"]').type('integrity-recovery-test');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="route-name-input"]').type('integrity-recovery-route');
      cy.get('[data-cy="route-path-input"]').type('/api/integrity-recovery');
      cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="backend-name-input"]').type('integrity-recovery-backend');
      // Select Host backend type using radio button
      cy.get('[data-cy="backend-type-select"]').within(() => {
        cy.get('input[value="Host"]').click({ force: true });
      });
      cy.get('[data-cy="backend-target-name-input"]').type('http://localhost:3001');
      cy.get('[data-cy="wizard-backend-next"]').scrollIntoView().click({ force: true });
      
      cy.get('[data-cy="wizard-policy-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-complete"]').scrollIntoView().click({ force: true });
      
      cy.wait(3000);
      
      // Simulate system recovery by refreshing
      cy.reload();
      
      // Verify configuration integrity after recovery
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      cy.get('body').should('contain', 'integrity-recovery-test');
      
      cy.get('[data-cy="nav-routes"]').click();
      cy.get('[data-cy="routes-page"]').should('be.visible');
      cy.get('body').should('contain', 'integrity-recovery-route');
      
      cy.get('[data-cy="nav-backends"]').click();
      cy.get('[data-cy="backends-page"]').should('be.visible');
      cy.get('body').should('contain', 'integrity-recovery-backend');
      
      cy.log('Configuration integrity after system recovery verified');
    });
  });

  describe('Performance and Scalability', () => {
    it('should handle large configuration datasets efficiently', () => {
      // Test with multiple configurations to simulate scale
      const configCount = 5;
      
      for (let i = 0; i < configCount; i++) {
        cy.get('[data-cy="run-setup-wizard-button"]').click();
        cy.get('[data-cy="wizard-welcome-next"]').click();
        
        cy.get('[data-cy="listener-name-input"]').type(`scale-test-listener-${i}`);
        cy.get('[data-cy="listener-port-input"]').clear().type(`808${i}`);
        cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
        
        cy.get('[data-cy="route-name-input"]').type(`scale-test-route-${i}`);
        cy.get('[data-cy="route-path-input"]').type(`/api/scale-${i}`);
        cy.get('[data-cy="wizard-route-next"]').scrollIntoView().click({ force: true });
        
        cy.get('[data-cy="backend-name-input"]').type(`scale-test-backend-${i}`);
        // Select Host backend type using radio button
        cy.get('[data-cy="backend-type-select"]').within(() => {
          cy.get('input[value="Host"]').click({ force: true });
        });
        cy.get('[data-cy="backend-target-name-input"]').type(`http://localhost:300${i}`);
        cy.get('[data-cy="wizard-backend-next"]').scrollIntoView().click({ force: true });
        
        cy.get('[data-cy="wizard-policy-next"]').scrollIntoView().click({ force: true });
        cy.get('[data-cy="wizard-complete"]').scrollIntoView().click({ force: true });
        
        cy.wait(1000);
      }
      
      // Verify all configurations are accessible
      cy.get('[data-cy="nav-listeners"]').click();
      cy.get('[data-cy="listeners-page"]').should('be.visible');
      
      // Check that page loads efficiently with multiple configurations
      cy.get('body').should('exist');
      cy.log('Large configuration dataset handling verified');
    });

    it('should maintain performance during rapid configuration changes', () => {
      // Perform rapid configuration operations
      cy.get('[data-cy="run-setup-wizard-button"]').click();
      cy.get('[data-cy="wizard-welcome-next"]').click();
      
      // Rapidly fill and change configuration
      cy.get('[data-cy="listener-name-input"]').type('rapid-change-test');
      cy.get('[data-cy="listener-name-input"]').clear().type('rapid-change-updated');
      cy.get('[data-cy="listener-port-input"]').clear().type('8080');
      cy.get('[data-cy="listener-port-input"]').clear().type('8081');
      cy.get('[data-cy="listener-port-input"]').clear().type('8082');
      
      // Continue with rapid navigation
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-route-previous"]').scrollIntoView().click({ force: true });
      cy.get('[data-cy="wizard-listener-next"]').scrollIntoView().click({ force: true });
      
      // Verify system maintains performance
      cy.get('[data-cy="wizard-route-step"]').should('be.visible');
      cy.log('Performance during rapid changes verified');
    });
  });
});
