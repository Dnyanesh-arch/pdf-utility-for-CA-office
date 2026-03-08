export const computePageRanges = (pagesPerFile: number[]): { from: number; to: number }[] => {
  let cursor = 1;
  return pagesPerFile.map((pages) => {
    const range = { from: cursor, to: cursor + pages - 1 };
    cursor = range.to + 1;
    return range;
  });
};
