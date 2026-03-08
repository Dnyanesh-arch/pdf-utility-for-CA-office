import { describe, expect, it } from 'vitest';
import { computePageRanges } from './range';

describe('computePageRanges', () => {
  it('computes sequential ranges', () => {
    expect(computePageRanges([2, 3, 1])).toEqual([
      { from: 1, to: 2 },
      { from: 3, to: 5 },
      { from: 6, to: 6 }
    ]);
  });
});
