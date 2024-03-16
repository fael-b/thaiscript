import { invoke } from "@tauri-apps/api/tauri";
import { LetterVariant, LetterVariantWithUnparsedSimilarWords } from "./types";

export async function fetchLetterVariants() {
  try {
    const response = await invoke("get_letter_variants");
    if (typeof response !== "object" || !Array.isArray(response)) {
      console.error("Invalid response type:", typeof response, response);
      throw new Error("Invalid response from backend");
    }
    const letterVariants: LetterVariant[] = response.map((letterVariant: LetterVariantWithUnparsedSimilarWords) => {
      return {
        ...letterVariant,
        similarWords: JSON.parse(letterVariant.similarWords),
      };
    });

    return letterVariants;
  } catch (err) {
    console.error(err);
    return [];
  }
}
