import { Result } from '../analysis.js'
import { Reporter } from './Reporter.js'

export class BaseReporter implements Reporter {
  public end() {}
  public progress(_: Result) {}
  public start() {}
}
