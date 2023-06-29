import { ConsoleReporter } from './ConsoleReporter.js'
import { QuickfixReporter } from './QuickfixReporter.js'
import { Reporter } from './Reporter.js'

const reporters = {
  console: ConsoleReporter,
  quickfix: QuickfixReporter,
}

export function getReporter(reporter: string | undefined): Reporter {
  const Klass = reporters[reporter as keyof typeof reporters] ?? ConsoleReporter
  return new Klass()
}
