import { useEffect, useState } from 'react'
import { BaseReporter } from './BaseReporter.js'

function Component() {
  return (
    <div className="foo">
      <p id="header">Howdy</p>
      <p className="foo">This is some content</p>
      <button id="one" />
      <button id="two" />
      <button bool={true} />
      <button re={/abc/} />
    </div>
  )
}
