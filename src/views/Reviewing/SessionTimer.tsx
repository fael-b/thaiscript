import { useInterval } from "@mantine/hooks";
import { useEffect, useState } from "react";

export function SessionTimer() {
  const [time, setTime] = useState(0);
  const { start, active } = useInterval(() => {
    setTime((c) => c + 1);
  }, 1000);
  useEffect(() => {
    if (!active) {
      start();
    }
  }, [active, start]);

  return <>{formatTime(time)}</>;
}

function formatTime(time: number) {
  const minutes = Math.floor(time / 60);
  const seconds = time % 60;
  return `${minutes < 10 ? `0${minutes}` : minutes}:${seconds < 10 ? `0${seconds}` : seconds}`;
}
