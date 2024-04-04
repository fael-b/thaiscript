import { Text } from "@mantine/core";
import { Review } from "../../../api/types";
import { Initial } from "./Initial";
import { LetterToRomanization } from "./LetterToRomanization";
import { LetterToWord } from "./LetterToWord";
import { LetterToTransliteration } from "./LetterToTransliteration";

export interface ReviewComponentBaseProps {
  review: Review;
  onComplete(correct: boolean): void;
}

export function ReviewSwitcher(args: ReviewComponentBaseProps) {
  let ReviewComponent;
  switch (args.review.type) {
    case "initial":
      ReviewComponent = Initial;
      break;
    case "letter-to-romanization":
      ReviewComponent = LetterToRomanization;
      break;
    case "letter-to-word":
      ReviewComponent = LetterToWord;
      break;
    case "letter-to-transliteration":
      ReviewComponent = LetterToTransliteration;
      break;
    default:
      return <Text>Unimplemented review type: "{args.review.type}"</Text>;
  }

  return <ReviewComponent key={args.review.letterVariant.id} {...args} />;
}
