export interface LetterVariant {
  id: string;
  letter: string;
  learningOrder: number;
  category: "consonant" | "vowel" | "digit" | "tone" | "other";
  group: "high" | "middle" | "low-sonorant" | "low-voiceless" | "short" | "long" | "complex-short" | "complex-long" | "final" | "diphtong" | "quasi-letter" | "other";
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

export interface ReviewOutcome {
  id: number;
  letterVariantId: string;
  reviewType: ReviewType;
  correct: boolean;
  msTimeTaken: number;
  date: string;
}

export interface SaveReviewOutcomeForm extends Omit<ReviewOutcome, "id" | "date"> {}

export type ReviewType =
  | "letter-to-romanization"
  | "romanization-to-letter"
  | "speech-to-letter"
  | "letter-to-speech"
  | "speech-to-word"
  | "speech-to-emoji"
  | "emoji-to-word"
  | "word-to-emoji"
  | "letter-to-drawing"
  | "speech-to-drawing"
  | "emoji-to-drawing";
