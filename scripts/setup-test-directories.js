#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * Setup Test Directories
 * 
 * This script ensures all necessary directories exist for Cypress E2E tests,
 * including video recording directories and results directories.
 */

const UI_DIR = path.join(__dirname, '..', 'ui');
const CYPRESS_DIR = path.join(UI_DIR, 'cypress');

// Define all the directories that need to exist
const REQUIRED_DIRECTORIES = [
  // Base cypress directories
  'cypress/videos',
  'cypress/screenshots',
  'cypress/results',
  'cypress/reports',
  
  // Video directories for each test category
  'cypress/videos/foundation',
  'cypress/videos/navigation',
  'cypress/videos/setup-wizard',
  'cypress/videos/configuration',
  'cypress/videos/playground',
  'cypress/videos/integration',
  'cypress/videos/error-handling',
  
  // Screenshot directories for each test category
  'cypress/screenshots/foundation',
  'cypress/screenshots/navigation',
  'cypress/screenshots/setup-wizard',
  'cypress/screenshots/configuration',
  'cypress/screenshots/playground',
  'cypress/screenshots/integration',
  'cypress/screenshots/error-handling',
  
  // Results directories for parallel test execution
  'cypress/results/worker-1',
  'cypress/results/worker-2',
  'cypress/results/worker-3',
  'cypress/results/worker-4',
  'cypress/results/worker-5',
  'cypress/results/worker-6',
  'cypress/results/worker-7',
  'cypress/results/worker-8',
];

/**
 * Create a directory if it doesn't exist
 * @param {string} dirPath - Path to the directory
 */
function ensureDirectoryExists(dirPath) {
  const fullPath = path.join(UI_DIR, dirPath);
  
  if (!fs.existsSync(fullPath)) {
    try {
      fs.mkdirSync(fullPath, { recursive: true });
      console.log(`✅ Created directory: ${dirPath}`);
    } catch (error) {
      console.error(`❌ Failed to create directory ${dirPath}:`, error.message);
      process.exit(1);
    }
  } else {
    console.log(`📁 Directory already exists: ${dirPath}`);
  }
}

/**
 * Clean up old test artifacts
 */
function cleanupOldArtifacts() {
  const artifactDirs = [
    path.join(UI_DIR, 'cypress/videos'),
    path.join(UI_DIR, 'cypress/screenshots'),
    path.join(UI_DIR, 'cypress/results'),
  ];
  
  artifactDirs.forEach(dir => {
    if (fs.existsSync(dir)) {
      try {
        // Remove all files in the directory but keep the directory structure
        const files = fs.readdirSync(dir, { withFileTypes: true });
        files.forEach(file => {
          if (file.isFile()) {
            const filePath = path.join(dir, file.name);
            fs.unlinkSync(filePath);
          }
        });
        console.log(`🧹 Cleaned up artifacts in: ${path.relative(UI_DIR, dir)}`);
      } catch (error) {
        console.warn(`⚠️ Warning: Could not clean ${dir}:`, error.message);
      }
    }
  });
}

/**
 * Verify Cypress configuration
 */
function verifyCypressConfig() {
  const configPath = path.join(UI_DIR, 'cypress.config.ts');
  
  if (!fs.existsSync(configPath)) {
    console.error('❌ Cypress configuration file not found:', configPath);
    process.exit(1);
  }
  
  console.log('✅ Cypress configuration file found');
}

/**
 * Main execution
 */
function main() {
  console.log('🚀 Setting up test directories...\n');
  
  // Verify we're in the right place
  if (!fs.existsSync(UI_DIR)) {
    console.error('❌ UI directory not found:', UI_DIR);
    process.exit(1);
  }
  
  // Verify Cypress config exists
  verifyCypressConfig();
  
  // Clean up old artifacts (optional)
  if (process.argv.includes('--clean')) {
    console.log('🧹 Cleaning up old test artifacts...');
    cleanupOldArtifacts();
    console.log('');
  }
  
  // Create all required directories
  console.log('📁 Creating required directories...');
  REQUIRED_DIRECTORIES.forEach(ensureDirectoryExists);
  
  console.log('\n✅ Test directory setup complete!');
  console.log('📊 Summary:');
  console.log(`   - Created ${REQUIRED_DIRECTORIES.length} directories`);
  console.log('   - Video recording directories ready');
  console.log('   - Screenshot directories ready');
  console.log('   - Parallel test result directories ready');
  console.log('\n🎯 You can now run tests without directory-related errors.');
}

// Run the script
if (require.main === module) {
  main();
}

module.exports = {
  ensureDirectoryExists,
  cleanupOldArtifacts,
  verifyCypressConfig,
  REQUIRED_DIRECTORIES,
};
