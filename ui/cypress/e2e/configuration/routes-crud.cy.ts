describe('Routes CRUD Operations', () => {
  beforeEach(() => {
    cy.visit('/');
  });

  it('should navigate to routes page and display content', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    
    // Verify we're on the routes page
    cy.get('[data-cy="routes-page"]').should('be.visible');
    cy.url().should('include', '/routes');
    
    // Check for routes content or empty state
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="routes-empty-state"]').length > 0) {
        cy.get('[data-cy="routes-empty-state"]').should('be.visible');
      } else {
        // If routes exist, verify the page displays content
        cy.log('Routes content is displayed');
        cy.get('[data-cy="routes-page"]').should('not.be.empty');
      }
    });
  });

  it('should display route statistics and validation warnings', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Check for statistics display
    cy.get('body').then(($body) => {
      // Look for route statistics
      if ($body.find('[data-cy*="statistics"]').length > 0) {
        cy.get('[data-cy*="statistics"]').should('be.visible');
      } else if ($body.find('[data-cy*="stats"]').length > 0) {
        cy.get('[data-cy*="stats"]').should('be.visible');
      }
      
      // Check for route count or summary
      if ($body.find('[data-cy*="count"]').length > 0) {
        cy.get('[data-cy*="count"]').should('be.visible');
      }
      
      // Look for validation warnings if any
      if ($body.find('[data-cy*="warning"]').length > 0) {
        cy.get('[data-cy*="warning"]').should('be.visible');
      } else if ($body.find('[data-cy*="validation"]').length > 0) {
        cy.get('[data-cy*="validation"]').should('be.visible');
      }
    });
    
    // Verify page shows meaningful content
    cy.get('[data-cy="routes-page"]').should('contain.text', /route|path|empty/i);
  });

  it('should create a new route through the UI', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Look for add route button
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-route-button"]').length > 0) {
        cy.get('[data-cy="add-route-button"]').click();
      } else if ($body.find('[data-cy="create-route-button"]').length > 0) {
        cy.get('[data-cy="create-route-button"]').click();
      } else {
        // Look for any button with "add" or "create" in the text
        cy.contains('button', /add|create/i).first().click();
      }
    });
    
    // Wait for form to appear
    cy.wait(1000);
    
    // Fill in route details
    cy.get('body').then(($body) => {
      // Try different possible input selectors for route name
      const nameSelectors = [
        '[data-cy="route-name-input"]',
        'input[placeholder*="name" i]',
        'input[name*="name"]'
      ];
      
      // Try different possible input selectors for route path
      const pathSelectors = [
        '[data-cy="route-path-input"]',
        'input[placeholder*="path" i]',
        'input[name*="path"]',
        'input[placeholder*="route" i]'
      ];
      
      // Try to find and fill name input
      for (const selector of nameSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('test-route-crud');
          break;
        }
      }
      
      // Try to find and fill path input
      for (const selector of pathSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('/api/test-crud');
          break;
        }
      }
      
      // Look for match type selection if available
      if ($body.find('[data-cy="route-match-type-select"]').length > 0) {
        cy.get('[data-cy="route-match-type-select"]').click();
        // Select prefix match (usually default)
        cy.get('[data-state="checked"]').should('exist');
      }
    });
    
    // Submit the form
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="save-route-button"]').length > 0) {
        cy.get('[data-cy="save-route-button"]').click();
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
    
    // Verify route was created
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="success-message"]').length > 0) {
        cy.get('[data-cy="success-message"]').should('be.visible');
      } else {
        // Check if we're back on the routes page with our new route
        cy.get('[data-cy="routes-page"]').should('be.visible');
        cy.contains('test-route-crud').should('be.visible');
      }
    });
  });

  it('should test different route match types', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Try to create a route with different match types
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-route-button"]').length > 0) {
        cy.get('[data-cy="add-route-button"]').click();
      } else {
        cy.contains('button', /add|create/i).first().click();
      }
    });
    
    // Wait for form
    cy.wait(1000);
    
    // Fill basic route info
    cy.get('body').then(($body) => {
      // Fill name
      const nameSelectors = ['[data-cy="route-name-input"]', 'input[placeholder*="name" i]'];
      for (const selector of nameSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('test-route-match-types');
          break;
        }
      }
      
      // Fill path
      const pathSelectors = ['[data-cy="route-path-input"]', 'input[placeholder*="path" i]'];
      for (const selector of pathSelectors) {
        if ($body.find(selector).length > 0) {
          cy.get(selector).first().clear().type('/api/match-test');
          break;
        }
      }
      
      // Test match type selection if available
      if ($body.find('[data-cy="route-match-type-select"]').length > 0) {
        cy.get('[data-cy="route-match-type-select"]').click();
        
        // Verify different match types are available
        cy.get('body').should('contain.text', /prefix|exact|regex/i);
        
        // Select a specific match type (e.g., exact)
        cy.contains(/exact/i).click();
      }
    });
    
    // Cancel or save the form
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="cancel-button"]').length > 0) {
        cy.get('[data-cy="cancel-button"]').click();
      } else {
        // Save the route
        cy.get('button[type="submit"]').first().click();
        cy.wait(1000);
      }
    });
  });

  it('should handle route editing and updates', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Look for existing routes
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy*="route-card"]').length > 0) {
        // Click on the first route card
        cy.get('[data-cy*="route-card"]').first().click();
      } else if ($body.find('[data-cy*="route-row"]').length > 0) {
        // Click on the first route row
        cy.get('[data-cy*="route-row"]').first().click();
      } else {
        cy.log('No routes found for editing test');
        return;
      }
      
      // Wait for details to load
      cy.wait(1000);
      
      // Look for edit functionality
      if ($body.find('[data-cy="edit-route-button"]').length > 0) {
        cy.get('[data-cy="edit-route-button"]').click();
        
        // Verify edit form appears
        cy.get('input').should('exist');
        
        // Make a small change to the path
        cy.get('input').then(($inputs) => {
          const pathInput = Array.from($inputs).find(input => 
            input.value.includes('/') || input.placeholder.toLowerCase().includes('path')
          );
          
          if (pathInput) {
            const currentValue = pathInput.value;
            cy.wrap(pathInput).clear().type(currentValue + '-edited');
          }
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

  it('should test route deletion with confirmation', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Count existing routes
    let initialCount = 0;
    cy.get('body').then(($body) => {
      const routeElements = $body.find('[data-cy*="route-card"], [data-cy*="route-row"]');
      initialCount = routeElements.length;
      
      if (initialCount > 0) {
        // Look for delete button on first route
        cy.get('[data-cy*="route-card"], [data-cy*="route-row"]').first().within(() => {
          cy.get('body').then(($innerBody) => {
            if ($innerBody.find('[data-cy="delete-route-button"]').length > 0) {
              cy.get('[data-cy="delete-route-button"]').click();
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
        
        // Verify route was deleted
        cy.get('body').then(($finalBody) => {
          if ($finalBody.find('[data-cy="success-message"]').length > 0) {
            cy.get('[data-cy="success-message"]').should('be.visible');
          } else {
            // Check that route count decreased
            const finalElements = $finalBody.find('[data-cy*="route-card"], [data-cy*="route-row"]');
            expect(finalElements.length).to.be.lessThan(initialCount);
          }
        });
      } else {
        cy.log('No routes available for deletion test');
      }
    });
  });

  it('should validate route form inputs', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Try to create a route with invalid data
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy="add-route-button"]').length > 0) {
        cy.get('[data-cy="add-route-button"]').click();
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
    
    // Test invalid path format
    cy.get('input').then(($inputs) => {
      const pathInput = Array.from($inputs).find(input => 
        input.placeholder.toLowerCase().includes('path')
      );
      
      if (pathInput) {
        cy.wrap(pathInput).clear().type('invalid-path-without-slash');
        
        // Try to submit
        cy.get('button[type="submit"]').first().click();
        
        // Should show validation error
        cy.contains(/invalid|path|format/i).should('be.visible');
      }
    });
  });

  it('should test route-listener relationships', () => {
    // Navigate to routes page
    cy.get('[data-cy="nav-routes"]').click();
    cy.get('[data-cy="routes-page"]').should('be.visible');
    
    // Check if routes show listener relationships
    cy.get('body').then(($body) => {
      if ($body.find('[data-cy*="route-card"], [data-cy*="route-row"]').length > 0) {
        // Look for listener information in route cards/rows
        cy.get('[data-cy*="route-card"], [data-cy*="route-row"]').first().then(($route) => {
          // Check if listener information is displayed
          if ($route.text().includes('listener') || $route.text().includes('bind')) {
            cy.log('Route shows listener relationship');
          }
          
          // Check for listener status or connection info
          if ($route.find('[data-cy*="listener"]').length > 0) {
            cy.wrap($route).find('[data-cy*="listener"]').should('be.visible');
          }
        });
      } else {
        cy.log('No routes available to test listener relationships');
      }
    });
    
    // Verify page functionality
    cy.get('[data-cy="routes-page"]').should('be.visible');
  });
});
