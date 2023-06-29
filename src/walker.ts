import { globbyStream } from 'globby'

export interface WalkOptions {
  cwd: string
}

export function walk({ cwd }: WalkOptions) {
  return globbyStream(['**/*.{js,jsx,ts,tsx}'], {
    cwd,
    gitignore: true,
  })
}
