import { invoke } from "@tauri-apps/api/tauri";

const synth = window.speechSynthesis;
const voices = synth.getVoices();
const thaiVoice = voices.find((voice) => voice.lang === "th-TH");

export function syntheticSpeak(text: string, speed: number = 1) {
  const utterrance = new SpeechSynthesisUtterance(text);

  // If local Thai voice isn't found, use google TTS as fallback
  if (!thaiVoice) {
    return googleSpeak(text, speed);
  }
  utterrance.voice = thaiVoice!;

  // Set speed of speech (sometimes slower for better understanding)
  utterrance.rate = speed;

  // Randomize pitch to make it sound less boring
  const basePitch = 1;
  const pitchRangeSlider = 0.3;
  const randomnessFactor = Math.random() * 0.75;
  utterrance.pitch = basePitch + randomnessFactor - pitchRangeSlider;

  // Cancel any previous ongoing speech
  synth.cancel();

  synth.speak(utterrance);
}

let currentAudio: HTMLAudioElement | null = null;

export async function googleSpeak(text: string, _speed: number = 1) {
  // Insert space after the first character to pronounce it correctly
  const formattedText = text.replace(/^(\S)/, "$1 ");
  const response: string | number[] = await invoke("speak", { text: encodeURIComponent(formattedText) });
  if (typeof response === "string") {
    console.error("Error from TTS API", response);
  } else {
    const blob = new Blob([new Uint8Array(response)], { type: "audio/mpeg" });
    const url = URL.createObjectURL(blob);
    if (currentAudio) {
      currentAudio.pause();
      currentAudio = null;
    }
    currentAudio = new Audio(url);
    currentAudio.play();
  }
}
