import { parse } from 'path'
import { cwd } from 'process'
import { pathToFileURL, fileURLToPath, URL } from 'url'

import { loadBinding } from '@node-rs/helper'

const { transform } = loadBinding(parse(fileURLToPath(import.meta.url)).dir, 'dog', '@napi-rs/dog')

const BASE_URL = pathToFileURL(`${cwd()}/`).href

export function transformSource(source, context, defaultTransformSource) {
  const { url } = context
  if (url.endsWith('.ts')) {
    return transform(source, url)
  }
  return defaultTransformSource(source, context, defaultTransformSource)
}

export function getFormat(request, context, rawGetFormat) {
  if (request.endsWith('.ts')) {
    return { format: 'module' }
  }
  return rawGetFormat(request, context, rawGetFormat)
}

export function resolve(specifier, context, rawResolve) {
  const { parentURL = BASE_URL } = context

  if (specifier.endsWith('.ts')) {
    return {
      url: new URL(specifier, parentURL).href,
    }
  }
  return rawResolve(specifier, context, rawResolve)
}
