import swc from '@swc/core'

export function parse(sourceCode: string) {
  return swc.parse(sourceCode, {
    syntax: 'typescript',
    target: 'esnext',
    tsx: true,
  })
}
