export interface LetterVariant {
  id: string;
  letter: string;
  learningOrder: number;
  category: "consonant" | "vowel" | "digit" | "tone" | "other";
  tone: "high" | "middle" | "low-sonorant" | "low-voiceless" | "short" | "long" | "final" | "diphtong" | "quasi-letter" | "other";
  position: "before" | "middle" | "end" | "anywhere" | "other";
  romanization: string;
  exampleWord: string;
  exampleWordExplanation: string;
  exampleWordTransliteration: string;
  exampleWordEmoji: string;
  similarWords: string[];
}

export interface LetterVariantWithUnparsedSimilarWords extends Omit<LetterVariant, "similarWords"> {
  similarWords: string;
}
