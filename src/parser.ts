import swc from '@swc/core'

export function parse(filename: string, sourceCode: string) {
  return swc.parse(sourceCode, {
    decorators: true,
    syntax: /\.jsx?$/.test(filename) ? 'ecmascript' : 'typescript',
    target: 'esnext',
    tsx: filename.endsWith('.tsx'),
  })
}
