import swc from '@swc/core'
import fs from 'node:fs/promises'
import { join } from 'node:path'
import { isImportDeclaration, isImportSpecifier } from './ast.js'
import { parse } from './parser.js'
import { Reporter } from './reporters/Reporter.js'
import { stringMatches } from './utils.js'
import { walk } from './walker.js'

export interface Result {
  filename: string
  source: string
  specifiers: swc.NamedImportSpecifier[]
}

export function getImports(program: swc.Module, source: string | RegExp) {
  return program.body
    .filter(isImportDeclaration)
    .filter((node) => stringMatches(node.source.value, source))
}

export function getSpecifiers(
  imports: swc.ImportDeclaration[],
  name: string | RegExp
) {
  return imports.flatMap((node) =>
    node.specifiers.filter(isImportSpecifier).filter((node) => {
      const identifier = node.imported ?? node.local
      return stringMatches(identifier.value, name)
    })
  )
}

export interface AnalyzeOptions {
  cwd: string
  name: string
  source: string
}

export async function analyze(
  reporter: Reporter,
  { cwd, name, source }: AnalyzeOptions
) {
  for await (const path of walk({ cwd })) {
    const filename = join(cwd, path.toString())
    const sourceCode = await fs.readFile(filename, 'utf8')
    const program = await parse(sourceCode)
    const imports = getImports(program, source)
    const specifiers = getSpecifiers(imports, name)

    if (!specifiers.length) return
    reporter.progress({
      filename,
      source: sourceCode,
      specifiers,
    })
  }
}
