import { describe, expect, it } from 'vitest';
import { buildNamedOutput, sanitizeFileName } from './naming';

describe('naming', () => {
  it('sanitizes filesystem-invalid chars', () => {
    expect(sanitizeFileName('A/B:C*D?')).toBe('A_B_C_D_');
  });

  it('renders template vars', () => {
    expect(buildNamedOutput('{client}_{ay}_{module}', { client: 'ABC', ay: '2025', module: 'merge' })).toBe('ABC_2025_merge');
  });
});
