export interface Language {
  name: string;
  orderedByFrequency?: boolean;
  noLazyMode?: boolean;
  words: string[];
}
