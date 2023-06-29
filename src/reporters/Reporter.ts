import { Result } from '../analysis.js'

export interface Reporter {
  end(): void
  progress(result: Result): void
  start(): void
}
