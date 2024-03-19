import { ActionIcon, Group, Stack, Text } from "@mantine/core";
import { X } from "lucide-react";
import { Link } from "react-router-dom";
import styles from "./ReviewingView.module.css";
import { SessionTimer } from "./SessionTimer";
import { useQueue } from "@mantine/hooks";
import { useCallback, useEffect, useState } from "react";
import { Review } from "../../api/types";
import { fetchNextReviews } from "../../api/queries";
import { saveReviewOutcome } from "../../api/mutations";
import { ReviewSwitcher } from "./reviews";

export function ReviewingView() {
  const {
    state: reviewsHead,
    queue: reviewsQueue,
    add,
    update,
  } = useQueue<Review>({
    limit: 1,
    initialValues: [],
  });
  const currentReview = reviewsHead[0];
  const [isFetchingReviews, setIsFetchingReviews] = useState(false);
  const [currentReviewStatus, setCurrentReviewStatus] = useState({
    startedAt: Date.now(),
  });

  function popQueue() {
    update((values) => values.filter((_, i) => i !== 0));
  }

  useEffect(() => {
    if (!currentReview && !isFetchingReviews) {
      setIsFetchingReviews(true);
      fetchNextReviews().then((newReviews) => {
        if (newReviews.length !== 0) {
          // This is a workaround for a rerendering issue I didn't have time to figure out
          update((_values) => [...newReviews]);
        }
        setIsFetchingReviews(false);
      });
    }
  }, []);

  const transitionToNextReview = useCallback(() => {
    setTimeout(() => {
      popQueue();
      setCurrentReviewStatus({
        startedAt: Date.now(),
      });
    }, 1800);
  }, [currentReview]);

  const handleReviewOutcome = useCallback(
    async (correct: boolean) => {
      if (!currentReview) {
        return;
      }
      let msTimeTaken = Date.now() - currentReviewStatus.startedAt;
      await saveReviewOutcome({
        correct,
        msTimeTaken,
        letterVariantId: currentReview.letterVariant.id,
        reviewType: currentReview.type,
      });

      if (!correct) {
        setCurrentReviewStatus({
          startedAt: Date.now(),
        });
        return;
      }

      if (reviewsQueue.length === 0) {
        setIsFetchingReviews(true);
        const newReviews = await fetchNextReviews();
        if (newReviews.length !== 0) {
          add(...newReviews);
        }
        setIsFetchingReviews(false);
      }
      transitionToNextReview();
    },
    [currentReview, currentReviewStatus],
  );

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
