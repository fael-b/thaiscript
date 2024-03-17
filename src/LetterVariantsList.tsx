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

  const letterVariantsByCategoryAndGroup = useMemo(() => {
    const letterVariantsByCategoryAndGroupMap = new Map<LetterVariant["category"], Map<LetterVariant["group"], LetterVariant[]>>();
    return (
      letterVariants?.reduce((acc, letterVariant) => {
        if (!acc.has(letterVariant.category)) {
          acc.set(letterVariant.category, new Map());
        }
        let categoryMap = acc.get(letterVariant.category)!;
        if (!categoryMap.has(letterVariant.group)) {
          categoryMap.set(letterVariant.group, []);
        }
        categoryMap.get(letterVariant.group)!.push(letterVariant);
        return acc;
      }, letterVariantsByCategoryAndGroupMap) ?? letterVariantsByCategoryAndGroupMap
    );
  }, [letterVariants]);

  const currentCategoryLetterVariants = useMemo(() => {
    return Array.from(letterVariantsByCategoryAndGroup.get(currentCategory)?.entries() ?? []);
  }, [currentCategory, letterVariantsByCategoryAndGroup]);

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
        {currentCategoryLetterVariants.map(([group, letterVariants]) => (
          <div key={group}>
            <Title order={2} px="xs" mb="xs">
              {groupLabels[group]}
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
  tone: "Groups",
  other: "Other",
};

export const groupLabels: Record<LetterVariant["group"], string> = {
  high: "High Class",
  middle: "Middle Class",
  "low-sonorant": "Low Class (Sonorant)",
  "low-voiceless": "Low Class (Voiceless)",
  short: "Short",
  long: "Long",
  "complex-short": "Short (Complex)",
  "complex-long": "Long (Complex)",
  final: "Final",
  diphtong: "Diphtong",
  "quasi-letter": "Quasi-Letter",
  other: "Other",
};
