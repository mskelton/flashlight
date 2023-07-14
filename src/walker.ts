import ignoreWalk from 'ignore-walk'

export interface WalkOptions {
  cwd: string
}

export function walk({ cwd }: WalkOptions) {
  return new ReadableStream({
    start(controller) {
      ignoreWalk(
        {
          ignoreFiles: ['.gitignore'],
          path: cwd,
        },
        (entry) => {
          console.log(entry)
          if (entry) {
            controller.enqueue(entry)
          }
        }
      )
    },
  })
}
