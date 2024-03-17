import { Button, Stack, Text } from "@mantine/core";
import { LetterVariant } from "./data/types";
import { speak } from "./speech";

interface LetterVariantCardProps extends LetterVariant {}

export function LetterVariantCard({ letter, romanization, exampleWord }: LetterVariantCardProps) {
  return (
    <Button variant="light" radius="lg" title={letter} onClick={() => speak(exampleWord)} h={80} w={80}>
      <Stack
        align="center"
        // py="xl"
        justify="center"
        style={{
          gap: 0,
        }}
      >
        <Text
          c="thai-purple"
          style={{
            fontSize: 38,
            fontFamily: "Sarabun, sans-serif",
          }}
        >
          {letter}
        </Text>
        <Text size="xs">{romanization}</Text>
      </Stack>
    </Button>
  );
}
