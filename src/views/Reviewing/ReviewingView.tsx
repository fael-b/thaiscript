import { ActionIcon, Group, Stack, Text } from "@mantine/core";
import { X } from "lucide-react";
import { Link } from "react-router-dom";
import styles from "./ReviewingView.module.css";
import { SessionTimer } from "./SessionTimer";
import { useListState } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { Review } from "../../api/types";
import { fetchNextReviews } from "../../api/queries";
import { saveReviewOutcome } from "../../api/mutations";
// import { LetterToRomanization } from "./reviews/LetterToRomanization";
import { ReviewSwitcher } from "./reviews";

export function ReviewingView() {
  const [reviews, handlers] = useListState<Review>([]);
  const [isFetchingReviews, setIsFetchingReviews] = useState(false);
  const [currentReviewStatus, setCurrentReviewStatus] = useState({
    startedAt: Date.now(),
  });
  const currentReview = reviews[0];

  useEffect(() => {
    if (reviews.length === 0 && !isFetchingReviews) {
      setIsFetchingReviews(true);
      fetchNextReviews().then((reviews) => {
        if (reviews.length !== 0) {
          handlers.append(...reviews);
        }
        setIsFetchingReviews(false);
      });
    }
  }, [reviews]);

  async function handleReviewOutcome(correct: boolean) {
    if (currentReview) {
      let msTimeTaken = Date.now() - currentReviewStatus.startedAt;
      await saveReviewOutcome({
        correct,
        msTimeTaken,
        letterVariantId: currentReview.letterVariant.id,
        reviewType: currentReview.type,
      });
      if (correct) {
        if (reviews.length === 1) {
          setIsFetchingReviews(true);
          console.log("Fetching next reviews");
          fetchNextReviews().then((reviews) => {
            if (reviews.length !== 0) {
              handlers.append(...reviews);
            }
            setIsFetchingReviews(false);
            setTimeout(() => {
              handlers.shift();
            }, 1800);
          });
        } else {
          setTimeout(() => {
            handlers.shift();
          }, 1800);
        }
      }
      setCurrentReviewStatus({
        startedAt: Date.now(),
      });
    }
  }

  return (
    <main className={styles.main}>
      <Stack gap="xl" justify="center" align="center">
        <Group justify="space-between" p="xs" w="100%">
          <ActionIcon color="thai-red" variant="subtle" radius="lg" component={Link} to="/" size="xl">
            <X />
          </ActionIcon>
          <Text
            mx="xs"
            size="lg"
            c="dark"
            style={{
              fontFamily: "monospace, sans-serif",
            }}
          >
            <SessionTimer />
          </Text>
        </Group>
        {currentReview && <ReviewSwitcher review={currentReview} onComplete={handleReviewOutcome} />}
      </Stack>
    </main>
  );
}
