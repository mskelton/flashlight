#!/usr/bin/env node
import swc from '@swc/core'
import { globby } from 'globby'
import { join } from 'node:path'
import yargs from 'yargs/yargs'

const argv = await yargs(process.argv.slice(2))
  .alias('c', 'cwd')
  .alias('n', 'name')
  .alias('s', 'source')
  .options({
    cwd: { default: process.cwd(), type: 'string' },
    name: { type: 'string' },
    source: { type: 'string' },
  }).argv

const paths = await globby(['**/*.{js,jsx,ts,tsx}'], { cwd: argv.cwd })
const filenames: string[] = []

for await (const path of paths) {
  const filename = join(argv.cwd, path)
  const res = await swc.parseFile(filename, {
    syntax: 'typescript',
    tsx: true,
  })

  const imports = res.body
    .filter(
      (node): node is swc.ImportDeclaration => node.type === 'ImportDeclaration'
    )
    .filter((node) => node.source.value === argv.source)

  const specifiers = imports.flatMap((node) =>
    node.specifiers
      .filter(
        (node): node is swc.NamedImportSpecifier =>
          node.type === 'ImportSpecifier'
      )
      .filter((node) => (node.imported ?? node.local).value === argv.name)
  )

  if (specifiers.length) {
    filenames.push(filename)
  }
}

console.log(filenames.join('\n'))
