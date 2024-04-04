import { Button, Stack, Text } from "@mantine/core";
import { LetterVariant } from "../../api/types";
import { speak } from "../../speech";

interface LetterVariantCardProps extends LetterVariant {}

export function LetterVariantCard({ letter, romanization, exampleWord }: LetterVariantCardProps) {
  return (
    <Button
      variant="light"
      radius="lg"
      title={letter}
      onClick={() => {
        speak(exampleWord);
      }}
      h={80}
      w={80}
    >
      <Stack
        align="center"
        justify="center"
        style={{
          gap: 0,
        }}
      >
        <Text
          c="thai-purple"
          style={{
            fontSize: 38,
          }}
        >
          {letter}
        </Text>
        <Text size="xs">{romanization}</Text>
      </Stack>
    </Button>
  );
}
