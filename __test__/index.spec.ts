import test from 'ava'

import { register } from '../index'

test('should work', (t) => {
  t.notThrows(() => register())
})
