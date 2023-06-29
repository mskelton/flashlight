import { Result } from '../analysis.js'
import { BaseReporter } from './BaseReporter.js'

export class QuickfixReporter extends BaseReporter {
  progress(result: Result) {
    const span = result.specifiers[0].span
    console.log(span)
    console.log(result.source.slice(span.start, span.end))
    console.log(`${result.filename}:${result.specifiers[0].span.start}`)
  }
}
