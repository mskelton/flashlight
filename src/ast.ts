import swc from '@swc/core'

export const isImportDeclaration = (
  node: swc.Node
): node is swc.ImportDeclaration => node.type === 'ImportDeclaration'

export const isImportSpecifier = (
  node: swc.Node
): node is swc.NamedImportSpecifier => node.type === 'ImportSpecifier'
