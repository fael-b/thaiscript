import { invoke } from "@tauri-apps/api/tauri";
import { SaveReviewOutcomeForm } from "./types";

export async function saveReviewOutcome(reviewOutcome: SaveReviewOutcomeForm) {
  try {
    const response = await invoke("save_review_outcome", {
      date: new Date().toISOString(),
      ...reviewOutcome,
    });
    if (typeof response !== "object") {
      console.error("Invalid response type:", typeof response, response);
      throw new Error("Invalid response from backend");
    }
    return response;
  } catch (err) {
    console.error(err);
    return null;
  }
}
