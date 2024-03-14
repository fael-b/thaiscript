const synth = window.speechSynthesis;
const voices = synth.getVoices();
const thaiVoice = voices.find((voice) => voice.lang === "th-TH");

export function say(letter: string, speed: number = 1) {
  const utterrance = new SpeechSynthesisUtterance(letter);

  // Set voice to Thai if available
  if (thaiVoice) {
    utterrance.voice = thaiVoice;
  }

  // Set speed of speech (sometimes slower for better understanding)
  utterrance.rate = speed;

  // Randomize pitch to make it sound less boring
  const basePitch = 1;
  const pitchRangeSlider = 0.3;
  const randomnessFactor = Math.random() * 0.75;
  utterrance.pitch = basePitch + randomnessFactor - pitchRangeSlider;

  synth.speak(utterrance);
}
