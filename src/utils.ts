export function stringMatches(str: string, pattern: string | RegExp) {
  return pattern instanceof RegExp ? pattern.test(str) : str === pattern
}
