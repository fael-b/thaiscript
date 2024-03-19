import { useEffect, useState } from "react";
import { speak } from "../../../speech";
import { Stack, Text, Group, Chip } from "@mantine/core";
import clsx from "clsx";
import styles from "./Review.module.css";
import { ReviewComponentBaseProps } from ".";

export function LetterToRomanization({ review, onComplete }: ReviewComponentBaseProps) {
  const [selectedOptionsIds, setSelectedOptionsIds] = useState(new Set());
  const isCorrectOptionSelected = selectedOptionsIds.has(review?.letterVariant.id);

  function handleOptionClick(optionId: string) {
    if (selectedOptionsIds.has(optionId)) return;
    setSelectedOptionsIds((prevIds) => {
      const newIds = new Set(prevIds);
      newIds.add(optionId);
      return newIds;
    });
    const isCorrect = optionId === review.letterVariant.id;
    onComplete(isCorrect);
    if (isCorrect) {
      speak(review.letterVariant.exampleWord);
    }
  }

  useEffect(() => {
    setSelectedOptionsIds(new Set());
  }, [review.letterVariant.id]);

  return (
    <Stack align="center" justify="center" gap="xl" w="100%">
      <Text c="thai-purple" className={clsx(styles.letter, isCorrectOptionSelected && styles.correct)}>
        {review.letterVariant.letter}
      </Text>
      <Group justify="center" align="center" w="90%">
        {review.options.map((option) => {
          const isCorrect = option.id === review.letterVariant.id;
          const isSelected = selectedOptionsIds.has(option.id);

          return (
            <Chip
              value={option.id}
              onClick={() => handleOptionClick(option.id)}
              checked={isSelected}
              icon={"⁠"}
              disabled={!isCorrect && !isSelected && isCorrectOptionSelected}
              variant={isSelected ? "filled" : "light"}
              color={isCorrect ? "thai-gold" : "thai-red"}
              c="thai-purple"
              radius="lg"
              className={clsx(isSelected && !isCorrect && styles.shaking)}
              styles={{
                root: {
                  width: "45%",
                  // Disable text selection
                  userSelect: "none",
                  WebkitTouchCallout: "none",
                  WebkitUserSelect: "none",
                },
                label: {
                  width: "100%",
                  display: "flex",
                  justifyContent: "center",
                  alignItems: "center",
                  height: 100,
                },
                input: {
                  width: "100%",
                },
                iconWrapper: {
                  display: "none",
                },
              }}
              // w="45%"
              size="xl"
              h="100"
              key={option.id}
            >
              {option.romanization}
            </Chip>
          );
        })}
      </Group>
    </Stack>
  );
}
