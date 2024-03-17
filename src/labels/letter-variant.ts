import { LetterVariant } from "../api/types";

export const categoryLabels: Record<LetterVariant["category"], string> = {
  consonant: "Consonants",
  vowel: "Vowels",
  digit: "Digits",
  tone: "Groups",
  other: "Other",
};

export const groupLabels: Record<LetterVariant["group"], string> = {
  high: "High Class",
  middle: "Middle Class",
  "low-sonorant": "Low Class (Sonorant)",
  "low-voiceless": "Low Class (Voiceless)",
  short: "Short",
  long: "Long",
  "complex-short": "Short (Complex)",
  "complex-long": "Long (Complex)",
  final: "Final",
  diphtong: "Diphtong",
  "quasi-letter": "Quasi-Letter",
  other: "Other",
};
