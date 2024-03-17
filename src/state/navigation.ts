import { atom } from "jotai";
import { LetterVariant } from "../api/types";

export const currentCategoryAtom = atom<LetterVariant["category"]>("consonant");
