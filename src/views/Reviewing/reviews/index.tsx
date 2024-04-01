import { Text } from "@mantine/core";
import { Review } from "../../../api/types";
import { Initial } from "./Initial";
import { LetterToRomanization } from "./LetterToRomanization";

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
    default:
      return <Text>Unimplemented review type: "{args.review.type}"</Text>;
  }

  return <ReviewComponent key={args.review.letterVariant.id} {...args} />;
}
