import { ActionIcon, AppShell, Button, Center, Group, SegmentedControl } from "@mantine/core";
import { LetterVariantsList } from "../LetterVariantsList";
import { LineChart, Play } from "lucide-react";
import styles from "./HomeView.module.css";
import { categoryLabels } from "../labels/letter-variant";
import { LetterVariant } from "../api/types";
import { currentCategoryAtom } from "../state/navigation";
import { useAtom } from "jotai";

export function HomeView() {
  const [currentCategory, setCurrentCategory] = useAtom(currentCategoryAtom);

  return (
    <AppShell
      footer={{
        height: 72,
      }}
      header={{
        collapsed: false,
        height: 72,
      }}
    >
      <AppShell.Header>
        <Center pt="xs">
          <SegmentedControl
            data={[
              { value: "consonant", label: categoryLabels.consonant },
              { value: "vowel", label: categoryLabels.vowel, disabled: true },
              { value: "digit", label: categoryLabels.digit, disabled: true },
            ]}
            size="lg"
            color="thai-purple"
            value={currentCategory}
            onChange={(v) => setCurrentCategory(v as LetterVariant["category"])}
          />
        </Center>
      </AppShell.Header>
      <AppShell.Main>
        <LetterVariantsList />
      </AppShell.Main>
      <AppShell.Footer>
        <Group justify="center" style={{ height: "100%" }}>
          <ActionIcon title="Logs" radius="xl" size="xl" variant="outline">
            <LineChart />
          </ActionIcon>
          <Button
            leftSection={<Play />}
            title="Review letters"
            radius="xl"
            color="thai-gold"
            variant="gradient"
            gradient={{
              from: "yellow",
              to: "thai-gold",
            }}
            size="md"
            miw="50%"
            className={styles.review}
          >
            Review
          </Button>
        </Group>
      </AppShell.Footer>
    </AppShell>
  );
}
