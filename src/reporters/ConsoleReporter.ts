import { Result } from '../analysis.js'
import { BaseReporter } from './BaseReporter.js'

export class ConsoleReporter extends BaseReporter {
  progress(result: Result) {
    console.log(result.filename)
  }
}
