// #[serde(rename_all = "camelCase")]
// pub struct Model {
//     #[sea_orm(primary_key, auto_increment = false)]
//     pub id: String,
//     pub letter: String,
//     pub learning_order: i32,
//     pub category: String,
//     pub tone: String,
//     pub position: String,
//     pub romanization: String,
//     pub example_word: String,
//     pub example_word_explanation: String,
//     pub example_word_transliteration: String,
//     pub example_word_emoji: String,
//     pub similar_words: String,
// }

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
