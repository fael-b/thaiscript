import { Center, Group, SegmentedControl, Title, Stack } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { fetchLetterVariants } from "./data/queries";
import { LetterVariant } from "./data/types";
import { useMemo, useState } from "react";
import { LetterVariantCard } from "./LetterVariantCard";

export function LetterVariantsList() {
  const [currentCategory, setCurrentCategory] = useState<LetterVariant["category"]>("consonant");
  const {
    data: letterVariants,
    isLoading,
    error,
  } = useQuery({
    queryKey: ["letterVariants"],
    queryFn: fetchLetterVariants,
  });

  const letterVariantsByCategoryAndTone = useMemo(() => {
    const letterVariantsByCategoryAndToneMap = new Map<LetterVariant["category"], Map<LetterVariant["tone"], LetterVariant[]>>();
    return (
      letterVariants?.reduce((acc, letterVariant) => {
        if (!acc.has(letterVariant.category)) {
          acc.set(letterVariant.category, new Map());
        }
        let categoryMap = acc.get(letterVariant.category)!;
        if (!categoryMap.has(letterVariant.tone)) {
          categoryMap.set(letterVariant.tone, []);
        }
        categoryMap.get(letterVariant.tone)!.push(letterVariant);
        return acc;
      }, letterVariantsByCategoryAndToneMap) ?? letterVariantsByCategoryAndToneMap
    );
  }, [letterVariants]);

  const currentCategoryLetterVariants = useMemo(() => {
    return Array.from(letterVariantsByCategoryAndTone.get(currentCategory)?.entries() ?? []);
  }, [currentCategory, letterVariantsByCategoryAndTone]);

  return (
    <Stack gap="xs">
      {/* <Title order={1}>🇹🇭 ThaiScript</Title> */}
      {isLoading && <div>Loading...</div>}
      {error && <div>Error: {error.message}</div>}
      <Center pt="xs">
        <SegmentedControl
          data={[
            { value: "consonant", label: categoryLabels.consonant },
            { value: "vowel", label: categoryLabels.vowel, disabled: true },
            { value: "digit", label: categoryLabels.digit, disabled: true },
          ]}
          size="xl"
          color="thai-purple"
          value={currentCategory}
          onChange={(v) => setCurrentCategory(v as LetterVariant["category"])}
        />
      </Center>
      <section key={currentCategory}>
        {currentCategoryLetterVariants.map(([tone, letterVariants]) => (
          <div key={tone}>
            <Title order={2} px="xs" mb="xs">
              {toneLabels[tone]}
            </Title>
            <Group gap="xs" px="xs" mb="md">
              {letterVariants.map((letterVariant) => {
                return <LetterVariantCard key={letterVariant.id} {...letterVariant} />;
              })}
            </Group>
          </div>
        ))}
      </section>
    </Stack>
  );
}

export const categoryLabels: Record<LetterVariant["category"], string> = {
  consonant: "Consonants",
  vowel: "Vowels",
  digit: "Digits",
  tone: "Tones",
  other: "Other",
};

export const toneLabels: Record<LetterVariant["tone"], string> = {
  high: "High Tone",
  middle: "Middle Tone",
  "low-sonorant": "Low Tone (Sonorant)",
  "low-voiceless": "Low Tone (Voiceless)",
  short: "Short",
  long: "Long",
  final: "Final",
  diphtong: "Diphtong",
  "quasi-letter": "Quasi-Letter",
  other: "Other",
};
