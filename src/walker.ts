import { globby, globbyStream } from 'globby'
import path from 'path'

export interface WalkOptions {
  cwd: string
}

const dirs = [
  'application-deployments',
  'auth-token',
  'backend-monorepo',
  'bob',
  'bufferline.nvim',
  'data-pipeline',
  'dtsfmt',
  'e2e',
  'eslint-annotate-action',
  'federato-monorepo',
  'flashlight',
  'github-actions',
  'key-config',
  'mskelton.dev',
  'react-spectrum',
  'router',
  'server',
  'tailwind-variants',
]

export async function walk({ cwd }: WalkOptions) {
  const ress = await Promise.all(
    dirs.map((dir) =>
      globby(['**/*.{ts,tsx}'], {
        cwd: path.join(cwd, dir),
        gitignore: true,
      }).then((paths) =>
        paths.map((file) => path.resolve(path.join(cwd, dir, file)))
      )
    )
  )

  return ress.flat()
}
