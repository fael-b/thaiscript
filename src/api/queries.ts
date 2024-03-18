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

export async function fetchLetterVariantsByCategory(category: LetterVariant["category"]) {
  try {
    const response = await invoke("get_letter_variants_by_category", { category });
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

export async function fetchNextReviews() {
  try {
    console.time("fetchNextReviews");
    const response = await invoke("get_next_reviews");
    console.log("nextReviews", response);
    if (typeof response !== "object" || !Array.isArray(response)) {
      console.error("Invalid response type:", typeof response, response);
      throw new Error("Invalid response from backend");
    }
    console.timeEnd("fetchNextReviews");
    return response;
  } catch (err) {
    console.error(err);
    return [];
  }
}
