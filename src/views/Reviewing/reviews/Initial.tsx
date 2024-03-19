import { Chip, Group, Stack, Text } from "@mantine/core";
import { ReviewComponentBaseProps } from ".";
import { speak } from "../../../speech";
import clsx from "clsx";
import { useEffect, useState } from "react";
import styles from "./Review.module.css";

export function Initial({ review, onComplete }: ReviewComponentBaseProps) {
  const [isSelected, setIsSelected] = useState(false);

  function handleOptionClick() {
    if (isSelected) {
      return;
    }
    setIsSelected(true);
    speak(review.letterVariant.exampleWord);
    onComplete(true);
  }

  useEffect(() => {
    setIsSelected(false);
  }, [review.letterVariant.id]);

  const option = review.letterVariant;

  return (
    <Stack align="center" justify="center" gap="xl" w="100%">
      <Text c="thai-purple" className={clsx(styles.letter, isSelected && styles.correct)}>
        {review.letterVariant.letter}
      </Text>
      <Group justify="center" align="center" w="90%">
        <Chip
          value={option.id}
          onClick={() => handleOptionClick()}
          checked={isSelected}
          icon={"⁠"}
          variant={isSelected ? "filled" : "light"}
          color="thai-gold"
          c="thai-purple"
          radius="lg"
          classNames={{
            root: styles.chip_root,
            label: styles.chip_label,
            input: styles.chip_input,
            iconWrapper: styles.chip_iconwrapper,
          }}
          styles={{
            root: {
              width: "100%",
            },
            label: {
              height: 256,
            },
          }}
          size="xl"
          key={option.id}
        >
          <Stack align="center" justify="center">
            <Text
              fw="bold"
              style={{
                fontSize: 48,
              }}
            >
              {option.romanization}
            </Text>
            <Text fs="italic">{option.exampleWordTransliteration}</Text>
            <Text fs="italic">
              {option.exampleWordExplanation} {option.exampleWordEmoji}
            </Text>
          </Stack>
        </Chip>
      </Group>
    </Stack>
  );
}
