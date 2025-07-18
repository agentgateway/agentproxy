const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs').promises;

/**
 * WorkerManager - Manages Cypress worker processes
 * 
 * Key Features:
 * - Worker lifecycle management (spawn, monitor, terminate)
 * - Worker communication protocols
 * - Worker health monitoring
 * - Dynamic worker scaling
 */
class WorkerManager {
  constructor(options = {}) {
    this.baseDir = options.baseDir || '.';
    this.cypressCommand = options.cypressCommand || 'npx cypress run';
    this.maxWorkers = options.maxWorkers || 8;
    this.workerTimeout = options.workerTimeout || 300000; // 5 minutes
    
    this.workers = new Map();
    this.workerResults = new Map();
    this.nextWorkerId = 1;
    
    this.isShuttingDown = false;
    this.shutdownPromise = null;
  }

  /**
   * Spawn a worker for a specific test schedule
   */
  async spawnWorker(workerSchedule, options = {}) {
    const workerId = this.nextWorkerId++;
    const workerName = `worker-${workerId}`;
    
    console.log(`🚀 Spawning ${workerName} with ${workerSchedule.tests.length} tests...`);
    
    try {
      // Create worker-specific configuration
      const workerConfig = await this.createWorkerConfig(workerId, workerSchedule, options);
      
      // Build Cypress command
      const command = this.buildCypressCommand(workerConfig, options);
      
      // Spawn the process
      const worker = this.spawnCypressProcess(workerId, command, options);
      
      // Set up worker monitoring
      this.setupWorkerMonitoring(worker, workerSchedule);
      
      // Store worker info
      this.workers.set(workerId, {
        id: workerId,
        name: workerName,
        process: worker,
        schedule: workerSchedule,
        config: workerConfig,
        startTime: new Date(),
        status: 'running',
        output: [],
        errors: []
      });
      
      console.log(`✅ ${workerName} started (PID: ${worker.pid})`);
      return workerId;
      
    } catch (error) {
      console.error(`❌ Failed to spawn ${workerName}:`, error.message);
      throw error;
    }
  }

  /**
   * Create worker-specific Cypress configuration
   */
  async createWorkerConfig(workerId, workerSchedule, options) {
    const configDir = path.join(this.baseDir, 'cypress', 'workers');
    await fs.mkdir(configDir, { recursive: true });
    
    const configPath = path.resolve(configDir, `cypress.worker-${workerId}.config.js`);
    
    // Generate test spec pattern for this worker
    const testSpecs = workerSchedule.tests.map(test => test.path).join(',');
    
    const configContent = this.generateWorkerConfigContent(workerId, testSpecs, options);
    
    await fs.writeFile(configPath, configContent);
    
    return {
      configPath,
      testSpecs,
      workerId,
      reporterOptions: {
        outputDir: path.resolve(this.baseDir, 'cypress', 'results', `worker-${workerId}`),
        outputFile: `results-worker-${workerId}.json`
      }
    };
  }

  /**
   * Generate worker-specific Cypress configuration content
   */
  generateWorkerConfigContent(workerId, testSpecs, options) {
    const basePort = 3000 + workerId; // Unique port per worker
    
    return `
// Load base configuration
const baseConfig = require('../../cypress.config.ts');

// Override with worker-specific settings
module.exports = {
  ...baseConfig,
  e2e: {
    ...baseConfig.e2e,
    
    // Worker-specific test patterns
    specPattern: [${testSpecs.split(',').map(spec => `'${spec}'`).join(', ')}],
    
    // Worker-specific settings
    video: ${options.video !== false},
    screenshot: true,
    screenshotOnRunFailure: true,
    
    // Unique base URL per worker
    baseUrl: 'http://localhost:${basePort}',
    
    // Reporter configuration for this worker
    reporter: 'json',
    reporterOptions: {
      outputDir: 'cypress/results/worker-${workerId}',
      outputFile: 'results-worker-${workerId}.json'
    },
    
    // Environment variables
    env: {
      ...baseConfig.e2e?.env,
      WORKER_ID: ${workerId},
      PARALLEL_MODE: true
    },
    
    setupNodeEvents(on, config) {
      // Call base setup if it exists
      if (baseConfig.e2e?.setupNodeEvents) {
        config = baseConfig.e2e.setupNodeEvents(on, config) || config;
      }
      
      // Worker-specific setup
      on('task', {
        log(message) {
          console.log(\`[Worker ${workerId}] \${message}\`);
          return null;
        }
      });
      
      return config;
    }
  }
};
`;
  }

  /**
   * Build Cypress command for worker
   */
  buildCypressCommand(workerConfig, options) {
    const args = [
      'cypress', 'run',
      '--config-file', workerConfig.configPath,
      '--reporter', 'json',
      '--reporter-options', `outputDir=${workerConfig.reporterOptions.outputDir},outputFile=${workerConfig.reporterOptions.outputFile}`
    ];
    
    // Add browser option if specified
    if (options.browser) {
      args.push('--browser', options.browser);
    }
    
    // Add headless option
    if (options.headless !== false) {
      args.push('--headless');
    }
    
    // Add quiet option for cleaner output
    if (options.quiet !== false) {
      args.push('--quiet');
    }
    
    return {
      command: 'npx',
      args,
      options: {
        cwd: this.baseDir,
        stdio: ['pipe', 'pipe', 'pipe'],
        env: {
          ...process.env,
          CYPRESS_WORKER_ID: workerConfig.workerId,
          CYPRESS_PARALLEL_MODE: 'true'
        }
      }
    };
  }

  /**
   * Spawn Cypress process
   */
  spawnCypressProcess(workerId, command, options) {
    const worker = spawn(command.command, command.args, command.options);
    
    // Handle process errors
    worker.on('error', (error) => {
      console.error(`❌ Worker ${workerId} process error:`, error.message);
      this.handleWorkerError(workerId, error);
    });
    
    return worker;
  }

  /**
   * Set up worker monitoring
   */
  setupWorkerMonitoring(worker, workerSchedule) {
    const workerId = worker.pid; // Use PID as temporary ID
    
    // Capture stdout
    worker.stdout.on('data', (data) => {
      const output = data.toString();
      this.handleWorkerOutput(workerId, output, 'stdout');
    });
    
    // Capture stderr
    worker.stderr.on('data', (data) => {
      const output = data.toString();
      this.handleWorkerOutput(workerId, output, 'stderr');
    });
    
    // Handle process exit
    worker.on('close', (code, signal) => {
      this.handleWorkerExit(workerId, code, signal);
    });
    
    // Set up timeout
    const timeout = setTimeout(() => {
      console.warn(`⚠️ Worker ${workerId} timeout after ${this.workerTimeout}ms`);
      this.terminateWorker(workerId, 'timeout');
    }, this.workerTimeout);
    
    worker.timeout = timeout;
  }

  /**
   * Handle worker output
   */
  handleWorkerOutput(workerId, output, type) {
    const workerInfo = this.findWorkerByPid(workerId);
    if (!workerInfo) return;
    
    // Store output
    workerInfo.output.push({
      type,
      content: output,
      timestamp: new Date()
    });
    
    // Log all output for debugging
    console.log(`[Worker ${workerInfo.id}] ${type}: ${output.trim()}`);
    
    // Check for errors
    if (type === 'stderr' && output.trim()) {
      workerInfo.errors.push({
        content: output,
        timestamp: new Date()
      });
    }
  }

  /**
   * Handle worker exit
   */
  handleWorkerExit(pid, code, signal) {
    const workerInfo = this.findWorkerByPid(pid);
    if (!workerInfo) return;
    
    const workerId = workerInfo.id;
    const endTime = new Date();
    const duration = endTime - workerInfo.startTime;
    
    // Clear timeout
    if (workerInfo.process.timeout) {
      clearTimeout(workerInfo.process.timeout);
    }
    
    // Update worker status
    workerInfo.status = code === 0 ? 'completed' : 'failed';
    workerInfo.endTime = endTime;
    workerInfo.duration = duration;
    workerInfo.exitCode = code;
    workerInfo.signal = signal;
    
    // Log completion
    if (code === 0) {
      console.log(`✅ Worker ${workerId} completed successfully (${(duration / 1000).toFixed(1)}s)`);
    } else {
      console.error(`❌ Worker ${workerId} failed with code ${code} (${(duration / 1000).toFixed(1)}s)`);
    }
    
    // Process results
    this.processWorkerResults(workerId);
  }

  /**
   * Handle worker error
   */
  handleWorkerError(workerId, error) {
    const workerInfo = this.workers.get(workerId);
    if (!workerInfo) return;
    
    workerInfo.status = 'error';
    workerInfo.error = error;
    
    console.error(`❌ Worker ${workerId} encountered error:`, error.message);
  }

  /**
   * Find worker by process PID
   */
  findWorkerByPid(pid) {
    for (const [workerId, workerInfo] of this.workers) {
      if (workerInfo.process.pid === pid) {
        return workerInfo;
      }
    }
    return null;
  }

  /**
   * Process worker results
   */
  async processWorkerResults(workerId) {
    const workerInfo = this.workers.get(workerId);
    if (!workerInfo) return;
    
    try {
      // Read Cypress results file
      const resultsPath = path.join(
        workerInfo.config.reporterOptions.outputDir,
        workerInfo.config.reporterOptions.outputFile
      );
      
      let results = null;
      try {
        const resultsContent = await fs.readFile(resultsPath, 'utf8');
        results = JSON.parse(resultsContent);
      } catch (error) {
        console.warn(`⚠️ Could not read results for worker ${workerId}:`, error.message);
      }
      
      // Store processed results
      this.workerResults.set(workerId, {
        workerId,
        schedule: workerInfo.schedule,
        status: workerInfo.status,
        duration: workerInfo.duration,
        exitCode: workerInfo.exitCode,
        results,
        output: workerInfo.output,
        errors: workerInfo.errors,
        startTime: workerInfo.startTime,
        endTime: workerInfo.endTime
      });
      
    } catch (error) {
      console.error(`❌ Error processing results for worker ${workerId}:`, error.message);
    }
  }

  /**
   * Run all workers for a schedule
   */
  async runWorkers(schedule, options = {}) {
    console.log(`🚀 Starting ${schedule.length} workers...`);
    
    const workerPromises = [];
    
    // Spawn all workers
    for (const workerSchedule of schedule) {
      const workerPromise = this.spawnWorker(workerSchedule, options)
        .then(workerId => this.waitForWorker(workerId))
        .catch(error => {
          console.error('Worker spawn error:', error);
          return { error: error.message };
        });
      
      workerPromises.push(workerPromise);
      
      // Small delay between spawns to avoid overwhelming the system
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
    
    // Wait for all workers to complete
    console.log('⏳ Waiting for all workers to complete...');
    const results = await Promise.all(workerPromises);
    
    console.log('✅ All workers completed');
    return this.consolidateResults();
  }

  /**
   * Wait for a specific worker to complete
   */
  async waitForWorker(workerId) {
    return new Promise((resolve) => {
      const checkWorker = () => {
        const workerInfo = this.workers.get(workerId);
        if (!workerInfo) {
          resolve({ error: 'Worker not found' });
          return;
        }
        
        if (workerInfo.status === 'completed' || workerInfo.status === 'failed' || workerInfo.status === 'error') {
          resolve(this.workerResults.get(workerId) || workerInfo);
        } else {
          setTimeout(checkWorker, 1000);
        }
      };
      
      checkWorker();
    });
  }

  /**
   * Terminate a specific worker
   */
  async terminateWorker(workerId, reason = 'manual') {
    const workerInfo = this.workers.get(workerId);
    if (!workerInfo) return;
    
    console.log(`🛑 Terminating worker ${workerId} (reason: ${reason})`);
    
    try {
      // Clear timeout
      if (workerInfo.process.timeout) {
        clearTimeout(workerInfo.process.timeout);
      }
      
      // Kill the process
      workerInfo.process.kill('SIGTERM');
      
      // Wait a bit, then force kill if necessary
      setTimeout(() => {
        if (!workerInfo.process.killed) {
          workerInfo.process.kill('SIGKILL');
        }
      }, 5000);
      
      workerInfo.status = 'terminated';
      workerInfo.terminationReason = reason;
      
    } catch (error) {
      console.error(`❌ Error terminating worker ${workerId}:`, error.message);
    }
  }

  /**
   * Terminate all workers
   */
  async terminateAllWorkers(reason = 'shutdown') {
    if (this.isShuttingDown) {
      return this.shutdownPromise;
    }
    
    this.isShuttingDown = true;
    console.log('🛑 Terminating all workers...');
    
    this.shutdownPromise = Promise.all(
      Array.from(this.workers.keys()).map(workerId => 
        this.terminateWorker(workerId, reason)
      )
    );
    
    await this.shutdownPromise;
    console.log('✅ All workers terminated');
    
    return this.shutdownPromise;
  }

  /**
   * Consolidate results from all workers
   */
  consolidateResults() {
    const allResults = Array.from(this.workerResults.values());
    
    const summary = {
      totalWorkers: allResults.length,
      successfulWorkers: allResults.filter(r => r.status === 'completed').length,
      failedWorkers: allResults.filter(r => r.status === 'failed').length,
      errorWorkers: allResults.filter(r => r.status === 'error').length,
      totalDuration: Math.max(...allResults.map(r => r.duration || 0)),
      averageDuration: allResults.reduce((sum, r) => sum + (r.duration || 0), 0) / allResults.length,
      results: allResults
    };
    
    return summary;
  }

  /**
   * Get worker status
   */
  getWorkerStatus(workerId) {
    const workerInfo = this.workers.get(workerId);
    if (!workerInfo) return null;
    
    return {
      id: workerId,
      status: workerInfo.status,
      startTime: workerInfo.startTime,
      duration: workerInfo.endTime ? workerInfo.endTime - workerInfo.startTime : Date.now() - workerInfo.startTime,
      testCount: workerInfo.schedule.tests.length,
      pid: workerInfo.process.pid
    };
  }

  /**
   * Get all workers status
   */
  getAllWorkersStatus() {
    return Array.from(this.workers.keys()).map(workerId => this.getWorkerStatus(workerId));
  }

  /**
   * Cleanup worker files
   */
  async cleanup() {
    try {
      const workersDir = path.join(this.baseDir, 'cypress', 'workers');
      await fs.rm(workersDir, { recursive: true });
      console.log('🧹 Cleaned up worker configuration files');
    } catch (error) {
      console.warn('⚠️ Could not clean up worker files:', error.message);
    }
  }
}

module.exports = WorkerManager;
