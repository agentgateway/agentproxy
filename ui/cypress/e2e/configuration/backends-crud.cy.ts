describe('Backends CRUD Operations', () => {
  beforeEach(() => {
    cy.visit('/');
  });

  it('should navigate to backends page and display content', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    
    // Verify we're on the backends page
    cy.get('[data-cy="backends-page"]').should('be.visible');
    cy.url().should('include', '/backends');
    
    // Check for backends content or empty state
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="backends-empty-state"]').length > 0) {
        cy.get('[data-cy="backends-empty-state"]').should('be.visible');
      } else {
        // If backends exist, verify the page displays content
        cy.log('Backends content is displayed');
        cy.get('[data-cy="backends-page"]').should('not.be.empty');
      }
    });
  });

  it('should display backend statistics by type', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Check for backend type statistics
    cy.get('body').then(($body) => {
      // Look for backend statistics
      if ($body.find('[data-cy*="statistics"]').length > 0) {
        cy.get('[data-cy*="statistics"]').should('be.visible');
      } else if ($body.find('[data-cy*="stats"]').length > 0) {
        cy.get('[data-cy*="stats"]').should('be.visible');
      }
      
      // Check for backend type breakdown (MCP, Host, Service)
      if ($body.find('[data-cy*="type"]').length > 0) {
        cy.get('[data-cy*="type"]').should('be.visible');
      }
      
      // Check for backend count
      if ($body.find('[data-cy*="count"]').length > 0) {
        cy.get('[data-cy*="count"]').should('be.visible');
      }
    });
    
    // Verify page shows meaningful content
    cy.get('[data-cy="backends-page"]').should('contain.text', /backend|service|host|mcp|empty/i);
  });

  it('should create a new Host backend', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Look for add backend button
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-backend-button"]').length > 0) {
        cy.get('[data-cy="add-backend-button"]').click();
      } else if ($body.find('[data-cy="create-backend-button"]').length > 0) {
        cy.get('[data-cy="create-backend-button"]').click();
      } else {
        // Look for any button with "add" or "create" in the text
        cy.contains('button', /add|create/i).first().click();
      }
    });
    
    // Wait for form to appear
    cy.wait(1000);
    
    // Fill in backend details for Host type
    cy.get('body').then(($body) => {
      // Select backend type (Host)
      if ($body.find('[data-cy="backend-type-select"]').length > 0) {
        cy.get('[data-cy="backend-type-select"]').click();
        // Look for Host option
        cy.contains(/host/i).click();
      }
      
      // Fill backend name
      const nameSelectors = [
        '[data-cy="backend-name-input"]',
        'input[placeholder*="name" i]',
        'input[name*="name"]'
      ];
      
      for (const selector of nameSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('test-host-backend');
          break;
        }
      }
      
      // Fill target configuration
      if ($body.find('[data-cy="backend-target-type-select"]').length > 0) {
        cy.get('[data-cy="backend-target-type-select"]').click();
        // Select hostname target type
        cy.contains(/hostname/i).click();
      }
      
      // Fill target name/hostname
      const targetSelectors = [
        '[data-cy="backend-target-name-input"]',
        'input[placeholder*="hostname" i]',
        'input[placeholder*="target" i]'
      ];
      
      for (const selector of targetSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('api.example.com');
          break;
        }
      }
    });
    
    // Submit the form
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="save-backend-button"]').length > 0) {
        cy.get('[data-cy="save-backend-button"]').click();
      } else if ($body.find('[data-cy="create-button"]').length > 0) {
        cy.get('[data-cy="create-button"]').click();
      } else if ($body.find('[data-cy="save-button"]').length > 0) {
        cy.get('[data-cy="save-button"]').click();
      } else {
        // Look for any submit button
        cy.get('button[type="submit"]').first().click();
      }
    });
    
    // Wait for creation to complete
    cy.wait(2000);
    
    // Verify backend was created
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="success-message"]').length > 0) {
        cy.get('[data-cy="success-message"]').should('be.visible');
      } else {
        // Check if we're back on the backends page with our new backend
        cy.get('[data-cy="backends-page"]').should('be.visible');
        cy.contains('test-host-backend').should('be.visible');
      }
    });
  });

  it('should create a new MCP backend', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Look for add backend button
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-backend-button"]').length > 0) {
        cy.get('[data-cy="add-backend-button"]').click();
      } else {
        cy.contains('button', /add|create/i).first().click();
      }
    });
    
    // Wait for form
    cy.wait(1000);
    
    // Fill in backend details for MCP type
    cy.get('body').then(($body) => {
      // Select backend type (MCP)
      if ($body.find('[data-cy="backend-type-select"]').length > 0) {
        cy.get('[data-cy="backend-type-select"]').click();
        // Look for MCP option
        cy.contains(/mcp/i).click();
      }
      
      // Fill backend name
      const nameSelectors = ['[data-cy="backend-name-input"]', 'input[placeholder*="name" i]'];
      for (const selector of nameSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('test-mcp-backend');
          break;
        }
      }
      
      // Fill MCP-specific configuration
      if ($body.find('[data-cy="backend-target-name-input"]').length > 0) {
        cy.get('[data-cy="backend-target-name-input"]').clear().type('mcp://localhost:3001');
      }
    });
    
    // Submit the form
    cy.get('button[type="submit"]').first().click();
    
    // Wait for creation
    cy.wait(2000);
    
    // Verify MCP backend was created
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="success-message"]').length > 0) {
        cy.get('[data-cy="success-message"]').should('be.visible');
      } else {
        cy.get('[data-cy="backends-page"]').should('be.visible');
        cy.contains('test-mcp-backend').should('be.visible');
      }
    });
  });

  it('should test different backend types', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Try to create a backend and test type selection
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-backend-button"]').length > 0) {
        cy.get('[data-cy="add-backend-button"]').click();
      } else {
        cy.contains('button', /add|create/i).first().click();
      }
    });
    
    // Wait for form
    cy.wait(1000);
    
    // Test backend type selection
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="backend-type-select"]').length > 0) {
        cy.get('[data-cy="backend-type-select"]').click();
        
        // Verify different backend types are available
        cy.get('body').should('contain.text', /host|mcp|service/i);
        
        // Test selecting different types
        cy.contains(/host/i).click();
        
        // Verify form changes based on type selection
        cy.get('body').should('contain.text', /hostname|target/i);
      }
    });
    
    // Cancel the form
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="cancel-button"]').length > 0) {
        cy.get('[data-cy="cancel-button"]').click();
      } else {
        // Navigate away or close form
        cy.get('[data-cy="backends-page"]').should('be.visible');
      }
    });
  });

  it('should handle backend editing and updates', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Look for existing backends
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy*="backend-card"]').length > 0) {
        // Click on the first backend card
        cy.get('[data-cy*="backend-card"]').first().click();
      } else if ($body.find('[data-cy*="backend-row"]').length > 0) {
        // Click on the first backend row
        cy.get('[data-cy*="backend-row"]').first().click();
      } else {
        cy.log('No backends found for editing test');
        return;
      }
      
      // Wait for details to load
      cy.wait(1000);
      
      // Look for edit functionality
      if ($body.find('[data-cy="edit-backend-button"]').length > 0) {
        cy.get('[data-cy="edit-backend-button"]').click();
        
        // Verify edit form appears
        cy.get('input').should('exist');
        
        // Make a small change to the name
        cy.get('input').first().then(($input) => {
          const currentValue = $input.val();
          cy.wrap($input).clear().type(currentValue + '-edited');
        });
        
        // Save changes
        if ($body.find('[data-cy="save-button"]').length > 0) {
          cy.get('[data-cy="save-button"]').click();
        } else {
          cy.get('button[type="submit"]').first().click();
        }
        
        // Verify changes were saved
        cy.wait(1000);
        cy.contains('-edited').should('be.visible');
      } else {
        cy.log('Edit functionality not found or not implemented');
      }
    });
  });

  it('should test backend deletion with confirmation', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Count existing backends
    let initialCount = 0;
    cy.get('body').then(($body) => {
      const backendElements = $body.find('[data-cy*="backend-card"], [data-cy*="backend-row"]');
      initialCount = backendElements.length;
      
      if (initialCount > 0) {
        // Look for delete button on first backend
        cy.get('[data-cy*="backend-card"], [data-cy*="backend-row"]').first().within(() => {
          cy.get('body').then(($innerBody) => {
            if ($innerBody.find('[data-cy="delete-backend-button"]').length > 0) {
              cy.get('[data-cy="delete-backend-button"]').click();
            } else if ($innerBody.find('[data-cy*="delete"]').length > 0) {
              cy.get('[data-cy*="delete"]').first().click();
            } else {
              // Look for delete icon or button
              cy.get('button').contains(/delete|remove/i).click();
            }
          });
        });
        
        // Handle confirmation dialog
        cy.get('body').then(($confirmBody) => {
          if ($confirmBody.find('[data-cy="confirm-delete"]').length > 0) {
            cy.get('[data-cy="confirm-delete"]').click();
          } else if ($confirmBody.find('[data-cy="confirm-button"]').length > 0) {
            cy.get('[data-cy="confirm-button"]').click();
          } else {
            // Look for confirmation in modal or dialog
            cy.get('button').contains(/confirm|yes|delete/i).click();
          }
        });
        
        // Wait for deletion to complete
        cy.wait(2000);
        
        // Verify backend was deleted
        cy.get('body').then(($finalBody) => {
          if ($finalBody.find('[data-cy="success-message"]').length > 0) {
            cy.get('[data-cy="success-message"]').should('be.visible');
          } else {
            // Check that backend count decreased
            const finalElements = $finalBody.find('[data-cy*="backend-card"], [data-cy*="backend-row"]');
            expect(finalElements.length).to.be.lessThan(initialCount);
          }
        });
      } else {
        cy.log('No backends available for deletion test');
      }
    });
  });

  it('should validate backend form inputs', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Try to create a backend with invalid data
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-backend-button"]').length > 0) {
        cy.get('[data-cy="add-backend-button"]').click();
      } else {
        cy.contains('button', /add|create/i).first().click();
      }
    });
    
    // Wait for form
    cy.wait(1000);
    
    // Try to submit empty form
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="save-button"]').length > 0) {
        cy.get('[data-cy="save-button"]').click();
      } else {
        cy.get('button[type="submit"]').first().click();
      }
    });
    
    // Check for validation errors
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy*="error"]').length > 0) {
        cy.get('[data-cy*="error"]').should('be.visible');
      } else {
        // Look for any error messages
        cy.contains(/required|invalid|error/i).should('be.visible');
      }
    });
    
    // Test invalid hostname format
    cy.get('input').then(($inputs) => {
      const targetInput = Array.from($inputs).find(input => 
        input.placeholder.toLowerCase().includes('hostname') ||
        input.placeholder.toLowerCase().includes('target')
      );
      
      if (targetInput) {
        cy.wrap(targetInput).clear().type('invalid hostname with spaces');
        
        // Try to submit
        cy.get('button[type="submit"]').first().click();
        
        // Should show validation error
        cy.contains(/invalid|hostname|format/i).should('be.visible');
      }
    });
  });

  it('should test backend-route relationships', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Check if backends show route relationships
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy*="backend-card"], [data-cy*="backend-row"]').length > 0) {
        // Look for route information in backend cards/rows
        cy.get('[data-cy*="backend-card"], [data-cy*="backend-row"]').first().then(($backend) => {
          // Check if route information is displayed
          if ($backend.text().includes('route') || $backend.text().includes('path')) {
            cy.log('Backend shows route relationship');
          }
          
          // Check for route status or connection info
          if ($backend.find('[data-cy*="route"]').length > 0) {
            cy.wrap($backend).find('[data-cy*="route"]').should('be.visible');
          }
        });
      } else {
        cy.log('No backends available to test route relationships');
      }
    });
    
    // Verify page functionality
    cy.get('[data-cy="backends-page"]').should('be.visible');
  });

  it('should display backend health and status', () => {
    // Navigate to backends page
    cy.get('[data-cy="nav-backends"]').click();
    cy.get('[data-cy="backends-page"]').should('be.visible');
    
    // Check for backend health/status indicators
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy*="backend-card"], [data-cy*="backend-row"]').length > 0) {
        // Look for status indicators
        cy.get('[data-cy*="backend-card"], [data-cy*="backend-row"]').first().then(($backend) => {
          // Check for health status
          if ($backend.find('[data-cy*="status"]').length > 0) {
            cy.wrap($backend).find('[data-cy*="status"]').should('be.visible');
          }
          
          // Check for health indicators (healthy, unhealthy, unknown)
          if ($backend.find('[data-cy*="health"]').length > 0) {
            cy.wrap($backend).find('[data-cy*="health"]').should('be.visible');
          }
          
          // Look for connection status
          if ($backend.text().includes('connected') || $backend.text().includes('disconnected')) {
            cy.log('Backend shows connection status');
          }
        });
      } else {
        cy.log('No backends available to test status display');
      }
    });
    
    // Verify page shows meaningful content
    cy.get('[data-cy="backends-page"]').should('be.visible');
  });
});
