export const sanitizeFileName = (raw: string): string =>
  raw.replace(/[<>:"/\\|?*\x00-\x1F]/g, '_').replace(/\s+/g, '_').slice(0, 120);

export const buildNamedOutput = (
  template: string,
  variables: Record<string, string>
): string => {
  const rendered = template.replace(/\{(\w+)\}/g, (_, key: string) => variables[key] ?? 'NA');
  return sanitizeFileName(rendered);
};
