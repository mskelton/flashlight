#!/usr/bin/env node
import { createRequire } from 'module'
import yargs from 'yargs/yargs'
import { analyze } from './analysis.js'
import { getReporter } from './reporters/index.js'

const require = createRequire(import.meta.url)

const argv = await yargs(process.argv.slice(2))
  .version(require('../package.json').version)
  .options({
    cwd: {
      alias: 'c',
      default: process.cwd(),
      type: 'string',
    },
    name: {
      alias: 'n',
      requiresArg: true,
      type: 'string',
    },
    reporter: {
      choices: ['console', 'quickfix'],
      default: 'console',
      type: 'string',
    },
    source: {
      alias: 's',
      requiresArg: true,
      type: 'string',
    },
  })
  .alias('h', 'help')
  .alias('v', 'version')
  .demandOption('source')
  .demandOption('name').argv

const reporter = getReporter(argv.reporter)
reporter.start()
await analyze(reporter, argv)
reporter.end()
